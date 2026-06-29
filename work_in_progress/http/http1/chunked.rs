use std::fmt;

#[derive(Debug)]
pub enum ChunkedError {
    Incomplete,
    BadChunkSize,
    BadChunkExtension,
    BadTrailer,
    InvalidChunkData,
}

impl fmt::Display for ChunkedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChunkedError::Incomplete => write!(f, "incomplete chunked data"),
            ChunkedError::BadChunkSize => write!(f, "invalid chunk size"),
            ChunkedError::BadChunkExtension => write!(f, "invalid chunk extension"),
            ChunkedError::BadTrailer => write!(f, "invalid trailer"),
            ChunkedError::InvalidChunkData => write!(f, "invalid chunk data"),
        }
    }
}

impl std::error::Error for ChunkedError {}

#[derive(Debug, Default)]
pub struct ChunkedDecoder {
    state: ChunkedState,
    current_chunk_remaining: u64,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum ChunkedState {
    #[default]
    ChunkSize,
    ChunkData,
    Trailer,
    Done,
}

impl ChunkedDecoder {
    pub fn new() -> Self {
        ChunkedDecoder::default()
    }

    pub fn decode<'a>(&mut self, data: &'a [u8]) -> Result<ChunkedResult<'a>, ChunkedError> {
        let mut pos = 0;

        loop {
            match self.state {
                ChunkedState::ChunkSize => {
                    if pos >= data.len() {
                        return Err(ChunkedError::Incomplete);
                    }
                    let start = pos;
                    while pos < data.len() && data[pos] != b'\r' && data[pos] != b';' && data[pos] != b'\n' {
                        pos += 1;
                    }
                    if pos == start || pos > data.len() {
                        return Err(ChunkedError::BadChunkSize);
                    }
                    let size_str = std::str::from_utf8(&data[start..pos]).map_err(|_| ChunkedError::BadChunkSize)?;
                    let size = u64::from_str_radix(size_str, 16).map_err(|_| ChunkedError::BadChunkSize)?;

                    // Skip chunk extensions
                    if pos < data.len() && data[pos] == b';' {
                        while pos < data.len() && data[pos] != b'\r' && data[pos] != b'\n' {
                            pos += 1;
                        }
                    }

                    if pos >= data.len() {
                        return Err(ChunkedError::Incomplete);
                    }
                    if data[pos] == b'\r' {
                        pos += 1;
                    }
                    if pos >= data.len() || data[pos] != b'\n' {
                        return Err(ChunkedError::Incomplete);
                    }
                    pos += 1;

                    self.current_chunk_remaining = size;
                    if size == 0 {
                        self.state = ChunkedState::Trailer;
                    } else {
                        self.state = ChunkedState::ChunkData;
                    }
                }

                ChunkedState::ChunkData => {
                    if self.current_chunk_remaining == 0 {
                        if pos >= data.len() {
                            return Err(ChunkedError::Incomplete);
                        }
                        if data[pos] == b'\r' {
                            pos += 1;
                        }
                        if pos >= data.len() || data[pos] != b'\n' {
                            return Err(ChunkedError::InvalidChunkData);
                        }
                        pos += 1;
                        self.state = ChunkedState::ChunkSize;
                        continue;
                    }
                    if pos >= data.len() {
                        return Err(ChunkedError::Incomplete);
                    }
                    let available = (data.len() - pos) as u64;
                    let take = available.min(self.current_chunk_remaining);
                    let result = &data[pos..pos + take as usize];
                    self.current_chunk_remaining -= take;
                    pos += take as usize;
                    return Ok(ChunkedResult::Data {
                        data: result,
                        consumed: pos,
                    });
                }

                ChunkedState::Trailer => {
                    if pos >= data.len() {
                        return Err(ChunkedError::Incomplete);
                    }
                    if data[pos] == b'\r' {
                        pos += 1;
                    }
                    if pos >= data.len() {
                        return Err(ChunkedError::Incomplete);
                    }
                    if data[pos] == b'\n' {
                        pos += 1;
                        self.state = ChunkedState::Done;
                        return Ok(ChunkedResult::Done(pos));
                    }
                    let start = pos;
                    while pos < data.len() && data[pos] != b'\r' && data[pos] != b'\n' {
                        pos += 1;
                    }
                    if pos == start {
                        return Err(ChunkedError::BadTrailer);
                    }
                    let line = &data[start..pos];
                    if pos < data.len() && data[pos] == b'\r' {
                        pos += 1;
                    }
                    if pos >= data.len() || data[pos] != b'\n' {
                        return Err(ChunkedError::Incomplete);
                    }
                    pos += 1;
                    if let Some(colon) = line.iter().position(|&b| b == b':') {
                        let name = String::from_utf8_lossy(&line[..colon]).trim().to_string();
                        let value = String::from_utf8_lossy(&line[colon + 1..]).trim().to_string();
                        return Ok(ChunkedResult::Trailer(name, value, pos));
                    }
                    return Err(ChunkedError::BadTrailer);
                }

                ChunkedState::Done => {
                    return Ok(ChunkedResult::Done(pos));
                }
            }
        }
    }

    pub fn is_done(&self) -> bool {
        self.state == ChunkedState::Done
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ChunkedResult<'a> {
    Data { data: &'a [u8], consumed: usize },
    Trailer(String, String, usize),
    Done(usize),
}

pub fn encode_chunked(data: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    let size_line = format!("{:x}\r\n", data.len());
    buf.extend_from_slice(size_line.as_bytes());
    buf.extend_from_slice(data);
    buf.extend_from_slice(b"\r\n");
    buf.extend_from_slice(b"0\r\n\r\n");
    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_chunk() {
        let encoded = encode_chunked(b"hello");
        let mut dec = ChunkedDecoder::new();
        let result = dec.decode(&encoded).unwrap();
        assert_eq!(
            result,
            ChunkedResult::Data {
                data: b"hello",
                consumed: 8
            }
        );
        let result = dec.decode(&encoded[8..]).unwrap();
        assert_eq!(result, ChunkedResult::Done(7));
        assert!(dec.is_done());
    }

    #[test]
    fn test_decode_full_message() {
        let input = b"5\r\nhello\r\n0\r\n\r\n";
        let mut dec = ChunkedDecoder::new();
        let mut collected = Vec::new();
        let mut pos = 0;
        loop {
            match dec.decode(&input[pos..]).unwrap() {
                ChunkedResult::Data {
                    data,
                    consumed,
                } => {
                    collected.extend_from_slice(data);
                    pos += consumed;
                }
                ChunkedResult::Trailer(_, _, _) => {}
                ChunkedResult::Done(consumed) => {
                    pos += consumed;
                    break;
                }
            }
        }
        assert_eq!(collected, b"hello");
        assert!(dec.is_done());
    }

    #[test]
    fn test_multiple_chunks() {
        let data = b"4\r\npart\r\n5\r\n-two!\r\n0\r\n\r\n";
        let mut dec = ChunkedDecoder::new();
        let mut collected = Vec::new();
        let mut pos = 0;
        loop {
            match dec.decode(&data[pos..]).unwrap() {
                ChunkedResult::Data {
                    data: d,
                    consumed,
                } => {
                    collected.extend_from_slice(d);
                    pos += consumed;
                }
                ChunkedResult::Done(consumed) => {
                    pos += consumed;
                    break;
                }
                _ => {}
            }
        }
        assert_eq!(collected, b"part-two!");
    }

    #[test]
    fn test_empty_chunked() {
        let mut dec = ChunkedDecoder::new();
        let data = b"0\r\n\r\n";
        loop {
            match dec.decode(data).unwrap() {
                ChunkedResult::Done(_) => break,
                _ => {}
            }
        }
        assert!(dec.is_done());
    }

    #[test]
    fn test_chunk_with_extensions() {
        let data = b"5;foo=bar\r\nhello\r\n0\r\n\r\n";
        let mut dec = ChunkedDecoder::new();
        let mut collected = Vec::new();
        let mut pos = 0;
        loop {
            match dec.decode(&data[pos..]).unwrap() {
                ChunkedResult::Data {
                    data: d,
                    consumed,
                } => {
                    collected.extend_from_slice(d);
                    pos += consumed;
                }
                ChunkedResult::Done(consumed) => {
                    pos += consumed;
                    break;
                }
                _ => {}
            }
        }
        assert_eq!(collected, b"hello");
    }

    #[test]
    fn test_trailers() {
        let data = b"0\r\nX-Custom: value\r\n\r\n";
        let mut dec = ChunkedDecoder::new();
        let mut trailers = Vec::new();
        let mut pos = 0;
        loop {
            match dec.decode(&data[pos..]).unwrap() {
                ChunkedResult::Done(consumed) => {
                    pos += consumed;
                    break;
                }
                ChunkedResult::Trailer(n, v, consumed) => {
                    pos += consumed;
                    trailers.push((n, v));
                }
                _ => {}
            }
        }
        assert_eq!(trailers, vec![("X-Custom".to_string(), "value".to_string())]);
    }

    #[test]
    fn test_incomplete_chunk_size() {
        let mut dec = ChunkedDecoder::new();
        assert!(matches!(dec.decode(b""), Err(ChunkedError::Incomplete)));
    }

    #[test]
    fn test_bad_hex() {
        let mut dec = ChunkedDecoder::new();
        assert!(matches!(dec.decode(b"ZZ\r\n"), Err(ChunkedError::BadChunkSize)));
    }
}
