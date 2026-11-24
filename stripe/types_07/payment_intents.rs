#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CapturePaymentIntentParamsHooksInputs {
    #[doc = "Tax arguments for automations"]
    pub tax: Option<stripe_types_06::payment_intents::CapturePaymentIntentParamsHooksInputsTax>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CapturePaymentIntentParamsHooksInputs {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("tax", self.tax.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsHooksInputs {
    #[doc = "Tax arguments for automations"]
    pub tax: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsHooksInputsTax>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsHooksInputs {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("tax", self.tax.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodDataEps {
    #[doc = "The customer's bank."]
    pub bank: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataEpsBank>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodDataEps {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("bank", self.bank.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodDataIdeal {
    #[doc = "The customer's bank."]
    pub bank: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataIdealBank>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodDataIdeal {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("bank", self.bank.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodDataKlarna {
    #[doc = "Customer's date of birth"]
    pub dob: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataKlarnaDob>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodDataKlarna {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("dob", self.dob.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone)]
pub struct ConfirmPaymentIntentParamsPaymentMethodDataSofort {
    #[doc = "Two-letter ISO code representing the country the bank account is located in."]
    pub country: stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataSofortCountry,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodDataSofort {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("country", &self.country);
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptions {
    #[doc = "A URL for custom mandate text to render during confirmation step."]
    pub custom_mandate_url: Option<json::OwnedValue>,
    #[doc = "Description of the mandate interval."]
    pub interval_description: Option<String>,
    #[doc = "Payment schedule for the mandate."]
    pub payment_schedule: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule>,
    #[doc = "Transaction type of the mandate."]
    pub transaction_type: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("interval_description", self.interval_description.as_ref());
        form.field_opt("payment_schedule", self.payment_schedule.as_ref());
        form.field_opt("transaction_type", self.transaction_type.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsAffirm {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod>,
    #[doc = "Preferred language of the Affirm authorization page that the customer is redirected to."]
    pub preferred_locale: Option<String>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsAffirm {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("preferred_locale", self.preferred_locale.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsAfterpayClearpay {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsAfterpayClearpayCaptureMethod>,
    #[doc = "An internal identifier or reference that this payment corresponds to."]
    pub reference: Option<String>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsAfterpayClearpay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("reference", self.reference.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsAlma {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsAlmaCaptureMethod>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsAlma {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsBacsDebit {
    #[doc = "Additional fields for Mandate creation"]
    pub mandate_options:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsBacsDebitMandateOptions>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsBacsDebitSetupFutureUsage,
    >,
    #[doc = "Controls when Stripe will attempt to debit the funds from the customer's account."]
    pub target_date: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsBacsDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("target_date", self.target_date.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsBancontact {
    #[doc = "Preferred language of the Bancontact authorization page that the customer is redirected to."]
    pub preferred_language: Option<
        stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsBancontactPreferredLanguage,
    >,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsBancontactSetupFutureUsage,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsBancontact {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preferred_language", self.preferred_language.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsCashapp {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCashappCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCashappSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsCashapp {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsEps {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsEpsSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsEps {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsFpx {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsFpxSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsFpx {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsGiropay {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsGiropaySetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsGiropay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsKlarnaOnDemand {
    #[doc = "Your average amount value."]
    pub average_amount: Option<i64>,
    #[doc = "The maximum value you may charge a customer per purchase."]
    pub maximum_amount: Option<i64>,
    #[doc = "The lowest or minimum value you may charge a customer per purchase."]
    pub minimum_amount: Option<i64>,
    #[doc = "Interval at which the customer is making purchases"]
    pub purchase_interval: Option<
        stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval,
    >,
    #[doc = "The number of `purchase_interval` between charges"]
    pub purchase_interval_count: Option<i64>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsKlarnaOnDemand {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("average_amount", self.average_amount.as_ref());
        form.field_opt("maximum_amount", self.maximum_amount.as_ref());
        form.field_opt("minimum_amount", self.minimum_amount.as_ref());
        form.field_opt("purchase_interval", self.purchase_interval.as_ref());
        form.field_opt("purchase_interval_count", self.purchase_interval_count.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsKonbini {
    #[doc = "An optional 10 to 11 digit numeric-only string determining the confirmation code at applicable convenience stores."]
    pub confirmation_number: Option<json::OwnedValue>,
    #[doc = "The number of calendar days (between 1 and 60) after which Konbini payment instructions will expire."]
    pub expires_after_days: Option<json::OwnedValue>,
    #[doc = "The timestamp at which the Konbini payment instructions will expire."]
    pub expires_at: Option<json::OwnedValue>,
    #[doc = "A product descriptor of up to 22 characters, which will appear to customers at the convenience store."]
    pub product_description: Option<json::OwnedValue>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsKonbiniSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsKonbini {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsKrCard {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsKrCardCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsKrCardSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsKrCard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsMbWay {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsMbWaySetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsMbWay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsMobilepay {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsMobilepayCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsMobilepaySetupFutureUsage,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsMobilepay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsMultibanco {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsMultibancoSetupFutureUsage,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsMultibanco {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsNaverPay {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsNaverPayCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsNaverPaySetupFutureUsage,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsNaverPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsNzBankAccount {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsNzBankAccountSetupFutureUsage,
    >,
    #[doc = "Controls when Stripe will attempt to debit the funds from the customer's account."]
    pub target_date: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsNzBankAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("target_date", self.target_date.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsPayco {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsPaycoCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsPaycoSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsPayco {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsPaynow {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsPaynowSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsPaynow {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsWechatPay {
    #[doc = "The app ID registered with WeChat Pay."]
    pub app_id: Option<String>,
    #[doc = "The client type that the end customer will pay from"]
    pub client: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsWechatPayClient>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsWechatPaySetupFutureUsage,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsWechatPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("app_id", self.app_id.as_ref());
        form.field_opt("client", self.client.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsHooks {
    #[doc = "Arguments passed in automations"]
    pub inputs: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsHooksInputs>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsHooks {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("inputs", self.inputs.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodDataUsBankAccount {
    #[doc = "Account holder type: individual or company."]
    pub account_holder_type: Option<
        stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodDataUsBankAccountAccountHolderType,
    >,
    #[doc = "Account number of the bank account."]
    pub account_number: Option<String>,
    #[doc = "Account type: checkings or savings."]
    pub account_type:
        Option<stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodDataUsBankAccountAccountType>,
    #[doc = "The ID of a Financial Connections Account to use as a payment method."]
    pub financial_connections_account: Option<String>,
    #[doc = "Routing number of the bank account."]
    pub routing_number: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodDataUsBankAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account_holder_type", self.account_holder_type.as_ref());
        form.field_opt("account_number", self.account_number.as_ref());
        form.field_opt("account_type", self.account_type.as_ref());
        form.field_opt("financial_connections_account", self.financial_connections_account.as_ref());
        form.field_opt("routing_number", self.routing_number.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsAlma {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAlmaCaptureMethod>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsAlma {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetails {
    #[doc = "The address the company or entity is registered with."]
    pub registered_address: Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegisteredAddress>,
    #[doc = "Company or entity name."]
    pub registered_name: Option<String>,
    #[doc = "The official registration number for the given registration type."]
    pub registration_number: Option<String>,
    #[doc = "Type of registration the company or entity holds in their registered country."]
    pub registration_type: Option<stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType>,
    #[doc = "VAT id number"]
    pub vat: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("registered_address", self.registered_address.as_ref());
        form.field_opt("registered_name", self.registered_name.as_ref());
        form.field_opt("registration_number", self.registration_number.as_ref());
        form.field_opt("registration_type", self.registration_type.as_ref());
        form.field_opt("vat", self.vat.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsBoleto {
    #[doc = "The number of calendar days before a Boleto voucher expires."]
    pub expires_after_days: Option<i64>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsBoletoSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsBoleto {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("expires_after_days", self.expires_after_days.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsCardPresent {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_03::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardPresentCaptureMethod>,
    #[doc = "Request ability to capture this payment beyond the standard authorization validity window..."]
    pub request_extended_authorization: Option<bool>,
    #[doc = "Request ability to [increment](https://docs.stripe.com/terminal/features/incremental-authorizations) this PaymentInte..."]
    pub request_incremental_authorization_support: Option<bool>,
    #[doc = "Network routing priority on co-branded EMV cards supporting domestic debit and international card schemes."]
    pub routing:
        Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardPresentRouting>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsCardPresent {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("request_extended_authorization", self.request_extended_authorization.as_ref());
        form.field_opt(
            "request_incremental_authorization_support",
            self.request_incremental_authorization_support.as_ref(),
        );
        form.field_opt("routing", self.routing.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsCashapp {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCashappCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCashappSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsCashapp {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsFpx {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsFpxSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsFpx {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsKakaoPay {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsKakaoPayCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsKakaoPaySetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsKakaoPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsMobilepay {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsMobilepayCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsMobilepaySetupFutureUsage,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsMobilepay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsPayco {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPaycoCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPaycoSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsPayco {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsPaynow {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPaynowSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsPaynow {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsPayto {
    #[doc = "Additional fields for Mandate creation."]
    pub mandate_options:
        Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptions>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPaytoSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsPayto {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsPixMandateOptions {
    #[doc = "Amount to be charged for future payments."]
    pub amount: Option<i64>,
    #[doc = "Determines if the amount includes the IOF tax."]
    pub amount_includes_iof: Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPixMandateOptionsAmountIncludesIof>,
    #[doc = "Type of amount."]
    pub amount_type: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPixMandateOptionsAmountType>,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: Option<String>,
    #[doc = "Date when the mandate expires and no further payments will be charged, in `YYYY-MM-DD`."]
    pub end_date: Option<String>,
    #[doc = "Schedule at which the future payments will be charged."]
    pub payment_schedule: Option<stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule>,
    #[doc = "Subscription name displayed to buyers in their bank app."]
    pub reference: Option<String>,
    #[doc = "Start date of the mandate, in `YYYY-MM-DD`."]
    pub start_date: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsPixMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("amount_includes_iof", self.amount_includes_iof.as_ref());
        form.field_opt("amount_type", self.amount_type.as_ref());
        form.field_opt("currency", self.currency.as_ref());
        form.field_opt("end_date", self.end_date.as_ref());
        form.field_opt("payment_schedule", self.payment_schedule.as_ref());
        form.field_opt("reference", self.reference.as_ref());
        form.field_opt("start_date", self.start_date.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsScalapay {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsScalapayCaptureMethod>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsScalapay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsSepaDebit {
    #[doc = "Additional fields for Mandate creation"]
    pub mandate_options:
        Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsSepaDebitMandateOptions>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsSepaDebitSetupFutureUsage,
    >,
    #[doc = "Controls when Stripe will attempt to debit the funds from the customer's account."]
    pub target_date: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsSepaDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("target_date", self.target_date.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsSunbit {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsSunbitCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsSunbitSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsSunbit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsSwish {
    #[doc = "A reference for this payment to be displayed in the Swish app."]
    pub reference: Option<json::OwnedValue>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsSwishSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsSwish {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsTwint {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsTwintSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsTwint {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsWechatPay {
    #[doc = "The app ID registered with WeChat Pay."]
    pub app_id: Option<String>,
    #[doc = "The client type that the end customer will pay from"]
    pub client: Option<stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsWechatPayClient>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsWechatPaySetupFutureUsage,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsWechatPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("app_id", self.app_id.as_ref());
        form.field_opt("client", self.client.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone)]
pub struct CreatePaymentIntentParamsShipping {
    #[doc = "Shipping address."]
    pub address: stripe_types_06::payment_intents::CreatePaymentIntentParamsShippingAddress,
    #[doc = "The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc."]
    pub carrier: Option<String>,
    #[doc = "Recipient name."]
    pub name: String,
    #[doc = "Recipient phone (including extension)."]
    pub phone: Option<String>,
    #[doc = "The tracking number for a physical product, obtained from the delivery service."]
    pub tracking_number: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsShipping {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("address", &self.address);
        form.field_opt("carrier", self.carrier.as_ref());
        form.field("name", &self.name);
        form.field_opt("phone", self.phone.as_ref());
        form.field_opt("tracking_number", self.tracking_number.as_ref());
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct FundingInstructionsBankTransferFinancialAddress {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub aba: Option<stripe_types_05::payment_intents::FundingInstructionsBankTransferAbaRecord>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub iban: Option<stripe_types_05::payment_intents::FundingInstructionsBankTransferIbanRecord>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<stripe_types_06::payment_intents::FundingInstructionsBankTransferSortCodeRecord>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub spei: Option<stripe_types_06::payment_intents::FundingInstructionsBankTransferSpeiRecord>,
    #[doc = "The payment networks supported by this FinancialAddress"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub supported_networks: Option<
        Vec<stripe_types_01::payment_intents::FundingInstructionsBankTransferFinancialAddressSupportedNetworksItem>,
    >,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub swift: Option<stripe_types_06::payment_intents::FundingInstructionsBankTransferSwiftRecord>,
    #[doc = "The type of financial address"]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::payment_intents::FundingInstructionsBankTransferFinancialAddressType,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub zengin: Option<stripe_types_06::payment_intents::FundingInstructionsBankTransferZenginRecord>,
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct IncrementAuthorizationParamsAmountDetails {
    #[doc = "The total discount applied on the transaction represented in the smallest currency unit..."]
    pub discount_amount: Option<json::OwnedValue>,
    #[doc = "Set to `false` to return arithmetic validation errors in the response without failing the request."]
    pub enforce_arithmetic_validation: Option<bool>,
    #[doc = "A list of line items, each containing information about a product in the PaymentIntent."]
    pub line_items: Option<json::OwnedValue>,
    #[doc = "Contains information about the shipping portion of the amount."]
    pub shipping: Option<stripe_types_05::payment_intents::IncrementAuthorizationParamsAmountDetailsShipping>,
    #[doc = "Contains information about the tax portion of the amount."]
    pub tax: Option<stripe_types_05::payment_intents::IncrementAuthorizationParamsAmountDetailsTax>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for IncrementAuthorizationParamsAmountDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("enforce_arithmetic_validation", self.enforce_arithmetic_validation.as_ref());
        form.field_opt("shipping", self.shipping.as_ref());
        form.field_opt("tax", self.tax.as_ref());
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentIntentPaymentMethodOptionsPayto {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<stripe_types_05::payment_intents::PaymentIntentPaymentMethodOptionsMandateOptionsPayto>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<
        stripe_types_04::payment_intents::PaymentFlowsPrivatePaymentMethodsKakaoPayPaymentMethodOptionsSetupFutureUsage,
    >,
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentIntentProcessing {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types_05::payment_intents::PaymentIntentCardProcessing>,
    #[doc = "Type of the payment method for which payment is in `processing` state, one of `card`."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::payment_intents::PaymentIntentProcessingType,
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsHooksInputs {
    #[doc = "Tax arguments for automations"]
    pub tax: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsHooksInputsTax>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsHooksInputs {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("tax", self.tax.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodDataBillingDetails {
    #[doc = "Billing address."]
    pub address:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataBillingDetailsAddress>,
    #[doc = "Email address."]
    pub email: Option<json::OwnedValue>,
    #[doc = "Full name."]
    pub name: Option<json::OwnedValue>,
    #[doc = "Billing phone number (including extension)."]
    pub phone: Option<json::OwnedValue>,
    #[doc = "Taxpayer identification number."]
    pub tax_id: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodDataBillingDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("address", self.address.as_ref());
        form.field_opt("tax_id", self.tax_id.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodDataIdeal {
    #[doc = "The customer's bank."]
    pub bank: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataIdealBank>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodDataIdeal {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("bank", self.bank.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodDataP24 {
    #[doc = "The customer's bank."]
    pub bank: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataP24Bank>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodDataP24 {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("bank", self.bank.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsAuBecsDebit {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsAuBecsDebitSetupFutureUsage,
    >,
    #[doc = "Controls when Stripe will attempt to debit the funds from the customer's account."]
    pub target_date: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsAuBecsDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("target_date", self.target_date.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsBlik {
    #[doc = "The 6-digit BLIK code that a customer has generated using their banking application."]
    pub code: Option<String>,
    #[doc = "Details of the BLIK mandate"]
    pub mandate_options:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsBlikMandateOptions>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsBlikSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsBlik {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("code", self.code.as_ref());
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlan {
    #[doc = "For `fixed_count` installment plans, this is required."]
    pub count: Option<i64>,
    #[doc = "For `fixed_count` installment plans, this is required."]
    pub interval: Option<
        stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlanInterval,
    >,
    #[doc = "Type of installment plan, one of `fixed_count`, `bonus`, or `revolving`."]
    pub r#type: stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlanType,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlan {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("count", self.count.as_ref());
        form.field_opt("interval", self.interval.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransfer {
    #[doc = "Configuration for the eu_bank_transfer funding type."]
    pub eu_bank_transfer: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer>,
    #[doc = "List of address types that should be returned in the financial_addresses response."]
    pub requested_address_types: Option<Vec<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem>>,
    #[doc = "The list of bank transfer types that this PaymentIntent is allowed to use for funding Permitted values include: `eu_b..."]
    pub r#type: stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferType,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransfer {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("eu_bank_transfer", self.eu_bank_transfer.as_ref());
        if let Some(values) = self.requested_address_types.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("requested_address_types", index, item);
            }
        }
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsFpx {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsFpxSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsFpx {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsGiropay {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsGiropaySetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsGiropay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsKakaoPay {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsKakaoPayCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsKakaoPaySetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsKakaoPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsMultibanco {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsMultibancoSetupFutureUsage,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsMultibanco {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsOxxo {
    #[doc = "The number of calendar days before an OXXO voucher expires."]
    pub expires_after_days: Option<i64>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsOxxoSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsOxxo {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("expires_after_days", self.expires_after_days.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsPaynow {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsPaynowSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsPaynow {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsPaypal {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsPaypalCaptureMethod>,
    #[doc = "[Preferred locale](https://docs.stripe.com/payments/paypal/supported-locales) of the PayPal checkout page that the cu..."]
    pub preferred_locale:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale>,
    #[doc = "A reference of the PayPal transaction visible to customer which is mapped to PayPal's invoice ID."]
    pub reference: Option<String>,
    #[doc = "The risk correlation ID for an on-session payment using a saved PayPal payment method."]
    pub risk_correlation_id: Option<String>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsPaypalSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsPaypal {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("preferred_locale", self.preferred_locale.as_ref());
        form.field_opt("reference", self.reference.as_ref());
        form.field_opt("risk_correlation_id", self.risk_correlation_id.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsRevolutPay {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsRevolutPayCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsRevolutPaySetupFutureUsage,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsRevolutPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsSofort {
    #[doc = "Language shown to the payer on redirect."]
    pub preferred_language:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsSofortPreferredLanguage>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsSofortSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsSofort {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preferred_language", self.preferred_language.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsSunbit {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsSunbitCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsSunbitSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsSunbit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsTwint {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsTwintSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsTwint {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsUsBankAccountNetworks {
    #[doc = "Triggers validations to run across the selected networks"]
    pub requested: Option<Vec<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsUsBankAccountNetworksRequestedItem>>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsUsBankAccountNetworks {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.requested.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("requested", index, item);
            }
        }
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsTransferData {
    #[doc = "The amount that will be transferred automatically when a charge succeeds."]
    pub amount: Option<i64>,
    #[doc = "An arbitrary string attached to the transfer."]
    pub description: Option<String>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The data with which to populate the destination payment."]
    pub payment_data: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsTransferDataPaymentData>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsTransferData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("description", self.description.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("payment_data", self.payment_data.as_ref());
    }
}
