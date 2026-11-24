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
pub struct DisputeEnhancedEvidenceMastercardCompliance {
    #[doc = "A field acknowledging the fee incurred when countering a Mastercard compliance dispute."]
    #[json(default)]
    pub fee_acknowledged: bool,
}

#[cfg(feature = "disputes")]
#[derive(Debug, Clone, Default)]
pub struct GetDisputeParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "disputes")]
impl crate::params::FormEncode for GetDisputeParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}

#[cfg(feature = "disputes")]
#[derive(Debug, Clone, PartialEq)]
pub enum UpdateDisputeParamsEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices {
    Merchandise,
    Services,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "disputes")]
impl UpdateDisputeParamsEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            UpdateDisputeParamsEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices::Merchandise => "merchandise",
            UpdateDisputeParamsEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices::Services => "services",
            UpdateDisputeParamsEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "disputes")]
impl<'json> json::Decode<'json>
    for UpdateDisputeParamsEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(UpdateDisputeParamsEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "merchandise" => UpdateDisputeParamsEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices::Merchandise,
            "services" => UpdateDisputeParamsEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices::Services,
            other => UpdateDisputeParamsEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "disputes")]
impl crate::params::FormEncode
    for UpdateDisputeParamsEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransactionMerchandiseOrServices
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "disputes")]
#[derive(Debug, Clone, Default)]
pub struct UpdateDisputeParamsEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransactionShippingAddress {
    #[doc = "City, district, suburb, town, or village."]
    pub city: Option<json::OwnedValue>,
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub country: Option<json::OwnedValue>,
    #[doc = "Address line 1, such as the street, PO Box, or company name."]
    pub line1: Option<json::OwnedValue>,
    #[doc = "Address line 2, such as the apartment, suite, unit, or building."]
    pub line2: Option<json::OwnedValue>,
    #[doc = "ZIP or postal code."]
    pub postal_code: Option<json::OwnedValue>,
    #[doc = "State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2))."]
    pub state: Option<json::OwnedValue>,
}

#[cfg(feature = "disputes")]
impl crate::params::FormEncode
    for UpdateDisputeParamsEvidenceEnhancedEvidenceVisaCompellingEvidence3DisputedTransactionShippingAddress
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        let _ = form;
    }
}
