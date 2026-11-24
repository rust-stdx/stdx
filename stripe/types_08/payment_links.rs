#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParamsConsentCollection {
    #[doc = "Determines the display of payment method reuse agreement text in the UI."]
    pub payment_method_reuse_agreement:
        Option<stripe_types_05::payment_links::CreatePaymentLinkParamsConsentCollectionPaymentMethodReuseAgreement>,
    #[doc = "If set to `auto`, enables the collection of customer consent for promotional communications."]
    pub promotions: Option<stripe_types_03::payment_links::CreatePaymentLinkParamsConsentCollectionPromotions>,
    #[doc = "If set to `required`, it requires customers to check a terms of service checkbox before being able to pay."]
    pub terms_of_service:
        Option<stripe_types_03::payment_links::CreatePaymentLinkParamsConsentCollectionTermsOfService>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsConsentCollection {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("payment_method_reuse_agreement", self.payment_method_reuse_agreement.as_ref());
        form.field_opt("promotions", self.promotions.as_ref());
        form.field_opt("terms_of_service", self.terms_of_service.as_ref());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone)]
pub struct CreatePaymentLinkParamsCustomFieldsItem {
    #[doc = "Configuration for `type=dropdown` fields."]
    pub dropdown: Option<stripe_types_06::payment_links::CreatePaymentLinkParamsCustomFieldsItemDropdown>,
    #[doc = "String of your choice that your integration can use to reconcile this field."]
    pub key: String,
    #[doc = "The label for the field, displayed to the customer."]
    pub label: stripe_types_06::payment_links::CreatePaymentLinkParamsCustomFieldsItemLabel,
    #[doc = "Configuration for `type=numeric` fields."]
    pub numeric: Option<stripe_types_02::payment_links::CreatePaymentLinkParamsCustomFieldsItemNumeric>,
    #[doc = "Whether the customer is required to complete the field before completing the Checkout Session."]
    pub optional: Option<bool>,
    #[doc = "Configuration for `type=text` fields."]
    pub text: Option<stripe_types_06::payment_links::CreatePaymentLinkParamsCustomFieldsItemText>,
    #[doc = "The type of the field."]
    pub r#type: stripe_types_00::payment_links::CreatePaymentLinkParamsCustomFieldsItemType,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsCustomFieldsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("dropdown", self.dropdown.as_ref());
        form.field("key", &self.key);
        form.field("label", &self.label);
        form.field_opt("numeric", self.numeric.as_ref());
        form.field_opt("optional", self.optional.as_ref());
        form.field_opt("text", self.text.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParamsLineItemsItemPriceData {
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "The ID of the [Product](https://docs.stripe.com/api/products) that this [Price](https://docs.stripe.com/api/prices) w..."]
    pub product: Option<String>,
    #[doc = "Data used to generate a new [Product](https://docs.stripe.com/api/products) object inline."]
    pub product_data: Option<stripe_types_05::payment_links::CreatePaymentLinkParamsLineItemsItemPriceDataProductData>,
    #[doc = "The recurring components of a price such as `interval` and `interval_count`."]
    pub recurring: Option<stripe_types_05::payment_links::CreatePaymentLinkParamsLineItemsItemPriceDataRecurring>,
    #[doc = "Only required if a default tax behavior..."]
    pub tax_behavior: Option<stripe_types_02::payment_links::CreatePaymentLinkParamsLineItemsItemPriceDataTaxBehavior>,
    #[doc = "A non-negative integer in cents (or local equivalent) representing how much to charge."]
    pub unit_amount: Option<i64>,
    #[doc = "Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places."]
    pub unit_amount_decimal: Option<String>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsLineItemsItemPriceData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("currency", &self.currency);
        form.field_opt("product", self.product.as_ref());
        form.field_opt("product_data", self.product_data.as_ref());
        form.field_opt("recurring", self.recurring.as_ref());
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
        form.field_opt("unit_amount", self.unit_amount.as_ref());
        form.field_opt("unit_amount_decimal", self.unit_amount_decimal.as_ref());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone)]
pub struct CreatePaymentLinkParamsSubscriptionDataInvoiceSettingsIssuer {
    #[doc = "The connected account being referenced when `type` is `account`."]
    pub account: Option<String>,
    #[doc = "Type of the account referenced in the request."]
    pub r#type: stripe_types_06::payment_links::CreatePaymentLinkParamsSubscriptionDataInvoiceSettingsIssuerType,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsSubscriptionDataInvoiceSettingsIssuer {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field("type", &self.r#type);
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
pub struct PaymentLinksResourceConsentCollection {
    #[doc = "Settings related to the payment method reuse text shown in the Checkout UI."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_reuse_agreement:
        Option<stripe_types_06::payment_links::PaymentLinksResourcePaymentMethodReuseAgreement>,
    #[doc = "If set to `auto`, enables the collection of customer consent for promotional communications."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub promotions: Option<stripe_types_02::payment_links::PaymentLinksResourceConsentCollectionPromotions>,
    #[doc = "If set to `required`, it requires cutomers to accept the terms of service before being able to pay."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<stripe_types_02::payment_links::PaymentLinksResourceConsentCollectionTermsOfService>,
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentLinkParamsNameCollection {
    #[doc = "Controls settings applied for collecting the customer's business name."]
    pub business: Option<stripe_types_06::payment_links::UpdatePaymentLinkParamsNameCollectionBusiness>,
    #[doc = "Controls settings applied for collecting the customer's individual name."]
    pub individual: Option<stripe_types_06::payment_links::UpdatePaymentLinkParamsNameCollectionIndividual>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for UpdatePaymentLinkParamsNameCollection {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("business", self.business.as_ref());
        form.field_opt("individual", self.individual.as_ref());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentLinkParamsPaymentMethodOptions {
    #[doc = "Configuration for `card` payment methods."]
    pub card: Option<stripe_types_06::payment_links::UpdatePaymentLinkParamsPaymentMethodOptionsCard>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for UpdatePaymentLinkParamsPaymentMethodOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("card", self.card.as_ref());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone)]
pub struct UpdatePaymentLinkParamsSubscriptionDataTrialSettingsEndBehavior {
    #[doc = "Indicates how the subscription should change when the trial ends if the user did not provide a payment method."]
    pub missing_payment_method: stripe_types_05::payment_links::UpdatePaymentLinkParamsSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for UpdatePaymentLinkParamsSubscriptionDataTrialSettingsEndBehavior {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("missing_payment_method", &self.missing_payment_method);
    }
}
