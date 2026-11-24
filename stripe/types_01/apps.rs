#[cfg(feature = "apps")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListAppInstallsParamsStatus {
    InstallFailed,
    Installed,
    Installing,
    UninstallFailed,
    Uninstalling,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "apps")]
impl ListAppInstallsParamsStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListAppInstallsParamsStatus::InstallFailed => "install_failed",
            ListAppInstallsParamsStatus::Installed => "installed",
            ListAppInstallsParamsStatus::Installing => "installing",
            ListAppInstallsParamsStatus::UninstallFailed => "uninstall_failed",
            ListAppInstallsParamsStatus::Uninstalling => "uninstalling",
            ListAppInstallsParamsStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "apps")]
impl<'json> json::Decode<'json> for ListAppInstallsParamsStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListAppInstallsParamsStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "install_failed" => ListAppInstallsParamsStatus::InstallFailed,
            "installed" => ListAppInstallsParamsStatus::Installed,
            "installing" => ListAppInstallsParamsStatus::Installing,
            "uninstall_failed" => ListAppInstallsParamsStatus::UninstallFailed,
            "uninstalling" => ListAppInstallsParamsStatus::Uninstalling,
            other => ListAppInstallsParamsStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "apps")]
impl crate::params::FormEncode for ListAppInstallsParamsStatus {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "apps")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListAppSecretsParamsScopeType {
    Account,
    User,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "apps")]
impl ListAppSecretsParamsScopeType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListAppSecretsParamsScopeType::Account => "account",
            ListAppSecretsParamsScopeType::User => "user",
            ListAppSecretsParamsScopeType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "apps")]
impl<'json> json::Decode<'json> for ListAppSecretsParamsScopeType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListAppSecretsParamsScopeType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account" => ListAppSecretsParamsScopeType::Account,
            "user" => ListAppSecretsParamsScopeType::User,
            other => ListAppSecretsParamsScopeType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "apps")]
impl crate::params::FormEncode for ListAppSecretsParamsScopeType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
