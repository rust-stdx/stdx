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
pub struct BillingBillResourceInvoicingLinesCommonCreditedItems {
    #[doc = "Invoice containing the credited invoice line items"]
    #[json(default)]
    pub invoice: String,
    #[doc = "Credited invoice line items"]
    #[json(default)]
    pub invoice_line_items: Vec<String>,
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
pub enum BillingBillResourceInvoicingPricingPricingType {
    PriceDetails,
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
impl BillingBillResourceInvoicingPricingPricingType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BillingBillResourceInvoicingPricingPricingType::PriceDetails => "price_details",
            BillingBillResourceInvoicingPricingPricingType::Unknown(value) => value,
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
impl Default for BillingBillResourceInvoicingPricingPricingType {
    fn default() -> Self {
        BillingBillResourceInvoicingPricingPricingType::Unknown(String::new())
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
impl<'json> json::Decode<'json> for BillingBillResourceInvoicingPricingPricingType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BillingBillResourceInvoicingPricingPricingType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "price_details" => BillingBillResourceInvoicingPricingPricingType::PriceDetails,
            other => BillingBillResourceInvoicingPricingPricingType::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "billing_portal",
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
pub enum BillingFeedbackOptionStatus {
    Active,
    Inactive,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "billing_portal",
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
impl BillingFeedbackOptionStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BillingFeedbackOptionStatus::Active => "active",
            BillingFeedbackOptionStatus::Inactive => "inactive",
            BillingFeedbackOptionStatus::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "billing_portal",
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
impl Default for BillingFeedbackOptionStatus {
    fn default() -> Self {
        BillingFeedbackOptionStatus::Unknown(String::new())
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "billing_portal",
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
impl<'json> json::Decode<'json> for BillingFeedbackOptionStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BillingFeedbackOptionStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "active" => BillingFeedbackOptionStatus::Active,
            "inactive" => BillingFeedbackOptionStatus::Inactive,
            other => BillingFeedbackOptionStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone, PartialEq)]
pub enum BillingMeterEventAdjustmentObject {
    BillingMeterEventAdjustment,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "billing")]
impl BillingMeterEventAdjustmentObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BillingMeterEventAdjustmentObject::BillingMeterEventAdjustment => "billing.meter_event_adjustment",
            BillingMeterEventAdjustmentObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "billing")]
impl Default for BillingMeterEventAdjustmentObject {
    fn default() -> Self {
        BillingMeterEventAdjustmentObject::Unknown(String::new())
    }
}

#[cfg(feature = "billing")]
impl<'json> json::Decode<'json> for BillingMeterEventAdjustmentObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BillingMeterEventAdjustmentObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "billing.meter_event_adjustment" => BillingMeterEventAdjustmentObject::BillingMeterEventAdjustment,
            other => BillingMeterEventAdjustmentObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone, PartialEq)]
pub enum BillingMeterEventAdjustmentStatus {
    Complete,
    Pending,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "billing")]
impl BillingMeterEventAdjustmentStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BillingMeterEventAdjustmentStatus::Complete => "complete",
            BillingMeterEventAdjustmentStatus::Pending => "pending",
            BillingMeterEventAdjustmentStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "billing")]
impl Default for BillingMeterEventAdjustmentStatus {
    fn default() -> Self {
        BillingMeterEventAdjustmentStatus::Unknown(String::new())
    }
}

#[cfg(feature = "billing")]
impl<'json> json::Decode<'json> for BillingMeterEventAdjustmentStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BillingMeterEventAdjustmentStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "complete" => BillingMeterEventAdjustmentStatus::Complete,
            "pending" => BillingMeterEventAdjustmentStatus::Pending,
            other => BillingMeterEventAdjustmentStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "billing", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum BillingMeterObject {
    BillingMeter,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "billing", feature = "events"))]
impl BillingMeterObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BillingMeterObject::BillingMeter => "billing.meter",
            BillingMeterObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "billing", feature = "events"))]
impl Default for BillingMeterObject {
    fn default() -> Self {
        BillingMeterObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "billing", feature = "events"))]
impl<'json> json::Decode<'json> for BillingMeterObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BillingMeterObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "billing.meter" => BillingMeterObject::BillingMeter,
            other => BillingMeterObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "billing", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BillingMeterResourceBillingMeterValue {
    #[doc = "The key in the meter event payload to use as the value for this meter."]
    #[json(default)]
    pub event_payload_key: String,
}

#[cfg(any(feature = "billing", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum BillingMeterResourceCustomerMappingSettingsType {
    ById,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "billing", feature = "events"))]
impl BillingMeterResourceCustomerMappingSettingsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BillingMeterResourceCustomerMappingSettingsType::ById => "by_id",
            BillingMeterResourceCustomerMappingSettingsType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "billing", feature = "events"))]
impl Default for BillingMeterResourceCustomerMappingSettingsType {
    fn default() -> Self {
        BillingMeterResourceCustomerMappingSettingsType::Unknown(String::new())
    }
}

#[cfg(any(feature = "billing", feature = "events"))]
impl<'json> json::Decode<'json> for BillingMeterResourceCustomerMappingSettingsType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BillingMeterResourceCustomerMappingSettingsType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "by_id" => BillingMeterResourceCustomerMappingSettingsType::ById,
            other => BillingMeterResourceCustomerMappingSettingsType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone, Default)]
pub struct CreateBillingMeterEventParams {
    #[doc = "The name of the meter event."]
    pub event_name: String,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "A unique identifier for the event."]
    pub identifier: Option<String>,
    #[doc = "The payload of the event."]
    pub payload: std::collections::HashMap<String, String>,
    #[doc = "The time of the event."]
    pub timestamp: Option<i64>,
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for CreateBillingMeterEventParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("event_name", &self.event_name);
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("identifier", self.identifier.as_ref());
        for (key, value) in self.payload.iter() {
            form.field_map("payload", key, value);
        }
        form.field_opt("timestamp", self.timestamp.as_ref());
    }
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateBillingMeterParamsEventTimeWindow {
    Day,
    Hour,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "billing")]
impl CreateBillingMeterParamsEventTimeWindow {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateBillingMeterParamsEventTimeWindow::Day => "day",
            CreateBillingMeterParamsEventTimeWindow::Hour => "hour",
            CreateBillingMeterParamsEventTimeWindow::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "billing")]
impl<'json> json::Decode<'json> for CreateBillingMeterParamsEventTimeWindow {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateBillingMeterParamsEventTimeWindow::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "day" => CreateBillingMeterParamsEventTimeWindow::Day,
            "hour" => CreateBillingMeterParamsEventTimeWindow::Hour,
            other => CreateBillingMeterParamsEventTimeWindow::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for CreateBillingMeterParamsEventTimeWindow {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone, Default)]
pub struct UpdateBillingCreditGrantParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The time when the billing credits created by this credit grant expire."]
    pub expires_at: Option<json::OwnedValue>,
    #[doc = "Set of key-value pairs you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for UpdateBillingCreditGrantParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
    }
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone, Default)]
pub struct UpdateBillingFeedbackOptionParams {
    #[doc = "The text of the feedback option, which customers see when canceling."]
    pub description: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for UpdateBillingFeedbackOptionParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("description", self.description.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}
