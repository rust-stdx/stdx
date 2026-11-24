#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionScheduleParams {
    #[doc = "Controls how prorations and invoices for subscriptions are calculated and orchestrated."]
    pub billing_mode: Option<stripe_types_09::subscription_schedules::CreateSubscriptionScheduleParamsBillingMode>,
    #[doc = "The identifier of the customer to create the subscription schedule for."]
    pub customer: Option<String>,
    #[doc = "The identifier of the account to create the subscription schedule for."]
    pub customer_account: Option<String>,
    #[doc = "Object representing the subscription schedule's default settings."]
    pub default_settings:
        Option<stripe_types_12::subscription_schedules::CreateSubscriptionScheduleParamsDefaultSettings>,
    #[doc = "Behavior of the subscription schedule and underlying subscription when it ends."]
    pub end_behavior: Option<stripe_types_01::subscription_schedules::CreateSubscriptionScheduleParamsEndBehavior>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Migrate an existing subscription to be managed by a subscription schedule."]
    pub from_subscription: Option<String>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Configures the subscription's pause behavior and, optionally, its resume behavior."]
    pub pause_schedules: Option<Vec<crate::subscription_schedules::CreateSubscriptionScheduleParamsPauseSchedulesItem>>,
    #[doc = "List representing phases of the subscription schedule."]
    pub phases: Option<Vec<crate::subscription_schedules::CreateSubscriptionScheduleParamsPhasesItem>>,
    #[doc = "When the subscription schedule starts."]
    pub start_date: Option<json::OwnedValue>,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("billing_mode", self.billing_mode.as_ref());
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("customer_account", self.customer_account.as_ref());
        form.field_opt("default_settings", self.default_settings.as_ref());
        form.field_opt("end_behavior", self.end_behavior.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("from_subscription", self.from_subscription.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        if let Some(values) = self.pause_schedules.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("pause_schedules", index, item);
            }
        }
        if let Some(values) = self.phases.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("phases", index, item);
            }
        }
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionScheduleParamsPauseSchedulesItem {
    #[doc = "A unique identifier for this pause schedule entry."]
    pub key: Option<String>,
    #[doc = "Configuration for when and how the subscription pauses."]
    pub pause: Option<stripe_types_12::subscription_schedules::CreateSubscriptionScheduleParamsPauseSchedulesItemPause>,
    #[doc = "Configuration for when and how the subscription resumes."]
    pub resume:
        Option<stripe_types_10::subscription_schedules::CreateSubscriptionScheduleParamsPauseSchedulesItemResume>,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsPauseSchedulesItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("key", self.key.as_ref());
        form.field_opt("pause", self.pause.as_ref());
        form.field_opt("resume", self.resume.as_ref());
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionScheduleParamsPhasesItem {
    #[doc = "A list of prices and quantities that will generate invoice items appended to the next invoice for this phase."]
    pub add_invoice_items: Option<
        Vec<stripe_types_11::subscription_schedules::CreateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItem>,
    >,
    #[doc = "A non-negative decimal between 0 and 100, with at most two decimal places."]
    pub application_fee_percent: Option<f64>,
    #[doc = "Automatic tax settings for this phase."]
    pub automatic_tax:
        Option<stripe_types_11::subscription_schedules::CreateSubscriptionScheduleParamsPhasesItemAutomaticTax>,
    #[doc = "Can be set to `phase_start` to set the anchor to the start of the phase or `automatic` to automatically change it if ..."]
    pub billing_cycle_anchor:
        Option<stripe_types_05::subscription_schedules::CreateSubscriptionScheduleParamsPhasesItemBillingCycleAnchor>,
    #[doc = "Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period."]
    pub billing_thresholds:
        Option<stripe_types_05::subscription_schedules::CreateSubscriptionScheduleParamsPhasesItemBillingThresholds>,
    #[doc = "Either `charge_automatically`, or `send_invoice`."]
    pub collection_method:
        Option<stripe_types_05::subscription_schedules::CreateSubscriptionScheduleParamsPhasesItemCollectionMethod>,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: Option<String>,
    #[doc = "ID of the default payment method for the subscription schedule."]
    pub default_payment_method: Option<String>,
    #[doc = "A list of [Tax Rate](https://docs.stripe.com/api/tax_rates) ids."]
    pub default_tax_rates: Option<json::OwnedValue>,
    #[doc = "Subscription description, meant to be displayable to the customer."]
    pub description: Option<json::OwnedValue>,
    #[doc = "The coupons to redeem into discounts for the schedule phase."]
    pub discounts: Option<json::OwnedValue>,
    #[doc = "The number of intervals the phase should last."]
    pub duration: Option<stripe_types_08::subscription_schedules::CreateSubscriptionScheduleParamsPhasesItemDuration>,
    #[doc = "The date at which this phase of the subscription schedule ends."]
    pub end_date: Option<i64>,
    #[doc = "All invoices will be billed using the specified settings."]
    pub invoice_settings:
        Option<stripe_types_11::subscription_schedules::CreateSubscriptionScheduleParamsPhasesItemInvoiceSettings>,
    #[doc = "List of configuration items, each with an attached price, to apply during this phase of the subscription schedule."]
    pub items: Vec<stripe_types_12::subscription_schedules::CreateSubscriptionScheduleParamsPhasesItemItemsItem>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to a phase."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The account on behalf of which to charge, for each of the associated subscription's invoices."]
    pub on_behalf_of: Option<String>,
    #[doc = "Controls whether the subscription schedule should create prorations..."]
    pub proration_behavior:
        Option<stripe_types_05::subscription_schedules::CreateSubscriptionScheduleParamsPhasesItemProrationBehavior>,
    #[doc = "The data with which to automatically create a Transfer for each of the associated subscription's invoices."]
    pub transfer_data:
        Option<stripe_types_06::subscription_schedules::CreateSubscriptionScheduleParamsPhasesItemTransferData>,
    #[doc = "If set to true the entire phase is counted as a trial and the customer will not be charged for any fees."]
    pub trial: Option<bool>,
    #[doc = "Sets the phase to trialing from the start date to this date."]
    pub trial_end: Option<i64>,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsPhasesItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.add_invoice_items.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("add_invoice_items", index, item);
            }
        }
        form.field_opt("application_fee_percent", self.application_fee_percent.as_ref());
        form.field_opt("automatic_tax", self.automatic_tax.as_ref());
        form.field_opt("billing_cycle_anchor", self.billing_cycle_anchor.as_ref());
        form.field_opt("billing_thresholds", self.billing_thresholds.as_ref());
        form.field_opt("collection_method", self.collection_method.as_ref());
        form.field_opt("currency", self.currency.as_ref());
        form.field_opt("default_payment_method", self.default_payment_method.as_ref());
        form.field_opt("duration", self.duration.as_ref());
        form.field_opt("end_date", self.end_date.as_ref());
        form.field_opt("invoice_settings", self.invoice_settings.as_ref());
        for (index, item) in self.items.iter().enumerate() {
            form.field_indexed("items", index, item);
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("on_behalf_of", self.on_behalf_of.as_ref());
        form.field_opt("proration_behavior", self.proration_behavior.as_ref());
        form.field_opt("transfer_data", self.transfer_data.as_ref());
        form.field_opt("trial", self.trial.as_ref());
        form.field_opt("trial_end", self.trial_end.as_ref());
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
#[derive(Debug, Clone)]
pub enum SubscriptionScheduleConfigurationItemPlanExpanded {
    /// Expanded `plan`.
    Plan(Box<crate::plans::Plan>),
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
impl<'json> json::Decode<'json> for SubscriptionScheduleConfigurationItemPlanExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "plan" => SubscriptionScheduleConfigurationItemPlanExpanded::Plan(Box::new(json::from_slice_owned(
                &json::to_vec(&value),
            )?)),
            _ => SubscriptionScheduleConfigurationItemPlanExpanded::Other(value),
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
pub struct SubscriptionSchedulesResourcePauseSchedule {
    #[doc = "A unique identifier for this pause schedule."]
    #[json(default)]
    pub key: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pause: Option<stripe_types_12::subscription_schedules::SubscriptionSchedulesResourcePauseSchedulePause>,
    #[doc = "Details about when and how the subscription resumes."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub resume: Option<stripe_types_07::subscription_schedules::SubscriptionSchedulesResourcePauseScheduleResume>,
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSubscriptionScheduleParams {
    #[doc = "Object representing the subscription schedule's default settings."]
    pub default_settings:
        Option<stripe_types_12::subscription_schedules::UpdateSubscriptionScheduleParamsDefaultSettings>,
    #[doc = "Behavior of the subscription schedule and underlying subscription when it ends."]
    pub end_behavior: Option<stripe_types_05::subscription_schedules::UpdateSubscriptionScheduleParamsEndBehavior>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Configures the subscription's pause behavior and, optionally, its resume behavior."]
    pub pause_schedules: Option<json::OwnedValue>,
    #[doc = "List representing phases of the subscription schedule."]
    pub phases: Option<Vec<crate::subscription_schedules::UpdateSubscriptionScheduleParamsPhasesItem>>,
    #[doc = "If the update changes the billing configuration (item price, quantity, etc.) of the current phase, indicates how pror..."]
    pub proration_behavior:
        Option<stripe_types_05::subscription_schedules::UpdateSubscriptionScheduleParamsProrationBehavior>,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for UpdateSubscriptionScheduleParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("default_settings", self.default_settings.as_ref());
        form.field_opt("end_behavior", self.end_behavior.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        if let Some(values) = self.phases.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("phases", index, item);
            }
        }
        form.field_opt("proration_behavior", self.proration_behavior.as_ref());
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSubscriptionScheduleParamsPhasesItem {
    #[doc = "A list of prices and quantities that will generate invoice items appended to the next invoice for this phase."]
    pub add_invoice_items: Option<
        Vec<stripe_types_12::subscription_schedules::UpdateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItem>,
    >,
    #[doc = "A non-negative decimal between 0 and 100, with at most two decimal places."]
    pub application_fee_percent: Option<f64>,
    #[doc = "Automatic tax settings for this phase."]
    pub automatic_tax:
        Option<stripe_types_10::subscription_schedules::UpdateSubscriptionScheduleParamsPhasesItemAutomaticTax>,
    #[doc = "Can be set to `phase_start` to set the anchor to the start of the phase or `automatic` to automatically change it if ..."]
    pub billing_cycle_anchor:
        Option<stripe_types_06::subscription_schedules::UpdateSubscriptionScheduleParamsPhasesItemBillingCycleAnchor>,
    #[doc = "Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period."]
    pub billing_thresholds:
        Option<stripe_types_06::subscription_schedules::UpdateSubscriptionScheduleParamsPhasesItemBillingThresholds>,
    #[doc = "Either `charge_automatically`, or `send_invoice`."]
    pub collection_method:
        Option<stripe_types_06::subscription_schedules::UpdateSubscriptionScheduleParamsPhasesItemCollectionMethod>,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: Option<String>,
    #[doc = "ID of the default payment method for the subscription schedule."]
    pub default_payment_method: Option<String>,
    #[doc = "A list of [Tax Rate](https://docs.stripe.com/api/tax_rates) ids."]
    pub default_tax_rates: Option<json::OwnedValue>,
    #[doc = "Subscription description, meant to be displayable to the customer."]
    pub description: Option<json::OwnedValue>,
    #[doc = "The coupons to redeem into discounts for the schedule phase."]
    pub discounts: Option<json::OwnedValue>,
    #[doc = "The number of intervals the phase should last."]
    pub duration: Option<stripe_types_09::subscription_schedules::UpdateSubscriptionScheduleParamsPhasesItemDuration>,
    #[doc = "The date at which this phase of the subscription schedule ends."]
    pub end_date: Option<json::OwnedValue>,
    #[doc = "All invoices will be billed using the specified settings."]
    pub invoice_settings:
        Option<stripe_types_10::subscription_schedules::UpdateSubscriptionScheduleParamsPhasesItemInvoiceSettings>,
    #[doc = "List of configuration items, each with an attached price, to apply during this phase of the subscription schedule."]
    pub items: Vec<stripe_types_12::subscription_schedules::UpdateSubscriptionScheduleParamsPhasesItemItemsItem>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to a phase."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The account on behalf of which to charge, for each of the associated subscription's invoices."]
    pub on_behalf_of: Option<String>,
    #[doc = "Controls whether the subscription schedule should create prorations..."]
    pub proration_behavior:
        Option<stripe_types_06::subscription_schedules::UpdateSubscriptionScheduleParamsPhasesItemProrationBehavior>,
    #[doc = "The date at which this phase of the subscription schedule starts or `now`."]
    pub start_date: Option<json::OwnedValue>,
    #[doc = "The data with which to automatically create a Transfer for each of the associated subscription's invoices."]
    pub transfer_data:
        Option<stripe_types_05::subscription_schedules::UpdateSubscriptionScheduleParamsPhasesItemTransferData>,
    #[doc = "If set to true the entire phase is counted as a trial and the customer will not be charged for any fees."]
    pub trial: Option<bool>,
    #[doc = "Sets the phase to trialing from the start date to this date."]
    pub trial_end: Option<json::OwnedValue>,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for UpdateSubscriptionScheduleParamsPhasesItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.add_invoice_items.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("add_invoice_items", index, item);
            }
        }
        form.field_opt("application_fee_percent", self.application_fee_percent.as_ref());
        form.field_opt("automatic_tax", self.automatic_tax.as_ref());
        form.field_opt("billing_cycle_anchor", self.billing_cycle_anchor.as_ref());
        form.field_opt("billing_thresholds", self.billing_thresholds.as_ref());
        form.field_opt("collection_method", self.collection_method.as_ref());
        form.field_opt("currency", self.currency.as_ref());
        form.field_opt("default_payment_method", self.default_payment_method.as_ref());
        form.field_opt("duration", self.duration.as_ref());
        form.field_opt("invoice_settings", self.invoice_settings.as_ref());
        for (index, item) in self.items.iter().enumerate() {
            form.field_indexed("items", index, item);
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("on_behalf_of", self.on_behalf_of.as_ref());
        form.field_opt("proration_behavior", self.proration_behavior.as_ref());
        form.field_opt("transfer_data", self.transfer_data.as_ref());
        form.field_opt("trial", self.trial.as_ref());
    }
}
