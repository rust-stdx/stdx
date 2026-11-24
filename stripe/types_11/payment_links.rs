#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParamsLineItemsItem {
    #[doc = "When set, provides configuration for this item’s quantity to be adjusted by the customer during checkout."]
    pub adjustable_quantity:
        Option<stripe_types_01::payment_links::CreatePaymentLinkParamsLineItemsItemAdjustableQuantity>,
    #[doc = "The ID of the [Price](https://docs.stripe.com/api/prices) or [Plan](https://docs.stripe.com/api/plans) object."]
    pub price: Option<String>,
    #[doc = "Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline."]
    pub price_data: Option<stripe_types_08::payment_links::CreatePaymentLinkParamsLineItemsItemPriceData>,
    #[doc = "The quantity of the line item being purchased."]
    pub quantity: i64,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsLineItemsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("adjustable_quantity", self.adjustable_quantity.as_ref());
        form.field_opt("price", self.price.as_ref());
        form.field_opt("price_data", self.price_data.as_ref());
        form.field("quantity", &self.quantity);
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParamsPaymentMethodOptions {
    #[doc = "Configuration for `card` payment methods."]
    pub card: Option<stripe_types_09::payment_links::CreatePaymentLinkParamsPaymentMethodOptionsCard>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsPaymentMethodOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("card", self.card.as_ref());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParamsSubscriptionDataInvoiceSettings {
    #[doc = "The connected account that issues the invoice."]
    pub issuer: Option<stripe_types_08::payment_links::CreatePaymentLinkParamsSubscriptionDataInvoiceSettingsIssuer>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsSubscriptionDataInvoiceSettings {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("issuer", self.issuer.as_ref());
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
pub struct PaymentLinksResourcePaymentMethodOptions {
    #[doc = "Configuration for `card` payment methods."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types_07::payment_links::PaymentLinksResourceCardPaymentMethodOptions>,
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentLinkParamsInvoiceCreationInvoiceData {
    #[doc = "The account tax IDs associated with the invoice."]
    pub account_tax_ids: Option<json::OwnedValue>,
    #[doc = "Default custom fields to be displayed on invoices for this customer."]
    pub custom_fields: Option<json::OwnedValue>,
    #[doc = "An arbitrary string attached to the object."]
    pub description: Option<String>,
    #[doc = "Default footer to be displayed on invoices for this customer."]
    pub footer: Option<String>,
    #[doc = "The connected account that issues the invoice."]
    pub issuer: Option<stripe_types_09::payment_links::UpdatePaymentLinkParamsInvoiceCreationInvoiceDataIssuer>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Default options for invoice PDF rendering for this customer."]
    pub rendering_options:
        Option<stripe_types_09::payment_links::UpdatePaymentLinkParamsInvoiceCreationInvoiceDataRenderingOptions>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for UpdatePaymentLinkParamsInvoiceCreationInvoiceData {
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
#[derive(Debug, Clone)]
pub struct UpdatePaymentLinkParamsSubscriptionDataTrialSettings {
    #[doc = "Defines how the subscription should behave when the user's free trial ends."]
    pub end_behavior: stripe_types_08::payment_links::UpdatePaymentLinkParamsSubscriptionDataTrialSettingsEndBehavior,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for UpdatePaymentLinkParamsSubscriptionDataTrialSettings {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("end_behavior", &self.end_behavior);
    }
}
