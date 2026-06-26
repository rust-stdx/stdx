use bytes::Bytes;

use crate::{
    Error,
    connection::{ClientConnection, ServerConnection},
    error::{IoError, IoErrorKind},
};

/// Async read trait — no_std compatible.
pub trait AsyncRead {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, IoError>;
}

/// Async write trait — no_std compatible.
pub trait AsyncWrite {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, IoError>;
    async fn flush(&mut self) -> Result<(), IoError>;

    async fn write_all(&mut self, mut buf: &[u8]) -> Result<(), IoError> {
        while !buf.is_empty() {
            let n = self.write(buf).await?;
            if n == 0 {
                return Err(IoError::new(IoErrorKind::WriteZero, "write returned 0"));
            }
            buf = &buf[n..];
        }
        Ok(())
    }
}

/// Wraps a [`ClientConnection`] over an async read+write stream.
pub struct ClientAsyncIo<S> {
    conn: ClientConnection,
    stream: S,
}

impl<S: AsyncRead + AsyncWrite> ClientAsyncIo<S> {
    pub async fn new(mut conn: ClientConnection, mut stream: S) -> Result<Self, Error> {
        while let Some(data) = conn.write_tls() {
            stream.write_all(&data).await.map_err(|e| Error::Io(e))?;
        }
        stream.flush().await.map_err(|e| Error::Io(e))?;
        Ok(Self {
            conn,
            stream,
        })
    }

    pub async fn handshake(&mut self) -> Result<(), Error> {
        loop {
            while let Some(data) = self.conn.write_tls() {
                self.stream.write_all(&data).await.map_err(|e| Error::Io(e))?;
                self.stream.flush().await.map_err(|e| Error::Io(e))?;
            }
            if self.conn.handshake_done() {
                return Ok(());
            }
            let mut buf = [0u8; 16384];
            let n = self.stream.read(&mut buf).await.map_err(|e| Error::Io(e))?;
            if n == 0 {
                return Err(Error::ConnectionClosed);
            }
            self.conn.inject(&buf[..n]);
            self.conn.process().await?;
        }
    }

    pub async fn read(&mut self) -> Result<Bytes, Error> {
        loop {
            while let Some(data) = self.conn.write_tls() {
                self.stream.write_all(&data).await.map_err(|e| Error::Io(e))?;
                self.stream.flush().await.map_err(|e| Error::Io(e))?;
            }
            if let Some(data) = self.conn.read_app_data() {
                return Ok(data);
            }
            let mut buf = [0u8; 16384];
            let n = self.stream.read(&mut buf).await.map_err(|e| Error::Io(e))?;
            if n == 0 {
                return Err(Error::ConnectionClosed);
            }
            self.conn.inject(&buf[..n]);
            self.conn.process().await?;
        }
    }

    pub async fn write(&mut self, data: &[u8]) -> Result<(), Error> {
        let encrypted = self.conn.send(data)?;
        self.stream.write_all(&encrypted).await.map_err(|e| Error::Io(e))?;
        self.stream.flush().await.map_err(|e| Error::Io(e))?;
        Ok(())
    }

    pub async fn close(&mut self) -> Result<(), Error> {
        let close_msg = self.conn.close()?;
        self.stream.write_all(&close_msg).await.map_err(|e| Error::Io(e))?;
        self.stream.flush().await.map_err(|e| Error::Io(e))?;
        Ok(())
    }

    pub fn conn(&self) -> &ClientConnection {
        &self.conn
    }
}

/// Wraps a [`ServerConnection`] over an async read+write stream.
pub struct ServerAsyncIo<S> {
    conn: ServerConnection,
    stream: S,
}

impl<S: AsyncRead + AsyncWrite> ServerAsyncIo<S> {
    pub fn new(conn: ServerConnection, stream: S) -> Self {
        Self {
            conn,
            stream,
        }
    }

    pub async fn handshake(&mut self) -> Result<(), Error> {
        loop {
            while let Some(data) = self.conn.write_tls() {
                self.stream.write_all(&data).await.map_err(|e| Error::Io(e))?;
                self.stream.flush().await.map_err(|e| Error::Io(e))?;
            }
            if self.conn.handshake_done() {
                return Ok(());
            }
            let mut buf = [0u8; 16384];
            let n = self.stream.read(&mut buf).await.map_err(|e| Error::Io(e))?;
            if n == 0 {
                return Err(Error::ConnectionClosed);
            }
            self.conn.inject(&buf[..n]);
            self.conn.process().await?;
        }
    }

    pub async fn read(&mut self) -> Result<Bytes, Error> {
        loop {
            while let Some(data) = self.conn.write_tls() {
                self.stream.write_all(&data).await.map_err(|e| Error::Io(e))?;
                self.stream.flush().await.map_err(|e| Error::Io(e))?;
            }
            if let Some(data) = self.conn.read_app_data() {
                return Ok(data);
            }
            let mut buf = [0u8; 16384];
            let n = self.stream.read(&mut buf).await.map_err(|e| Error::Io(e))?;
            if n == 0 {
                return Err(Error::ConnectionClosed);
            }
            self.conn.inject(&buf[..n]);
            self.conn.process().await?;
        }
    }

    pub async fn write(&mut self, data: &[u8]) -> Result<(), Error> {
        let encrypted = self.conn.send(data)?;
        self.stream.write_all(&encrypted).await.map_err(|e| Error::Io(e))?;
        self.stream.flush().await.map_err(|e| Error::Io(e))?;
        Ok(())
    }

    pub async fn close(&mut self) -> Result<(), Error> {
        let close_msg = self.conn.close()?;
        self.stream.write_all(&close_msg).await.map_err(|e| Error::Io(e))?;
        self.stream.flush().await.map_err(|e| Error::Io(e))?;
        Ok(())
    }

    pub fn conn(&self) -> &ServerConnection {
        &self.conn
    }
}
