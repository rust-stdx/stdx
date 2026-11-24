#[cfg(feature = "payment_links")]
#[derive(Debug, Clone)]
pub struct CreatePaymentLinkParamsAfterCompletion {
    #[doc = "Configuration when `type=hosted_confirmation`."]
    pub hosted_confirmation:
        Option<stripe_types_02::payment_links::CreatePaymentLinkParamsAfterCompletionHostedConfirmation>,
    #[doc = "Configuration when `type=redirect`."]
    pub redirect: Option<stripe_types_00::payment_links::CreatePaymentLinkParamsAfterCompletionRedirect>,
    #[doc = "The specified behavior after the purchase is complete."]
    pub r#type: stripe_types_03::payment_links::CreatePaymentLinkParamsAfterCompletionType,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsAfterCompletion {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("hosted_confirmation", self.hosted_confirmation.as_ref());
        form.field_opt("redirect", self.redirect.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone)]
pub struct CreatePaymentLinkParamsAutomaticTaxLiability {
    #[doc = "The connected account being referenced when `type` is `account`."]
    pub account: Option<String>,
    #[doc = "Type of the account referenced in the request."]
    pub r#type: stripe_types_03::payment_links::CreatePaymentLinkParamsAutomaticTaxLiabilityType,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsAutomaticTaxLiability {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParamsCustomFieldsItemDropdown {
    #[doc = "The value that pre-fills the field on the payment page."]
    pub default_value: Option<String>,
    #[doc = "The options available for the customer to select."]
    pub options: Vec<stripe_types_01::payment_links::CreatePaymentLinkParamsCustomFieldsItemDropdownOptionsItem>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsCustomFieldsItemDropdown {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("default_value", self.default_value.as_ref());
        for (index, item) in self.options.iter().enumerate() {
            form.field_indexed("options", index, item);
        }
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone)]
pub struct CreatePaymentLinkParamsCustomFieldsItemLabel {
    #[doc = "Custom text for the label, displayed to the customer."]
    pub custom: String,
    #[doc = "The type of the label."]
    pub r#type: stripe_types_01::payment_links::CreatePaymentLinkParamsCustomFieldsItemLabelType,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsCustomFieldsItemLabel {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("custom", &self.custom);
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "payment_links")]
pub type CreatePaymentLinkParamsCustomFieldsItemText =
    stripe_types_02::payment_links::CreatePaymentLinkParamsCustomFieldsItemNumeric;

#[cfg(feature = "payment_links")]
pub type CreatePaymentLinkParamsCustomTextShippingAddress =
    stripe_types_01::payment_links::CreatePaymentLinkParamsCustomTextAfterSubmit;

#[cfg(feature = "payment_links")]
pub type CreatePaymentLinkParamsCustomTextSubmit =
    stripe_types_01::payment_links::CreatePaymentLinkParamsCustomTextAfterSubmit;

#[cfg(feature = "payment_links")]
pub type CreatePaymentLinkParamsInvoiceCreationInvoiceDataIssuerType =
    stripe_types_03::payment_links::CreatePaymentLinkParamsAutomaticTaxLiabilityType;

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParamsInvoiceCreationInvoiceDataRenderingOptions {
    #[doc = "How line-item prices and amounts will be displayed with respect to tax on invoice PDFs."]
    pub amount_tax_display: Option<stripe_types_01::payment_links::CreatePaymentLinkParamsInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay>,
    #[doc = "ID of the invoice rendering template to use for this invoice."]
    pub template: Option<String>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsInvoiceCreationInvoiceDataRenderingOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount_tax_display", self.amount_tax_display.as_ref());
        form.field_opt("template", self.template.as_ref());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParamsPaymentMethodOptionsCardRestrictions {
    #[doc = "The card brands to block."]
    pub brands_blocked: Option<Vec<stripe_types_04::payment_links::CreatePaymentLinkParamsPaymentMethodOptionsCardRestrictionsBrandsBlockedItem>>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsPaymentMethodOptionsCardRestrictions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.brands_blocked.as_ref() {
            form.field_array("brands_blocked", values.as_slice());
        }
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone)]
pub struct CreatePaymentLinkParamsRestrictions {
    #[doc = "Configuration for the `completed_sessions` restriction type."]
    pub completed_sessions: stripe_types_03::payment_links::CreatePaymentLinkParamsRestrictionsCompletedSessions,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsRestrictions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("completed_sessions", &self.completed_sessions);
    }
}

#[cfg(feature = "payment_links")]
pub type CreatePaymentLinkParamsSubscriptionDataInvoiceSettingsIssuerType =
    stripe_types_03::payment_links::CreatePaymentLinkParamsAutomaticTaxLiabilityType;

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParamsTaxIdCollection {
    #[doc = "Enable tax ID collection during checkout."]
    pub enabled: bool,
    #[doc = "Describes whether a tax ID is required during checkout."]
    pub required: Option<stripe_types_00::payment_links::CreatePaymentLinkParamsTaxIdCollectionRequired>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsTaxIdCollection {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("required", self.required.as_ref());
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
pub struct PaymentLinksResourceAfterCompletion {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub hosted_confirmation:
        Option<stripe_types_02::payment_links::PaymentLinksResourceCompletionBehaviorConfirmationPage>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub redirect: Option<stripe_types_03::payment_links::PaymentLinksResourceCompletionBehaviorRedirect>,
    #[doc = "The specified behavior after the purchase is complete."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::payment_links::PaymentLinksResourceAfterCompletionType,
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
pub struct PaymentLinksResourceCardRestrictions {
    #[doc = "The card brands to block."]
    #[json(default)]
    pub brands_blocked: Vec<stripe_types_04::payment_links::PaymentLinksResourceCardRestrictionsBrandsBlockedItem>,
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
pub struct PaymentLinksResourceCustomFieldsDropdown {
    #[doc = "The value that pre-fills on the payment page."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[doc = "The options available for the customer to select."]
    #[json(default)]
    pub options: Vec<stripe_types_01::payment_links::PaymentLinksResourceCustomFieldsDropdownOption>,
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
pub struct PaymentLinksResourceNameCollection {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub business: Option<stripe_types_01::payment_links::PaymentLinksResourceBusinessName>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub individual: Option<stripe_types_01::payment_links::PaymentLinksResourceIndividualName>,
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
pub struct PaymentLinksResourceOptionalItem {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub adjustable_quantity: Option<stripe_types_02::payment_links::PaymentLinksResourceOptionalItemAdjustableQuantity>,
    #[json(default)]
    pub price: String,
    #[json(default)]
    pub quantity: i64,
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
pub struct PaymentLinksResourcePaymentIntentData {
    #[doc = "Indicates when the funds will be captured from the customer's account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<stripe_types_03::payment_links::PaymentLinksResourcePaymentIntentDataCaptureMethod>,
    #[doc = "An arbitrary string attached to the object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that will set metadata on Payment Intents..."]
    #[json(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "Indicates that you intend to make future payments with the payment method collected during checkout."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<stripe_types_01::payment_links::PaymentLinksResourcePaymentIntentDataSetupFutureUsage>,
    #[doc = "For a non-card payment, information about the charge that appears on the customer's statement when this payment succe..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    #[doc = "For a card payment, information about the charge that appears on the customer's statement when this payment succeeds ..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<String>,
    #[doc = "A string that identifies the resulting payment as part of a group."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
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
pub struct PaymentLinksResourcePaymentMethodReuseAgreement {
    #[doc = "Determines the position and visibility of the payment method reuse agreement in the UI."]
    #[json(default)]
    pub position: stripe_types_04::payment_links::PaymentLinksResourcePaymentMethodReuseAgreementPosition,
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
pub struct PaymentLinksResourceRestrictions {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub completed_sessions: Option<stripe_types_04::payment_links::PaymentLinksResourceCompletedSessions>,
}

#[cfg(feature = "payment_links")]
pub type UpdatePaymentLinkParamsAfterCompletionRedirect =
    stripe_types_00::payment_links::CreatePaymentLinkParamsAfterCompletionRedirect;

#[cfg(feature = "payment_links")]
pub type UpdatePaymentLinkParamsAfterCompletionType =
    stripe_types_03::payment_links::CreatePaymentLinkParamsAfterCompletionType;

#[cfg(feature = "payment_links")]
pub type UpdatePaymentLinkParamsAutomaticTaxLiabilityType =
    stripe_types_03::payment_links::CreatePaymentLinkParamsAutomaticTaxLiabilityType;

#[cfg(feature = "payment_links")]
pub type UpdatePaymentLinkParamsBillingAddressCollection =
    stripe_types_00::payment_links::CreatePaymentLinkParamsBillingAddressCollection;

#[cfg(feature = "payment_links")]
pub type UpdatePaymentLinkParamsConsentCollectionPromotions =
    stripe_types_03::payment_links::CreatePaymentLinkParamsConsentCollectionPromotions;

#[cfg(feature = "payment_links")]
pub type UpdatePaymentLinkParamsCustomTextAfterSubmit =
    stripe_types_01::payment_links::CreatePaymentLinkParamsCustomTextAfterSubmit;

#[cfg(feature = "payment_links")]
pub type UpdatePaymentLinkParamsCustomTextTermsOfServiceAcceptance =
    stripe_types_01::payment_links::CreatePaymentLinkParamsCustomTextAfterSubmit;

#[cfg(feature = "payment_links")]
pub type UpdatePaymentLinkParamsCustomerCreation =
    stripe_types_02::payment_links::CreatePaymentLinkParamsCustomerCreation;

#[cfg(feature = "payment_links")]
pub type UpdatePaymentLinkParamsInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay =
    stripe_types_01::payment_links::CreatePaymentLinkParamsInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay;

#[cfg(feature = "payment_links")]
pub type UpdatePaymentLinkParamsLineItemsItemAdjustableQuantity =
    stripe_types_01::payment_links::CreatePaymentLinkParamsLineItemsItemAdjustableQuantity;

#[cfg(feature = "payment_links")]
pub type UpdatePaymentLinkParamsNameCollectionBusiness =
    stripe_types_03::payment_links::CreatePaymentLinkParamsNameCollectionBusiness;

#[cfg(feature = "payment_links")]
pub type UpdatePaymentLinkParamsNameCollectionIndividual =
    stripe_types_03::payment_links::CreatePaymentLinkParamsNameCollectionBusiness;

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentLinkParamsPaymentIntentData {
    #[doc = "An arbitrary string attached to the object."]
    pub description: Option<json::OwnedValue>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that will declaratively set metadata on Payment Inten..."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Indicates that you intend to [make future payments](https://docs.stripe.com/payments/payment-intents#future-usage) wi..."]
    pub setup_future_usage:
        Option<stripe_types_00::payment_links::UpdatePaymentLinkParamsPaymentIntentDataSetupFutureUsage>,
    #[doc = "Text that appears on the customer's statement as the statement descriptor for a non-card charge."]
    pub statement_descriptor: Option<json::OwnedValue>,
    #[doc = "Provides information about a card charge."]
    pub statement_descriptor_suffix: Option<json::OwnedValue>,
    #[doc = "A string that identifies the resulting payment as part of a group."]
    pub transfer_group: Option<json::OwnedValue>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for UpdatePaymentLinkParamsPaymentIntentData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_links")]
pub type UpdatePaymentLinkParamsPaymentMethodCollection =
    stripe_types_02::payment_links::CreatePaymentLinkParamsCustomerCreation;

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentLinkParamsPaymentMethodOptionsCard {
    #[doc = "Restrictions to apply to the card payment method."]
    pub restrictions:
        Option<stripe_types_01::payment_links::UpdatePaymentLinkParamsPaymentMethodOptionsCardRestrictions>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for UpdatePaymentLinkParamsPaymentMethodOptionsCard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("restrictions", self.restrictions.as_ref());
    }
}

#[cfg(feature = "payment_links")]
pub type UpdatePaymentLinkParamsRestrictionsCompletedSessions =
    stripe_types_03::payment_links::CreatePaymentLinkParamsRestrictionsCompletedSessions;

#[cfg(feature = "payment_links")]
pub type UpdatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem =
    stripe_types_01::payment_links::CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem;

#[cfg(feature = "payment_links")]
pub type UpdatePaymentLinkParamsSubmitType = stripe_types_03::payment_links::CreatePaymentLinkParamsSubmitType;

#[cfg(feature = "payment_links")]
pub type UpdatePaymentLinkParamsSubscriptionDataInvoiceSettingsIssuerType =
    stripe_types_03::payment_links::CreatePaymentLinkParamsAutomaticTaxLiabilityType;
