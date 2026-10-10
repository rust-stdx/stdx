//! Transactions.

use std::{future::Future, sync::Arc};

use crate::{
    connection::{Connection, RowStream},
    encode::ToSql,
    error::Error,
    pool::Checkout,
};

/// An open transaction.
///
/// Owns its connection (no lifetime), so it can be passed to generic code that
/// accepts any [`crate::Executor`]. Dropping without committing enqueues a `ROLLBACK`
/// and, for pooled transactions, returns the connection to its pool once any
/// row stream taken from the transaction has also been dropped.
pub struct Transaction {
    checkout: Option<Arc<Checkout>>,
    done: bool,
}

impl Transaction {
    /// Begins a transaction on a connection.
    ///
    /// # Errors
    ///
    /// Returns an error if `BEGIN` fails.
    pub async fn begin(conn: Connection) -> Result<Self, Error> {
        Self::begin_checkout(Arc::new(Checkout::detached(conn))).await
    }

    /// Runs `BEGIN` on a freshly built checkout.
    ///
    /// On failure the checkout is dropped, which returns the connection to its
    /// pool (or closes it).
    pub(crate) async fn begin_checkout(checkout: Arc<Checkout>) -> Result<Self, Error> {
        match checkout.connection().execute("BEGIN", &[]).await {
            Ok(_) => Ok(Transaction {
                checkout: Some(checkout),
                done: false,
            }),
            Err(err) => {
                drop(checkout);
                Err(err)
            }
        }
    }

    /// Commits the transaction.
    ///
    /// # Errors
    ///
    /// Returns an error if `COMMIT` fails. The transaction is considered
    /// finished either way.
    pub async fn commit(mut self) -> Result<(), Error> {
        let conn = self.connection();
        let result = conn.execute("COMMIT", &[]).await.map(|_| ());
        self.done = true;
        result
    }

    /// Rolls the transaction back.
    ///
    /// # Errors
    ///
    /// Returns an error if `ROLLBACK` fails.
    pub async fn rollback(mut self) -> Result<(), Error> {
        let conn = self.connection();
        let result = conn.execute("ROLLBACK", &[]).await.map(|_| ());
        self.done = true;
        result
    }

    fn checkout(&self) -> &Checkout {
        self.checkout.as_deref().expect("transaction already completed")
    }

    fn connection(&self) -> &Connection {
        self.checkout().connection()
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        if let Some(checkout) = self.checkout.take() {
            if !self.done {
                // Enqueue a rollback; the actor serialises it before any later
                // query on this connection, so no `tokio::spawn` is needed.
                checkout.connection().enqueue("ROLLBACK");
            }
            // Dropping the checkout returns the connection to the pool once any
            // stream taken from this transaction is also gone.
            drop(checkout);
        }
    }
}

impl crate::query::sealed::Sealed for Transaction {}

impl crate::query::Capable for Transaction {
    fn execute<'a>(
        &'a self,
        sql: &'a str,
        params: &'a [&'a dyn ToSql],
    ) -> impl Future<Output = Result<u64, Error>> + Send + 'a {
        async move { self.connection().execute(sql, params).await }
    }

    fn fetch_rows<'a>(
        &'a self,
        sql: &'a str,
        params: &'a [&'a dyn ToSql],
        max_rows: i32,
    ) -> impl Future<Output = Result<RowStream, Error>> + Send + 'a {
        async move {
            // Tie a clone of the checkout to the stream: the connection (and the
            // rollback-on-drop) is only settled once the stream is gone.
            let checkout = self.checkout.as_ref().expect("transaction already completed").clone();
            let stream = checkout.connection().fetch_rows(sql, params, max_rows).await?;
            Ok(stream.with_keepalive(checkout))
        }
    }
}

impl crate::query::ScriptExecutor for Transaction {
    fn execute_script<'a>(&'a self, sql: &'a str) -> impl Future<Output = Result<u64, Error>> + Send + 'a {
        async move { self.connection().execute_script(sql).await }
    }
}
