use super::{DEFAULT_INITIAL_WINDOW_SIZE, settings::Settings};

#[derive(Debug, Clone)]
pub struct ConnectionState {
    pub settings_sent: bool,
    pub settings_received: bool,
    pub local_settings: Settings,
    pub remote_settings: Settings,
    pub local_window: u32,
    pub remote_window: u32,
}

impl ConnectionState {
    pub fn new() -> Self {
        ConnectionState {
            settings_sent: false,
            settings_received: false,
            local_settings: Settings::new(),
            remote_settings: Settings::new(),
            local_window: DEFAULT_INITIAL_WINDOW_SIZE,
            remote_window: DEFAULT_INITIAL_WINDOW_SIZE,
        }
    }
}

impl Default for ConnectionState {
    fn default() -> Self {
        ConnectionState::new()
    }
}

pub fn validate_preface(data: &[u8]) -> bool {
    data.len() >= 24 && &data[..24] == super::PREFACE
}

pub fn negotiate_settings(local: &Settings, remote: &Settings) -> Settings {
    Settings {
        header_table_size: remote.header_table_size.or(local.header_table_size),
        enable_push: remote.enable_push.or(local.enable_push),
        max_concurrent_streams: remote.max_concurrent_streams.or(local.max_concurrent_streams),
        initial_window_size: remote.initial_window_size.or(local.initial_window_size),
        max_frame_size: remote.max_frame_size.or(local.max_frame_size),
        max_header_list_size: remote.max_header_list_size.or(local.max_header_list_size),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preface_validation() {
        assert!(validate_preface(b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n"));
        assert!(!validate_preface(b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r")); // truncated
        assert!(!validate_preface(b"GET / HTTP/1.1\r\n")); // wrong protocol
    }

    #[test]
    fn test_connection_state_defaults() {
        let state = ConnectionState::new();
        assert!(!state.settings_sent);
        assert!(!state.settings_received);
        assert_eq!(state.local_window, 65535);
        assert_eq!(state.remote_window, 65535);
    }
}
