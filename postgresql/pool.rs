//! A bounded connection pool.

use std::{
    future::Future,
    ops::Deref,
    sync::{
        Arc, Mutex, Weak,
        atomic::{AtomicBool, Ordering},
    },
    time::Instant,
};

use tokio::sync::{Notify, OwnedSemaphorePermit, Semaphore};

use crate::{
    config::{Config, PoolConfig},
    connection::{Connection, RowStream},
    encode::ToSql,
    error::Error,
    transaction::Transaction,
};

pub(crate) struct IdleConn {
    pub(crate) conn: Connection,
    pub(crate) created: Instant,
    pub(crate) idle_since: Instant,
    pub(crate) permit: OwnedSemaphorePermit,
}

pub(crate) struct PoolInner {
    config: Config,
    settings: PoolConfig,
    idle: Mutex<Vec<IdleConn>>,
    semaphore: Arc<Semaphore>,
    closed: AtomicBool,
    notify: Notify,
}

impl PoolInner {
    fn lock_idle(&self) -> std::sync::MutexGuard<'_, Vec<IdleConn>> {
        self.idle.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    pub(crate) fn pop_idle(&self) -> Option<IdleConn> {
        let mut idle = self.lock_idle();
        loop {
            let conn = idle.pop()?;
            if conn.conn.is_closed() {
                // Drop the dead connection (releasing its permit) and try the
                // next one.
                continue;
            }
            return Some(conn);
        }
    }

    pub(crate) fn push_idle(&self, conn: Connection, permit: OwnedSemaphorePermit, created: Instant) {
        {
            let mut idle = self.lock_idle();
            if self.closed.load(Ordering::Acquire) {
                // The pool was closed: drop the connection (and release its
                // permit) instead of storing it in a pool that will never hand
                // it out again.
                return;
            }
            idle.push(IdleConn {
                conn,
                created,
                idle_since: Instant::now(),
                permit,
            });
        }
        self.notify.notify_one();
    }

    pub(crate) fn is_closed(&self) -> bool {
        self.closed.load(Ordering::Acquire)
    }
}

/// Ownership of one checked-out connection.
///
/// The connection is returned to its pool — or discarded when the pool is
/// closed or the connection is broken — when the **last** clone of the `Arc`
/// is dropped. A [`PooledConnection`] or [`Transaction`] holds one clone and
/// every [`RowStream`] taken from it holds another, so a connection can never
/// be handed to another user while rows are still being streamed from it.
pub(crate) struct Checkout {
    conn: Option<Connection>,
    pool: Option<Arc<PoolInner>>,
    permit: Option<OwnedSemaphorePermit>,
    created: Instant,
}

impl Checkout {
    /// A checkout backed by a pool.
    pub(crate) fn pooled(
        conn: Connection,
        pool: Arc<PoolInner>,
        permit: OwnedSemaphorePermit,
        created: Instant,
    ) -> Self {
        Checkout {
            conn: Some(conn),
            pool: Some(pool),
            permit: Some(permit),
            created,
        }
    }

    /// A checkout that is not backed by a pool (a bare `Transaction`).
    pub(crate) fn detached(conn: Connection) -> Self {
        Checkout {
            conn: Some(conn),
            pool: None,
            permit: None,
            created: Instant::now(),
        }
    }

    pub(crate) fn connection(&self) -> &Connection {
        self.conn.as_ref().expect("checkout already returned")
    }
}

impl Drop for Checkout {
    fn drop(&mut self) {
        let Some(conn) = self.conn.take() else { return };
        match self.pool.take() {
            Some(pool) => {
                let Some(permit) = self.permit.take() else { return };
                if pool.is_closed() || conn.is_closed() {
                    // Discard the connection (releasing the permit) rather than
                    // storing it in a pool that will never reuse it.
                    drop(permit);
                    drop(conn);
                } else {
                    pool.push_idle(conn, permit, self.created);
                }
            }
            None => drop(conn),
        }
    }
}

/// A pool of reusable connections.
#[derive(Clone)]
pub struct Pool {
    inner: Arc<PoolInner>,
}

impl Pool {
    /// Creates a pool from a `postgres://` URL.
    ///
    /// Pool settings are read from the URL (`pool_size`, `min_connections`,
    /// `acquire_timeout`, `idle_timeout`, `max_lifetime`); when `config` is
    /// `Some`, it overrides them entirely.
    ///
    /// # TLS
    ///
    /// TLS is always required; plaintext connections are not possible.
    ///
    /// | `sslmode` | Chain verified | Hostname verified |
    /// | --- | --- | --- |
    /// | `require` (default) | no | no |
    /// | `verify-ca` | yes | no |
    /// | `verify-full` | yes | yes |
    ///
    /// `disable`, `prefer` and `allow` are rejected. `verify-ca` and
    /// `verify-full` read trusted roots from `sslrootcert` (a PEM file) or the
    /// operating system trust store.
    ///
    /// ```no_run
    /// # async fn run() -> Result<(), postgresql::Error> {
    /// use postgresql::{Pool, PoolConfig};
    ///
    /// // Pool of up to 16 connections, TLS required (default).
    /// let pool = Pool::connect("postgres://user:pass@db.example.com/app?pool_size=16", None).await?;
    ///
    /// // Verify the certificate chain and hostname; override the pool size.
    /// let other = Pool::connect(
    ///     "postgres://user:pass@db.example.com/app?sslmode=verify-full&sslrootcert=/etc/ssl/db-ca.pem",
    ///     Some(PoolConfig { pool_size: 32, min_connections: 2, ..PoolConfig::default() }),
    /// )
    /// .await?;
    /// # let _ = (pool, other);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a configuration error for an invalid URL. Opening the
    /// `min_connections` is **best-effort**: if some cannot be established the
    /// pool starts with fewer and opens more on demand (the minimum is not
    /// maintained afterwards).
    pub async fn connect(url: &str, config: Option<PoolConfig>) -> Result<Self, Error> {
        let mut parsed = Config::parse(url)?;
        if let Some(overrides) = config {
            // The explicit pool config wins over the URL for pool-wide settings.
            parsed.max_message_len = overrides.max_message_len;
            parsed.pool = overrides;
        }
        Pool::from_config(parsed).await
    }

    async fn from_config(config: Config) -> Result<Self, Error> {
        let settings = config.pool.clone();
        let pool = Pool {
            inner: Arc::new(PoolInner {
                config,
                settings: settings.clone(),
                idle: Mutex::new(Vec::new()),
                semaphore: Arc::new(Semaphore::new(settings.pool_size as usize)),
                closed: AtomicBool::new(false),
                notify: Notify::new(),
            }),
        };

        for _ in 0..settings.min_connections {
            let permit = match pool.inner.semaphore.clone().acquire_owned().await {
                Ok(permit) => permit,
                Err(_) => break,
            };
            match Connection::connect_with_config(pool.inner.config.clone()).await {
                Ok(conn) => pool.inner.push_idle(conn, permit, Instant::now()),
                Err(_) => break,
            }
        }

        // The reaper only holds a `Weak` reference, so dropping the last
        // `Pool` handle (without `close`) ends the task and the idle
        // connections with it instead of leaking both forever.
        tokio::spawn(Self::reap_loop(Arc::downgrade(&pool.inner)));

        Ok(pool)
    }

    /// Acquires a connection, waiting up to the configured acquire timeout.
    ///
    /// # Errors
    ///
    /// Returns [`Error::PoolClosed`] if the pool is closed,
    /// [`Error::PoolTimedOut`] if no connection becomes available in time, or a
    /// connection error if a new connection cannot be established.
    pub async fn get(&self) -> Result<PooledConnection, Error> {
        if self.inner.is_closed() {
            return Err(Error::PoolClosed);
        }

        loop {
            if let Some(idle) = self.inner.pop_idle() {
                return Ok(self.wrap_idle(idle));
            }

            let notified = self.inner.notify.notified();

            if self.inner.is_closed() {
                return Err(Error::PoolClosed);
            }
            if let Some(idle) = self.inner.pop_idle() {
                return Ok(self.wrap_idle(idle));
            }

            tokio::select! {
                _ = notified => continue,
                acquired = tokio::time::timeout(
                    self.inner.settings.acquire_timeout,
                    self.inner.semaphore.clone().acquire_owned(),
                ) => {
                    let permit = acquired
                        .map_err(|_| Error::PoolTimedOut)?
                        .map_err(|_| Error::PoolClosed)?;

                    // A connection may have been returned while we waited.
                    if let Some(idle) = self.inner.pop_idle() {
                        drop(permit);
                        return Ok(self.wrap_idle(idle));
                    }

                    let conn = Connection::connect_with_config(self.inner.config.clone()).await?;
                    return Ok(PooledConnection {
                        checkout: Some(Arc::new(Checkout::pooled(
                            conn,
                            self.inner.clone(),
                            permit,
                            Instant::now(),
                        ))),
                    });
                }
            }
        }
    }

    fn wrap_idle(&self, idle: IdleConn) -> PooledConnection {
        PooledConnection {
            checkout: Some(Arc::new(Checkout::pooled(
                idle.conn,
                self.inner.clone(),
                idle.permit,
                idle.created,
            ))),
        }
    }

    /// Acquires a connection and starts a transaction on it.
    pub async fn begin(&self) -> Result<Transaction, Error> {
        let pooled = self.get().await?;
        pooled.begin().await
    }

    /// Returns `true` if the pool has been closed.
    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }

    /// Closes the pool and discards idle connections.
    ///
    /// Connections still checked out are closed when they are returned, not
    /// reused.
    pub fn close(&self) {
        self.inner.closed.store(true, Ordering::Release);
        self.inner.semaphore.close();
        self.inner.lock_idle().clear();
        self.inner.notify.notify_waiters();
    }

    async fn reap_loop(inner: Weak<PoolInner>) {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(30)).await;
            // The pool is gone: drop the idle connections and stop.
            let Some(inner) = inner.upgrade() else { return };
            if inner.is_closed() {
                return;
            }
            let now = Instant::now();
            let idle_timeout = inner.settings.idle_timeout;
            let max_lifetime = inner.settings.max_lifetime;
            let min = inner.settings.min_connections as usize;

            let mut idle = inner.lock_idle();
            let mut keep: Vec<IdleConn> = Vec::with_capacity(idle.len());
            let mut expired: Vec<IdleConn> = Vec::new();
            for conn in idle.drain(..) {
                let is_expired = now.duration_since(conn.idle_since) > idle_timeout
                    || now.duration_since(conn.created) > max_lifetime;
                if is_expired {
                    expired.push(conn);
                } else {
                    keep.push(conn);
                }
            }
            // Keep at least `min` connections even if some have expired.
            while keep.len() < min {
                let Some(conn) = expired.pop() else { break };
                keep.push(conn);
            }
            drop(expired);
            *idle = keep;
        }
    }
}

impl crate::query::sealed::Sealed for Pool {}

impl crate::query::Capable for Pool {
    fn execute<'a>(
        &'a self,
        sql: &'a str,
        params: &'a [&'a dyn ToSql],
    ) -> impl Future<Output = Result<u64, Error>> + Send + 'a {
        async move {
            let conn = self.get().await?;
            conn.execute(sql, params).await
        }
    }

    fn fetch_rows<'a>(
        &'a self,
        sql: &'a str,
        params: &'a [&'a dyn ToSql],
        max_rows: i32,
    ) -> impl Future<Output = Result<RowStream, Error>> + Send + 'a {
        async move {
            // The checkout is held until the stream is finished or dropped:
            // handing it back while rows are still streaming would let another
            // query block behind, or interleave with, this one.
            let conn = self.get().await?;
            conn.fetch_rows(sql, params, max_rows).await
        }
    }
}

impl crate::query::ScriptExecutor for Pool {
    fn execute_script<'a>(&'a self, sql: &'a str) -> impl Future<Output = Result<u64, Error>> + Send + 'a {
        async move {
            let conn = self.get().await?;
            conn.execute_script(sql).await
        }
    }
}

/// A connection checked out of a [`Pool`].
///
/// Dereferences to [`Connection`]. When dropped, the connection is returned to
/// the pool rather than closed — but only once any [`RowStream`] taken from it
/// has also been dropped, so the connection is never handed out twice while a
/// query is still streaming.
pub struct PooledConnection {
    checkout: Option<Arc<Checkout>>,
}

impl PooledConnection {
    /// Starts a transaction on this connection.
    pub async fn begin(mut self) -> Result<Transaction, Error> {
        let checkout = self.checkout.take().expect("connection already taken");
        Transaction::begin_checkout(checkout).await
    }

    fn checkout(&self) -> &Checkout {
        self.checkout.as_deref().expect("connection already taken")
    }
}

impl Deref for PooledConnection {
    type Target = Connection;

    fn deref(&self) -> &Connection {
        self.checkout().connection()
    }
}

impl Drop for PooledConnection {
    fn drop(&mut self) {
        // Dropping our clone returns the connection to the pool when no stream
        // still holds one (see `Checkout::drop`).
        let _ = self.checkout.take();
    }
}

impl crate::query::sealed::Sealed for PooledConnection {}

impl crate::query::Capable for PooledConnection {
    fn execute<'a>(
        &'a self,
        sql: &'a str,
        params: &'a [&'a dyn ToSql],
    ) -> impl Future<Output = Result<u64, Error>> + Send + 'a {
        async move { self.checkout().connection().execute(sql, params).await }
    }

    fn fetch_rows<'a>(
        &'a self,
        sql: &'a str,
        params: &'a [&'a dyn ToSql],
        max_rows: i32,
    ) -> impl Future<Output = Result<RowStream, Error>> + Send + 'a {
        async move {
            // Tie a clone of the checkout to the stream: the connection is only
            // returned once both this handle and the stream are gone.
            let checkout = self.checkout.as_ref().expect("connection already taken").clone();
            let stream = checkout.connection().fetch_rows(sql, params, max_rows).await?;
            Ok(stream.with_keepalive(checkout))
        }
    }
}

impl crate::query::ScriptExecutor for PooledConnection {
    fn execute_script<'a>(&'a self, sql: &'a str) -> impl Future<Output = Result<u64, Error>> + Send + 'a {
        async move { self.checkout().connection().execute_script(sql).await }
    }
}
