#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone)]
pub struct CreateSubscriptionScheduleParamsBillingMode {
    #[doc = "Configure behavior for flexible billing mode."]
    pub flexible: Option<stripe_types_06::subscription_schedules::CreateSubscriptionScheduleParamsBillingModeFlexible>,
    #[doc = "Controls the calculation and orchestration of prorations and invoices for subscriptions."]
    pub r#type: stripe_types_03::subscription_schedules::CreateSubscriptionScheduleParamsBillingModeType,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsBillingMode {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("flexible", self.flexible.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone)]
pub struct CreateSubscriptionScheduleParamsDefaultSettingsInvoiceSettingsIssuer {
    #[doc = "The connected account being referenced when `type` is `account`."]
    pub account: Option<String>,
    #[doc = "Type of the account referenced in the request."]
    pub r#type: stripe_types_05::subscription_schedules::CreateSubscriptionScheduleParamsDefaultSettingsInvoiceSettingsIssuerType,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsDefaultSettingsInvoiceSettingsIssuer {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone)]
pub struct CreateSubscriptionScheduleParamsPhasesItemInvoiceSettingsIssuer {
    #[doc = "The connected account being referenced when `type` is `account`."]
    pub account: Option<String>,
    #[doc = "Type of the account referenced in the request."]
    pub r#type:
        stripe_types_05::subscription_schedules::CreateSubscriptionScheduleParamsPhasesItemInvoiceSettingsIssuerType,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsPhasesItemInvoiceSettingsIssuer {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, Default)]
pub struct ListSubscriptionSchedulesParams {
    pub canceled_at: Option<stripe_types_00::subscription_schedules::ListSubscriptionSchedulesParamsCanceledAt>,
    pub completed_at: Option<stripe_types_06::subscription_schedules::ListSubscriptionSchedulesParamsCompletedAt>,
    pub created: Option<stripe_types_06::subscription_schedules::ListSubscriptionSchedulesParamsCreated>,
    pub customer: Option<String>,
    pub customer_account: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub released_at: Option<stripe_types_06::subscription_schedules::ListSubscriptionSchedulesParamsReleasedAt>,
    pub scheduled: Option<bool>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for ListSubscriptionSchedulesParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("canceled_at", self.canceled_at.as_ref());
        form.field_opt("completed_at", self.completed_at.as_ref());
        form.field_opt("created", self.created.as_ref());
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("customer_account", self.customer_account.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("released_at", self.released_at.as_ref());
        form.field_opt("scheduled", self.scheduled.as_ref());
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
pub struct SubscriptionScheduleAddInvoiceItemPeriod {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<
        stripe_types_06::subscription_schedules::SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodEnd,
    >,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<
        stripe_types_05::subscription_schedules::SubscriptionSchedulesResourceInvoiceItemPeriodResourcePeriodStart,
    >,
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
#[derive(Debug, Clone)]
pub enum SubscriptionScheduleApplicationExpanded {
    /// Expanded `application`.
    Application(Box<stripe_types_05::common::Application>),
    /// An object type not known to this version of the crate.
    Other(json::OwnedValue),
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
impl<'json> json::Decode<'json> for SubscriptionScheduleApplicationExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "application" => SubscriptionScheduleApplicationExpanded::Application(Box::new(json::from_slice_owned(
                &json::to_vec(&value),
            )?)),
            _ => SubscriptionScheduleApplicationExpanded::Other(value),
        })
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone)]
pub struct UpdateSubscriptionScheduleParamsDefaultSettingsAutomaticTaxLiability {
    #[doc = "The connected account being referenced when `type` is `account`."]
    pub account: Option<String>,
    #[doc = "Type of the account referenced in the request."]
    pub r#type: stripe_types_06::subscription_schedules::UpdateSubscriptionScheduleParamsDefaultSettingsAutomaticTaxLiabilityType,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for UpdateSubscriptionScheduleParamsDefaultSettingsAutomaticTaxLiability {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone)]
pub struct UpdateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodStart {
    #[doc = "A precise Unix timestamp for the start of the invoice item period."]
    pub timestamp: Option<i64>,
    #[doc = "Select how to calculate the start of the invoice item period."]
    pub r#type: stripe_types_05::subscription_schedules::UpdateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodStartType,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for UpdateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriodStart {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("timestamp", self.timestamp.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPriceData {
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "The ID of the [Product](https://docs.stripe.com/api/products) that this [Price](https://docs.stripe.com/api/prices) w..."]
    pub product: String,
    #[doc = "Only required if a default tax behavior..."]
    pub tax_behavior: Option<stripe_types_05::subscription_schedules::UpdateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPriceDataTaxBehavior>,
    #[doc = "A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge or a negati..."]
    pub unit_amount: Option<i64>,
    #[doc = "Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places."]
    pub unit_amount_decimal: Option<String>,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for UpdateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPriceData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("currency", &self.currency);
        form.field("product", &self.product);
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
        form.field_opt("unit_amount", self.unit_amount.as_ref());
        form.field_opt("unit_amount_decimal", self.unit_amount_decimal.as_ref());
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone)]
pub struct UpdateSubscriptionScheduleParamsPhasesItemDuration {
    #[doc = "Specifies phase duration."]
    pub interval: stripe_types_05::subscription_schedules::UpdateSubscriptionScheduleParamsPhasesItemDurationInterval,
    #[doc = "The multiplier applied to the interval."]
    pub interval_count: Option<i64>,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for UpdateSubscriptionScheduleParamsPhasesItemDuration {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("interval", &self.interval);
        form.field_opt("interval_count", self.interval_count.as_ref());
    }
}
