#[cfg(any(feature = "apps", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum AppsInstallChannel {
    PrivateLive,
    PrivateTest,
    Public,
    Review,
    Testing,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "apps", feature = "events"))]
impl AppsInstallChannel {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            AppsInstallChannel::PrivateLive => "private_live",
            AppsInstallChannel::PrivateTest => "private_test",
            AppsInstallChannel::Public => "public",
            AppsInstallChannel::Review => "review",
            AppsInstallChannel::Testing => "testing",
            AppsInstallChannel::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "apps", feature = "events"))]
impl Default for AppsInstallChannel {
    fn default() -> Self {
        AppsInstallChannel::Unknown(String::new())
    }
}

#[cfg(any(feature = "apps", feature = "events"))]
impl<'json> json::Decode<'json> for AppsInstallChannel {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(AppsInstallChannel::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "private_live" => AppsInstallChannel::PrivateLive,
            "private_test" => AppsInstallChannel::PrivateTest,
            "public" => AppsInstallChannel::Public,
            "review" => AppsInstallChannel::Review,
            "testing" => AppsInstallChannel::Testing,
            other => AppsInstallChannel::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "apps", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum AppsInstallObject {
    AppsInstall,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "apps", feature = "events"))]
impl AppsInstallObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            AppsInstallObject::AppsInstall => "apps.install",
            AppsInstallObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "apps", feature = "events"))]
impl Default for AppsInstallObject {
    fn default() -> Self {
        AppsInstallObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "apps", feature = "events"))]
impl<'json> json::Decode<'json> for AppsInstallObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(AppsInstallObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "apps.install" => AppsInstallObject::AppsInstall,
            other => AppsInstallObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "apps")]
#[derive(Debug, Clone, PartialEq)]
pub enum AppsSecretObject {
    AppsSecret,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "apps")]
impl AppsSecretObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            AppsSecretObject::AppsSecret => "apps.secret",
            AppsSecretObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "apps")]
impl Default for AppsSecretObject {
    fn default() -> Self {
        AppsSecretObject::Unknown(String::new())
    }
}

#[cfg(feature = "apps")]
impl<'json> json::Decode<'json> for AppsSecretObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(AppsSecretObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "apps.secret" => AppsSecretObject::AppsSecret,
            other => AppsSecretObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "apps")]
#[derive(Debug, Clone, Default)]
pub struct GetAppInstallParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "apps")]
impl crate::params::FormEncode for GetAppInstallParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}

#[cfg(feature = "apps")]
#[derive(Debug, Clone, PartialEq)]
pub enum SecretServiceResourceScopeType {
    Account,
    User,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "apps")]
impl SecretServiceResourceScopeType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SecretServiceResourceScopeType::Account => "account",
            SecretServiceResourceScopeType::User => "user",
            SecretServiceResourceScopeType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "apps")]
impl Default for SecretServiceResourceScopeType {
    fn default() -> Self {
        SecretServiceResourceScopeType::Unknown(String::new())
    }
}

#[cfg(feature = "apps")]
impl<'json> json::Decode<'json> for SecretServiceResourceScopeType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SecretServiceResourceScopeType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account" => SecretServiceResourceScopeType::Account,
            "user" => SecretServiceResourceScopeType::User,
            other => SecretServiceResourceScopeType::Unknown(other.to_string()),
        })
    }
}
