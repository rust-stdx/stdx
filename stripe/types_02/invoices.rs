#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct AddLinesParamsLinesItemPriceDataProductDataTaxDetails {
    #[doc = "A tax location ID."]
    pub performance_location: Option<String>,
    #[doc = "A [tax code](https://docs.stripe.com/tax/tax-categories) ID."]
    pub tax_code: Option<json::OwnedValue>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for AddLinesParamsLinesItemPriceDataProductDataTaxDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("performance_location", self.performance_location.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct AddLinesParamsLinesItemPricing {
    #[doc = "The ID of the price object."]
    pub price: Option<String>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for AddLinesParamsLinesItemPricing {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("price", self.price.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem
{
    Checking,
    Savings,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem::Checking => "checking",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem::Savings => "savings",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "checking" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem::Checking,
            "savings" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem::Savings,
            other => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Balances => "balances",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Ownership => "ownership",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::PaymentMethod => "payment_method",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Transactions => "transactions",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json>
    for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "balances" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Balances,
            "ownership" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Ownership,
            "payment_method" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::PaymentMethod,
            "transactions" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Transactions,
            other => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode
    for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit::BusinessDay => "business_day",
            CreateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit::Day => "day",
            CreateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit::Hour => "hour",
            CreateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit::Month => "month",
            CreateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit::Week => "week",
            CreateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for CreateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "business_day" => CreateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit::BusinessDay,
            "day" => CreateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit::Day,
            "hour" => CreateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit::Hour,
            "month" => CreateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit::Month,
            "week" => CreateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit::Week,
            other => {
                CreateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreateInvoiceParamsTransferData {
    #[doc = "The amount that will be transferred automatically when the invoice is paid."]
    pub amount: Option<i64>,
    #[doc = "ID of an existing, connected Stripe account."]
    pub destination: String,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsTransferData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field("destination", &self.destination);
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreatePreviewParamsInvoiceItemsItemPeriod {
    #[doc = "The end of the period, which must be greater than or equal to the start."]
    pub end: i64,
    #[doc = "The start of the period."]
    pub start: i64,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsInvoiceItemsItemPeriod {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("end", &self.end);
        form.field("start", &self.start);
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePreviewParamsScheduleDetailsBillingModeFlexibleProrationDiscounts {
    Included,
    Itemized,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreatePreviewParamsScheduleDetailsBillingModeFlexibleProrationDiscounts {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePreviewParamsScheduleDetailsBillingModeFlexibleProrationDiscounts::Included => "included",
            CreatePreviewParamsScheduleDetailsBillingModeFlexibleProrationDiscounts::Itemized => "itemized",
            CreatePreviewParamsScheduleDetailsBillingModeFlexibleProrationDiscounts::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for CreatePreviewParamsScheduleDetailsBillingModeFlexibleProrationDiscounts {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePreviewParamsScheduleDetailsBillingModeFlexibleProrationDiscounts::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "included" => CreatePreviewParamsScheduleDetailsBillingModeFlexibleProrationDiscounts::Included,
            "itemized" => CreatePreviewParamsScheduleDetailsBillingModeFlexibleProrationDiscounts::Itemized,
            other => {
                CreatePreviewParamsScheduleDetailsBillingModeFlexibleProrationDiscounts::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsScheduleDetailsBillingModeFlexibleProrationDiscounts {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePreviewParamsScheduleDetailsPhasesItemBillingCycleAnchor {
    Automatic,
    PhaseStart,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreatePreviewParamsScheduleDetailsPhasesItemBillingCycleAnchor {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePreviewParamsScheduleDetailsPhasesItemBillingCycleAnchor::Automatic => "automatic",
            CreatePreviewParamsScheduleDetailsPhasesItemBillingCycleAnchor::PhaseStart => "phase_start",
            CreatePreviewParamsScheduleDetailsPhasesItemBillingCycleAnchor::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for CreatePreviewParamsScheduleDetailsPhasesItemBillingCycleAnchor {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePreviewParamsScheduleDetailsPhasesItemBillingCycleAnchor::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "automatic" => CreatePreviewParamsScheduleDetailsPhasesItemBillingCycleAnchor::Automatic,
            "phase_start" => CreatePreviewParamsScheduleDetailsPhasesItemBillingCycleAnchor::PhaseStart,
            other => CreatePreviewParamsScheduleDetailsPhasesItemBillingCycleAnchor::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsScheduleDetailsPhasesItemBillingCycleAnchor {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePreviewParamsScheduleDetailsPhasesItemDurationInterval {
    Day,
    Month,
    Week,
    Year,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreatePreviewParamsScheduleDetailsPhasesItemDurationInterval {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePreviewParamsScheduleDetailsPhasesItemDurationInterval::Day => "day",
            CreatePreviewParamsScheduleDetailsPhasesItemDurationInterval::Month => "month",
            CreatePreviewParamsScheduleDetailsPhasesItemDurationInterval::Week => "week",
            CreatePreviewParamsScheduleDetailsPhasesItemDurationInterval::Year => "year",
            CreatePreviewParamsScheduleDetailsPhasesItemDurationInterval::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for CreatePreviewParamsScheduleDetailsPhasesItemDurationInterval {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePreviewParamsScheduleDetailsPhasesItemDurationInterval::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "day" => CreatePreviewParamsScheduleDetailsPhasesItemDurationInterval::Day,
            "month" => CreatePreviewParamsScheduleDetailsPhasesItemDurationInterval::Month,
            "week" => CreatePreviewParamsScheduleDetailsPhasesItemDurationInterval::Week,
            "year" => CreatePreviewParamsScheduleDetailsPhasesItemDurationInterval::Year,
            other => CreatePreviewParamsScheduleDetailsPhasesItemDurationInterval::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsScheduleDetailsPhasesItemDurationInterval {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreatePreviewParamsScheduleDetailsPhasesItemItemsItemBillingThresholds {
    #[doc = "Number of units that meets the billing threshold to advance the subscription to a new billing period (e.g., it takes ..."]
    pub usage_gte: i64,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsScheduleDetailsPhasesItemItemsItemBillingThresholds {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("usage_gte", &self.usage_gte);
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePreviewParamsSubscriptionDetailsPauseInvoicingBehavior {
    Invoice,
    PendingInvoiceItem,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreatePreviewParamsSubscriptionDetailsPauseInvoicingBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePreviewParamsSubscriptionDetailsPauseInvoicingBehavior::Invoice => "invoice",
            CreatePreviewParamsSubscriptionDetailsPauseInvoicingBehavior::PendingInvoiceItem => "pending_invoice_item",
            CreatePreviewParamsSubscriptionDetailsPauseInvoicingBehavior::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for CreatePreviewParamsSubscriptionDetailsPauseInvoicingBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePreviewParamsSubscriptionDetailsPauseInvoicingBehavior::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "invoice" => CreatePreviewParamsSubscriptionDetailsPauseInvoicingBehavior::Invoice,
            "pending_invoice_item" => CreatePreviewParamsSubscriptionDetailsPauseInvoicingBehavior::PendingInvoiceItem,
            other => CreatePreviewParamsSubscriptionDetailsPauseInvoicingBehavior::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsSubscriptionDetailsPauseInvoicingBehavior {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct GetInvoiceParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for GetInvoiceParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
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
#[derive(Debug, Clone, PartialEq)]
pub enum InvoiceBillingReason {
    AutomaticPendingInvoiceItemInvoice,
    Manual,
    QuoteAccept,
    Subscription,
    SubscriptionCreate,
    SubscriptionCycle,
    SubscriptionThreshold,
    SubscriptionUpdate,
    Upcoming,
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
impl InvoiceBillingReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InvoiceBillingReason::AutomaticPendingInvoiceItemInvoice => "automatic_pending_invoice_item_invoice",
            InvoiceBillingReason::Manual => "manual",
            InvoiceBillingReason::QuoteAccept => "quote_accept",
            InvoiceBillingReason::Subscription => "subscription",
            InvoiceBillingReason::SubscriptionCreate => "subscription_create",
            InvoiceBillingReason::SubscriptionCycle => "subscription_cycle",
            InvoiceBillingReason::SubscriptionThreshold => "subscription_threshold",
            InvoiceBillingReason::SubscriptionUpdate => "subscription_update",
            InvoiceBillingReason::Upcoming => "upcoming",
            InvoiceBillingReason::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for InvoiceBillingReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InvoiceBillingReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "automatic_pending_invoice_item_invoice" => InvoiceBillingReason::AutomaticPendingInvoiceItemInvoice,
            "manual" => InvoiceBillingReason::Manual,
            "quote_accept" => InvoiceBillingReason::QuoteAccept,
            "subscription" => InvoiceBillingReason::Subscription,
            "subscription_create" => InvoiceBillingReason::SubscriptionCreate,
            "subscription_cycle" => InvoiceBillingReason::SubscriptionCycle,
            "subscription_threshold" => InvoiceBillingReason::SubscriptionThreshold,
            "subscription_update" => InvoiceBillingReason::SubscriptionUpdate,
            "upcoming" => InvoiceBillingReason::Upcoming,
            other => InvoiceBillingReason::Unknown(other.to_string()),
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
pub enum InvoicesPaymentSettingsPaymentMethodTypesItem {
    AchCreditTransfer,
    AchDebit,
    AcssDebit,
    Affirm,
    Alipay,
    AmazonPay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Billie,
    Blik,
    Boleto,
    Card,
    Cashapp,
    Crypto,
    Custom,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    JpCreditTransfer,
    KakaoPay,
    Klarna,
    Konbini,
    KrCard,
    Link,
    MbWay,
    Multibanco,
    NaverPay,
    NzBankAccount,
    P24,
    PayByBank,
    Payco,
    Paynow,
    Paypal,
    Payto,
    Pix,
    Promptpay,
    RevolutPay,
    Satispay,
    SepaCreditTransfer,
    SepaDebit,
    Sofort,
    Swish,
    Twint,
    Upi,
    UsBankAccount,
    WechatPay,
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
impl InvoicesPaymentSettingsPaymentMethodTypesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InvoicesPaymentSettingsPaymentMethodTypesItem::AchCreditTransfer => "ach_credit_transfer",
            InvoicesPaymentSettingsPaymentMethodTypesItem::AchDebit => "ach_debit",
            InvoicesPaymentSettingsPaymentMethodTypesItem::AcssDebit => "acss_debit",
            InvoicesPaymentSettingsPaymentMethodTypesItem::Affirm => "affirm",
            InvoicesPaymentSettingsPaymentMethodTypesItem::Alipay => "alipay",
            InvoicesPaymentSettingsPaymentMethodTypesItem::AmazonPay => "amazon_pay",
            InvoicesPaymentSettingsPaymentMethodTypesItem::AuBecsDebit => "au_becs_debit",
            InvoicesPaymentSettingsPaymentMethodTypesItem::BacsDebit => "bacs_debit",
            InvoicesPaymentSettingsPaymentMethodTypesItem::Bancontact => "bancontact",
            InvoicesPaymentSettingsPaymentMethodTypesItem::Billie => "billie",
            InvoicesPaymentSettingsPaymentMethodTypesItem::Blik => "blik",
            InvoicesPaymentSettingsPaymentMethodTypesItem::Boleto => "boleto",
            InvoicesPaymentSettingsPaymentMethodTypesItem::Card => "card",
            InvoicesPaymentSettingsPaymentMethodTypesItem::Cashapp => "cashapp",
            InvoicesPaymentSettingsPaymentMethodTypesItem::Crypto => "crypto",
            InvoicesPaymentSettingsPaymentMethodTypesItem::Custom => "custom",
            InvoicesPaymentSettingsPaymentMethodTypesItem::CustomerBalance => "customer_balance",
            InvoicesPaymentSettingsPaymentMethodTypesItem::Eps => "eps",
            InvoicesPaymentSettingsPaymentMethodTypesItem::Fpx => "fpx",
            InvoicesPaymentSettingsPaymentMethodTypesItem::Giropay => "giropay",
            InvoicesPaymentSettingsPaymentMethodTypesItem::Grabpay => "grabpay",
            InvoicesPaymentSettingsPaymentMethodTypesItem::Ideal => "ideal",
            InvoicesPaymentSettingsPaymentMethodTypesItem::JpCreditTransfer => "jp_credit_transfer",
            InvoicesPaymentSettingsPaymentMethodTypesItem::KakaoPay => "kakao_pay",
            InvoicesPaymentSettingsPaymentMethodTypesItem::Klarna => "klarna",
            InvoicesPaymentSettingsPaymentMethodTypesItem::Konbini => "konbini",
            InvoicesPaymentSettingsPaymentMethodTypesItem::KrCard => "kr_card",
            InvoicesPaymentSettingsPaymentMethodTypesItem::Link => "link",
            InvoicesPaymentSettingsPaymentMethodTypesItem::MbWay => "mb_way",
            InvoicesPaymentSettingsPaymentMethodTypesItem::Multibanco => "multibanco",
            InvoicesPaymentSettingsPaymentMethodTypesItem::NaverPay => "naver_pay",
            InvoicesPaymentSettingsPaymentMethodTypesItem::NzBankAccount => "nz_bank_account",
            InvoicesPaymentSettingsPaymentMethodTypesItem::P24 => "p24",
            InvoicesPaymentSettingsPaymentMethodTypesItem::PayByBank => "pay_by_bank",
            InvoicesPaymentSettingsPaymentMethodTypesItem::Payco => "payco",
            InvoicesPaymentSettingsPaymentMethodTypesItem::Paynow => "paynow",
            InvoicesPaymentSettingsPaymentMethodTypesItem::Paypal => "paypal",
            InvoicesPaymentSettingsPaymentMethodTypesItem::Payto => "payto",
            InvoicesPaymentSettingsPaymentMethodTypesItem::Pix => "pix",
            InvoicesPaymentSettingsPaymentMethodTypesItem::Promptpay => "promptpay",
            InvoicesPaymentSettingsPaymentMethodTypesItem::RevolutPay => "revolut_pay",
            InvoicesPaymentSettingsPaymentMethodTypesItem::Satispay => "satispay",
            InvoicesPaymentSettingsPaymentMethodTypesItem::SepaCreditTransfer => "sepa_credit_transfer",
            InvoicesPaymentSettingsPaymentMethodTypesItem::SepaDebit => "sepa_debit",
            InvoicesPaymentSettingsPaymentMethodTypesItem::Sofort => "sofort",
            InvoicesPaymentSettingsPaymentMethodTypesItem::Swish => "swish",
            InvoicesPaymentSettingsPaymentMethodTypesItem::Twint => "twint",
            InvoicesPaymentSettingsPaymentMethodTypesItem::Upi => "upi",
            InvoicesPaymentSettingsPaymentMethodTypesItem::UsBankAccount => "us_bank_account",
            InvoicesPaymentSettingsPaymentMethodTypesItem::WechatPay => "wechat_pay",
            InvoicesPaymentSettingsPaymentMethodTypesItem::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for InvoicesPaymentSettingsPaymentMethodTypesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InvoicesPaymentSettingsPaymentMethodTypesItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ach_credit_transfer" => InvoicesPaymentSettingsPaymentMethodTypesItem::AchCreditTransfer,
            "ach_debit" => InvoicesPaymentSettingsPaymentMethodTypesItem::AchDebit,
            "acss_debit" => InvoicesPaymentSettingsPaymentMethodTypesItem::AcssDebit,
            "affirm" => InvoicesPaymentSettingsPaymentMethodTypesItem::Affirm,
            "alipay" => InvoicesPaymentSettingsPaymentMethodTypesItem::Alipay,
            "amazon_pay" => InvoicesPaymentSettingsPaymentMethodTypesItem::AmazonPay,
            "au_becs_debit" => InvoicesPaymentSettingsPaymentMethodTypesItem::AuBecsDebit,
            "bacs_debit" => InvoicesPaymentSettingsPaymentMethodTypesItem::BacsDebit,
            "bancontact" => InvoicesPaymentSettingsPaymentMethodTypesItem::Bancontact,
            "billie" => InvoicesPaymentSettingsPaymentMethodTypesItem::Billie,
            "blik" => InvoicesPaymentSettingsPaymentMethodTypesItem::Blik,
            "boleto" => InvoicesPaymentSettingsPaymentMethodTypesItem::Boleto,
            "card" => InvoicesPaymentSettingsPaymentMethodTypesItem::Card,
            "cashapp" => InvoicesPaymentSettingsPaymentMethodTypesItem::Cashapp,
            "crypto" => InvoicesPaymentSettingsPaymentMethodTypesItem::Crypto,
            "custom" => InvoicesPaymentSettingsPaymentMethodTypesItem::Custom,
            "customer_balance" => InvoicesPaymentSettingsPaymentMethodTypesItem::CustomerBalance,
            "eps" => InvoicesPaymentSettingsPaymentMethodTypesItem::Eps,
            "fpx" => InvoicesPaymentSettingsPaymentMethodTypesItem::Fpx,
            "giropay" => InvoicesPaymentSettingsPaymentMethodTypesItem::Giropay,
            "grabpay" => InvoicesPaymentSettingsPaymentMethodTypesItem::Grabpay,
            "ideal" => InvoicesPaymentSettingsPaymentMethodTypesItem::Ideal,
            "jp_credit_transfer" => InvoicesPaymentSettingsPaymentMethodTypesItem::JpCreditTransfer,
            "kakao_pay" => InvoicesPaymentSettingsPaymentMethodTypesItem::KakaoPay,
            "klarna" => InvoicesPaymentSettingsPaymentMethodTypesItem::Klarna,
            "konbini" => InvoicesPaymentSettingsPaymentMethodTypesItem::Konbini,
            "kr_card" => InvoicesPaymentSettingsPaymentMethodTypesItem::KrCard,
            "link" => InvoicesPaymentSettingsPaymentMethodTypesItem::Link,
            "mb_way" => InvoicesPaymentSettingsPaymentMethodTypesItem::MbWay,
            "multibanco" => InvoicesPaymentSettingsPaymentMethodTypesItem::Multibanco,
            "naver_pay" => InvoicesPaymentSettingsPaymentMethodTypesItem::NaverPay,
            "nz_bank_account" => InvoicesPaymentSettingsPaymentMethodTypesItem::NzBankAccount,
            "p24" => InvoicesPaymentSettingsPaymentMethodTypesItem::P24,
            "pay_by_bank" => InvoicesPaymentSettingsPaymentMethodTypesItem::PayByBank,
            "payco" => InvoicesPaymentSettingsPaymentMethodTypesItem::Payco,
            "paynow" => InvoicesPaymentSettingsPaymentMethodTypesItem::Paynow,
            "paypal" => InvoicesPaymentSettingsPaymentMethodTypesItem::Paypal,
            "payto" => InvoicesPaymentSettingsPaymentMethodTypesItem::Payto,
            "pix" => InvoicesPaymentSettingsPaymentMethodTypesItem::Pix,
            "promptpay" => InvoicesPaymentSettingsPaymentMethodTypesItem::Promptpay,
            "revolut_pay" => InvoicesPaymentSettingsPaymentMethodTypesItem::RevolutPay,
            "satispay" => InvoicesPaymentSettingsPaymentMethodTypesItem::Satispay,
            "sepa_credit_transfer" => InvoicesPaymentSettingsPaymentMethodTypesItem::SepaCreditTransfer,
            "sepa_debit" => InvoicesPaymentSettingsPaymentMethodTypesItem::SepaDebit,
            "sofort" => InvoicesPaymentSettingsPaymentMethodTypesItem::Sofort,
            "swish" => InvoicesPaymentSettingsPaymentMethodTypesItem::Swish,
            "twint" => InvoicesPaymentSettingsPaymentMethodTypesItem::Twint,
            "upi" => InvoicesPaymentSettingsPaymentMethodTypesItem::Upi,
            "us_bank_account" => InvoicesPaymentSettingsPaymentMethodTypesItem::UsBankAccount,
            "wechat_pay" => InvoicesPaymentSettingsPaymentMethodTypesItem::WechatPay,
            other => InvoicesPaymentSettingsPaymentMethodTypesItem::Unknown(other.to_string()),
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
pub struct InvoicesResourceStatusTransitions {
    #[doc = "The time that the invoice draft was finalized."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub finalized_at: Option<i64>,
    #[doc = "The time that the invoice was marked uncollectible."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub marked_uncollectible_at: Option<i64>,
    #[doc = "The time that the invoice was paid."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<i64>,
    #[doc = "The time that the invoice was voided."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub voided_at: Option<i64>,
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum RemoveLinesParamsLinesItemBehavior {
    Delete,
    Unassign,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl RemoveLinesParamsLinesItemBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            RemoveLinesParamsLinesItemBehavior::Delete => "delete",
            RemoveLinesParamsLinesItemBehavior::Unassign => "unassign",
            RemoveLinesParamsLinesItemBehavior::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for RemoveLinesParamsLinesItemBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(RemoveLinesParamsLinesItemBehavior::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "delete" => RemoveLinesParamsLinesItemBehavior::Delete,
            "unassign" => RemoveLinesParamsLinesItemBehavior::Unassign,
            other => RemoveLinesParamsLinesItemBehavior::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for RemoveLinesParamsLinesItemBehavior {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
