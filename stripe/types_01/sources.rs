#[cfg(feature = "sources")]
#[derive(Debug, Clone, Default)]
pub struct CreateSourceParamsMandateAcceptanceOnline {
    #[doc = "The Unix timestamp (in seconds) when the mandate was accepted or refused by the customer."]
    pub date: Option<i64>,
    #[doc = "The IP address from which the mandate was accepted or refused by the customer."]
    pub ip: Option<String>,
    #[doc = "The user agent of the browser from which the mandate was accepted or refused by the customer."]
    pub user_agent: Option<String>,
}

#[cfg(feature = "sources")]
impl crate::params::FormEncode for CreateSourceParamsMandateAcceptanceOnline {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("date", self.date.as_ref());
        form.field_opt("ip", self.ip.as_ref());
        form.field_opt("user_agent", self.user_agent.as_ref());
    }
}

#[cfg(feature = "sources")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSourceParamsMandateInterval {
    OneTime,
    Scheduled,
    Variable,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "sources")]
impl CreateSourceParamsMandateInterval {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSourceParamsMandateInterval::OneTime => "one_time",
            CreateSourceParamsMandateInterval::Scheduled => "scheduled",
            CreateSourceParamsMandateInterval::Variable => "variable",
            CreateSourceParamsMandateInterval::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "sources")]
impl<'json> json::Decode<'json> for CreateSourceParamsMandateInterval {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSourceParamsMandateInterval::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "one_time" => CreateSourceParamsMandateInterval::OneTime,
            "scheduled" => CreateSourceParamsMandateInterval::Scheduled,
            "variable" => CreateSourceParamsMandateInterval::Variable,
            other => CreateSourceParamsMandateInterval::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "sources")]
impl crate::params::FormEncode for CreateSourceParamsMandateInterval {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "sources")]
#[derive(Debug, Clone, Default)]
pub struct CreateSourceParamsOwnerAddress {
    #[doc = "City, district, suburb, town, or village."]
    pub city: Option<String>,
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub country: Option<String>,
    #[doc = "Address line 1, such as the street, PO Box, or company name."]
    pub line1: Option<String>,
    #[doc = "Address line 2, such as the apartment, suite, unit, or building."]
    pub line2: Option<String>,
    #[doc = "ZIP or postal code."]
    pub postal_code: Option<String>,
    #[doc = "State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2))."]
    pub state: Option<String>,
}

#[cfg(feature = "sources")]
impl crate::params::FormEncode for CreateSourceParamsOwnerAddress {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("city", self.city.as_ref());
        form.field_opt("country", self.country.as_ref());
        form.field_opt("line1", self.line1.as_ref());
        form.field_opt("line2", self.line2.as_ref());
        form.field_opt("postal_code", self.postal_code.as_ref());
        form.field_opt("state", self.state.as_ref());
    }
}

#[cfg(feature = "sources")]
#[derive(Debug, Clone, Default)]
pub struct CreateSourceParamsSourceOrderShippingAddress {
    #[doc = "City, district, suburb, town, or village."]
    pub city: Option<String>,
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub country: Option<String>,
    #[doc = "Address line 1, such as the street, PO Box, or company name."]
    pub line1: String,
    #[doc = "Address line 2, such as the apartment, suite, unit, or building."]
    pub line2: Option<String>,
    #[doc = "ZIP or postal code."]
    pub postal_code: Option<String>,
    #[doc = "State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2))."]
    pub state: Option<String>,
}

#[cfg(feature = "sources")]
impl crate::params::FormEncode for CreateSourceParamsSourceOrderShippingAddress {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("city", self.city.as_ref());
        form.field_opt("country", self.country.as_ref());
        form.field("line1", &self.line1);
        form.field_opt("line2", self.line2.as_ref());
        form.field_opt("postal_code", self.postal_code.as_ref());
        form.field_opt("state", self.state.as_ref());
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SourceMandateNotificationAcssDebitData {
    #[doc = "The statement descriptor associate with the debit."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}

#[cfg(any(feature = "events", feature = "sources"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SourceTransactionAchCreditTransferData {
    #[doc = "Customer data associated with the transfer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_data: Option<String>,
    #[doc = "Bank account fingerprint associated with the transfer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[doc = "Last 4 digits of the account number associated with the transfer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[doc = "Routing number associated with the transfer."]
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
pub struct SourceTypeBancontact {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub iban_last4: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<String>,
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
pub struct SourceTypeP24 {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
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
pub struct SourceTypeSepaCreditTransfer {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_address_city: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_address_country: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_address_line1: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_address_line2: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_address_postal_code: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_address_state: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_name: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refund_iban: Option<String>,
}

#[cfg(feature = "sources")]
#[derive(Debug, Clone, Default)]
pub struct VerifySourceParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The values needed to verify the source."]
    pub values: Vec<String>,
}

#[cfg(feature = "sources")]
impl crate::params::FormEncode for VerifySourceParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_array("values", self.values.as_slice());
    }
}
