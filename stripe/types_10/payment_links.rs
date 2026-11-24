#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParamsInvoiceCreationInvoiceData {
    #[doc = "The account tax IDs associated with the invoice."]
    pub account_tax_ids: Option<json::OwnedValue>,
    #[doc = "Default custom fields to be displayed on invoices for this customer."]
    pub custom_fields: Option<json::OwnedValue>,
    #[doc = "An arbitrary string attached to the object."]
    pub description: Option<String>,
    #[doc = "Default footer to be displayed on invoices for this customer."]
    pub footer: Option<String>,
    #[doc = "The connected account that issues the invoice."]
    pub issuer: Option<stripe_types_07::payment_links::CreatePaymentLinkParamsInvoiceCreationInvoiceDataIssuer>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Default options for invoice PDF rendering for this customer."]
    pub rendering_options:
        Option<stripe_types_06::payment_links::CreatePaymentLinkParamsInvoiceCreationInvoiceDataRenderingOptions>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsInvoiceCreationInvoiceData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("description", self.description.as_ref());
        form.field_opt("footer", self.footer.as_ref());
        form.field_opt("issuer", self.issuer.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("rendering_options", self.rendering_options.as_ref());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentLinkParamsAutomaticTax {
    #[doc = "Set to `true` to [calculate tax automatically](https://docs.stripe.com/tax) using the customer's location."]
    pub enabled: bool,
    #[doc = "The account that's liable for tax."]
    pub liability: Option<stripe_types_07::payment_links::UpdatePaymentLinkParamsAutomaticTaxLiability>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for UpdatePaymentLinkParamsAutomaticTax {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("liability", self.liability.as_ref());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentLinkParamsConsentCollection {
    #[doc = "Determines the display of payment method reuse agreement text in the UI."]
    pub payment_method_reuse_agreement:
        Option<stripe_types_09::payment_links::UpdatePaymentLinkParamsConsentCollectionPaymentMethodReuseAgreement>,
    #[doc = "If set to `auto`, enables the collection of customer consent for promotional communications."]
    pub promotions: Option<stripe_types_06::payment_links::UpdatePaymentLinkParamsConsentCollectionPromotions>,
    #[doc = "If set to `required`, it requires customers to check a terms of service checkbox before being able to pay."]
    pub terms_of_service:
        Option<stripe_types_05::payment_links::UpdatePaymentLinkParamsConsentCollectionTermsOfService>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for UpdatePaymentLinkParamsConsentCollection {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("payment_method_reuse_agreement", self.payment_method_reuse_agreement.as_ref());
        form.field_opt("promotions", self.promotions.as_ref());
        form.field_opt("terms_of_service", self.terms_of_service.as_ref());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentLinkParamsSubscriptionDataInvoiceSettings {
    #[doc = "The connected account that issues the invoice."]
    pub issuer: Option<stripe_types_07::payment_links::UpdatePaymentLinkParamsSubscriptionDataInvoiceSettingsIssuer>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for UpdatePaymentLinkParamsSubscriptionDataInvoiceSettings {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("issuer", self.issuer.as_ref());
    }
}
