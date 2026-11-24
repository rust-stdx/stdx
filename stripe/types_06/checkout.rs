#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutAffirmPaymentMethodOptions {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<stripe_types_01::checkout::CheckoutAffirmPaymentMethodOptionsCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_types_03::checkout::CheckoutAffirmPaymentMethodOptionsSetupFutureUsage>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutAlmaPaymentMethodOptions {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<stripe_types_01::checkout::CheckoutAffirmPaymentMethodOptionsCaptureMethod>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutAmazonPayPaymentMethodOptions {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<stripe_types_01::checkout::CheckoutAffirmPaymentMethodOptionsCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_types_03::checkout::CheckoutAmazonPayPaymentMethodOptionsSetupFutureUsage>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutBoletoPaymentMethodOptions {
    #[doc = "The number of calendar days before a Boleto voucher expires."]
    #[json(default)]
    pub expires_after_days: i64,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_types_04::checkout::CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutCashappPaymentMethodOptions {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<stripe_types_01::checkout::CheckoutAffirmPaymentMethodOptionsCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_types_03::checkout::CheckoutAffirmPaymentMethodOptionsSetupFutureUsage>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutEpsPaymentMethodOptions {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_types_03::checkout::CheckoutAffirmPaymentMethodOptionsSetupFutureUsage>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutFinancialConnectionsPaymentMethodOptionsFilters {
    #[doc = "The account subcategories to use to filter for possible accounts to link."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_subcategories: Option<
        Vec<stripe_types_00::checkout::CheckoutFinancialConnectionsPaymentMethodOptionsFiltersAccountSubcategoriesItem>,
    >,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutGiropayPaymentMethodOptions {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_types_03::checkout::CheckoutAffirmPaymentMethodOptionsSetupFutureUsage>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutIdealPaymentMethodOptions {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_types_03::checkout::CheckoutAffirmPaymentMethodOptionsSetupFutureUsage>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutKlarnaPaymentMethodOptions {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<stripe_types_01::checkout::CheckoutAffirmPaymentMethodOptionsCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_types_04::checkout::CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutKonbiniPaymentMethodOptions {
    #[doc = "The number of calendar days (between 1 and 60) after which Konbini payment instructions will expire."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<i64>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_types_03::checkout::CheckoutAffirmPaymentMethodOptionsSetupFutureUsage>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutMobilepayPaymentMethodOptions {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<stripe_types_01::checkout::CheckoutAffirmPaymentMethodOptionsCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_types_03::checkout::CheckoutAffirmPaymentMethodOptionsSetupFutureUsage>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutMultibancoPaymentMethodOptions {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_types_03::checkout::CheckoutAffirmPaymentMethodOptionsSetupFutureUsage>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutNaverPayPaymentMethodOptions {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<stripe_types_01::checkout::CheckoutAffirmPaymentMethodOptionsCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_types_03::checkout::CheckoutAmazonPayPaymentMethodOptionsSetupFutureUsage>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutP24PaymentMethodOptions {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_types_03::checkout::CheckoutAffirmPaymentMethodOptionsSetupFutureUsage>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutPaynowPaymentMethodOptions {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_types_03::checkout::CheckoutAffirmPaymentMethodOptionsSetupFutureUsage>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutPaypalPaymentMethodOptions {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<stripe_types_01::checkout::CheckoutAffirmPaymentMethodOptionsCaptureMethod>,
    #[doc = "Preferred locale of the PayPal checkout page that the customer is redirected to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub preferred_locale: Option<String>,
    #[doc = "A reference of the PayPal transaction visible to customer which is mapped to PayPal's invoice ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_types_03::checkout::CheckoutAmazonPayPaymentMethodOptionsSetupFutureUsage>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutSamsungPayPaymentMethodOptions {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<stripe_types_01::checkout::CheckoutAffirmPaymentMethodOptionsCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_types_03::checkout::CheckoutAffirmPaymentMethodOptionsSetupFutureUsage>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutSatispayPaymentMethodOptions {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<stripe_types_01::checkout::CheckoutAffirmPaymentMethodOptionsCaptureMethod>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutSepaDebitPaymentMethodOptions {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<stripe_types_01::checkout::CheckoutPaymentMethodOptionsMandateOptionsSepaDebit>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_types_04::checkout::CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage>,
    #[doc = "Controls when Stripe will attempt to debit the funds from the customer's account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub target_date: Option<String>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutSequraPaymentMethodOptions {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<stripe_types_01::checkout::CheckoutAffirmPaymentMethodOptionsCaptureMethod>,
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone)]
pub struct CreateCheckoutSessionParamsBrandingSettingsIcon {
    #[doc = "The ID of a [File upload](https://stripe.com/docs/api/files) representing the icon."]
    pub file: Option<String>,
    #[doc = "The type of image for the icon."]
    pub r#type: stripe_types_03::checkout::CreateCheckoutSessionParamsBrandingSettingsIconType,
    #[doc = "The URL of the image."]
    pub url: Option<String>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsBrandingSettingsIcon {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("file", self.file.as_ref());
        form.field("type", &self.r#type);
        form.field_opt("url", self.url.as_ref());
    }
}

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsBrandingSettingsLogoType =
    stripe_types_03::checkout::CreateCheckoutSessionParamsBrandingSettingsIconType;

#[cfg(feature = "checkout")]
#[derive(Debug, Clone)]
pub struct CreateCheckoutSessionParamsConsentCollectionPaymentMethodReuseAgreement {
    #[doc = "Determines the position and visibility of the payment method reuse agreement in the UI."]
    pub position:
        stripe_types_01::checkout::CreateCheckoutSessionParamsConsentCollectionPaymentMethodReuseAgreementPosition,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsConsentCollectionPaymentMethodReuseAgreement {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("position", &self.position);
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsCustomFieldsItemDropdown {
    #[doc = "The value that pre-fills the field on the payment page."]
    pub default_value: Option<String>,
    #[doc = "The options available for the customer to select."]
    pub options: Vec<stripe_types_02::checkout::CreateCheckoutSessionParamsCustomFieldsItemDropdownOptionsItem>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsCustomFieldsItemDropdown {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("default_value", self.default_value.as_ref());
        for (index, item) in self.options.iter().enumerate() {
            form.field_indexed("options", index, item);
        }
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone)]
pub struct CreateCheckoutSessionParamsCustomFieldsItemLabel {
    #[doc = "Custom text for the label, displayed to the customer."]
    pub custom: String,
    #[doc = "The type of the label."]
    pub r#type: stripe_types_01::checkout::CreateCheckoutSessionParamsCustomFieldsItemLabelType,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsCustomFieldsItemLabel {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("custom", &self.custom);
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsCustomFieldsItemText =
    stripe_types_00::checkout::CreateCheckoutSessionParamsCustomFieldsItemNumeric;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsCustomTextSubmit =
    stripe_types_04::checkout::CreateCheckoutSessionParamsCustomTextAfterSubmit;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsCustomTextTermsOfServiceAcceptance =
    stripe_types_04::checkout::CreateCheckoutSessionParamsCustomTextAfterSubmit;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsCustomerUpdateName =
    stripe_types_02::checkout::CreateCheckoutSessionParamsCustomerUpdateAddress;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsInvoiceCreationInvoiceDataIssuerType =
    stripe_types_00::checkout::CreateCheckoutSessionParamsAutomaticTaxLiabilityType;

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsInvoiceCreationInvoiceDataRenderingOptions {
    #[doc = "How line-item prices and amounts will be displayed with respect to tax on invoice PDFs."]
    pub amount_tax_display: Option<stripe_types_03::checkout::CreateCheckoutSessionParamsInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay>,
    #[doc = "ID of the invoice rendering template to use for this invoice."]
    pub template: Option<String>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsInvoiceCreationInvoiceDataRenderingOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount_tax_display", self.amount_tax_display.as_ref());
        form.field_opt("template", self.template.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsLineItemsItemPriceDataProductData {
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
        Option<stripe_types_01::checkout::CreateCheckoutSessionParamsLineItemsItemPriceDataProductDataTaxDetails>,
    #[doc = "A label that represents units of this product."]
    pub unit_label: Option<String>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsLineItemsItemPriceDataProductData {
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

#[cfg(feature = "checkout")]
#[derive(Debug, Clone)]
pub struct CreateCheckoutSessionParamsLineItemsItemPriceDataRecurring {
    #[doc = "Specifies billing frequency."]
    pub interval: stripe_types_01::checkout::CreateCheckoutSessionParamsLineItemsItemPriceDataRecurringInterval,
    #[doc = "The number of intervals between subscription billings."]
    pub interval_count: Option<i64>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsLineItemsItemPriceDataRecurring {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("interval", &self.interval);
        form.field_opt("interval_count", self.interval_count.as_ref());
    }
}

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsManagedPayments =
    stripe_types_00::checkout::CreateCheckoutSessionParamsAdaptivePricing;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsNameCollectionIndividual =
    stripe_types_04::checkout::CreateCheckoutSessionParamsNameCollectionBusiness;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsOptionalItemsItemAdjustableQuantity =
    stripe_types_04::checkout::CreateCheckoutSessionParamsLineItemsItemAdjustableQuantity;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsAlipaySetupFutureUsage =
    stripe_types_02::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsAlmaCaptureMethod =
    stripe_types_01::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsBacsDebitSetupFutureUsage =
    stripe_types_04::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitSetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsBancontactSetupFutureUsage =
    stripe_types_04::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAmazonPaySetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestMulticapture =
    stripe_types_00::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestExtendedAuthorization;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestOvercapture =
    stripe_types_00::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestExtendedAuthorization;

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsCardRestrictions {
    #[doc = "The card brands to block."]
    pub brands_blocked: Option<Vec<stripe_types_00::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsCardRestrictionsBrandsBlockedItem>>,
    #[doc = "Card funding types to block for this Checkout Session."]
    pub funding_types_blocked: Option<Vec<stripe_types_02::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsCardRestrictionsFundingTypesBlockedItem>>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsCardRestrictions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.brands_blocked.as_ref() {
            form.field_array("brands_blocked", values.as_slice());
        }
        if let Some(values) = self.funding_types_blocked.as_ref() {
            form.field_array("funding_types_blocked", values.as_slice());
        }
    }
}

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsCashappCaptureMethod =
    stripe_types_01::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsCashappSetupFutureUsage =
    stripe_types_04::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitSetupFutureUsage;

#[cfg(feature = "checkout")]
#[derive(Debug, Clone)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransfer {
    #[doc = "Configuration for eu_bank_transfer funding type."]
    pub eu_bank_transfer: Option<stripe_types_01::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer>,
    #[doc = "List of address types that should be returned in the financial_addresses response."]
    pub requested_address_types: Option<Vec<stripe_types_01::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem>>,
    #[doc = "The list of bank transfer types that this PaymentIntent is allowed to use for funding."]
    pub r#type: stripe_types_03::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransferType,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransfer {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("eu_bank_transfer", self.eu_bank_transfer.as_ref());
        if let Some(values) = self.requested_address_types.as_ref() {
            form.field_array("requested_address_types", values.as_slice());
        }
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceSetupFutureUsage =
    stripe_types_02::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsEpsSetupFutureUsage =
    stripe_types_02::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsGrabpaySetupFutureUsage =
    stripe_types_02::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsIdealSetupFutureUsage =
    stripe_types_02::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsKakaoPaySetupFutureUsage =
    stripe_types_04::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAmazonPaySetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsKlarnaCaptureMethod =
    stripe_types_01::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsKlarnaSetupFutureUsage =
    stripe_types_02::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsKrCardSetupFutureUsage =
    stripe_types_04::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAmazonPaySetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsLinkCaptureMethod =
    stripe_types_01::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsLinkSetupFutureUsage =
    stripe_types_04::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAmazonPaySetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsMobilepayCaptureMethod =
    stripe_types_01::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsMultibancoSetupFutureUsage =
    stripe_types_02::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsNaverPayCaptureMethod =
    stripe_types_01::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsPaycoSetupFutureUsage =
    stripe_types_02::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsPaypalSetupFutureUsage =
    stripe_types_00::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsBlikSetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsPaytoSetupFutureUsage =
    stripe_types_04::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAmazonPaySetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsPixSetupFutureUsage =
    stripe_types_04::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAmazonPaySetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsRevolutPaySetupFutureUsage =
    stripe_types_04::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAmazonPaySetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsSamsungPayCaptureMethod =
    stripe_types_01::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsScalapayCaptureMethod =
    stripe_types_01::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsSequraCaptureMethod =
    stripe_types_01::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsSunbitSetupFutureUsage =
    stripe_types_02::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsTwintSetupFutureUsage =
    stripe_types_04::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAmazonPaySetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsUpiMandateOptionsAmountType =
    stripe_types_00::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptionsAmountType;

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountFinancialConnections {
    #[doc = "The list of permissions to request."]
    pub permissions: Option<Vec<stripe_types_01::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem>>,
    #[doc = "List of data features that you would like to retrieve upon account creation."]
    pub prefetch: Option<Vec<stripe_types_04::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem>>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountFinancialConnections {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.permissions.as_ref() {
            form.field_array("permissions", values.as_slice());
        }
        if let Some(values) = self.prefetch.as_ref() {
            form.field_array("prefetch", values.as_slice());
        }
    }
}

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsWechatPaySetupFutureUsage =
    stripe_types_02::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPermissions {
    #[doc = "Determines which entity is allowed to update the shipping details."]
    pub update_shipping_details:
        Option<stripe_types_04::checkout::CreateCheckoutSessionParamsPermissionsUpdateShippingDetails>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPermissions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("update_shipping_details", self.update_shipping_details.as_ref());
    }
}

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsSavedPaymentMethodOptionsAllowRedisplayFiltersItem =
    stripe_types_01::checkout::CreateCheckoutSessionParamsPaymentMethodDataAllowRedisplay;

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsShippingAddressCollection {
    #[doc = "An array of two-letter ISO country codes representing which countries Checkout should provide as options for shipping..."]
    pub allowed_countries:
        Vec<stripe_types_03::checkout::CreateCheckoutSessionParamsShippingAddressCollectionAllowedCountriesItem>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsShippingAddressCollection {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_array("allowed_countries", self.allowed_countries.as_slice());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone)]
pub struct CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataDeliveryEstimateMaximum {
    #[doc = "A unit of time."]
    pub unit: stripe_types_02::checkout::CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataDeliveryEstimateMaximumUnit,
    #[doc = "Must be greater than 0."]
    pub value: i64,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode
    for CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataDeliveryEstimateMaximum
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("unit", &self.unit);
        form.field("value", &self.value);
    }
}

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsSubscriptionDataPendingInvoiceItemIntervalInterval =
    stripe_types_01::checkout::CreateCheckoutSessionParamsLineItemsItemPriceDataRecurringInterval;

#[cfg(feature = "checkout")]
#[derive(Debug, Clone)]
pub struct CreateCheckoutSessionParamsSubscriptionDataTrialSettingsEndBehavior {
    #[doc = "Indicates how the subscription should change when the trial ends if the user did not provide a payment method."]
    pub missing_payment_method: stripe_types_03::checkout::CreateCheckoutSessionParamsSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsSubscriptionDataTrialSettingsEndBehavior {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("missing_payment_method", &self.missing_payment_method);
    }
}

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsWalletOptionsLinkDisplay =
    stripe_types_02::checkout::CreateCheckoutSessionParamsCustomerUpdateAddress;

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct ListCheckoutSessionsParams {
    pub created: Option<stripe_types_00::checkout::ListCheckoutSessionsParamsCreated>,
    pub customer: Option<String>,
    pub customer_account: Option<String>,
    pub customer_details: Option<stripe_types_00::checkout::ListCheckoutSessionsParamsCustomerDetails>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub payment_intent: Option<String>,
    pub payment_link: Option<String>,
    pub starting_after: Option<String>,
    pub status: Option<stripe_types_01::checkout::ListCheckoutSessionsParamsStatus>,
    pub subscription: Option<String>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for ListCheckoutSessionsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("created", self.created.as_ref());
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("customer_account", self.customer_account.as_ref());
        form.field_opt("customer_details", self.customer_details.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("payment_intent", self.payment_intent.as_ref());
        form.field_opt("payment_link", self.payment_link.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("status", self.status.as_ref());
        form.field_opt("subscription", self.subscription.as_ref());
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentPagesCheckoutSessionBrandingSettingsIcon {
    #[doc = "The ID of a [File upload](https://stripe.com/docs/api/files) representing the icon."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[doc = "The type of image for the icon."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::checkout::PaymentPagesCheckoutSessionBrandingSettingsIconType,
    #[doc = "The URL of the image."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentPagesCheckoutSessionCustomFieldsDropdown {
    #[doc = "The value that pre-fills on the payment page."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[doc = "The options available for the customer to select."]
    #[json(default)]
    pub options: Vec<stripe_types_02::checkout::PaymentPagesCheckoutSessionCustomFieldsOption>,
    #[doc = "The option selected by the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentPagesCheckoutSessionCustomFieldsLabel {
    #[doc = "Custom text for the label, displayed to the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<String>,
    #[doc = "The type of the label."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::checkout::PaymentPagesCheckoutSessionCustomFieldsLabelType,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentPagesCheckoutSessionOptionalItem {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub adjustable_quantity:
        Option<stripe_types_00::checkout::PaymentPagesCheckoutSessionOptionalItemAdjustableQuantity>,
    #[json(default)]
    pub price: String,
    #[json(default)]
    pub quantity: i64,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentPagesCheckoutSessionPermissions {
    #[doc = "Determines which entity is allowed to update the shipping details."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub update_shipping_details:
        Option<stripe_types_04::checkout::PaymentPagesCheckoutSessionPermissionsUpdateShippingDetails>,
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone)]
pub struct UpdateCheckoutSessionParamsCollectedInformationShippingDetails {
    #[doc = "The address of the customer"]
    pub address: stripe_types_03::checkout::UpdateCheckoutSessionParamsCollectedInformationShippingDetailsAddress,
    #[doc = "The name of customer"]
    pub name: String,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for UpdateCheckoutSessionParamsCollectedInformationShippingDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("address", &self.address);
        form.field("name", &self.name);
    }
}

#[cfg(feature = "checkout")]
pub type UpdateCheckoutSessionParamsLineItemsItemPriceDataProductDataTaxDetails =
    stripe_types_01::checkout::CreateCheckoutSessionParamsLineItemsItemPriceDataProductDataTaxDetails;

#[cfg(feature = "checkout")]
pub type UpdateCheckoutSessionParamsLineItemsItemPriceDataRecurringInterval =
    stripe_types_01::checkout::CreateCheckoutSessionParamsLineItemsItemPriceDataRecurringInterval;

#[cfg(feature = "checkout")]
pub type UpdateCheckoutSessionParamsLineItemsItemPriceDataTaxBehavior =
    stripe_types_03::checkout::CreateCheckoutSessionParamsLineItemsItemPriceDataTaxBehavior;
