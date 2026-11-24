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
pub struct SchedulesPhaseAutomaticTax {
    #[doc = "If Stripe disabled automatic tax, this enum describes why."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<stripe_types_02::subscription_schedules::SchedulesPhaseAutomaticTaxDisabledReason>,
    #[doc = "Whether Stripe automatically computes tax on invoices created during this phase."]
    #[json(default)]
    pub enabled: bool,
    #[doc = "The account that's liable for tax."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub liability: Option<crate::accounts::ConnectAccountReference>,
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
pub struct SubscriptionSchedule {
    #[doc = "ID of the Connect Application that created the schedule."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub application:
        Option<crate::Expandable<stripe_types_09::subscription_schedules::SubscriptionScheduleApplicationExpanded>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billing_mode: Option<stripe_types_08::subscriptions::SubscriptionsResourceBillingMode>,
    #[doc = "Time at which the subscription schedule was canceled."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<i64>,
    #[doc = "Time at which the subscription schedule was completed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<i64>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Object representing the start and end dates for the current phase of the subscription schedule, if it is `active`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub current_phase: Option<stripe_types_03::subscription_schedules::SubscriptionScheduleCurrentPhase>,
    #[doc = "ID of the customer who owns the subscription schedule."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<crate::Expandable<crate::subscription_schedules::SubscriptionScheduleCustomerExpanded>>,
    #[doc = "ID of the account who owns the subscription schedule."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_account: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub default_settings: Option<crate::subscription_schedules::SubscriptionSchedulesResourceDefaultSettings>,
    #[doc = "Behavior of the subscription schedule and underlying subscription when it ends."]
    #[json(default)]
    pub end_behavior: stripe_types_00::subscription_schedules::SubscriptionScheduleEndBehavior,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::subscription_schedules::SubscriptionScheduleObject,
    #[doc = "The pause schedules for this subscription schedule."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pause_schedules:
        Option<Vec<stripe_types_13::subscription_schedules::SubscriptionSchedulesResourcePauseSchedule>>,
    #[doc = "Configuration for the subscription schedule's phases."]
    #[json(default)]
    pub phases: Vec<crate::subscription_schedules::SubscriptionSchedulePhaseConfiguration>,
    #[doc = "Time at which the subscription schedule was released."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub released_at: Option<i64>,
    #[doc = "ID of the subscription once managed by the subscription schedule (if it is released)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub released_subscription: Option<String>,
    #[doc = "The present status of the subscription schedule."]
    #[json(default)]
    pub status: stripe_types_00::subscription_schedules::SubscriptionScheduleStatus,
    #[doc = "ID of the subscription managed by the subscription schedule."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub subscription: Option<crate::Expandable<crate::subscriptions::Subscription>>,
    #[doc = "ID of the test clock this subscription schedule belongs to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<crate::Expandable<stripe_types_08::test_helpers::TestHelpersTestClock>>,
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
impl crate::PagedItem for SubscriptionSchedule {
    fn id(&self) -> &str {
        &self.id
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
pub struct SubscriptionScheduleAddInvoiceItem {
    #[doc = "Controls whether discounts apply to this invoice item."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub discountable: Option<bool>,
    #[doc = "The stackable discounts that will be applied to the item."]
    #[json(default)]
    pub discounts: Vec<crate::common::DiscountsResourceStackableDiscountWithDiscountEnd>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<stripe_types_09::subscription_schedules::SubscriptionScheduleAddInvoiceItemPeriod>,
    #[doc = "ID of the price used to generate the invoice item."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<
        crate::Expandable<stripe_types_12::subscription_schedules::SubscriptionScheduleAddInvoiceItemPriceExpanded>,
    >,
    #[doc = "The quantity of the invoice item."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[doc = "The tax rates which apply to the item."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<stripe_types_06::tax_rates::TaxRate>>,
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
pub struct SubscriptionScheduleConfigurationItem {
    #[doc = "Define thresholds at which an invoice will be sent, and the related subscription advanced to a new billing period"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<stripe_types_01::subscription_items::SubscriptionItemBillingThresholds>,
    #[doc = "The discounts applied to the subscription item."]
    #[json(default)]
    pub discounts: Vec<crate::coupons::StackableDiscountWithDiscountSettings>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an item."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "ID of the plan to which the customer should be subscribed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<
        crate::Expandable<stripe_types_13::subscription_schedules::SubscriptionScheduleConfigurationItemPlanExpanded>,
    >,
    #[doc = "ID of the price to which the customer should be subscribed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<
        crate::Expandable<stripe_types_12::subscription_schedules::SubscriptionScheduleAddInvoiceItemPriceExpanded>,
    >,
    #[doc = "Quantity of the plan to which the customer should be subscribed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[doc = "The tax rates which apply to this `phase_item`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<stripe_types_06::tax_rates::TaxRate>>,
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
pub enum SubscriptionScheduleCustomerExpanded {
    /// Expanded `customer`.
    Customer(Box<crate::customers::Customer>),
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
impl<'json> json::Decode<'json> for SubscriptionScheduleCustomerExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "customer" => {
                SubscriptionScheduleCustomerExpanded::Customer(Box::new(json::from_slice_owned(&json::to_vec(&value))?))
            }
            _ => SubscriptionScheduleCustomerExpanded::Other(value),
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
pub struct SubscriptionSchedulePhaseConfiguration {
    #[doc = "A list of prices and quantities that will generate invoice items appended to the next invoice for this phase."]
    #[json(default)]
    pub add_invoice_items: Vec<crate::subscription_schedules::SubscriptionScheduleAddInvoiceItem>,
    #[doc = "A non-negative decimal between 0 and 100, with at most two decimal places."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<crate::subscription_schedules::SchedulesPhaseAutomaticTax>,
    #[doc = "Possible values are `phase_start` or `automatic`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor:
        Option<stripe_types_02::subscription_schedules::SubscriptionSchedulePhaseConfigurationBillingCycleAnchor>,
    #[doc = "Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<stripe_types_01::subscriptions::SubscriptionBillingThresholds>,
    #[doc = "Either `charge_automatically`, or `send_invoice`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub collection_method:
        Option<stripe_types_03::subscription_schedules::SubscriptionSchedulePhaseConfigurationCollectionMethod>,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "ID of the default payment method for the subscription schedule."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<crate::Expandable<crate::payment_methods::PaymentMethod>>,
    #[doc = "The default tax rates to apply to the subscription during this phase of the subscription schedule."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<stripe_types_06::tax_rates::TaxRate>>,
    #[doc = "Subscription description, meant to be displayable to the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The stackable discounts that will be applied to the subscription on this phase."]
    #[json(default)]
    pub discounts: Vec<crate::coupons::StackableDiscountWithDiscountSettingsAndDiscountEnd>,
    #[doc = "The end of this phase of the subscription schedule."]
    #[json(default)]
    pub end_date: i64,
    #[doc = "The invoice settings applicable during this phase."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<crate::invoices::InvoiceSettingSubscriptionSchedulePhaseSetting>,
    #[doc = "Subscription items to configure the subscription to during this phase of the subscription schedule."]
    #[json(default)]
    pub items: Vec<crate::subscription_schedules::SubscriptionScheduleConfigurationItem>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to a phase."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The account (if any) the charge was made on behalf of for charges associated with the schedule's subscription."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<crate::Expandable<crate::account::Account>>,
    #[doc = "When transitioning phases, controls how prorations are handled (if any)."]
    #[json(default)]
    pub proration_behavior:
        stripe_types_01::subscription_schedules::SubscriptionSchedulePhaseConfigurationProrationBehavior,
    #[doc = "The start of this phase of the subscription schedule."]
    #[json(default)]
    pub start_date: i64,
    #[doc = "The account (if any) the associated subscription's payments will be attributed to for tax reporting, and where funds ..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<crate::subscriptions::SubscriptionTransferData>,
    #[doc = "If set to true the entire phase is counted as a trial and the customer will not be charged for any fees."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub trial: Option<bool>,
    #[doc = "When the trial ends within the phase."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<i64>,
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
pub struct SubscriptionSchedulesResourceDefaultSettings {
    #[doc = "A non-negative decimal between 0 and 100, with at most two decimal places."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<crate::subscription_schedules::SubscriptionSchedulesResourceDefaultSettingsAutomaticTax>,
    #[doc = "Possible values are `phase_start` or `automatic`."]
    #[json(default)]
    pub billing_cycle_anchor:
        stripe_types_02::subscription_schedules::SubscriptionSchedulePhaseConfigurationBillingCycleAnchor,
    #[doc = "Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<stripe_types_01::subscriptions::SubscriptionBillingThresholds>,
    #[doc = "Either `charge_automatically`, or `send_invoice`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub collection_method:
        Option<stripe_types_03::subscription_schedules::SubscriptionSchedulePhaseConfigurationCollectionMethod>,
    #[doc = "ID of the default payment method for the subscription schedule."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<crate::Expandable<crate::payment_methods::PaymentMethod>>,
    #[doc = "Subscription description, meant to be displayable to the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<crate::invoices::InvoiceSettingSubscriptionScheduleSetting>,
    #[doc = "The account (if any) the charge was made on behalf of for charges associated with the schedule's subscription."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<crate::Expandable<crate::account::Account>>,
    #[doc = "The account (if any) the associated subscription's payments will be attributed to for tax reporting, and where funds ..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<crate::subscriptions::SubscriptionTransferData>,
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
pub struct SubscriptionSchedulesResourceDefaultSettingsAutomaticTax {
    #[doc = "If Stripe disabled automatic tax, this enum describes why."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<stripe_types_02::subscription_schedules::SchedulesPhaseAutomaticTaxDisabledReason>,
    #[doc = "Whether Stripe automatically computes tax on invoices created during this phase."]
    #[json(default)]
    pub enabled: bool,
    #[doc = "The account that's liable for tax."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub liability: Option<crate::accounts::ConnectAccountReference>,
}
