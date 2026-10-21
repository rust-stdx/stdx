//! Integration tests for the forward-only migration helpers.
//!
//! The database tests use `DATABASE_URL` and are skipped when it is not set.
//! Every test uses unique table names so the tests can run in parallel.

use std::{
    fs,
    path::{Path, PathBuf},
};

use postgresql::{
    Connection, Pool, QueryAs, ScriptExecutor, ToSql,
    migrate::{Migration, MigrationState, Migrator},
};
use uuid::Uuid;

fn database_url() -> Option<String> {
    match std::env::var("DATABASE_URL") {
        Ok(url) if !url.is_empty() => Some(url),
        _ => None,
    }
}

async fn pool_or_skip() -> Option<Pool> {
    let url = match database_url() {
        Some(url) => url,
        None => {
            eprintln!("skipping test: DATABASE_URL is not set");
            return None;
        }
    };
    Some(Pool::connect(&url, None).await.expect("connect"))
}

fn unique(prefix: &str) -> String {
    format!("{prefix}_{}", Uuid::new_v4().to_string().replace('-', "_"))
}

struct TempDir {
    path: PathBuf,
}

impl TempDir {
    fn new() -> Self {
        let path =
            std::env::temp_dir().join(format!("stdx_pg_migrations_{}", Uuid::new_v4().to_string().replace('-', "")));
        fs::create_dir_all(&path).unwrap();
        Self {
            path,
        }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn write_migration(dir: &Path, relative: &str, sql: &str) {
    let path = dir.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, sql).unwrap();
}

async fn regclass(conn: &Connection, name: &str) -> Option<String> {
    let params: [&dyn ToSql; 1] = [&name];
    QueryAs::<Option<String>>::new("SELECT to_regclass($1)::text", &params)
        .fetch_one(conn)
        .await
        .expect("to_regclass")
}

#[tokio::test]
async fn filesystem_subfolders_apply_and_status() {
    let Some(pool) = pool_or_skip().await else {
        return;
    };
    let dir = TempDir::new();
    let table_a = unique("mig_a");
    let table_b = unique("mig_b");
    let migrations_table = unique("stdx_migrations");

    write_migration(
        dir.path(),
        "2025/20250101_create_a.sql",
        &format!("CREATE TABLE {table_a} (id INT PRIMARY KEY);"),
    );
    write_migration(
        dir.path(),
        "2026/20260101_create_b.sql",
        &format!("CREATE TABLE {table_b} (id INT PRIMARY KEY);"),
    );

    let migrator = Migrator::from_dir(dir.path())
        .unwrap()
        .table_name(migrations_table.clone());
    assert_eq!(migrator.migrations().len(), 2);
    assert_eq!(migrator.migrations()[0].version(), 20250101);
    assert_eq!(migrator.migrations()[1].version(), 20260101);

    let report = migrator.run(&pool).await.unwrap();
    assert_eq!(report.applied.len(), 2);
    assert_eq!(report.already_applied, 0);

    let status = migrator.status(&pool).await.unwrap();
    assert!(status.is_up_to_date());
    assert_eq!(status.current_version, Some(20260101));

    let conn = pool.get().await.unwrap();
    assert!(regclass(&conn, &table_a).await.is_some());
    assert!(regclass(&conn, &table_b).await.is_some());
    drop(conn);

    let conn = pool.get().await.unwrap();
    conn.execute_script(&format!(
        "DROP TABLE IF EXISTS {table_a}; DROP TABLE IF EXISTS {table_b}; DROP TABLE IF EXISTS {migrations_table};"
    ))
    .await
    .unwrap();
}

#[tokio::test]
async fn whole_run_transaction_rolls_back_on_failure() {
    let Some(pool) = pool_or_skip().await else {
        return;
    };
    let dir = TempDir::new();
    let table_a = unique("tx_a");
    let table_b = unique("tx_b");
    let migrations_table = unique("stdx_migrations");

    write_migration(
        dir.path(),
        "1_create_a.sql",
        &format!("CREATE TABLE {table_a} (id INT PRIMARY KEY);"),
    );
    write_migration(
        dir.path(),
        "2_fail.sql",
        &format!("CREATE TABLE {table_b} (id INT PRIMARY KEY); SELECT 1/0;"),
    );

    let migrator = Migrator::from_dir(dir.path())
        .unwrap()
        .table_name(migrations_table.clone());
    let error = migrator.run(&pool).await.unwrap_err();
    assert!(error.to_string().contains("migration 2"), "{error}");

    let status = migrator.status(&pool).await.unwrap();
    assert!(!status.is_up_to_date());
    assert!(
        status
            .migrations
            .iter()
            .all(|migration| migration.state == MigrationState::Pending)
    );

    let conn = pool.get().await.unwrap();
    assert!(regclass(&conn, &table_a).await.is_none());
    assert!(regclass(&conn, &table_b).await.is_none());
    drop(conn);

    write_migration(
        dir.path(),
        "2_fail.sql",
        &format!("CREATE TABLE {table_b} (id INT PRIMARY KEY);"),
    );
    let fixed = Migrator::from_dir(dir.path())
        .unwrap()
        .table_name(migrations_table.clone());
    let report = fixed.run(&pool).await.unwrap();
    assert_eq!(report.applied.len(), 2);

    let conn = pool.get().await.unwrap();
    assert!(regclass(&conn, &table_a).await.is_some());
    assert!(regclass(&conn, &table_b).await.is_some());
    conn.execute_script(&format!(
        "DROP TABLE IF EXISTS {table_a}; DROP TABLE IF EXISTS {table_b}; DROP TABLE IF EXISTS {migrations_table};"
    ))
    .await
    .unwrap();
}

#[tokio::test]
async fn no_transaction_migration_splits_the_run() {
    let Some(pool) = pool_or_skip().await else {
        return;
    };
    let dir = TempDir::new();
    let items = unique("items");
    let other = unique("other");
    let index = unique("items_value_idx");
    let migrations_table = unique("stdx_migrations");

    write_migration(
        dir.path(),
        "1_create_items.sql",
        &format!("CREATE TABLE {items} (id INT PRIMARY KEY, value INT);"),
    );
    write_migration(
        dir.path(),
        "2_create_index.sql",
        &format!("-- migrate:no-transaction\nCREATE INDEX CONCURRENTLY {index} ON {items} (value);"),
    );
    write_migration(
        dir.path(),
        "3_create_other.sql",
        &format!("CREATE TABLE {other} (id INT PRIMARY KEY);"),
    );

    let migrator = Migrator::from_dir(dir.path())
        .unwrap()
        .table_name(migrations_table.clone());
    let report = migrator.run(&pool).await.unwrap();
    assert_eq!(report.applied.len(), 3);

    let status = migrator.status(&pool).await.unwrap();
    assert!(status.is_up_to_date());

    let conn = pool.get().await.unwrap();
    assert!(regclass(&conn, &index).await.is_some());
    conn.execute_script(&format!(
        "DROP TABLE IF EXISTS {items}; DROP TABLE IF EXISTS {other}; DROP TABLE IF EXISTS {migrations_table};"
    ))
    .await
    .unwrap();
}

#[tokio::test]
async fn failed_no_transaction_migration_is_dirty() {
    let Some(pool) = pool_or_skip().await else {
        return;
    };
    let dir = TempDir::new();
    let migrations_table = unique("stdx_migrations");

    write_migration(dir.path(), "1_dirty.sql", "-- migrate:no-transaction\nSELECT 1/0;");

    let migrator = Migrator::from_dir(dir.path())
        .unwrap()
        .table_name(migrations_table.clone());
    let error = migrator.run(&pool).await.unwrap_err();
    assert!(error.to_string().contains("migration 1"), "{error}");

    let status = migrator.status(&pool).await.unwrap();
    assert_eq!(status.migrations.len(), 1);
    assert_eq!(status.migrations[0].state, MigrationState::Dirty);

    let error = migrator.run(&pool).await.unwrap_err();
    assert!(error.to_string().contains("marked as failed"), "{error}");

    let conn = pool.get().await.unwrap();
    conn.execute_script(&format!("DROP TABLE IF EXISTS {migrations_table};"))
        .await
        .unwrap();
}

#[tokio::test]
async fn checksum_mismatch_can_be_disabled() {
    let Some(pool) = pool_or_skip().await else {
        return;
    };
    let dir = TempDir::new();
    let table = unique("checksum_table");
    let migrations_table = unique("stdx_migrations");
    let sql = format!("CREATE TABLE {table} (id INT PRIMARY KEY);");

    write_migration(dir.path(), "1_init.sql", &sql);
    let migrator = Migrator::from_dir(dir.path())
        .unwrap()
        .table_name(migrations_table.clone());
    migrator.run(&pool).await.unwrap();

    write_migration(dir.path(), "1_init.sql", &format!("{sql}\n-- edited\n"));
    let edited = Migrator::from_dir(dir.path())
        .unwrap()
        .table_name(migrations_table.clone());
    let error = edited.run(&pool).await.unwrap_err();
    assert!(error.to_string().contains("checksum mismatch"), "{error}");

    let status = edited.status(&pool).await.unwrap();
    assert_eq!(status.migrations[0].state, MigrationState::Modified);

    let relaxed = Migrator::from_dir(dir.path())
        .unwrap()
        .table_name(migrations_table.clone())
        .verify_checksums(false);
    let report = relaxed.run(&pool).await.unwrap();
    assert!(report.applied.is_empty());

    let status = relaxed.status(&pool).await.unwrap();
    assert_eq!(status.migrations[0].state, MigrationState::Applied);

    let conn = pool.get().await.unwrap();
    conn.execute_script(&format!(
        "DROP TABLE IF EXISTS {table}; DROP TABLE IF EXISTS {migrations_table};"
    ))
    .await
    .unwrap();
}

#[tokio::test]
async fn missing_applied_migration_is_reported() {
    let Some(pool) = pool_or_skip().await else {
        return;
    };
    let dir = TempDir::new();
    let table_a = unique("missing_a");
    let table_b = unique("missing_b");
    let migrations_table = unique("stdx_migrations");

    write_migration(
        dir.path(),
        "1_init.sql",
        &format!("CREATE TABLE {table_a} (id INT PRIMARY KEY);"),
    );
    write_migration(
        dir.path(),
        "2_second.sql",
        &format!("CREATE TABLE {table_b} (id INT PRIMARY KEY);"),
    );
    Migrator::from_dir(dir.path())
        .unwrap()
        .table_name(migrations_table.clone())
        .run(&pool)
        .await
        .unwrap();

    fs::remove_file(dir.path().join("1_init.sql")).unwrap();
    let missing = Migrator::from_dir(dir.path())
        .unwrap()
        .table_name(migrations_table.clone());
    let error = missing.run(&pool).await.unwrap_err();
    assert!(error.to_string().contains("missing"), "{error}");

    let status = missing.status(&pool).await.unwrap();
    assert!(
        status
            .migrations
            .iter()
            .any(|migration| migration.version == 1 && migration.state == MigrationState::Missing)
    );

    let allowed = Migrator::from_dir(dir.path())
        .unwrap()
        .table_name(migrations_table.clone())
        .allow_missing(true);
    assert!(allowed.run(&pool).await.unwrap().applied.is_empty());

    let conn = pool.get().await.unwrap();
    conn.execute_script(&format!(
        "DROP TABLE IF EXISTS {table_a}; DROP TABLE IF EXISTS {table_b}; DROP TABLE IF EXISTS {migrations_table};"
    ))
    .await
    .unwrap();
}

#[tokio::test]
async fn out_of_order_migrations_are_rejected_by_default() {
    let Some(pool) = pool_or_skip().await else {
        return;
    };
    let dir = TempDir::new();
    let table_a = unique("order_a");
    let table_b = unique("order_b");
    let migrations_table = unique("stdx_migrations");

    write_migration(
        dir.path(),
        "2_second.sql",
        &format!("CREATE TABLE {table_b} (id INT PRIMARY KEY);"),
    );
    Migrator::from_dir(dir.path())
        .unwrap()
        .table_name(migrations_table.clone())
        .run(&pool)
        .await
        .unwrap();

    write_migration(
        dir.path(),
        "1_first.sql",
        &format!("CREATE TABLE {table_a} (id INT PRIMARY KEY);"),
    );
    let strict = Migrator::from_dir(dir.path())
        .unwrap()
        .table_name(migrations_table.clone());
    let status = strict.status(&pool).await.unwrap();
    assert!(
        status
            .migrations
            .iter()
            .any(|migration| migration.version == 1 && migration.state == MigrationState::OutOfOrder)
    );

    let error = strict.run(&pool).await.unwrap_err();
    assert!(error.to_string().contains("out of order"), "{error}");

    let relaxed = Migrator::from_dir(dir.path())
        .unwrap()
        .table_name(migrations_table.clone())
        .allow_out_of_order(true);
    let report = relaxed.run(&pool).await.unwrap();
    assert_eq!(report.applied.len(), 1);
    assert_eq!(report.applied[0].version, 1);

    let conn = pool.get().await.unwrap();
    conn.execute_script(&format!(
        "DROP TABLE IF EXISTS {table_a}; DROP TABLE IF EXISTS {table_b}; DROP TABLE IF EXISTS {migrations_table};"
    ))
    .await
    .unwrap();
}

#[tokio::test]
async fn status_does_not_create_the_tracking_table() {
    let Some(pool) = pool_or_skip().await else {
        return;
    };
    let dir = TempDir::new();
    let migrations_table = unique("stdx_migrations");

    write_migration(dir.path(), "1_init.sql", "SELECT 1;");
    let migrator = Migrator::from_dir(dir.path())
        .unwrap()
        .table_name(migrations_table.clone());
    let status = migrator.status(&pool).await.unwrap();
    assert_eq!(status.current_version, None);
    assert_eq!(status.migrations.len(), 1);
    assert_eq!(status.migrations[0].state, MigrationState::Pending);
    assert!(!status.is_up_to_date());

    let conn = pool.get().await.unwrap();
    assert!(regclass(&conn, &migrations_table).await.is_none());
}

#[tokio::test]
async fn concurrent_runners_serialize() {
    let Some(pool) = pool_or_skip().await else {
        return;
    };
    let dir = TempDir::new();
    let table = unique("concurrent_table");
    let migrations_table = unique("stdx_migrations");

    write_migration(
        dir.path(),
        "1_init.sql",
        &format!("CREATE TABLE {table} (id INT PRIMARY KEY); SELECT pg_sleep(0.2);"),
    );

    let first = Migrator::from_dir(dir.path())
        .unwrap()
        .table_name(migrations_table.clone());
    let second = Migrator::from_dir(dir.path())
        .unwrap()
        .table_name(migrations_table.clone());
    let (first_report, second_report) = tokio::join!(first.run(&pool), second.run(&pool));
    let first_report = first_report.unwrap();
    let second_report = second_report.unwrap();
    assert_eq!(first_report.applied.len() + second_report.applied.len(), 1);

    let status = first.status(&pool).await.unwrap();
    assert!(status.is_up_to_date());

    let conn = pool.get().await.unwrap();
    assert!(regclass(&conn, &table).await.is_some());
    conn.execute_script(&format!(
        "DROP TABLE IF EXISTS {table}; DROP TABLE IF EXISTS {migrations_table};"
    ))
    .await
    .unwrap();
}

#[test]
fn manual_migrations_are_sorted() {
    let migrator = Migrator::from_migrations([
        Migration::new(2, "second", "SELECT 2;"),
        Migration::new(1, "first", "SELECT 1;"),
    ])
    .unwrap();
    let versions = migrator
        .migrations()
        .iter()
        .map(|migration| migration.version())
        .collect::<Vec<_>>();
    assert_eq!(versions, vec![1, 2]);
}

#[cfg(feature = "embed")]
mod embedded {
    use embed::Embed;

    use super::*;

    #[derive(Embed)]
    #[folder = "tests/fixtures/migrations"]
    struct EmbeddedMigrations;

    #[test]
    fn loads_embedded_migrations() {
        let migrator = Migrator::from_embed::<EmbeddedMigrations>().unwrap();
        let versions = migrator
            .migrations()
            .iter()
            .map(|migration| migration.version())
            .collect::<Vec<_>>();
        assert_eq!(versions, vec![1, 2]);
        assert_eq!(migrator.migrations()[0].name(), "init");
        assert_eq!(migrator.migrations()[1].name(), "update");
    }
}

/// A failed non-transactional migration can be repaired explicitly and then
/// applied again.
#[tokio::test]
async fn repair_failed_allows_a_failed_migration_to_run_again() {
    let Some(pool) = pool_or_skip().await else {
        return;
    };
    let dir = TempDir::new();
    let migrations_table = unique("stdx_migrations");
    let table = unique("repair_table");

    write_migration(dir.path(), "1_fail.sql", "-- migrate:no-transaction\nSELECT 1/0;");
    let migrator = Migrator::from_dir(dir.path())
        .unwrap()
        .table_name(migrations_table.clone());
    assert!(migrator.run(&pool).await.is_err());
    assert_eq!(migrator.status(&pool).await.unwrap().migrations[0].state, MigrationState::Dirty);

    assert!(migrator.repair_failed(&pool, 1).await.unwrap());
    assert!(!migrator.repair_failed(&pool, 1).await.unwrap(), "the row is gone");

    write_migration(
        dir.path(),
        "1_fail.sql",
        &format!("-- migrate:no-transaction\nCREATE TABLE {table} (id INT);"),
    );
    let fixed = Migrator::from_dir(dir.path())
        .unwrap()
        .table_name(migrations_table.clone());
    let report = fixed.run(&pool).await.unwrap();
    assert_eq!(report.applied.len(), 1);

    let conn = pool.get().await.unwrap();
    assert!(regclass(&conn, &table).await.is_some());
    conn.execute_script(&format!(
        "DROP TABLE IF EXISTS {table}; DROP TABLE IF EXISTS {migrations_table};"
    ))
    .await
    .unwrap();
}

/// A migration that commits or rolls back the surrounding transaction is
/// reported instead of silently letting the rest of the group run outside it.
#[tokio::test]
async fn a_migration_that_ends_the_transaction_is_rejected() {
    let Some(pool) = pool_or_skip().await else {
        return;
    };
    let dir = TempDir::new();
    let first = unique("commit_tx");
    let second = unique("commit_after");
    let migrations_table = unique("stdx_migrations");

    write_migration(dir.path(), "1_commit.sql", &format!("CREATE TABLE {first} (id INT); COMMIT;"));
    write_migration(dir.path(), "2_after.sql", &format!("CREATE TABLE {second} (id INT);"));
    let migrator = Migrator::from_dir(dir.path())
        .unwrap()
        .table_name(migrations_table.clone());
    let error = migrator.run(&pool).await.unwrap_err();
    assert!(error.to_string().contains("transaction"), "{error}");

    // Nothing after the commit may run, and migration 1 was not recorded.
    let status = migrator.status(&pool).await.unwrap();
    assert!(
        status
            .migrations
            .iter()
            .all(|migration| migration.state == MigrationState::Pending),
        "unexpected status: {status:?}"
    );

    let conn = pool.get().await.unwrap();
    assert!(
        regclass(&conn, &second).await.is_none(),
        "a migration after the commit must not run"
    );
    conn.execute_script(&format!(
        "DROP TABLE IF EXISTS {first}; DROP TABLE IF EXISTS {second}; DROP TABLE IF EXISTS {migrations_table};"
    ))
    .await
    .unwrap();
}

/// A tracking table created before `success`/`execution_time_ms` existed is
/// upgraded instead of failing every query.
#[tokio::test]
async fn an_older_tracking_table_is_upgraded() {
    let Some(pool) = pool_or_skip().await else {
        return;
    };
    let dir = TempDir::new();
    let migrations_table = unique("legacy_migrations");
    let table = unique("legacy_table");

    let conn = pool.get().await.unwrap();
    conn.execute_script(&format!(
        "CREATE TABLE {migrations_table} (
            version BIGINT PRIMARY KEY,
            name TEXT NOT NULL,
            checksum BYTEA NOT NULL,
            applied_at TIMESTAMPTZ NOT NULL DEFAULT now()
        )"
    ))
    .await
    .unwrap();
    drop(conn);

    write_migration(dir.path(), "1_init.sql", &format!("CREATE TABLE {table} (id INT);"));
    let report = Migrator::from_dir(dir.path())
        .unwrap()
        .table_name(migrations_table.clone())
        .run(&pool)
        .await
        .unwrap();
    assert_eq!(report.applied.len(), 1);

    let conn = pool.get().await.unwrap();
    let params: [&dyn ToSql; 1] = [&migrations_table];
    let columns: i64 = QueryAs::<i64>::new(
        "SELECT count(*)::int8 FROM pg_catalog.pg_attribute a \
         JOIN pg_catalog.pg_class c ON c.oid = a.attrelid \
         WHERE c.relname = $1 AND a.attname IN ('success', 'execution_time_ms') AND a.attnum > 0",
        &params,
    )
    .fetch_one(&conn)
    .await
    .unwrap();
    assert_eq!(columns, 2, "the tracking table was not upgraded");
    conn.execute_script(&format!(
        "DROP TABLE IF EXISTS {table}; DROP TABLE IF EXISTS {migrations_table};"
    ))
    .await
    .unwrap();
}

/// `schema.table` tracking names are quoted and resolved correctly.
#[tokio::test]
async fn schema_qualified_tracking_table_names_work() {
    let Some(pool) = pool_or_skip().await else {
        return;
    };
    let dir = TempDir::new();
    let schema = unique("mig_schema");
    let table = unique("schema_table");

    let conn = pool.get().await.unwrap();
    conn.execute_script(&format!("CREATE SCHEMA {schema}")).await.unwrap();
    drop(conn);

    write_migration(dir.path(), "1_init.sql", &format!("CREATE TABLE {table} (id INT);"));
    let qualified = format!("{schema}.migrations");
    let report = Migrator::from_dir(dir.path())
        .unwrap()
        .table_name(qualified.clone())
        .run(&pool)
        .await
        .unwrap();
    assert_eq!(report.applied.len(), 1);

    let conn = pool.get().await.unwrap();
    assert!(regclass(&conn, &qualified).await.is_some());
    conn.execute_script(&format!("DROP SCHEMA {schema} CASCADE; DROP TABLE IF EXISTS {table};"))
        .await
        .unwrap();
}
