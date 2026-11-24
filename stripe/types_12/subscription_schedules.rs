#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionScheduleParamsDefaultSettings {
    #[doc = "A non-negative decimal between 0 and 100, with at most two decimal places."]
    pub application_fee_percent: Option<f64>,
    #[doc = "Default settings for automatic tax computation."]
    pub automatic_tax:
        Option<stripe_types_08::subscription_schedules::CreateSubscriptionScheduleParamsDefaultSettingsAutomaticTax>,
    #[doc = "Can be set to `phase_start` to set the anchor to the start of the phase or `automatic` to automatically change it if ..."]
    pub billing_cycle_anchor: Option<
        stripe_types_04::subscription_schedules::CreateSubscriptionScheduleParamsDefaultSettingsBillingCycleAnchor,
    >,
    #[doc = "Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period."]
    pub billing_thresholds: Option<
        stripe_types_01::subscription_schedules::CreateSubscriptionScheduleParamsDefaultSettingsBillingThresholds,
    >,
    #[doc = "Either `charge_automatically`, or `send_invoice`."]
    pub collection_method: Option<
        stripe_types_01::subscription_schedules::CreateSubscriptionScheduleParamsDefaultSettingsCollectionMethod,
    >,
    #[doc = "ID of the default payment method for the subscription schedule."]
    pub default_payment_method: Option<String>,
    #[doc = "Subscription description, meant to be displayable to the customer."]
    pub description: Option<json::OwnedValue>,
    #[doc = "All invoices will be billed using the specified settings."]
    pub invoice_settings:
        Option<stripe_types_10::subscription_schedules::CreateSubscriptionScheduleParamsDefaultSettingsInvoiceSettings>,
    #[doc = "The account on behalf of which to charge, for each of the associated subscription's invoices."]
    pub on_behalf_of: Option<json::OwnedValue>,
    #[doc = "The data with which to automatically create a Transfer for each of the associated subscription's invoices."]
    pub transfer_data:
        Option<stripe_types_01::subscription_schedules::CreateSubscriptionScheduleParamsDefaultSettingsTransferData>,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsDefaultSettings {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("application_fee_percent", self.application_fee_percent.as_ref());
        form.field_opt("automatic_tax", self.automatic_tax.as_ref());
        form.field_opt("billing_cycle_anchor", self.billing_cycle_anchor.as_ref());
        form.field_opt("billing_thresholds", self.billing_thresholds.as_ref());
        form.field_opt("collection_method", self.collection_method.as_ref());
        form.field_opt("default_payment_method", self.default_payment_method.as_ref());
        form.field_opt("invoice_settings", self.invoice_settings.as_ref());
        form.field_opt("transfer_data", self.transfer_data.as_ref());
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone)]
pub struct CreateSubscriptionScheduleParamsPauseSchedulesItemPause {
    #[doc = "When to pause the subscription."]
    pub pause_at:
        stripe_types_05::subscription_schedules::CreateSubscriptionScheduleParamsPauseSchedulesItemPausePauseAt,
    #[doc = "Settings controlling billing behavior during the pause."]
    pub settings: Option<
        stripe_types_11::subscription_schedules::CreateSubscriptionScheduleParamsPauseSchedulesItemPauseSettings,
    >,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsPauseSchedulesItemPause {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("pause_at", &self.pause_at);
        form.field_opt("settings", self.settings.as_ref());
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionScheduleParamsPhasesItemItemsItem {
    #[doc = "Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period."]
    pub billing_thresholds: Option<
        stripe_types_00::subscription_schedules::CreateSubscriptionScheduleParamsPhasesItemItemsItemBillingThresholds,
    >,
    #[doc = "The coupons to redeem into discounts for the subscription item."]
    pub discounts: Option<json::OwnedValue>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to a configuration item."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The plan ID to subscribe to."]
    pub plan: Option<String>,
    #[doc = "The ID of the price object."]
    pub price: Option<String>,
    #[doc = "Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline."]
    pub price_data:
        Option<stripe_types_11::subscription_schedules::CreateSubscriptionScheduleParamsPhasesItemItemsItemPriceData>,
    #[doc = "Quantity for the given price."]
    pub quantity: Option<i64>,
    #[doc = "A list of [Tax Rate](https://docs.stripe.com/api/tax_rates) ids."]
    pub tax_rates: Option<json::OwnedValue>,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for CreateSubscriptionScheduleParamsPhasesItemItemsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("billing_thresholds", self.billing_thresholds.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("plan", self.plan.as_ref());
        form.field_opt("price", self.price.as_ref());
        form.field_opt("price_data", self.price_data.as_ref());
        form.field_opt("quantity", self.quantity.as_ref());
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
pub enum SubscriptionScheduleAddInvoiceItemPriceExpanded {
    /// Expanded `price`.
    Price(Box<stripe_types_11::prices::Price>),
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
impl<'json> json::Decode<'json> for SubscriptionScheduleAddInvoiceItemPriceExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "price" => SubscriptionScheduleAddInvoiceItemPriceExpanded::Price(Box::new(json::from_slice_owned(
                &json::to_vec(&value),
            )?)),
            _ => SubscriptionScheduleAddInvoiceItemPriceExpanded::Other(value),
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
pub struct SubscriptionSchedulesResourcePauseSchedulePause {
    #[doc = "Time at which the subscription pauses."]
    #[json(default)]
    pub pause_at: i64,
    #[doc = "Settings controlling billing behavior during the pause."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub settings:
        Option<stripe_types_10::subscription_schedules::SubscriptionSchedulesResourcePauseSchedulePauseSettings>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<stripe_types_05::subscription_schedules::SubscriptionSchedulesResourcePauseSchedulePauseStatus>,
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSubscriptionScheduleParamsDefaultSettings {
    #[doc = "A non-negative decimal between 0 and 100, with at most two decimal places."]
    pub application_fee_percent: Option<f64>,
    #[doc = "Default settings for automatic tax computation."]
    pub automatic_tax:
        Option<stripe_types_11::subscription_schedules::UpdateSubscriptionScheduleParamsDefaultSettingsAutomaticTax>,
    #[doc = "Can be set to `phase_start` to set the anchor to the start of the phase or `automatic` to automatically change it if ..."]
    pub billing_cycle_anchor: Option<
        stripe_types_06::subscription_schedules::UpdateSubscriptionScheduleParamsDefaultSettingsBillingCycleAnchor,
    >,
    #[doc = "Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period."]
    pub billing_thresholds: Option<
        stripe_types_05::subscription_schedules::UpdateSubscriptionScheduleParamsDefaultSettingsBillingThresholds,
    >,
    #[doc = "Either `charge_automatically`, or `send_invoice`."]
    pub collection_method: Option<
        stripe_types_06::subscription_schedules::UpdateSubscriptionScheduleParamsDefaultSettingsCollectionMethod,
    >,
    #[doc = "ID of the default payment method for the subscription schedule."]
    pub default_payment_method: Option<String>,
    #[doc = "Subscription description, meant to be displayable to the customer."]
    pub description: Option<json::OwnedValue>,
    #[doc = "All invoices will be billed using the specified settings."]
    pub invoice_settings:
        Option<stripe_types_11::subscription_schedules::UpdateSubscriptionScheduleParamsDefaultSettingsInvoiceSettings>,
    #[doc = "The account on behalf of which to charge, for each of the associated subscription's invoices."]
    pub on_behalf_of: Option<json::OwnedValue>,
    #[doc = "The data with which to automatically create a Transfer for each of the associated subscription's invoices."]
    pub transfer_data:
        Option<stripe_types_05::subscription_schedules::UpdateSubscriptionScheduleParamsDefaultSettingsTransferData>,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for UpdateSubscriptionScheduleParamsDefaultSettings {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("application_fee_percent", self.application_fee_percent.as_ref());
        form.field_opt("automatic_tax", self.automatic_tax.as_ref());
        form.field_opt("billing_cycle_anchor", self.billing_cycle_anchor.as_ref());
        form.field_opt("billing_thresholds", self.billing_thresholds.as_ref());
        form.field_opt("collection_method", self.collection_method.as_ref());
        form.field_opt("default_payment_method", self.default_payment_method.as_ref());
        form.field_opt("invoice_settings", self.invoice_settings.as_ref());
        form.field_opt("transfer_data", self.transfer_data.as_ref());
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItem {
    #[doc = "Controls whether discounts apply to this invoice item."]
    pub discountable: Option<bool>,
    #[doc = "The coupons to redeem into discounts for the item."]
    pub discounts: Option<Vec<stripe_types_06::subscription_schedules::UpdateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemDiscountsItem>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The period associated with this invoice item."]
    pub period: Option<stripe_types_11::subscription_schedules::UpdateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPeriod>,
    #[doc = "The ID of the price object."]
    pub price: Option<String>,
    #[doc = "Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline."]
    pub price_data: Option<stripe_types_09::subscription_schedules::UpdateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItemPriceData>,
    #[doc = "Quantity for this item."]
    pub quantity: Option<i64>,
    #[doc = "The tax rates which apply to the item."]
    pub tax_rates: Option<json::OwnedValue>,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for UpdateSubscriptionScheduleParamsPhasesItemAddInvoiceItemsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("discountable", self.discountable.as_ref());
        if let Some(values) = self.discounts.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("discounts", index, item);
            }
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("period", self.period.as_ref());
        form.field_opt("price", self.price.as_ref());
        form.field_opt("price_data", self.price_data.as_ref());
        form.field_opt("quantity", self.quantity.as_ref());
    }
}

#[cfg(feature = "subscription_schedules")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSubscriptionScheduleParamsPhasesItemItemsItem {
    #[doc = "Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period."]
    pub billing_thresholds: Option<
        stripe_types_05::subscription_schedules::UpdateSubscriptionScheduleParamsPhasesItemItemsItemBillingThresholds,
    >,
    #[doc = "The coupons to redeem into discounts for the subscription item."]
    pub discounts: Option<json::OwnedValue>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to a configuration item."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The plan ID to subscribe to."]
    pub plan: Option<String>,
    #[doc = "The ID of the price object."]
    pub price: Option<String>,
    #[doc = "Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline."]
    pub price_data:
        Option<stripe_types_10::subscription_schedules::UpdateSubscriptionScheduleParamsPhasesItemItemsItemPriceData>,
    #[doc = "Quantity for the given price."]
    pub quantity: Option<i64>,
    #[doc = "A list of [Tax Rate](https://docs.stripe.com/api/tax_rates) ids."]
    pub tax_rates: Option<json::OwnedValue>,
}

#[cfg(feature = "subscription_schedules")]
impl crate::params::FormEncode for UpdateSubscriptionScheduleParamsPhasesItemItemsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("billing_thresholds", self.billing_thresholds.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("plan", self.plan.as_ref());
        form.field_opt("price", self.price.as_ref());
        form.field_opt("price_data", self.price_data.as_ref());
        form.field_opt("quantity", self.quantity.as_ref());
    }
}
