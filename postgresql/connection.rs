//! Connection establishment, the background actor, and query execution.

use std::{
    collections::{HashMap, VecDeque},
    fmt,
    future::Future,
    pin::Pin,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    task::{Context, Poll},
};

use bytes::{Bytes, BytesMut};
use futures_util::Stream;
use indexmap::IndexMap;
use postgresql_protocol::{
    DbError,
    backend::BackendMessage,
    error::Error as ProtocolError,
    frontend,
    oid::{Format, Oid},
    scram::ScramClient,
};
use small_collections::SmallVec;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::{Notify, broadcast, mpsc, oneshot},
};
use xxhash::{Checksum, Xxh3_128};

use crate::{
    config::Config,
    encode::{IsNull, ToSql},
    error::Error,
    row::{Columns, Row},
    stream::PgStream,
};

const READ_BUFFER_SIZE: usize = 8192;

/// Maximum number of distinct SQL strings interned per connection.
const INTERNER_CAPACITY: usize = 256;

/// Number of rows buffered per query before the actor applies backpressure.
const ROW_CHANNEL_CAPACITY: usize = 32;

/// Rows are coalesced into `CopyData` messages of at least this size.
const COPY_FLUSH_THRESHOLD: usize = 8 * 1024;

/// Number of `CopyData` payloads buffered before the actor applies
/// backpressure. Bounded so that a slow `COPY TO STDOUT` consumer cannot make
/// the client buffer an arbitrary amount of data.
const COPY_CHANNEL_CAPACITY: usize = 16;

/// How long a query is allowed to keep running after its stream was dropped,
/// before a `CancelRequest` is sent to the server.
///
/// Dropping a stream first lets the connection drain the result normally (the
/// actor keeps reading until `ReadyForQuery`); this timeout only fires for
/// queries that would otherwise keep the connection busy for a long time. A
/// random jitter of up to half this value is added so that dropping many
/// streams at once does not produce a synchronized cancel storm.
const ABANDONED_QUERY_CANCEL_DELAY: std::time::Duration = std::time::Duration::from_secs(2);

/// Bound on how many cancellation requests may be pending at once.
///
/// A dropped stream is best-effort: when this many cancellations are already
/// scheduled the request is skipped, and the actor still drains the query to
/// completion.
const CANCEL_QUEUE_CAPACITY: usize = 256;

/// Maximum number of cancellation connections opened at once.
const CANCEL_CONCURRENCY: usize = 16;

/// Timeout for one cancellation attempt (connect + TLS + write).
const CANCEL_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);

/// Largest number of backend messages accepted during the startup handshake.
///
/// A malicious or misbehaving server must not be able to keep the client in the
/// authentication loop indefinitely with a stream of `ParameterStatus` /
/// `NoticeResponse` messages (the handshake timeout also bounds this, but a
/// message cap fails fast and cheaply).
const MAX_AUTH_MESSAGES: usize = 1024;

/// A server notification from `LISTEN` / `NOTIFY`.
#[derive(Debug, Clone)]
pub struct Notification {
    /// Notifying backend process ID.
    pub process_id: i32,
    /// Channel name.
    pub channel: String,
    /// Notification payload.
    pub payload: String,
}

/// A token that cancels the query currently running on a connection.
///
/// Cancelling opens a short-lived TLS connection to the server and sends a
/// `CancelRequest`, exactly like `PQcancel`. It is a no-op if the query has
/// already finished. If the server never sent `BackendKeyData` during startup
/// (some non-PostgreSQL gateways), the token carries a zero key and the request
/// has no effect.
#[derive(Clone)]
pub struct CancelToken {
    config: Arc<Config>,
    process_id: i32,
    secret_key: i32,
}

impl CancelToken {
    /// Requests cancellation of the connection's current query.
    ///
    /// # Errors
    ///
    /// Returns an error if the cancel connection cannot be established.
    pub async fn cancel(&self) -> Result<(), Error> {
        self.cancel_with_timeout(CANCEL_TIMEOUT).await
    }

    /// Like [`CancelToken::cancel`], but gives up after `timeout`.
    pub(crate) async fn cancel_with_timeout(&self, timeout: std::time::Duration) -> Result<(), Error> {
        tokio::time::timeout(timeout, async {
            let tcp = connect_tcp(&self.config).await.map_err(Error::Io)?;
            let _ = tcp.set_nodelay(true);
            let mut stream = negotiate_tls(tcp, &self.config).await?;
            let mut buf = BytesMut::with_capacity(16);
            frontend::cancel_request(&mut buf, self.process_id, self.secret_key);
            stream.write_all(&buf).await.map_err(Error::Io)?;
            stream.flush().await.map_err(Error::Io)?;
            Ok(())
        })
        .await
        .map_err(|_| Error::Config("timed out sending the cancellation request".into()))?
    }
}

/// Number of cancellation jobs currently scheduled but not yet finished.
///
/// Shared process-wide so that dropping many streams cannot schedule more than
/// [`CANCEL_QUEUE_CAPACITY`] cancellations at once.
static PENDING_CANCELS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// Process-wide cap on the number of cancellations running at the same time.
///
/// The semaphore is runtime-independent, so it keeps bounding concurrency even
/// though each job runs on the runtime that scheduled it.
fn cancel_semaphore() -> Arc<tokio::sync::Semaphore> {
    static SEMAPHORE: std::sync::OnceLock<Arc<tokio::sync::Semaphore>> = std::sync::OnceLock::new();
    SEMAPHORE
        .get_or_init(|| Arc::new(tokio::sync::Semaphore::new(CANCEL_CONCURRENCY)))
        .clone()
}

/// A reservation for one scheduled cancellation.
///
/// Released (and its pending-count decremented) when the job finishes or is
/// dropped because its runtime shut down, so a dead runtime cannot leak slots.
struct CancelSlot {
    permit: Option<tokio::sync::OwnedSemaphorePermit>,
}

impl CancelSlot {
    /// Reserves a slot, or returns `None` when the process is already at the
    /// pending or concurrency limit (in which case the best-effort cancel is
    /// skipped).
    fn acquire() -> Option<Self> {
        let previous = PENDING_CANCELS.fetch_add(1, Ordering::AcqRel);
        if previous >= CANCEL_QUEUE_CAPACITY {
            PENDING_CANCELS.fetch_sub(1, Ordering::AcqRel);
            return None;
        }
        match cancel_semaphore().try_acquire_owned() {
            Ok(permit) => Some(CancelSlot {
                permit: Some(permit),
            }),
            Err(_) => {
                PENDING_CANCELS.fetch_sub(1, Ordering::AcqRel);
                None
            }
        }
    }
}

impl Drop for CancelSlot {
    fn drop(&mut self) {
        drop(self.permit.take());
        PENDING_CANCELS.fetch_sub(1, Ordering::AcqRel);
    }
}

/// Cancels a query when dropped before it completes.
struct QueryGuard {
    completed: Arc<AtomicBool>,
    cancel: Option<CancelToken>,
}

impl Drop for QueryGuard {
    fn drop(&mut self) {
        cancel_when_stuck(&self.completed, self.cancel.take());
    }
}

/// Schedules a cancellation for a query that is still running after
/// [`ABANDONED_QUERY_CANCEL_DELAY`].
///
/// The connection keeps draining the result in the background meanwhile, so
/// most abandoned queries finish normally without a cancellation; the timeout
/// only fires for queries that would otherwise occupy the connection for a
/// long time. Cancelling is inherently racy: the server may already be done.
///
/// The job runs on the **current** runtime (never a process-wide worker bound
/// to whichever runtime ran first), so a runtime shutting down cannot disable
/// cancellation for the rest of the process. It is dropped — never queued
/// unboundedly — when the process is already at its pending or concurrency
/// limit.
fn cancel_when_stuck(completed: &Arc<AtomicBool>, cancel: Option<CancelToken>) {
    let Some(token) = cancel else { return };
    if tokio::runtime::Handle::try_current().is_err() {
        return;
    }
    let Some(slot) = CancelSlot::acquire() else { return };
    let completed = completed.clone();
    tokio::spawn(async move {
        let _slot = slot;
        let jitter_ms = (ABANDONED_QUERY_CANCEL_DELAY.as_millis() as u64) / 2;
        let extra = rand::random_range(0..=jitter_ms);
        tokio::time::sleep(ABANDONED_QUERY_CANCEL_DELAY + std::time::Duration::from_millis(extra)).await;
        if !completed.load(Ordering::Acquire) {
            let _ = token.cancel_with_timeout(CANCEL_TIMEOUT).await;
        }
    });
}

/// Interns SQL text so repeated queries send an `Arc<str>` clone instead of
/// allocating a fresh `String` each time.
struct Interner {
    map: HashMap<u128, Arc<str>>,
    order: VecDeque<u128>,
    capacity: usize,
}

impl Interner {
    fn new(capacity: usize) -> Self {
        Interner {
            map: HashMap::with_capacity(capacity),
            order: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    fn intern(&mut self, sql: &str) -> Arc<str> {
        let key = xxhash::xxh3_128(sql.as_bytes());
        if let Some(existing) = self.map.get(&key) {
            if existing.as_ref() == sql {
                return existing.clone();
            }
            // A hash collision: drop the other string under this key (and its
            // eviction slot) so the two entries cannot evict each other.
            self.map.remove(&key);
            self.order.retain(|&k| k != key);
        }
        if self.capacity == 0 {
            return Arc::from(sql);
        }
        while self.map.len() >= self.capacity {
            match self.order.pop_front() {
                Some(old) => {
                    self.map.remove(&old);
                }
                None => break,
            }
        }
        let arc: Arc<str> = Arc::from(sql);
        self.map.insert(key, arc.clone());
        self.order.push_back(key);
        arc
    }
}

/// The 19-byte binary `COPY` file header: signature, flags and extension length.
pub(crate) const COPY_HEADER: &[u8] = b"PGCOPY\n\xff\r\n\0\x00\x00\x00\x00\x00\x00\x00\x00";

/// Parameters encoded for one query: the parameter OIDs plus their framed
/// binary values (each prefixed with its 4-byte length, `-1` for `NULL`).
pub(crate) struct EncodedParams {
    pub oids: SmallVec<Oid, 8>,
    pub values: BytesMut,
}

pub(crate) fn encode_params(params: &[&dyn ToSql]) -> Result<EncodedParams, Error> {
    let mut values = BytesMut::with_capacity(params.len() * 16);
    let mut oids: SmallVec<Oid, 8> = SmallVec::new();
    for param in params {
        oids.push(param.oid());
        let len_pos = values.len();
        values.extend_from_slice(&0i32.to_be_bytes());
        let start = values.len();
        match param.encode(&mut values)? {
            IsNull::Null => {
                values.truncate(len_pos);
                values.extend_from_slice(&(-1i32).to_be_bytes());
            }
            IsNull::NotNull => {
                let len = i32::try_from(values.len() - start)
                    .map_err(|_| Error::Encode("parameter is larger than 2 GiB".into()))?;
                values[len_pos..len_pos + 4].copy_from_slice(&len.to_be_bytes());
            }
        }
    }
    Ok(EncodedParams {
        oids,
        values,
    })
}

/// A row handed from the actor to a [`RowStream`], with the number of bytes it
/// accounts for in the stream's [`BufferBudget`].
pub(crate) struct RowItem {
    pub(crate) row: Row,
    pub(crate) size: usize,
}

pub(crate) enum RowSink {
    Channel {
        tx: mpsc::Sender<Result<RowItem, Error>>,
        budget: Option<Arc<BufferBudget>>,
    },
    One(Option<oneshot::Sender<Result<Option<Row>, Error>>>),
}

pub(crate) enum RowReceiver {
    Channel(mpsc::Receiver<Result<RowItem, Error>>),
    One(oneshot::Receiver<Result<Option<Row>, Error>>),
}

/// A byte budget shared between the actor (producer) and a stream (consumer).
///
/// It bounds how many decoded bytes are buffered for a reader that is not
/// keeping up, independently of the message count. A single item of any size is
/// always allowed when nothing is queued, so an oversized row cannot deadlock
/// the reader.
pub(crate) struct BufferBudget {
    limit: usize,
    used: AtomicUsize,
    closed: AtomicBool,
    notify: Notify,
}

impl BufferBudget {
    pub(crate) fn new(limit: usize) -> Arc<Self> {
        Arc::new(BufferBudget {
            limit,
            used: AtomicUsize::new(0),
            closed: AtomicBool::new(false),
            notify: Notify::new(),
        })
    }

    /// Releases the bytes of an item that left the queue.
    pub(crate) fn release(&self, size: usize) {
        let _ = self
            .used
            .try_update(Ordering::AcqRel, Ordering::Acquire, |used| used.checked_sub(size));
        // Exactly one producer waits on a budget, so `notify_one` is both
        // sufficient and race-free: unlike `notify_waiters`, it stores a permit
        // when the producer has not registered its `Notified` future yet.
        self.notify.notify_one();
    }

    /// Wakes (and stops) any producer waiting for capacity.
    pub(crate) fn close(&self) {
        self.closed.store(true, Ordering::Release);
        self.notify.notify_one();
    }

    fn try_reserve(&self, size: usize) -> bool {
        let mut used = self.used.load(Ordering::Acquire);
        loop {
            let next = used.saturating_add(size);
            if used != 0 && next > self.limit {
                return false;
            }
            match self
                .used
                .compare_exchange_weak(used, next, Ordering::AcqRel, Ordering::Acquire)
            {
                Ok(_) => return true,
                Err(actual) => used = actual,
            }
        }
    }

    /// Waits until `size` bytes fit, or returns `false` when the reader is gone.
    pub(crate) async fn reserve(&self, size: usize) -> bool {
        loop {
            if self.try_reserve(size) {
                return true;
            }
            if self.closed.load(Ordering::Acquire) {
                return false;
            }
            let notified = self.notify.notified();
            if self.try_reserve(size) {
                return true;
            }
            if self.closed.load(Ordering::Acquire) {
                return false;
            }
            notified.await;
        }
    }
}

pub(crate) struct QueryRequest {
    sql: Arc<str>,
    params: EncodedParams,
    rows: Option<RowSink>,
    done: oneshot::Sender<Result<u64, Error>>,
    max_rows: i32,
    completed: Arc<AtomicBool>,
}

/// The outcome of a script run.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ScriptOutcome {
    /// Affected-row count of the last command.
    pub(crate) affected: u64,
    /// The transaction status byte from the final `ReadyForQuery` (`I`, `T` or
    /// `E`).
    pub(crate) transaction_status: u8,
}

pub(crate) struct ScriptRequest {
    sql: String,
    done: oneshot::Sender<Result<ScriptOutcome, Error>>,
    completed: Arc<AtomicBool>,
}

pub(crate) enum CopyInChunk {
    Data(Bytes),
    Done,
    Fail(String),
}

pub(crate) struct CopyInRequest {
    sql: String,
    chunks: mpsc::Receiver<CopyInChunk>,
    done: oneshot::Sender<Result<u64, Error>>,
}

pub(crate) struct CopyOutRequest {
    sql: String,
    chunks: mpsc::Sender<Result<Bytes, Error>>,
    budget: Arc<BufferBudget>,
    done: oneshot::Sender<Result<u64, Error>>,
    completed: Arc<AtomicBool>,
}

pub(crate) enum Request {
    Query(QueryRequest),
    Script(ScriptRequest),
    CopyIn(CopyInRequest),
    CopyOut(CopyOutRequest),
}

/// A failed query attempt.
///
/// `rows_sent` records whether any `DataRow` was already handed to the caller's
/// stream: a stale-plan error can only be retried safely when nothing has been
/// delivered, otherwise the retry would duplicate those rows.
struct AttemptError {
    error: Error,
    rows_sent: bool,
}

impl AttemptError {
    /// A failure before any row was streamed.
    fn before_rows(error: impl Into<Error>) -> Self {
        AttemptError {
            error: error.into(),
            rows_sent: false,
        }
    }
}

impl From<Error> for AttemptError {
    fn from(error: Error) -> Self {
        AttemptError::before_rows(error)
    }
}

impl From<ProtocolError> for AttemptError {
    fn from(error: ProtocolError) -> Self {
        AttemptError::before_rows(error)
    }
}

/// A live connection to one PostgreSQL server.
///
/// `Connection` is a cheap, cloneable handle to a background actor that owns
/// the socket. Cloning shares the same connection. All clones must be dropped
/// for the connection to close.
///
/// Queries are queued and executed one at a time. The queue is unbounded, so a
/// connection is best treated as "one in-flight query at a time": spawning
/// many queries against a single connection without awaiting them queues their
/// encoded parameters in memory. Use a [`Pool`](crate::Pool) for concurrency.
///
/// A row stream that is kept alive but not polled applies backpressure: the
/// connection (and, for a pooled connection, its checkout) stays busy until
/// the stream is consumed or dropped. Consume or drop streams promptly.
///
/// The background task runs on the runtime that created the connection; see
/// the [crate documentation](crate#runtime-affinity).
#[derive(Clone)]
pub struct Connection {
    tx: mpsc::UnboundedSender<Request>,
    interner: Arc<Mutex<Interner>>,
    broken: Arc<AtomicBool>,
    config: Arc<Config>,
    notifications: broadcast::Sender<Notification>,
    process_id: i32,
    secret_key: i32,
}

impl fmt::Debug for Connection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Connection").finish_non_exhaustive()
    }
}

struct CachedStatement {
    sql: Arc<str>,
    oids: SmallVec<Oid, 8>,
    name: Arc<str>,
    fields: Arc<Columns>,
}

impl CachedStatement {
    fn matches(&self, sql: &str, oids: &[Oid]) -> bool {
        self.sql.as_ref() == sql && self.oids.as_slice() == oids
    }
}

struct Actor {
    stream: PgStream,
    read_buf: BytesMut,
    write_buf: BytesMut,
    cache: IndexMap<u128, CachedStatement>,
    pending_closes: Vec<Arc<str>>,
    statement_counter: u64,
    broken: Arc<AtomicBool>,
    zone: time::TimeZone,
    notifications: broadcast::Sender<Notification>,
    statement_cache_size: usize,
    max_message_len: usize,
}

impl Actor {
    fn mark_broken(&self) {
        self.broken.store(true, Ordering::Release);
    }

    fn is_broken(&self) -> bool {
        self.broken.load(Ordering::Acquire)
    }
}

/// Resolves the configured IANA time zone (default UTC).
fn resolve_zone(config: &Config) -> Result<time::TimeZone, Error> {
    match &config.timezone {
        Some(name) => time::TimeZone::named(name).map_err(|e| Error::Config(format!("invalid timezone `{name}`: {e}"))),
        None => Ok(time::TimeZone::UTC),
    }
}

/// Hashes a statement identity with XXH3-128, without allocating.
///
/// The hashed byte sequence is exactly `sql` followed by each parameter OID in
/// big-endian order, so the hash is unchanged from when the input was built in
/// a scratch buffer.
fn statement_key(sql: &str, oids: &[Oid]) -> u128 {
    let mut hasher = Xxh3_128::new();
    hasher.update(sql.as_bytes());
    for oid in oids {
        hasher.update(&oid.to_be_bytes());
    }
    hasher.sum()
}

/// `0A000` (cached plan must not change result type) and `26000` (prepared
/// statement does not exist) are recoverable by re-preparing.
fn is_retryable_plan_error(code: &str) -> bool {
    code == "0A000" || code == "26000"
}

/// Whether a failed attempt may be retried after re-preparing the statement.
///
/// A retry is only safe when the caller's stream has not yet received a row:
/// re-running the query after rows were delivered would duplicate them. Only
/// the first attempt is retried, and only for a stale-plan error.
fn should_retry(attempt: u8, rows_sent: bool, error: &Error) -> bool {
    attempt == 1 && !rows_sent && matches!(error, Error::Server(db) if is_retryable_plan_error(&db.code))
}

impl Connection {
    /// Connects using a `postgres://` URL.
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
    /// use postgresql::Connection;
    ///
    /// // Encrypted; the server certificate is not verified (default).
    /// let conn = Connection::connect("postgres://user:pass@db.example.com/app").await?;
    ///
    /// // Verify the certificate chain and the hostname.
    /// let verified = Connection::connect(
    ///     "postgres://user:pass@db.example.com/app?sslmode=verify-full&sslrootcert=/etc/ssl/db-ca.pem",
    /// )
    /// .await?;
    /// # let _ = (conn, verified);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a configuration error for an invalid URL, or an I/O, TLS,
    /// authentication or protocol error if the handshake fails.
    pub async fn connect(url: &str) -> Result<Self, Error> {
        let config = Config::parse(url)?;
        Connection::connect_with_config(config).await
    }

    /// Connects using an already-parsed [`Config`].
    pub(crate) async fn connect_with_config(config: Config) -> Result<Self, Error> {
        // The timeout covers the whole handshake: TCP connect, TLS negotiation
        // and authentication. A hostile (or merely slow) server that accepts
        // TCP and then stalls must not hang `connect` forever.
        let (stream, read_buf, process_id, secret_key) =
            tokio::time::timeout(config.connect_timeout, establish(&config))
                .await
                .map_err(|_| Error::Config(format!("timed out connecting to {}:{}", config.host, config.port)))??;
        let zone = resolve_zone(&config)?;
        let broken = Arc::new(AtomicBool::new(false));
        let (notifications, _) = broadcast::channel(64);
        let actor = Actor {
            stream,
            read_buf,
            write_buf: BytesMut::with_capacity(512),
            cache: IndexMap::new(),
            pending_closes: Vec::new(),
            statement_counter: 0,
            broken: broken.clone(),
            zone,
            notifications: notifications.clone(),
            statement_cache_size: config.statement_cache_size,
            max_message_len: config.max_message_len,
        };
        let (tx, rx) = mpsc::unbounded_channel();
        tokio::spawn(actor.run(rx));
        Ok(Connection {
            tx,
            interner: Arc::new(Mutex::new(Interner::new(INTERNER_CAPACITY))),
            broken,
            config: Arc::new(config),
            notifications,
            process_id,
            secret_key,
        })
    }

    /// Returns `true` if the connection has been closed or hit an I/O error.
    pub fn is_closed(&self) -> bool {
        self.broken.load(Ordering::Acquire) || self.tx.is_closed()
    }

    /// The configured maximum backend message / row size, in bytes.
    pub(crate) fn max_message_len(&self) -> usize {
        self.config.max_message_len
    }

    /// Subscribes to `LISTEN` / `NOTIFY` notifications.
    ///
    /// The returned receiver yields notifications delivered on this connection
    /// (after `LISTEN`). Multiple subscribers are supported.
    pub fn notifications(&self) -> broadcast::Receiver<Notification> {
        self.notifications.subscribe()
    }

    /// Returns a token that can cancel the currently running query.
    pub fn cancel_token(&self) -> CancelToken {
        CancelToken {
            config: self.config.clone(),
            process_id: self.process_id,
            secret_key: self.secret_key,
        }
    }

    /// Verifies the connection is usable with a trivial round trip.
    pub async fn ping(&self) -> Result<(), Error> {
        self.execute("SELECT 1", &[]).await?;
        Ok(())
    }

    fn intern(&self, sql: &str) -> Arc<str> {
        self.interner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .intern(sql)
    }

    pub(crate) async fn fetch_rows(&self, sql: &str, params: &[&dyn ToSql], max_rows: i32) -> Result<RowStream, Error> {
        let sql = self.intern(sql);
        let encoded = encode_params(params)?;
        let (rows, receiver, budget) = if max_rows == 1 {
            let (tx, rx) = oneshot::channel();
            (RowSink::One(Some(tx)), RowReceiver::One(rx), None)
        } else {
            let (tx, rx) = mpsc::channel(ROW_CHANNEL_CAPACITY);
            let budget = (max_rows == 0).then(|| BufferBudget::new(self.config.max_buffered_bytes));
            (
                RowSink::Channel {
                    tx,
                    budget: budget.clone(),
                },
                RowReceiver::Channel(rx),
                budget,
            )
        };
        let (done, _done_rx) = oneshot::channel();
        let completed = Arc::new(AtomicBool::new(false));
        self.tx
            .send(Request::Query(QueryRequest {
                sql,
                params: encoded,
                rows: Some(rows),
                done,
                max_rows,
                completed: completed.clone(),
            }))
            .map_err(|_| Error::Closed)?;
        Ok(RowStream::new(
            receiver,
            budget,
            completed,
            if max_rows == 0 { Some(self.cancel_token()) } else { None },
        ))
    }

    pub(crate) async fn execute(&self, sql: &str, params: &[&dyn ToSql]) -> Result<u64, Error> {
        let sql = self.intern(sql);
        let encoded = encode_params(params)?;
        // `execute` does not return rows: no row channel is allocated at all,
        // and the actor skips rows instead of blocking on an unread channel.
        // Rows produced by the statement are discarded; the affected-row count
        // is taken from the command tag.
        let (done, done_rx) = oneshot::channel();
        let completed = Arc::new(AtomicBool::new(false));
        let guard = QueryGuard {
            completed: completed.clone(),
            cancel: Some(self.cancel_token()),
        };
        self.tx
            .send(Request::Query(QueryRequest {
                sql,
                params: encoded,
                rows: None,
                done,
                max_rows: 0,
                completed: completed.clone(),
            }))
            .map_err(|_| Error::Closed)?;
        let result = match done_rx.await {
            Ok(result) => result,
            Err(_) => Err(Error::Closed),
        };
        completed.store(true, Ordering::Release);
        drop(guard);
        result
    }

    /// Runs a multi-statement script via the simple query protocol.
    ///
    /// # Errors
    ///
    /// Returns an error if the connection is closed or the server rejects a
    /// statement.
    pub(crate) async fn execute_script(&self, sql: &str) -> Result<u64, Error> {
        Ok(self.execute_script_status(sql).await?.affected)
    }

    /// Runs a script and reports its transaction status.
    ///
    /// # Errors
    ///
    /// Returns an error if the connection is closed or the server rejects a
    /// statement.
    pub(crate) async fn execute_script_status(&self, sql: &str) -> Result<ScriptOutcome, Error> {
        let (done, done_rx) = oneshot::channel();
        let completed = Arc::new(AtomicBool::new(false));
        let guard = QueryGuard {
            completed: completed.clone(),
            cancel: Some(self.cancel_token()),
        };
        self.tx
            .send(Request::Script(ScriptRequest {
                sql: sql.to_string(),
                done,
                completed: completed.clone(),
            }))
            .map_err(|_| Error::Closed)?;
        let result = match done_rx.await {
            Ok(result) => result,
            Err(_) => Err(Error::Closed),
        };
        completed.store(true, Ordering::Release);
        drop(guard);
        result
    }

    /// Enqueues a statement without waiting for its result.
    ///
    /// Used by [`Transaction`](crate::Transaction)'s `Drop` to issue a
    /// `ROLLBACK` before the connection is returned to its pool.
    pub(crate) fn enqueue(&self, sql: &str) {
        let sql = self.intern(sql);
        let Ok(params) = encode_params(&[]) else {
            return;
        };
        let (done, _done_rx) = oneshot::channel();
        let completed = Arc::new(AtomicBool::new(false));
        let _ = self.tx.send(Request::Query(QueryRequest {
            sql,
            params,
            rows: None,
            done,
            max_rows: 0,
            completed,
        }));
    }

    /// Starts a binary `COPY ... FROM STDIN` and returns a writer.
    ///
    /// Values are encoded into a shared buffer and coalesced into `CopyData`
    /// messages of at least 8 KiB. Dropping the writer without finishing
    /// aborts the COPY and leaves the connection usable.
    ///
    /// # Errors
    ///
    /// Returns an error if the connection is closed.
    pub(crate) async fn copy_in(&self, sql: &str) -> Result<CopyInWriter, Error> {
        let (tx, rx) = mpsc::channel(16);
        let (done, done_rx) = oneshot::channel();
        self.tx
            .send(Request::CopyIn(CopyInRequest {
                sql: sql.to_string(),
                chunks: rx,
                done,
            }))
            .map_err(|_| Error::Closed)?;
        // Binary COPY begins with the 19-byte file header.
        let _ = tx.send(CopyInChunk::Data(Bytes::from_static(COPY_HEADER))).await;
        Ok(CopyInWriter {
            tx,
            done: Some(done_rx),
            finished: false,
            buf: Mutex::new(BytesMut::with_capacity(COPY_FLUSH_THRESHOLD * 2)),
        })
    }

    /// Starts a binary `COPY ... TO STDOUT` and returns a stream of raw
    /// `CopyData` payloads.
    ///
    /// # Errors
    ///
    /// Returns an error if the connection is closed.
    pub(crate) async fn copy_out(&self, sql: &str) -> Result<CopyOutStream, Error> {
        let (chunks, rx) = mpsc::channel(COPY_CHANNEL_CAPACITY);
        let budget = BufferBudget::new(self.config.max_buffered_bytes);
        let completed = Arc::new(AtomicBool::new(false));
        let (done, _done_rx) = oneshot::channel();
        self.tx
            .send(Request::CopyOut(CopyOutRequest {
                sql: sql.to_string(),
                chunks,
                budget: budget.clone(),
                done,
                completed: completed.clone(),
            }))
            .map_err(|_| Error::Closed)?;
        Ok(CopyOutStream {
            rx,
            budget,
            completed,
            cancel: Some(self.cancel_token()),
            terminated: false,
        })
    }
}

/// Encodes one binary-format `COPY` row: an `Int16` field count followed by
/// each field as an `Int32` length (or `-1` for `NULL`) and its bytes.
pub fn encode_copy_row(values: &[&dyn ToSql], buf: &mut BytesMut) -> Result<(), Error> {
    let count = u16::try_from(values.len())
        .map_err(|_| Error::Encode(format!("too many columns in COPY row: {} (max {})", values.len(), u16::MAX)))?;
    buf.extend_from_slice(&count.to_be_bytes());
    for value in values {
        let len_pos = buf.len();
        buf.extend_from_slice(&0i32.to_be_bytes());
        let start = buf.len();
        match value.encode(buf)? {
            IsNull::Null => {
                buf.truncate(len_pos);
                buf.extend_from_slice(&(-1i32).to_be_bytes());
            }
            IsNull::NotNull => {
                let len = i32::try_from(buf.len() - start)
                    .map_err(|_| Error::Encode("COPY field is larger than 2 GiB".into()))?;
                buf[len_pos..len_pos + 4].copy_from_slice(&len.to_be_bytes());
            }
        }
    }
    Ok(())
}

/// A writer for a binary `COPY ... FROM STDIN`.
pub(crate) struct CopyInWriter {
    tx: mpsc::Sender<CopyInChunk>,
    done: Option<oneshot::Receiver<Result<u64, Error>>>,
    finished: bool,
    buf: Mutex<BytesMut>,
}

impl CopyInWriter {
    /// Encodes one value into the pending `CopyData` buffer, flushing it once
    /// it is large enough (at least 8 KiB). Small values are coalesced into
    /// larger `CopyData` messages, so a bulk load of short rows does not cost
    /// one syscall and one TLS record per row. Used by
    /// [`crate::copy::CopyIn`] to avoid one allocation per row.
    ///
    /// # Errors
    ///
    /// Returns an error if the connection is closed.
    pub(crate) async fn append(&self, encode: impl FnOnce(&mut BytesMut) -> Result<(), Error>) -> Result<(), Error> {
        let chunk = {
            let mut buf = self.buf.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            encode(&mut buf)?;
            if buf.len() < COPY_FLUSH_THRESHOLD {
                None
            } else {
                Some(buf.split().freeze())
            }
        };
        match chunk {
            Some(chunk) => self.tx.send(CopyInChunk::Data(chunk)).await.map_err(|_| Error::Closed),
            None => Ok(()),
        }
    }

    /// Sends the COPY trailer and completes the COPY.
    ///
    /// Returns the number of rows copied.
    ///
    /// # Errors
    ///
    /// Returns an error if the server rejects the COPY.
    pub async fn finish(mut self) -> Result<u64, Error> {
        // Binary COPY ends with a 16-bit -1 word.
        self.append(|buf| {
            buf.extend_from_slice(&[0xff, 0xff]);
            Ok(())
        })
        .await?;
        if let Some(rest) = {
            let mut buf = self.buf.lock().unwrap_or_else(|p| p.into_inner());
            (!buf.is_empty()).then(|| buf.split().freeze())
        } {
            let _ = self.tx.send(CopyInChunk::Data(rest)).await;
        }
        let _ = self.tx.send(CopyInChunk::Done).await;
        let done = self.done.take().expect("COPY already finished");
        self.finished = true;
        match done.await {
            Ok(result) => result,
            Err(_) => Err(Error::Closed),
        }
    }
}

impl Drop for CopyInWriter {
    fn drop(&mut self) {
        if !self.finished {
            // Best effort: when the channel is full this is lost, and the actor
            // aborts the COPY on its own when the channel closes (see
            // `Actor::handle_copy_in`), so the connection can never be left in
            // COPY-in mode.
            let _ = self.tx.try_send(CopyInChunk::Fail("COPY cancelled".into()));
        }
    }
}

/// A stream of raw `CopyData` payloads from a binary `COPY ... TO STDOUT`.
pub(crate) struct CopyOutStream {
    pub(crate) rx: mpsc::Receiver<Result<Bytes, Error>>,
    budget: Arc<BufferBudget>,
    completed: Arc<AtomicBool>,
    cancel: Option<CancelToken>,
    terminated: bool,
}

impl CopyOutStream {
    #[cfg(test)]
    pub(crate) fn from_receiver(rx: mpsc::Receiver<Result<Bytes, Error>>) -> Self {
        CopyOutStream {
            rx,
            budget: BufferBudget::new(usize::MAX),
            completed: Arc::new(AtomicBool::new(true)),
            cancel: None,
            terminated: false,
        }
    }
}

impl Stream for CopyOutStream {
    type Item = Result<Bytes, Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        match this.rx.poll_recv(cx) {
            Poll::Ready(Some(Ok(bytes))) => {
                this.budget.release(bytes.len());
                Poll::Ready(Some(Ok(bytes)))
            }
            Poll::Ready(Some(Err(err))) => Poll::Ready(Some(Err(err))),
            Poll::Ready(None) => {
                this.budget.close();
                if !this.completed.load(Ordering::Acquire) && !this.terminated {
                    this.terminated = true;
                    return Poll::Ready(Some(Err(Error::Closed)));
                }
                Poll::Ready(None)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

impl Drop for CopyOutStream {
    fn drop(&mut self) {
        // Wakes the actor if it is waiting for buffer capacity, and cancels a
        // transfer the caller abandoned. `COPY TO STDOUT` cannot be aborted by
        // the client without a cancel request, so without this the connection
        // would stream the whole copy just to discard it.
        self.budget.close();
        if !self.completed.load(Ordering::Acquire) {
            cancel_when_stuck(&self.completed, self.cancel.take());
        }
    }
}

/// A stream of rows from a query.
///
/// A pooled connection is held for as long as its stream is alive, so a query
/// started on a [`crate::Pool`] cannot be interleaved with another checkout of
/// the same connection.
pub struct RowStream {
    rx: RowReceiver,
    budget: Option<Arc<BufferBudget>>,
    completed: Arc<AtomicBool>,
    cancel: Option<CancelToken>,
    keepalive: Option<Box<dyn Send + 'static>>,
    terminated: bool,
}

impl RowStream {
    pub(crate) fn new(
        rx: RowReceiver,
        budget: Option<Arc<BufferBudget>>,
        completed: Arc<AtomicBool>,
        cancel: Option<CancelToken>,
    ) -> Self {
        RowStream {
            rx,
            budget,
            completed,
            cancel,
            keepalive: None,
            terminated: false,
        }
    }

    /// Ties the lifetime of `guard` (typically a pooled connection) to this
    /// stream.
    pub(crate) fn with_keepalive(mut self, guard: impl Send + 'static) -> Self {
        self.keepalive = Some(Box::new(guard));
        self
    }
}

impl Stream for RowStream {
    type Item = Result<Row, Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        match &mut this.rx {
            RowReceiver::Channel(rx) => match rx.poll_recv(cx) {
                Poll::Ready(Some(Ok(item))) => {
                    if let Some(budget) = &this.budget {
                        budget.release(item.size);
                    }
                    Poll::Ready(Some(Ok(item.row)))
                }
                Poll::Ready(Some(Err(err))) => Poll::Ready(Some(Err(err))),
                Poll::Ready(None) => {
                    this.keepalive.take();
                    if let Some(budget) = &this.budget {
                        budget.close();
                    }
                    if !this.completed.load(Ordering::Acquire) && !this.terminated {
                        this.terminated = true;
                        return Poll::Ready(Some(Err(Error::Closed)));
                    }
                    Poll::Ready(None)
                }
                Poll::Pending => Poll::Pending,
            },
            RowReceiver::One(rx) => {
                if this.terminated {
                    return Poll::Ready(None);
                }
                match Pin::new(rx).poll(cx) {
                    Poll::Ready(Ok(Ok(Some(row)))) => {
                        this.terminated = true;
                        this.keepalive.take();
                        Poll::Ready(Some(Ok(row)))
                    }
                    Poll::Ready(Ok(Ok(None))) => {
                        this.terminated = true;
                        this.keepalive.take();
                        Poll::Ready(None)
                    }
                    Poll::Ready(Ok(Err(err))) => {
                        this.terminated = true;
                        this.keepalive.take();
                        Poll::Ready(Some(Err(err)))
                    }
                    Poll::Ready(Err(_)) => {
                        this.terminated = true;
                        this.keepalive.take();
                        if this.completed.load(Ordering::Acquire) {
                            Poll::Ready(None)
                        } else {
                            Poll::Ready(Some(Err(Error::Closed)))
                        }
                    }
                    Poll::Pending => Poll::Pending,
                }
            }
        }
    }
}

impl Drop for RowStream {
    fn drop(&mut self) {
        // Wake an actor waiting for buffer capacity.
        if let Some(budget) = &self.budget {
            budget.close();
        }
        // The connection keeps reading the rest of the result in the background
        // (so it is never left half-read); if that takes too long, cancel the
        // query server-side.
        if !self.completed.load(Ordering::Acquire) {
            cancel_when_stuck(&self.completed, self.cancel.take());
        }
    }
}

impl Actor {
    async fn run(mut self, mut rx: mpsc::UnboundedReceiver<Request>) {
        loop {
            let broken = self.is_broken();
            tokio::select! {
                request = rx.recv() => {
                    let Some(request) = request else { break };
                    if broken || self.is_broken() {
                        fail_request(request, Error::Closed);
                        continue;
                    }
                    match request {
                        Request::Query(query) => self.handle_query(query).await,
                        Request::Script(script) => self.handle_script(script).await,
                        Request::CopyIn(copy) => self.handle_copy_in(copy).await,
                        Request::CopyOut(copy) => self.handle_copy_out(copy).await,
                    }
                    self.maybe_shrink_read_buf();
                }
                read = self.stream.read_buf(&mut self.read_buf), if !broken => {
                    match read {
                        Ok(0) => self.mark_broken(),
                        Ok(_) => self.drain_notifications(),
                        Err(_) => self.mark_broken(),
                    }
                }
            }
        }

        // Say goodbye: closing the socket without `Terminate` makes the server
        // log an unexpected EOF and can reset the connection mid-reply.
        self.write_buf.clear();
        let _ = frontend::terminate(&mut self.write_buf);
        let _ = self.flush().await;
    }

    /// Releases an oversized read buffer once it is empty.
    ///
    /// `read_buf` grows to hold the largest message seen and never shrinks on
    /// its own; on a long-lived pooled connection that would keep the memory
    /// for the connection's whole lifetime (an RSS ratchet). Replacing it when
    /// it is empty and large bounds that, while never dropping buffered bytes.
    fn maybe_shrink_read_buf(&mut self) {
        const SHRINK_THRESHOLD: usize = 1024 * 1024;
        if self.read_buf.is_empty() && self.read_buf.capacity() > SHRINK_THRESHOLD {
            self.read_buf = BytesMut::with_capacity(READ_BUFFER_SIZE);
        }
    }

    /// Decodes and forwards any notifications sitting in the read buffer.
    ///
    /// Only asynchronous messages (`NotificationResponse`, `ParameterStatus`,
    /// `NoticeResponse`) can arrive between requests. Anything else means the
    /// stream is out of sync, and the connection is closed rather than having
    /// its messages silently thrown away.
    fn drain_notifications(&mut self) {
        loop {
            match postgresql_protocol::decode_with_limit(&mut self.read_buf, self.max_message_len) {
                Ok(Some(BackendMessage::NotificationResponse {
                    pid,
                    channel,
                    payload,
                })) => {
                    let _ = self.notifications.send(Notification {
                        process_id: pid,
                        channel,
                        payload,
                    });
                }
                Ok(Some(BackendMessage::ParameterStatus {
                    ..
                }))
                | Ok(Some(BackendMessage::NoticeResponse(_))) => {}
                Ok(Some(other)) => {
                    tracing::warn!("postgresql: unexpected message while idle: {other:?}");
                    self.mark_broken();
                    return;
                }
                Ok(None) => break,
                Err(err) => {
                    tracing::warn!("postgresql: protocol error while idle: {err}");
                    self.mark_broken();
                    return;
                }
            }
        }
    }

    /// Runs a multi-statement script via the simple query protocol.
    async fn handle_script(&mut self, request: ScriptRequest) {
        let result = self.run_script(&request.sql).await;
        request.completed.store(true, Ordering::Release);
        let _ = request.done.send(result);
    }

    /// Runs a script and returns the last command's affected rows plus the
    /// transaction status reported by the final `ReadyForQuery`.
    async fn run_script(&mut self, sql: &str) -> Result<ScriptOutcome, Error> {
        tracing::debug!(sql_len = sql.len(), "postgresql: script");
        self.write_buf.clear();
        if let Err(err) = frontend::query(&mut self.write_buf, sql) {
            // Nothing reached the wire: a caller error (for example a NUL in
            // the SQL) must not poison the connection.
            return Err(err.into());
        }
        if let Err(err) = self.flush().await {
            self.mark_broken();
            return Err(err);
        }

        let mut affected = 0u64;
        let mut pending_error: Option<Error> = None;
        let mut transaction_status = b'I';
        loop {
            match self.read_message().await {
                Ok(BackendMessage::CommandComplete(tag)) => match parse_command_tag(&tag) {
                    Ok(count) => affected = count,
                    Err(err) => {
                        self.mark_broken();
                        return Err(err);
                    }
                },
                Ok(BackendMessage::ReadyForQuery(status)) => {
                    transaction_status = status;
                    break;
                }
                Ok(BackendMessage::RowDescription(_)) => {}
                Ok(BackendMessage::DataRow(_)) => {}
                Ok(BackendMessage::NoticeResponse(_)) => {}
                Ok(BackendMessage::EmptyQueryResponse) => {}
                // A script cannot stream `COPY` data: leave COPY mode so the
                // connection is not wedged, then report the error.
                Ok(BackendMessage::CopyInResponse {
                    ..
                }) => {
                    pending_error = Some(self.abort_unexpected_copy(true, false).await);
                    break;
                }
                Ok(BackendMessage::CopyOutResponse {
                    ..
                }) => {
                    pending_error = Some(self.abort_unexpected_copy(false, false).await);
                    break;
                }
                Ok(_) => {}
                Err(Error::Server(db)) => pending_error = Some(Error::Server(db)),
                Err(err) => {
                    self.mark_broken();
                    return Err(err);
                }
            }
        }

        match pending_error {
            Some(err) => Err(err),
            None => Ok(ScriptOutcome {
                affected,
                transaction_status,
            }),
        }
    }

    async fn handle_copy_in(&mut self, request: CopyInRequest) {
        tracing::debug!(sql = %request.sql, "postgresql: copy in");
        self.write_buf.clear();
        let mut encode_error = frontend::parse(&mut self.write_buf, "", &request.sql, &[]).err();
        if encode_error.is_none() {
            match frontend::bind_start(&mut self.write_buf, "", "", 0) {
                Ok(len_pos) => {
                    encode_error = frontend::bind_finish(&mut self.write_buf, Format::Binary, len_pos)
                        .and_then(|()| frontend::execute(&mut self.write_buf, "", 0))
                        .err();
                }
                Err(err) => encode_error = Some(err),
            }
        }
        if let Some(err) = encode_error {
            // The message never reached the wire; the connection is still in a
            // known state.
            let _ = request.done.send(Err(err.into()));
            return;
        }
        if let Err(err) = self.flush().await {
            self.mark_broken();
            let _ = request.done.send(Err(err));
            return;
        }

        let mut pending_error: Option<Box<DbError>> = None;
        loop {
            match self.read_message().await {
                Ok(BackendMessage::ParseComplete) => {}
                Ok(BackendMessage::BindComplete) => {}
                Ok(BackendMessage::CopyInResponse {
                    ..
                }) => break,
                Ok(BackendMessage::NoticeResponse(_)) => {}
                Ok(_) => {}
                Err(Error::Server(db)) => {
                    pending_error = Some(db);
                    break;
                }
                Err(err) => {
                    self.mark_broken();
                    let _ = request.done.send(Err(err));
                    return;
                }
            }
        }

        if let Some(db) = pending_error {
            let _ = self.sync_and_drain().await;
            let _ = request.done.send(Err(Error::Server(db)));
            return;
        }

        let mut chunks = request.chunks;
        let mut stream_error: Option<Error> = None;
        let mut finished = false;
        while let Some(chunk) = chunks.recv().await {
            self.write_buf.clear();
            match chunk {
                CopyInChunk::Data(data) => {
                    if let Err(err) = frontend::copy_data(&mut self.write_buf, &data) {
                        stream_error = Some(err.into());
                        break;
                    }
                }
                CopyInChunk::Done => {
                    if let Err(err) = frontend::copy_done(&mut self.write_buf) {
                        stream_error = Some(err.into());
                        break;
                    }
                    finished = true;
                    if let Err(err) = self.flush().await {
                        self.mark_broken();
                        let _ = request.done.send(Err(err));
                        return;
                    }
                    break;
                }
                CopyInChunk::Fail(message) => {
                    if let Err(err) = frontend::copy_fail(&mut self.write_buf, &message) {
                        stream_error = Some(err.into());
                        break;
                    }
                    finished = true;
                    if let Err(err) = self.flush().await {
                        self.mark_broken();
                        let _ = request.done.send(Err(err));
                        return;
                    }
                    break;
                }
            }
            if let Err(err) = self.flush().await {
                self.mark_broken();
                let _ = request.done.send(Err(err));
                return;
            }
        }

        // If we did not send `CopyDone` or `CopyFail`, leave COPY-in mode
        // explicitly: the server otherwise waits for more data forever and
        // every later query on this connection hangs. This also covers the
        // error paths above, which must not send a bare `Sync` while the
        // server is still in COPY mode.
        if !finished {
            self.write_buf.clear();
            let message = if stream_error.is_some() {
                "COPY aborted by the client"
            } else {
                "COPY writer was dropped before completion"
            };
            if let Err(err) = frontend::copy_fail(&mut self.write_buf, message) {
                self.mark_broken();
                let _ = request.done.send(Err(err.into()));
                return;
            }
            if let Err(err) = self.flush().await {
                self.mark_broken();
                let _ = request.done.send(Err(err));
                return;
            }
        }

        let mut result = self.sync_and_drain().await;
        if let Some(err) = stream_error {
            result = Err(err);
        }
        let _ = request.done.send(result);
    }

    async fn handle_copy_out(&mut self, request: CopyOutRequest) {
        tracing::debug!(sql = %request.sql, "postgresql: copy out");
        self.write_buf.clear();
        let mut encode_error = frontend::parse(&mut self.write_buf, "", &request.sql, &[]).err();
        if encode_error.is_none() {
            match frontend::bind_start(&mut self.write_buf, "", "", 0) {
                Ok(len_pos) => {
                    encode_error = frontend::bind_finish(&mut self.write_buf, Format::Binary, len_pos)
                        .and_then(|()| frontend::execute(&mut self.write_buf, "", 0))
                        .and_then(|()| frontend::sync(&mut self.write_buf))
                        .err();
                }
                Err(err) => encode_error = Some(err),
            }
        }
        if let Some(err) = encode_error {
            // The message never reached the wire; the connection is still in a
            // known state.
            let _ = request.done.send(Err(err.into()));
            return;
        }
        if let Err(err) = self.flush().await {
            self.mark_broken();
            let _ = request.done.send(Err(err));
            return;
        }

        let mut pending_error: Option<Error> = None;
        loop {
            match self.read_message().await {
                Ok(BackendMessage::CopyData(data)) => {
                    // Channel capacity and the byte budget both apply
                    // backpressure. A consumer that is gone stops the sends but
                    // the actor keeps draining to `ReadyForQuery` (a cancel is
                    // scheduled by `CopyOutStream::drop`).
                    let size = data.len();
                    if !request.chunks.is_closed()
                        && request.budget.reserve(size).await
                        && request.chunks.send(Ok(data)).await.is_err()
                    {
                        request.budget.release(size);
                    }
                }
                Ok(BackendMessage::ReadyForQuery(_)) => break,
                Ok(BackendMessage::NoticeResponse(_)) => {}
                Ok(_) => {}
                Err(Error::Server(db)) => {
                    pending_error = Some(Error::Server(db));
                }
                Err(err) => {
                    self.mark_broken();
                    pending_error = Some(err);
                    break;
                }
            }
        }

        request.completed.store(true, Ordering::Release);
        match pending_error {
            // The stream is what the caller reads, so it gets the real error;
            // `done` is only awaited for the row count.
            Some(Error::Server(db)) => {
                let _ = request.chunks.send(Err(Error::Server(db.clone()))).await;
                let _ = request.done.send(Err(Error::Server(db)));
            }
            Some(err) => {
                let _ = request.chunks.send(Err(Error::Closed)).await;
                let _ = request.done.send(Err(err));
            }
            None => {
                let _ = request.done.send(Ok(0));
            }
        }
    }

    /// Sends `Sync` and reads until `ReadyForQuery`, returning the affected
    /// row count (or the server error).
    async fn sync_and_drain(&mut self) -> Result<u64, Error> {
        self.write_buf.clear();
        if let Err(err) = frontend::sync(&mut self.write_buf) {
            self.mark_broken();
            return Err(err.into());
        }
        if let Err(err) = self.flush().await {
            self.mark_broken();
            return Err(err);
        }
        let mut affected = 0;
        let mut result = Ok(affected);
        loop {
            match self.read_message().await {
                Ok(BackendMessage::CommandComplete(tag)) => match parse_command_tag(&tag) {
                    Ok(count) => affected = count,
                    Err(err) => {
                        self.mark_broken();
                        result = Err(err);
                        break;
                    }
                },
                Ok(BackendMessage::ReadyForQuery(_)) => break,
                Ok(BackendMessage::NoticeResponse(_)) => {}
                Ok(_) => {}
                Err(Error::Server(db)) => {
                    result = Err(Error::Server(db));
                }
                Err(err) => {
                    self.mark_broken();
                    result = Err(err);
                    break;
                }
            }
        }
        match result {
            Ok(_) => Ok(affected),
            Err(err) => Err(err),
        }
    }

    async fn handle_query(&mut self, mut query: QueryRequest) {
        tracing::debug!(
            sql = %query.sql,
            params = query.params.oids.len(),
            "postgresql: execute"
        );

        let mut attempts = 0u8;
        let result = loop {
            attempts += 1;
            match self.attempt_query(&mut query).await {
                Ok(affected) => break Ok(affected),
                Err(AttemptError {
                    error,
                    rows_sent,
                }) => {
                    let retryable = should_retry(attempts, rows_sent, &error);
                    if retryable {
                        let hash = statement_key(&query.sql, query.params.oids.as_slice());
                        if let Some(cached) = self.cache.swap_remove(&hash) {
                            self.pending_closes.push(cached.name);
                        }
                        tracing::debug!("postgresql: re-preparing statement after a stale plan");
                        continue;
                    }
                    break Err(error);
                }
            }
        };

        query.completed.store(true, Ordering::Release);
        match result {
            Ok(affected) => {
                if let Some(RowSink::One(tx)) = &mut query.rows
                    && let Some(tx) = tx.take()
                {
                    let _ = tx.send(Ok(None));
                }
                let _ = query.done.send(Ok(affected));
            }
            Err(err) => {
                let for_rows = match &err {
                    Error::Server(db) => Error::Server(db.clone()),
                    Error::Closed => Error::Closed,
                    other => Error::ConnectionFailed(other.to_string()),
                };
                match &mut query.rows {
                    Some(RowSink::Channel {
                        tx, ..
                    }) if !tx.is_closed() => {
                        let _ = tx.send(Err(for_rows)).await;
                    }
                    Some(RowSink::One(tx)) => {
                        if let Some(tx) = tx.take() {
                            let _ = tx.send(Err(for_rows));
                        }
                    }
                    _ => {}
                }
                let _ = query.done.send(Err(err));
            }
        }
    }

    /// Allocates the next server-side statement name.
    fn next_statement_name(&mut self) -> Arc<str> {
        self.statement_counter += 1;
        Arc::from(format!("s{}", self.statement_counter))
    }

    /// Performs one attempt: prepare (when the statement is not cached), bind,
    /// execute and read the result. Rows are streamed to `query.rows`; an error
    /// carries whether any row was already delivered, so a stale plan can only
    /// be retried when the caller's stream has seen nothing yet.
    async fn attempt_query(&mut self, query: &mut QueryRequest) -> Result<u64, AttemptError> {
        let hash = statement_key(&query.sql, query.params.oids.as_slice());
        let param_count = query.params.oids.len();

        // A cache hit only needs Bind/Execute/Sync (one round trip). On a miss
        // -- including `statement_cache_size = 0` -- Parse and Describe are
        // sent in the same flush as Bind/Execute/Sync: the row description
        // arrives before any row, so the whole cycle is still one round trip
        // and one implicit transaction (which keeps it safe behind a
        // transaction-mode pooler).
        let (name, mut fields): (Arc<str>, Option<Arc<Columns>>) = if self.statement_cache_size > 0 {
            let matches = self
                .cache
                .get(&hash)
                .map(|cached| cached.matches(&query.sql, query.params.oids.as_slice()))
                .unwrap_or(false);
            if matches {
                // Move the entry to the back of the map in O(1): `swap_remove`
                // followed by `insert` is an approximate LRU and avoids the
                // O(cache size) scan a `VecDeque::retain` would cost on every
                // cache hit.
                let (_, cached) = self.cache.swap_remove_entry(&hash).expect("checked");
                let name = cached.name.clone();
                let fields = Some(cached.fields.clone());
                self.cache.insert(hash, cached);
                (name, fields)
            } else {
                (self.next_statement_name(), None)
            }
        } else {
            // Caching disabled (e.g. transaction-mode pooler): use the unnamed
            // statement.
            (Arc::from(""), None)
        };
        let needs_prepare = fields.is_none();

        self.write_buf.clear();
        self.write_pending_closes()?;
        if needs_prepare {
            frontend::parse(&mut self.write_buf, &name, &query.sql, &query.params.oids)?;
            frontend::describe(&mut self.write_buf, b'S', &name)?;
        }
        let len_pos = frontend::bind_start(&mut self.write_buf, "", &name, param_count)?;
        self.write_buf.extend_from_slice(&query.params.values);
        frontend::bind_finish(&mut self.write_buf, Format::Binary, len_pos)?;
        frontend::execute(&mut self.write_buf, "", query.max_rows)?;
        frontend::sync(&mut self.write_buf)?;

        if let Err(err) = self.flush().await {
            self.mark_broken();
            return Err(AttemptError::before_rows(err));
        }

        let mut pending_error: Option<Box<DbError>> = None;
        let mut affected = 0u64;
        let mut rows_sent = false;

        loop {
            match self.read_message().await {
                Ok(BackendMessage::ParseComplete)
                | Ok(BackendMessage::ParameterDescription(_))
                | Ok(BackendMessage::BindComplete)
                | Ok(BackendMessage::CloseComplete) => {}
                Ok(BackendMessage::RowDescription(described)) => {
                    if fields.is_none() {
                        fields = Some(Columns::from_fields(&described));
                    }
                }
                Ok(BackendMessage::NoData) => {
                    if fields.is_none() {
                        fields = Some(Columns::from_fields(&[]));
                    }
                }
                Ok(BackendMessage::DataRow(payload)) => {
                    rows_sent = true;
                    let Some(fields) = &fields else {
                        self.mark_broken();
                        return Err(AttemptError {
                            error: Error::Protocol(ProtocolError::Protocol(
                                "the server sent a DataRow before describing the result".into(),
                            )),
                            rows_sent,
                        });
                    };
                    match query.rows.as_mut() {
                        Some(RowSink::Channel {
                            tx,
                            budget,
                        }) => {
                            if !tx.is_closed() {
                                match Row::parse(fields.clone(), &payload, self.zone) {
                                    Ok(row) => {
                                        let size = payload.len();
                                        let admitted = match budget.as_ref() {
                                            Some(budget) => budget.reserve(size).await,
                                            None => true,
                                        };
                                        if admitted
                                            && tx
                                                .send(Ok(RowItem {
                                                    row,
                                                    size,
                                                }))
                                                .await
                                                .is_err()
                                            && let Some(budget) = budget.as_ref()
                                        {
                                            budget.release(size);
                                        }
                                    }
                                    Err(err) => {
                                        let _ = tx.send(Err(err)).await;
                                    }
                                }
                            }
                        }
                        Some(RowSink::One(tx)) => {
                            if let Some(tx) = tx.take() {
                                match Row::parse(fields.clone(), &payload, self.zone) {
                                    Ok(row) => {
                                        let _ = tx.send(Ok(Some(row)));
                                    }
                                    Err(err) => {
                                        let _ = tx.send(Err(err));
                                    }
                                }
                            }
                        }
                        None => {}
                    }
                }
                Ok(BackendMessage::CommandComplete(tag)) => match parse_command_tag(&tag) {
                    Ok(count) => affected = count,
                    Err(err) => {
                        self.mark_broken();
                        return Err(AttemptError {
                            error: err,
                            rows_sent,
                        });
                    }
                },
                Ok(BackendMessage::NoticeResponse(_notice)) => {}
                Ok(BackendMessage::EmptyQueryResponse) => {}
                // The row limit was reached; the trailing `Sync` closes the
                // portal, so keep reading to `ReadyForQuery`.
                Ok(BackendMessage::PortalSuspended) => {}
                // The server entered COPY mode for a statement run through the
                // parameterized path. Leave COPY mode (a bare `COPY ... FROM
                // STDIN` would otherwise make every later query hang), report
                // it, and let `copy_in!` / `copy_out!` be the supported paths.
                Ok(BackendMessage::CopyInResponse {
                    ..
                }) => {
                    let error = self.abort_unexpected_copy(true, true).await;
                    return Err(AttemptError {
                        error,
                        rows_sent,
                    });
                }
                Ok(BackendMessage::CopyOutResponse {
                    ..
                }) => {
                    let error = self.abort_unexpected_copy(false, true).await;
                    return Err(AttemptError {
                        error,
                        rows_sent,
                    });
                }
                Ok(BackendMessage::ReadyForQuery(_)) => break,
                Ok(_) => {}
                Err(Error::Server(db)) => {
                    tracing::debug!(code = %db.code, message = %db.message, "postgresql: server error");
                    pending_error = Some(db);
                }
                Err(err) => {
                    self.mark_broken();
                    return Err(AttemptError {
                        error: err,
                        rows_sent,
                    });
                }
            }
        }

        // Cache the statement as soon as Parse+Describe succeeded, even if the
        // execution itself failed: a constraint violation still leaves a valid
        // prepared statement, and the next call can use the fast path.
        if needs_prepare
            && self.statement_cache_size > 0
            && let Some(fields) = &fields
        {
            let entry = CachedStatement {
                sql: query.sql.clone(),
                oids: query.params.oids.clone(),
                name: name.clone(),
                fields: fields.clone(),
            };
            // A hash collision displaces an unrelated statement; close it so it
            // is not leaked server-side.
            if let Some(old) = self.cache.insert(hash, entry) {
                self.pending_closes.push(old.name);
            }
            self.evict_statements();
        }

        match pending_error {
            Some(db) => Err(AttemptError {
                error: Error::Server(db),
                rows_sent,
            }),
            None => Ok(affected),
        }
    }

    /// Recovers the connection after the server unexpectedly entered COPY mode
    /// for a statement that is not a `copy_in!` / `copy_out!` operation.
    ///
    /// For COPY-in a `CopyFail` is sent — otherwise the server keeps waiting
    /// for data and every later query on the connection hangs. When the
    /// statement was run through the extended protocol (`extended`), a fresh
    /// `Sync` follows: the backend discards frontend messages until a `Sync`
    /// after a copy-in error, and the `Sync` that accompanied `Execute` was
    /// consumed when COPY mode was entered. Draining then reaches
    /// `ReadyForQuery` and the connection is reusable. Returns the error to
    /// report to the caller.
    async fn abort_unexpected_copy(&mut self, copy_in: bool, extended: bool) -> Error {
        if copy_in {
            self.write_buf.clear();
            let encode =
                frontend::copy_fail(&mut self.write_buf, "COPY ... FROM STDIN is not supported here; use copy_in!")
                    .and_then(|()| {
                        if extended {
                            frontend::sync(&mut self.write_buf)
                        } else {
                            Ok(())
                        }
                    });
            if let Err(err) = encode {
                self.mark_broken();
                return err.into();
            }
            if let Err(err) = self.flush().await {
                self.mark_broken();
                return err;
            }
        }
        loop {
            match self.read_message().await {
                Ok(BackendMessage::ReadyForQuery(_)) => break,
                Ok(_) => {}
                Err(Error::Server(_)) => {}
                Err(err) => {
                    self.mark_broken();
                    return err;
                }
            }
        }
        Error::Decode(
            if copy_in {
                "COPY ... FROM STDIN cannot be run as a parameterized query; use copy_in!"
            } else {
                "COPY ... TO STDOUT cannot be run as a parameterized query; use copy_out!"
            }
            .into(),
        )
    }

    /// Evicts the least recently used statements beyond the cache capacity and
    /// queues a `Close` for each of them.
    ///
    /// The `Close` messages are written into the next request, so eviction
    /// costs no round trip of its own (the `CloseComplete` replies are simply
    /// skipped by the response loop).
    fn evict_statements(&mut self) {
        if self.statement_cache_size == 0 {
            return;
        }
        while self.cache.len() > self.statement_cache_size {
            let Some((_, cached)) = self.cache.swap_remove_index(0) else {
                break;
            };
            self.pending_closes.push(cached.name);
        }
    }

    /// Appends the queued `Close` messages to the write buffer.
    fn write_pending_closes(&mut self) -> Result<(), Error> {
        for name in self.pending_closes.drain(..) {
            frontend::close(&mut self.write_buf, b'S', &name)?;
        }
        Ok(())
    }

    async fn flush(&mut self) -> Result<(), Error> {
        self.stream.write_all(&self.write_buf).await.map_err(Error::Io)?;
        self.stream.flush().await.map_err(Error::Io)?;
        self.write_buf.clear();
        Ok(())
    }

    async fn read_message(&mut self) -> Result<BackendMessage, Error> {
        loop {
            match postgresql_protocol::decode_with_limit(&mut self.read_buf, self.max_message_len) {
                // Asynchronous notifications can arrive at any time; forward
                // them and re-decode the buffer.
                Ok(Some(BackendMessage::NotificationResponse {
                    pid,
                    channel,
                    payload,
                })) => {
                    let _ = self.notifications.send(Notification {
                        process_id: pid,
                        channel,
                        payload,
                    });
                    continue;
                }
                Ok(Some(message)) => return Ok(message),
                Ok(None) => {}
                Err(ProtocolError::Server(db)) => return Err(Error::Server(db)),
                Err(err) => return Err(Error::Protocol(err)),
            }
            self.read_buf.reserve(READ_BUFFER_SIZE);
            let read = self.stream.read_buf(&mut self.read_buf).await.map_err(Error::Io)?;
            if read == 0 {
                return Err(Error::Closed);
            }
        }
    }
}

fn fail_request(request: Request, err: Error) {
    match request {
        Request::Query(query) => {
            let _ = query.done.send(Err(err));
        }
        Request::Script(script) => {
            let _ = script.done.send(Err(err));
        }
        Request::CopyIn(copy) => {
            let _ = copy.done.send(Err(err));
        }
        Request::CopyOut(copy) => {
            let _ = copy.done.send(Err(err));
        }
    }
}

/// Parses the affected-row count from a `CommandComplete` tag.
///
/// Returns `0` for tags without a trailing number (`CREATE TABLE`), and a
/// protocol error for a tag that is not valid UTF-8: the connection may be
/// desynchronized, and silently reporting zero rows would hide that.
fn parse_command_tag(tag: &Bytes) -> Result<u64, Error> {
    let text = std::str::from_utf8(tag)
        .map_err(|_| Error::Protocol(ProtocolError::Protocol("CommandComplete tag is not valid UTF-8".into())))?;
    Ok(text.rsplit(' ').next().and_then(|n| n.parse::<u64>().ok()).unwrap_or(0))
}

/// Opens a TCP connection to the configured host.
///
/// An IP literal is parsed first: passing `("::1", port)` to `ToSocketAddrs`
/// asks the resolver for a *hostname*, and `format!("{}:{}", "::1", port)`
/// parses as an IPv6 address with port 0. Host names still go through the
/// resolver.
async fn connect_tcp(config: &Config) -> std::io::Result<TcpStream> {
    match config.host.parse::<std::net::IpAddr>() {
        Ok(ip) => TcpStream::connect(std::net::SocketAddr::new(ip, config.port)).await,
        Err(_) => TcpStream::connect((config.host.as_str(), config.port)).await,
    }
}

async fn establish(config: &Config) -> Result<(PgStream, BytesMut, i32, i32), Error> {
    let tcp = tokio::time::timeout(config.connect_timeout, connect_tcp(config))
        .await
        .map_err(|_| Error::Config(format!("timed out connecting to {}:{}", config.host, config.port)))?
        .map_err(Error::Io)?;
    let _ = tcp.set_nodelay(true);

    let mut stream = negotiate_tls(tcp, config).await?;

    let mut params: SmallVec<(&str, &str), 5> = SmallVec::new();
    params.push(("client_encoding", "UTF8"));
    params.push(("user", &config.user));
    if let Some(database) = &config.database {
        params.push(("database", database));
    }
    if let Some(app) = &config.application_name {
        params.push(("application_name", app));
    }
    if let Some(options) = &config.options {
        params.push(("options", options));
    }

    let mut write_buf = BytesMut::with_capacity(256);
    frontend::startup(&mut write_buf, &params)?;
    stream.write_all(&write_buf).await.map_err(Error::Io)?;
    stream.flush().await.map_err(Error::Io)?;

    let mut read_buf = BytesMut::with_capacity(READ_BUFFER_SIZE);
    let (process_id, secret_key) = authenticate(&mut stream, &mut read_buf, config).await?;
    tracing::debug!(
        host = %config.host,
        port = config.port,
        user = %config.user,
        "postgresql: connected"
    );
    Ok((stream, read_buf, process_id, secret_key))
}

async fn negotiate_tls(tcp: TcpStream, config: &Config) -> Result<PgStream, Error> {
    let mut tcp = tcp;
    let mut request = BytesMut::new();
    frontend::ssl_request(&mut request);
    tcp.write_all(&request).await.map_err(Error::Io)?;
    tcp.flush().await.map_err(Error::Io)?;

    let mut response = [0u8; 1];
    tcp.read_exact(&mut response).await.map_err(Error::Io)?;

    match response[0] {
        b'S' => {
            let connector = crate::stream::tls::connector(config)?;
            let server_name = rustls::pki_types::ServerName::try_from(config.host.clone())
                .map_err(|_| Error::Config(format!("invalid TLS server name `{}`", config.host)))?;
            let stream = connector.connect(server_name, tcp).await.map_err(Error::Io)?;
            Ok(PgStream::Tls(Box::new(stream)))
        }
        b'N' => Err(Error::Config(
            "the server does not support TLS; this client always requires TLS".into(),
        )),
        other => Err(Error::Protocol(ProtocolError::Protocol(format!(
            "unexpected response to SSLRequest: {other}"
        )))),
    }
}

async fn authenticate(stream: &mut PgStream, read_buf: &mut BytesMut, config: &Config) -> Result<(i32, i32), Error> {
    let mut write_buf = BytesMut::with_capacity(256);
    let mut integer_datetimes: Option<String> = None;
    let mut backend_key: Option<(i32, i32)> = None;
    let mut messages = 0usize;

    loop {
        messages += 1;
        if messages > MAX_AUTH_MESSAGES {
            return Err(Error::Protocol(ProtocolError::Protocol(format!(
                "the server sent more than {MAX_AUTH_MESSAGES} messages during authentication"
            ))));
        }
        let message = read_backend(stream, read_buf, config.max_message_len).await?;
        match message {
            BackendMessage::AuthenticationOk => {}
            BackendMessage::AuthenticationCleartextPassword => {
                let password = config.password.as_deref().unwrap_or("");
                write_buf.clear();
                frontend::password(&mut write_buf, password)?;
                stream.write_all(&write_buf).await.map_err(Error::Io)?;
                stream.flush().await.map_err(Error::Io)?;
            }
            BackendMessage::AuthenticationSasl(mechanisms) => {
                if !mechanisms.iter().any(|m| m == "SCRAM-SHA-256") {
                    return Err(Error::Server(Box::new(DbError {
                        severity: "FATAL".into(),
                        code: Default::default(),
                        message: "server does not offer SCRAM-SHA-256 authentication".into(),
                        ..DbError::default()
                    })));
                }
                let mut client = ScramClient::with_max_iterations(
                    config.password.as_deref().unwrap_or(""),
                    config.max_scram_iterations,
                )?;
                write_buf.clear();
                frontend::sasl_initial_response(
                    &mut write_buf,
                    "SCRAM-SHA-256",
                    client.client_first_message().as_bytes(),
                )?;
                stream.write_all(&write_buf).await.map_err(Error::Io)?;
                stream.flush().await.map_err(Error::Io)?;

                let continue_message = read_backend(stream, read_buf, config.max_message_len).await?;
                match continue_message {
                    BackendMessage::AuthenticationSaslContinue(data) => {
                        // The PBKDF2 key derivation is pure CPU work and can
                        // take hundreds of milliseconds (the server picks the
                        // iteration count); run it on the blocking pool so it
                        // does not stall an async worker thread.
                        let (returned, final_message) = tokio::task::spawn_blocking(move || {
                            client.parse_server_first_message(&data)?;
                            let final_message = client.build_client_final_message()?;
                            Ok::<_, ProtocolError>((client, final_message))
                        })
                        .await
                        .map_err(|_| Error::Closed)??;
                        client = returned;
                        write_buf.clear();
                        frontend::sasl_response(&mut write_buf, &final_message)?;
                        stream.write_all(&write_buf).await.map_err(Error::Io)?;
                        stream.flush().await.map_err(Error::Io)?;
                    }
                    other => {
                        return Err(Error::Protocol(ProtocolError::Protocol(format!(
                            "expected SASLContinue, got {other:?}"
                        ))));
                    }
                }

                let final_message = read_backend(stream, read_buf, config.max_message_len).await?;
                match final_message {
                    BackendMessage::AuthenticationSaslFinal(data) => {
                        // Verifying the server signature is a couple of HMACs;
                        // keep it off the worker thread for consistency.
                        tokio::task::spawn_blocking(move || client.parse_server_final_message(&data))
                            .await
                            .map_err(|_| Error::Closed)??;
                    }
                    other => {
                        return Err(Error::Protocol(ProtocolError::Protocol(format!(
                            "expected SASLFinal, got {other:?}"
                        ))));
                    }
                }
            }
            BackendMessage::AuthenticationMd5Password(_) => {
                return Err(Error::Server(Box::new(DbError {
                    severity: "FATAL".into(),
                    code: Default::default(),
                    message: "MD5 authentication is not supported; configure the server to use SCRAM-SHA-256".into(),
                    ..DbError::default()
                })));
            }
            BackendMessage::ParameterStatus {
                name,
                value,
            } => {
                if name == "integer_datetimes" {
                    integer_datetimes = Some(value);
                }
            }
            BackendMessage::BackendKeyData {
                pid,
                secret_key,
            } => {
                backend_key = Some((pid, secret_key));
            }
            BackendMessage::NoticeResponse(_) => {}
            BackendMessage::ReadyForQuery(_) => break,
            other => {
                return Err(Error::Protocol(ProtocolError::Protocol(format!(
                    "unexpected message during startup: {other:?}"
                ))));
            }
        }
    }

    if let Some(setting) = integer_datetimes
        && setting != "on"
    {
        return Err(Error::Config(
            "the server reports integer_datetimes=off, which is not supported".into(),
        ));
    }

    Ok(backend_key.unwrap_or((0, 0)))
}

async fn read_backend(
    stream: &mut PgStream,
    buf: &mut BytesMut,
    max_message_len: usize,
) -> Result<BackendMessage, Error> {
    loop {
        match postgresql_protocol::decode_with_limit(buf, max_message_len) {
            Ok(Some(message)) => return Ok(message),
            Ok(None) => {}
            Err(ProtocolError::Server(db)) => return Err(Error::Server(db)),
            Err(err) => return Err(Error::Protocol(err)),
        }
        buf.reserve(READ_BUFFER_SIZE);
        let read = stream.read_buf(buf).await.map_err(Error::Io)?;
        if read == 0 {
            return Err(Error::Closed);
        }
    }
}

impl crate::query::sealed::Sealed for Connection {}

impl crate::query::Capable for Connection {
    async fn execute<'a>(&'a self, sql: &'a str, params: &'a [&'a dyn ToSql]) -> Result<u64, Error> {
        Connection::execute(self, sql, params).await
    }

    async fn fetch_rows<'a>(
        &'a self,
        sql: &'a str,
        params: &'a [&'a dyn ToSql],
        max_rows: i32,
    ) -> Result<RowStream, Error> {
        Connection::fetch_rows(self, sql, params, max_rows).await
    }
}

impl crate::query::ScriptExecutor for Connection {
    async fn execute_script<'a>(&'a self, sql: &'a str) -> Result<u64, Error> {
        Connection::execute_script(self, sql).await
    }
}

#[cfg(test)]
mod tests {
    use futures_util::StreamExt;

    use super::*;

    fn one_int4_row() -> Row {
        let columns = crate::__private::bench_columns(&[23]);
        let mut payload = BytesMut::new();
        bytes::BufMut::put_i16(&mut payload, 1);
        bytes::BufMut::put_i32(&mut payload, 4);
        bytes::BufMut::put_i32(&mut payload, 7);
        Row::parse(columns, &payload.freeze(), time::TimeZone::UTC).unwrap()
    }

    #[tokio::test]
    async fn one_shot_stream_yields_the_row_then_ends() {
        let (tx, rx) = oneshot::channel();
        tx.send(Ok(Some(one_int4_row()))).unwrap();
        let mut stream = RowStream::new(RowReceiver::One(rx), None, Arc::new(AtomicBool::new(false)), None);
        let row = stream.next().await.unwrap().unwrap();
        assert_eq!(row.try_get_by_index::<i32>(0).unwrap(), 7);
        assert!(stream.next().await.is_none());
        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn one_shot_stream_reports_zero_rows_as_a_clean_end() {
        let (tx, rx) = oneshot::channel();
        tx.send(Ok(None)).unwrap();
        let mut stream = RowStream::new(RowReceiver::One(rx), None, Arc::new(AtomicBool::new(true)), None);
        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn one_shot_stream_reports_errors() {
        let (tx, rx) = oneshot::channel();
        tx.send(Err(Error::RowNotFound)).unwrap();
        let mut stream = RowStream::new(RowReceiver::One(rx), None, Arc::new(AtomicBool::new(true)), None);
        assert!(matches!(stream.next().await, Some(Err(Error::RowNotFound))));
        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn one_shot_stream_reports_a_disappeared_actor() {
        // The sender is dropped before sending anything, and the query was
        // never marked complete: the missing row is an error, not an empty
        // result.
        let (tx, rx) = oneshot::channel::<Result<Option<Row>, Error>>();
        drop(tx);
        let mut stream = RowStream::new(RowReceiver::One(rx), None, Arc::new(AtomicBool::new(false)), None);
        assert!(matches!(stream.next().await, Some(Err(Error::Closed))));
        assert!(stream.next().await.is_none());

        // A completed query that produced no row is a clean end.
        let (tx, rx) = oneshot::channel::<Result<Option<Row>, Error>>();
        drop(tx);
        let mut stream = RowStream::new(RowReceiver::One(rx), None, Arc::new(AtomicBool::new(true)), None);
        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn a_closed_row_channel_is_an_error_until_completed() {
        let (tx, rx) = mpsc::channel(1);
        drop(tx);
        let mut stream = RowStream::new(
            RowReceiver::Channel(rx),
            Some(BufferBudget::new(usize::MAX)),
            Arc::new(AtomicBool::new(false)),
            None,
        );
        // The actor vanished before marking the query complete: the missing
        // rows are an error, not a clean end.
        assert!(matches!(stream.next().await, Some(Err(Error::Closed))));
        assert!(stream.next().await.is_none());

        let (tx, rx) = mpsc::channel(1);
        drop(tx);
        let mut stream = RowStream::new(
            RowReceiver::Channel(rx),
            Some(BufferBudget::new(usize::MAX)),
            Arc::new(AtomicBool::new(true)),
            None,
        );
        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn buffer_budget_bounds_the_queue_and_admits_one_oversized_item() {
        let budget = BufferBudget::new(100);
        // Nothing is queued: even an item larger than the limit is admitted, so
        // a single huge row cannot deadlock the reader.
        assert!(budget.try_reserve(1_000));
        assert!(!budget.try_reserve(1), "a second item must wait");
        budget.release(1_000);
        assert!(budget.try_reserve(80));
        assert!(!budget.try_reserve(30));
        budget.release(80);
        assert!(budget.try_reserve(100));

        // Closing wakes a waiter and makes further reservations fail.
        let waiter = budget.clone();
        let handle = tokio::spawn(async move { waiter.reserve(1).await });
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        budget.close();
        assert!(!handle.await.unwrap());
    }

    #[test]
    fn command_tags_are_parsed_strictly() {
        assert_eq!(parse_command_tag(&Bytes::from_static(b"INSERT 0 3")).unwrap(), 3);
        assert_eq!(parse_command_tag(&Bytes::from_static(b"SELECT 42")).unwrap(), 42);
        // DDL tags have no count.
        assert_eq!(parse_command_tag(&Bytes::from_static(b"CREATE TABLE")).unwrap(), 0);
        // A non-UTF-8 tag means the stream may be desynchronized.
        assert!(parse_command_tag(&Bytes::from_static(b"\xff\xfe")).is_err());
    }

    #[test]
    fn stale_plan_is_only_retried_before_rows_are_delivered() {
        let stale = Error::Server(Box::new(DbError {
            code: "0A000".into(),
            ..DbError::default()
        }));
        assert!(should_retry(1, false, &stale));
        assert!(
            !should_retry(1, true, &stale),
            "retrying after rows were delivered would duplicate them"
        );
        assert!(!should_retry(2, false, &stale), "only one retry is allowed");

        let missing = Error::Server(Box::new(DbError {
            code: "26000".into(),
            ..DbError::default()
        }));
        assert!(should_retry(1, false, &missing));

        let other = Error::Server(Box::new(DbError {
            code: "23505".into(),
            ..DbError::default()
        }));
        assert!(!should_retry(1, false, &other));

        assert!(!should_retry(1, false, &Error::Closed));
    }
}
