//! ReadBuf bridging matrix for `TokioIo`, with mock Tokio and Hyper IO types.
//!
//! hyper-util 0.1.19 implements these four traits with byte-for-byte the same
//! `ReadBuf`/`ReadBufCursor` sequence; these tests pin the observable behavior
//! (partial reads, EOF, errors, `Pending`, zero-length buffers, initialized vs
//! uninitialized destinations) against the mock types.

use std::{
    io,
    mem::MaybeUninit,
    pin::Pin,
    task::{Context, Poll, Waker},
};

use hyper::rt::{Read as HyperRead, ReadBuf as HyperReadBuf, ReadBufCursor, Write as HyperWrite};
use hyper_utils::rt::TokioIo;

fn cx() -> Context<'static> {
    Context::from_waker(Waker::noop())
}

/// A Tokio reader that returns at most `chunk` bytes per poll.
struct MockTokioReader {
    data: Vec<u8>,
    pos: usize,
    chunk: usize,
    pending_calls: usize,
    fail: bool,
}

impl MockTokioReader {
    fn new(data: &[u8], chunk: usize) -> Self {
        Self {
            data: data.to_vec(),
            pos: 0,
            chunk,
            pending_calls: 0,
            fail: false,
        }
    }
}

impl tokio::io::AsyncRead for MockTokioReader {
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut tokio::io::ReadBuf<'_>) -> Poll<io::Result<()>> {
        let this = self.get_mut();
        if this.fail {
            return Poll::Ready(Err(io::Error::other("mock read error")));
        }
        if this.pending_calls > 0 {
            this.pending_calls -= 1;
            cx.waker().wake_by_ref();
            return Poll::Pending;
        }
        let remaining = &this.data[this.pos..];
        let n = remaining.len().min(this.chunk).min(buf.remaining());
        if n > 0 {
            buf.put_slice(&remaining[..n]);
            this.pos += n;
        }
        Poll::Ready(Ok(()))
    }
}

/// A Hyper reader that returns at most `chunk` bytes per poll.
struct MockHyperReader {
    data: Vec<u8>,
    pos: usize,
    chunk: usize,
    pending_calls: usize,
    fail: bool,
}

impl MockHyperReader {
    fn new(data: &[u8], chunk: usize) -> Self {
        Self {
            data: data.to_vec(),
            pos: 0,
            chunk,
            pending_calls: 0,
            fail: false,
        }
    }
}

impl HyperRead for MockHyperReader {
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, mut buf: ReadBufCursor<'_>) -> Poll<io::Result<()>> {
        let this = self.get_mut();
        if this.fail {
            return Poll::Ready(Err(io::Error::other("mock read error")));
        }
        if this.pending_calls > 0 {
            this.pending_calls -= 1;
            cx.waker().wake_by_ref();
            return Poll::Pending;
        }
        let remaining = &this.data[this.pos..];
        let n = remaining.len().min(this.chunk).min(buf.remaining());
        if n > 0 {
            buf.put_slice(&remaining[..n]);
            this.pos += n;
        }
        Poll::Ready(Ok(()))
    }
}

/// Reads `cap` bytes through `<TokioIo<MockTokioReader> as HyperRead>`.
fn tokio_to_hyper(io: &mut TokioIo<MockTokioReader>, cap: usize) -> (Poll<io::Result<()>>, Vec<u8>) {
    let mut backing: Vec<MaybeUninit<u8>> = vec![MaybeUninit::uninit(); cap];
    let mut read_buf = HyperReadBuf::uninit(&mut backing);
    let poll = {
        let cursor = read_buf.unfilled();
        Pin::new(io).poll_read(&mut cx(), cursor)
    };
    (poll, read_buf.filled().to_vec())
}

/// Reads through `<TokioIo<MockHyperReader> as tokio::io::AsyncRead>` into an
/// uninitialized buffer.
fn hyper_to_tokio_uninit(io: &mut TokioIo<MockHyperReader>, cap: usize) -> (Poll<io::Result<()>>, Vec<u8>) {
    let mut backing: Vec<MaybeUninit<u8>> = vec![MaybeUninit::uninit(); cap];
    let mut read_buf = tokio::io::ReadBuf::uninit(&mut backing);
    let poll = tokio::io::AsyncRead::poll_read(Pin::new(io), &mut cx(), &mut read_buf);
    (poll, read_buf.filled().to_vec())
}

#[test]
fn tokio_to_hyper_returns_partial_reads() {
    let mut io = TokioIo::new(MockTokioReader::new(b"hello world", 4));
    let (poll, first) = tokio_to_hyper(&mut io, 16);
    assert!(matches!(poll, Poll::Ready(Ok(()))));
    assert_eq!(first, b"hell");
    let (poll, second) = tokio_to_hyper(&mut io, 16);
    assert!(matches!(poll, Poll::Ready(Ok(()))));
    assert_eq!(second, b"o wo");
}

#[test]
fn tokio_to_hyper_eof_fills_nothing() {
    let mut io = TokioIo::new(MockTokioReader::new(b"", 8));
    let (poll, filled) = tokio_to_hyper(&mut io, 16);
    assert!(matches!(poll, Poll::Ready(Ok(()))));
    assert!(filled.is_empty(), "EOF must not fabricate bytes");
}

#[test]
fn tokio_to_hyper_propagates_errors() {
    let mut reader = MockTokioReader::new(b"ignored", 8);
    reader.fail = true;
    let mut io = TokioIo::new(reader);
    let (poll, filled) = tokio_to_hyper(&mut io, 16);
    match poll {
        Poll::Ready(Err(err)) => assert_eq!(err.to_string(), "mock read error"),
        other => panic!("expected an error, got {other:?}"),
    }
    assert!(filled.is_empty());
}

#[test]
fn tokio_to_hyper_pending_does_not_advance() {
    let mut reader = MockTokioReader::new(b"data", 8);
    reader.pending_calls = 1;
    let mut io = TokioIo::new(reader);
    let (poll, filled) = tokio_to_hyper(&mut io, 16);
    assert!(matches!(poll, Poll::Pending));
    assert!(filled.is_empty(), "Pending must not mark bytes filled");
    let (poll, filled) = tokio_to_hyper(&mut io, 16);
    assert!(matches!(poll, Poll::Ready(Ok(()))));
    assert_eq!(filled, b"data");
}

#[test]
fn tokio_to_hyper_zero_length_buffer_is_ok() {
    let mut io = TokioIo::new(MockTokioReader::new(b"data", 8));
    let (poll, filled) = tokio_to_hyper(&mut io, 0);
    assert!(matches!(poll, Poll::Ready(Ok(()))));
    assert!(filled.is_empty());
}

#[test]
fn hyper_to_tokio_extends_a_partially_filled_buffer() {
    let mut io = TokioIo::new(MockHyperReader::new(b"cd", 8));
    let mut backing = *b"ab................";
    let mut read_buf = tokio::io::ReadBuf::new(&mut backing);
    read_buf.put_slice(b"ab");
    let poll = tokio::io::AsyncRead::poll_read(Pin::new(&mut io), &mut cx(), &mut read_buf);
    assert!(matches!(poll, Poll::Ready(Ok(()))));
    assert_eq!(read_buf.filled(), b"abcd", "the existing prefix must be preserved");
}

#[test]
fn hyper_to_tokio_reports_only_the_new_bytes_as_initialized() {
    let mut io = TokioIo::new(MockHyperReader::new(b"xyz", 8));
    let (poll, filled) = hyper_to_tokio_uninit(&mut io, 8);
    assert!(matches!(poll, Poll::Ready(Ok(()))));
    assert_eq!(filled, b"xyz");
}

#[test]
fn hyper_to_tokio_eof_pending_and_errors() {
    // EOF.
    let mut io = TokioIo::new(MockHyperReader::new(b"", 8));
    let (poll, filled) = hyper_to_tokio_uninit(&mut io, 8);
    assert!(matches!(poll, Poll::Ready(Ok(()))));
    assert!(filled.is_empty());

    // Pending then data.
    let mut reader = MockHyperReader::new(b"ok", 8);
    reader.pending_calls = 1;
    let mut io = TokioIo::new(reader);
    let (poll, filled) = hyper_to_tokio_uninit(&mut io, 8);
    assert!(matches!(poll, Poll::Pending));
    assert!(filled.is_empty());
    let (poll, filled) = hyper_to_tokio_uninit(&mut io, 8);
    assert!(matches!(poll, Poll::Ready(Ok(()))));
    assert_eq!(filled, b"ok");

    // Error.
    let mut reader = MockHyperReader::new(b"ignored", 8);
    reader.fail = true;
    let mut io = TokioIo::new(reader);
    let (poll, filled) = hyper_to_tokio_uninit(&mut io, 8);
    assert!(matches!(poll, Poll::Ready(Err(_))));
    assert!(filled.is_empty());
}

#[test]
fn hyper_to_tokio_zero_length_buffer_is_ok() {
    let mut io = TokioIo::new(MockHyperReader::new(b"data", 8));
    let (poll, filled) = hyper_to_tokio_uninit(&mut io, 0);
    assert!(matches!(poll, Poll::Ready(Ok(()))));
    assert!(filled.is_empty());
}

/// Writes must be forwarded verbatim to the inner type, in both directions.
struct MockTokioWriter {
    written: Vec<u8>,
    vectored: bool,
}

impl tokio::io::AsyncWrite for MockTokioWriter {
    fn poll_write(self: Pin<&mut Self>, _cx: &mut Context<'_>, buf: &[u8]) -> Poll<io::Result<usize>> {
        let this = self.get_mut();
        this.written.extend_from_slice(buf);
        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn is_write_vectored(&self) -> bool {
        true
    }

    fn poll_write_vectored(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        bufs: &[io::IoSlice<'_>],
    ) -> Poll<io::Result<usize>> {
        let this = self.get_mut();
        this.vectored = true;
        let n = bufs.iter().map(|b| b.len()).sum();
        for b in bufs {
            this.written.extend_from_slice(b);
        }
        Poll::Ready(Ok(n))
    }
}

struct MockHyperWriter {
    written: Vec<u8>,
    vectored: bool,
}

impl HyperWrite for MockHyperWriter {
    fn poll_write(self: Pin<&mut Self>, _cx: &mut Context<'_>, buf: &[u8]) -> Poll<io::Result<usize>> {
        let this = self.get_mut();
        this.written.extend_from_slice(buf);
        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn is_write_vectored(&self) -> bool {
        true
    }

    fn poll_write_vectored(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        bufs: &[io::IoSlice<'_>],
    ) -> Poll<io::Result<usize>> {
        let this = self.get_mut();
        this.vectored = true;
        let n = bufs.iter().map(|b| b.len()).sum();
        for b in bufs {
            this.written.extend_from_slice(b);
        }
        Poll::Ready(Ok(n))
    }
}

#[test]
fn writes_are_forwarded_in_both_directions() {
    // Tokio writer behind `HyperWrite`.
    let mut io = TokioIo::new(MockTokioWriter {
        written: Vec::new(),
        vectored: false,
    });
    let (poll, n) = {
        let poll = Pin::new(&mut io).poll_write(&mut cx(), b"abc");
        (poll, 3)
    };
    assert!(matches!(poll, Poll::Ready(Ok(3))));
    assert_eq!(n, 3);
    assert_eq!(io.inner().written, b"abc");
    assert!(HyperWrite::is_write_vectored(&io));

    let bufs = [io::IoSlice::new(b"de"), io::IoSlice::new(b"f")];
    let poll = Pin::new(&mut io).poll_write_vectored(&mut cx(), &bufs);
    assert!(matches!(poll, Poll::Ready(Ok(3))));
    assert_eq!(io.inner().written, b"abcdef");
    assert!(io.inner().vectored);

    // Hyper writer behind `tokio::io::AsyncWrite`.
    let mut io = TokioIo::new(MockHyperWriter {
        written: Vec::new(),
        vectored: false,
    });
    let poll = tokio::io::AsyncWrite::poll_write(Pin::new(&mut io), &mut cx(), b"xyz");
    assert!(matches!(poll, Poll::Ready(Ok(3))));
    assert_eq!(io.inner().written, b"xyz");
    assert!(tokio::io::AsyncWrite::is_write_vectored(&io));

    let bufs = [io::IoSlice::new(b"12"), io::IoSlice::new(b"34")];
    let poll = tokio::io::AsyncWrite::poll_write_vectored(Pin::new(&mut io), &mut cx(), &bufs);
    assert!(matches!(poll, Poll::Ready(Ok(4))));
    assert_eq!(io.inner().written, b"xyz1234");
    assert!(io.inner().vectored);
}
