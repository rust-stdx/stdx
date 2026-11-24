#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutAcssDebitPaymentMethodOptions {
    #[doc = "Currency supported by the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types_02::checkout::CheckoutAcssDebitPaymentMethodOptionsCurrency>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<stripe_types_05::checkout::CheckoutAcssDebitMandateOptions>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_types_04::checkout::CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage>,
    #[doc = "Controls when Stripe will attempt to debit the funds from the customer's account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub target_date: Option<String>,
    #[doc = "Bank account verification method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<stripe_types_01::checkout::CheckoutAcssDebitPaymentMethodOptionsVerificationMethod>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutCardPaymentMethodOptions {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<stripe_types_01::checkout::CheckoutAffirmPaymentMethodOptionsCaptureMethod>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub installments: Option<stripe_types_04::checkout::CheckoutCardInstallmentsOptions>,
    #[doc = "Request ability to [capture beyond the standard authorization validity window](/payments/extended-authorization) for ..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub request_extended_authorization:
        Option<stripe_types_02::checkout::CheckoutCardPaymentMethodOptionsRequestExtendedAuthorization>,
    #[doc = "Request ability to [increment the authorization](/payments/incremental-authorization) for this CheckoutSession."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub request_incremental_authorization:
        Option<stripe_types_02::checkout::CheckoutCardPaymentMethodOptionsRequestExtendedAuthorization>,
    #[doc = "Request ability to make [multiple captures](/payments/multicapture) for this CheckoutSession."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub request_multicapture:
        Option<stripe_types_02::checkout::CheckoutCardPaymentMethodOptionsRequestExtendedAuthorization>,
    #[doc = "Request ability to [overcapture](/payments/overcapture) for this CheckoutSession."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub request_overcapture:
        Option<stripe_types_02::checkout::CheckoutCardPaymentMethodOptionsRequestExtendedAuthorization>,
    #[doc = "We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based..."]
    #[json(default)]
    pub request_three_d_secure: stripe_types_03::checkout::CheckoutCardPaymentMethodOptionsRequestThreeDSecure,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub restrictions:
        Option<stripe_types_05::checkout::PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictions>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_types_04::checkout::CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage>,
    #[doc = "Provides information about a card payment that customers see on their statements."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix_kana: Option<String>,
    #[doc = "Provides information about a card payment that customers see on their statements."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix_kanji: Option<String>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutPaytoPaymentMethodOptions {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<stripe_types_06::mandates::MandateOptionsPayto>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_types_03::checkout::CheckoutAmazonPayPaymentMethodOptionsSetupFutureUsage>,
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone)]
pub struct CreateCheckoutSessionParamsBrandingSettingsLogo {
    #[doc = "The ID of a [File upload](https://stripe.com/docs/api/files) representing the logo."]
    pub file: Option<String>,
    #[doc = "The type of image for the logo."]
    pub r#type: stripe_types_06::checkout::CreateCheckoutSessionParamsBrandingSettingsLogoType,
    #[doc = "The URL of the image."]
    pub url: Option<String>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsBrandingSettingsLogo {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("file", self.file.as_ref());
        form.field("type", &self.r#type);
        form.field_opt("url", self.url.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsCustomText {
    #[doc = "Custom text that should be displayed after the payment confirmation button."]
    pub after_submit: Option<stripe_types_04::checkout::CreateCheckoutSessionParamsCustomTextAfterSubmit>,
    #[doc = "Custom text that should be displayed alongside shipping address collection."]
    pub shipping_address: Option<stripe_types_05::checkout::CreateCheckoutSessionParamsCustomTextShippingAddress>,
    #[doc = "Custom text that should be displayed alongside the payment confirmation button."]
    pub submit: Option<stripe_types_06::checkout::CreateCheckoutSessionParamsCustomTextSubmit>,
    #[doc = "Custom text that should be displayed in place of the default terms of service agreement text."]
    pub terms_of_service_acceptance:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsCustomTextTermsOfServiceAcceptance>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsCustomText {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("after_submit", self.after_submit.as_ref());
        form.field_opt("shipping_address", self.shipping_address.as_ref());
        form.field_opt("submit", self.submit.as_ref());
        form.field_opt("terms_of_service_acceptance", self.terms_of_service_acceptance.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsCustomerUpdate {
    #[doc = "Describes whether Checkout saves the billing address onto `customer.address`."]
    pub address: Option<stripe_types_02::checkout::CreateCheckoutSessionParamsCustomerUpdateAddress>,
    #[doc = "Describes whether Checkout saves the name onto `customer.name`."]
    pub name: Option<stripe_types_06::checkout::CreateCheckoutSessionParamsCustomerUpdateName>,
    #[doc = "Describes whether Checkout saves shipping information onto `customer.shipping`."]
    pub shipping: Option<stripe_types_05::checkout::CreateCheckoutSessionParamsCustomerUpdateShipping>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsCustomerUpdate {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("address", self.address.as_ref());
        form.field_opt("name", self.name.as_ref());
        form.field_opt("shipping", self.shipping.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone)]
pub struct CreateCheckoutSessionParamsInvoiceCreationInvoiceDataIssuer {
    #[doc = "The connected account being referenced when `type` is `account`."]
    pub account: Option<String>,
    #[doc = "Type of the account referenced in the request."]
    pub r#type: stripe_types_06::checkout::CreateCheckoutSessionParamsInvoiceCreationInvoiceDataIssuerType,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsInvoiceCreationInvoiceDataIssuer {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsNameCollection {
    #[doc = "Controls settings applied for collecting the customer's business name on the session."]
    pub business: Option<stripe_types_04::checkout::CreateCheckoutSessionParamsNameCollectionBusiness>,
    #[doc = "Controls settings applied for collecting the customer's individual name on the session."]
    pub individual: Option<stripe_types_06::checkout::CreateCheckoutSessionParamsNameCollectionIndividual>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsNameCollection {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("business", self.business.as_ref());
        form.field_opt("individual", self.individual.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsOptionalItemsItem {
    #[doc = "When set, provides configuration for the customer to adjust the quantity of the line item created when a customer cho..."]
    pub adjustable_quantity:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsOptionalItemsItemAdjustableQuantity>,
    #[doc = "The ID of the [Price](https://docs.stripe.com/api/prices) or [Plan](https://docs.stripe.com/api/plans) object."]
    pub price: String,
    #[doc = "The initial quantity of the line item created when a customer chooses to add this optional item to their order."]
    pub quantity: i64,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsOptionalItemsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("adjustable_quantity", self.adjustable_quantity.as_ref());
        form.field("price", &self.price);
        form.field("quantity", &self.quantity);
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsAuBecsDebit {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAuBecsDebitSetupFutureUsage>,
    #[doc = "Controls when Stripe will attempt to debit the funds from the customer's account."]
    pub target_date: Option<String>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsAuBecsDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("target_date", self.target_date.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsBacsDebit {
    #[doc = "Additional fields for Mandate creation"]
    pub mandate_options:
        Option<stripe_types_02::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsBacsDebitMandateOptions>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsBacsDebitSetupFutureUsage>,
    #[doc = "Controls when Stripe will attempt to debit the funds from the customer's account."]
    pub target_date: Option<String>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsBacsDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("target_date", self.target_date.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsBancontact {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsBancontactSetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsBancontact {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsCashapp {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsCashappCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsCashappSetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsCashapp {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsCrypto {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsCryptoSetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsCrypto {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsEps {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsEpsSetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsEps {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsGrabpay {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsGrabpaySetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsGrabpay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsIdeal {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsIdealSetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsIdeal {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsKrCard {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsKrCardCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsKrCardSetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsKrCard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsLink {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsLinkCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsLinkSetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsLink {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsMobilepay {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsMobilepayCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsMobilepaySetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsMobilepay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsP24 {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsP24SetupFutureUsage>,
    #[doc = "Confirm that the payer has accepted the P24 terms and conditions."]
    pub tos_shown_and_accepted: Option<bool>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsP24 {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("tos_shown_and_accepted", self.tos_shown_and_accepted.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsSatispay {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsSatispayCaptureMethod>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsSatispay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsScalapay {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsScalapayCaptureMethod>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsScalapay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsSequra {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsSequraCaptureMethod>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsSequra {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsTwint {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsTwintSetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsTwint {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsWechatPay {
    #[doc = "The app ID registered with WeChat Pay."]
    pub app_id: Option<String>,
    #[doc = "The client type that the end customer will pay from"]
    pub client: stripe_types_02::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsWechatPayClient,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsWechatPaySetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsWechatPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("app_id", self.app_id.as_ref());
        form.field("client", &self.client);
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone)]
pub struct CreateCheckoutSessionParamsSubscriptionDataTrialSettings {
    #[doc = "Defines how the subscription should behave when the user's free trial ends."]
    pub end_behavior: stripe_types_06::checkout::CreateCheckoutSessionParamsSubscriptionDataTrialSettingsEndBehavior,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsSubscriptionDataTrialSettings {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("end_behavior", &self.end_behavior);
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
pub struct PaymentPagesCheckoutSessionCollectedInformation {
    #[doc = "Customer’s business name for this Checkout Session"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    #[doc = "Customer’s individual name for this Checkout Session"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub individual_name: Option<String>,
    #[doc = "Shipping information for this Checkout Session."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping_details: Option<stripe_types_05::checkout::PaymentPagesCheckoutSessionCheckoutAddressDetails>,
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct UpdateCheckoutSessionParamsCollectedInformation {
    #[doc = "The shipping details to apply to this Session."]
    pub shipping_details:
        Option<stripe_types_06::checkout::UpdateCheckoutSessionParamsCollectedInformationShippingDetails>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for UpdateCheckoutSessionParamsCollectedInformation {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("shipping_details", self.shipping_details.as_ref());
    }
}
