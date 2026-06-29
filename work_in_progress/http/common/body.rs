use std::future::Future;

use crate::common::Headers;

pub enum Frame {
    Data(bytes::Bytes),
    Trailers(Headers),
}

pub trait Body {
    type Error;

    fn next_frame(&mut self) -> impl Future<Output = Option<Result<Frame, Self::Error>>>;

    fn size_hint(&self) -> Option<usize> {
        None
    }
}

impl Body for bytes::Bytes {
    type Error = std::convert::Infallible;

    async fn next_frame(&mut self) -> Option<Result<Frame, Self::Error>> {
        if self.is_empty() {
            None
        } else {
            let data = std::mem::replace(self, bytes::Bytes::new());
            Some(Ok(Frame::Data(data)))
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl Body for Vec<u8> {
    type Error = std::convert::Infallible;

    async fn next_frame(&mut self) -> Option<Result<Frame, Self::Error>> {
        if self.is_empty() {
            None
        } else {
            let data = std::mem::replace(self, Vec::new());
            Some(Ok(Frame::Data(bytes::Bytes::from(data))))
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl Body for () {
    type Error = std::convert::Infallible;

    async fn next_frame(&mut self) -> Option<Result<Frame, Self::Error>> {
        None
    }

    fn size_hint(&self) -> Option<usize> {
        Some(0)
    }
}
