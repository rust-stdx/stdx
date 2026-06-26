use bytes::Bytes;
use futures_util::io::{AsyncReadExt, AsyncWriteExt};

use crate::{
    Error,
    connection::{ClientConnection, ServerConnection},
};

/// Wraps a [`ClientConnection`] over an asynchronous `AsyncRead + AsyncWrite` stream.
pub struct ClientAsyncIo<S> {
    conn: ClientConnection,
    stream: S,
}

impl<S> ClientAsyncIo<S>
where
    S: AsyncReadExt + AsyncWriteExt + Unpin,
{
    /// Create a new client, sending the initial ClientHello on the stream.
    pub async fn new(mut conn: ClientConnection, mut stream: S) -> Result<Self, Error> {
        while let Some(data) = conn.write_tls() {
            stream.write_all(&data).await.map_err(Error::Io)?;
        }
        stream.flush().await.map_err(Error::Io)?;
        Ok(Self {
            conn,
            stream,
        })
    }

    /// Perform the handshake. Resolves when complete.
    pub async fn handshake(&mut self) -> Result<(), Error> {
        loop {
            while let Some(data) = self.conn.write_tls() {
                self.stream.write_all(&data).await.map_err(Error::Io)?;
                self.stream.flush().await.map_err(Error::Io)?;
            }
            if self.conn.handshake_done() {
                return Ok(());
            }
            let mut buf = [0u8; 16384];
            let n = self.stream.read(&mut buf).await.map_err(Error::Io)?;
            if n == 0 {
                return Err(Error::ConnectionClosed);
            }
            self.conn.inject(&buf[..n]);
            self.conn.process().await?;
        }
    }

    /// Read decrypted application data.
    pub async fn read(&mut self) -> Result<Bytes, Error> {
        loop {
            while let Some(data) = self.conn.write_tls() {
                self.stream.write_all(&data).await.map_err(Error::Io)?;
                self.stream.flush().await.map_err(Error::Io)?;
            }
            if let Some(data) = self.conn.read_app_data() {
                return Ok(data);
            }
            let mut buf = [0u8; 16384];
            let n = self.stream.read(&mut buf).await.map_err(Error::Io)?;
            if n == 0 {
                return Err(Error::ConnectionClosed);
            }
            self.conn.inject(&buf[..n]);
            self.conn.process().await?;
        }
    }

    /// Write application data.
    pub async fn write(&mut self, data: &[u8]) -> Result<(), Error> {
        let encrypted = self.conn.send(data)?;
        self.stream.write_all(&encrypted).await.map_err(Error::Io)?;
        self.stream.flush().await.map_err(Error::Io)?;
        Ok(())
    }

    /// Clean close.
    pub async fn close(&mut self) -> Result<(), Error> {
        let close_msg = self.conn.close()?;
        self.stream.write_all(&close_msg).await.map_err(Error::Io)?;
        self.stream.flush().await.map_err(Error::Io)?;
        Ok(())
    }

    /// Return a reference to the inner [`ClientConnection`].
    pub fn conn(&self) -> &ClientConnection {
        &self.conn
    }
}

/// Wraps a [`ServerConnection`] over an asynchronous `AsyncRead + AsyncWrite` stream.
pub struct ServerAsyncIo<S> {
    conn: ServerConnection,
    stream: S,
}

impl<S> ServerAsyncIo<S>
where
    S: AsyncReadExt + AsyncWriteExt + Unpin,
{
    /// Create a new server, ready to receive a ClientHello.
    pub fn new(conn: ServerConnection, stream: S) -> Self {
        Self {
            conn,
            stream,
        }
    }

    /// Perform the handshake. Resolves when complete.
    pub async fn handshake(&mut self) -> Result<(), Error> {
        loop {
            while let Some(data) = self.conn.write_tls() {
                self.stream.write_all(&data).await.map_err(Error::Io)?;
                self.stream.flush().await.map_err(Error::Io)?;
            }
            if self.conn.handshake_done() {
                return Ok(());
            }
            let mut buf = [0u8; 16384];
            let n = self.stream.read(&mut buf).await.map_err(Error::Io)?;
            if n == 0 {
                return Err(Error::ConnectionClosed);
            }
            self.conn.inject(&buf[..n]);
            self.conn.process().await?;
        }
    }

    /// Read decrypted application data.
    pub async fn read(&mut self) -> Result<Bytes, Error> {
        loop {
            while let Some(data) = self.conn.write_tls() {
                self.stream.write_all(&data).await.map_err(Error::Io)?;
                self.stream.flush().await.map_err(Error::Io)?;
            }
            if let Some(data) = self.conn.read_app_data() {
                return Ok(data);
            }
            let mut buf = [0u8; 16384];
            let n = self.stream.read(&mut buf).await.map_err(Error::Io)?;
            if n == 0 {
                return Err(Error::ConnectionClosed);
            }
            self.conn.inject(&buf[..n]);
            self.conn.process().await?;
        }
    }

    /// Write application data.
    pub async fn write(&mut self, data: &[u8]) -> Result<(), Error> {
        let encrypted = self.conn.send(data)?;
        self.stream.write_all(&encrypted).await.map_err(Error::Io)?;
        self.stream.flush().await.map_err(Error::Io)?;
        Ok(())
    }

    /// Clean close.
    pub async fn close(&mut self) -> Result<(), Error> {
        let close_msg = self.conn.close()?;
        self.stream.write_all(&close_msg).await.map_err(Error::Io)?;
        self.stream.flush().await.map_err(Error::Io)?;
        Ok(())
    }

    /// Return a reference to the inner [`ServerConnection`].
    pub fn conn(&self) -> &ServerConnection {
        &self.conn
    }
}
