use crate::{
    Error,
    value::{ToValue, Value},
};

/// A template rendering context.
///
/// Constructed via the [`context!`](crate::context) macro, through the
/// [`IntoContext`] trait, or by passing a reference to a type that derives
/// [`ToValue`] to [`Engine::render`](crate::Engine::render).
pub struct Context(pub(crate) Value);

impl From<Context> for Value {
    fn from(ctx: Context) -> Value {
        ctx.0
    }
}

impl ToValue for Context {
    fn to_value(&self) -> Value {
        self.0.clone()
    }
}

/// Trait for types that can be converted into a [`Context`] for template rendering.
///
/// Implemented for [`Context`] directly and for references to any type that
/// implements [`ToValue`]. The resulting value must be a map (i.e.
/// [`Value::Map`](crate::value::Value::Map)); non-map values such as plain
/// strings or numbers return an error.
pub trait IntoContext {
    fn into_context(self) -> Result<Context, Error>;
}

impl IntoContext for Context {
    /// Returns itself
    fn into_context(self) -> Result<Context, Error> {
        Ok(self)
    }
}

impl<T: ToValue + ?Sized> IntoContext for &T {
    fn into_context(self) -> Result<Context, Error> {
        let value = self.to_value();
        match value {
            Value::Map(_) => Ok(Context(value)),
            _ => Err(Error::r#type("context must be a map-like value")),
        }
    }
}
