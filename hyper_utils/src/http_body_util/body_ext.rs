use crate::http_body_util::{Frame, combinators};

/// An extension trait for [`hyper::body::Body`] adding various combinators and adapters
pub trait BodyExt: hyper::body::Body {
    /// Returns a future that resolves to the next [`Frame`], if any.
    fn frame(&mut self) -> Frame<'_, Self>
    where
        Self: Unpin,
    {
        Frame(self)
    }

    /// Turn this body into [`Collected`](crate::http_body_util::Collected) body which will collect all the DATA frames
    /// and trailers.
    fn collect(self) -> combinators::Collect<Self>
    where
        Self: Sized,
    {
        combinators::Collect {
            body: self,
            collected: Some(super::Collected::default()),
        }
    }
}

impl<T: ?Sized> BodyExt for T where T: hyper::body::Body {}
