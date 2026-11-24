#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParamsInvoiceCreation {
    #[doc = "Whether the feature is enabled"]
    pub enabled: bool,
    #[doc = "Invoice PDF configuration."]
    pub invoice_data: Option<stripe_types_10::payment_links::CreatePaymentLinkParamsInvoiceCreationInvoiceData>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsInvoiceCreation {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("invoice_data", self.invoice_data.as_ref());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParamsSubscriptionData {
    #[doc = "The subscription's description, meant to be displayable to the customer."]
    pub description: Option<String>,
    #[doc = "All invoices will be billed using the specified settings."]
    pub invoice_settings:
        Option<stripe_types_11::payment_links::CreatePaymentLinkParamsSubscriptionDataInvoiceSettings>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that will declaratively set metadata on Subscriptions..."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Integer representing the number of trial period days before the customer is charged for the first time."]
    pub trial_period_days: Option<i64>,
    #[doc = "Settings related to subscription trials."]
    pub trial_settings: Option<stripe_types_07::payment_links::CreatePaymentLinkParamsSubscriptionDataTrialSettings>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsSubscriptionData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("description", self.description.as_ref());
        form.field_opt("invoice_settings", self.invoice_settings.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("trial_period_days", self.trial_period_days.as_ref());
        form.field_opt("trial_settings", self.trial_settings.as_ref());
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentLinksResourceShippingOption {
    #[doc = "A non-negative integer in cents representing how much to charge."]
    #[json(default)]
    pub shipping_amount: i64,
    #[doc = "The ID of the Shipping Rate to use for this shipping option."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<crate::Expandable<stripe_types_11::shipping_rates::ShippingRate>>,
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentLinkParamsInvoiceCreation {
    #[doc = "Whether the feature is enabled"]
    pub enabled: bool,
    #[doc = "Invoice PDF configuration."]
    pub invoice_data: Option<stripe_types_11::payment_links::UpdatePaymentLinkParamsInvoiceCreationInvoiceData>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for UpdatePaymentLinkParamsInvoiceCreation {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("invoice_data", self.invoice_data.as_ref());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentLinkParamsSubscriptionData {
    #[doc = "All invoices will be billed using the specified settings."]
    pub invoice_settings:
        Option<stripe_types_10::payment_links::UpdatePaymentLinkParamsSubscriptionDataInvoiceSettings>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that will declaratively set metadata on Subscriptions..."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Integer representing the number of trial period days before the customer is charged for the first time."]
    pub trial_period_days: Option<json::OwnedValue>,
    #[doc = "Settings related to subscription trials."]
    pub trial_settings: Option<stripe_types_11::payment_links::UpdatePaymentLinkParamsSubscriptionDataTrialSettings>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for UpdatePaymentLinkParamsSubscriptionData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("invoice_settings", self.invoice_settings.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("trial_settings", self.trial_settings.as_ref());
    }
}
