#[cfg(feature = "sources")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSourceParamsMandateAcceptanceStatus {
    Accepted,
    Pending,
    Refused,
    Revoked,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "sources")]
impl CreateSourceParamsMandateAcceptanceStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSourceParamsMandateAcceptanceStatus::Accepted => "accepted",
            CreateSourceParamsMandateAcceptanceStatus::Pending => "pending",
            CreateSourceParamsMandateAcceptanceStatus::Refused => "refused",
            CreateSourceParamsMandateAcceptanceStatus::Revoked => "revoked",
            CreateSourceParamsMandateAcceptanceStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "sources")]
impl<'json> json::Decode<'json> for CreateSourceParamsMandateAcceptanceStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSourceParamsMandateAcceptanceStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "accepted" => CreateSourceParamsMandateAcceptanceStatus::Accepted,
            "pending" => CreateSourceParamsMandateAcceptanceStatus::Pending,
            "refused" => CreateSourceParamsMandateAcceptanceStatus::Refused,
            "revoked" => CreateSourceParamsMandateAcceptanceStatus::Revoked,
            other => CreateSourceParamsMandateAcceptanceStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "sources")]
impl crate::params::FormEncode for CreateSourceParamsMandateAcceptanceStatus {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "sources")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSourceParamsUsage {
    Reusable,
    SingleUse,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "sources")]
impl CreateSourceParamsUsage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSourceParamsUsage::Reusable => "reusable",
            CreateSourceParamsUsage::SingleUse => "single_use",
            CreateSourceParamsUsage::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "sources")]
impl<'json> json::Decode<'json> for CreateSourceParamsUsage {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSourceParamsUsage::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "reusable" => CreateSourceParamsUsage::Reusable,
            "single_use" => CreateSourceParamsUsage::SingleUse,
            other => CreateSourceParamsUsage::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "sources")]
impl crate::params::FormEncode for CreateSourceParamsUsage {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "sources")]
#[derive(Debug, Clone, Default)]
pub struct ListSourceSourceTransactionsParams {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "sources")]
impl crate::params::FormEncode for ListSourceSourceTransactionsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
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
pub enum SourceObject {
    Source,
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
impl SourceObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SourceObject::Source => "source",
            SourceObject::Unknown(value) => value,
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
impl Default for SourceObject {
    fn default() -> Self {
        SourceObject::Unknown(String::new())
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
impl<'json> json::Decode<'json> for SourceObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SourceObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "source" => SourceObject::Source,
            other => SourceObject::Unknown(other.to_string()),
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
pub struct SourceReceiverFlow {
    #[doc = "The address of the receiver source."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[doc = "The total amount that was moved to your balance."]
    #[json(default)]
    pub amount_charged: i64,
    #[doc = "The total amount received by the receiver source."]
    #[json(default)]
    pub amount_received: i64,
    #[doc = "The total amount that was returned to the customer."]
    #[json(default)]
    pub amount_returned: i64,
    #[doc = "Type of refund attribute method, one of `email`, `manual`, or `none`."]
    #[json(default)]
    pub refund_attributes_method: String,
    #[doc = "Type of refund attribute status, one of `missing`, `requested`, or `available`."]
    #[json(default)]
    pub refund_attributes_status: String,
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
pub struct SourceTypeAcssDebit {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_address_city: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_address_line_1: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_address_line_2: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_address_postal_code: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
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
pub struct SourceTypeAuBecsDebit {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bsb_number: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
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
pub struct SourceTypeCardPresent {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub application_cryptogram: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub application_preferred_name: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub authorization_response_code: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cvm_type: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub dedicated_file_name: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub emv_auth_data: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub evidence_customer_signature: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub evidence_transaction_certificate: Option<String>,
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
    pub pos_device_id: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pos_entry_mode: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub read_method: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reader: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub terminal_verification_results: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transaction_status_information: Option<String>,
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
pub struct SourceTypeGiropay {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
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
pub struct SourceTypeKlarna {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub background_image_url: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub page_title: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pay_later_asset_urls_descriptive: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pay_later_asset_urls_standard: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pay_later_name: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pay_later_redirect_url: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pay_now_asset_urls_descriptive: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pay_now_asset_urls_standard: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pay_now_name: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pay_now_redirect_url: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pay_over_time_asset_urls_descriptive: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pay_over_time_asset_urls_standard: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pay_over_time_name: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pay_over_time_redirect_url: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_categories: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub purchase_country: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub purchase_type: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping_delay: Option<i64>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping_first_name: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping_last_name: Option<String>,
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
pub struct SourceTypeWechat {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub prepay_id: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub qr_code_url: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}
