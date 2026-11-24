#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutSessionWalletOptions {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<stripe_types_05::checkout::CheckoutLinkWalletOptions>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutUpiPaymentMethodOptions {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<stripe_types_06::mandates::MandateOptionsUpi>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_types_04::checkout::CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage>,
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsConsentCollection {
    #[doc = "Determines the display of payment method reuse agreement text in the UI."]
    pub payment_method_reuse_agreement:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsConsentCollectionPaymentMethodReuseAgreement>,
    #[doc = "If set to `auto`, enables the collection of customer consent for promotional communications."]
    pub promotions: Option<stripe_types_00::checkout::CreateCheckoutSessionParamsConsentCollectionPromotions>,
    #[doc = "If set to `required`, it requires customers to check a terms of service checkbox before being able to pay."]
    pub terms_of_service: Option<stripe_types_03::checkout::CreateCheckoutSessionParamsConsentCollectionTermsOfService>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsConsentCollection {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("payment_method_reuse_agreement", self.payment_method_reuse_agreement.as_ref());
        form.field_opt("promotions", self.promotions.as_ref());
        form.field_opt("terms_of_service", self.terms_of_service.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone)]
pub struct CreateCheckoutSessionParamsCustomFieldsItem {
    #[doc = "Configuration for `type=dropdown` fields."]
    pub dropdown: Option<stripe_types_06::checkout::CreateCheckoutSessionParamsCustomFieldsItemDropdown>,
    #[doc = "String of your choice that your integration can use to reconcile this field."]
    pub key: String,
    #[doc = "The label for the field, displayed to the customer."]
    pub label: stripe_types_06::checkout::CreateCheckoutSessionParamsCustomFieldsItemLabel,
    #[doc = "Configuration for `type=numeric` fields."]
    pub numeric: Option<stripe_types_00::checkout::CreateCheckoutSessionParamsCustomFieldsItemNumeric>,
    #[doc = "Whether the customer is required to complete the field before completing the Checkout Session."]
    pub optional: Option<bool>,
    #[doc = "Configuration for `type=text` fields."]
    pub text: Option<stripe_types_06::checkout::CreateCheckoutSessionParamsCustomFieldsItemText>,
    #[doc = "The type of the field."]
    pub r#type: stripe_types_03::checkout::CreateCheckoutSessionParamsCustomFieldsItemType,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsCustomFieldsItem {
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

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsAlipay {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAlipaySetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsAlipay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsAmazonPay {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAmazonPayCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_04::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAmazonPaySetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsAmazonPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalance {
    #[doc = "Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`."]
    pub bank_transfer:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceBankTransfer>,
    #[doc = "The funding method type to be used when there are not enough funds in the customer balance."]
    pub funding_type:
        Option<stripe_types_02::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceFundingType>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceSetupFutureUsage,
    >,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalance {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("bank_transfer", self.bank_transfer.as_ref());
        form.field_opt("funding_type", self.funding_type.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsFpx {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsFpxSetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsFpx {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsGiropay {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsGiropaySetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsGiropay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsKlarna {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsKlarnaCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsKlarnaSetupFutureUsage>,
    #[doc = "Subscription details if the Checkout Session sets up a future subscription."]
    pub subscriptions: Option<json::OwnedValue>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsKlarna {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsKonbini {
    #[doc = "The number of calendar days (between 1 and 60) after which Konbini payment instructions will expire."]
    pub expires_after_days: Option<i64>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsKonbiniSetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsKonbini {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("expires_after_days", self.expires_after_days.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsMultibanco {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsMultibancoSetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsMultibanco {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsOxxo {
    #[doc = "The number of calendar days before an OXXO voucher expires."]
    pub expires_after_days: Option<i64>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsOxxoSetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsOxxo {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("expires_after_days", self.expires_after_days.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsPaypal {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_02::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsPaypalCaptureMethod>,
    #[doc = "[Preferred locale](https://docs.stripe.com/payments/paypal/supported-locales) of the PayPal checkout page that the cu..."]
    pub preferred_locale:
        Option<stripe_types_00::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsPaypalPreferredLocale>,
    #[doc = "A reference of the PayPal transaction visible to customer which is mapped to PayPal's invoice ID."]
    pub reference: Option<String>,
    #[doc = "The risk correlation ID for an on-session payment using a saved PayPal payment method."]
    pub risk_correlation_id: Option<String>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsPaypalSetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsPaypal {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("preferred_locale", self.preferred_locale.as_ref());
        form.field_opt("reference", self.reference.as_ref());
        form.field_opt("risk_correlation_id", self.risk_correlation_id.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsSofort {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsSofortSetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsSofort {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone)]
pub struct CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataDeliveryEstimateMinimum {
    #[doc = "A unit of time."]
    pub unit: stripe_types_05::checkout::CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataDeliveryEstimateMinimumUnit,
    #[doc = "Must be greater than 0."]
    pub value: i64,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode
    for CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataDeliveryEstimateMinimum
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("unit", &self.unit);
        form.field("value", &self.value);
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataFixedAmountCurrencyOptionsValue {
    #[doc = "A non-negative integer in cents representing how much to charge."]
    pub amount: i64,
    #[doc = "Specifies whether the rate is considered inclusive of taxes or exclusive of taxes."]
    pub tax_behavior: Option<stripe_types_05::checkout::CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataFixedAmountCurrencyOptionsValueTaxBehavior>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode
    for CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataFixedAmountCurrencyOptionsValue
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone)]
pub struct CreateCheckoutSessionParamsSubscriptionDataBillingMode {
    #[doc = "Configure behavior for flexible billing mode."]
    pub flexible: Option<stripe_types_05::checkout::CreateCheckoutSessionParamsSubscriptionDataBillingModeFlexible>,
    #[doc = "Controls the calculation and orchestration of prorations and invoices for subscriptions."]
    pub r#type: stripe_types_02::checkout::CreateCheckoutSessionParamsSubscriptionDataBillingModeType,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsSubscriptionDataBillingMode {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("flexible", self.flexible.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone)]
pub struct CreateCheckoutSessionParamsSubscriptionDataInvoiceSettingsIssuer {
    #[doc = "The connected account being referenced when `type` is `account`."]
    pub account: Option<String>,
    #[doc = "Type of the account referenced in the request."]
    pub r#type: stripe_types_05::checkout::CreateCheckoutSessionParamsSubscriptionDataInvoiceSettingsIssuerType,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsSubscriptionDataInvoiceSettingsIssuer {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsWalletOptionsLink {
    #[doc = "Specifies whether Checkout should display Link as a payment option."]
    pub display: Option<stripe_types_06::checkout::CreateCheckoutSessionParamsWalletOptionsLinkDisplay>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsWalletOptionsLink {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display", self.display.as_ref());
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
pub struct PaymentPagesCheckoutSessionConsentCollection {
    #[doc = "If set to `hidden`, it will hide legal text related to the reuse of a payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_reuse_agreement:
        Option<stripe_types_05::checkout::PaymentPagesCheckoutSessionPaymentMethodReuseAgreement>,
    #[doc = "If set to `auto`, enables the collection of customer consent for promotional communications."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub promotions: Option<stripe_types_03::checkout::PaymentPagesCheckoutSessionConsentCollectionPromotions>,
    #[doc = "If set to `required`, it requires customers to accept the terms of service before being able to pay."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<stripe_types_03::checkout::PaymentPagesCheckoutSessionConsentCollectionTermsOfService>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentPagesCheckoutSessionCustomFields {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub dropdown: Option<stripe_types_06::checkout::PaymentPagesCheckoutSessionCustomFieldsDropdown>,
    #[doc = "String of your choice that your integration can use to reconcile this field."]
    #[json(default)]
    pub key: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<stripe_types_06::checkout::PaymentPagesCheckoutSessionCustomFieldsLabel>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub numeric: Option<stripe_types_02::checkout::PaymentPagesCheckoutSessionCustomFieldsNumeric>,
    #[doc = "Whether the customer is required to complete the field before completing the Checkout Session."]
    #[json(default)]
    pub optional: bool,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<stripe_types_04::checkout::PaymentPagesCheckoutSessionCustomFieldsText>,
    #[doc = "The type of the field."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::checkout::PaymentPagesCheckoutSessionCustomFieldsType,
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct UpdateCheckoutSessionParamsLineItemsItemPriceDataProductData {
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
        Option<stripe_types_06::checkout::UpdateCheckoutSessionParamsLineItemsItemPriceDataProductDataTaxDetails>,
    #[doc = "A label that represents units of this product."]
    pub unit_label: Option<String>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for UpdateCheckoutSessionParamsLineItemsItemPriceDataProductData {
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
pub struct UpdateCheckoutSessionParamsLineItemsItemPriceDataRecurring {
    #[doc = "Specifies billing frequency."]
    pub interval: stripe_types_06::checkout::UpdateCheckoutSessionParamsLineItemsItemPriceDataRecurringInterval,
    #[doc = "The number of intervals between subscription billings."]
    pub interval_count: Option<i64>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for UpdateCheckoutSessionParamsLineItemsItemPriceDataRecurring {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("interval", &self.interval);
        form.field_opt("interval_count", self.interval_count.as_ref());
    }
}
