#[cfg(any(feature = "apps", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum AppsInstallStatus {
    InstallFailed,
    Installed,
    Installing,
    UninstallFailed,
    Uninstalling,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "apps", feature = "events"))]
impl AppsInstallStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            AppsInstallStatus::InstallFailed => "install_failed",
            AppsInstallStatus::Installed => "installed",
            AppsInstallStatus::Installing => "installing",
            AppsInstallStatus::UninstallFailed => "uninstall_failed",
            AppsInstallStatus::Uninstalling => "uninstalling",
            AppsInstallStatus::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "apps", feature = "events"))]
impl Default for AppsInstallStatus {
    fn default() -> Self {
        AppsInstallStatus::Unknown(String::new())
    }
}

#[cfg(any(feature = "apps", feature = "events"))]
impl<'json> json::Decode<'json> for AppsInstallStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(AppsInstallStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "install_failed" => AppsInstallStatus::InstallFailed,
            "installed" => AppsInstallStatus::Installed,
            "installing" => AppsInstallStatus::Installing,
            "uninstall_failed" => AppsInstallStatus::UninstallFailed,
            "uninstalling" => AppsInstallStatus::Uninstalling,
            other => AppsInstallStatus::Unknown(other.to_string()),
        })
    }
}
