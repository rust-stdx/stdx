#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateThreeDSecureAuthenticationParamsDirectoryServer {
    AmericanExpress,
    CartesBancaires,
    Discover,
    Mastercard,
    Visa,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "three_d_secure")]
impl CreateThreeDSecureAuthenticationParamsDirectoryServer {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateThreeDSecureAuthenticationParamsDirectoryServer::AmericanExpress => "american_express",
            CreateThreeDSecureAuthenticationParamsDirectoryServer::CartesBancaires => "cartes_bancaires",
            CreateThreeDSecureAuthenticationParamsDirectoryServer::Discover => "discover",
            CreateThreeDSecureAuthenticationParamsDirectoryServer::Mastercard => "mastercard",
            CreateThreeDSecureAuthenticationParamsDirectoryServer::Visa => "visa",
            CreateThreeDSecureAuthenticationParamsDirectoryServer::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "three_d_secure")]
impl<'json> json::Decode<'json> for CreateThreeDSecureAuthenticationParamsDirectoryServer {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateThreeDSecureAuthenticationParamsDirectoryServer::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "american_express" => CreateThreeDSecureAuthenticationParamsDirectoryServer::AmericanExpress,
            "cartes_bancaires" => CreateThreeDSecureAuthenticationParamsDirectoryServer::CartesBancaires,
            "discover" => CreateThreeDSecureAuthenticationParamsDirectoryServer::Discover,
            "mastercard" => CreateThreeDSecureAuthenticationParamsDirectoryServer::Mastercard,
            "visa" => CreateThreeDSecureAuthenticationParamsDirectoryServer::Visa,
            other => CreateThreeDSecureAuthenticationParamsDirectoryServer::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for CreateThreeDSecureAuthenticationParamsDirectoryServer {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateThreeDSecureAuthenticationParamsFlowPreferenceChallengeType {
    Mandated,
    Preferred,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "three_d_secure")]
impl CreateThreeDSecureAuthenticationParamsFlowPreferenceChallengeType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateThreeDSecureAuthenticationParamsFlowPreferenceChallengeType::Mandated => "mandated",
            CreateThreeDSecureAuthenticationParamsFlowPreferenceChallengeType::Preferred => "preferred",
            CreateThreeDSecureAuthenticationParamsFlowPreferenceChallengeType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "three_d_secure")]
impl<'json> json::Decode<'json> for CreateThreeDSecureAuthenticationParamsFlowPreferenceChallengeType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateThreeDSecureAuthenticationParamsFlowPreferenceChallengeType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "mandated" => CreateThreeDSecureAuthenticationParamsFlowPreferenceChallengeType::Mandated,
            "preferred" => CreateThreeDSecureAuthenticationParamsFlowPreferenceChallengeType::Preferred,
            other => CreateThreeDSecureAuthenticationParamsFlowPreferenceChallengeType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for CreateThreeDSecureAuthenticationParamsFlowPreferenceChallengeType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateThreeDSecureAuthenticationParamsFutureUsageInstallmentExpiryType {
    Date,
    Never,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "three_d_secure")]
impl CreateThreeDSecureAuthenticationParamsFutureUsageInstallmentExpiryType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateThreeDSecureAuthenticationParamsFutureUsageInstallmentExpiryType::Date => "date",
            CreateThreeDSecureAuthenticationParamsFutureUsageInstallmentExpiryType::Never => "never",
            CreateThreeDSecureAuthenticationParamsFutureUsageInstallmentExpiryType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "three_d_secure")]
impl<'json> json::Decode<'json> for CreateThreeDSecureAuthenticationParamsFutureUsageInstallmentExpiryType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateThreeDSecureAuthenticationParamsFutureUsageInstallmentExpiryType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "date" => CreateThreeDSecureAuthenticationParamsFutureUsageInstallmentExpiryType::Date,
            "never" => CreateThreeDSecureAuthenticationParamsFutureUsageInstallmentExpiryType::Never,
            other => CreateThreeDSecureAuthenticationParamsFutureUsageInstallmentExpiryType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for CreateThreeDSecureAuthenticationParamsFutureUsageInstallmentExpiryType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateThreeDSecureAuthenticationParamsFutureUsageType {
    CardOnFile,
    Installment,
    Recurring,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "three_d_secure")]
impl CreateThreeDSecureAuthenticationParamsFutureUsageType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateThreeDSecureAuthenticationParamsFutureUsageType::CardOnFile => "card_on_file",
            CreateThreeDSecureAuthenticationParamsFutureUsageType::Installment => "installment",
            CreateThreeDSecureAuthenticationParamsFutureUsageType::Recurring => "recurring",
            CreateThreeDSecureAuthenticationParamsFutureUsageType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "three_d_secure")]
impl<'json> json::Decode<'json> for CreateThreeDSecureAuthenticationParamsFutureUsageType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateThreeDSecureAuthenticationParamsFutureUsageType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "card_on_file" => CreateThreeDSecureAuthenticationParamsFutureUsageType::CardOnFile,
            "installment" => CreateThreeDSecureAuthenticationParamsFutureUsageType::Installment,
            "recurring" => CreateThreeDSecureAuthenticationParamsFutureUsageType::Recurring,
            other => CreateThreeDSecureAuthenticationParamsFutureUsageType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for CreateThreeDSecureAuthenticationParamsFutureUsageType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateThreeDSecureAuthenticationParamsReason {
    CardholderAuthentication,
    IssuerRequested,
    LiabilityShift,
    ProcessingCosts,
    RegulatoryCompliance,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "three_d_secure")]
impl CreateThreeDSecureAuthenticationParamsReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateThreeDSecureAuthenticationParamsReason::CardholderAuthentication => "cardholder_authentication",
            CreateThreeDSecureAuthenticationParamsReason::IssuerRequested => "issuer_requested",
            CreateThreeDSecureAuthenticationParamsReason::LiabilityShift => "liability_shift",
            CreateThreeDSecureAuthenticationParamsReason::ProcessingCosts => "processing_costs",
            CreateThreeDSecureAuthenticationParamsReason::RegulatoryCompliance => "regulatory_compliance",
            CreateThreeDSecureAuthenticationParamsReason::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "three_d_secure")]
impl<'json> json::Decode<'json> for CreateThreeDSecureAuthenticationParamsReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateThreeDSecureAuthenticationParamsReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "cardholder_authentication" => CreateThreeDSecureAuthenticationParamsReason::CardholderAuthentication,
            "issuer_requested" => CreateThreeDSecureAuthenticationParamsReason::IssuerRequested,
            "liability_shift" => CreateThreeDSecureAuthenticationParamsReason::LiabilityShift,
            "processing_costs" => CreateThreeDSecureAuthenticationParamsReason::ProcessingCosts,
            "regulatory_compliance" => CreateThreeDSecureAuthenticationParamsReason::RegulatoryCompliance,
            other => CreateThreeDSecureAuthenticationParamsReason::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for CreateThreeDSecureAuthenticationParamsReason {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, PartialEq)]
pub enum ThreeDSecureAuthenticationObject {
    ThreeDSecureAuthentication,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "three_d_secure")]
impl ThreeDSecureAuthenticationObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ThreeDSecureAuthenticationObject::ThreeDSecureAuthentication => "three_d_secure.authentication",
            ThreeDSecureAuthenticationObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "three_d_secure")]
impl Default for ThreeDSecureAuthenticationObject {
    fn default() -> Self {
        ThreeDSecureAuthenticationObject::Unknown(String::new())
    }
}

#[cfg(feature = "three_d_secure")]
impl<'json> json::Decode<'json> for ThreeDSecureAuthenticationObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ThreeDSecureAuthenticationObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "three_d_secure.authentication" => ThreeDSecureAuthenticationObject::ThreeDSecureAuthentication,
            other => ThreeDSecureAuthenticationObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum ThreeDSecureDetailsAuthenticationFlow {
    Challenge,
    Frictionless,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl ThreeDSecureDetailsAuthenticationFlow {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ThreeDSecureDetailsAuthenticationFlow::Challenge => "challenge",
            ThreeDSecureDetailsAuthenticationFlow::Frictionless => "frictionless",
            ThreeDSecureDetailsAuthenticationFlow::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl<'json> json::Decode<'json> for ThreeDSecureDetailsAuthenticationFlow {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ThreeDSecureDetailsAuthenticationFlow::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "challenge" => ThreeDSecureDetailsAuthenticationFlow::Challenge,
            "frictionless" => ThreeDSecureDetailsAuthenticationFlow::Frictionless,
            other => ThreeDSecureDetailsAuthenticationFlow::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum ThreeDSecureDetailsElectronicCommerceIndicator {
    N01,
    N02,
    N05,
    N06,
    N07,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl ThreeDSecureDetailsElectronicCommerceIndicator {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ThreeDSecureDetailsElectronicCommerceIndicator::N01 => "01",
            ThreeDSecureDetailsElectronicCommerceIndicator::N02 => "02",
            ThreeDSecureDetailsElectronicCommerceIndicator::N05 => "05",
            ThreeDSecureDetailsElectronicCommerceIndicator::N06 => "06",
            ThreeDSecureDetailsElectronicCommerceIndicator::N07 => "07",
            ThreeDSecureDetailsElectronicCommerceIndicator::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl<'json> json::Decode<'json> for ThreeDSecureDetailsElectronicCommerceIndicator {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ThreeDSecureDetailsElectronicCommerceIndicator::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "01" => ThreeDSecureDetailsElectronicCommerceIndicator::N01,
            "02" => ThreeDSecureDetailsElectronicCommerceIndicator::N02,
            "05" => ThreeDSecureDetailsElectronicCommerceIndicator::N05,
            "06" => ThreeDSecureDetailsElectronicCommerceIndicator::N06,
            "07" => ThreeDSecureDetailsElectronicCommerceIndicator::N07,
            other => ThreeDSecureDetailsElectronicCommerceIndicator::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum ThreeDSecureDetailsResultReason {
    Abandoned,
    Bypassed,
    Canceled,
    CardNotEnrolled,
    NetworkNotSupported,
    ProtocolError,
    Rejected,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl ThreeDSecureDetailsResultReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ThreeDSecureDetailsResultReason::Abandoned => "abandoned",
            ThreeDSecureDetailsResultReason::Bypassed => "bypassed",
            ThreeDSecureDetailsResultReason::Canceled => "canceled",
            ThreeDSecureDetailsResultReason::CardNotEnrolled => "card_not_enrolled",
            ThreeDSecureDetailsResultReason::NetworkNotSupported => "network_not_supported",
            ThreeDSecureDetailsResultReason::ProtocolError => "protocol_error",
            ThreeDSecureDetailsResultReason::Rejected => "rejected",
            ThreeDSecureDetailsResultReason::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl<'json> json::Decode<'json> for ThreeDSecureDetailsResultReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ThreeDSecureDetailsResultReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "abandoned" => ThreeDSecureDetailsResultReason::Abandoned,
            "bypassed" => ThreeDSecureDetailsResultReason::Bypassed,
            "canceled" => ThreeDSecureDetailsResultReason::Canceled,
            "card_not_enrolled" => ThreeDSecureDetailsResultReason::CardNotEnrolled,
            "network_not_supported" => ThreeDSecureDetailsResultReason::NetworkNotSupported,
            "protocol_error" => ThreeDSecureDetailsResultReason::ProtocolError,
            "rejected" => ThreeDSecureDetailsResultReason::Rejected,
            other => ThreeDSecureDetailsResultReason::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, PartialEq)]
pub enum ThreeDSecureStandaloneResourceDataShareDetailsType {
    DsSpecific,
    EmvStandard,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "three_d_secure")]
impl ThreeDSecureStandaloneResourceDataShareDetailsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ThreeDSecureStandaloneResourceDataShareDetailsType::DsSpecific => "ds_specific",
            ThreeDSecureStandaloneResourceDataShareDetailsType::EmvStandard => "emv_standard",
            ThreeDSecureStandaloneResourceDataShareDetailsType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "three_d_secure")]
impl Default for ThreeDSecureStandaloneResourceDataShareDetailsType {
    fn default() -> Self {
        ThreeDSecureStandaloneResourceDataShareDetailsType::Unknown(String::new())
    }
}

#[cfg(feature = "three_d_secure")]
impl<'json> json::Decode<'json> for ThreeDSecureStandaloneResourceDataShareDetailsType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ThreeDSecureStandaloneResourceDataShareDetailsType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ds_specific" => ThreeDSecureStandaloneResourceDataShareDetailsType::DsSpecific,
            "emv_standard" => ThreeDSecureStandaloneResourceDataShareDetailsType::EmvStandard,
            other => ThreeDSecureStandaloneResourceDataShareDetailsType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, PartialEq)]
pub enum ThreeDSecureStandaloneResourceInstallmentInterval {
    Day,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "three_d_secure")]
impl ThreeDSecureStandaloneResourceInstallmentInterval {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ThreeDSecureStandaloneResourceInstallmentInterval::Day => "day",
            ThreeDSecureStandaloneResourceInstallmentInterval::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "three_d_secure")]
impl Default for ThreeDSecureStandaloneResourceInstallmentInterval {
    fn default() -> Self {
        ThreeDSecureStandaloneResourceInstallmentInterval::Unknown(String::new())
    }
}

#[cfg(feature = "three_d_secure")]
impl<'json> json::Decode<'json> for ThreeDSecureStandaloneResourceInstallmentInterval {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ThreeDSecureStandaloneResourceInstallmentInterval::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "day" => ThreeDSecureStandaloneResourceInstallmentInterval::Day,
            other => ThreeDSecureStandaloneResourceInstallmentInterval::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ThreeDSecureStandaloneResourceShippingAddress {
    #[doc = "City, district, suburb, town, or village."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "Address line 1, such as the street, PO Box, or company name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    #[doc = "Address line 2, such as the apartment, suite, unit, or building."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    #[doc = "ZIP or postal code."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[doc = "State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2))."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, PartialEq)]
pub enum ThreeDSecureStandaloneResourceThreeRiDetailsType {
    DelayedShipment,
    OtherPayment,
    Recurring,
    SplitShipment,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "three_d_secure")]
impl ThreeDSecureStandaloneResourceThreeRiDetailsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ThreeDSecureStandaloneResourceThreeRiDetailsType::DelayedShipment => "delayed_shipment",
            ThreeDSecureStandaloneResourceThreeRiDetailsType::OtherPayment => "other_payment",
            ThreeDSecureStandaloneResourceThreeRiDetailsType::Recurring => "recurring",
            ThreeDSecureStandaloneResourceThreeRiDetailsType::SplitShipment => "split_shipment",
            ThreeDSecureStandaloneResourceThreeRiDetailsType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "three_d_secure")]
impl Default for ThreeDSecureStandaloneResourceThreeRiDetailsType {
    fn default() -> Self {
        ThreeDSecureStandaloneResourceThreeRiDetailsType::Unknown(String::new())
    }
}

#[cfg(feature = "three_d_secure")]
impl<'json> json::Decode<'json> for ThreeDSecureStandaloneResourceThreeRiDetailsType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ThreeDSecureStandaloneResourceThreeRiDetailsType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "delayed_shipment" => ThreeDSecureStandaloneResourceThreeRiDetailsType::DelayedShipment,
            "other_payment" => ThreeDSecureStandaloneResourceThreeRiDetailsType::OtherPayment,
            "recurring" => ThreeDSecureStandaloneResourceThreeRiDetailsType::Recurring,
            "split_shipment" => ThreeDSecureStandaloneResourceThreeRiDetailsType::SplitShipment,
            other => ThreeDSecureStandaloneResourceThreeRiDetailsType::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ThreeDSecureUsage {
    #[doc = "Whether 3D Secure is supported on this card."]
    #[json(default)]
    pub supported: bool,
}
