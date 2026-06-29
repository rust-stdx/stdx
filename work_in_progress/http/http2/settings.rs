#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum SettingId {
    HeaderTableSize = 0x01,
    EnablePush = 0x02,
    MaxConcurrentStreams = 0x03,
    InitialWindowSize = 0x04,
    MaxFrameSize = 0x05,
    MaxHeaderListSize = 0x06,
    Unknown(u16),
}

impl SettingId {
    pub fn from_u16(id: u16) -> Self {
        match id {
            0x01 => SettingId::HeaderTableSize,
            0x02 => SettingId::EnablePush,
            0x03 => SettingId::MaxConcurrentStreams,
            0x04 => SettingId::InitialWindowSize,
            0x05 => SettingId::MaxFrameSize,
            0x06 => SettingId::MaxHeaderListSize,
            other => SettingId::Unknown(other),
        }
    }

    pub fn to_u16(self) -> u16 {
        match self {
            SettingId::HeaderTableSize => 0x01,
            SettingId::EnablePush => 0x02,
            SettingId::MaxConcurrentStreams => 0x03,
            SettingId::InitialWindowSize => 0x04,
            SettingId::MaxFrameSize => 0x05,
            SettingId::MaxHeaderListSize => 0x06,
            SettingId::Unknown(v) => v,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub header_table_size: Option<u32>,
    pub enable_push: Option<bool>,
    pub max_concurrent_streams: Option<u32>,
    pub initial_window_size: Option<u32>,
    pub max_frame_size: Option<u32>,
    pub max_header_list_size: Option<u32>,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            header_table_size: None,
            enable_push: None,
            max_concurrent_streams: None,
            initial_window_size: None,
            max_frame_size: None,
            max_header_list_size: None,
        }
    }

    pub fn from_pairs(pairs: &[(u16, u32)]) -> Self {
        let mut s = Settings::new();
        for &(id, value) in pairs {
            match SettingId::from_u16(id) {
                SettingId::HeaderTableSize => s.header_table_size = Some(value),
                SettingId::EnablePush => s.enable_push = Some(value != 0),
                SettingId::MaxConcurrentStreams => s.max_concurrent_streams = Some(value),
                SettingId::InitialWindowSize => s.initial_window_size = Some(value),
                SettingId::MaxFrameSize => s.max_frame_size = Some(value),
                SettingId::MaxHeaderListSize => s.max_header_list_size = Some(value),
                _ => {}
            }
        }
        s
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setting_id() {
        assert_eq!(SettingId::from_u16(0x01), SettingId::HeaderTableSize);
        assert_eq!(SettingId::from_u16(0xFF), SettingId::Unknown(0xFF));
    }

    #[test]
    fn test_settings_from_pairs() {
        let s = Settings::from_pairs(&[(0x01, 8192), (0x04, 131072)]);
        assert_eq!(s.header_table_size, Some(8192));
        assert_eq!(s.initial_window_size, Some(131072));
        assert!(s.enable_push.is_none());
    }

    #[test]
    fn test_enable_push_bool() {
        let s = Settings::from_pairs(&[(0x02, 1)]);
        assert_eq!(s.enable_push, Some(true));
        let s = Settings::from_pairs(&[(0x02, 0)]);
        assert_eq!(s.enable_push, Some(false));
    }
}
