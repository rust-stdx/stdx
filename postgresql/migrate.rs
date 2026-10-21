//! Forward-only SQL migrations.
//!
//! A [`Migrator`] loads plain SQL files, compares them with the migrations
//! recorded in the database, and applies the pending ones in version order.
//! There are no down migrations: a migration is only ever applied once.
//!
//! # File layout
//!
//! Each migration is a `.sql` file named `<version>_<description>.sql`, where
//! `<version>` is a non-negative integer. The version is read from the file
//! name, not from its directory, so files can be grouped into subdirectories:
//!
//! ```text
//! migrations/
//!   2025/
//!     20250101_init.sql
//!     20250615_add_users.sql
//!   2026/
//!     20260101_add_orders.sql
//! ```
//!
//! Files are sorted numerically by version across all subdirectories. Files
//! that do not end in `.sql` are ignored. A `.sql` file whose name does not
//! follow the format is an error, because silently skipping a mistyped
//! migration is usually worse than failing.
//!
//! # Loading migrations
//!
//! From the file system:
//!
//! ```ignore
//! use postgresql::migrate::Migrator;
//!
//! let migrator = Migrator::from_dir("migrations")?;
//! migrator.run(&pool).await?;
//! ```
//!
//! From an embedded folder (the `embed` feature is enabled by default):
//!
//! ```ignore
//! use embed::Embed;
//! use postgresql::migrate::Migrator;
//!
//! #[derive(Embed)]
//! #[folder = "migrations/"]
//! struct Migrations;
//!
//! let migrator = Migrator::from_embed::<Migrations>()?;
//! migrator.run(&pool).await?;
//! ```
//!
//! In debug builds the `embed` derive reads the folder from the file system,
//! so edits are picked up without recompiling. Release builds embed the files
//! that exist at build time.
//!
//! For code that is already embedded with `include_str!`, build migrations
//! manually:
//!
//! ```ignore
//! use postgresql::migrate::{Migration, Migrator};
//!
//! let migrator = Migrator::from_migrations([
//!     Migration::new(1, "init", include_str!("../migrations/0001_init.sql")),
//!     Migration::new(2, "add_users", include_str!("../migrations/0002_add_users.sql")),
//! ])?;
//! ```
//!
//! # Tracking and validation
//!
//! Applied migrations are recorded in the table selected by
//! [`Migrator::table_name`] (by default `_stdx_postgresql_migrations`). Each
//! row stores the version, description, a BLAKE3 checksum of the SQL, when it
//! was applied, and whether it succeeded. A tracking table created by an older
//! version of this library is upgraded in place (missing `success` and
//! `execution_time_ms` columns are added).
//!
//! By default the migrator is strict:
//!
//! * an applied migration that is missing from the source is an error;
//! * an applied migration whose checksum changed is an error;
//! * a pending migration with a lower version than an already-applied one is
//!   an error (out of order).
//!
//! These checks can be relaxed individually with [`Migrator::allow_missing`],
//! [`Migrator::verify_checksums`], and [`Migrator::allow_out_of_order`].
//! Disabling checksum verification is intended for local development while
//! iterating on a migration that has already been applied; it does not re-run
//! the edited migration.
//!
//! # Transactions
//!
//! By default all pending transactional migrations are applied in a single
//! transaction. A migration whose first non-empty line is
//! `-- migrate:no-transaction` is a transaction boundary: the preceding group
//! is committed, the migration runs outside a transaction, and a new group is
//! started for the following migrations. The same can be requested
//! programmatically with [`Migration::no_transaction`].
//!
//! A failed transactional group is rolled back as a whole.
//!
//! A no-transaction migration is still sent as one simple-query script.
//! PostgreSQL treats a multi-statement simple query as one implicit
//! transaction, so a statement that cannot run in a transaction (such as
//! `CREATE INDEX CONCURRENTLY`) should be the only statement in its migration.
//!
//! Migrations must not contain their own `BEGIN` / `COMMIT` / `ROLLBACK`; the
//! migrator manages transactions. A transactional migration that ends the
//! surrounding transaction is detected (from the server's `ReadyForQuery`
//! status) and aborts the run before any later migration is applied.
//!
//! A failed non-transactional migration leaves a row with `success = false`,
//! and later runs refuse to continue until it is repaired with
//! [`Migrator::repair_failed`] (which deletes the row; verify the database
//! first, because the migration is applied again from the beginning).
//!
//! # Concurrency
//!
//! By default the migrator holds a PostgreSQL session advisory lock for the
//! whole run, so several application instances can safely run migrations on
//! startup. [`Migrator::with_advisory_lock`] can disable this when the caller
//! guarantees that only one migrator runs at a time.
//!
//! The lock is session-level, so it needs a direct connection or session
//! pooling. Transaction-pooling proxies such as PgBouncer can move successive
//! statements to different backends and are not safe for migrations. All
//! migrators in the same database share the same lock key, so unrelated
//! migration sets also serialize with each other.
//!
//! # Errors
//!
//! Loading errors, history validation failures, and failed migrations are
//! returned as [`Error::Migration`]. Server, connection, and I/O errors keep
//! their usual variants.

use std::{
    borrow::Cow,
    collections::BTreeMap,
    fmt, fs,
    path::Path,
    time::{Duration, Instant},
};

use crypto::{Hasher, blake3::Blake3};

use crate::{
    connection::Connection,
    encode::ToSql,
    error::Error,
    pool::Pool,
    query::{Executor, Query, QueryAs},
    transaction::Transaction,
};

const DEFAULT_TABLE_NAME: &str = "_stdx_postgresql_migrations";
const MIGRATION_LOCK_KEY: i64 = 0x7067_6d69_6772_6174;
const DEFAULT_LOCK_TIMEOUT: Duration = Duration::from_secs(60);
const LOCK_POLL_INTERVAL: Duration = Duration::from_millis(100);
const NO_TRANSACTION_MARKER: &str = "-- migrate:no-transaction";

/// One forward migration.
///
/// Migrations are usually loaded with [`Migrator::from_dir`] or
/// `Migrator::from_embed`, but can also be constructed directly with
/// [`Migration::new`].
///
/// The checksum is computed automatically from the SQL. A leading UTF-8 BOM is
/// removed before the SQL is stored and hashed.
#[derive(Clone)]
pub struct Migration {
    version: i64,
    name: String,
    sql: Cow<'static, str>,
    checksum: [u8; 32],
    transaction: bool,
    source: Option<String>,
}

impl Migration {
    /// Creates a migration from its version, description, and SQL.
    ///
    /// The version must be non-negative and the description must not be empty;
    /// these constraints are enforced when the migration is added to a
    /// [`Migrator`].
    ///
    /// If the first non-empty line of `sql` is
    /// `-- migrate:no-transaction`, the migration is marked as running outside
    /// a transaction. Use [`Migration::no_transaction`] to do the same
    /// programmatically.
    pub fn new(version: i64, name: impl Into<String>, sql: impl Into<Cow<'static, str>>) -> Self {
        let mut sql = sql.into();
        if let Some(rest) = sql.strip_prefix('\u{feff}') {
            sql = Cow::Owned(rest.to_owned());
        }
        let transaction = !starts_with_no_transaction(&sql);
        let checksum = checksum(&sql);
        Self {
            version,
            name: name.into(),
            sql,
            checksum,
            transaction,
            source: None,
        }
    }

    /// Marks this migration as running outside a transaction.
    ///
    /// Use this for statements that PostgreSQL cannot run in a transaction,
    /// such as `CREATE INDEX CONCURRENTLY`, `REINDEX CONCURRENTLY`, or
    /// `VACUUM`.
    ///
    /// The SQL is sent as one simple-query script. PostgreSQL treats a
    /// multi-statement simple query as one implicit transaction, so a
    /// statement that must run outside a transaction should be the only
    /// statement in its migration.
    pub fn no_transaction(mut self) -> Self {
        self.transaction = false;
        self
    }

    /// Returns the migration version.
    pub fn version(&self) -> i64 {
        self.version
    }

    /// Returns the human-readable description.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the SQL.
    pub fn sql(&self) -> &str {
        &self.sql
    }

    /// Returns the BLAKE3 checksum of the SQL, normalized to LF line endings.
    pub fn checksum(&self) -> &[u8; 32] {
        &self.checksum
    }

    /// Returns `true` when this migration runs inside a transaction.
    pub fn is_transactional(&self) -> bool {
        self.transaction
    }

    fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    fn display_source(&self) -> &str {
        self.source.as_deref().unwrap_or(&self.name)
    }
}

impl fmt::Debug for Migration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Migration")
            .field("version", &self.version)
            .field("name", &self.name)
            .field("source", &self.source)
            .field("transaction", &self.transaction)
            .finish_non_exhaustive()
    }
}

/// A resolved set of migrations and the settings used to apply them.
///
/// Construct one with [`Migrator::from_dir`], `Migrator::from_embed`, or
/// [`Migrator::from_migrations`], then call [`Migrator::run`] or
/// [`Migrator::status`].
#[derive(Clone)]
pub struct Migrator {
    migrations: Vec<Migration>,
    table_name: String,
    lock_timeout: Duration,
    verify_checksums: bool,
    allow_missing: bool,
    allow_out_of_order: bool,
    advisory_lock: bool,
}

impl Default for Migrator {
    fn default() -> Self {
        Self {
            migrations: Vec::new(),
            table_name: DEFAULT_TABLE_NAME.to_owned(),
            lock_timeout: DEFAULT_LOCK_TIMEOUT,
            verify_checksums: true,
            allow_missing: false,
            allow_out_of_order: false,
            advisory_lock: true,
        }
    }
}

impl fmt::Debug for Migrator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Migrator")
            .field("migrations", &self.migrations)
            .field("table_name", &self.table_name)
            .field("lock_timeout", &self.lock_timeout)
            .field("verify_checksums", &self.verify_checksums)
            .field("allow_missing", &self.allow_missing)
            .field("allow_out_of_order", &self.allow_out_of_order)
            .field("advisory_lock", &self.advisory_lock)
            .finish()
    }
}

impl Migrator {
    /// Loads every `.sql` migration below `path`.
    ///
    /// Subdirectories are walked recursively. The version and description are
    /// parsed from each file name, so directory names are purely
    /// organizational.
    ///
    /// The path is read synchronously. It is intended to be called once during
    /// startup, before the rest of the application starts serving traffic.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Migration`] if the path cannot be read, is not a
    /// directory, contains no `.sql` files, contains a `.sql` file with an
    /// invalid name, or contains two migrations with the same version.
    pub fn from_dir(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref();
        let metadata = fs::metadata(path)
            .map_err(|err| Error::Migration(format!("cannot access migration directory {}: {err}", path.display())))?;
        if !metadata.is_dir() {
            return Err(Error::Migration(format!(
                "migration path {} is not a directory",
                path.display()
            )));
        }

        let mut migrations = Vec::new();
        walk_dir(path, &mut migrations)?;
        if migrations.is_empty() {
            return Err(Error::Migration(format!("no .sql migrations found in {}", path.display())));
        }
        Self::from_migrations(migrations)
    }

    /// Loads every `.sql` migration from a type generated by
    /// [`embed::Embed`](https://docs.rs/embed).
    ///
    /// This method is available when the `embed` feature is enabled, which is
    /// the default. In debug builds the embedded folder is read from the file
    /// system; in release builds the files present at build time are embedded
    /// in the binary.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Migration`] if the embedded folder contains no `.sql`
    /// files, contains a `.sql` file with an invalid name, contains a file
    /// that is not valid UTF-8, or contains duplicate versions.
    #[cfg(feature = "embed")]
    pub fn from_embed<M: embed::RustEmbed>() -> Result<Self, Error> {
        let mut migrations = Vec::new();
        for path in M::iter() {
            let path = path.as_ref();
            let file_name = path.rsplit('/').next().unwrap_or(path);
            if !file_name.ends_with(".sql") {
                continue;
            }
            let (version, name) = parse_file_name(file_name).map_err(|message| {
                Error::Migration(format!("invalid embedded migration filename {path}: {message}"))
            })?;
            let file =
                M::get(path).ok_or_else(|| Error::Migration(format!("embedded migration {path} was not found")))?;
            let sql: Cow<'static, str> =
                match file.data {
                    Cow::Borrowed(bytes) => Cow::Borrowed(std::str::from_utf8(bytes).map_err(|err| {
                        Error::Migration(format!("embedded migration {path} is not valid UTF-8: {err}"))
                    })?),
                    Cow::Owned(bytes) => Cow::Owned(String::from_utf8(bytes).map_err(|err| {
                        Error::Migration(format!("embedded migration {path} is not valid UTF-8: {err}"))
                    })?),
                };
            migrations.push(Migration::new(version, name, sql).with_source(path.to_owned()));
        }

        if migrations.is_empty() {
            return Err(Error::Migration("no .sql migrations found in the embedded folder".to_owned()));
        }
        Self::from_migrations(migrations)
    }

    /// Builds a migrator from already-loaded migrations.
    ///
    /// The migrations are sorted by version. An empty set is allowed.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Migration`] if a version is negative, a description is
    /// empty, or two migrations have the same version.
    pub fn from_migrations(migrations: impl IntoIterator<Item = Migration>) -> Result<Self, Error> {
        let mut migrations: Vec<Migration> = migrations.into_iter().collect();
        for migration in &migrations {
            if migration.version < 0 {
                return Err(Error::Migration(format!(
                    "migration {} ({}) has a negative version",
                    migration.version, migration.name
                )));
            }
            if migration.name.trim().is_empty() {
                return Err(Error::Migration(format!(
                    "migration {} has an empty description",
                    migration.version
                )));
            }
        }

        migrations.sort_by_key(|migration| migration.version);
        for pair in migrations.windows(2) {
            if pair[0].version == pair[1].version {
                return Err(Error::Migration(format!(
                    "duplicate migration version {}: {} and {}",
                    pair[0].version,
                    pair[0].display_source(),
                    pair[1].display_source()
                )));
            }
        }

        Ok(Self {
            migrations,
            ..Self::default()
        })
    }

    /// Sets the tracking table name.
    ///
    /// The name may be `table` or `schema.table`. It is quoted as an
    /// identifier, so pass unquoted names. The default is
    /// `_stdx_postgresql_migrations`.
    ///
    /// Changing this on a database that already has a migration history makes
    /// previously applied migrations appear missing.
    pub fn table_name(mut self, name: impl Into<String>) -> Self {
        self.table_name = name.into();
        self
    }

    /// Sets how long to wait for the migration advisory lock.
    ///
    /// The default is 60 seconds. [`Duration::ZERO`] waits forever.
    pub fn lock_timeout(mut self, timeout: Duration) -> Self {
        self.lock_timeout = timeout;
        self
    }

    /// Enables or disables checksum verification for applied migrations.
    ///
    /// Verification is enabled by default. Disabling it is intended for local
    /// development while iterating on a migration that has already been
    /// applied: the stored checksum is not rewritten, and the edited
    /// migration is not re-run.
    pub fn verify_checksums(mut self, verify: bool) -> Self {
        self.verify_checksums = verify;
        self
    }

    /// Allows applied migrations that are missing from the migration source.
    ///
    /// This is disabled by default.
    pub fn allow_missing(mut self, allow: bool) -> Self {
        self.allow_missing = allow;
        self
    }

    /// Allows pending migrations whose version is lower than the highest
    /// applied version.
    ///
    /// This is disabled by default. Applying an out-of-order migration can
    /// produce a different schema history than a fresh database, so use it
    /// deliberately.
    pub fn allow_out_of_order(mut self, allow: bool) -> Self {
        self.allow_out_of_order = allow;
        self
    }

    /// Enables or disables the PostgreSQL advisory lock.
    ///
    /// The lock is enabled by default. Disable it only when the caller
    /// guarantees that a single migrator runs at a time; concurrent migrators
    /// without the lock can execute the same migration twice.
    pub fn with_advisory_lock(mut self, enabled: bool) -> Self {
        self.advisory_lock = enabled;
        self
    }

    /// Returns the loaded migrations, sorted by version.
    pub fn migrations(&self) -> &[Migration] {
        &self.migrations
    }

    /// Applies every pending migration using a pooled connection.
    ///
    /// One connection is checked out for the whole run, so the advisory lock
    /// is held on a single session. The connection is returned to the pool
    /// when the run finishes.
    ///
    /// # Errors
    ///
    /// Returns an error if the pool has no available connection, the
    /// migration history is invalid, a migration fails, or the lock cannot be
    /// acquired within [`Migrator::lock_timeout`].
    pub async fn run(&self, pool: &Pool) -> Result<Report, Error> {
        let conn = pool.get().await?;
        self.run_on(&conn).await
    }

    /// Applies every pending migration on an existing connection.
    ///
    /// The connection must be dedicated to the migration run: it must not
    /// have an open transaction, and no other task may use another clone of
    /// it while the migrator is running.
    ///
    /// # Errors
    ///
    /// Returns an error if the migration history is invalid, a migration
    /// fails, or the lock cannot be acquired within
    /// [`Migrator::lock_timeout`].
    pub async fn run_on(&self, conn: &Connection) -> Result<Report, Error> {
        let table = Table::new(&self.table_name)?;
        let lock = if self.advisory_lock {
            Some(MigrationLock::acquire(conn, self.lock_timeout).await?)
        } else {
            None
        };

        let result = self.run_locked(conn, &table).await;
        if let Some(lock) = lock
            && let Err(err) = lock.release().await
        {
            tracing::warn!(error = %err, "postgresql: failed to release the migration advisory lock");
        }
        result
    }

    /// Reports the state of the local migrations and the database history.
    ///
    /// This is read-only: it does not create the tracking table and does not
    /// acquire the advisory lock. If the tracking table does not exist, every
    /// local migration is reported as [`MigrationState::Pending`].
    ///
    /// History problems (missing, modified, dirty, or out-of-order migrations)
    /// are reported in the returned [`StatusReport`] rather than returned as
    /// errors. [`Migrator::allow_missing`] and
    /// [`Migrator::allow_out_of_order`] only affect [`Migrator::run`]; status
    /// always reports those states. [`Migrator::verify_checksums`] does affect
    /// status: when verification is disabled, modified migrations are reported
    /// as [`MigrationState::Applied`].
    ///
    /// # Errors
    ///
    /// Returns an error if the pool has no available connection or the
    /// tracking table cannot be read.
    pub async fn status(&self, pool: &Pool) -> Result<StatusReport, Error> {
        let conn = pool.get().await?;
        self.status_on(&conn).await
    }

    /// Reports the state of the local migrations and the database history on
    /// an existing connection.
    ///
    /// See [`Migrator::status`] for the behavior.
    ///
    /// # Errors
    ///
    /// Returns an error if the tracking table cannot be read.
    pub async fn status_on(&self, conn: &Connection) -> Result<StatusReport, Error> {
        let table = Table::new(&self.table_name)?;
        let qualified = table.qualified();
        let params: [&dyn ToSql; 1] = [&qualified];
        let exists: Option<String> = QueryAs::<Option<String>>::new("SELECT to_regclass($1)::text", &params)
            .fetch_one(conn)
            .await
            .map_err(|err| Error::Migration(format!("could not check migration table {qualified}: {err}")))?;

        if exists.is_none() {
            return Ok(StatusReport {
                migrations: self
                    .migrations
                    .iter()
                    .map(|migration| MigrationStatus {
                        version: migration.version,
                        name: migration.name.clone(),
                        state: MigrationState::Pending,
                        applied_at: None,
                    })
                    .collect(),
                current_version: None,
            });
        }

        let applied = load_applied(conn, &table).await?;
        let max_applied = applied.iter().map(|row| row.version).max();
        let mut migrations = Vec::with_capacity(self.migrations.len() + applied.len());

        for migration in &self.migrations {
            if let Some(row) = applied.iter().find(|row| row.version == migration.version) {
                let state = if !row.success {
                    MigrationState::Dirty
                } else if self.verify_checksums && migration.checksum != row.checksum {
                    MigrationState::Modified
                } else {
                    MigrationState::Applied
                };
                migrations.push(MigrationStatus {
                    version: migration.version,
                    name: migration.name.clone(),
                    state,
                    applied_at: row.applied_at,
                });
            } else {
                let state = if max_applied.is_some_and(|max| migration.version < max) {
                    MigrationState::OutOfOrder
                } else {
                    MigrationState::Pending
                };
                migrations.push(MigrationStatus {
                    version: migration.version,
                    name: migration.name.clone(),
                    state,
                    applied_at: None,
                });
            }
        }

        for row in &applied {
            if !self.migrations.iter().any(|migration| migration.version == row.version) {
                migrations.push(MigrationStatus {
                    version: row.version,
                    name: row.name.clone(),
                    state: MigrationState::Missing,
                    applied_at: row.applied_at,
                });
            }
        }

        migrations.sort_by_key(|migration| migration.version);
        Ok(StatusReport {
            migrations,
            current_version: max_applied,
        })
    }

    /// Deletes the tracking row of a failed migration so it can be run again.
    ///
    /// Use this only after verifying — and, if necessary, cleaning up — the
    /// partial changes a failed non-transactional migration may have made: the
    /// migration is applied again from the beginning.
    ///
    /// Returns `true` when a failed row was removed.
    ///
    /// # Errors
    ///
    /// Returns an error if the pool has no available connection, the tracking
    /// table cannot be read, or the delete fails.
    pub async fn repair_failed(&self, pool: &Pool, version: i64) -> Result<bool, Error> {
        let conn = pool.get().await?;
        self.repair_failed_on(&conn, version).await
    }

    /// Like [`Migrator::repair_failed`], but on an existing connection.
    ///
    /// # Errors
    ///
    /// Returns an error if the tracking table cannot be read or the delete
    /// fails.
    pub async fn repair_failed_on(&self, conn: &Connection, version: i64) -> Result<bool, Error> {
        let table = Table::new(&self.table_name)?;
        let sql = format!("DELETE FROM {} WHERE version = $1 AND success = FALSE", table.qualified());
        let params: [&dyn ToSql; 1] = [&version];
        let affected = Query::new(&sql, &params)
            .execute(conn)
            .await
            .map_err(|err| Error::Migration(format!("could not repair migration {version}: {err}")))?;
        Ok(affected > 0)
    }

    async fn run_locked(&self, conn: &Connection, table: &Table) -> Result<Report, Error> {
        ensure_table(conn, table).await?;
        let applied = load_applied(conn, table).await?;
        let pending = self.validate(&applied)?;

        let mut report = Report {
            applied: Vec::new(),
            already_applied: applied.len(),
        };
        let mut group: Vec<&Migration> = Vec::new();

        for migration in pending {
            if migration.is_transactional() {
                group.push(migration);
            } else {
                if !group.is_empty() {
                    self.apply_group(conn, table, &group, &mut report).await?;
                    group.clear();
                }
                self.apply_without_transaction(conn, table, migration, &mut report)
                    .await?;
            }
        }

        if !group.is_empty() {
            self.apply_group(conn, table, &group, &mut report).await?;
        }
        Ok(report)
    }

    fn validate<'a>(&'a self, applied: &[AppliedRow]) -> Result<Vec<&'a Migration>, Error> {
        let mut applied_by_version = BTreeMap::new();
        for row in applied {
            if !row.success {
                return Err(Error::Migration(format!(
                    "migration {} ({}) is marked as failed; repair the database, then delete its row from {} for version {}",
                    row.version, row.name, self.table_name, row.version
                )));
            }
            applied_by_version.insert(row.version, row);
        }

        let missing: Vec<&AppliedRow> = applied
            .iter()
            .filter(|row| !self.migrations.iter().any(|migration| migration.version == row.version))
            .collect();
        if !self.allow_missing && !missing.is_empty() {
            let list = missing
                .iter()
                .map(|row| format!("{} ({})", row.version, row.name))
                .collect::<Vec<_>>()
                .join(", ");
            return Err(Error::Migration(format!(
                "applied migrations missing from the migration source: {list}"
            )));
        }

        for row in applied {
            if let Some(migration) = self
                .migrations
                .iter()
                .find(|migration| migration.version == row.version)
            {
                if self.verify_checksums && migration.checksum != row.checksum {
                    return Err(Error::Migration(format!(
                        "migration {} ({}) checksum mismatch: the file was edited after it was applied",
                        migration.version, migration.name
                    )));
                }
                if migration.name != row.name {
                    // The version and SQL are unchanged but the file was
                    // renamed. It is not an error (the migration is not
                    // re-run), but the stored history is now inconsistent.
                    tracing::warn!(
                        version = migration.version,
                        recorded = %row.name,
                        current = %migration.name,
                        "postgresql: a migration was renamed after it was applied"
                    );
                }
            }
        }

        let max_applied = applied.iter().map(|row| row.version).max();
        let pending: Vec<&Migration> = self
            .migrations
            .iter()
            .filter(|migration| !applied_by_version.contains_key(&migration.version))
            .collect();
        if !self.allow_out_of_order
            && let Some(max) = max_applied
            && let Some(migration) = pending.iter().find(|migration| migration.version < max)
        {
            return Err(Error::Migration(format!(
                "migration {} ({}) is out of order: version {} has already been applied",
                migration.version, migration.name, max
            )));
        }

        Ok(pending)
    }

    async fn apply_group(
        &self,
        conn: &Connection,
        table: &Table,
        group: &[&Migration],
        report: &mut Report,
    ) -> Result<(), Error> {
        let transaction = Transaction::begin(conn.clone()).await?;
        for migration in group {
            let started = Instant::now();
            if !migration.sql().trim().is_empty() {
                let outcome = transaction
                    .connection()
                    .execute_script_status(migration.sql())
                    .await
                    .map_err(|err| migration_error(migration, err))?;
                // A migration must not commit or roll back the group: the
                // server reports `I` (idle) once the transaction block is gone,
                // and the rest of the run would silently execute outside it.
                if outcome.transaction_status != b'T' {
                    return Err(Error::Migration(format!(
                        "migration {} ({}) ended the surrounding transaction (transaction status `{}`); \
                         migrations must not contain BEGIN, COMMIT or ROLLBACK",
                        migration.version, migration.name, outcome.transaction_status as char
                    )));
                }
            }
            let duration = started.elapsed();
            insert_applied(&transaction, table, migration, duration).await?;
            report.applied.push(AppliedMigration {
                version: migration.version,
                name: migration.name.clone(),
                duration,
            });
        }
        transaction
            .commit()
            .await
            .map_err(|err| Error::Migration(format!("failed to commit a migration group: {err}")))?;
        Ok(())
    }

    async fn apply_without_transaction(
        &self,
        conn: &Connection,
        table: &Table,
        migration: &Migration,
        report: &mut Report,
    ) -> Result<(), Error> {
        insert_running(conn, table, migration).await?;
        let started = Instant::now();
        if !migration.sql().trim().is_empty() {
            conn.execute_script(migration.sql())
                .await
                .map_err(|err| migration_error(migration, err))?;
        }
        let duration = started.elapsed();
        mark_applied(conn, table, migration, duration).await?;
        report.applied.push(AppliedMigration {
            version: migration.version,
            name: migration.name.clone(),
            duration,
        });
        Ok(())
    }
}

/// One migration recorded in the database.
struct AppliedRow {
    version: i64,
    name: String,
    checksum: [u8; 32],
    success: bool,
    applied_at: Option<time::DateTime>,
}

/// A migration that was applied during a [`Migrator::run`].
#[derive(Debug, Clone)]
pub struct AppliedMigration {
    /// The migration version.
    pub version: i64,
    /// The migration description.
    pub name: String,
    /// How long applying the migration took.
    pub duration: Duration,
}

/// The result of a [`Migrator::run`].
#[derive(Debug, Clone, Default)]
pub struct Report {
    /// Migrations applied by this run, in order.
    pub applied: Vec<AppliedMigration>,
    /// How many migrations were already recorded before the run.
    pub already_applied: usize,
}

/// The state of one migration in a [`StatusReport`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum MigrationState {
    /// The migration exists locally but has not been applied.
    Pending,
    /// The migration has been applied and matches the local file.
    Applied,
    /// The migration is recorded in the database but not present locally.
    Missing,
    /// The migration has been applied, but its checksum no longer matches.
    Modified,
    /// The migration is recorded as failed or incomplete.
    Dirty,
    /// The migration is pending but its version is lower than the highest
    /// applied version.
    OutOfOrder,
}

/// The state of one migration reported by [`Migrator::status`].
#[derive(Debug, Clone)]
pub struct MigrationStatus {
    /// The migration version.
    pub version: i64,
    /// The local or recorded description.
    pub name: String,
    /// The current state.
    pub state: MigrationState,
    /// When the migration was applied, if it is recorded in the database.
    pub applied_at: Option<time::DateTime>,
}

/// The result of a [`Migrator::status`].
#[derive(Debug, Clone, Default)]
pub struct StatusReport {
    /// Every local or recorded migration, sorted by version.
    pub migrations: Vec<MigrationStatus>,
    /// The highest applied version, if any.
    pub current_version: Option<i64>,
}

impl StatusReport {
    /// Returns `true` when every migration is applied and matches.
    pub fn is_up_to_date(&self) -> bool {
        self.migrations
            .iter()
            .all(|migration| migration.state == MigrationState::Applied)
    }
}

struct Table {
    schema: Option<String>,
    name: String,
}

impl Table {
    fn new(input: &str) -> Result<Self, Error> {
        if input.is_empty() || input.contains('\0') {
            return Err(Error::Migration(format!(
                "invalid migration table name {input:?}; expected `name` or `schema.name`"
            )));
        }

        let mut parts = input.split('.');
        let first = parts.next().unwrap_or_default();
        let second = parts.next();
        let third = parts.next();
        match (first, second, third) {
            (name, None, None) if !name.is_empty() => Ok(Self {
                schema: None,
                name: name.to_owned(),
            }),
            (schema, Some(name), None) if !schema.is_empty() && !name.is_empty() => Ok(Self {
                schema: Some(schema.to_owned()),
                name: name.to_owned(),
            }),
            _ => Err(Error::Migration(format!(
                "invalid migration table name {input:?}; expected `name` or `schema.name`"
            ))),
        }
    }

    fn qualified(&self) -> String {
        match &self.schema {
            Some(schema) => format!("{}.{}", quote_ident(schema), quote_ident(&self.name)),
            None => quote_ident(&self.name),
        }
    }
}

fn quote_ident(identifier: &str) -> String {
    let mut quoted = String::with_capacity(identifier.len() + 2);
    quoted.push('"');
    for character in identifier.chars() {
        if character == '"' {
            quoted.push('"');
        }
        quoted.push(character);
    }
    quoted.push('"');
    quoted
}

struct MigrationLock {
    conn: Connection,
    released: bool,
}

impl MigrationLock {
    async fn acquire(conn: &Connection, timeout: Duration) -> Result<Self, Error> {
        let started = Instant::now();
        let mut delay = LOCK_POLL_INTERVAL;
        loop {
            // Check the deadline before the attempt so the wait does not
            // overshoot by a whole round trip.
            if !timeout.is_zero() && started.elapsed() >= timeout {
                return Err(Error::Migration(format!(
                    "timed out after {timeout:?} waiting for the migration advisory lock"
                )));
            }
            let key = MIGRATION_LOCK_KEY;
            let params: [&dyn ToSql; 1] = [&key];
            let acquired: bool = QueryAs::<bool>::new("SELECT pg_try_advisory_lock($1)", &params)
                .fetch_one(conn)
                .await
                .map_err(|err| Error::Migration(format!("could not acquire the migration advisory lock: {err}")))?;
            if acquired {
                return Ok(Self {
                    conn: conn.clone(),
                    released: false,
                });
            }
            let sleep = if timeout.is_zero() {
                delay
            } else {
                timeout.saturating_sub(started.elapsed()).min(delay)
            };
            if sleep.is_zero() {
                continue;
            }
            tokio::time::sleep(sleep).await;
            delay = (delay * 2).min(Duration::from_secs(1));
        }
    }

    async fn release(mut self) -> Result<(), Error> {
        let sql = format!("SELECT pg_advisory_unlock({MIGRATION_LOCK_KEY})");
        self.conn
            .execute(&sql, &[])
            .await
            .map_err(|err| Error::Migration(format!("could not release the migration advisory lock: {err}")))?;
        self.released = true;
        Ok(())
    }
}

impl Drop for MigrationLock {
    fn drop(&mut self) {
        if !self.released {
            // `Transaction` normally rolls back on drop, but a cancelled or
            // panicking future may leave a transaction open. Issuing ROLLBACK
            // here is harmless when no transaction is active and guarantees
            // the session is not returned to the pool mid-transaction.
            self.conn.enqueue("ROLLBACK");
            let sql = format!("SELECT pg_advisory_unlock({MIGRATION_LOCK_KEY})");
            self.conn.enqueue(&sql);
        }
    }
}

fn starts_with_no_transaction(sql: &str) -> bool {
    sql.lines()
        .find(|line| !line.trim().is_empty())
        .is_some_and(|line| line.trim() == NO_TRANSACTION_MARKER)
}

fn checksum(sql: &str) -> [u8; 32] {
    let normalized = if sql.contains("\r\n") {
        Some(sql.replace("\r\n", "\n"))
    } else {
        None
    };
    let sql = normalized.as_deref().unwrap_or(sql);
    let hash = Blake3::hash(sql.as_bytes());
    let mut checksum = [0u8; 32];
    checksum.copy_from_slice(hash.as_ref());
    checksum
}

fn walk_dir(dir: &Path, migrations: &mut Vec<Migration>) -> Result<(), Error> {
    let entries = fs::read_dir(dir)
        .map_err(|err| Error::Migration(format!("cannot read migration directory {}: {err}", dir.display())))?;
    for entry in entries {
        let entry = entry
            .map_err(|err| Error::Migration(format!("cannot read migration directory {}: {err}", dir.display())))?;
        let path = entry.path();
        let file_type = entry
            .file_type()
            .map_err(|err| Error::Migration(format!("cannot read migration path {}: {err}", path.display())))?;
        if file_type.is_dir() {
            walk_dir(&path, migrations)?;
            continue;
        }

        let is_file = if file_type.is_file() {
            true
        } else if file_type.is_symlink() {
            fs::metadata(&path).is_ok_and(|metadata| metadata.is_file())
        } else {
            false
        };
        if !is_file || path.extension().and_then(|extension| extension.to_str()) != Some("sql") {
            continue;
        }

        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| Error::Migration(format!("migration filename is not valid UTF-8: {}", path.display())))?;
        migrations.push(load_file(&path, file_name)?);
    }
    Ok(())
}

fn load_file(path: &Path, file_name: &str) -> Result<Migration, Error> {
    let (version, name) = parse_file_name(file_name)
        .map_err(|message| Error::Migration(format!("invalid migration filename {}: {message}", path.display())))?;
    let sql = fs::read_to_string(path)
        .map_err(|err| Error::Migration(format!("cannot read migration {}: {err}", path.display())))?;
    Ok(Migration::new(version, name, sql).with_source(path.display().to_string()))
}

fn parse_file_name(file_name: &str) -> Result<(i64, String), String> {
    let stem = file_name
        .strip_suffix(".sql")
        .ok_or_else(|| "expected a `.sql` file".to_owned())?;
    let (version, description) = stem
        .split_once('_')
        .ok_or_else(|| format!("expected `<version>_<description>.sql`, got {file_name:?}"))?;
    if version.is_empty() || !version.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(format!("invalid version {version:?}; expected a non-negative integer"));
    }
    let version: i64 = version
        .parse()
        .map_err(|_| format!("invalid version {version:?}; expected a non-negative integer"))?;
    if description.is_empty() {
        return Err("the description after the version must not be empty".to_owned());
    }
    Ok((version, description.replace('_', " ")))
}

/// Creates the tracking table, upgrading one created by an older version of
/// this library.
async fn ensure_table(conn: &Connection, table: &Table) -> Result<(), Error> {
    let qualified = table.qualified();
    let sql = format!(
        "CREATE TABLE IF NOT EXISTS {qualified} (
            version BIGINT PRIMARY KEY,
            name TEXT NOT NULL,
            checksum BYTEA NOT NULL,
            applied_at TIMESTAMPTZ NOT NULL DEFAULT now(),
            execution_time_ms BIGINT NOT NULL DEFAULT 0,
            success BOOLEAN NOT NULL DEFAULT TRUE
        )"
    );
    conn.execute_script(&sql)
        .await
        .map_err(|err| Error::Migration(format!("could not create migration table {qualified}: {err}")))?;

    // `CREATE TABLE IF NOT EXISTS` leaves an existing table untouched. Add the
    // columns introduced later so a history written by an older version keeps
    // working instead of failing every query.
    for column in [
        "execution_time_ms BIGINT NOT NULL DEFAULT 0",
        "success BOOLEAN NOT NULL DEFAULT TRUE",
    ] {
        let sql = format!("ALTER TABLE {qualified} ADD COLUMN IF NOT EXISTS {column}");
        conn.execute_script(&sql)
            .await
            .map_err(|err| Error::Migration(format!("could not upgrade migration table {qualified}: {err}")))?;
    }
    Ok(())
}

async fn load_applied(conn: &Connection, table: &Table) -> Result<Vec<AppliedRow>, Error> {
    let qualified = table.qualified();
    let sql = format!("SELECT version, name, checksum, success, applied_at FROM {qualified} ORDER BY version");
    let rows: Vec<(i64, String, Vec<u8>, bool, Option<time::DateTime>)> = QueryAs::new(&sql, &[])
        .fetch_all(conn)
        .await
        .map_err(|err| Error::Migration(format!("could not read migration history from {qualified}: {err}")))?;

    rows.into_iter()
        .map(|(version, name, checksum, success, applied_at)| {
            let checksum: [u8; 32] = checksum.as_slice().try_into().map_err(|_| {
                Error::Migration(format!(
                    "migration {version} has an invalid checksum (expected 32 bytes, got {})",
                    checksum.len()
                ))
            })?;
            Ok(AppliedRow {
                version,
                name,
                checksum,
                success,
                applied_at,
            })
        })
        .collect()
}

async fn insert_applied<E: Executor>(
    executor: &E,
    table: &Table,
    migration: &Migration,
    duration: Duration,
) -> Result<(), Error> {
    let sql = format!(
        "INSERT INTO {} (version, name, checksum, execution_time_ms, success) VALUES ($1, $2, $3, $4, TRUE)",
        table.qualified()
    );
    let version = migration.version;
    let name = migration.name.as_str();
    let checksum = migration.checksum.to_vec();
    let millis = duration_to_millis(duration);
    let params: [&dyn ToSql; 4] = [&version, &name, &checksum, &millis];
    let affected = Query::new(&sql, &params)
        .execute(executor)
        .await
        .map_err(|err| Error::Migration(format!("could not record migration {version} ({name}): {err}")))?;
    if affected == 0 {
        return Err(Error::Migration(format!(
            "could not record migration {version} ({name}): the database did not insert a row"
        )));
    }
    Ok(())
}

async fn insert_running(conn: &Connection, table: &Table, migration: &Migration) -> Result<(), Error> {
    let sql = format!(
        "INSERT INTO {} (version, name, checksum, success) VALUES ($1, $2, $3, FALSE)",
        table.qualified()
    );
    let version = migration.version;
    let name = migration.name.as_str();
    let checksum = migration.checksum.to_vec();
    let params: [&dyn ToSql; 3] = [&version, &name, &checksum];
    let affected = Query::new(&sql, &params)
        .execute(conn)
        .await
        .map_err(|err| Error::Migration(format!("could not record migration {version} ({name}): {err}")))?;
    if affected == 0 {
        return Err(Error::Migration(format!(
            "could not record migration {version} ({name}): the database did not insert a row"
        )));
    }
    Ok(())
}

async fn mark_applied(
    conn: &Connection,
    table: &Table,
    migration: &Migration,
    duration: Duration,
) -> Result<(), Error> {
    let sql = format!(
        "UPDATE {} SET success = TRUE, applied_at = now(), execution_time_ms = $1 WHERE version = $2",
        table.qualified()
    );
    let version = migration.version;
    let millis = duration_to_millis(duration);
    let params: [&dyn ToSql; 2] = [&millis, &version];
    let affected = Query::new(&sql, &params)
        .execute(conn)
        .await
        .map_err(|err| Error::Migration(format!("could not record migration {version} ({}): {err}", migration.name)))?;
    if affected == 0 {
        return Err(Error::Migration(format!(
            "could not record migration {version} ({}): tracking row disappeared",
            migration.name
        )));
    }
    Ok(())
}

fn duration_to_millis(duration: Duration) -> i64 {
    i64::try_from(duration.as_millis()).unwrap_or(i64::MAX)
}

fn migration_error(migration: &Migration, error: impl fmt::Display) -> Error {
    Error::Migration(format!("migration {} ({}) failed: {error}", migration.version, migration.name))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn migration(version: i64, sql: &'static str) -> Migration {
        Migration::new(version, "test", sql)
    }

    fn applied(version: i64, checksum: [u8; 32]) -> AppliedRow {
        AppliedRow {
            version,
            name: format!("migration {version}"),
            checksum,
            success: true,
            applied_at: None,
        }
    }

    #[test]
    fn parses_versions_and_descriptions() {
        assert_eq!(parse_file_name("20250101_init.sql").unwrap(), (20250101, "init".to_owned()));
        assert_eq!(parse_file_name("1_add_users.sql").unwrap(), (1, "add users".to_owned()));
        assert_eq!(
            parse_file_name("0001_add_users_table.sql").unwrap(),
            (1, "add users table".to_owned())
        );
    }

    #[test]
    fn rejects_invalid_file_names() {
        assert!(parse_file_name("init.sql").is_err());
        assert!(parse_file_name("1_.sql").is_err());
        assert!(parse_file_name("-1_init.sql").is_err());
        assert!(parse_file_name("x_init.sql").is_err());
    }

    #[test]
    fn checksums_ignore_line_ending_style() {
        let lf = migration(1, "SELECT 1;\nSELECT 2;\n");
        let crlf = migration(1, "SELECT 1;\r\nSELECT 2;\r\n");
        assert_eq!(lf.checksum(), crlf.checksum());
        assert_ne!(lf.checksum(), migration(1, "SELECT 1;\n").checksum());
    }

    #[test]
    fn strips_a_leading_bom() {
        let migration = Migration::new(1, "bom", "\u{feff}SELECT 1;");
        assert_eq!(migration.sql(), "SELECT 1;");
        assert_eq!(migration.checksum(), Migration::new(1, "plain", "SELECT 1;").checksum());
    }

    #[test]
    fn defaults_are_safe() {
        let migrator = Migrator::default();
        assert_eq!(migrator.table_name, DEFAULT_TABLE_NAME);
        assert!(migrator.advisory_lock);
        assert!(migrator.verify_checksums);
        assert!(!migrator.allow_missing);
        assert!(!migrator.allow_out_of_order);
        assert!(migrator.migrations.is_empty());
    }

    #[test]
    fn loads_nested_directories() {
        let dir = std::env::temp_dir().join(format!(
            "stdx_pg_mig_unit_{}",
            uuid::Uuid::new_v4().to_string().replace('-', "")
        ));
        fs::create_dir_all(dir.join("2025")).unwrap();
        fs::create_dir_all(dir.join("2026")).unwrap();
        fs::write(dir.join("2025/1_a.sql"), "SELECT 1;").unwrap();
        fs::write(dir.join("2026/2_b.sql"), "SELECT 2;").unwrap();
        fs::write(dir.join("README.md"), "ignored").unwrap();

        let migrator = Migrator::from_dir(&dir).unwrap();
        assert_eq!(
            migrator
                .migrations()
                .iter()
                .map(|migration| migration.version())
                .collect::<Vec<_>>(),
            vec![1, 2]
        );

        fs::write(dir.join("2026/1_duplicate.sql"), "SELECT 3;").unwrap();
        assert!(Migrator::from_dir(&dir).is_err());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn detects_the_no_transaction_marker() {
        assert!(Migration::new(1, "a", "SELECT 1").is_transactional());
        assert!(!Migration::new(1, "a", "-- migrate:no-transaction\nSELECT 1").is_transactional());
        assert!(!Migration::new(1, "a", "SELECT 1").no_transaction().is_transactional());
        assert!(Migration::new(1, "a", "-- a comment\nSELECT 1").is_transactional());
    }

    #[test]
    fn rejects_duplicate_and_invalid_migrations() {
        assert!(Migrator::from_migrations([migration(1, "SELECT 1"), migration(1, "SELECT 2")]).is_err());
        assert!(Migrator::from_migrations([migration(-1, "SELECT 1")]).is_err());
        assert!(Migrator::from_migrations([Migration::new(1, "", "SELECT 1")]).is_err());
    }

    #[test]
    fn quotes_table_identifiers() {
        assert_eq!(Table::new("migrations").unwrap().qualified(), "\"migrations\"");
        assert_eq!(Table::new("app.migrations").unwrap().qualified(), "\"app\".\"migrations\"");
        assert_eq!(Table::new("weird\"name").unwrap().qualified(), "\"weird\"\"name\"");
        assert!(Table::new("a.b.c").is_err());
        assert!(Table::new("").is_err());
    }

    #[test]
    fn validates_history() {
        let migrator = Migrator::from_migrations([
            migration(1, "SELECT 1"),
            migration(2, "SELECT 2"),
            migration(3, "SELECT 3"),
        ])
        .unwrap();

        let first = migrator.migrations[0].checksum;
        assert!(migrator.validate(&[applied(1, first)]).is_ok());

        assert!(
            migrator
                .validate(&[
                    applied(1, migrator.migrations[0].checksum),
                    applied(2, migrator.migrations[1].checksum),
                    applied(3, migrator.migrations[2].checksum),
                ])
                .is_ok()
        );

        let missing = migrator.validate(&[applied(4, [0; 32])]).unwrap_err();
        assert!(missing.to_string().contains("missing"));

        let mismatch = migrator.validate(&[applied(1, [0; 32])]).unwrap_err();
        assert!(mismatch.to_string().contains("checksum mismatch"));

        let out_of_order = migrator
            .validate(&[
                applied(2, migrator.migrations[1].checksum),
                applied(3, migrator.migrations[2].checksum),
            ])
            .unwrap_err();
        assert!(out_of_order.to_string().contains("out of order"));
    }

    #[test]
    fn relaxed_validation_options_work() {
        let migrator = Migrator::from_migrations([migration(1, "SELECT 1"), migration(2, "SELECT 2")])
            .unwrap()
            .verify_checksums(false)
            .allow_missing(true)
            .allow_out_of_order(true);

        assert!(migrator.validate(&[applied(2, [0; 32]), applied(4, [0; 32])]).is_ok());
    }

    #[test]
    fn dirty_rows_are_rejected() {
        let migrator = Migrator::from_migrations([migration(1, "SELECT 1")]).unwrap();
        let mut row = applied(1, migrator.migrations[0].checksum);
        row.success = false;
        let error = migrator.validate(&[row]).unwrap_err();
        assert!(error.to_string().contains("marked as failed"));
    }
}
