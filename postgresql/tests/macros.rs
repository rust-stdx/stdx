//! Tests for the `query!` / `query_as!` macros.
//!
//! Run with `DATABASE_URL` set to exercise compile-time checking; the same code
//! also compiles (unchecked) without it.

use postgresql::{FromRow, Pool, PoolConfig, query, query_as};

fn database_url() -> Option<String> {
    std::env::var("DATABASE_URL").ok().filter(|u| !u.is_empty())
}

#[tokio::test]
async fn macro_scalar() {
    let Some(url) = database_url() else {
        eprintln!("skipping: DATABASE_URL is not set");
        return;
    };
    let conn = postgresql::Connection::connect(&url).await.unwrap();

    let value: i64 = query_as!(i64, "SELECT (41 + 1)::int8").fetch_one(&conn).await.unwrap();
    assert_eq!(value, 42);

    let text: String = query_as!(String, "SELECT 'hi'::text").fetch_one(&conn).await.unwrap();
    assert_eq!(text, "hi");
}

#[tokio::test]
async fn macro_multiple_parameters() {
    let Some(url) = database_url() else {
        eprintln!("skipping: DATABASE_URL is not set");
        return;
    };
    let conn = postgresql::Connection::connect(&url).await.unwrap();

    let row: (bool, i32, String) =
        query_as!((bool, i32, String), "SELECT $1::bool, $2::int4, $3::text", true, 18i32, "hello")
            .fetch_one(&conn)
            .await
            .unwrap();
    assert_eq!(row, (true, 18, "hello".to_string()));
}

#[derive(FromRow, Debug, PartialEq)]
struct User {
    id: i32,
    name: String,
}

#[derive(FromRow)]
struct PgTypeRow {
    typname: String,
    typlen: i16,
}

#[tokio::test]
async fn macro_struct_and_array() {
    let Some(url) = database_url() else {
        eprintln!("skipping: DATABASE_URL is not set");
        return;
    };
    let pool = Pool::connect(
        &url,
        Some(PoolConfig {
            pool_size: 3,
            min_connections: 0,
            ..PoolConfig::default()
        }),
    )
    .await
    .unwrap();

    let table = format!("pg_macro_{}", uuid::Uuid::new_v4().to_string().replace('-', "_"));
    query!(&format!("CREATE TABLE {table} (id INT PRIMARY KEY, name TEXT NOT NULL)"))
        .execute(&pool)
        .await
        .unwrap();

    let ids = vec![1i32, 2, 3];
    let names = vec!["a", "b", "c"];
    let affected = query!(
        "INSERT INTO pg_macro_placeholder (id, name) SELECT * FROM unnest($1::int4[], $2::text[])"
            .replace("pg_macro_placeholder", &table)
            .as_str(),
        ids,
        postgresql::array(names.iter())
    )
    .execute(&pool)
    .await
    .unwrap();
    assert_eq!(affected, 3);

    let users: Vec<User> = query_as!(User, &format!("SELECT id, name FROM {table} ORDER BY id") as &str,)
        .fetch_all(&pool)
        .await
        .unwrap();
    assert_eq!(users.len(), 3);
    assert_eq!(users[0].name, "a");

    // Compile-time-checked literal query (schema is known at build time).
    let count: i64 = query_as!(i64, "SELECT count(*)::int8 FROM pg_catalog.pg_class")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(count >= 0);

    query!(&format!("DROP TABLE {table}")).execute(&pool).await.unwrap();
}

#[tokio::test]
async fn macro_unchecked_dynamic_sql() {
    let Some(url) = database_url() else {
        eprintln!("skipping: DATABASE_URL is not set");
        return;
    };
    let conn = postgresql::Connection::connect(&url).await.unwrap();

    let sql = "SELECT $1::int4".to_string();
    let value: i32 = query_as!(i32, &sql, 7i32).fetch_one(&conn).await.unwrap();
    assert_eq!(value, 7);
}

#[tokio::test]
async fn macro_struct_shape_checked() {
    let Some(url) = database_url() else {
        eprintln!("skipping: DATABASE_URL is not set");
        return;
    };
    let conn = postgresql::Connection::connect(&url).await.unwrap();

    // Checked at compile time: names, OIDs and nullability all match.
    let row: PgTypeRow = query_as!(PgTypeRow, "SELECT typname, typlen FROM pg_catalog.pg_type LIMIT 1")
        .fetch_one(&conn)
        .await
        .unwrap();
    let _ = (row.typname, row.typlen);
}
