#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParamsOptionalItemsItem {
    #[doc = "When set, provides configuration for the customer to adjust the quantity of the line item created when a customer cho..."]
    pub adjustable_quantity:
        Option<stripe_types_05::payment_links::CreatePaymentLinkParamsOptionalItemsItemAdjustableQuantity>,
    #[doc = "The ID of the [Price](https://docs.stripe.com/api/prices) or [Plan](https://docs.stripe.com/api/plans) object."]
    pub price: String,
    #[doc = "The initial quantity of the line item created when a customer chooses to add this optional item to their order."]
    pub quantity: i64,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsOptionalItemsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("adjustable_quantity", self.adjustable_quantity.as_ref());
        form.field("price", &self.price);
        form.field("quantity", &self.quantity);
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParamsPaymentMethodOptionsCard {
    #[doc = "Restrictions to apply to the card payment method."]
    pub restrictions:
        Option<stripe_types_06::payment_links::CreatePaymentLinkParamsPaymentMethodOptionsCardRestrictions>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsPaymentMethodOptionsCard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("restrictions", self.restrictions.as_ref());
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
pub struct PaymentLinksResourceCustomFields {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub dropdown: Option<stripe_types_06::payment_links::PaymentLinksResourceCustomFieldsDropdown>,
    #[doc = "String of your choice that your integration can use to reconcile this field."]
    #[json(default)]
    pub key: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<stripe_types_05::payment_links::PaymentLinksResourceCustomFieldsLabel>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub numeric: Option<stripe_types_00::payment_links::PaymentLinksResourceCustomFieldsNumeric>,
    #[doc = "Whether the customer is required to complete the field before completing the Checkout Session."]
    #[json(default)]
    pub optional: bool,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<stripe_types_02::payment_links::PaymentLinksResourceCustomFieldsText>,
    #[doc = "The type of the field."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::payment_links::PaymentLinksResourceCustomFieldsType,
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone)]
pub struct UpdatePaymentLinkParamsConsentCollectionPaymentMethodReuseAgreement {
    #[doc = "Determines the position and visibility of the payment method reuse agreement in the UI."]
    pub position:
        stripe_types_05::payment_links::UpdatePaymentLinkParamsConsentCollectionPaymentMethodReuseAgreementPosition,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for UpdatePaymentLinkParamsConsentCollectionPaymentMethodReuseAgreement {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("position", &self.position);
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone)]
pub struct UpdatePaymentLinkParamsInvoiceCreationInvoiceDataIssuer {
    #[doc = "The connected account being referenced when `type` is `account`."]
    pub account: Option<String>,
    #[doc = "Type of the account referenced in the request."]
    pub r#type: stripe_types_05::payment_links::UpdatePaymentLinkParamsInvoiceCreationInvoiceDataIssuerType,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for UpdatePaymentLinkParamsInvoiceCreationInvoiceDataIssuer {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentLinkParamsInvoiceCreationInvoiceDataRenderingOptions {
    #[doc = "How line-item prices and amounts will be displayed with respect to tax on invoice PDFs."]
    pub amount_tax_display: Option<stripe_types_06::payment_links::UpdatePaymentLinkParamsInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay>,
    #[doc = "ID of the invoice rendering template to use for this invoice."]
    pub template: Option<String>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for UpdatePaymentLinkParamsInvoiceCreationInvoiceDataRenderingOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount_tax_display", self.amount_tax_display.as_ref());
        form.field_opt("template", self.template.as_ref());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone)]
pub struct UpdatePaymentLinkParamsRestrictions {
    #[doc = "Configuration for the `completed_sessions` restriction type."]
    pub completed_sessions: stripe_types_06::payment_links::UpdatePaymentLinkParamsRestrictionsCompletedSessions,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for UpdatePaymentLinkParamsRestrictions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("completed_sessions", &self.completed_sessions);
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentLinkParamsShippingAddressCollection {
    #[doc = "An array of two-letter ISO country codes representing which countries Checkout should provide as options for shipping..."]
    pub allowed_countries:
        Vec<stripe_types_06::payment_links::UpdatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for UpdatePaymentLinkParamsShippingAddressCollection {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        for (index, item) in self.allowed_countries.iter().enumerate() {
            form.field_indexed("allowed_countries", index, item);
        }
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentLinkParamsTaxIdCollection {
    #[doc = "Enable tax ID collection during checkout."]
    pub enabled: bool,
    #[doc = "Describes whether a tax ID is required during checkout."]
    pub required: Option<stripe_types_05::payment_links::UpdatePaymentLinkParamsTaxIdCollectionRequired>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for UpdatePaymentLinkParamsTaxIdCollection {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("required", self.required.as_ref());
    }
}
