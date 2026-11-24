#[cfg(feature = "files")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateFileParamsPurpose {
    AccountRequirement,
    AdditionalVerification,
    BusinessIcon,
    BusinessLogo,
    CustomerSignature,
    DisputeEvidence,
    IdentityDocument,
    IssuingRegulatoryReporting,
    PciDocument,
    PlatformTermsOfService,
    TaxDocumentUserUpload,
    TerminalAndroidApk,
    TerminalReaderSplashscreen,
    TerminalWifiCertificate,
    TerminalWifiPrivateKey,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "files")]
impl CreateFileParamsPurpose {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateFileParamsPurpose::AccountRequirement => "account_requirement",
            CreateFileParamsPurpose::AdditionalVerification => "additional_verification",
            CreateFileParamsPurpose::BusinessIcon => "business_icon",
            CreateFileParamsPurpose::BusinessLogo => "business_logo",
            CreateFileParamsPurpose::CustomerSignature => "customer_signature",
            CreateFileParamsPurpose::DisputeEvidence => "dispute_evidence",
            CreateFileParamsPurpose::IdentityDocument => "identity_document",
            CreateFileParamsPurpose::IssuingRegulatoryReporting => "issuing_regulatory_reporting",
            CreateFileParamsPurpose::PciDocument => "pci_document",
            CreateFileParamsPurpose::PlatformTermsOfService => "platform_terms_of_service",
            CreateFileParamsPurpose::TaxDocumentUserUpload => "tax_document_user_upload",
            CreateFileParamsPurpose::TerminalAndroidApk => "terminal_android_apk",
            CreateFileParamsPurpose::TerminalReaderSplashscreen => "terminal_reader_splashscreen",
            CreateFileParamsPurpose::TerminalWifiCertificate => "terminal_wifi_certificate",
            CreateFileParamsPurpose::TerminalWifiPrivateKey => "terminal_wifi_private_key",
            CreateFileParamsPurpose::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "files")]
impl<'json> json::Decode<'json> for CreateFileParamsPurpose {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateFileParamsPurpose::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account_requirement" => CreateFileParamsPurpose::AccountRequirement,
            "additional_verification" => CreateFileParamsPurpose::AdditionalVerification,
            "business_icon" => CreateFileParamsPurpose::BusinessIcon,
            "business_logo" => CreateFileParamsPurpose::BusinessLogo,
            "customer_signature" => CreateFileParamsPurpose::CustomerSignature,
            "dispute_evidence" => CreateFileParamsPurpose::DisputeEvidence,
            "identity_document" => CreateFileParamsPurpose::IdentityDocument,
            "issuing_regulatory_reporting" => CreateFileParamsPurpose::IssuingRegulatoryReporting,
            "pci_document" => CreateFileParamsPurpose::PciDocument,
            "platform_terms_of_service" => CreateFileParamsPurpose::PlatformTermsOfService,
            "tax_document_user_upload" => CreateFileParamsPurpose::TaxDocumentUserUpload,
            "terminal_android_apk" => CreateFileParamsPurpose::TerminalAndroidApk,
            "terminal_reader_splashscreen" => CreateFileParamsPurpose::TerminalReaderSplashscreen,
            "terminal_wifi_certificate" => CreateFileParamsPurpose::TerminalWifiCertificate,
            "terminal_wifi_private_key" => CreateFileParamsPurpose::TerminalWifiPrivateKey,
            other => CreateFileParamsPurpose::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "files")]
impl crate::params::FormEncode for CreateFileParamsPurpose {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
