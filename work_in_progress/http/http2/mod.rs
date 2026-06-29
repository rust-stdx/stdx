pub mod connection;
pub mod error;
pub mod frame;
pub mod hpack;
pub mod settings;
pub mod stream;

pub use error::ErrorCode;
pub use frame::{Frame, FrameType, decode_frame, encode_frame};
pub use hpack::{HpackDecoder, HpackEncoder};
pub use settings::SettingId;
pub use stream::StreamId;

pub const PREFACE: &[u8; 24] = b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n";

pub const DEFAULT_MAX_FRAME_SIZE: u32 = 16384;
pub const DEFAULT_INITIAL_WINDOW_SIZE: u32 = 65535;
pub const DEFAULT_HEADER_TABLE_SIZE: u32 = 4096;
pub const MAX_MAX_FRAME_SIZE: u32 = 16777215;
