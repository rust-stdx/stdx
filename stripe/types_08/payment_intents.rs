#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CapturePaymentIntentParamsAmountDetails {
    #[doc = "The total discount applied on the transaction represented in the smallest currency unit..."]
    pub discount_amount: Option<json::OwnedValue>,
    #[doc = "Set to `false` to return arithmetic validation errors in the response without failing the request."]
    pub enforce_arithmetic_validation: Option<bool>,
    #[doc = "A list of line items, each containing information about a product in the PaymentIntent."]
    pub line_items: Option<json::OwnedValue>,
    #[doc = "Contains information about the shipping portion of the amount."]
    pub shipping: Option<stripe_types_06::payment_intents::CapturePaymentIntentParamsAmountDetailsShipping>,
    #[doc = "Contains information about the tax portion of the amount."]
    pub tax: Option<stripe_types_06::payment_intents::CapturePaymentIntentParamsAmountDetailsTax>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CapturePaymentIntentParamsAmountDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("enforce_arithmetic_validation", self.enforce_arithmetic_validation.as_ref());
        form.field_opt("shipping", self.shipping.as_ref());
        form.field_opt("tax", self.tax.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsAmountDetails {
    #[doc = "The total discount applied on the transaction represented in the smallest currency unit..."]
    pub discount_amount: Option<json::OwnedValue>,
    #[doc = "Set to `false` to return arithmetic validation errors in the response without failing the request."]
    pub enforce_arithmetic_validation: Option<bool>,
    #[doc = "A list of line items, each containing information about a product in the PaymentIntent."]
    pub line_items: Option<json::OwnedValue>,
    #[doc = "Contains information about the shipping portion of the amount."]
    pub shipping: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsAmountDetailsShipping>,
    #[doc = "Contains information about the tax portion of the amount."]
    pub tax: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsAmountDetailsTax>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsAmountDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("enforce_arithmetic_validation", self.enforce_arithmetic_validation.as_ref());
        form.field_opt("shipping", self.shipping.as_ref());
        form.field_opt("tax", self.tax.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone)]
pub struct ConfirmPaymentIntentParamsPaymentMethodDataFpx {
    #[doc = "Account holder type for FPX transaction"]
    pub account_holder_type:
        Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataFpxAccountHolderType>,
    #[doc = "The customer's bank."]
    pub bank: stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataFpxBank,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodDataFpx {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account_holder_type", self.account_holder_type.as_ref());
        form.field("bank", &self.bank);
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsAmazonPay {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsAmazonPayCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsAmazonPaySetupFutureUsage,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsAmazonPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsAuBecsDebit {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsAuBecsDebitSetupFutureUsage,
    >,
    #[doc = "Controls when Stripe will attempt to debit the funds from the customer's account."]
    pub target_date: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsAuBecsDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("target_date", self.target_date.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsBoleto {
    #[doc = "The number of calendar days before a Boleto voucher expires."]
    pub expires_after_days: Option<i64>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsBoletoSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsBoleto {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("expires_after_days", self.expires_after_days.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlan {
    #[doc = "For `fixed_count` installment plans, this is required."]
    pub count: Option<i64>,
    #[doc = "For `fixed_count` installment plans, this is required."]
    pub interval: Option<
        stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlanInterval,
    >,
    #[doc = "Type of installment plan, one of `fixed_count`, `bonus`, or `revolving`."]
    pub r#type:
        stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlanType,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlan {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("count", self.count.as_ref());
        form.field_opt("interval", self.interval.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsCardMandateOptions {
    #[doc = "Amount to be charged for future payments, specified in the presentment currency."]
    pub amount: i64,
    #[doc = "One of `fixed` or `maximum`."]
    pub amount_type: stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCardMandateOptionsAmountType,
    #[doc = "A description of the mandate or subscription that is meant to be displayed to the customer."]
    pub description: Option<String>,
    #[doc = "End date of the mandate or subscription."]
    pub end_date: Option<i64>,
    #[doc = "Specifies payment frequency."]
    pub interval: stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCardMandateOptionsInterval,
    #[doc = "The number of intervals between payments."]
    pub interval_count: Option<i64>,
    #[doc = "Unique identifier for the mandate or subscription."]
    pub reference: String,
    #[doc = "Start date of the mandate or subscription."]
    pub start_date: i64,
    #[doc = "Specifies the type of mandates supported."]
    pub supported_types: Option<Vec<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCardMandateOptionsSupportedTypesItem>>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsCardMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        form.field("amount_type", &self.amount_type);
        form.field_opt("description", self.description.as_ref());
        form.field_opt("end_date", self.end_date.as_ref());
        form.field("interval", &self.interval);
        form.field_opt("interval_count", self.interval_count.as_ref());
        form.field("reference", &self.reference);
        form.field("start_date", &self.start_date);
        if let Some(values) = self.supported_types.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("supported_types", index, item);
            }
        }
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransfer {
    #[doc = "Configuration for the eu_bank_transfer funding type."]
    pub eu_bank_transfer: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer>,
    #[doc = "List of address types that should be returned in the financial_addresses response."]
    pub requested_address_types: Option<Vec<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem>>,
    #[doc = "The list of bank transfer types that this PaymentIntent is allowed to use for funding Permitted values include: `eu_b..."]
    pub r#type: stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferType,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransfer {
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
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsGrabpay {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsGrabpaySetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsGrabpay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsLink {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsLinkCaptureMethod>,
    #[doc = "Deprecated This is a legacy parameter that no longer has any function."]
    pub persistent_token: Option<String>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsLinkSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsLink {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("persistent_token", self.persistent_token.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsOxxo {
    #[doc = "The number of calendar days before an OXXO voucher expires."]
    pub expires_after_days: Option<i64>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsOxxoSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsOxxo {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("expires_after_days", self.expires_after_days.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsPaytoMandateOptions {
    #[doc = "Amount that will be collected."]
    pub amount: Option<json::OwnedValue>,
    #[doc = "The type of amount that will be collected."]
    pub amount_type: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType>,
    #[doc = "Date, in YYYY-MM-DD format, after which payments will not be collected."]
    pub end_date: Option<json::OwnedValue>,
    #[doc = "The periodicity at which payments will be collected."]
    pub payment_schedule: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule>,
    #[doc = "The number of payments that will be made during a payment period."]
    pub payments_per_period: Option<json::OwnedValue>,
    #[doc = "The purpose for which payments are made."]
    pub purpose: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsPaytoMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount_type", self.amount_type.as_ref());
        form.field_opt("payment_schedule", self.payment_schedule.as_ref());
        form.field_opt("purpose", self.purpose.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsPixMandateOptions {
    #[doc = "Amount to be charged for future payments."]
    pub amount: Option<i64>,
    #[doc = "Determines if the amount includes the IOF tax."]
    pub amount_includes_iof: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsPixMandateOptionsAmountIncludesIof>,
    #[doc = "Type of amount."]
    pub amount_type: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsPixMandateOptionsAmountType>,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: Option<String>,
    #[doc = "Date when the mandate expires and no further payments will be charged, in `YYYY-MM-DD`."]
    pub end_date: Option<String>,
    #[doc = "Schedule at which the future payments will be charged."]
    pub payment_schedule: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule>,
    #[doc = "Subscription name displayed to buyers in their bank app."]
    pub reference: Option<String>,
    #[doc = "Start date of the mandate, in `YYYY-MM-DD`."]
    pub start_date: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsPixMandateOptions {
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
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsSamsungPay {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsSamsungPayCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsSamsungPaySetupFutureUsage,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsSamsungPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsScalapay {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsScalapayCaptureMethod>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsScalapay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsSepaDebit {
    #[doc = "Additional fields for Mandate creation"]
    pub mandate_options:
        Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsSepaDebitMandateOptions>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsSepaDebitSetupFutureUsage,
    >,
    #[doc = "Controls when Stripe will attempt to debit the funds from the customer's account."]
    pub target_date: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsSepaDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("target_date", self.target_date.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsSofort {
    #[doc = "Language shown to the payer on redirect."]
    pub preferred_language:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsSofortPreferredLanguage>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsSofortSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsSofort {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preferred_language", self.preferred_language.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsSunbit {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsSunbitCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsSunbitSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsSunbit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsTwint {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsTwintSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsTwint {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsUpiMandateOptions {
    #[doc = "Amount to be charged for future payments."]
    pub amount: Option<i64>,
    #[doc = "One of `fixed` or `maximum`."]
    pub amount_type: Option<
        stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsUpiMandateOptionsAmountType,
    >,
    #[doc = "A description of the mandate or subscription that is meant to be displayed to the customer."]
    pub description: Option<String>,
    #[doc = "End date of the mandate or subscription."]
    pub end_date: Option<i64>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsUpiMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("amount_type", self.amount_type.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field_opt("end_date", self.end_date.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsUsBankAccountMandateOptions {
    #[doc = "The method used to collect offline mandate customer acceptance."]
    pub collection_method: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsUsBankAccountMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("collection_method", self.collection_method.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsZip {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsZipSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsZip {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone)]
pub struct ConfirmPaymentIntentParamsShipping {
    #[doc = "Shipping address."]
    pub address: stripe_types_06::payment_intents::ConfirmPaymentIntentParamsShippingAddress,
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
impl crate::params::FormEncode for ConfirmPaymentIntentParamsShipping {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("address", &self.address);
        form.field_opt("carrier", self.carrier.as_ref());
        form.field("name", &self.name);
        form.field_opt("phone", self.phone.as_ref());
        form.field_opt("tracking_number", self.tracking_number.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodDataUpi {
    #[doc = "Configuration options for setting up an eMandate"]
    pub mandate_options:
        Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodDataUpiMandateOptions>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodDataUpi {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsAcssDebit {
    #[doc = "Additional fields for Mandate creation"]
    pub mandate_options:
        Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptions>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage,
    >,
    #[doc = "Controls when Stripe will attempt to debit the funds from the customer's account."]
    pub target_date: Option<String>,
    #[doc = "Bank account verification method."]
    pub verification_method: Option<
        stripe_types_03::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsAcssDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("target_date", self.target_date.as_ref());
        form.field_opt("verification_method", self.verification_method.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsAmazonPay {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAmazonPayCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAmazonPaySetupFutureUsage,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsAmazonPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsBacsDebit {
    #[doc = "Additional fields for Mandate creation"]
    pub mandate_options:
        Option<stripe_types_03::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsBacsDebitMandateOptions>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsBacsDebitSetupFutureUsage,
    >,
    #[doc = "Controls when Stripe will attempt to debit the funds from the customer's account."]
    pub target_date: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsBacsDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("target_date", self.target_date.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsCardInstallments {
    #[doc = "Setting to true enables installments for this PaymentIntent."]
    pub enabled: Option<bool>,
    #[doc = "The selected installment plan to use for this payment attempt."]
    pub plan:
        Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlan>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsCardInstallments {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("enabled", self.enabled.as_ref());
        form.field_opt("plan", self.plan.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptions {
    #[doc = "Cartes Bancaires-specific 3DS fields."]
    pub cartes_bancaires: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancaires>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("cartes_bancaires", self.cartes_bancaires.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsGiropay {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsGiropaySetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsGiropay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsKrCard {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsKrCardCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsKrCardSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsKrCard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsLink {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsLinkCaptureMethod>,
    #[doc = "Deprecated This is a legacy parameter that no longer has any function."]
    pub persistent_token: Option<String>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsLinkSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsLink {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("persistent_token", self.persistent_token.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsMbWay {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsMbWaySetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsMbWay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsP24 {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsP24SetupFutureUsage>,
    #[doc = "Confirm that the payer has accepted the P24 terms and conditions."]
    pub tos_shown_and_accepted: Option<bool>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsP24 {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("tos_shown_and_accepted", self.tos_shown_and_accepted.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsPromptpay {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPromptpaySetupFutureUsage,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsPromptpay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsRevolutPay {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsRevolutPayCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsRevolutPaySetupFutureUsage,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsRevolutPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsSatispay {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsSatispayCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsSatispaySetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsSatispay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsSofort {
    #[doc = "Language shown to the payer on redirect."]
    pub preferred_language:
        Option<stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsSofortPreferredLanguage>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsSofortSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsSofort {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preferred_language", self.preferred_language.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsUpiMandateOptions {
    #[doc = "Amount to be charged for future payments."]
    pub amount: Option<i64>,
    #[doc = "One of `fixed` or `maximum`."]
    pub amount_type: Option<
        stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsUpiMandateOptionsAmountType,
    >,
    #[doc = "A description of the mandate or subscription that is meant to be displayed to the customer."]
    pub description: Option<String>,
    #[doc = "End date of the mandate or subscription."]
    pub end_date: Option<i64>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsUpiMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("amount_type", self.amount_type.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field_opt("end_date", self.end_date.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsFilters {
    #[doc = "The account subcategories to use to filter for selectable accounts."]
    pub account_subcategories: Option<Vec<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem>>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode
    for CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsFilters
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.account_subcategories.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("account_subcategories", index, item);
            }
        }
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsZip {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsZipSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsZip {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
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
pub struct PaymentIntentNextActionKonbini {
    #[doc = "The timestamp at which the pending Konbini payment expires."]
    #[json(default)]
    pub expires_at: i64,
    #[doc = "The URL for the Konbini payment instructions page, which allows customers to view and print a Konbini voucher."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub hosted_voucher_url: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub stores: Option<stripe_types_05::payment_intents::PaymentIntentNextActionKonbiniStores>,
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
pub struct PaymentIntentPaymentMethodOptionsBlik {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<stripe_types_06::payment_intents::PaymentIntentPaymentMethodOptionsMandateOptionsBlik>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<
        stripe_types_04::payment_intents::PaymentFlowsPrivatePaymentMethodsKakaoPayPaymentMethodOptionsSetupFutureUsage,
    >,
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone)]
pub struct UpdatePaymentIntentParamsPaymentMethodDataFpx {
    #[doc = "Account holder type for FPX transaction"]
    pub account_holder_type:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataFpxAccountHolderType>,
    #[doc = "The customer's bank."]
    pub bank: stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataFpxBank,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodDataFpx {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account_holder_type", self.account_holder_type.as_ref());
        form.field("bank", &self.bank);
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodDataUpiMandateOptions {
    #[doc = "Amount to be charged for future payments."]
    pub amount: Option<i64>,
    #[doc = "One of `fixed` or `maximum`."]
    pub amount_type:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataUpiMandateOptionsAmountType>,
    #[doc = "A description of the mandate or subscription that is meant to be displayed to the customer."]
    pub description: Option<String>,
    #[doc = "End date of the mandate or subscription."]
    pub end_date: Option<i64>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodDataUpiMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("amount_type", self.amount_type.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field_opt("end_date", self.end_date.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodDataUsBankAccount {
    #[doc = "Account holder type: individual or company."]
    pub account_holder_type: Option<
        stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataUsBankAccountAccountHolderType,
    >,
    #[doc = "Account number of the bank account."]
    pub account_number: Option<String>,
    #[doc = "Account type: checkings or savings."]
    pub account_type:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataUsBankAccountAccountType>,
    #[doc = "The ID of a Financial Connections Account to use as a payment method."]
    pub financial_connections_account: Option<String>,
    #[doc = "Routing number of the bank account."]
    pub routing_number: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodDataUsBankAccount {
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
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptions {
    #[doc = "A URL for custom mandate text to render during confirmation step."]
    pub custom_mandate_url: Option<json::OwnedValue>,
    #[doc = "Description of the mandate interval."]
    pub interval_description: Option<String>,
    #[doc = "Payment schedule for the mandate."]
    pub payment_schedule: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule>,
    #[doc = "Transaction type of the mandate."]
    pub transaction_type: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("interval_description", self.interval_description.as_ref());
        form.field_opt("payment_schedule", self.payment_schedule.as_ref());
        form.field_opt("transaction_type", self.transaction_type.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsAfterpayClearpay {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method: Option<
        stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsAfterpayClearpayCaptureMethod,
    >,
    #[doc = "An internal identifier or reference that this payment corresponds to."]
    pub reference: Option<String>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsAfterpayClearpay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("reference", self.reference.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsAlipay {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsAlipay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsAlma {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsAlmaCaptureMethod>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsAlma {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsBancontact {
    #[doc = "Preferred language of the Bancontact authorization page that the customer is redirected to."]
    pub preferred_language: Option<
        stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsBancontactPreferredLanguage,
    >,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsBancontactSetupFutureUsage,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsBancontact {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preferred_language", self.preferred_language.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetails {
    #[doc = "The address the company or entity is registered with."]
    pub registered_address: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegisteredAddress>,
    #[doc = "Company or entity name."]
    pub registered_name: Option<String>,
    #[doc = "The official registration number for the given registration type."]
    pub registration_number: Option<String>,
    #[doc = "Type of registration the company or entity holds in their registered country."]
    pub registration_type: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType>,
    #[doc = "VAT id number"]
    pub vat: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetails {
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
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsBoleto {
    #[doc = "The number of calendar days before a Boleto voucher expires."]
    pub expires_after_days: Option<i64>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsBoletoSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsBoleto {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("expires_after_days", self.expires_after_days.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsCardPresentRouting {
    #[doc = "Routing requested priority"]
    pub requested_priority: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCardPresentRoutingRequestedPriority>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsCardPresentRouting {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("requested_priority", self.requested_priority.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancaires {
    #[doc = "The cryptogram calculation algorithm used by the card Issuer's ACS to calculate the Authentication cryptogram."]
    pub cb_avalgo: stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo,
    #[doc = "The exemption indicator returned from Cartes Bancaires in the ARes."]
    pub cb_exemption: Option<String>,
    #[doc = "The risk score returned from Cartes Bancaires in the ARes."]
    pub cb_score: Option<i64>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode
    for UpdatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancaires
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("cb_avalgo", &self.cb_avalgo);
        form.field_opt("cb_exemption", self.cb_exemption.as_ref());
        form.field_opt("cb_score", self.cb_score.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsGrabpay {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsGrabpaySetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsGrabpay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsKlarnaOnDemand {
    #[doc = "Your average amount value."]
    pub average_amount: Option<i64>,
    #[doc = "The maximum value you may charge a customer per purchase."]
    pub maximum_amount: Option<i64>,
    #[doc = "The lowest or minimum value you may charge a customer per purchase."]
    pub minimum_amount: Option<i64>,
    #[doc = "Interval at which the customer is making purchases"]
    pub purchase_interval: Option<
        stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval,
    >,
    #[doc = "The number of `purchase_interval` between charges"]
    pub purchase_interval_count: Option<i64>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsKlarnaOnDemand {
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
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsKonbini {
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
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsKonbiniSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsKonbini {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsKrCard {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsKrCardCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsKrCardSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsKrCard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsMbWay {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsMbWaySetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsMbWay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsMobilepay {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsMobilepayCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsMobilepaySetupFutureUsage,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsMobilepay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsNzBankAccount {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsNzBankAccountSetupFutureUsage,
    >,
    #[doc = "Controls when Stripe will attempt to debit the funds from the customer's account."]
    pub target_date: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsNzBankAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("target_date", self.target_date.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsPixMandateOptions {
    #[doc = "Amount to be charged for future payments."]
    pub amount: Option<i64>,
    #[doc = "Determines if the amount includes the IOF tax."]
    pub amount_includes_iof: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsPixMandateOptionsAmountIncludesIof>,
    #[doc = "Type of amount."]
    pub amount_type: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsPixMandateOptionsAmountType>,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: Option<String>,
    #[doc = "Date when the mandate expires and no further payments will be charged, in `YYYY-MM-DD`."]
    pub end_date: Option<String>,
    #[doc = "Schedule at which the future payments will be charged."]
    pub payment_schedule: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule>,
    #[doc = "Subscription name displayed to buyers in their bank app."]
    pub reference: Option<String>,
    #[doc = "Start date of the mandate, in `YYYY-MM-DD`."]
    pub start_date: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsPixMandateOptions {
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
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsSamsungPay {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsSamsungPayCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsSamsungPaySetupFutureUsage,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsSamsungPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsSatispay {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsSatispayCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsSatispaySetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsSatispay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsScalapay {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsScalapayCaptureMethod>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsScalapay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsSepaDebit {
    #[doc = "Additional fields for Mandate creation"]
    pub mandate_options:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsSepaDebitMandateOptions>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsSepaDebitSetupFutureUsage,
    >,
    #[doc = "Controls when Stripe will attempt to debit the funds from the customer's account."]
    pub target_date: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsSepaDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("target_date", self.target_date.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsSwish {
    #[doc = "A reference for this payment to be displayed in the Swish app."]
    pub reference: Option<json::OwnedValue>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsSwishSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsSwish {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsFilters {
    #[doc = "The account subcategories to use to filter for selectable accounts."]
    pub account_subcategories: Option<Vec<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem>>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode
    for UpdatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsFilters
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.account_subcategories.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("account_subcategories", index, item);
            }
        }
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsWechatPay {
    #[doc = "The app ID registered with WeChat Pay."]
    pub app_id: Option<String>,
    #[doc = "The client type that the end customer will pay from"]
    pub client: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsWechatPayClient>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsWechatPaySetupFutureUsage,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsWechatPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("app_id", self.app_id.as_ref());
        form.field_opt("client", self.client.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsZip {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsZipSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsZip {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}
