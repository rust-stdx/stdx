#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionParamsAddInvoiceItemsItemDiscountsItem {
    #[doc = "ID of the coupon to create a new discount for."]
    pub coupon: Option<String>,
    #[doc = "ID of an existing discount on the object (or one of its ancestors) to reuse."]
    pub discount: Option<String>,
    #[doc = "ID of the promotion code to create a new discount for."]
    pub promotion_code: Option<String>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsAddInvoiceItemsItemDiscountsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("coupon", self.coupon.as_ref());
        form.field_opt("discount", self.discount.as_ref());
        form.field_opt("promotion_code", self.promotion_code.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionParamsAddInvoiceItemsItemPeriodEndType {
    MinItemPeriodEnd,
    Timestamp,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl CreateSubscriptionParamsAddInvoiceItemsItemPeriodEndType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionParamsAddInvoiceItemsItemPeriodEndType::MinItemPeriodEnd => "min_item_period_end",
            CreateSubscriptionParamsAddInvoiceItemsItemPeriodEndType::Timestamp => "timestamp",
            CreateSubscriptionParamsAddInvoiceItemsItemPeriodEndType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json> for CreateSubscriptionParamsAddInvoiceItemsItemPeriodEndType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSubscriptionParamsAddInvoiceItemsItemPeriodEndType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "min_item_period_end" => CreateSubscriptionParamsAddInvoiceItemsItemPeriodEndType::MinItemPeriodEnd,
            "timestamp" => CreateSubscriptionParamsAddInvoiceItemsItemPeriodEndType::Timestamp,
            other => CreateSubscriptionParamsAddInvoiceItemsItemPeriodEndType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsAddInvoiceItemsItemPeriodEndType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionParamsAutomaticTaxLiabilityType {
    Account,
    SelfValue,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl CreateSubscriptionParamsAutomaticTaxLiabilityType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionParamsAutomaticTaxLiabilityType::Account => "account",
            CreateSubscriptionParamsAutomaticTaxLiabilityType::SelfValue => "self",
            CreateSubscriptionParamsAutomaticTaxLiabilityType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json> for CreateSubscriptionParamsAutomaticTaxLiabilityType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSubscriptionParamsAutomaticTaxLiabilityType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account" => CreateSubscriptionParamsAutomaticTaxLiabilityType::Account,
            "self" => CreateSubscriptionParamsAutomaticTaxLiabilityType::SelfValue,
            other => CreateSubscriptionParamsAutomaticTaxLiabilityType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsAutomaticTaxLiabilityType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionParamsBillingCycleAnchorConfig {
    #[doc = "The day of the month the anchor should be."]
    pub day_of_month: i64,
    #[doc = "The hour of the day the anchor should be."]
    pub hour: Option<i64>,
    #[doc = "The minute of the hour the anchor should be."]
    pub minute: Option<i64>,
    #[doc = "The month to start full cycle periods."]
    pub month: Option<i64>,
    #[doc = "The second of the minute the anchor should be."]
    pub second: Option<i64>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsBillingCycleAnchorConfig {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("day_of_month", &self.day_of_month);
        form.field_opt("hour", self.hour.as_ref());
        form.field_opt("minute", self.minute.as_ref());
        form.field_opt("month", self.month.as_ref());
        form.field_opt("second", self.second.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionParamsBillingModeFlexibleProrationDiscounts {
    Included,
    Itemized,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl CreateSubscriptionParamsBillingModeFlexibleProrationDiscounts {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionParamsBillingModeFlexibleProrationDiscounts::Included => "included",
            CreateSubscriptionParamsBillingModeFlexibleProrationDiscounts::Itemized => "itemized",
            CreateSubscriptionParamsBillingModeFlexibleProrationDiscounts::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json> for CreateSubscriptionParamsBillingModeFlexibleProrationDiscounts {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSubscriptionParamsBillingModeFlexibleProrationDiscounts::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "included" => CreateSubscriptionParamsBillingModeFlexibleProrationDiscounts::Included,
            "itemized" => CreateSubscriptionParamsBillingModeFlexibleProrationDiscounts::Itemized,
            other => CreateSubscriptionParamsBillingModeFlexibleProrationDiscounts::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsBillingModeFlexibleProrationDiscounts {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionParamsBillingSchedulesItemBillUntilDurationInterval {
    Day,
    Month,
    Week,
    Year,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl CreateSubscriptionParamsBillingSchedulesItemBillUntilDurationInterval {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionParamsBillingSchedulesItemBillUntilDurationInterval::Day => "day",
            CreateSubscriptionParamsBillingSchedulesItemBillUntilDurationInterval::Month => "month",
            CreateSubscriptionParamsBillingSchedulesItemBillUntilDurationInterval::Week => "week",
            CreateSubscriptionParamsBillingSchedulesItemBillUntilDurationInterval::Year => "year",
            CreateSubscriptionParamsBillingSchedulesItemBillUntilDurationInterval::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json> for CreateSubscriptionParamsBillingSchedulesItemBillUntilDurationInterval {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSubscriptionParamsBillingSchedulesItemBillUntilDurationInterval::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "day" => CreateSubscriptionParamsBillingSchedulesItemBillUntilDurationInterval::Day,
            "month" => CreateSubscriptionParamsBillingSchedulesItemBillUntilDurationInterval::Month,
            "week" => CreateSubscriptionParamsBillingSchedulesItemBillUntilDurationInterval::Week,
            "year" => CreateSubscriptionParamsBillingSchedulesItemBillUntilDurationInterval::Year,
            other => CreateSubscriptionParamsBillingSchedulesItemBillUntilDurationInterval::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsBillingSchedulesItemBillUntilDurationInterval {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionParamsBillingThresholds {
    #[doc = "Monetary threshold that triggers the subscription to advance to a new billing period"]
    pub amount_gte: Option<i64>,
    #[doc = "Indicates if the `billing_cycle_anchor` should be reset when a threshold is reached."]
    pub reset_billing_cycle_anchor: Option<bool>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsBillingThresholds {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount_gte", self.amount_gte.as_ref());
        form.field_opt("reset_billing_cycle_anchor", self.reset_billing_cycle_anchor.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionParamsItemsItemCurrentTrial {
    #[doc = "The ID of the trial offer to apply to the subscription item."]
    pub trial_offer: String,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsItemsItemCurrentTrial {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("trial_offer", &self.trial_offer);
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Business => "business",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Personal => "personal",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json>
    for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "business" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Business,
            "personal" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Personal,
            other => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode
    for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork {
    Amex,
    CartesBancaires,
    Diners,
    Discover,
    EftposAu,
    Girocard,
    Interac,
    Jcb,
    Link,
    Mastercard,
    Unionpay,
    UnknownValue,
    Visa,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork::Amex => "amex",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork::CartesBancaires => {
                "cartes_bancaires"
            }
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork::Diners => "diners",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork::Discover => "discover",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork::EftposAu => "eftpos_au",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork::Girocard => "girocard",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork::Interac => "interac",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork::Jcb => "jcb",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork::Link => "link",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork::Mastercard => "mastercard",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork::Unionpay => "unionpay",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork::UnknownValue => "unknown",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork::Visa => "visa",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json> for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "amex" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork::Amex,
            "cartes_bancaires" => {
                CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork::CartesBancaires
            }
            "diners" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork::Diners,
            "discover" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork::Discover,
            "eftpos_au" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork::EftposAu,
            "girocard" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork::Girocard,
            "interac" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork::Interac,
            "jcb" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork::Jcb,
            "link" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork::Link,
            "mastercard" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork::Mastercard,
            "unionpay" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork::Unionpay,
            "unknown" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork::UnknownValue,
            "visa" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork::Visa,
            other => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardNetwork {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer {
    #[doc = "The desired country code of the bank account information."]
    pub country: String,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode
    for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("country", &self.country);
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsPaymentSchedule {
    Halfyearly,
    Monthly,
    Quarterly,
    Weekly,
    Yearly,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsPaymentSchedule {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Halfyearly => {
                "halfyearly"
            }
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Monthly => {
                "monthly"
            }
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Quarterly => {
                "quarterly"
            }
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Weekly => {
                "weekly"
            }
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Yearly => {
                "yearly"
            }
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Unknown(
                value,
            ) => value,
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json>
    for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsPaymentSchedule
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "halfyearly" => {
                CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Halfyearly
            }
            "monthly" => {
                CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Monthly
            }
            "quarterly" => {
                CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Quarterly
            }
            "weekly" => {
                CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Weekly
            }
            "yearly" => {
                CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Yearly
            }
            other => {
                CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Unknown(
                    other.to_string(),
                )
            }
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode
    for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsPaymentSchedule
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct GetSubscriptionParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for GetSubscriptionParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct SearchSubscriptionsParams {
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub page: Option<String>,
    pub query: String,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for SearchSubscriptionsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("page", self.page.as_ref());
        form.field("query", &self.query);
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
pub enum SubscriptionCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
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
impl SubscriptionCollectionMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionCollectionMethod::ChargeAutomatically => "charge_automatically",
            SubscriptionCollectionMethod::SendInvoice => "send_invoice",
            SubscriptionCollectionMethod::Unknown(value) => value,
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
impl Default for SubscriptionCollectionMethod {
    fn default() -> Self {
        SubscriptionCollectionMethod::Unknown(String::new())
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
impl<'json> json::Decode<'json> for SubscriptionCollectionMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SubscriptionCollectionMethod::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "charge_automatically" => SubscriptionCollectionMethod::ChargeAutomatically,
            "send_invoice" => SubscriptionCollectionMethod::SendInvoice,
            other => SubscriptionCollectionMethod::Unknown(other.to_string()),
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
pub enum SubscriptionStatus {
    Active,
    Canceled,
    Incomplete,
    IncompleteExpired,
    PastDue,
    Paused,
    Trialing,
    Unpaid,
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
impl SubscriptionStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionStatus::Active => "active",
            SubscriptionStatus::Canceled => "canceled",
            SubscriptionStatus::Incomplete => "incomplete",
            SubscriptionStatus::IncompleteExpired => "incomplete_expired",
            SubscriptionStatus::PastDue => "past_due",
            SubscriptionStatus::Paused => "paused",
            SubscriptionStatus::Trialing => "trialing",
            SubscriptionStatus::Unpaid => "unpaid",
            SubscriptionStatus::Unknown(value) => value,
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
impl Default for SubscriptionStatus {
    fn default() -> Self {
        SubscriptionStatus::Unknown(String::new())
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
impl<'json> json::Decode<'json> for SubscriptionStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SubscriptionStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "active" => SubscriptionStatus::Active,
            "canceled" => SubscriptionStatus::Canceled,
            "incomplete" => SubscriptionStatus::Incomplete,
            "incomplete_expired" => SubscriptionStatus::IncompleteExpired,
            "past_due" => SubscriptionStatus::PastDue,
            "paused" => SubscriptionStatus::Paused,
            "trialing" => SubscriptionStatus::Trialing,
            "unpaid" => SubscriptionStatus::Unpaid,
            other => SubscriptionStatus::Unknown(other.to_string()),
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
pub enum SubscriptionsResourceBillingSchedulesAppliesToType {
    Price,
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
impl SubscriptionsResourceBillingSchedulesAppliesToType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionsResourceBillingSchedulesAppliesToType::Price => "price",
            SubscriptionsResourceBillingSchedulesAppliesToType::Unknown(value) => value,
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
impl Default for SubscriptionsResourceBillingSchedulesAppliesToType {
    fn default() -> Self {
        SubscriptionsResourceBillingSchedulesAppliesToType::Unknown(String::new())
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
impl<'json> json::Decode<'json> for SubscriptionsResourceBillingSchedulesAppliesToType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SubscriptionsResourceBillingSchedulesAppliesToType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "price" => SubscriptionsResourceBillingSchedulesAppliesToType::Price,
            other => SubscriptionsResourceBillingSchedulesAppliesToType::Unknown(other.to_string()),
        })
    }
}
