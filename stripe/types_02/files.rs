#[cfg(feature = "files")]
#[derive(Debug, Clone, Default)]
pub struct CreateFileParamsFileLinkData {
    #[doc = "Set this to `true` to create a file link for the newly created file."]
    pub create: bool,
    #[doc = "The link isn't available after this future timestamp."]
    pub expires_at: Option<i64>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

#[cfg(feature = "files")]
impl crate::params::FormEncode for CreateFileParamsFileLinkData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("create", &self.create);
        form.field_opt("expires_at", self.expires_at.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
    }
}

#[cfg(feature = "files")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListFilesParamsPurpose {
    AccountRequirement,
    AdditionalVerification,
    BusinessIcon,
    BusinessLogo,
    CustomerSignature,
    DisputeEvidence,
    DocumentProviderIdentityDocument,
    FinanceReportRun,
    FinancialAccountStatement,
    IdentityDocument,
    IdentityDocumentDownloadable,
    IssuingRegulatoryReporting,
    PciDocument,
    PlatformTermsOfService,
    Selfie,
    SigmaScheduledQuery,
    TaxDocumentUserUpload,
    TerminalAndroidApk,
    TerminalReaderSplashscreen,
    TerminalWifiCertificate,
    TerminalWifiPrivateKey,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "files")]
impl ListFilesParamsPurpose {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListFilesParamsPurpose::AccountRequirement => "account_requirement",
            ListFilesParamsPurpose::AdditionalVerification => "additional_verification",
            ListFilesParamsPurpose::BusinessIcon => "business_icon",
            ListFilesParamsPurpose::BusinessLogo => "business_logo",
            ListFilesParamsPurpose::CustomerSignature => "customer_signature",
            ListFilesParamsPurpose::DisputeEvidence => "dispute_evidence",
            ListFilesParamsPurpose::DocumentProviderIdentityDocument => "document_provider_identity_document",
            ListFilesParamsPurpose::FinanceReportRun => "finance_report_run",
            ListFilesParamsPurpose::FinancialAccountStatement => "financial_account_statement",
            ListFilesParamsPurpose::IdentityDocument => "identity_document",
            ListFilesParamsPurpose::IdentityDocumentDownloadable => "identity_document_downloadable",
            ListFilesParamsPurpose::IssuingRegulatoryReporting => "issuing_regulatory_reporting",
            ListFilesParamsPurpose::PciDocument => "pci_document",
            ListFilesParamsPurpose::PlatformTermsOfService => "platform_terms_of_service",
            ListFilesParamsPurpose::Selfie => "selfie",
            ListFilesParamsPurpose::SigmaScheduledQuery => "sigma_scheduled_query",
            ListFilesParamsPurpose::TaxDocumentUserUpload => "tax_document_user_upload",
            ListFilesParamsPurpose::TerminalAndroidApk => "terminal_android_apk",
            ListFilesParamsPurpose::TerminalReaderSplashscreen => "terminal_reader_splashscreen",
            ListFilesParamsPurpose::TerminalWifiCertificate => "terminal_wifi_certificate",
            ListFilesParamsPurpose::TerminalWifiPrivateKey => "terminal_wifi_private_key",
            ListFilesParamsPurpose::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "files")]
impl<'json> json::Decode<'json> for ListFilesParamsPurpose {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListFilesParamsPurpose::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account_requirement" => ListFilesParamsPurpose::AccountRequirement,
            "additional_verification" => ListFilesParamsPurpose::AdditionalVerification,
            "business_icon" => ListFilesParamsPurpose::BusinessIcon,
            "business_logo" => ListFilesParamsPurpose::BusinessLogo,
            "customer_signature" => ListFilesParamsPurpose::CustomerSignature,
            "dispute_evidence" => ListFilesParamsPurpose::DisputeEvidence,
            "document_provider_identity_document" => ListFilesParamsPurpose::DocumentProviderIdentityDocument,
            "finance_report_run" => ListFilesParamsPurpose::FinanceReportRun,
            "financial_account_statement" => ListFilesParamsPurpose::FinancialAccountStatement,
            "identity_document" => ListFilesParamsPurpose::IdentityDocument,
            "identity_document_downloadable" => ListFilesParamsPurpose::IdentityDocumentDownloadable,
            "issuing_regulatory_reporting" => ListFilesParamsPurpose::IssuingRegulatoryReporting,
            "pci_document" => ListFilesParamsPurpose::PciDocument,
            "platform_terms_of_service" => ListFilesParamsPurpose::PlatformTermsOfService,
            "selfie" => ListFilesParamsPurpose::Selfie,
            "sigma_scheduled_query" => ListFilesParamsPurpose::SigmaScheduledQuery,
            "tax_document_user_upload" => ListFilesParamsPurpose::TaxDocumentUserUpload,
            "terminal_android_apk" => ListFilesParamsPurpose::TerminalAndroidApk,
            "terminal_reader_splashscreen" => ListFilesParamsPurpose::TerminalReaderSplashscreen,
            "terminal_wifi_certificate" => ListFilesParamsPurpose::TerminalWifiCertificate,
            "terminal_wifi_private_key" => ListFilesParamsPurpose::TerminalWifiPrivateKey,
            other => ListFilesParamsPurpose::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "files")]
impl crate::params::FormEncode for ListFilesParamsPurpose {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
