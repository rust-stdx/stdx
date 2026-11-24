#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionParamsInvoiceSettings {
    #[doc = "The account tax IDs associated with the subscription."]
    pub account_tax_ids: Option<json::OwnedValue>,
    #[doc = "A list of up to 4 custom fields to be displayed on the invoice."]
    pub custom_fields: Option<json::OwnedValue>,
    #[doc = "An arbitrary string attached to the object."]
    pub description: Option<String>,
    #[doc = "Footer to be displayed on the invoice."]
    pub footer: Option<String>,
    #[doc = "The connected account that issues the invoice."]
    pub issuer: Option<stripe_types_08::subscriptions::CreateSubscriptionParamsInvoiceSettingsIssuer>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsInvoiceSettings {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("description", self.description.as_ref());
        form.field_opt("footer", self.footer.as_ref());
        form.field_opt("issuer", self.issuer.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUpi {
    #[doc = "Configuration options for setting up an eMandate"]
    pub mandate_options: Option<
        stripe_types_08::subscriptions::CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUpiMandateOptions,
    >,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUpi {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct DeleteSubscriptionParams {
    #[doc = "Details about why this subscription was cancelled"]
    pub cancellation_details: Option<stripe_types_08::subscriptions::DeleteSubscriptionParamsCancellationDetails>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Will generate a final invoice that invoices for any un-invoiced metered usage and new/pending proration invoice items."]
    pub invoice_now: Option<bool>,
    #[doc = "Will generate a proration invoice item that credits remaining unused time until the subscription period end."]
    pub prorate: Option<bool>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for DeleteSubscriptionParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("cancellation_details", self.cancellation_details.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("invoice_now", self.invoice_now.as_ref());
        form.field_opt("prorate", self.prorate.as_ref());
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
pub struct SubscriptionsResourceStatusDetailsPublic {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paused: Option<stripe_types_07::subscriptions::SubscriptionsResourcePausedStatusDetailsPublic>,
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone)]
pub struct UpdateSubscriptionParamsAddInvoiceItemsItemPeriod {
    #[doc = "End of the invoice item period."]
    pub end: stripe_types_08::subscriptions::UpdateSubscriptionParamsAddInvoiceItemsItemPeriodEnd,
    #[doc = "Start of the invoice item period."]
    pub start: stripe_types_07::subscriptions::UpdateSubscriptionParamsAddInvoiceItemsItemPeriodStart,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for UpdateSubscriptionParamsAddInvoiceItemsItemPeriod {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("end", &self.end);
        form.field("start", &self.start);
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSubscriptionParamsInvoiceSettings {
    #[doc = "The account tax IDs associated with the subscription."]
    pub account_tax_ids: Option<json::OwnedValue>,
    #[doc = "A list of up to 4 custom fields to be displayed on the invoice."]
    pub custom_fields: Option<json::OwnedValue>,
    #[doc = "An arbitrary string attached to the object."]
    pub description: Option<json::OwnedValue>,
    #[doc = "Footer to be displayed on the invoice."]
    pub footer: Option<json::OwnedValue>,
    #[doc = "The connected account that issues the invoice."]
    pub issuer: Option<stripe_types_07::subscriptions::UpdateSubscriptionParamsInvoiceSettingsIssuer>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for UpdateSubscriptionParamsInvoiceSettings {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("issuer", self.issuer.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone)]
pub struct UpdateSubscriptionParamsItemsItemPriceData {
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "The ID of the [Product](https://docs.stripe.com/api/products) that this [Price](https://docs.stripe.com/api/prices) w..."]
    pub product: String,
    #[doc = "The recurring components of a price such as `interval` and `interval_count`."]
    pub recurring: stripe_types_07::subscriptions::UpdateSubscriptionParamsItemsItemPriceDataRecurring,
    #[doc = "Only required if a default tax behavior..."]
    pub tax_behavior: Option<stripe_types_05::subscriptions::UpdateSubscriptionParamsItemsItemPriceDataTaxBehavior>,
    #[doc = "A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge."]
    pub unit_amount: Option<i64>,
    #[doc = "Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places."]
    pub unit_amount_decimal: Option<String>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for UpdateSubscriptionParamsItemsItemPriceData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("currency", &self.currency);
        form.field("product", &self.product);
        form.field("recurring", &self.recurring);
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
        form.field_opt("unit_amount", self.unit_amount.as_ref());
        form.field_opt("unit_amount_decimal", self.unit_amount_decimal.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebit {
    #[doc = "Additional fields for Mandate creation"]
    pub mandate_options: Option<stripe_types_09::subscriptions::UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions>,
    #[doc = "Verification method for the intent"]
    pub verification_method: Option<stripe_types_05::subscriptions::UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("verification_method", self.verification_method.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPayto {
    #[doc = "Additional fields for Mandate creation."]
    pub mandate_options: Option<
        stripe_types_07::subscriptions::UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptions,
    >,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPayto {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPix {
    #[doc = "The number of seconds (between 10 and 1209600) after which Pix payment will expire."]
    pub expires_after_seconds: Option<i64>,
    #[doc = "Configuration options for setting up a mandate"]
    pub mandate_options: Option<
        stripe_types_07::subscriptions::UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptions,
    >,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for UpdateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPix {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("expires_after_seconds", self.expires_after_seconds.as_ref());
        form.field_opt("mandate_options", self.mandate_options.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone)]
pub struct UpdateSubscriptionParamsTrialSettings {
    #[doc = "Defines how the subscription should behave when the user's free trial ends."]
    pub end_behavior: stripe_types_07::subscriptions::UpdateSubscriptionParamsTrialSettingsEndBehavior,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for UpdateSubscriptionParamsTrialSettings {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("end_behavior", &self.end_behavior);
    }
}
