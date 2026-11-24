#[cfg(feature = "sources")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSourceParamsMandateNotificationMethod {
    DeprecatedNone,
    Email,
    Manual,
    None,
    StripeEmail,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "sources")]
impl CreateSourceParamsMandateNotificationMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSourceParamsMandateNotificationMethod::DeprecatedNone => "deprecated_none",
            CreateSourceParamsMandateNotificationMethod::Email => "email",
            CreateSourceParamsMandateNotificationMethod::Manual => "manual",
            CreateSourceParamsMandateNotificationMethod::None => "none",
            CreateSourceParamsMandateNotificationMethod::StripeEmail => "stripe_email",
            CreateSourceParamsMandateNotificationMethod::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "sources")]
impl<'json> json::Decode<'json> for CreateSourceParamsMandateNotificationMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSourceParamsMandateNotificationMethod::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "deprecated_none" => CreateSourceParamsMandateNotificationMethod::DeprecatedNone,
            "email" => CreateSourceParamsMandateNotificationMethod::Email,
            "manual" => CreateSourceParamsMandateNotificationMethod::Manual,
            "none" => CreateSourceParamsMandateNotificationMethod::None,
            "stripe_email" => CreateSourceParamsMandateNotificationMethod::StripeEmail,
            other => CreateSourceParamsMandateNotificationMethod::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "sources")]
impl crate::params::FormEncode for CreateSourceParamsMandateNotificationMethod {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
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
    feature = "sources",
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
pub enum SourceAllowRedisplay {
    Always,
    Limited,
    Unspecified,
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
    feature = "sources",
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
impl SourceAllowRedisplay {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SourceAllowRedisplay::Always => "always",
            SourceAllowRedisplay::Limited => "limited",
            SourceAllowRedisplay::Unspecified => "unspecified",
            SourceAllowRedisplay::Unknown(value) => value,
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
    feature = "sources",
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
impl<'json> json::Decode<'json> for SourceAllowRedisplay {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SourceAllowRedisplay::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "always" => SourceAllowRedisplay::Always,
            "limited" => SourceAllowRedisplay::Limited,
            "unspecified" => SourceAllowRedisplay::Unspecified,
            other => SourceAllowRedisplay::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SourceMandateNotificationSepaDebitData {
    #[doc = "SEPA creditor ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub creditor_identifier: Option<String>,
    #[doc = "Last 4 digits of the account number associated with the debit."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[doc = "Mandate reference associated with the debit."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate_reference: Option<String>,
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
    feature = "sources",
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
pub struct SourceRedirectFlow {
    #[doc = "The failure reason for the redirect, either `user_abort` (the customer aborted or dropped out of the redirect flow), ..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[doc = "The URL you provide to redirect the customer to after they authenticated their payment."]
    #[json(default)]
    pub return_url: String,
    #[doc = "The status of the redirect, either `pending` (ready to be used by your customer to authenticate the transaction), `su..."]
    #[json(default)]
    pub status: String,
    #[doc = "The URL provided to you to redirect a customer to as part of a `redirect` authentication flow."]
    #[json(default)]
    pub url: String,
}

#[cfg(any(feature = "events", feature = "sources"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SourceTransactionChfCreditTransferData {
    #[doc = "Reference associated with the transfer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[doc = "Sender's country address."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sender_address_country: Option<String>,
    #[doc = "Sender's line 1 address."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sender_address_line1: Option<String>,
    #[doc = "Sender's bank account IBAN."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sender_iban: Option<String>,
    #[doc = "Sender's name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
}

#[cfg(any(feature = "events", feature = "sources"))]
#[derive(Debug, Clone, PartialEq)]
pub enum SourceTransactionType {
    AchCreditTransfer,
    AchDebit,
    Alipay,
    Bancontact,
    Card,
    CardPresent,
    Eps,
    Giropay,
    Ideal,
    Klarna,
    Multibanco,
    P24,
    SepaDebit,
    Sofort,
    ThreeDSecure,
    Wechat,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "sources"))]
impl SourceTransactionType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SourceTransactionType::AchCreditTransfer => "ach_credit_transfer",
            SourceTransactionType::AchDebit => "ach_debit",
            SourceTransactionType::Alipay => "alipay",
            SourceTransactionType::Bancontact => "bancontact",
            SourceTransactionType::Card => "card",
            SourceTransactionType::CardPresent => "card_present",
            SourceTransactionType::Eps => "eps",
            SourceTransactionType::Giropay => "giropay",
            SourceTransactionType::Ideal => "ideal",
            SourceTransactionType::Klarna => "klarna",
            SourceTransactionType::Multibanco => "multibanco",
            SourceTransactionType::P24 => "p24",
            SourceTransactionType::SepaDebit => "sepa_debit",
            SourceTransactionType::Sofort => "sofort",
            SourceTransactionType::ThreeDSecure => "three_d_secure",
            SourceTransactionType::Wechat => "wechat",
            SourceTransactionType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "sources"))]
impl Default for SourceTransactionType {
    fn default() -> Self {
        SourceTransactionType::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "sources"))]
impl<'json> json::Decode<'json> for SourceTransactionType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SourceTransactionType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ach_credit_transfer" => SourceTransactionType::AchCreditTransfer,
            "ach_debit" => SourceTransactionType::AchDebit,
            "alipay" => SourceTransactionType::Alipay,
            "bancontact" => SourceTransactionType::Bancontact,
            "card" => SourceTransactionType::Card,
            "card_present" => SourceTransactionType::CardPresent,
            "eps" => SourceTransactionType::Eps,
            "giropay" => SourceTransactionType::Giropay,
            "ideal" => SourceTransactionType::Ideal,
            "klarna" => SourceTransactionType::Klarna,
            "multibanco" => SourceTransactionType::Multibanco,
            "p24" => SourceTransactionType::P24,
            "sepa_debit" => SourceTransactionType::SepaDebit,
            "sofort" => SourceTransactionType::Sofort,
            "three_d_secure" => SourceTransactionType::ThreeDSecure,
            "wechat" => SourceTransactionType::Wechat,
            other => SourceTransactionType::Unknown(other.to_string()),
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
    feature = "sources",
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
pub struct SourceTypeAlipay {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub data_string: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub native_url: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
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
    feature = "sources",
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
pub struct SourceTypeCard {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address_line1_check: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address_zip_check: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cvc_check: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub dynamic_last4: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<i64>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<i64>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub funding: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub iin: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tokenization_method: Option<String>,
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
    feature = "sources",
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
pub struct SourceTypeEps {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
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
    feature = "sources",
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
pub struct SourceTypeSofort {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub iban_last4: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}
