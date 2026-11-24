#[cfg(feature = "radar")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InsightsResourcesPaymentEvaluationClientDeviceMetadata {
    #[doc = "ID for the Radar Session associated with the payment evaluation."]
    #[json(default)]
    pub radar_session: String,
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, Default)]
pub struct ListRadarEarlyFraudWarningsParamsCreated {
    #[doc = "Minimum value to filter by (exclusive)"]
    pub gt: Option<i64>,
    #[doc = "Minimum value to filter by (inclusive)"]
    pub gte: Option<i64>,
    #[doc = "Maximum value to filter by (exclusive)"]
    pub lt: Option<i64>,
    #[doc = "Maximum value to filter by (inclusive)"]
    pub lte: Option<i64>,
}

#[cfg(feature = "radar")]
impl crate::params::FormEncode for ListRadarEarlyFraudWarningsParamsCreated {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("gt", self.gt.as_ref());
        form.field_opt("gte", self.gte.as_ref());
        form.field_opt("lt", self.lt.as_ref());
        form.field_opt("lte", self.lte.as_ref());
    }
}

#[cfg(any(feature = "events", feature = "radar"))]
#[derive(Debug, Clone, PartialEq)]
pub enum RadarEarlyFraudWarningObject {
    RadarEarlyFraudWarning,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "radar"))]
impl RadarEarlyFraudWarningObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            RadarEarlyFraudWarningObject::RadarEarlyFraudWarning => "radar.early_fraud_warning",
            RadarEarlyFraudWarningObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "radar"))]
impl Default for RadarEarlyFraudWarningObject {
    fn default() -> Self {
        RadarEarlyFraudWarningObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "radar"))]
impl<'json> json::Decode<'json> for RadarEarlyFraudWarningObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(RadarEarlyFraudWarningObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "radar.early_fraud_warning" => RadarEarlyFraudWarningObject::RadarEarlyFraudWarning,
            other => RadarEarlyFraudWarningObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, PartialEq)]
pub enum RadarPaymentEvaluationObject {
    RadarPaymentEvaluation,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "radar")]
impl RadarPaymentEvaluationObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            RadarPaymentEvaluationObject::RadarPaymentEvaluation => "radar.payment_evaluation",
            RadarPaymentEvaluationObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "radar")]
impl Default for RadarPaymentEvaluationObject {
    fn default() -> Self {
        RadarPaymentEvaluationObject::Unknown(String::new())
    }
}

#[cfg(feature = "radar")]
impl<'json> json::Decode<'json> for RadarPaymentEvaluationObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(RadarPaymentEvaluationObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "radar.payment_evaluation" => RadarPaymentEvaluationObject::RadarPaymentEvaluation,
            other => RadarPaymentEvaluationObject::Unknown(other.to_string()),
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
pub struct RadarReviewResourceLocation {
    #[doc = "The city where the payment originated."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "Two-letter ISO code representing the country where the payment originated."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "The geographic latitude where the payment originated."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[doc = "The geographic longitude where the payment originated."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[doc = "The state/county/province/region where the payment originated."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}
