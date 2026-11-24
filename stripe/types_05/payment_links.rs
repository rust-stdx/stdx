#[cfg(feature = "payment_links")]
#[derive(Debug, Clone)]
pub struct CreatePaymentLinkParamsConsentCollectionPaymentMethodReuseAgreement {
    #[doc = "Determines the position and visibility of the payment method reuse agreement in the UI."]
    pub position:
        stripe_types_00::payment_links::CreatePaymentLinkParamsConsentCollectionPaymentMethodReuseAgreementPosition,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsConsentCollectionPaymentMethodReuseAgreement {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("position", &self.position);
    }
}

#[cfg(feature = "payment_links")]
pub type CreatePaymentLinkParamsCustomTextTermsOfServiceAcceptance =
    stripe_types_01::payment_links::CreatePaymentLinkParamsCustomTextAfterSubmit;

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParamsLineItemsItemPriceDataProductData {
    #[doc = "The product's description, meant to be displayable to the customer."]
    pub description: Option<String>,
    #[doc = "A list of up to 8 URLs of images for this product, meant to be displayable to the customer."]
    pub images: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The product's name, meant to be displayable to the customer."]
    pub name: String,
    #[doc = "A [tax code](https://docs.stripe.com/tax/tax-categories) ID."]
    pub tax_code: Option<String>,
    #[doc = "Tax details for this product, including the [tax code](/tax/tax-codes) and an optional performance location."]
    pub tax_details:
        Option<stripe_types_02::payment_links::CreatePaymentLinkParamsLineItemsItemPriceDataProductDataTaxDetails>,
    #[doc = "A label that represents units of this product."]
    pub unit_label: Option<String>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsLineItemsItemPriceDataProductData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("description", self.description.as_ref());
        if let Some(values) = self.images.as_ref() {
            form.field_array("images", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field("name", &self.name);
        form.field_opt("tax_code", self.tax_code.as_ref());
        form.field_opt("tax_details", self.tax_details.as_ref());
        form.field_opt("unit_label", self.unit_label.as_ref());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone)]
pub struct CreatePaymentLinkParamsLineItemsItemPriceDataRecurring {
    #[doc = "Specifies billing frequency."]
    pub interval: stripe_types_01::payment_links::CreatePaymentLinkParamsLineItemsItemPriceDataRecurringInterval,
    #[doc = "The number of intervals between subscription billings."]
    pub interval_count: Option<i64>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsLineItemsItemPriceDataRecurring {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("interval", &self.interval);
        form.field_opt("interval_count", self.interval_count.as_ref());
    }
}

#[cfg(feature = "payment_links")]
pub type CreatePaymentLinkParamsNameCollectionIndividual =
    stripe_types_03::payment_links::CreatePaymentLinkParamsNameCollectionBusiness;

#[cfg(feature = "payment_links")]
pub type CreatePaymentLinkParamsOptionalItemsItemAdjustableQuantity =
    stripe_types_01::payment_links::CreatePaymentLinkParamsLineItemsItemAdjustableQuantity;

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParamsPaymentIntentData {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    pub capture_method: Option<stripe_types_02::payment_links::CreatePaymentLinkParamsPaymentIntentDataCaptureMethod>,
    #[doc = "An arbitrary string attached to the object."]
    pub description: Option<String>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that will declaratively set metadata on Payment Inten..."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Indicates that you intend to [make future payments](https://docs.stripe.com/payments/payment-intents#future-usage) wi..."]
    pub setup_future_usage:
        Option<stripe_types_03::payment_links::CreatePaymentLinkParamsPaymentIntentDataSetupFutureUsage>,
    #[doc = "Text that appears on the customer's statement as the statement descriptor for a non-card charge."]
    pub statement_descriptor: Option<String>,
    #[doc = "Provides information about a card charge."]
    pub statement_descriptor_suffix: Option<String>,
    #[doc = "A string that identifies the resulting payment as part of a group."]
    pub transfer_group: Option<String>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsPaymentIntentData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("description", self.description.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("statement_descriptor", self.statement_descriptor.as_ref());
        form.field_opt("statement_descriptor_suffix", self.statement_descriptor_suffix.as_ref());
        form.field_opt("transfer_group", self.transfer_group.as_ref());
    }
}

#[cfg(feature = "payment_links")]
pub type CreatePaymentLinkParamsPaymentMethodCollection =
    stripe_types_02::payment_links::CreatePaymentLinkParamsCustomerCreation;

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParamsShippingAddressCollection {
    #[doc = "An array of two-letter ISO country codes representing which countries Checkout should provide as options for shipping..."]
    pub allowed_countries:
        Vec<stripe_types_01::payment_links::CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsShippingAddressCollection {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_array("allowed_countries", self.allowed_countries.as_slice());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone)]
pub struct CreatePaymentLinkParamsSubscriptionDataTrialSettingsEndBehavior {
    #[doc = "Indicates how the subscription should change when the trial ends if the user did not provide a payment method."]
    pub missing_payment_method: stripe_types_04::payment_links::CreatePaymentLinkParamsSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsSubscriptionDataTrialSettingsEndBehavior {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("missing_payment_method", &self.missing_payment_method);
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
pub struct PaymentLinksResourceCustomFieldsLabel {
    #[doc = "Custom text for the label, displayed to the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<String>,
    #[doc = "The type of the label."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::payment_links::PaymentLinksResourceCustomFieldsLabelType,
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
pub struct PaymentLinksResourceCustomText {
    #[doc = "Custom text that should be displayed after the payment confirmation button."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub after_submit: Option<stripe_types_01::payment_links::PaymentLinksResourceCustomTextPosition>,
    #[doc = "Custom text that should be displayed alongside shipping address collection."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<stripe_types_01::payment_links::PaymentLinksResourceCustomTextPosition>,
    #[doc = "Custom text that should be displayed alongside the payment confirmation button."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub submit: Option<stripe_types_01::payment_links::PaymentLinksResourceCustomTextPosition>,
    #[doc = "Custom text that should be displayed in place of the default terms of service agreement text."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub terms_of_service_acceptance: Option<stripe_types_01::payment_links::PaymentLinksResourceCustomTextPosition>,
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
pub struct PaymentLinksResourceShippingAddressCollection {
    #[doc = "An array of two-letter ISO country codes representing which countries Checkout should provide as options for shipping..."]
    #[json(default)]
    pub allowed_countries:
        Vec<stripe_types_04::payment_links::PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem>,
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
pub struct PaymentLinksResourceTaxIdCollection {
    #[doc = "Indicates whether tax ID collection is enabled for the session."]
    #[json(default)]
    pub enabled: bool,
    #[json(default)]
    pub required: stripe_types_01::payment_links::PaymentLinksResourceTaxIdCollectionRequired,
}

#[cfg(feature = "payment_links")]
pub type UpdatePaymentLinkParamsAfterCompletionHostedConfirmation =
    stripe_types_02::payment_links::CreatePaymentLinkParamsAfterCompletionHostedConfirmation;

#[cfg(feature = "payment_links")]
pub type UpdatePaymentLinkParamsConsentCollectionPaymentMethodReuseAgreementPosition =
    stripe_types_00::payment_links::CreatePaymentLinkParamsConsentCollectionPaymentMethodReuseAgreementPosition;

#[cfg(feature = "payment_links")]
pub type UpdatePaymentLinkParamsConsentCollectionTermsOfService =
    stripe_types_03::payment_links::CreatePaymentLinkParamsConsentCollectionTermsOfService;

#[cfg(feature = "payment_links")]
pub type UpdatePaymentLinkParamsCustomTextShippingAddress =
    stripe_types_01::payment_links::CreatePaymentLinkParamsCustomTextAfterSubmit;

#[cfg(feature = "payment_links")]
pub type UpdatePaymentLinkParamsCustomTextSubmit =
    stripe_types_01::payment_links::CreatePaymentLinkParamsCustomTextAfterSubmit;

#[cfg(feature = "payment_links")]
pub type UpdatePaymentLinkParamsInvoiceCreationInvoiceDataIssuerType =
    stripe_types_03::payment_links::CreatePaymentLinkParamsAutomaticTaxLiabilityType;

#[cfg(feature = "payment_links")]
pub type UpdatePaymentLinkParamsPhoneNumberCollection =
    stripe_types_04::payment_links::CreatePaymentLinkParamsPhoneNumberCollection;

#[cfg(feature = "payment_links")]
pub type UpdatePaymentLinkParamsSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod =
    stripe_types_04::payment_links::CreatePaymentLinkParamsSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod;

#[cfg(feature = "payment_links")]
pub type UpdatePaymentLinkParamsTaxIdCollectionRequired =
    stripe_types_00::payment_links::CreatePaymentLinkParamsTaxIdCollectionRequired;
