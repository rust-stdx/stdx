#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParamsAutomaticTax {
    #[doc = "Set to `true` to [calculate tax automatically](https://docs.stripe.com/tax) using the customer's location."]
    pub enabled: bool,
    #[doc = "The account that's liable for tax."]
    pub liability: Option<stripe_types_06::payment_links::CreatePaymentLinkParamsAutomaticTaxLiability>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsAutomaticTax {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("liability", self.liability.as_ref());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParamsCustomText {
    #[doc = "Custom text that should be displayed after the payment confirmation button."]
    pub after_submit: Option<stripe_types_01::payment_links::CreatePaymentLinkParamsCustomTextAfterSubmit>,
    #[doc = "Custom text that should be displayed alongside shipping address collection."]
    pub shipping_address: Option<stripe_types_06::payment_links::CreatePaymentLinkParamsCustomTextShippingAddress>,
    #[doc = "Custom text that should be displayed alongside the payment confirmation button."]
    pub submit: Option<stripe_types_06::payment_links::CreatePaymentLinkParamsCustomTextSubmit>,
    #[doc = "Custom text that should be displayed in place of the default terms of service agreement text."]
    pub terms_of_service_acceptance:
        Option<stripe_types_05::payment_links::CreatePaymentLinkParamsCustomTextTermsOfServiceAcceptance>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsCustomText {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("after_submit", self.after_submit.as_ref());
        form.field_opt("shipping_address", self.shipping_address.as_ref());
        form.field_opt("submit", self.submit.as_ref());
        form.field_opt("terms_of_service_acceptance", self.terms_of_service_acceptance.as_ref());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone)]
pub struct CreatePaymentLinkParamsInvoiceCreationInvoiceDataIssuer {
    #[doc = "The connected account being referenced when `type` is `account`."]
    pub account: Option<String>,
    #[doc = "Type of the account referenced in the request."]
    pub r#type: stripe_types_06::payment_links::CreatePaymentLinkParamsInvoiceCreationInvoiceDataIssuerType,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsInvoiceCreationInvoiceDataIssuer {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParamsNameCollection {
    #[doc = "Controls settings applied for collecting the customer's business name."]
    pub business: Option<stripe_types_03::payment_links::CreatePaymentLinkParamsNameCollectionBusiness>,
    #[doc = "Controls settings applied for collecting the customer's individual name."]
    pub individual: Option<stripe_types_05::payment_links::CreatePaymentLinkParamsNameCollectionIndividual>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsNameCollection {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("business", self.business.as_ref());
        form.field_opt("individual", self.individual.as_ref());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone)]
pub struct CreatePaymentLinkParamsSubscriptionDataTrialSettings {
    #[doc = "Defines how the subscription should behave when the user's free trial ends."]
    pub end_behavior: stripe_types_05::payment_links::CreatePaymentLinkParamsSubscriptionDataTrialSettingsEndBehavior,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsSubscriptionDataTrialSettings {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("end_behavior", &self.end_behavior);
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
#[derive(Debug, Clone)]
pub enum PaymentLinkApplicationExpanded {
    /// Expanded `application`.
    Application(Box<stripe_types_05::common::Application>),
    /// An object type not known to this version of the crate.
    Other(json::OwnedValue),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
impl<'json> json::Decode<'json> for PaymentLinkApplicationExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "application" => {
                PaymentLinkApplicationExpanded::Application(Box::new(json::from_slice_owned(&json::to_vec(&value))?))
            }
            _ => PaymentLinkApplicationExpanded::Other(value),
        })
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
pub struct PaymentLinksResourceCardPaymentMethodOptions {
    #[doc = "Restrictions to apply to the card payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<stripe_types_06::payment_links::PaymentLinksResourceCardRestrictions>,
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone)]
pub struct UpdatePaymentLinkParamsAfterCompletion {
    #[doc = "Configuration when `type=hosted_confirmation`."]
    pub hosted_confirmation:
        Option<stripe_types_05::payment_links::UpdatePaymentLinkParamsAfterCompletionHostedConfirmation>,
    #[doc = "Configuration when `type=redirect`."]
    pub redirect: Option<stripe_types_06::payment_links::UpdatePaymentLinkParamsAfterCompletionRedirect>,
    #[doc = "The specified behavior after the purchase is complete."]
    pub r#type: stripe_types_06::payment_links::UpdatePaymentLinkParamsAfterCompletionType,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for UpdatePaymentLinkParamsAfterCompletion {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("hosted_confirmation", self.hosted_confirmation.as_ref());
        form.field_opt("redirect", self.redirect.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone)]
pub struct UpdatePaymentLinkParamsAutomaticTaxLiability {
    #[doc = "The connected account being referenced when `type` is `account`."]
    pub account: Option<String>,
    #[doc = "Type of the account referenced in the request."]
    pub r#type: stripe_types_06::payment_links::UpdatePaymentLinkParamsAutomaticTaxLiabilityType,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for UpdatePaymentLinkParamsAutomaticTaxLiability {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentLinkParamsCustomText {
    #[doc = "Custom text that should be displayed after the payment confirmation button."]
    pub after_submit: Option<stripe_types_06::payment_links::UpdatePaymentLinkParamsCustomTextAfterSubmit>,
    #[doc = "Custom text that should be displayed alongside shipping address collection."]
    pub shipping_address: Option<stripe_types_05::payment_links::UpdatePaymentLinkParamsCustomTextShippingAddress>,
    #[doc = "Custom text that should be displayed alongside the payment confirmation button."]
    pub submit: Option<stripe_types_05::payment_links::UpdatePaymentLinkParamsCustomTextSubmit>,
    #[doc = "Custom text that should be displayed in place of the default terms of service agreement text."]
    pub terms_of_service_acceptance:
        Option<stripe_types_06::payment_links::UpdatePaymentLinkParamsCustomTextTermsOfServiceAcceptance>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for UpdatePaymentLinkParamsCustomText {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("after_submit", self.after_submit.as_ref());
        form.field_opt("shipping_address", self.shipping_address.as_ref());
        form.field_opt("submit", self.submit.as_ref());
        form.field_opt("terms_of_service_acceptance", self.terms_of_service_acceptance.as_ref());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentLinkParamsLineItemsItem {
    #[doc = "When set, provides configuration for this item’s quantity to be adjusted by the customer during checkout."]
    pub adjustable_quantity:
        Option<stripe_types_06::payment_links::UpdatePaymentLinkParamsLineItemsItemAdjustableQuantity>,
    #[doc = "The ID of an existing line item on the payment link."]
    pub id: String,
    #[doc = "The quantity of the line item being purchased."]
    pub quantity: Option<i64>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for UpdatePaymentLinkParamsLineItemsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("adjustable_quantity", self.adjustable_quantity.as_ref());
        form.field("id", &self.id);
        form.field_opt("quantity", self.quantity.as_ref());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone)]
pub struct UpdatePaymentLinkParamsSubscriptionDataInvoiceSettingsIssuer {
    #[doc = "The connected account being referenced when `type` is `account`."]
    pub account: Option<String>,
    #[doc = "Type of the account referenced in the request."]
    pub r#type: stripe_types_06::payment_links::UpdatePaymentLinkParamsSubscriptionDataInvoiceSettingsIssuerType,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for UpdatePaymentLinkParamsSubscriptionDataInvoiceSettingsIssuer {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field("type", &self.r#type);
    }
}
