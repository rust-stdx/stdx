//! The query builder surface used by the `query!` / `query_as!` macros, and
//! the [`Executor`] marker implemented by connections, pools and transactions.

use std::{future::Future, marker::PhantomData};

use futures_util::StreamExt;

use crate::{connection::RowStream, encode::ToSql, error::Error, row::FromRow};

pub(crate) mod sealed {
    pub trait Sealed {}
}

/// Internal execution capability. Not part of the public API.
///
/// Sealed: only this crate implements it. Query execution is reached through
/// the `query!` / `query_as!` macros, never by calling these methods directly.
#[doc(hidden)]
pub trait Capable: sealed::Sealed + Send + Sync {
    /// Runs a statement without collecting rows and returns the number of
    /// affected rows.
    fn execute<'a>(
        &'a self,
        sql: &'a str,
        params: &'a [&'a dyn ToSql],
    ) -> impl Future<Output = Result<u64, Error>> + Send + 'a;

    /// Runs a statement and returns a stream of rows.
    ///
    /// `max_rows` limits the number of rows the server sends (`0` means no
    /// limit).
    fn fetch_rows<'a>(
        &'a self,
        sql: &'a str,
        params: &'a [&'a dyn ToSql],
        max_rows: i32,
    ) -> impl Future<Output = Result<RowStream, Error>> + Send + 'a;
}

/// Anything a query can run against: a connection, a pooled connection, a
/// pool, or a transaction.
///
/// This is a sealed marker trait, so generic helpers can accept
/// `E: Executor`, but queries themselves are only run through the `query!` /
/// `query_as!` macros.
pub trait Executor: Capable {}

impl<T: Capable + ?Sized> Executor for T {}

/// Runs multi-statement SQL scripts via the simple query protocol.
///
/// This is the unchecked, parameterless path (for migrations, `SET` scripts and
/// statements the extended protocol cannot express, such as `VACUUM`). Use
/// `query!` / `query_as!` for anything parameterized or typed.
pub trait ScriptExecutor: sealed::Sealed {
    /// Runs one or more statements and returns the affected-row count of the
    /// last command.
    ///
    /// # Errors
    ///
    /// Returns an error if the connection is closed or the server rejects a
    /// statement.
    fn execute_script<'a>(&'a self, sql: &'a str) -> impl Future<Output = Result<u64, Error>> + Send + 'a;
}

/// A prepared query returning untyped [`crate::Row`]s.
pub struct Query<'a> {
    sql: &'a str,
    params: &'a [&'a dyn ToSql],
}

impl<'a> Query<'a> {
    /// Builds a query from SQL and already-built parameters.
    pub fn new(sql: &'a str, params: &'a [&'a dyn ToSql]) -> Self {
        Query {
            sql,
            params,
        }
    }

    /// Runs the statement and returns the number of affected rows.
    ///
    /// Use this for statements that change data (`INSERT`, `UPDATE`, `DELETE`,
    /// DDL). Any rows the statement produces are read and **discarded**; the
    /// returned count is the command tag's affected-row count (for example the
    /// number of inserted or updated rows).
    ///
    /// # Errors
    ///
    /// Returns a server error if the statement fails, or [`Error::Closed`] if
    /// the connection is gone.
    pub async fn execute<E: Executor>(&self, executor: &E) -> Result<u64, Error> {
        executor.execute(self.sql, self.params).await
    }

    /// Runs the statement and returns a stream of rows.
    pub async fn fetch_rows<E: Executor>(&self, executor: &E) -> Result<RowStream, Error> {
        executor.fetch_rows(self.sql, self.params, 0).await
    }

    /// Collects all rows into a `Vec<T>`.
    ///
    /// The result is unbounded: use [`Query::fetch_rows`] to process a large
    /// result without holding it all in memory.
    pub async fn fetch_all<E: Executor, T: FromRow>(&self, executor: &E) -> Result<Vec<T>, Error> {
        let mut rows = executor.fetch_rows(self.sql, self.params, 0).await?;
        let mut out = Vec::new();
        while let Some(row) = rows.next().await {
            out.push(T::from_row(&row?)?);
        }
        Ok(out)
    }

    /// Returns exactly one row, or [`Error::RowNotFound`].
    pub async fn fetch_one<E: Executor, T: FromRow>(&self, executor: &E) -> Result<T, Error> {
        let mut rows = executor.fetch_rows(self.sql, self.params, 1).await?;
        match rows.next().await {
            Some(row) => T::from_row(&row?),
            None => Err(Error::RowNotFound),
        }
    }

    /// Returns the first row, if any.
    pub async fn fetch_optional<E: Executor, T: FromRow>(&self, executor: &E) -> Result<Option<T>, Error> {
        let mut rows = executor.fetch_rows(self.sql, self.params, 1).await?;
        match rows.next().await {
            Some(row) => Ok(Some(T::from_row(&row?)?)),
            None => Ok(None),
        }
    }
}

/// A prepared query decoded into `T`.
pub struct QueryAs<'a, T> {
    query: Query<'a>,
    _marker: PhantomData<fn() -> T>,
}

impl<'a, T: FromRow> QueryAs<'a, T> {
    /// Builds a typed query from SQL and already-built parameters.
    pub fn new(sql: &'a str, params: &'a [&'a dyn ToSql]) -> Self {
        QueryAs {
            query: Query::new(sql, params),
            _marker: PhantomData,
        }
    }

    /// Runs the statement and returns the number of affected rows.
    ///
    /// See [`Query::execute`]: rows produced by the statement are discarded.
    pub async fn execute<E: Executor>(&self, executor: &E) -> Result<u64, Error> {
        self.query.execute(executor).await
    }

    /// Runs the statement and returns a stream of rows.
    pub async fn fetch_rows<E: Executor>(&self, executor: &E) -> Result<RowStream, Error> {
        self.query.fetch_rows(executor).await
    }

    /// Collects all rows into a `Vec<T>`.
    pub async fn fetch_all<E: Executor>(&self, executor: &E) -> Result<Vec<T>, Error> {
        self.query.fetch_all::<E, T>(executor).await
    }

    /// Returns exactly one row, or [`Error::RowNotFound`].
    pub async fn fetch_one<E: Executor>(&self, executor: &E) -> Result<T, Error> {
        self.query.fetch_one::<E, T>(executor).await
    }

    /// Returns the first row, if any.
    pub async fn fetch_optional<E: Executor>(&self, executor: &E) -> Result<Option<T>, Error> {
        self.query.fetch_optional::<E, T>(executor).await
    }
}
