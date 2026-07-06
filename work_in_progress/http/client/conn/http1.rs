#[cfg(feature = "tls")]
use tls::io_tokio::{TlsConnector, TlsStream};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use super::super::{error::Error, pool::PoolKey};
use crate::{
    common::{Request, Response},
    http1::{ResponseDecoder, encode_request},
};

pub(crate) struct Http1Conn {
    pub key: PoolKey,
    stream: Option<Http1Stream>,
    reuse: bool,
}

enum Http1Stream {
    Tcp(TcpStream),
    #[cfg(feature = "tls")]
    TlsTunnel(TlsStream<TcpStream>),
}

impl Http1Conn {
    pub async fn connect_tcp(host: &str, port: u16) -> Result<Self, Error> {
        let addr = tokio::net::lookup_host(format!("{host}:{port}"))
            .await
            .map_err(|e| Error::Dns(e.to_string()))?
            .next()
            .ok_or_else(|| Error::Dns(format!("could not resolve {host}:{port}")))?;

        let stream = TcpStream::connect(addr)
            .await
            .map_err(|e| Error::Connect(e.to_string()))?;

        Ok(Http1Conn {
            key: PoolKey {
                host: host.to_string(),
                port,
                tls: false,
            },
            stream: Some(Http1Stream::Tcp(stream)),
            reuse: true,
        })
    }

    #[cfg(feature = "tls")]
    pub async fn connect_tls(host: &str, port: u16, alpn: Vec<Bytes>) -> Result<Self, Error> {
        use std::sync::Arc;

        use tls::{
            ClientConfig,
            config::{CertificateValidator, ReceivedCertificate},
            crypto_default_provider::DefaultCryptoProvider,
        };

        struct AcceptAll;
        #[async_trait::async_trait]
        impl CertificateValidator for AcceptAll {
            async fn validate(
                &self,
                _cert: &ReceivedCertificate,
                _server_name: Option<&str>,
            ) -> Result<(), tls::Error> {
                Ok(())
            }
        }

        let addr = tokio::net::lookup_host(format!("{host}:{port}"))
            .await
            .map_err(|e| Error::Dns(e.to_string()))?
            .next()
            .ok_or_else(|| Error::Dns(format!("could not resolve {host}:{port}")))?;

        let tcp = TcpStream::connect(addr)
            .await
            .map_err(|e| Error::Connect(e.to_string()))?;

        let config = ClientConfig::new(Arc::new(DefaultCryptoProvider::new()), alpn, Arc::new(AcceptAll));
        let connector = TlsConnector::new(config);
        let stream = connector
            .connect(host, tcp)
            .await
            .map_err(|e| Error::Tls(e.to_string()))?;

        let alpn_result = stream.alpn_protocol();
        let reuse = alpn_result.map_or(true, |b| b.as_ref() != b"h2");

        Ok(Http1Conn {
            key: PoolKey {
                host: host.to_string(),
                port,
                tls: true,
            },
            stream: Some(Http1Stream::TlsTunnel(stream)),
            reuse,
        })
    }

    pub fn can_reuse(&self) -> bool {
        self.reuse
    }

    pub fn set_can_reuse(&mut self, v: bool) {
        self.reuse = v;
    }

    #[cfg(feature = "tls")]
    pub fn take_tls_stream(&mut self) -> Option<TlsStream<TcpStream>> {
        if !self.reuse {
            if let Some(Http1Stream::TlsTunnel(s)) = self.stream.take() {
                return Some(s);
            }
        }
        None
    }
}

pub(crate) async fn send(conn: &mut Http1Conn, req: Request) -> Result<Response, Error> {
    let req = if req.method != crate::common::Method::Connect {
        let has_host = req.headers.iter().any(|(n, _)| n.as_str() == "host");
        if !has_host {
            let mut r = req;
            r.headers.push(("host".into(), conn.key.host.as_str().into()));
            r
        } else {
            req
        }
    } else {
        req
    };

    let encoded = encode_request(req)
        .await
        .map_err(|e| Error::BodyError(format!("{e:?}")))?;

    {
        let stream = conn.stream.as_mut().ok_or(Error::ConnectionClosed)?;
        match stream {
            Http1Stream::Tcp(s) => {
                s.write_all(&encoded).await.map_err(Error::from)?;
            }
            #[cfg(feature = "tls")]
            Http1Stream::TlsTunnel(s) => {
                s.write_all(&encoded).await.map_err(Error::from)?;
            }
        }
    }

    let mut decoder = ResponseDecoder::new();
    let mut recv_buf = vec![0u8; 65536];

    loop {
        let n = {
            let stream = conn.stream.as_mut().ok_or(Error::ConnectionClosed)?;
            match stream {
                Http1Stream::Tcp(s) => s.read(&mut recv_buf).await.map_err(Error::from)?,
                #[cfg(feature = "tls")]
                Http1Stream::TlsTunnel(s) => s.read(&mut recv_buf).await.map_err(Error::from)?,
            }
        };
        if n == 0 {
            return Err(Error::ConnectionClosed);
        }
        match decoder.feed(&recv_buf[..n]).map_err(|e| Error::H1(format!("{e}")))? {
            Some((resp, _remaining)) => {
                let close = resp
                    .headers
                    .iter()
                    .any(|(n, v)| n.as_str() == "connection" && v.as_str().eq_ignore_ascii_case("close"));
                conn.set_can_reuse(!close);
                return Ok(resp);
            }
            None => {}
        }
    }
}
