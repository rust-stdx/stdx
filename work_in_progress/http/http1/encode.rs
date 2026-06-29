use crate::common::{Body, Frame, HeaderValue, Request, Response};

fn write_crlf(buf: &mut Vec<u8>) {
    buf.extend_from_slice(b"\r\n");
}

fn write_header(buf: &mut Vec<u8>, name: &str, value: &HeaderValue) {
    buf.extend_from_slice(name.as_bytes());
    buf.extend_from_slice(b": ");
    buf.extend_from_slice(value.as_bytes());
    write_crlf(buf);
}

/// Copy as many bytes from `src[pos..]` into `buf[wrote..]` as will fit.
///
/// Returns `true` when the entire source has been exhausted (`pos == src.len()`),
/// meaning the caller should advance to the next encoding step.
fn write_partial(buf: &mut [u8], wrote: &mut usize, pos: &mut usize, src: &[u8]) -> bool {
    let space = buf.len() - *wrote;
    if space == 0 {
        return false;
    }
    let n = src.len().min(space);
    buf[*wrote..*wrote + n].copy_from_slice(&src[*pos..*pos + n]);
    *pos += n;
    *wrote += n;
    *pos == src.len()
}

/// Encode a complete request in one shot.
pub async fn encode_request<B: Body>(req: Request<B>) -> Result<Vec<u8>, B::Error> {
    let estimated = req.body.as_ref().map_or(4096, |b| b.size_hint().unwrap_or(4096)) + 512;
    let mut encoder = RequestEncoder::new(req);
    let mut out = Vec::with_capacity(estimated);
    let mut buf = [0u8; 4096];
    while let Some(n) = encoder.encode(&mut buf).await? {
        out.extend_from_slice(&buf[..n]);
    }
    Ok(out)
}

/// Encode a complete response in one shot.
pub async fn encode_response<B: Body>(resp: Response<B>) -> Result<Vec<u8>, B::Error> {
    let estimated = resp.body.size_hint().unwrap_or(4096) + 512;
    let mut encoder = ResponseEncoder::new(resp);
    let mut out = Vec::with_capacity(estimated);
    let mut buf = [0u8; 4096];
    while let Some(n) = encoder.encode(&mut buf).await? {
        out.extend_from_slice(&buf[..n]);
    }
    Ok(out)
}

/// Create a streaming request encoder.
pub fn encoder<B: Body>(req: Request<B>) -> RequestEncoder<B> {
    RequestEncoder::new(req)
}

/// Create a streaming response encoder.
pub fn encoder_response<B: Body>(resp: Response<B>) -> ResponseEncoder<B> {
    ResponseEncoder::new(resp)
}

#[derive(Debug, PartialEq, Eq)]
enum EncoderState {
    Head,
    Body,
    Done,
}

#[derive(Debug, PartialEq, Eq)]
enum Src {
    Idle,
    Head,
    Scratch,
    BodyData,
    ChunkCrlf,
    FinalChunk,
}

/// Shared encoder inner type used by both [`RequestEncoder`] and [`ResponseEncoder`].
#[derive(Debug)]
struct EncoderInner<B: Body> {
    body: Option<B>,
    state: EncoderState,
    is_chunked: bool,
    size_hint: Option<usize>,
    head: Vec<u8>,
    src: Src,
    src_pos: usize,
    body_data: Option<bytes::Bytes>,
    scratch: [u8; 20],
    scratch_len: usize,
}

impl<B: Body> EncoderInner<B> {
    fn format_chunk_size(&mut self) {
        let size = self.body_data.as_ref().map_or(0, |d| d.len());
        if size == 0 {
            self.scratch[..3].copy_from_slice(b"0\r\n");
            self.scratch_len = 3;
            return;
        }
        let bytes = size.to_be_bytes();
        let start = bytes.iter().position(|&b| b != 0).unwrap();
        let data = &bytes[start..];
        let n = data.len() * 2;
        hex::encode_into(&mut self.scratch[..n], data, hex::Alphabet::Lower).unwrap();
        self.scratch[n] = b'\r';
        self.scratch[n + 1] = b'\n';
        self.scratch_len = n + 2;
    }

    fn advance_after_head(&mut self) {
        match self.state {
            EncoderState::Head => match self.size_hint {
                Some(0) => self.state = EncoderState::Done,
                _ => self.state = EncoderState::Body,
            },
            EncoderState::Body => self.state = EncoderState::Done,
            _ => {}
        }
        self.src = Src::Idle;
    }

    async fn poll_body(&mut self) -> Result<(), B::Error> {
        match &mut self.body {
            None => self.state = EncoderState::Done,
            Some(body) => match body.next_frame().await {
                None => {
                    if self.is_chunked {
                        self.src = Src::FinalChunk;
                        self.src_pos = 0;
                    } else {
                        self.state = EncoderState::Done;
                    }
                }
                Some(Ok(Frame::Data(data))) => {
                    if !data.is_empty() {
                        if self.is_chunked {
                            self.body_data = Some(data);
                            self.format_chunk_size();
                            self.src = Src::Scratch;
                            self.src_pos = 0;
                        } else {
                            self.body_data = Some(data);
                            self.src = Src::BodyData;
                            self.src_pos = 0;
                        }
                    }
                }
                Some(Ok(Frame::Trailers(t))) => {
                    self.head.clear();
                    self.head.extend_from_slice(b"0\r\n");
                    for (name, value) in &t {
                        self.head.extend_from_slice(name.as_str().as_bytes());
                        self.head.extend_from_slice(b": ");
                        self.head.extend_from_slice(value.as_str().as_bytes());
                        write_crlf(&mut self.head);
                    }
                    write_crlf(&mut self.head);
                    self.src = Src::Head;
                    self.src_pos = 0;
                }
                Some(Err(e)) => return Err(e),
            },
        }
        Ok(())
    }

    async fn encode(&mut self, buf: &mut [u8]) -> Result<Option<usize>, B::Error> {
        let mut wrote = 0;

        loop {
            if wrote == buf.len() {
                return Ok(Some(wrote));
            }

            match self.src {
                Src::Idle => match self.state {
                    EncoderState::Done => {
                        return if wrote > 0 { Ok(Some(wrote)) } else { Ok(None) };
                    }
                    EncoderState::Head => {
                        self.src = Src::Head;
                    }
                    EncoderState::Body => self.poll_body().await?,
                },

                Src::Head => {
                    if write_partial(buf, &mut wrote, &mut self.src_pos, &self.head) {
                        self.advance_after_head();
                    }
                }

                Src::Scratch => {
                    if write_partial(buf, &mut wrote, &mut self.src_pos, &self.scratch[..self.scratch_len]) {
                        self.src = Src::BodyData;
                        self.src_pos = 0;
                    }
                }

                Src::BodyData => {
                    let data = self.body_data.as_ref().unwrap();
                    if write_partial(buf, &mut wrote, &mut self.src_pos, data) {
                        if self.is_chunked {
                            self.src = Src::ChunkCrlf;
                            self.src_pos = 0;
                        } else {
                            self.body_data = None;
                            self.src = Src::Idle;
                        }
                    }
                }

                Src::ChunkCrlf => {
                    if write_partial(buf, &mut wrote, &mut self.src_pos, b"\r\n") {
                        self.body_data = None;
                        self.src = Src::Idle;
                    }
                }

                Src::FinalChunk => {
                    if write_partial(buf, &mut wrote, &mut self.src_pos, b"0\r\n\r\n") {
                        self.state = EncoderState::Done;
                        self.src = Src::Idle;
                    }
                }
            }
        }
    }
}

/// Streaming encoder for HTTP/1.1 requests.
///
/// Produces encoded bytes on-demand by polling the body's [`Body::next_frame`].
/// The caller provides a fixed-size buffer – the encoder never allocates
/// output buffers beyond the internally stored request head.
///
/// ```ignore
/// let mut encoder = http1::encoder(req);
/// let mut buf = [0u8; 4096];
/// while let Some(n) = encoder.encode(&mut buf).await.unwrap() {
///     stream.write_all(&buf[..n]).await.unwrap();
/// }
/// ```
#[derive(Debug)]
pub struct RequestEncoder<B: Body>(EncoderInner<B>);

impl<B: Body> RequestEncoder<B> {
    /// Build a new request encoder from a [`Request`].
    pub fn new(req: Request<B>) -> Self {
        let size_hint = req.body.as_ref().map_or(Some(0), |b| b.size_hint());
        let is_chunked = size_hint.is_none();

        let mut head = Vec::new();

        // request line
        head.extend_from_slice(req.method.as_str().as_bytes());
        head.push(b' ');
        head.extend_from_slice(req.uri.as_str().as_bytes());
        head.push(b' ');
        head.extend_from_slice(b"HTTP/1.1");
        write_crlf(&mut head);

        // headers
        let mut has_content_length = false;
        let mut has_transfer_encoding = false;
        for (name, value) in &req.headers {
            write_header(&mut head, name.as_str(), value);
            if name.as_str() == "content-length" {
                has_content_length = true;
            }
            if name.as_str() == "transfer-encoding" {
                has_transfer_encoding = true;
            }
        }

        if let Some(len) = size_hint {
            if len > 0 && !has_content_length {
                let cl = len.to_string();
                write_header(&mut head, "Content-Length", &cl.as_str().into());
            }
        } else if !has_transfer_encoding {
            write_header(&mut head, "Transfer-Encoding", &"chunked".into());
        }

        write_crlf(&mut head);

        RequestEncoder(EncoderInner {
            body: req.body,
            state: EncoderState::Head,
            is_chunked,
            size_hint,
            head,
            src: Src::Idle,
            src_pos: 0,
            body_data: None,
            scratch: [0u8; 20],
            scratch_len: 0,
        })
    }

    /// Write the next chunk of encoded data into `buf`.
    ///
    /// Returns `Some(n)` when `n` bytes were written and more data follows.
    /// Returns `None` when the entire message has been encoded.
    pub async fn encode(&mut self, buf: &mut [u8]) -> Result<Option<usize>, B::Error> {
        self.0.encode(buf).await
    }
}

/// Streaming encoder for HTTP/1.1 responses.
///
/// Identical in operation to [`RequestEncoder`] but built from a
/// [`Response`] instead of a [`Request`].
#[derive(Debug)]
pub struct ResponseEncoder<B: Body>(EncoderInner<B>);

impl<B: Body> ResponseEncoder<B> {
    /// Build a new response encoder from a [`Response`].
    pub fn new(resp: Response<B>) -> Self {
        let size_hint = resp.body.size_hint();
        let is_chunked = size_hint.is_none();

        let mut head = Vec::new();

        // status line
        head.extend_from_slice(b"HTTP/1.1");
        head.push(b' ');
        head.extend_from_slice(resp.status.as_u16().to_string().as_bytes());
        head.push(b' ');
        head.extend_from_slice(resp.status.canonical_reason().unwrap_or("").as_bytes());
        write_crlf(&mut head);

        // headers
        let mut has_content_length = false;
        let mut has_transfer_encoding = false;
        for (name, value) in &resp.headers {
            write_header(&mut head, name.as_str(), value);
            if name.as_str() == "content-length" {
                has_content_length = true;
            }
            if name.as_str() == "transfer-encoding" {
                has_transfer_encoding = true;
            }
        }

        if let Some(len) = size_hint {
            if len > 0 && !has_content_length {
                let cl = len.to_string();
                write_header(&mut head, "Content-Length", &cl.as_str().into());
            }
        } else if !has_transfer_encoding {
            write_header(&mut head, "Transfer-Encoding", &"chunked".into());
        }

        write_crlf(&mut head);

        ResponseEncoder(EncoderInner {
            body: Some(resp.body),
            state: EncoderState::Head,
            is_chunked,
            size_hint,
            head,
            src: Src::Idle,
            src_pos: 0,
            body_data: None,
            scratch: [0u8; 20],
            scratch_len: 0,
        })
    }

    /// Write the next chunk of encoded data into `buf`.
    ///
    /// See [`RequestEncoder::encode`] for a usage example.
    pub async fn encode(&mut self, buf: &mut [u8]) -> Result<Option<usize>, B::Error> {
        self.0.encode(buf).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        common::*,
        http1::{H1Error, decode_request, decode_response},
    };

    fn block_on<F: std::future::Future>(f: F) -> F::Output {
        use std::{
            sync::Arc,
            task::{Context, Poll, Wake},
        };

        struct Noop;
        impl Wake for Noop {
            fn wake(self: Arc<Self>) {}
        }

        let waker = Arc::new(Noop).into();
        let mut cx = Context::from_waker(&waker);
        let mut pin = std::pin::pin!(f);
        loop {
            match pin.as_mut().poll(&mut cx) {
                Poll::Ready(v) => return v,
                Poll::Pending => continue,
            }
        }
    }

    fn req(method: Method, uri: &str) -> Request {
        Request {
            method,
            uri: Uri::parse(uri).unwrap(),
            version: Version::Http11,
            headers: vec![
                ("host".into(), "example.com".into()),
                ("user-agent".into(), "test/1.0".into()),
            ],
            body: None,
        }
    }

    #[test]
    fn test_encode_decode_get() {
        let r = req(Method::Get, "/");
        let encoded = block_on(encode_request(r)).unwrap();
        assert!(encoded.starts_with(b"GET / HTTP/1.1\r\n"));

        let (decoded, _) = decode_request(&encoded).unwrap();
        assert_eq!(decoded.method, Method::Get);
        assert_eq!(decoded.uri.as_str(), "/");
        assert!(decoded.body.is_none());
    }

    #[test]
    fn test_get_response() {
        let resp = Response {
            version: Version::Http11,
            status: StatusCode::Ok,
            headers: vec![
                ("content-type".into(), "text/plain".into()),
                ("content-length".into(), "5".into()),
            ],
            body: bytes::Bytes::from_static(b"hello"),
        };

        let encoded = block_on(encode_response(resp)).unwrap();
        assert!(encoded.starts_with(b"HTTP/1.1 200 OK\r\n"));

        let (decoded, _) = decode_response(&encoded).unwrap();
        assert_eq!(decoded.status, StatusCode::Ok);
        assert_eq!(&decoded.body[..], b"hello");
    }

    #[test]
    fn test_post_request() {
        let req = Request {
            method: Method::Post,
            uri: Uri::parse("/submit").unwrap(),
            version: Version::Http11,
            headers: vec![
                ("host".into(), "example.com".into()),
                ("content-type".into(), "application/json".into()),
            ],
            body: Some(bytes::Bytes::from_static(br#"{"key":"value"}"#)),
        };

        let encoded = block_on(encode_request(req)).unwrap();
        let (decoded, _) = decode_request(&encoded).unwrap();
        assert_eq!(decoded.method, Method::Post);
        assert_eq!(&decoded.body.unwrap()[..], br#"{"key":"value"}"#);
    }

    #[test]
    fn test_response_no_body() {
        let resp = Response {
            version: Version::Http11,
            status: StatusCode::NoContent,
            headers: vec![],
            body: bytes::Bytes::new(),
        };

        let encoded = block_on(encode_response(resp)).unwrap();
        let (decoded, _) = decode_response(&encoded).unwrap();
        assert_eq!(decoded.status, StatusCode::NoContent);
        assert!(decoded.body.is_empty());
    }

    #[test]
    fn test_not_found_response() {
        let resp = Response {
            version: Version::Http11,
            status: StatusCode::NotFound,
            headers: vec![("content-length".into(), "0".into())],
            body: bytes::Bytes::new(),
        };
        let encoded = block_on(encode_response(resp)).unwrap();
        let (decoded, _) = decode_response(&encoded).unwrap();
        assert_eq!(decoded.status, StatusCode::NotFound);
    }

    #[test]
    fn test_decode_incomplete() {
        let r = req(Method::Get, "/");
        let encoded = block_on(encode_request(r)).unwrap();
        let truncated = &encoded[..encoded.len() - 5];
        assert!(matches!(decode_request(truncated), Err(H1Error::Incomplete)));
    }

    #[test]
    fn test_decode_bad_request_line() {
        let bad = b"NOT_HTTP\r\nHost: example.com\r\n\r\n";
        assert!(matches!(decode_request(bad), Err(H1Error::BadRequestLine(_))));
    }

    #[test]
    fn test_decode_bad_status_line() {
        let bad = b"JUNK\r\nContent-Length: 0\r\n\r\n";
        assert!(matches!(decode_response(bad), Err(H1Error::BadStatusLine(_))));
    }
}
