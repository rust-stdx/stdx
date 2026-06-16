#[cfg(feature = "std")]
use serde::ser::Serialize;

#[cfg(feature = "std")]
use crate::value::to_value;
use crate::{SerdeError, value::Value};

/// A template rendering context.
///
/// Constructed via the [`context!`](crate::context) macro or through the
/// [`IntoContext`] trait (implemented for all `Serialize` types when the
/// `std` feature is enabled).
pub struct Context(pub(crate) Value);

impl From<Context> for Value {
    fn from(ctx: Context) -> Value {
        ctx.0
    }
}

/// Trait for types that can be converted into a [`Context`] for template rendering.
///
/// Implemented for [`Context`] directly and, with the `std` feature, for all
/// types that implement [`Serialize`](serde::Serialize).
///
/// The `std` blanket impl requires the result to be a map-like value
/// (i.e. [`Value::Map`](crate::value::Value::Map)). Non-map values such as
/// plain strings or numbers return an error.
pub trait IntoContext {
    fn into_context(self) -> Result<Context, SerdeError>;
}

impl IntoContext for Context {
    /// Returns itself
    fn into_context(self) -> Result<Context, SerdeError> {
        Ok(self)
    }
}

#[cfg(feature = "std")]
impl<T: Serialize + ?Sized> IntoContext for &T {
    fn into_context(self) -> Result<Context, SerdeError> {
        let v = to_value(self)?;
        match v {
            Value::Map(_) => Ok(Context(v)),
            _ => Err(SerdeError("context must be a map-like value".into())),
        }
    }
}
