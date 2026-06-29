pub mod frame;
pub mod qpack;

pub use frame::{Frame, FrameType, Setting, decode_frame, encode_frame};
pub use qpack::{QpackDecoder, QpackEncoder};

pub const CONTROL_STREAM_TYPE: u64 = 0x00;
pub const PUSH_STREAM_TYPE: u64 = 0x01;
pub const QPACK_ENCODER_STREAM_TYPE: u64 = 0x02;
pub const QPACK_DECODER_STREAM_TYPE: u64 = 0x03;

pub const H3_NO_ERROR: u64 = 0x0100;
pub const H3_GENERAL_PROTOCOL_ERROR: u64 = 0x0101;
pub const H3_INTERNAL_ERROR: u64 = 0x0102;
pub const H3_STREAM_CREATION_ERROR: u64 = 0x0103;
pub const H3_CLOSED_CRITICAL_STREAM: u64 = 0x0104;
pub const H3_FRAME_UNEXPECTED: u64 = 0x0105;
pub const H3_FRAME_ERROR: u64 = 0x0106;
pub const H3_EXCESSIVE_LOAD: u64 = 0x0107;
pub const H3_ID_ERROR: u64 = 0x0109;
pub const H3_SETTINGS_ERROR: u64 = 0x010A;
pub const H3_MISSING_SETTINGS: u64 = 0x010B;
pub const H3_REQUEST_REJECTED: u64 = 0x010C;
pub const H3_REQUEST_CANCELLED: u64 = 0x010D;
pub const H3_REQUEST_INCOMPLETE: u64 = 0x010E;
pub const H3_MESSAGE_ERROR: u64 = 0x010F;
pub const H3_CONNECT_ERROR: u64 = 0x0110;
pub const H3_VERSION_FALLBACK: u64 = 0x0111;
