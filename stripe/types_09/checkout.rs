#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutCustomerBalanceBankTransferPaymentMethodOptions {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub eu_bank_transfer: Option<stripe_types_06::payment_methods::PaymentMethodOptionsCustomerBalanceEuBankAccount>,
    #[doc = "List of address types that should be returned in the financial_addresses response."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub requested_address_types: Option<
        Vec<
            stripe_types_04::checkout::CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypesItem,
        >,
    >,
    #[doc = "The bank transfer type that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_trans..."]
    #[json(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<stripe_types_02::checkout::CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutFinancialConnectionsPaymentMethodOptions {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<stripe_types_06::checkout::CheckoutFinancialConnectionsPaymentMethodOptionsFilters>,
    #[doc = "The list of permissions to request."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub permissions:
        Option<Vec<stripe_types_03::checkout::CheckoutFinancialConnectionsPaymentMethodOptionsPermissionsItem>>,
    #[doc = "Data features requested to be retrieved upon account creation."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub prefetch: Option<Vec<stripe_types_03::checkout::CheckoutFinancialConnectionsPaymentMethodOptionsPrefetchItem>>,
    #[doc = "For webview integrations only."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutPixPaymentMethodOptions {
    #[doc = "Determines if the amount includes the IOF tax."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_includes_iof: Option<stripe_types_04::checkout::CheckoutPixPaymentMethodOptionsAmountIncludesIof>,
    #[doc = "The number of seconds after which Pix payment will expire."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub expires_after_seconds: Option<i64>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<stripe_types_05::payment_methods::PaymentMethodOptionsMandateOptionsPix>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_types_03::checkout::CheckoutAmazonPayPaymentMethodOptionsSetupFutureUsage>,
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsAutomaticTax {
    #[doc = "Set to `true` to [calculate tax automatically](https://docs.stripe.com/tax) using the customer's location."]
    pub enabled: bool,
    #[doc = "The account that's liable for tax."]
    pub liability: Option<stripe_types_05::checkout::CreateCheckoutSessionParamsAutomaticTaxLiability>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsAutomaticTax {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("liability", self.liability.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsLineItemsItemPriceData {
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "The ID of the [Product](https://docs.stripe.com/api/products) that this [Price](https://docs.stripe.com/api/prices) w..."]
    pub product: Option<String>,
    #[doc = "Data used to generate a new [Product](https://docs.stripe.com/api/products) object inline."]
    pub product_data: Option<stripe_types_06::checkout::CreateCheckoutSessionParamsLineItemsItemPriceDataProductData>,
    #[doc = "The recurring components of a price such as `interval` and `interval_count`."]
    pub recurring: Option<stripe_types_06::checkout::CreateCheckoutSessionParamsLineItemsItemPriceDataRecurring>,
    #[doc = "Only required if a default tax behavior..."]
    pub tax_behavior: Option<stripe_types_03::checkout::CreateCheckoutSessionParamsLineItemsItemPriceDataTaxBehavior>,
    #[doc = "A non-negative integer in cents (or local equivalent) representing how much to charge."]
    pub unit_amount: Option<i64>,
    #[doc = "Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places."]
    pub unit_amount_decimal: Option<String>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsLineItemsItemPriceData {
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

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentIntentData {
    #[doc = "The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the..."]
    pub application_fee_amount: Option<i64>,
    #[doc = "Controls when the funds will be captured from the customer's account."]
    pub capture_method: Option<stripe_types_03::checkout::CreateCheckoutSessionParamsPaymentIntentDataCaptureMethod>,
    #[doc = "An arbitrary string attached to the object."]
    pub description: Option<String>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The Stripe account ID for which these funds are intended."]
    pub on_behalf_of: Option<String>,
    #[doc = "Email address that the receipt for the resulting payment will be sent to."]
    pub receipt_email: Option<String>,
    #[doc = "Indicates that you intend to [make future payments](https://docs.stripe.com/payments/payment-intents#future-usage) wi..."]
    pub setup_future_usage:
        Option<stripe_types_01::checkout::CreateCheckoutSessionParamsPaymentIntentDataSetupFutureUsage>,
    #[doc = "Shipping information for this payment."]
    pub shipping: Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentIntentDataShipping>,
    #[doc = "Text that appears on the customer's statement as the statement descriptor for a non-card charge."]
    pub statement_descriptor: Option<String>,
    #[doc = "Provides information about a card charge."]
    pub statement_descriptor_suffix: Option<String>,
    #[doc = "The parameters used to automatically create a Transfer when the payment succeeds."]
    pub transfer_data: Option<stripe_types_01::checkout::CreateCheckoutSessionParamsPaymentIntentDataTransferData>,
    #[doc = "A string that identifies the resulting payment as part of a group."]
    pub transfer_group: Option<String>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentIntentData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("application_fee_amount", self.application_fee_amount.as_ref());
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("description", self.description.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("on_behalf_of", self.on_behalf_of.as_ref());
        form.field_opt("receipt_email", self.receipt_email.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("shipping", self.shipping.as_ref());
        form.field_opt("statement_descriptor", self.statement_descriptor.as_ref());
        form.field_opt("statement_descriptor_suffix", self.statement_descriptor_suffix.as_ref());
        form.field_opt("transfer_data", self.transfer_data.as_ref());
        form.field_opt("transfer_group", self.transfer_group.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebit {
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: Option<stripe_types_02::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitCurrency>,
    #[doc = "Additional fields for Mandate creation"]
    pub mandate_options:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptions>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_04::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitSetupFutureUsage>,
    #[doc = "Controls when Stripe will attempt to debit the funds from the customer's account."]
    pub target_date: Option<String>,
    #[doc = "Verification method for the intent"]
    pub verification_method:
        Option<stripe_types_00::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitVerificationMethod>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("currency", self.currency.as_ref());
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("target_date", self.target_date.as_ref());
        form.field_opt("verification_method", self.verification_method.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsAfterpayClearpay {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAfterpayClearpayCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage,
    >,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsAfterpayClearpay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsAlma {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAlmaCaptureMethod>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsAlma {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsBillie {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsBillieCaptureMethod>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsBillie {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsBoleto {
    #[doc = "The number of calendar days before a Boleto voucher expires."]
    pub expires_after_days: Option<i64>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsBoletoSetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsBoleto {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("expires_after_days", self.expires_after_days.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsCard {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsCardCaptureMethod>,
    #[doc = "Installment options for card payments"]
    pub installments:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsCardInstallments>,
    #[doc = "Request ability to [capture beyond the standard authorization validity window](/payments/extended-authorization) for ..."]
    pub request_extended_authorization: Option<
        stripe_types_00::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestExtendedAuthorization,
    >,
    #[doc = "Request ability to [increment the authorization](/payments/incremental-authorization) for this CheckoutSession."]
    pub request_incremental_authorization: Option<
        stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestIncrementalAuthorization,
    >,
    #[doc = "Request ability to make [multiple captures](/payments/multicapture) for this CheckoutSession."]
    pub request_multicapture:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestMulticapture>,
    #[doc = "Request ability to [overcapture](/payments/overcapture) for this CheckoutSession."]
    pub request_overcapture:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestOvercapture>,
    #[doc = "We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based..."]
    pub request_three_d_secure:
        Option<stripe_types_03::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsCardRequestThreeDSecure>,
    #[doc = "Restrictions to apply to the card payment method."]
    pub restrictions:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsCardRestrictions>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsCardSetupFutureUsage>,
    #[doc = "Provides information about a card payment that customers see on their statements."]
    pub statement_descriptor_suffix_kana: Option<String>,
    #[doc = "Provides information about a card payment that customers see on their statements."]
    pub statement_descriptor_suffix_kanji: Option<String>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsCard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("installments", self.installments.as_ref());
        form.field_opt("request_extended_authorization", self.request_extended_authorization.as_ref());
        form.field_opt(
            "request_incremental_authorization",
            self.request_incremental_authorization.as_ref(),
        );
        form.field_opt("request_multicapture", self.request_multicapture.as_ref());
        form.field_opt("request_overcapture", self.request_overcapture.as_ref());
        form.field_opt("request_three_d_secure", self.request_three_d_secure.as_ref());
        form.field_opt("restrictions", self.restrictions.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt(
            "statement_descriptor_suffix_kana",
            self.statement_descriptor_suffix_kana.as_ref(),
        );
        form.field_opt(
            "statement_descriptor_suffix_kanji",
            self.statement_descriptor_suffix_kanji.as_ref(),
        );
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsDemoPay {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsDemoPaySetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsDemoPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsKakaoPay {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsKakaoPayCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsKakaoPaySetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsKakaoPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsNaverPay {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsNaverPayCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsNaverPaySetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsNaverPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsPayco {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsPaycoCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsPaycoSetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsPayco {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsPaynow {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsPaynowSetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsPaynow {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsPayto {
    #[doc = "Additional fields for Mandate creation"]
    pub mandate_options:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptions>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsPaytoSetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsPayto {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptions {
    #[doc = "Amount to be charged for future payments."]
    pub amount: Option<i64>,
    #[doc = "Determines if the amount includes the IOF tax."]
    pub amount_includes_iof: Option<
        stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptionsAmountIncludesIof,
    >,
    #[doc = "Type of amount."]
    pub amount_type:
        Option<stripe_types_00::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptionsAmountType>,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: Option<String>,
    #[doc = "Date when the mandate expires and no further payments will be charged, in `YYYY-MM-DD`."]
    pub end_date: Option<String>,
    #[doc = "Schedule at which the future payments will be charged."]
    pub payment_schedule: Option<
        stripe_types_01::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule,
    >,
    #[doc = "Subscription name displayed to buyers in their bank app."]
    pub reference: Option<String>,
    #[doc = "Start date of the mandate, in `YYYY-MM-DD`."]
    pub start_date: Option<String>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsPixMandateOptions {
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

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsRevolutPay {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsRevolutPayCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsRevolutPaySetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsRevolutPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsSamsungPay {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsSamsungPayCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsSamsungPaySetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsSamsungPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsSepaDebit {
    #[doc = "Additional fields for Mandate creation"]
    pub mandate_options:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsSepaDebitMandateOptions>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsSepaDebitSetupFutureUsage>,
    #[doc = "Controls when Stripe will attempt to debit the funds from the customer's account."]
    pub target_date: Option<String>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsSepaDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("target_date", self.target_date.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsSunbit {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsSunbitCaptureMethod>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsSunbitSetupFutureUsage>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsSunbit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsUpiMandateOptions {
    #[doc = "Amount to be charged for future payments."]
    pub amount: Option<i64>,
    #[doc = "One of `fixed` or `maximum`."]
    pub amount_type:
        Option<stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsUpiMandateOptionsAmountType>,
    #[doc = "A description of the mandate or subscription that is meant to be displayed to the customer."]
    pub description: Option<String>,
    #[doc = "End date of the mandate or subscription."]
    pub end_date: Option<i64>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsUpiMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("amount_type", self.amount_type.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field_opt("end_date", self.end_date.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccount {
    #[doc = "Additional fields for Financial Connections Session creation"]
    pub financial_connections: Option<
        stripe_types_06::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountFinancialConnections,
    >,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountSetupFutureUsage>,
    #[doc = "Controls when Stripe will attempt to debit the funds from the customer's account."]
    pub target_date: Option<String>,
    #[doc = "Verification method for the intent"]
    pub verification_method: Option<
        stripe_types_00::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountVerificationMethod,
    >,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("financial_connections", self.financial_connections.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("target_date", self.target_date.as_ref());
        form.field_opt("verification_method", self.verification_method.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsSavedPaymentMethodOptions {
    #[doc = "Uses the `allow_redisplay` value of each saved payment method to filter the set presented to a returning customer."]
    pub allow_redisplay_filters: Option<
        Vec<stripe_types_06::checkout::CreateCheckoutSessionParamsSavedPaymentMethodOptionsAllowRedisplayFiltersItem>,
    >,
    #[doc = "Enable customers to choose if they wish to remove their saved payment methods."]
    pub payment_method_remove:
        Option<stripe_types_01::checkout::CreateCheckoutSessionParamsSavedPaymentMethodOptionsPaymentMethodRemove>,
    #[doc = "Enable customers to choose if they wish to save their payment method for future use."]
    pub payment_method_save:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsSavedPaymentMethodOptionsPaymentMethodSave>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsSavedPaymentMethodOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.allow_redisplay_filters.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("allow_redisplay_filters", index, item);
            }
        }
        form.field_opt("payment_method_remove", self.payment_method_remove.as_ref());
        form.field_opt("payment_method_save", self.payment_method_save.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone)]
pub struct CreateCheckoutSessionParamsSubscriptionDataPendingInvoiceItemInterval {
    #[doc = "Specifies invoicing frequency."]
    pub interval:
        stripe_types_06::checkout::CreateCheckoutSessionParamsSubscriptionDataPendingInvoiceItemIntervalInterval,
    #[doc = "The number of intervals between invoices."]
    pub interval_count: Option<i64>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsSubscriptionDataPendingInvoiceItemInterval {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("interval", &self.interval);
        form.field_opt("interval_count", self.interval_count.as_ref());
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
pub struct PaymentPagesCheckoutSessionBrandingSettings {
    #[doc = "A hex color value starting with `#` representing the background color for the Checkout Session."]
    #[json(default)]
    pub background_color: String,
    #[doc = "The border style for the Checkout Session."]
    #[json(default)]
    pub border_style: stripe_types_00::checkout::PaymentPagesCheckoutSessionBrandingSettingsBorderStyle,
    #[doc = "A hex color value starting with `#` representing the button color for the Checkout Session."]
    #[json(default)]
    pub button_color: String,
    #[doc = "The display name shown on the Checkout Session."]
    #[json(default)]
    pub display_name: String,
    #[doc = "The font family for the Checkout Session."]
    #[json(default)]
    pub font_family: String,
    #[doc = "The icon for the Checkout Session."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<stripe_types_06::checkout::PaymentPagesCheckoutSessionBrandingSettingsIcon>,
    #[doc = "The logo for the Checkout Session."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub logo: Option<stripe_types_05::checkout::PaymentPagesCheckoutSessionBrandingSettingsLogo>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentPagesCheckoutSessionCustomerDetails {
    #[doc = "The customer's address after a completed Checkout Session."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_01::common::Address>,
    #[doc = "The customer's business name after a completed Checkout Session."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    #[doc = "The email associated with the Customer, if one exists, on the Checkout Session after a completed Checkout Session or ..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "The customer's individual name after a completed Checkout Session."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub individual_name: Option<String>,
    #[doc = "The customer's name after a completed Checkout Session."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The customer's phone number after a completed Checkout Session."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "The customer’s tax exempt status after a completed Checkout Session."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<stripe_types_01::checkout::PaymentPagesCheckoutSessionCustomerDetailsTaxExempt>,
    #[doc = "The customer’s tax IDs after a completed Checkout Session."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax_ids: Option<Vec<stripe_types_05::checkout::PaymentPagesCheckoutSessionTaxId>>,
}
