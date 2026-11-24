#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreateInvoiceParams {
    #[doc = "The account tax IDs associated with the invoice."]
    pub account_tax_ids: Option<json::OwnedValue>,
    #[doc = "A fee in cents (or local equivalent) that will be applied to the invoice and transferred to the application owner's S..."]
    pub application_fee_amount: Option<i64>,
    #[doc = "Controls whether Stripe performs automatic collection..."]
    pub auto_advance: Option<bool>,
    #[doc = "Settings for automatic tax lookup for this invoice."]
    pub automatic_tax: Option<stripe_types_07::invoices::CreateInvoiceParamsAutomaticTax>,
    #[doc = "The time when this invoice should be scheduled to finalize (up to 5 years in the future)."]
    pub automatically_finalizes_at: Option<i64>,
    #[doc = "Either `charge_automatically`, or `send_invoice`."]
    pub collection_method: Option<stripe_types_06::invoices::CreateInvoiceParamsCollectionMethod>,
    #[doc = "The currency to create this invoice in."]
    pub currency: Option<String>,
    #[doc = "A list of up to 4 custom fields to be displayed on the invoice."]
    pub custom_fields: Option<json::OwnedValue>,
    #[doc = "The ID of the customer to bill."]
    pub customer: Option<String>,
    #[doc = "The ID of the account to bill."]
    pub customer_account: Option<String>,
    #[doc = "The number of days from when the invoice is created until it is due."]
    pub days_until_due: Option<i64>,
    #[doc = "ID of the default payment method for the invoice."]
    pub default_payment_method: Option<String>,
    #[doc = "ID of the default payment source for the invoice."]
    pub default_source: Option<String>,
    #[doc = "The tax rates that will apply to any line item that does not have `tax_rates` set."]
    pub default_tax_rates: Option<Vec<String>>,
    #[doc = "An arbitrary string attached to the object."]
    pub description: Option<String>,
    #[doc = "The coupons and promotion codes to redeem into discounts for the invoice."]
    pub discounts: Option<json::OwnedValue>,
    #[doc = "The date on which payment for this invoice is due."]
    pub due_date: Option<i64>,
    #[doc = "The date when this invoice is in effect."]
    pub effective_at: Option<i64>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Footer to be displayed on the invoice."]
    pub footer: Option<String>,
    #[doc = "Revise an existing invoice."]
    pub from_invoice: Option<stripe_types_06::invoices::CreateInvoiceParamsFromInvoice>,
    #[doc = "The connected account that issues the invoice."]
    pub issuer: Option<stripe_types_07::invoices::CreateInvoiceParamsIssuer>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Set the number for this invoice."]
    pub number: Option<String>,
    #[doc = "The account (if any) for which the funds of the invoice payment are intended."]
    pub on_behalf_of: Option<String>,
    #[doc = "Configuration settings for the PaymentIntent that is generated when the invoice is finalized."]
    pub payment_settings: Option<crate::invoices::CreateInvoiceParamsPaymentSettings>,
    #[doc = "How to handle pending invoice items on invoice creation."]
    pub pending_invoice_items_behavior:
        Option<stripe_types_00::invoices::CreateInvoiceParamsPendingInvoiceItemsBehavior>,
    #[doc = "The rendering-related settings that control how invoices render in customer-facing interfaces such as the PDF or host..."]
    pub rendering: Option<stripe_types_09::invoices::CreateInvoiceParamsRendering>,
    #[doc = "Settings for the cost of shipping for this invoice."]
    pub shipping_cost: Option<crate::invoices::CreateInvoiceParamsShippingCost>,
    #[doc = "Shipping details for the invoice."]
    pub shipping_details: Option<stripe_types_09::invoices::CreateInvoiceParamsShippingDetails>,
    #[doc = "Extra information about a charge for the customer's credit card statement."]
    pub statement_descriptor: Option<String>,
    #[doc = "The ID of the subscription to invoice, if any."]
    pub subscription: Option<String>,
    #[doc = "If specified, the funds from the invoice will be transferred to the destination and the ID of the resulting transfer ..."]
    pub transfer_data: Option<stripe_types_02::invoices::CreateInvoiceParamsTransferData>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("application_fee_amount", self.application_fee_amount.as_ref());
        form.field_opt("auto_advance", self.auto_advance.as_ref());
        form.field_opt("automatic_tax", self.automatic_tax.as_ref());
        form.field_opt("automatically_finalizes_at", self.automatically_finalizes_at.as_ref());
        form.field_opt("collection_method", self.collection_method.as_ref());
        form.field_opt("currency", self.currency.as_ref());
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("customer_account", self.customer_account.as_ref());
        form.field_opt("days_until_due", self.days_until_due.as_ref());
        form.field_opt("default_payment_method", self.default_payment_method.as_ref());
        form.field_opt("default_source", self.default_source.as_ref());
        if let Some(values) = self.default_tax_rates.as_ref() {
            form.field_array("default_tax_rates", values.as_slice());
        }
        form.field_opt("description", self.description.as_ref());
        form.field_opt("due_date", self.due_date.as_ref());
        form.field_opt("effective_at", self.effective_at.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("footer", self.footer.as_ref());
        form.field_opt("from_invoice", self.from_invoice.as_ref());
        form.field_opt("issuer", self.issuer.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("number", self.number.as_ref());
        form.field_opt("on_behalf_of", self.on_behalf_of.as_ref());
        form.field_opt("payment_settings", self.payment_settings.as_ref());
        form.field_opt("pending_invoice_items_behavior", self.pending_invoice_items_behavior.as_ref());
        form.field_opt("rendering", self.rendering.as_ref());
        form.field_opt("shipping_cost", self.shipping_cost.as_ref());
        form.field_opt("shipping_details", self.shipping_details.as_ref());
        form.field_opt("statement_descriptor", self.statement_descriptor.as_ref());
        form.field_opt("subscription", self.subscription.as_ref());
        form.field_opt("transfer_data", self.transfer_data.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreateInvoiceParamsPaymentSettings {
    #[doc = "ID of the mandate to be used for this invoice."]
    pub default_mandate: Option<json::OwnedValue>,
    #[doc = "Payment-method-specific configuration to provide to the invoice’s PaymentIntent."]
    pub payment_method_options:
        Option<stripe_types_12::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptions>,
    #[doc = "The list of payment method types (e.g."]
    pub payment_method_types: Option<json::OwnedValue>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsPaymentSettings {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("payment_method_options", self.payment_method_options.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreateInvoiceParamsShippingCost {
    #[doc = "The ID of the shipping rate to use for this order."]
    pub shipping_rate: Option<String>,
    #[doc = "Parameters to create a new ad-hoc shipping rate for this order."]
    pub shipping_rate_data: Option<stripe_types_12::invoices::CreateInvoiceParamsShippingCostShippingRateData>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsShippingCost {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("shipping_rate", self.shipping_rate.as_ref());
        form.field_opt("shipping_rate_data", self.shipping_rate_data.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreatePreviewParams {
    #[doc = "Settings for automatic tax lookup for this invoice preview."]
    pub automatic_tax: Option<stripe_types_10::invoices::CreatePreviewParamsAutomaticTax>,
    #[doc = "The currency to preview this invoice in."]
    pub currency: Option<String>,
    #[doc = "The identifier of the customer whose upcoming invoice you're retrieving."]
    pub customer: Option<String>,
    #[doc = "The identifier of the account representing the customer whose upcoming invoice you're retrieving."]
    pub customer_account: Option<String>,
    #[doc = "Details about the customer you want to invoice or overrides for an existing customer."]
    pub customer_details: Option<stripe_types_11::invoices::CreatePreviewParamsCustomerDetails>,
    #[doc = "The coupons to redeem into discounts for the invoice preview."]
    pub discounts: Option<json::OwnedValue>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "List of invoice items to add or update in the upcoming invoice preview (up to 250)."]
    pub invoice_items: Option<Vec<stripe_types_11::invoices::CreatePreviewParamsInvoiceItemsItem>>,
    #[doc = "The connected account that issues the invoice."]
    pub issuer: Option<stripe_types_09::invoices::CreatePreviewParamsIssuer>,
    #[doc = "The account (if any) for which the funds of the invoice payment are intended."]
    pub on_behalf_of: Option<json::OwnedValue>,
    #[doc = "Customizes the types of values to include when calculating the invoice."]
    pub preview_mode: Option<stripe_types_00::invoices::CreatePreviewParamsPreviewMode>,
    #[doc = "The identifier of the schedule whose upcoming invoice you'd like to retrieve."]
    pub schedule: Option<String>,
    #[doc = "The schedule creation or modification params to apply as a preview."]
    pub schedule_details: Option<crate::invoices::CreatePreviewParamsScheduleDetails>,
    #[doc = "The identifier of the subscription for which you'd like to retrieve the upcoming invoice."]
    pub subscription: Option<String>,
    #[doc = "The subscription creation or modification params to apply as a preview."]
    pub subscription_details: Option<crate::invoices::CreatePreviewParamsSubscriptionDetails>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("automatic_tax", self.automatic_tax.as_ref());
        form.field_opt("currency", self.currency.as_ref());
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("customer_account", self.customer_account.as_ref());
        form.field_opt("customer_details", self.customer_details.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.invoice_items.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("invoice_items", index, item);
            }
        }
        form.field_opt("issuer", self.issuer.as_ref());
        form.field_opt("preview_mode", self.preview_mode.as_ref());
        form.field_opt("schedule", self.schedule.as_ref());
        form.field_opt("schedule_details", self.schedule_details.as_ref());
        form.field_opt("subscription", self.subscription.as_ref());
        form.field_opt("subscription_details", self.subscription_details.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreatePreviewParamsScheduleDetails {
    #[doc = "Controls how prorations and invoices for subscriptions are calculated and orchestrated."]
    pub billing_mode: Option<stripe_types_07::invoices::CreatePreviewParamsScheduleDetailsBillingMode>,
    #[doc = "Behavior of the subscription schedule and underlying subscription when it ends."]
    pub end_behavior: Option<stripe_types_03::invoices::CreatePreviewParamsScheduleDetailsEndBehavior>,
    #[doc = "List representing phases of the subscription schedule."]
    pub phases: Option<Vec<crate::invoices::CreatePreviewParamsScheduleDetailsPhasesItem>>,
    #[doc = "In cases where the `schedule_details` params update the currently active phase, specifies if and how to prorate at th..."]
    pub proration_behavior: Option<stripe_types_06::invoices::CreatePreviewParamsScheduleDetailsProrationBehavior>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsScheduleDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("billing_mode", self.billing_mode.as_ref());
        form.field_opt("end_behavior", self.end_behavior.as_ref());
        if let Some(values) = self.phases.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("phases", index, item);
            }
        }
        form.field_opt("proration_behavior", self.proration_behavior.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreatePreviewParamsScheduleDetailsPhasesItem {
    #[doc = "A list of prices and quantities that will generate invoice items appended to the next invoice for this phase."]
    pub add_invoice_items:
        Option<Vec<stripe_types_10::invoices::CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItem>>,
    #[doc = "A non-negative decimal between 0 and 100, with at most two decimal places."]
    pub application_fee_percent: Option<f64>,
    #[doc = "Automatic tax settings for this phase."]
    pub automatic_tax: Option<stripe_types_10::invoices::CreatePreviewParamsScheduleDetailsPhasesItemAutomaticTax>,
    #[doc = "Can be set to `phase_start` to set the anchor to the start of the phase or `automatic` to automatically change it if ..."]
    pub billing_cycle_anchor:
        Option<stripe_types_02::invoices::CreatePreviewParamsScheduleDetailsPhasesItemBillingCycleAnchor>,
    #[doc = "Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period."]
    pub billing_thresholds:
        Option<stripe_types_00::invoices::CreatePreviewParamsScheduleDetailsPhasesItemBillingThresholds>,
    #[doc = "Either `charge_automatically`, or `send_invoice`."]
    pub collection_method:
        Option<stripe_types_05::invoices::CreatePreviewParamsScheduleDetailsPhasesItemCollectionMethod>,
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
    pub duration: Option<stripe_types_06::invoices::CreatePreviewParamsScheduleDetailsPhasesItemDuration>,
    #[doc = "The date at which this phase of the subscription schedule ends."]
    pub end_date: Option<json::OwnedValue>,
    #[doc = "All invoices will be billed using the specified settings."]
    pub invoice_settings:
        Option<stripe_types_10::invoices::CreatePreviewParamsScheduleDetailsPhasesItemInvoiceSettings>,
    #[doc = "List of configuration items, each with an attached price, to apply during this phase of the subscription schedule."]
    pub items: Vec<stripe_types_12::invoices::CreatePreviewParamsScheduleDetailsPhasesItemItemsItem>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to a phase."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The account on behalf of which to charge, for each of the associated subscription's invoices."]
    pub on_behalf_of: Option<String>,
    #[doc = "Controls whether the subscription schedule should create prorations..."]
    pub proration_behavior:
        Option<stripe_types_04::invoices::CreatePreviewParamsScheduleDetailsPhasesItemProrationBehavior>,
    #[doc = "The date at which this phase of the subscription schedule starts or `now`."]
    pub start_date: Option<json::OwnedValue>,
    #[doc = "The data with which to automatically create a Transfer for each of the associated subscription's invoices."]
    pub transfer_data: Option<stripe_types_00::invoices::CreatePreviewParamsScheduleDetailsPhasesItemTransferData>,
    #[doc = "If set to true the entire phase is counted as a trial and the customer will not be charged for any fees."]
    pub trial: Option<bool>,
    #[doc = "Sets the phase to trialing from the start date to this date."]
    pub trial_end: Option<json::OwnedValue>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsScheduleDetailsPhasesItem {
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

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreatePreviewParamsSubscriptionDetails {
    #[doc = "For new subscriptions, a future timestamp to anchor the subscription's billing cycle..."]
    pub billing_cycle_anchor:
        Option<stripe_types_05::invoices::CreatePreviewParamsSubscriptionDetailsBillingCycleAnchor>,
    #[doc = "Controls how prorations and invoices for subscriptions are calculated and orchestrated."]
    pub billing_mode: Option<stripe_types_11::invoices::CreatePreviewParamsSubscriptionDetailsBillingMode>,
    #[doc = "Sets the billing schedules for the subscription."]
    pub billing_schedules: Option<json::OwnedValue>,
    #[doc = "A timestamp at which the subscription should cancel."]
    pub cancel_at: Option<json::OwnedValue>,
    #[doc = "Indicate whether this subscription should cancel at the end of the current period (`current_period_end`)."]
    pub cancel_at_period_end: Option<bool>,
    #[doc = "This simulates the subscription being canceled or expired immediately."]
    pub cancel_now: Option<bool>,
    #[doc = "If provided, the invoice returned will preview updating or creating a subscription with these default tax rates."]
    pub default_tax_rates: Option<json::OwnedValue>,
    #[doc = "A list of up to 20 subscription items, each with an attached price."]
    pub items: Option<Vec<stripe_types_12::invoices::CreatePreviewParamsSubscriptionDetailsItemsItem>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Previews the invoice that would be generated when pausing the subscription."]
    pub pause: Option<stripe_types_11::invoices::CreatePreviewParamsSubscriptionDetailsPause>,
    #[doc = "Determines how to handle [prorations](https://docs.stripe.com/billing/subscriptions/prorations) when the billing cycl..."]
    pub proration_behavior: Option<stripe_types_05::invoices::CreatePreviewParamsSubscriptionDetailsProrationBehavior>,
    #[doc = "If previewing an update to a subscription, and doing proration, `subscription_details.proration_date` forces the pror..."]
    pub proration_date: Option<i64>,
    #[doc = "For paused subscriptions, setting `subscription_details.resume_at` to `now` will preview the invoice that will be gen..."]
    pub resume_at: Option<stripe_types_03::invoices::CreatePreviewParamsSubscriptionDetailsResumeAt>,
    #[doc = "Date a subscription is intended to start (can be future or past)."]
    pub start_date: Option<i64>,
    #[doc = "If provided, the invoice returned will preview updating or creating a subscription with that trial end."]
    pub trial_end: Option<json::OwnedValue>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsSubscriptionDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("billing_cycle_anchor", self.billing_cycle_anchor.as_ref());
        form.field_opt("billing_mode", self.billing_mode.as_ref());
        form.field_opt("cancel_at_period_end", self.cancel_at_period_end.as_ref());
        form.field_opt("cancel_now", self.cancel_now.as_ref());
        if let Some(values) = self.items.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("items", index, item);
            }
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("pause", self.pause.as_ref());
        form.field_opt("proration_behavior", self.proration_behavior.as_ref());
        form.field_opt("proration_date", self.proration_date.as_ref());
        form.field_opt("resume_at", self.resume_at.as_ref());
        form.field_opt("start_date", self.start_date.as_ref());
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
pub struct InvoicesPaymentSettings {
    #[doc = "ID of the mandate to be used for this invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub default_mandate: Option<String>,
    #[doc = "Payment-method-specific configuration to provide to the invoice’s PaymentIntent."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<stripe_types_12::invoices::InvoicesPaymentMethodOptions>,
    #[doc = "The list of payment method types (e.g."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<stripe_types_02::invoices::InvoicesPaymentSettingsPaymentMethodTypesItem>>,
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct UpdateInvoiceParams {
    #[doc = "The account tax IDs associated with the invoice."]
    pub account_tax_ids: Option<json::OwnedValue>,
    #[doc = "A fee in cents (or local equivalent) that will be applied to the invoice and transferred to the application owner's S..."]
    pub application_fee_amount: Option<i64>,
    #[doc = "Controls whether Stripe performs automatic collection..."]
    pub auto_advance: Option<bool>,
    #[doc = "Settings for automatic tax lookup for this invoice."]
    pub automatic_tax: Option<stripe_types_11::invoices::UpdateInvoiceParamsAutomaticTax>,
    #[doc = "The time when this invoice should be scheduled to finalize (up to 5 years in the future)."]
    pub automatically_finalizes_at: Option<i64>,
    #[doc = "Either `charge_automatically` or `send_invoice`."]
    pub collection_method: Option<stripe_types_06::invoices::UpdateInvoiceParamsCollectionMethod>,
    #[doc = "A list of up to 4 custom fields to be displayed on the invoice."]
    pub custom_fields: Option<json::OwnedValue>,
    #[doc = "The number of days from which the invoice is created until it is due."]
    pub days_until_due: Option<i64>,
    #[doc = "ID of the default payment method for the invoice."]
    pub default_payment_method: Option<String>,
    #[doc = "ID of the default payment source for the invoice."]
    pub default_source: Option<json::OwnedValue>,
    #[doc = "The tax rates that will apply to any line item that does not have `tax_rates` set."]
    pub default_tax_rates: Option<json::OwnedValue>,
    #[doc = "An arbitrary string attached to the object."]
    pub description: Option<String>,
    #[doc = "The discounts that will apply to the invoice."]
    pub discounts: Option<json::OwnedValue>,
    #[doc = "The date on which payment for this invoice is due."]
    pub due_date: Option<i64>,
    #[doc = "The date when this invoice is in effect."]
    pub effective_at: Option<json::OwnedValue>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Footer to be displayed on the invoice."]
    pub footer: Option<String>,
    #[doc = "The connected account that issues the invoice."]
    pub issuer: Option<stripe_types_08::invoices::UpdateInvoiceParamsIssuer>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Set the number for this invoice."]
    pub number: Option<json::OwnedValue>,
    #[doc = "The account (if any) for which the funds of the invoice payment are intended."]
    pub on_behalf_of: Option<json::OwnedValue>,
    #[doc = "Configuration settings for the PaymentIntent that is generated when the invoice is finalized."]
    pub payment_settings: Option<crate::invoices::UpdateInvoiceParamsPaymentSettings>,
    #[doc = "The rendering-related settings that control how invoices render in customer-facing interfaces such as the PDF or host..."]
    pub rendering: Option<stripe_types_11::invoices::UpdateInvoiceParamsRendering>,
    #[doc = "Settings for the cost of shipping for this invoice."]
    pub shipping_cost: Option<crate::invoices::UpdateInvoiceParamsShippingCost>,
    #[doc = "Shipping details for the invoice."]
    pub shipping_details: Option<stripe_types_09::invoices::UpdateInvoiceParamsShippingDetails>,
    #[doc = "Extra information about a charge for the customer's credit card statement."]
    pub statement_descriptor: Option<String>,
    #[doc = "If specified, the funds from the invoice will be transferred to the destination and the ID of the resulting transfer ..."]
    pub transfer_data: Option<stripe_types_06::invoices::UpdateInvoiceParamsTransferData>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for UpdateInvoiceParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("application_fee_amount", self.application_fee_amount.as_ref());
        form.field_opt("auto_advance", self.auto_advance.as_ref());
        form.field_opt("automatic_tax", self.automatic_tax.as_ref());
        form.field_opt("automatically_finalizes_at", self.automatically_finalizes_at.as_ref());
        form.field_opt("collection_method", self.collection_method.as_ref());
        form.field_opt("days_until_due", self.days_until_due.as_ref());
        form.field_opt("default_payment_method", self.default_payment_method.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field_opt("due_date", self.due_date.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("footer", self.footer.as_ref());
        form.field_opt("issuer", self.issuer.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("payment_settings", self.payment_settings.as_ref());
        form.field_opt("rendering", self.rendering.as_ref());
        form.field_opt("shipping_cost", self.shipping_cost.as_ref());
        form.field_opt("shipping_details", self.shipping_details.as_ref());
        form.field_opt("statement_descriptor", self.statement_descriptor.as_ref());
        form.field_opt("transfer_data", self.transfer_data.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct UpdateInvoiceParamsPaymentSettings {
    #[doc = "ID of the mandate to be used for this invoice."]
    pub default_mandate: Option<json::OwnedValue>,
    #[doc = "Payment-method-specific configuration to provide to the invoice’s PaymentIntent."]
    pub payment_method_options: Option<crate::invoices::UpdateInvoiceParamsPaymentSettingsPaymentMethodOptions>,
    #[doc = "The list of payment method types (e.g."]
    pub payment_method_types: Option<json::OwnedValue>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for UpdateInvoiceParamsPaymentSettings {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("payment_method_options", self.payment_method_options.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct UpdateInvoiceParamsPaymentSettingsPaymentMethodOptions {
    #[doc = "If paying by `acss_debit`, this sub-hash contains details about the Canadian pre-authorized debit payment method opti..."]
    pub acss_debit: Option<stripe_types_10::invoices::UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebit>,
    #[doc = "If paying by `bancontact`, this sub-hash contains details about the Bancontact payment method options to pass to the ..."]
    pub bancontact: Option<stripe_types_08::invoices::UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsBancontact>,
    #[doc = "If paying by `billie`, this sub-hash contains details about the Billie payment method options to pass to the invoice’..."]
    pub billie: Option<stripe_types_11::invoices::UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillie>,
    #[doc = "If paying by `blik`, this sub-hash contains details about the Blik payment method options to pass to the invoice’s Pa..."]
    pub blik: Option<stripe_types_06::invoices::UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsBlik>,
    #[doc = "If paying by `card`, this sub-hash contains details about the Card payment method options to pass to the invoice’s Pa..."]
    pub card: Option<stripe_types_12::invoices::UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsCard>,
    #[doc = "If paying by `customer_balance`, this sub-hash contains details about the Bank transfer payment method options to pas..."]
    pub customer_balance:
        Option<stripe_types_11::invoices::UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsCustomerBalance>,
    #[doc = "If paying by `konbini`, this sub-hash contains details about the Konbini payment method options to pass to the invoic..."]
    pub konbini: Option<stripe_types_05::invoices::UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsKonbini>,
    #[doc = "If paying by `payto`, this sub-hash contains details about the PayTo payment method options to pass to the invoice’s ..."]
    pub payto: Option<stripe_types_10::invoices::UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsPayto>,
    #[doc = "If paying by `pix`, this sub-hash contains details about the Pix payment method options to pass to the invoice’s Paym..."]
    pub pix: Option<stripe_types_09::invoices::UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsPix>,
    #[doc = "If paying by `sepa_debit`, this sub-hash contains details about the SEPA Direct Debit payment method options to pass ..."]
    pub sepa_debit: Option<stripe_types_05::invoices::UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsSepaDebit>,
    #[doc = "If paying by `upi`, this sub-hash contains details about the UPI payment method options to pass to the invoice’s Paym..."]
    pub upi: Option<stripe_types_11::invoices::UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsUpi>,
    #[doc = "If paying by `us_bank_account`, this sub-hash contains details about the ACH direct debit payment method options to p..."]
    pub us_bank_account:
        Option<stripe_types_12::invoices::UpdateInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccount>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for UpdateInvoiceParamsPaymentSettingsPaymentMethodOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("acss_debit", self.acss_debit.as_ref());
        form.field_opt("bancontact", self.bancontact.as_ref());
        form.field_opt("billie", self.billie.as_ref());
        form.field_opt("blik", self.blik.as_ref());
        form.field_opt("card", self.card.as_ref());
        form.field_opt("customer_balance", self.customer_balance.as_ref());
        form.field_opt("konbini", self.konbini.as_ref());
        form.field_opt("payto", self.payto.as_ref());
        form.field_opt("pix", self.pix.as_ref());
        form.field_opt("sepa_debit", self.sepa_debit.as_ref());
        form.field_opt("upi", self.upi.as_ref());
        form.field_opt("us_bank_account", self.us_bank_account.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct UpdateInvoiceParamsShippingCost {
    #[doc = "The ID of the shipping rate to use for this order."]
    pub shipping_rate: Option<String>,
    #[doc = "Parameters to create a new ad-hoc shipping rate for this order."]
    pub shipping_rate_data: Option<stripe_types_12::invoices::UpdateInvoiceParamsShippingCostShippingRateData>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for UpdateInvoiceParamsShippingCost {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("shipping_rate", self.shipping_rate.as_ref());
        form.field_opt("shipping_rate_data", self.shipping_rate_data.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct UpdateLinesParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub invoice_metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The line items to update."]
    pub lines: Vec<stripe_types_12::invoices::UpdateLinesParamsLinesItem>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for UpdateLinesParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.invoice_metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("invoice_metadata", key, value);
            }
        }
        for (index, item) in self.lines.iter().enumerate() {
            form.field_indexed("lines", index, item);
        }
    }
}
