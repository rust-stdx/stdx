pub(crate) mod http1;
#[cfg(feature = "http2")]
pub(crate) mod http2;
#[cfg(feature = "http3")]
pub(crate) mod http3;

use std::sync::Arc;

use super::{error::Error, pool::PoolKey};
use crate::common::{Request, Response};

pub(crate) enum ConnRef {
    Http1(http1::Http1Conn),
    #[cfg(feature = "http2")]
    Http2(Arc<http2::Http2Conn>),
    #[cfg(feature = "http3")]
    #[allow(dead_code)]
    Http3(PoolKey, Arc<tokio::sync::Mutex<http3::Http3Conn>>),
}

impl ConnRef {
    pub fn pool_key(&self) -> &PoolKey {
        match self {
            ConnRef::Http1(c) => &c.key,
            #[cfg(feature = "http2")]
            ConnRef::Http2(c) => &c.key,
            #[cfg(feature = "http3")]
            ConnRef::Http3(key, _) => key,
        }
    }

    pub fn can_reuse(&self) -> bool {
        match self {
            ConnRef::Http1(c) => c.can_reuse(),
            #[cfg(feature = "http2")]
            ConnRef::Http2(_) => true,
            #[cfg(feature = "http3")]
            ConnRef::Http3(_, _) => true,
        }
    }

    pub async fn send(&mut self, req: Request) -> Result<Response, Error> {
        match self {
            ConnRef::Http1(c) => http1::send(c, req).await,
            #[cfg(feature = "http2")]
            ConnRef::Http2(c) => c.send_request(req).await,
            #[cfg(feature = "http3")]
            ConnRef::Http3(_, c) => {
                let mut guard = c.lock().await;
                guard.send_request(req).await
            }
        }
    }
}
