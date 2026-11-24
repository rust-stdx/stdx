#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutAcssDebitMandateOptions {
    #[doc = "A URL for custom mandate text"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<String>,
    #[doc = "List of Stripe products where this mandate can be selected automatically."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub default_for: Option<Vec<stripe_types_00::checkout::CheckoutAcssDebitMandateOptionsDefaultForItem>>,
    #[doc = "Description of the interval."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<String>,
    #[doc = "Payment schedule for the mandate."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_schedule: Option<stripe_types_00::checkout::CheckoutAcssDebitMandateOptionsPaymentSchedule>,
    #[doc = "Transaction type of the mandate."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<stripe_types_01::checkout::CheckoutAcssDebitMandateOptionsTransactionType>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutAfterpayClearpayPaymentMethodOptions {
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
pub struct CheckoutAlipayPaymentMethodOptions {
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
pub struct CheckoutAuBecsDebitPaymentMethodOptions {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_types_03::checkout::CheckoutAffirmPaymentMethodOptionsSetupFutureUsage>,
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
pub struct CheckoutBacsDebitPaymentMethodOptions {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<stripe_types_00::checkout::CheckoutPaymentMethodOptionsMandateOptionsBacsDebit>,
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
pub struct CheckoutBancontactPaymentMethodOptions {
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
pub struct CheckoutBilliePaymentMethodOptions {
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
pub struct CheckoutFpxPaymentMethodOptions {
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
pub struct CheckoutGrabPayPaymentMethodOptions {
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
pub struct CheckoutKakaoPayPaymentMethodOptions {
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
pub struct CheckoutKrCardPaymentMethodOptions {
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
pub struct CheckoutLinkPaymentMethodOptions {
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
pub struct CheckoutLinkWalletOptions {
    #[doc = "Describes whether Checkout should display Link."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<stripe_types_01::checkout::CheckoutLinkWalletOptionsDisplay>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutOxxoPaymentMethodOptions {
    #[doc = "The number of calendar days before an OXXO invoice expires."]
    #[json(default)]
    pub expires_after_days: i64,
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
pub struct CheckoutPaycoPaymentMethodOptions {
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
pub struct CheckoutRevolutPayPaymentMethodOptions {
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
pub struct CheckoutScalapayPaymentMethodOptions {
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
pub struct CheckoutSofortPaymentMethodOptions {
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
pub struct CheckoutSunbitPaymentMethodOptions {
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
pub struct CheckoutTwintPaymentMethodOptions {
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
pub struct CheckoutWechatPayPaymentMethodOptions {
    #[doc = "The app ID registered with WeChat Pay."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[doc = "The client type that the end customer will pay from"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub client: Option<stripe_types_01::checkout::CheckoutWechatPayPaymentMethodOptionsClient>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_types_03::checkout::CheckoutAffirmPaymentMethodOptionsSetupFutureUsage>,
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsAfterExpiration {
    #[doc = "Configure a Checkout Session that can be used to recover an expired session."]
    pub recovery: Option<stripe_types_03::checkout::CreateCheckoutSessionParamsAfterExpirationRecovery>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsAfterExpiration {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("recovery", self.recovery.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone)]
pub struct CreateCheckoutSessionParamsAutomaticTaxLiability {
    #[doc = "The connected account being referenced when `type` is `account`."]
    pub account: Option<String>,
    #[doc = "Type of the account referenced in the request."]
    pub r#type: stripe_types_00::checkout::CreateCheckoutSessionParamsAutomaticTaxLiabilityType,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsAutomaticTaxLiability {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsCustomTextShippingAddress =
    stripe_types_04::checkout::CreateCheckoutSessionParamsCustomTextAfterSubmit;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsCustomerUpdateShipping =
    stripe_types_02::checkout::CreateCheckoutSessionParamsCustomerUpdateAddress;

#[cfg(feature = "checkout")]
#[derive(Debug, Clone)]
pub struct CreateCheckoutSessionParamsPaymentIntentDataShipping {
    #[doc = "Shipping address."]
    pub address: stripe_types_02::checkout::CreateCheckoutSessionParamsPaymentIntentDataShippingAddress,
    #[doc = "The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc."]
    pub carrier: Option<String>,
    #[doc = "Recipient name."]
    pub name: String,
    #[doc = "Recipient phone (including extension)."]
    pub phone: Option<String>,
    #[doc = "The tracking number for a physical product, obtained from the delivery service."]
    pub tracking_number: Option<String>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentIntentDataShipping {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("address", &self.address);
        form.field_opt("carrier", self.carrier.as_ref());
        form.field("name", &self.name);
        form.field_opt("phone", self.phone.as_ref());
        form.field_opt("tracking_number", self.tracking_number.as_ref());
    }
}

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodCollection =
    stripe_types_02::checkout::CreateCheckoutSessionParamsCustomerCreation;

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodData {
    #[doc = "Allow redisplay will be set on the payment method on confirmation and indicates whether this payment method can be sh..."]
    pub allow_redisplay: Option<stripe_types_01::checkout::CreateCheckoutSessionParamsPaymentMethodDataAllowRedisplay>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("allow_redisplay", self.allow_redisplay.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptions {
    #[doc = "A URL for custom mandate text to render during confirmation step."]
    pub custom_mandate_url: Option<json::OwnedValue>,
    #[doc = "List of Stripe products where this mandate can be selected automatically."]
    pub default_for: Option<Vec<stripe_types_03::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsDefaultForItem>>,
    #[doc = "Description of the mandate interval."]
    pub interval_description: Option<String>,
    #[doc = "Payment schedule for the mandate."]
    pub payment_schedule: Option<stripe_types_03::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule>,
    #[doc = "Transaction type of the mandate."]
    pub transaction_type: Option<stripe_types_04::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.default_for.as_ref() {
            form.field_array("default_for", values.as_slice());
        }
        form.field_opt("interval_description", self.interval_description.as_ref());
        form.field_opt("payment_schedule", self.payment_schedule.as_ref());
        form.field_opt("transaction_type", self.transaction_type.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsAffirm {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_01::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_02::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmSetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsAffirm {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsAfterpayClearpayCaptureMethod =
    stripe_types_01::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage =
    stripe_types_02::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsAmazonPayCaptureMethod =
    stripe_types_01::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsAuBecsDebitSetupFutureUsage =
    stripe_types_02::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsBillieCaptureMethod =
    stripe_types_01::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsBlik {
    #[doc = "Additional fields for Mandate creation"]
    pub mandate_options:
        Option<stripe_types_00::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsBlikMandateOptions>,
    pub setup_future_usage:
        Option<stripe_types_00::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsBlikSetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsBlik {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsBoletoSetupFutureUsage =
    stripe_types_04::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitSetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsCardCaptureMethod =
    stripe_types_01::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsCardInstallments =
    stripe_types_00::checkout::CreateCheckoutSessionParamsAdaptivePricing;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestIncrementalAuthorization =
    stripe_types_00::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestExtendedAuthorization;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsCardSetupFutureUsage =
    stripe_types_01::checkout::CreateCheckoutSessionParamsPaymentIntentDataSetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsCryptoSetupFutureUsage =
    stripe_types_02::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsDemoPaySetupFutureUsage =
    stripe_types_04::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAmazonPaySetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsFpxSetupFutureUsage =
    stripe_types_02::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsGiropaySetupFutureUsage =
    stripe_types_02::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsKakaoPayCaptureMethod =
    stripe_types_01::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsKonbiniSetupFutureUsage =
    stripe_types_02::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsKrCardCaptureMethod =
    stripe_types_01::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsMobilepaySetupFutureUsage =
    stripe_types_02::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsNaverPaySetupFutureUsage =
    stripe_types_04::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAmazonPaySetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsOxxoSetupFutureUsage =
    stripe_types_02::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsP24SetupFutureUsage =
    stripe_types_02::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsPaycoCaptureMethod =
    stripe_types_01::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsPaynowSetupFutureUsage =
    stripe_types_02::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptions {
    #[doc = "Amount that will be collected."]
    pub amount: Option<json::OwnedValue>,
    #[doc = "The type of amount that will be collected."]
    pub amount_type:
        Option<stripe_types_04::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsAmountType>,
    #[doc = "Date, in YYYY-MM-DD format, after which payments will not be collected."]
    pub end_date: Option<json::OwnedValue>,
    #[doc = "The periodicity at which payments will be collected."]
    pub payment_schedule: Option<
        stripe_types_02::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule,
    >,
    #[doc = "The number of payments that will be made during a payment period."]
    pub payments_per_period: Option<json::OwnedValue>,
    #[doc = "The purpose for which payments are made."]
    pub purpose:
        Option<stripe_types_03::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPurpose>,
    #[doc = "Date, in YYYY-MM-DD format, from which payments will be collected."]
    pub start_date: Option<json::OwnedValue>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount_type", self.amount_type.as_ref());
        form.field_opt("payment_schedule", self.payment_schedule.as_ref());
        form.field_opt("purpose", self.purpose.as_ref());
    }
}

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptionsAmountIncludesIof =
    stripe_types_03::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsPixAmountIncludesIof;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsRevolutPayCaptureMethod =
    stripe_types_01::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsSamsungPaySetupFutureUsage =
    stripe_types_02::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsSatispayCaptureMethod =
    stripe_types_01::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsSepaDebitMandateOptions =
    stripe_types_02::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsBacsDebitMandateOptions;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsSepaDebitSetupFutureUsage =
    stripe_types_04::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitSetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsSofortSetupFutureUsage =
    stripe_types_02::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsSunbitCaptureMethod =
    stripe_types_01::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountSetupFutureUsage =
    stripe_types_04::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitSetupFutureUsage;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsSavedPaymentMethodOptionsPaymentMethodSave =
    stripe_types_01::checkout::CreateCheckoutSessionParamsSavedPaymentMethodOptionsPaymentMethodRemove;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataDeliveryEstimateMinimumUnit = stripe_types_02::checkout::CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataDeliveryEstimateMaximumUnit;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataFixedAmountCurrencyOptionsValueTaxBehavior =
    stripe_types_03::checkout::CreateCheckoutSessionParamsLineItemsItemPriceDataTaxBehavior;

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataTaxBehavior =
    stripe_types_03::checkout::CreateCheckoutSessionParamsLineItemsItemPriceDataTaxBehavior;

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsSubscriptionDataBillingModeFlexible {
    #[doc = "Controls how invoices and invoice items display proration amounts and discount amounts."]
    pub proration_discounts: Option<
        stripe_types_00::checkout::CreateCheckoutSessionParamsSubscriptionDataBillingModeFlexibleProrationDiscounts,
    >,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsSubscriptionDataBillingModeFlexible {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("proration_discounts", self.proration_discounts.as_ref());
    }
}

#[cfg(feature = "checkout")]
pub type CreateCheckoutSessionParamsSubscriptionDataInvoiceSettingsIssuerType =
    stripe_types_00::checkout::CreateCheckoutSessionParamsAutomaticTaxLiabilityType;

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsTaxIdCollection {
    #[doc = "Enable tax ID collection during checkout."]
    pub enabled: bool,
    #[doc = "Describes whether a tax ID is required during checkout."]
    pub required: Option<stripe_types_03::checkout::CreateCheckoutSessionParamsTaxIdCollectionRequired>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsTaxIdCollection {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("required", self.required.as_ref());
    }
}

#[cfg(feature = "checkout")]
pub type ExpireCheckoutSessionParams = stripe_types_04::checkout::GetCheckoutSessionParams;

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentPagesCheckoutSessionAfterExpiration {
    #[doc = "When set, configuration used to recover the Checkout Session on expiry."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub recovery: Option<stripe_types_00::checkout::PaymentPagesCheckoutSessionAfterExpirationRecovery>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentPagesCheckoutSessionBrandingSettingsLogo {
    #[doc = "The ID of a [File upload](https://stripe.com/docs/api/files) representing the logo."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[doc = "The type of image for the logo."]
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
pub struct PaymentPagesCheckoutSessionCheckoutAddressDetails {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_01::common::Address>,
    #[doc = "Customer name."]
    #[json(default)]
    pub name: String,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentPagesCheckoutSessionConsent {
    #[doc = "If `opt_in`, the customer consents to receiving promotional communications from the merchant about this Checkout Sess..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub promotions: Option<stripe_types_03::checkout::PaymentPagesCheckoutSessionConsentPromotions>,
    #[doc = "If `accepted`, the customer in this Checkout Session has agreed to the merchant's terms of service."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<stripe_types_01::checkout::PaymentPagesCheckoutSessionConsentTermsOfService>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentPagesCheckoutSessionCustomText {
    #[doc = "Custom text that should be displayed after the payment confirmation button."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub after_submit: Option<stripe_types_03::checkout::PaymentPagesCheckoutSessionCustomTextPosition>,
    #[doc = "Custom text that should be displayed alongside shipping address collection."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<stripe_types_03::checkout::PaymentPagesCheckoutSessionCustomTextPosition>,
    #[doc = "Custom text that should be displayed alongside the payment confirmation button."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub submit: Option<stripe_types_03::checkout::PaymentPagesCheckoutSessionCustomTextPosition>,
    #[doc = "Custom text that should be displayed in place of the default terms of service agreement text."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub terms_of_service_acceptance: Option<stripe_types_03::checkout::PaymentPagesCheckoutSessionCustomTextPosition>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentPagesCheckoutSessionNameCollection {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub business: Option<stripe_types_03::checkout::PaymentPagesCheckoutSessionBusinessName>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub individual: Option<stripe_types_02::checkout::PaymentPagesCheckoutSessionIndividualName>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentPagesCheckoutSessionPaymentMethodReuseAgreement {
    #[doc = "Determines the position and visibility of the payment method reuse agreement in the UI."]
    #[json(default)]
    pub position: stripe_types_04::checkout::PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentPagesCheckoutSessionSavedPaymentMethodOptions {
    #[doc = "Uses the `allow_redisplay` value of each saved payment method to filter the set presented to a returning customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub allow_redisplay_filters: Option<
        Vec<stripe_types_02::checkout::PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFiltersItem>,
    >,
    #[doc = "Enable customers to choose if they wish to remove their saved payment methods."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_remove:
        Option<stripe_types_01::checkout::PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove>,
    #[doc = "Enable customers to choose if they wish to save their payment method for future use."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_save:
        Option<stripe_types_01::checkout::PaymentPagesCheckoutSessionSavedPaymentMethodOptionsPaymentMethodRemove>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentPagesCheckoutSessionShippingAddressCollection {
    #[doc = "An array of two-letter ISO country codes representing which countries Checkout should provide as options for shipping..."]
    #[json(default)]
    pub allowed_countries:
        Vec<stripe_types_02::checkout::PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentPagesCheckoutSessionTaxId {
    #[doc = "The type of the tax ID, one of `ad_nrt`, `ar_cuit`, `eu_vat`, `bo_tin`, `br_cnpj`, `br_cpf`, `cn_tin`, `co_nit`, `cr_..."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::checkout::PaymentPagesCheckoutSessionTaxIdType,
    #[doc = "The value of the tax ID."]
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
pub struct PaymentPagesCheckoutSessionTaxIdCollection {
    #[doc = "Indicates whether tax ID collection is enabled for the session"]
    #[json(default)]
    pub enabled: bool,
    #[doc = "Indicates whether a tax ID is required on the payment page"]
    #[json(default)]
    pub required: stripe_types_01::checkout::PaymentPagesCheckoutSessionTaxIdCollectionRequired,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictions {
    #[doc = "The card brands to block."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub brands_blocked: Option<Vec<stripe_types_03::checkout::PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsBrandsBlockedItem>>,
    #[doc = "Card funding types to block for this Checkout Session."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub funding_types_blocked: Option<Vec<stripe_types_00::checkout::PaymentPagesPrivateCardPaymentMethodOptionsResourceRestrictionsFundingTypesBlockedItem>>,
}

#[cfg(feature = "checkout")]
pub type UpdateCheckoutSessionParamsLineItemsItemAdjustableQuantity =
    stripe_types_04::checkout::CreateCheckoutSessionParamsLineItemsItemAdjustableQuantity;
