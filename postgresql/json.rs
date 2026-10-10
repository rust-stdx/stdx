//! `json` / `jsonb` support via `serde`.
//!
//! [`Json`] stores any `serde`-serializable value in a PostgreSQL `json` or
//! `jsonb` column. It is the escape hatch for the cases where the preferred
//! `#[derive(Json)]` form cannot be used:
//!
//! * the type already derives [`FromRow`](crate::FromRow) and is row-shaped,
//!   so deriving `Json` too would define `FromRow`/`RowShape` twice;
//! * the value has no nameable type — for example a `HashMap` or a foreign
//!   type — so there is nothing to put the derive on.
//!
//! Values are encoded as `jsonb`; on decode both `json` and `jsonb` columns are
//! accepted.

use std::ops::{Deref, DerefMut};

/// A value stored as a PostgreSQL `json` / `jsonb` column.
///
/// Wrapping a value makes it a query parameter (encoded as `jsonb`) and lets
/// it be decoded from a result column. See the crate documentation for the
/// preferred `#[derive(Json)]` form and when to reach for this wrapper.
///
/// ```ignore
/// #[derive(FromRow)]
/// struct Something {
///     id: i32,
///     user: Json<HashMap<String, i32>>,
/// }
///
/// query_as!(Something, "SELECT id, user FROM something")
///     .fetch_all(&pool)
///     .await?;
/// query!("UPDATE something SET user = $1", something.user)
///     .execute(&pool)
///     .await?;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Json<T>(pub T);

impl<T> Json<T> {
    /// Wraps `value` so it is stored as `jsonb`.
    pub const fn new(value: T) -> Self {
        Json(value)
    }

    /// Unwraps the value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for Json<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for Json<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> AsRef<T> for Json<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> From<T> for Json<T> {
    fn from(value: T) -> Self {
        Json(value)
    }
}
