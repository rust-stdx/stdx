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
    let names = ["a", "b", "c"];
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

/// `Option<T>` is always a safe target, even for a NOT NULL column.
#[tokio::test]
async fn option_targets_are_allowed_for_not_null_columns() {
    let Some(url) = database_url() else {
        eprintln!("skipping: DATABASE_URL is not set");
        return;
    };
    let conn = postgresql::Connection::connect(&url).await.unwrap();

    let name: Option<String> = query_as!(Option<String>, "SELECT typname FROM pg_catalog.pg_type LIMIT 1")
        .fetch_one(&conn)
        .await
        .unwrap();
    assert!(name.is_some());
}

/// Nullability is resolved per attribute, not by result-column position.
#[tokio::test]
async fn nullability_is_matched_by_attribute_not_position() {
    let Some(url) = database_url() else {
        eprintln!("skipping: DATABASE_URL is not set");
        return;
    };
    let conn = postgresql::Connection::connect(&url).await.unwrap();

    // `attoptions` (nullable text[]) is selected before `atttypmod` (NOT NULL
    // int4), so the result order is the reverse of the attribute order. Before
    // the fix the two nullability flags were swapped and this did not compile.
    let row: (Option<Vec<String>>, i32) = query_as!(
        (Option<Vec<String>>, i32),
        "SELECT attoptions, atttypmod FROM pg_catalog.pg_attribute LIMIT 1"
    )
    .fetch_one(&conn)
    .await
    .unwrap();
    let _ = row;
}

/// View columns cannot carry NOT NULL constraints, so their nullability is
/// unknown and a non-Option target is allowed.
#[tokio::test]
async fn view_columns_can_decode_into_non_option_fields() {
    let Some(url) = database_url() else {
        eprintln!("skipping: DATABASE_URL is not set");
        return;
    };
    let conn = postgresql::Connection::connect(&url).await.unwrap();

    let name: String = query_as!(String, "SELECT tablename FROM pg_catalog.pg_tables LIMIT 1")
        .fetch_one(&conn)
        .await
        .unwrap();
    assert!(!name.is_empty());
}

#[derive(FromRow, Debug)]
struct DefaultedRow {
    typname: String,
    #[postgresql(default)]
    typlen: i16,
}

/// A type whose only field is defaulted (the shape must still be typed).
#[derive(FromRow)]
struct AllDefaultRow {
    #[postgresql(default)]
    value: i64,
}

/// The derive must compile for a struct with no fields at all.
#[allow(dead_code)]
#[derive(FromRow)]
struct EmptyRow {}

#[tokio::test]
async fn default_fields_decode_and_fall_back_on_null() {
    let Some(url) = database_url() else {
        eprintln!("skipping: DATABASE_URL is not set");
        return;
    };
    let conn = postgresql::Connection::connect(&url).await.unwrap();

    // Compile-time checked: the defaulted column is present.
    let row: DefaultedRow = query_as!(DefaultedRow, "SELECT typname, typlen FROM pg_catalog.pg_type LIMIT 1")
        .fetch_one(&conn)
        .await
        .unwrap();
    assert!(!row.typname.is_empty());

    // NULL falls back to Default.
    let sql = "SELECT 'x'::text AS typname, NULL::int2 AS typlen".to_string();
    let row: DefaultedRow = query_as!(DefaultedRow, &sql).fetch_one(&conn).await.unwrap();
    assert_eq!(row.typlen, 0);

    // A wrong-typed column is an error, not a silent default.
    let sql = "SELECT 'x'::text AS typname, true AS typlen".to_string();
    let err = query_as!(DefaultedRow, &sql).fetch_one(&conn).await.unwrap_err();
    assert!(err.to_string().contains("cannot be decoded"), "{err}");

    let sql = "SELECT NULL::int8 AS value".to_string();
    let row: AllDefaultRow = query_as!(AllDefaultRow, &sql).fetch_one(&conn).await.unwrap();
    assert_eq!(row.value, 0);
}
