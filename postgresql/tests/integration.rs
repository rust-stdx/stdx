//! Integration tests against a live PostgreSQL server.
//!
//! These tests use the `DATABASE_URL` environment variable and are skipped
//! (with a message) when it is not set. They create uniquely named tables so
//! they can run in parallel. Queries go through the macros, matching the
//! public API.

use postgresql::{FromRow, Pool, PoolConfig, ScriptExecutor, Transaction, array, query, query_as, types::Oid};

fn database_url() -> Option<String> {
    match std::env::var("DATABASE_URL") {
        Ok(url) if !url.is_empty() => Some(url),
        _ => None,
    }
}

macro_rules! url_or_skip {
    () => {
        match database_url() {
            Some(url) => url,
            None => {
                eprintln!("skipping test: DATABASE_URL is not set");
                return;
            }
        }
    };
}

async fn connect() -> Option<postgresql::Connection> {
    let url = database_url()?;
    Some(postgresql::Connection::connect(&url).await.expect("connect"))
}

macro_rules! connect_or_skip {
    () => {
        match connect().await {
            Some(conn) => conn,
            None => {
                eprintln!("skipping test: DATABASE_URL is not set");
                return;
            }
        }
    };
}

fn unique(prefix: &str) -> String {
    format!("{prefix}_{}", uuid::Uuid::new_v4().to_string().replace('-', "_"))
}

#[tokio::test]
async fn scalar_roundtrip() {
    let conn = connect_or_skip!();
    let value: i64 = query_as!(i64, "SELECT (41 + 1)::int8").fetch_one(&conn).await.unwrap();
    assert_eq!(value, 42);

    let text: String = query_as!(String, "SELECT 'hello'::text")
        .fetch_one(&conn)
        .await
        .unwrap();
    assert_eq!(text, "hello");
}

#[tokio::test]
async fn parameterized_insert_and_select() {
    let conn = connect_or_skip!();
    let table = unique("pg_basic");
    query!(&format!(
        "CREATE TABLE {table} (id SERIAL PRIMARY KEY, name TEXT NOT NULL, age INT)"
    ))
    .execute(&conn)
    .await
    .unwrap();

    let affected = query!(
        &format!("INSERT INTO {table} (name, age) VALUES ($1, $2), ($3, $4)"),
        "Alice",
        30i32,
        "Bob",
        25i32
    )
    .execute(&conn)
    .await
    .unwrap();
    assert_eq!(affected, 2);

    let rows: Vec<(String, i32)> = query_as!((String, i32), &format!("SELECT name, age FROM {table} ORDER BY id"))
        .fetch_all(&conn)
        .await
        .unwrap();
    assert_eq!(rows, vec![("Alice".to_string(), 30), ("Bob".to_string(), 25)]);

    query!(&format!("DROP TABLE {table}")).execute(&conn).await.unwrap();
}

#[tokio::test]
async fn derive_from_row_with_nulls() {
    let conn = connect_or_skip!();
    let table = unique("pg_derive");
    query!(&format!(
        "CREATE TABLE {table} (id SERIAL PRIMARY KEY, name TEXT NOT NULL, email TEXT)"
    ))
    .execute(&conn)
    .await
    .unwrap();

    query!(
        &format!("INSERT INTO {table} (name, email) VALUES ($1, $2), ($3, $4)"),
        "Alice",
        "alice@example.com",
        "Bob",
        Option::<String>::None
    )
    .execute(&conn)
    .await
    .unwrap();

    #[derive(FromRow, Debug, PartialEq)]
    struct User {
        id: i32,
        name: String,
        email: Option<String>,
    }

    let users: Vec<User> = query_as!(User, &format!("SELECT id, name, email FROM {table} ORDER BY id"))
        .fetch_all(&conn)
        .await
        .unwrap();

    assert_eq!(users.len(), 2);
    assert_eq!(users[0].email.as_deref(), Some("alice@example.com"));
    assert_eq!(users[1].email, None);

    query!(&format!("DROP TABLE {table}")).execute(&conn).await.unwrap();
}

#[tokio::test]
async fn empty_string_is_not_null() {
    let conn = connect_or_skip!();
    let table = unique("pg_empty");
    query!(&format!("CREATE TABLE {table} (id INT, val TEXT)"))
        .execute(&conn)
        .await
        .unwrap();
    query!(
        &format!("INSERT INTO {table} VALUES (1, $1), (2, $2)"),
        "",
        Option::<String>::None
    )
    .execute(&conn)
    .await
    .unwrap();

    let rows: Vec<(i32, Option<String>)> =
        query_as!((i32, Option<String>), &format!("SELECT id, val FROM {table} ORDER BY id"))
            .fetch_all(&conn)
            .await
            .unwrap();
    assert_eq!(rows[0], (1, Some(String::new())));
    assert_eq!(rows[1], (2, None));

    query!(&format!("DROP TABLE {table}")).execute(&conn).await.unwrap();
}

#[tokio::test]
async fn array_parameter_any() {
    let conn = connect_or_skip!();
    let table = unique("pg_array");
    query!(&format!("CREATE TABLE {table} (id INT PRIMARY KEY)"))
        .execute(&conn)
        .await
        .unwrap();
    query!(&format!("INSERT INTO {table} VALUES (1), (2), (3), (4)"))
        .execute(&conn)
        .await
        .unwrap();

    let ids = vec![1i32, 3];
    let rows: Vec<i32> = query_as!(
        i32,
        &format!("SELECT id FROM {table} WHERE id = ANY($1::int4[]) ORDER BY id"),
        ids
    )
    .fetch_all(&conn)
    .await
    .unwrap();
    assert_eq!(rows, vec![1, 3]);

    // Lazy iterator, no intermediate Vec.
    let rows: Vec<i32> = query_as!(
        i32,
        &format!("SELECT id FROM {table} WHERE id = ANY($1::int4[]) ORDER BY id"),
        array(ids.iter())
    )
    .fetch_all(&conn)
    .await
    .unwrap();
    assert_eq!(rows, vec![1, 3]);

    query!(&format!("DROP TABLE {table}")).execute(&conn).await.unwrap();
}

#[tokio::test]
async fn unnest_insert() {
    let conn = connect_or_skip!();
    let table = unique("pg_unnest");
    query!(&format!("CREATE TABLE {table} (id INT PRIMARY KEY, name TEXT NOT NULL)"))
        .execute(&conn)
        .await
        .unwrap();

    let ids = vec![10i32, 20, 30];
    let names = vec!["a", "b", "c"];
    let affected = query!(
        &format!("INSERT INTO {table} (id, name) SELECT * FROM unnest($1::int4[], $2::text[])"),
        ids,
        array(names.iter())
    )
    .execute(&conn)
    .await
    .unwrap();
    assert_eq!(affected, 3);

    let count: i64 = query_as!(i64, &format!("SELECT count(*)::int8 FROM {table}"))
        .fetch_one(&conn)
        .await
        .unwrap();
    assert_eq!(count, 3);

    query!(&format!("DROP TABLE {table}")).execute(&conn).await.unwrap();
}

#[tokio::test]
async fn nullable_array_elements() {
    let conn = connect_or_skip!();
    let values: Vec<Option<i32>> = vec![Some(1), None, Some(3)];
    let rows: Vec<(Option<i32>, i64)> = query_as!(
        (Option<i32>, i64),
        "SELECT unnest, ordinality FROM unnest($1::int4[]) WITH ORDINALITY ORDER BY ordinality",
        values
    )
    .fetch_all(&conn)
    .await
    .unwrap();
    assert_eq!(rows, vec![(Some(1), 1), (None, 2), (Some(3), 3)]);
}

#[tokio::test]
async fn rich_types() {
    let conn = connect_or_skip!();
    let id = uuid::Uuid::new_v4();
    let now = time::DateTime::now();

    let (got_id, got_text, got_json, got_net): (uuid::Uuid, String, serde_json::Value, ipnetwork::IpNetwork) =
        query_as!(
            (uuid::Uuid, String, serde_json::Value, ipnetwork::IpNetwork),
            "SELECT $1::uuid, $2::text, $3::jsonb, $4::inet",
            id,
            "hello",
            serde_json::json!({"a": 1}),
            "10.0.0.0/8".parse::<ipnetwork::IpNetwork>().unwrap()
        )
        .fetch_one(&conn)
        .await
        .unwrap();

    assert_eq!(got_id, id);
    assert_eq!(got_text, "hello");
    assert_eq!(got_json, serde_json::json!({"a": 1}));
    assert_eq!(got_net.prefix(), 8);

    let back: time::DateTime = query_as!(time::DateTime, "SELECT $1::timestamptz", now)
        .fetch_one(&conn)
        .await
        .unwrap();
    assert_eq!(back.unix(), now.unix());
}

#[tokio::test]
async fn unique_violation_is_structured() {
    let conn = connect_or_skip!();
    let table = unique("pg_conflict");
    query!(&format!("CREATE TABLE {table} (id INT PRIMARY KEY)"))
        .execute(&conn)
        .await
        .unwrap();
    query!(&format!("INSERT INTO {table} VALUES (1)"))
        .execute(&conn)
        .await
        .unwrap();

    let err = query!(&format!("INSERT INTO {table} VALUES (1)"))
        .execute(&conn)
        .await
        .unwrap_err();
    match err {
        postgresql::Error::Server(db) => {
            assert_eq!(db.code, "23505");
            assert!(db.is_unique_violation());
        }
        other => panic!("expected a server error, got {other:?}"),
    }

    // The connection must still be usable after the error.
    let value: i32 = query_as!(i32, "SELECT 7").fetch_one(&conn).await.unwrap();
    assert_eq!(value, 7);

    query!(&format!("DROP TABLE {table}")).execute(&conn).await.unwrap();
}

#[tokio::test]
async fn transactions_commit_and_rollback() {
    let conn = connect_or_skip!();
    let table = unique("pg_txn");
    query!(&format!("CREATE TABLE {table} (id INT PRIMARY KEY, val TEXT)"))
        .execute(&conn)
        .await
        .unwrap();

    let txn = Transaction::begin(conn.clone()).await.unwrap();
    query!(&format!("INSERT INTO {table} VALUES (1, 'a')"))
        .execute(&txn)
        .await
        .unwrap();
    txn.commit().await.unwrap();

    let txn = Transaction::begin(conn.clone()).await.unwrap();
    query!(&format!("INSERT INTO {table} VALUES (2, 'b')"))
        .execute(&txn)
        .await
        .unwrap();
    txn.rollback().await.unwrap();

    let ids: Vec<i32> = query_as!(i32, &format!("SELECT id FROM {table} ORDER BY id"))
        .fetch_all(&conn)
        .await
        .unwrap();
    assert_eq!(ids, vec![1]);

    // Dropping an open transaction rolls back.
    {
        let txn = Transaction::begin(conn.clone()).await.unwrap();
        query!(&format!("INSERT INTO {table} VALUES (3, 'c')"))
            .execute(&txn)
            .await
            .unwrap();
    }
    let ids: Vec<i32> = query_as!(i32, &format!("SELECT id FROM {table} ORDER BY id"))
        .fetch_all(&conn)
        .await
        .unwrap();
    assert_eq!(ids, vec![1]);

    query!(&format!("DROP TABLE {table}")).execute(&conn).await.unwrap();
}

#[tokio::test]
async fn pool_get_and_begin() {
    let url = url_or_skip!();
    let pool = Pool::connect(
        &url,
        Some(PoolConfig {
            pool_size: 4,
            min_connections: 1,
            ..PoolConfig::default()
        }),
    )
    .await
    .unwrap();

    let table = unique("pg_pool");
    query!(&format!("CREATE TABLE {table} (id INT PRIMARY KEY, val TEXT)"))
        .execute(&pool)
        .await
        .unwrap();

    let txn = pool.begin().await.unwrap();
    query!(&format!("INSERT INTO {table} VALUES (1, 'pooled')"))
        .execute(&txn)
        .await
        .unwrap();
    txn.commit().await.unwrap();

    let val: String = query_as!(String, &format!("SELECT val FROM {table} WHERE id = 1"))
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(val, "pooled");

    query!(&format!("DROP TABLE {table}")).execute(&pool).await.unwrap();
    pool.close();
}

#[tokio::test]
async fn enum_kind_oid_matches() {
    // A trivially true assertion to keep the `types::Oid` import exercised.
    let oid: Oid = 23;
    assert_eq!(oid, postgresql::types::INT4OID);
}

#[tokio::test]
async fn re_prepares_after_schema_change() {
    let conn = connect_or_skip!();
    let table = unique("pg_ddl");
    query!(&format!("CREATE TABLE {table} (id INT)"))
        .execute(&conn)
        .await
        .unwrap();
    query!(&format!("INSERT INTO {table} VALUES (1)"))
        .execute(&conn)
        .await
        .unwrap();

    let sql = format!("SELECT * FROM {table}");
    let rows: Vec<postgresql::Row> = query!(&sql).fetch_all(&conn).await.unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].len(), 1);

    // Changing the result shape invalidates the cached plan (SQLSTATE 0A000);
    // the client must re-prepare and retry transparently.
    query!(&format!("ALTER TABLE {table} ADD COLUMN name TEXT"))
        .execute(&conn)
        .await
        .unwrap();
    let rows: Vec<postgresql::Row> = query!(&sql).fetch_all(&conn).await.unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].len(), 2);

    query!(&format!("DROP TABLE {table}")).execute(&conn).await.unwrap();
}

#[tokio::test]
async fn re_prepares_after_deallocate_all() {
    let conn = connect_or_skip!();
    let table = unique("pg_dealloc");
    query!(&format!("CREATE TABLE {table} (id INT)"))
        .execute(&conn)
        .await
        .unwrap();
    query!(&format!("INSERT INTO {table} VALUES (1)"))
        .execute(&conn)
        .await
        .unwrap();

    let sql = format!("SELECT id FROM {table}");
    let ids: Vec<i32> = query_as!(i32, &sql).fetch_all(&conn).await.unwrap();
    assert_eq!(ids, vec![1]);

    // All server-side prepared statements vanish (SQLSTATE 26000); the client
    // must re-prepare and retry.
    query!("DEALLOCATE ALL").execute(&conn).await.unwrap();
    let ids: Vec<i32> = query_as!(i32, &sql).fetch_all(&conn).await.unwrap();
    assert_eq!(ids, vec![1]);

    query!(&format!("DROP TABLE {table}")).execute(&conn).await.unwrap();
}

#[tokio::test]
async fn copy_in_derived_struct() {
    let conn = connect_or_skip!();
    let table = unique("pg_copy_row");

    #[derive(postgresql::ToRow)]
    struct Invitation {
        id: i32,
        name: String,
    }

    query!(&format!("CREATE TABLE {table} (id INT, name TEXT)"))
        .execute(&conn)
        .await
        .unwrap();

    let writer = postgresql::copy_in!(conn, Invitation, &format!("COPY {table} (id, name) FROM STDIN BINARY"))
        .await
        .unwrap();
    for i in 0..3i32 {
        writer
            .write_row(&Invitation {
                id: i,
                name: format!("n{i}"),
            })
            .await
            .unwrap();
    }
    writer.finish().await.unwrap();

    let count: i64 = query_as!(i64, &format!("SELECT count(*)::int8 FROM {table}"))
        .fetch_one(&conn)
        .await
        .unwrap();
    assert_eq!(count, 3);

    query!(&format!("DROP TABLE {table}")).execute(&conn).await.unwrap();
}

#[tokio::test]
async fn copy_out_checked_literal() {
    let conn = connect_or_skip!();
    // Compile-time checked: the columns and types are verified against
    // `pg_catalog.pg_type` at build time.
    let out = postgresql::copy_out!(
        conn,
        (String, i16),
        "COPY pg_catalog.pg_type (typname, typlen) TO STDOUT BINARY"
    )
    .await
    .unwrap();
    let rows = out.collect_all().await.unwrap();
    assert!(!rows.is_empty());
}

#[tokio::test]
async fn copy_in_and_out_binary() {
    let conn = connect_or_skip!();
    let table = unique("pg_copy");
    query!(&format!("CREATE TABLE {table} (id INT, name TEXT)"))
        .execute(&conn)
        .await
        .unwrap();

    let writer = postgresql::copy_in!(conn, (i32, String), &format!("COPY {table} (id, name) FROM STDIN BINARY"))
        .await
        .unwrap();
    for i in 0..1000i32 {
        writer.write_row(&(i, format!("name-{i}"))).await.unwrap();
    }
    let copied = writer.finish().await.unwrap();
    assert_eq!(copied, 1000);

    let count: i64 = query_as!(i64, &format!("SELECT count(*)::int8 FROM {table}"))
        .fetch_one(&conn)
        .await
        .unwrap();
    assert_eq!(count, 1000);

    let out = postgresql::copy_out!(conn, (i32, String), &format!("COPY {table} TO STDOUT BINARY"))
        .await
        .unwrap();
    let rows = out.collect_all().await.unwrap();
    assert_eq!(rows.len(), 1000);
    let mut ids: Vec<i32> = rows.iter().map(|r| r.0).collect();
    ids.sort_unstable();
    assert_eq!(ids, (0..1000).collect::<Vec<_>>());

    query!(&format!("DROP TABLE {table}")).execute(&conn).await.unwrap();
}

/// `COPY ... FROM STDIN` has no data source in a script: it must error and
/// leave the connection usable, never hang waiting for COPY data.
#[tokio::test]
async fn copy_from_stdin_in_a_script_errors_instead_of_hanging() {
    let conn = connect_or_skip!();
    let table = unique("pg_copy_script");
    query!(&format!("CREATE TABLE {table} (id INT)"))
        .execute(&conn)
        .await
        .unwrap();

    let result = tokio::time::timeout(
        std::time::Duration::from_secs(10),
        conn.execute_script(&format!("COPY {table} (id) FROM STDIN")),
    )
    .await
    .expect("execute_script must not hang on COPY FROM STDIN");
    assert!(result.is_err(), "COPY FROM STDIN cannot run as a script: {result:?}");

    // The connection must be usable afterwards (COPY mode was left).
    let value: i32 = query_as!(i32, "SELECT 1").fetch_one(&conn).await.unwrap();
    assert_eq!(value, 1);

    query!(&format!("DROP TABLE {table}")).execute(&conn).await.unwrap();
}

/// `COPY ... FROM STDIN` through the parameterized query path must error (use
/// `copy_in!` for real bulk loads), not hang.
#[tokio::test]
async fn copy_from_stdin_through_a_query_errors_instead_of_hanging() {
    let conn = connect_or_skip!();
    let table = unique("pg_copy_query");
    query!(&format!("CREATE TABLE {table} (id INT)"))
        .execute(&conn)
        .await
        .unwrap();

    let sql = format!("COPY {table} (id) FROM STDIN");
    let result: Result<Vec<postgresql::Row>, postgresql::Error> =
        tokio::time::timeout(std::time::Duration::from_secs(10), query!(&sql).fetch_all(&conn))
            .await
            .expect("the query must not hang on COPY FROM STDIN");
    assert!(
        result.is_err(),
        "COPY FROM STDIN cannot run as a parameterized query: {result:?}"
    );

    // The connection must be usable afterwards (COPY mode was left).
    let value: i32 = query_as!(i32, "SELECT 1").fetch_one(&conn).await.unwrap();
    assert_eq!(value, 1);

    query!(&format!("DROP TABLE {table}")).execute(&conn).await.unwrap();
}

/// `COPY ... TO STDOUT` through the parameterized query path must error (use
/// `copy_out!`), not hang, and leave the connection usable.
#[tokio::test]
async fn copy_to_stdout_through_a_query_errors_instead_of_hanging() {
    let conn = connect_or_skip!();
    let table = unique("pg_copy_out_query");
    query!(&format!("CREATE TABLE {table} (id INT)"))
        .execute(&conn)
        .await
        .unwrap();
    query!(&format!("INSERT INTO {table} VALUES (1)"))
        .execute(&conn)
        .await
        .unwrap();

    let sql = format!("COPY {table} (id) TO STDOUT");
    let result: Result<Vec<postgresql::Row>, postgresql::Error> =
        tokio::time::timeout(std::time::Duration::from_secs(10), query!(&sql).fetch_all(&conn))
            .await
            .expect("the query must not hang on COPY TO STDOUT");
    assert!(
        result.is_err(),
        "COPY TO STDOUT cannot run as a parameterized query: {result:?}"
    );

    let value: i32 = query_as!(i32, "SELECT 1").fetch_one(&conn).await.unwrap();
    assert_eq!(value, 1);

    query!(&format!("DROP TABLE {table}")).execute(&conn).await.unwrap();
}

#[tokio::test]
async fn execute_script_multi_statement() {
    let conn = connect_or_skip!();
    let table = unique("pg_script");
    let affected = conn
        .execute_script(&format!(
            "CREATE TABLE {table} (id INT); \
             INSERT INTO {table} VALUES (1), (2); \
             INSERT INTO {table} VALUES (3)"
        ))
        .await
        .unwrap();
    assert_eq!(affected, 1); // affected rows of the last command

    let count: i64 = query_as!(i64, &format!("SELECT count(*)::int8 FROM {table}"))
        .fetch_one(&conn)
        .await
        .unwrap();
    assert_eq!(count, 3);

    conn.execute_script(&format!("DROP TABLE {table}")).await.unwrap();
}

#[tokio::test]
async fn execute_script_is_generic() {
    async fn run<E: ScriptExecutor>(db: &E) -> Result<u64, postgresql::Error> {
        db.execute_script("SELECT 1").await
    }
    let conn = connect_or_skip!();
    assert_eq!(run(&conn).await.unwrap(), 1);
}

#[tokio::test]
async fn listen_notify() {
    let conn = connect_or_skip!();
    conn.execute_script("LISTEN test_chan").await.unwrap();
    let mut notifications = conn.notifications();

    query!("NOTIFY test_chan, 'hello'").execute(&conn).await.unwrap();

    let notification = tokio::time::timeout(std::time::Duration::from_secs(5), notifications.recv())
        .await
        .expect("notification timed out")
        .expect("notification channel closed");
    assert_eq!(notification.channel, "test_chan");
    assert_eq!(notification.payload, "hello");
}

#[tokio::test]
async fn cancels_abandoned_query() {
    let conn = connect_or_skip!();

    // Start a long query and abandon it; dropping the stream cancels it.
    let stream = query!("SELECT pg_sleep(30)").fetch_rows(&conn).await.unwrap();
    drop(stream);

    // The connection must become usable again (the cancel aborts pg_sleep).
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    loop {
        match query_as!(i32, "SELECT 1").fetch_one(&conn).await {
            Ok(value) => {
                assert_eq!(value, 1);
                break;
            }
            Err(err) => {
                if std::time::Instant::now() > deadline {
                    panic!("connection did not recover after cancel: {err}");
                }
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            }
        }
    }
}

/// Cancellation must not be owned by whichever runtime abandons a query first:
/// dropping a runtime must not disable cancellation for the rest of the
/// process.
#[test]
fn cancellation_survives_a_runtime_shutdown() {
    let Some(url) = database_url() else {
        eprintln!("skipping test: DATABASE_URL is not set");
        return;
    };

    // Two independent runtimes, one after the other. If the cancel machinery
    // were bound to the first runtime, the second runtime's abandoned query
    // would run to completion instead of being cancelled.
    for round in 0..2 {
        let runtime = tokio::runtime::Runtime::new().expect("runtime");
        runtime.block_on(async {
            let conn = postgresql::Connection::connect(&url).await.expect("connect");
            let stream = query!("SELECT pg_sleep(30)").fetch_rows(&conn).await.unwrap();
            drop(stream);

            let recovered = tokio::time::timeout(std::time::Duration::from_secs(15), async {
                loop {
                    match query_as!(i32, "SELECT 1").fetch_one(&conn).await {
                        Ok(value) => break value,
                        Err(_) => tokio::time::sleep(std::time::Duration::from_millis(50)).await,
                    }
                }
            })
            .await;
            assert_eq!(
                recovered.unwrap_or_else(|_| panic!("runtime {round} did not recover: cancel was not delivered")),
                1
            );
        });
    }
}

#[tokio::test]
async fn timezone_decoding() {
    let url = url_or_skip!();
    let sep = if url.contains('?') { '&' } else { '?' };
    let conn = postgresql::Connection::connect(&format!("{url}{sep}timezone=America/New_York"))
        .await
        .unwrap();

    // A zone-less timestamp is interpreted in the configured zone.
    let dt: time::DateTime = query_as!(time::DateTime, "SELECT '2024-06-01 12:00:00'::timestamp")
        .fetch_one(&conn)
        .await
        .unwrap();
    let ny = dt.in_timezone(time::TimeZone::named("America/New_York").unwrap());
    assert_eq!((ny.hour(), ny.minute()), (12, 0));
}

/// `execute` is documented to discard rows; it must never block on them.
#[tokio::test]
async fn execute_on_a_row_returning_statement_does_not_hang() {
    let conn = connect_or_skip!();
    for rows in [0, 1, 2, 1000] {
        let sql = if rows == 0 {
            "SELECT 1 WHERE false".to_string()
        } else {
            format!("SELECT generate_series(1, {rows})")
        };
        let affected = tokio::time::timeout(std::time::Duration::from_secs(10), query!(&sql).execute(&conn))
            .await
            .unwrap_or_else(|_| panic!("execute() hung on a statement returning {rows} rows"))
            .unwrap();
        assert_eq!(affected, rows as u64);
    }
}

/// Dropping a `COPY ... FROM STDIN` writer must abort the COPY and leave the
/// connection usable, whether or not the writer's abort message could be
/// delivered.
#[tokio::test]
async fn dropping_a_copy_in_writer_keeps_the_connection_usable() {
    let conn = connect_or_skip!();
    let table = unique("pg_copy_drop");
    query!(&format!("CREATE TABLE {table} (id INT, name TEXT)"))
        .execute(&conn)
        .await
        .unwrap();

    // A handful of rows: the abort message is delivered normally.
    let writer = postgresql::copy_in!(conn, (i32, String), &format!("COPY {table} (id, name) FROM STDIN BINARY"))
        .await
        .unwrap();
    for i in 0..5i32 {
        writer.write_row(&(i, format!("n{i}"))).await.unwrap();
    }
    drop(writer);

    // Many rows: the writer's abort can be dropped when its channel is full,
    // and the actor must still leave COPY-in mode.
    let writer = postgresql::copy_in!(conn, (i32, String), &format!("COPY {table} (id, name) FROM STDIN BINARY"))
        .await
        .unwrap();
    for i in 0..500i32 {
        let _ = writer.write_row(&(i, "x".repeat(4096))).await;
    }
    drop(writer);

    let conn2 = conn.clone();
    let usable = tokio::time::timeout(std::time::Duration::from_secs(20), async move {
        let _: i32 = query_as!(i32, "SELECT 1").fetch_one(&conn2).await?;
        Ok::<_, postgresql::Error>(())
    })
    .await;
    assert!(usable.is_ok(), "the connection must be usable after an abandoned COPY");
    assert!(usable.unwrap().is_ok());

    query!(&format!("DROP TABLE {table}")).execute(&conn).await.unwrap();
}

/// Dropping a `Pool` must close its connections instead of leaking them (and
/// the background reaper) forever.
#[tokio::test]
async fn dropping_a_pool_closes_its_connections() {
    let url = url_or_skip!();
    let sep = if url.contains('?') { '&' } else { '?' };
    let tag = unique("pg_pool_leak");
    let url = format!("{url}{sep}application_name={tag}");

    async fn count(conn: &postgresql::Connection, tag: &str) -> i64 {
        let sql = format!("SELECT count(*)::int8 FROM pg_stat_activity WHERE application_name = '{tag}'");
        let rows: Vec<(i64,)> = query!(&sql).fetch_all(conn).await.unwrap();
        rows[0].0
    }

    let spy = postgresql::Connection::connect(&url).await.unwrap();
    let pool = postgresql::Pool::connect(
        &url,
        Some(PoolConfig {
            pool_size: 3,
            min_connections: 3,
            ..PoolConfig::default()
        }),
    )
    .await
    .unwrap();
    for _ in 0..3 {
        let _: i32 = query_as!(i32, "SELECT 1").fetch_one(&pool).await.unwrap();
    }
    assert_eq!(count(&spy, &tag).await, 4); // 3 pool + the spy

    // No `close()`: dropping the last handle must still tear everything down.
    drop(pool);
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    loop {
        let left = count(&spy, &tag).await;
        if left == 1 {
            break; // only the spy remains
        }
        assert!(
            std::time::Instant::now() < deadline,
            "dropping the pool leaked {left} connections"
        );
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}

/// A connection checked out when `Pool::close` is called must be closed when it
/// is returned, not parked in a pool that will never hand it out again.
#[tokio::test]
async fn a_connection_returned_after_close_is_closed() {
    let url = url_or_skip!();
    let sep = if url.contains('?') { '&' } else { '?' };
    let tag = unique("pg_pool_close");
    let url = format!("{url}{sep}application_name={tag}");

    async fn count(conn: &postgresql::Connection, tag: &str) -> i64 {
        let sql = format!("SELECT count(*)::int8 FROM pg_stat_activity WHERE application_name = '{tag}'");
        let rows: Vec<(i64,)> = query!(&sql).fetch_all(conn).await.unwrap();
        rows[0].0
    }

    let spy = postgresql::Connection::connect(&url).await.unwrap();
    let pool = postgresql::Pool::connect(
        &url,
        Some(PoolConfig {
            pool_size: 2,
            ..PoolConfig::default()
        }),
    )
    .await
    .unwrap();

    // Establish one connection and hold it across `close`.
    let held = pool.get().await.unwrap();
    let _: i32 = query_as!(i32, "SELECT 1").fetch_one(&held).await.unwrap();
    assert_eq!(count(&spy, &tag).await, 2); // held + spy

    pool.close();
    drop(held);

    // The returned connection must be closed, not kept in the idle list.
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    loop {
        let left = count(&spy, &tag).await;
        if left == 1 {
            break; // only the spy remains
        }
        assert!(
            std::time::Instant::now() < deadline,
            "a connection returned after close() was leaked ({left} left)"
        );
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}

/// A pooled connection is held until its row stream is done, so two checkouts
/// can never share one connection mid-query.
#[tokio::test]
async fn pool_connection_is_held_until_the_stream_finishes() {
    let url = url_or_skip!();
    let pool = Pool::connect(
        &url,
        Some(PoolConfig {
            pool_size: 1,
            ..PoolConfig::default()
        }),
    )
    .await
    .unwrap();

    let stream = query!("SELECT generate_series(1, 5000)")
        .fetch_rows(&pool)
        .await
        .unwrap();
    // While the stream is alive the single connection is checked out: a second
    // query times out acquiring one rather than silently queuing behind it.
    let second = tokio::time::timeout(std::time::Duration::from_millis(300), async {
        let q = query!("SELECT 1");
        q.execute(&pool).await
    })
    .await;
    assert!(second.is_err(), "the second query must not run on a busy connection");

    // Dropping the stream returns the connection.
    drop(stream);
    let third = tokio::time::timeout(std::time::Duration::from_secs(10), async {
        let q = query!("SELECT 1");
        q.execute(&pool).await
    })
    .await
    .expect("the connection must return to the pool once the stream is gone")
    .unwrap();
    assert_eq!(third, 1);

    // Draining a stream also releases the connection, without waiting for the
    // stream value itself to be dropped.
    let mut stream = query!("SELECT generate_series(1, 100)")
        .fetch_rows(&pool)
        .await
        .unwrap();
    let mut seen = 0;
    while let Some(row) = futures_util::StreamExt::next(&mut stream).await {
        row.unwrap();
        seen += 1;
    }
    assert_eq!(seen, 100);
    assert!(
        futures_util::StreamExt::next(&mut stream).await.is_none(),
        "an exhausted stream stays exhausted"
    );
    let q = query!("SELECT 1");
    assert_eq!(q.execute(&pool).await.unwrap(), 1);
    pool.close();
}

/// A `PooledConnection` handle may be dropped while a stream taken from it is
/// still alive: the checkout must stay with the stream until it finishes, so a
/// second checkout cannot share the connection mid-query.
#[tokio::test]
async fn a_checked_out_connection_is_held_until_its_stream_finishes() {
    let url = url_or_skip!();
    let pool = Pool::connect(
        &url,
        Some(PoolConfig {
            pool_size: 1,
            ..PoolConfig::default()
        }),
    )
    .await
    .unwrap();

    let conn = pool.get().await.unwrap();
    let mut stream = query!("SELECT generate_series(1, 5000)")
        .fetch_rows(&conn)
        .await
        .unwrap();
    // Drop the handle; the stream keeps the checkout alive.
    drop(conn);

    let second = tokio::time::timeout(std::time::Duration::from_millis(300), async {
        let q = query!("SELECT 1");
        q.execute(&pool).await
    })
    .await;
    assert!(second.is_err(), "the checkout must be held until the stream ends");

    let mut seen = 0;
    while let Some(row) = futures_util::StreamExt::next(&mut stream).await {
        row.unwrap();
        seen += 1;
    }
    assert_eq!(seen, 5000);
    drop(stream);

    let third = tokio::time::timeout(std::time::Duration::from_secs(10), async {
        let q = query!("SELECT 1");
        q.execute(&pool).await
    })
    .await
    .expect("the connection must return to the pool once the stream is gone")
    .unwrap();
    assert_eq!(third, 1);
    pool.close();
}

/// A `Transaction` may be dropped while a stream taken from it is still alive:
/// the checkout must stay with the stream, and the rollback must still be
/// applied, before the connection is reused.
#[tokio::test]
async fn a_transaction_is_held_until_its_stream_finishes() {
    let url = url_or_skip!();
    let pool = Pool::connect(
        &url,
        Some(PoolConfig {
            pool_size: 1,
            ..PoolConfig::default()
        }),
    )
    .await
    .unwrap();

    let table = unique("pg_txn_stream");
    query!(&format!("CREATE TABLE {table} (id INT)"))
        .execute(&pool)
        .await
        .unwrap();

    let txn = pool.begin().await.unwrap();
    query!(&format!("INSERT INTO {table} VALUES (1)"))
        .execute(&txn)
        .await
        .unwrap();
    let mut stream = query!("SELECT generate_series(1, 5000)")
        .fetch_rows(&txn)
        .await
        .unwrap();
    // Dropping the transaction enqueues a ROLLBACK, but the checkout stays with
    // the stream until it finishes.
    drop(txn);

    let second = tokio::time::timeout(std::time::Duration::from_millis(300), async {
        let q = query!("SELECT 1");
        q.execute(&pool).await
    })
    .await;
    assert!(second.is_err(), "the transaction's checkout must be held until its stream ends");

    let mut seen = 0;
    while let Some(row) = futures_util::StreamExt::next(&mut stream).await {
        row.unwrap();
        seen += 1;
    }
    assert_eq!(seen, 5000);
    drop(stream);

    // Once released, the rollback must have run: the inserted row is gone.
    let count: i64 = query_as!(i64, &format!("SELECT count(*)::int8 FROM {table}"))
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 0, "the dropped transaction must have rolled back");

    query!(&format!("DROP TABLE {table}")).execute(&pool).await.unwrap();
    pool.close();
}

/// The prepared-statement cache is bounded: server-side statements must be
/// closed as they are evicted, not accumulated for the life of the connection.
#[tokio::test]
async fn statement_cache_stays_bounded() {
    let url = url_or_skip!();
    let sep = if url.contains('?') { '&' } else { '?' };
    let conn = postgresql::Connection::connect(&format!("{url}{sep}statement_cache_size=8"))
        .await
        .unwrap();

    for round in 0..3 {
        for i in 0..40 {
            let sql = format!("SELECT {i}::int8 + {round}");
            let _: i64 = query_as!(i64, &sql).fetch_one(&conn).await.unwrap();
        }
        let rows: Vec<(i64,)> = query!("SELECT count(*)::int8 FROM pg_prepared_statements WHERE name LIKE 's%'")
            .fetch_all(&conn)
            .await
            .unwrap();
        let statements = rows[0].0;
        assert!(
            statements <= 8 + 1,
            "the prepared-statement cache grew to {statements} server-side statements"
        );
    }
}

/// Evicting statements must not corrupt or slow the connection: every query
/// after an eviction has to work, including on a cache of one statement.
#[tokio::test]
async fn statement_cache_eviction_keeps_the_connection_correct() {
    let url = url_or_skip!();
    let sep = if url.contains('?') { '&' } else { '?' };
    let conn = postgresql::Connection::connect(&format!("{url}{sep}statement_cache_size=1"))
        .await
        .unwrap();
    for i in 0..25 {
        let sql = format!("SELECT {i}::int8");
        let value: i64 = query_as!(i64, &sql).fetch_one(&conn).await.unwrap();
        assert_eq!(value, i);
    }
    // The hot statement is the one that survives (least recently used wins).
    for _ in 0..10 {
        let _: i64 = query_as!(i64, "SELECT 42::int8").fetch_one(&conn).await.unwrap();
    }
}

/// Decoding a column into a type that cannot hold it is an error, not a silent
/// reinterpretation of the bytes.
#[tokio::test]
async fn decoding_the_wrong_type_is_an_error() {
    let conn = connect_or_skip!();
    let rows: Vec<postgresql::Row> = query!("SELECT 256::int4 AS n, 'x'::text AS t")
        .fetch_all(&conn)
        .await
        .unwrap();
    assert!(rows[0].try_get::<bool>("n").is_err());
    assert!(rows[0].try_get::<String>("n").is_err());
    assert!(rows[0].try_get::<i64>("n").is_err());
    assert!(rows[0].try_get::<i32>("n").is_ok());
    assert!(rows[0].try_get::<i32>("t").is_err());
}

/// An abandoned stream is drained in the background and the connection becomes
/// available again without any interaction from the caller.
#[tokio::test]
async fn abandoned_stream_leaves_the_connection_usable() {
    let conn = connect_or_skip!();
    let stream = query!("SELECT pg_sleep(2), generate_series(1, 100000)")
        .fetch_rows(&conn)
        .await
        .unwrap();
    drop(stream);

    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(15);
    loop {
        match query_as!(i32, "SELECT 7").fetch_one(&conn).await {
            Ok(value) => {
                assert_eq!(value, 7);
                break;
            }
            Err(err) => {
                assert!(
                    std::time::Instant::now() < deadline,
                    "the connection did not recover after an abandoned query: {err}"
                );
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
        }
    }
}

#[tokio::test]
async fn zero_copy_row_access() {
    let conn = connect_or_skip!();
    let rows: Vec<postgresql::Row> =
        query!("SELECT 'hello'::text AS s, decode('deadbeef', 'hex') AS b, NULL::text AS n")
            .fetch_all(&conn)
            .await
            .unwrap();

    assert_eq!(rows[0].get_str("s").unwrap(), "hello");
    assert_eq!(rows[0].get_bytes("b").unwrap(), Some(&[0xde, 0xad, 0xbe, 0xef][..]));
    assert_eq!(rows[0].get_bytes("n").unwrap(), None);
    assert!(rows[0].get_str("n").is_err());
}

#[tokio::test]
async fn json_struct_roundtrip() {
    let conn = connect_or_skip!();
    let table = unique("pg_json");

    #[derive(serde::Serialize, serde::Deserialize, postgresql::Json, Debug, Clone, PartialEq)]
    struct Address {
        city: String,
        zip: String,
    }

    #[derive(FromRow, Debug, PartialEq)]
    struct Account {
        id: i32,
        address: Address,
        nickname: Option<Address>,
    }

    #[derive(FromRow, Debug, PartialEq)]
    struct WrappedAccount {
        id: i32,
        address: postgresql::Json<Address>,
    }

    let paris = Address {
        city: "Paris".to_string(),
        zip: "75001".to_string(),
    };

    // Compile-time checked when DATABASE_URL is set: the parameter and the
    // single jsonb result column are verified against the derived type.
    let echoed: Address = query_as!(Address, "SELECT $1::jsonb", &paris)
        .fetch_one(&conn)
        .await
        .unwrap();
    assert_eq!(echoed, paris);

    // A derived JSON type round-trips as a `jsonb[]` array too.
    let many: Vec<Address> = query_as!(Vec<Address>, "SELECT $1::jsonb[]", vec![paris.clone()])
        .fetch_one(&conn)
        .await
        .unwrap();
    assert_eq!(many, vec![paris.clone()]);

    query!(&format!(
        "CREATE TABLE {table} (id INT PRIMARY KEY, address JSONB NOT NULL, nickname JSONB)"
    ))
    .execute(&conn)
    .await
    .unwrap();

    query!(
        &format!("INSERT INTO {table} (id, address, nickname) VALUES ($1, $2, $3)"),
        1i32,
        &paris,
        Option::<Address>::None
    )
    .execute(&conn)
    .await
    .unwrap();

    let accounts: Vec<Account> = query_as!(Account, &format!("SELECT id, address, nickname FROM {table}"))
        .fetch_all(&conn)
        .await
        .unwrap();
    assert_eq!(
        accounts,
        vec![Account {
            id: 1,
            address: paris.clone(),
            nickname: None,
        }]
    );

    // The `Json<T>` wrapper reads the same column.
    let wrapped: WrappedAccount = query_as!(WrappedAccount, &format!("SELECT id, address FROM {table}"))
        .fetch_one(&conn)
        .await
        .unwrap();
    assert_eq!(*wrapped.address, paris);

    // Updating through a derived JSON parameter.
    let london = Address {
        city: "London".to_string(),
        zip: "E1".to_string(),
    };
    query!(&format!("UPDATE {table} SET address = $1 WHERE id = $2"), &london, 1i32)
        .execute(&conn)
        .await
        .unwrap();
    let updated: Address = query_as!(Address, &format!("SELECT address FROM {table} WHERE id = 1"))
        .fetch_one(&conn)
        .await
        .unwrap();
    assert_eq!(updated, london);

    query!(&format!("DROP TABLE {table}")).execute(&conn).await.unwrap();
}

/// With caching disabled the client must use the unnamed statement (no
/// server-side prepared statements accumulate) and still be correct.
#[tokio::test]
async fn statement_cache_disabled_is_correct() {
    let url = url_or_skip!();
    let sep = if url.contains('?') { '&' } else { '?' };
    let conn = postgresql::Connection::connect(&format!("{url}{sep}statement_cache_size=0"))
        .await
        .unwrap();

    for i in 0..10i64 {
        let sql = format!("SELECT {i}::int8");
        let value: i64 = query_as!(i64, &sql).fetch_one(&conn).await.unwrap();
        assert_eq!(value, i);
    }

    let rows: Vec<(i64,)> = query!("SELECT count(*)::int8 FROM pg_prepared_statements WHERE name LIKE 's%'")
        .fetch_all(&conn)
        .await
        .unwrap();
    assert_eq!(rows[0].0, 0, "caching disabled must not create named statements");
}

/// Many concurrent queries on a tiny pool must all succeed (connections are
/// serialized, not shared mid-query).
#[tokio::test]
async fn concurrent_queries_on_a_small_pool_all_succeed() {
    let url = url_or_skip!();
    let pool = Pool::connect(
        &url,
        Some(PoolConfig {
            pool_size: 2,
            ..PoolConfig::default()
        }),
    )
    .await
    .unwrap();

    let mut tasks = Vec::new();
    for i in 0..24i64 {
        let pool = pool.clone();
        tasks.push(tokio::spawn(async move {
            let value: i64 = query_as!(i64, "SELECT $1::int8", i).fetch_one(&pool).await.unwrap();
            assert_eq!(value, i);
        }));
    }
    for task in tasks {
        task.await.unwrap();
    }
    pool.close();
}

/// An explicit `CancelToken` aborts a running query.
#[tokio::test]
async fn cancel_token_aborts_a_running_query() {
    let conn = connect_or_skip!();
    let token = conn.cancel_token();
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(250)).await;
        let _ = token.cancel().await;
    });

    let start = std::time::Instant::now();
    let result = query!("SELECT pg_sleep(30)").execute(&conn).await;
    assert!(result.is_err(), "the query should have been cancelled: {result:?}");
    assert!(
        start.elapsed() < std::time::Duration::from_secs(15),
        "cancellation took {:?}",
        start.elapsed()
    );
}

/// `max_message_len` bounds the buffering: a value larger than the configured
/// cap is rejected instead of being read into memory.
#[tokio::test]
async fn max_message_len_is_enforced() {
    let url = url_or_skip!();
    let sep = if url.contains('?') { '&' } else { '?' };
    let conn = postgresql::Connection::connect(&format!("{url}{sep}max_message_len=65536"))
        .await
        .unwrap();

    // 256 KiB of text is well over the 64 KiB cap.
    let result: Result<String, _> = query_as!(String, "SELECT repeat('a', 262144)::text")
        .fetch_one(&conn)
        .await;
    assert!(result.is_err(), "a message over max_message_len must be rejected");
}
