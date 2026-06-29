use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ErrorCode {
    NoError = 0x00,
    ProtocolError = 0x01,
    InternalError = 0x02,
    FlowControlError = 0x03,
    SettingsTimeout = 0x04,
    StreamClosed = 0x05,
    FrameSizeError = 0x06,
    RefusedStream = 0x07,
    Cancel = 0x08,
    CompressionError = 0x09,
    ConnectError = 0x0A,
    EnhanceYourCalm = 0x0B,
    InadequateSecurity = 0x0C,
    Http11Required = 0x0D,
}

impl ErrorCode {
    pub fn from_u32(code: u32) -> Self {
        match code {
            0x00 => ErrorCode::NoError,
            0x01 => ErrorCode::ProtocolError,
            0x02 => ErrorCode::InternalError,
            0x03 => ErrorCode::FlowControlError,
            0x04 => ErrorCode::SettingsTimeout,
            0x05 => ErrorCode::StreamClosed,
            0x06 => ErrorCode::FrameSizeError,
            0x07 => ErrorCode::RefusedStream,
            0x08 => ErrorCode::Cancel,
            0x09 => ErrorCode::CompressionError,
            0x0A => ErrorCode::ConnectError,
            0x0B => ErrorCode::EnhanceYourCalm,
            0x0C => ErrorCode::InadequateSecurity,
            0x0D => ErrorCode::Http11Required,
            _ => ErrorCode::InternalError,
        }
    }

    pub fn to_u32(self) -> u32 {
        self as u32
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            ErrorCode::NoError => "NO_ERROR",
            ErrorCode::ProtocolError => "PROTOCOL_ERROR",
            ErrorCode::InternalError => "INTERNAL_ERROR",
            ErrorCode::FlowControlError => "FLOW_CONTROL_ERROR",
            ErrorCode::SettingsTimeout => "SETTINGS_TIMEOUT",
            ErrorCode::StreamClosed => "STREAM_CLOSED",
            ErrorCode::FrameSizeError => "FRAME_SIZE_ERROR",
            ErrorCode::RefusedStream => "REFUSED_STREAM",
            ErrorCode::Cancel => "CANCEL",
            ErrorCode::CompressionError => "COMPRESSION_ERROR",
            ErrorCode::ConnectError => "CONNECT_ERROR",
            ErrorCode::EnhanceYourCalm => "ENHANCE_YOUR_CALM",
            ErrorCode::InadequateSecurity => "INADEQUATE_SECURITY",
            ErrorCode::Http11Required => "HTTP_1_1_REQUIRED",
        };
        f.write_str(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_code_roundtrip() {
        for &code in &[0x00, 0x01, 0x02, 0x0A, 0x0D] {
            assert_eq!(ErrorCode::from_u32(code).to_u32(), code);
        }
    }

    #[test]
    fn test_unknown_error_code() {
        assert_eq!(ErrorCode::from_u32(0xFF), ErrorCode::InternalError);
    }
}
