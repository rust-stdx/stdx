#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutSessionPaymentMethodOptions {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<stripe_types_08::checkout::CheckoutAcssDebitPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub affirm: Option<stripe_types_06::checkout::CheckoutAffirmPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<stripe_types_05::checkout::CheckoutAfterpayClearpayPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub alipay: Option<stripe_types_05::checkout::CheckoutAlipayPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub alma: Option<stripe_types_06::checkout::CheckoutAlmaPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amazon_pay: Option<stripe_types_06::checkout::CheckoutAmazonPayPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<stripe_types_05::checkout::CheckoutAuBecsDebitPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<stripe_types_05::checkout::CheckoutBacsDebitPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<stripe_types_05::checkout::CheckoutBancontactPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billie: Option<stripe_types_05::checkout::CheckoutBilliePaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub boleto: Option<stripe_types_06::checkout::CheckoutBoletoPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types_08::checkout::CheckoutCardPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<stripe_types_06::checkout::CheckoutCashappPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<stripe_types_11::checkout::CheckoutCustomerBalancePaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub eps: Option<stripe_types_06::checkout::CheckoutEpsPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fpx: Option<stripe_types_05::checkout::CheckoutFpxPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub giropay: Option<stripe_types_06::checkout::CheckoutGiropayPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<stripe_types_05::checkout::CheckoutGrabPayPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ideal: Option<stripe_types_06::checkout::CheckoutIdealPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub kakao_pay: Option<stripe_types_05::checkout::CheckoutKakaoPayPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub klarna: Option<stripe_types_06::checkout::CheckoutKlarnaPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub konbini: Option<stripe_types_06::checkout::CheckoutKonbiniPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub kr_card: Option<stripe_types_05::checkout::CheckoutKrCardPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<stripe_types_05::checkout::CheckoutLinkPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mobilepay: Option<stripe_types_06::checkout::CheckoutMobilepayPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub multibanco: Option<stripe_types_06::checkout::CheckoutMultibancoPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub naver_pay: Option<stripe_types_06::checkout::CheckoutNaverPayPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<stripe_types_05::checkout::CheckoutOxxoPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub p24: Option<stripe_types_06::checkout::CheckoutP24PaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payco: Option<stripe_types_05::checkout::CheckoutPaycoPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paynow: Option<stripe_types_06::checkout::CheckoutPaynowPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paypal: Option<stripe_types_06::checkout::CheckoutPaypalPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payto: Option<stripe_types_08::checkout::CheckoutPaytoPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pix: Option<stripe_types_09::checkout::CheckoutPixPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub revolut_pay: Option<stripe_types_05::checkout::CheckoutRevolutPayPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub samsung_pay: Option<stripe_types_06::checkout::CheckoutSamsungPayPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub satispay: Option<stripe_types_06::checkout::CheckoutSatispayPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub scalapay: Option<stripe_types_05::checkout::CheckoutScalapayPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<stripe_types_06::checkout::CheckoutSepaDebitPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sequra: Option<stripe_types_06::checkout::CheckoutSequraPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sofort: Option<stripe_types_05::checkout::CheckoutSofortPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sunbit: Option<stripe_types_05::checkout::CheckoutSunbitPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub swish: Option<stripe_types_03::checkout::CheckoutSwishPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub twint: Option<stripe_types_05::checkout::CheckoutTwintPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub upi: Option<stripe_types_07::checkout::CheckoutUpiPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<stripe_types_10::checkout::CheckoutUsBankAccountPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<stripe_types_05::checkout::CheckoutWechatPayPaymentMethodOptions>,
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsInvoiceCreation {
    #[doc = "Set to `true` to enable invoice creation."]
    pub enabled: bool,
    #[doc = "Parameters passed when creating invoices for payment-mode Checkout Sessions."]
    pub invoice_data: Option<stripe_types_10::checkout::CreateCheckoutSessionParamsInvoiceCreationInvoiceData>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsInvoiceCreation {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("invoice_data", self.invoice_data.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptions {
    #[doc = "contains details about the ACSS Debit payment method options."]
    pub acss_debit: Option<stripe_types_09::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebit>,
    #[doc = "contains details about the Affirm payment method options."]
    pub affirm: Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAffirm>,
    #[doc = "contains details about the Afterpay Clearpay payment method options."]
    pub afterpay_clearpay:
        Option<stripe_types_09::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAfterpayClearpay>,
    #[doc = "contains details about the Alipay payment method options."]
    pub alipay: Option<stripe_types_07::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAlipay>,
    #[doc = "contains details about the Alma payment method options."]
    pub alma: Option<stripe_types_09::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAlma>,
    #[doc = "contains details about the AmazonPay payment method options."]
    pub amazon_pay: Option<stripe_types_07::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAmazonPay>,
    #[doc = "contains details about the AU Becs Debit payment method options."]
    pub au_becs_debit: Option<stripe_types_08::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsAuBecsDebit>,
    #[doc = "contains details about the Bacs Debit payment method options."]
    pub bacs_debit: Option<stripe_types_08::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsBacsDebit>,
    #[doc = "contains details about the Bancontact payment method options."]
    pub bancontact: Option<stripe_types_08::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsBancontact>,
    #[doc = "contains details about the Billie payment method options."]
    pub billie: Option<stripe_types_09::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsBillie>,
    #[doc = "contains details about the BLIK payment method options."]
    pub blik: Option<stripe_types_05::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsBlik>,
    #[doc = "contains details about the Boleto payment method options."]
    pub boleto: Option<stripe_types_09::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsBoleto>,
    #[doc = "contains details about the Card payment method options."]
    pub card: Option<stripe_types_09::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsCard>,
    #[doc = "contains details about the Cashapp Pay payment method options."]
    pub cashapp: Option<stripe_types_08::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsCashapp>,
    #[doc = "contains details about the Crypto payment method options."]
    pub crypto: Option<stripe_types_08::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsCrypto>,
    #[doc = "contains details about the Customer Balance payment method options."]
    pub customer_balance:
        Option<stripe_types_07::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalance>,
    #[doc = "contains details about the DemoPay payment method options."]
    pub demo_pay: Option<stripe_types_09::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsDemoPay>,
    #[doc = "contains details about the EPS payment method options."]
    pub eps: Option<stripe_types_08::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsEps>,
    #[doc = "contains details about the FPX payment method options."]
    pub fpx: Option<stripe_types_07::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsFpx>,
    #[doc = "contains details about the Giropay payment method options."]
    pub giropay: Option<stripe_types_07::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsGiropay>,
    #[doc = "contains details about the Grabpay payment method options."]
    pub grabpay: Option<stripe_types_08::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsGrabpay>,
    #[doc = "contains details about the Ideal payment method options."]
    pub ideal: Option<stripe_types_08::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsIdeal>,
    #[doc = "contains details about the Kakao Pay payment method options."]
    pub kakao_pay: Option<stripe_types_09::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsKakaoPay>,
    #[doc = "contains details about the Klarna payment method options."]
    pub klarna: Option<stripe_types_07::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsKlarna>,
    #[doc = "contains details about the Konbini payment method options."]
    pub konbini: Option<stripe_types_07::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsKonbini>,
    #[doc = "contains details about the Korean card payment method options."]
    pub kr_card: Option<stripe_types_08::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsKrCard>,
    #[doc = "contains details about the Link payment method options (Link is also known as Onelink in the UK)."]
    pub link: Option<stripe_types_08::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsLink>,
    #[doc = "contains details about the Mobilepay payment method options."]
    pub mobilepay: Option<stripe_types_08::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsMobilepay>,
    #[doc = "contains details about the Multibanco payment method options."]
    pub multibanco: Option<stripe_types_07::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsMultibanco>,
    #[doc = "contains details about the Naver Pay payment method options."]
    pub naver_pay: Option<stripe_types_09::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsNaverPay>,
    #[doc = "contains details about the OXXO payment method options."]
    pub oxxo: Option<stripe_types_07::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsOxxo>,
    #[doc = "contains details about the P24 payment method options."]
    pub p24: Option<stripe_types_08::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsP24>,
    #[doc = "contains details about the Pay By Bank payment method options."]
    pub pay_by_bank: Option<stripe_types_04::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsPayByBank>,
    #[doc = "contains details about the PAYCO payment method options."]
    pub payco: Option<stripe_types_09::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsPayco>,
    #[doc = "contains details about the PayNow payment method options."]
    pub paynow: Option<stripe_types_09::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsPaynow>,
    #[doc = "contains details about the PayPal payment method options."]
    pub paypal: Option<stripe_types_07::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsPaypal>,
    #[doc = "contains details about the PayTo payment method options."]
    pub payto: Option<stripe_types_09::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsPayto>,
    #[doc = "contains details about the Pix payment method options."]
    pub pix: Option<stripe_types_11::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsPix>,
    #[doc = "contains details about the RevolutPay payment method options."]
    pub revolut_pay: Option<stripe_types_09::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsRevolutPay>,
    #[doc = "contains details about the Samsung Pay payment method options."]
    pub samsung_pay: Option<stripe_types_09::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsSamsungPay>,
    #[doc = "contains details about the Satispay payment method options."]
    pub satispay: Option<stripe_types_08::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsSatispay>,
    #[doc = "contains details about the Scalapay payment method options."]
    pub scalapay: Option<stripe_types_08::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsScalapay>,
    #[doc = "contains details about the Sepa Debit payment method options."]
    pub sepa_debit: Option<stripe_types_09::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsSepaDebit>,
    #[doc = "contains details about the SeQura payment method options."]
    pub sequra: Option<stripe_types_08::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsSequra>,
    #[doc = "contains details about the Sofort payment method options."]
    pub sofort: Option<stripe_types_07::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsSofort>,
    #[doc = "contains details about the Sunbit payment method options."]
    pub sunbit: Option<stripe_types_09::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsSunbit>,
    #[doc = "contains details about the Swish payment method options."]
    pub swish: Option<stripe_types_00::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsSwish>,
    #[doc = "contains details about the TWINT payment method options."]
    pub twint: Option<stripe_types_08::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsTwint>,
    #[doc = "contains details about the UPI payment method options."]
    pub upi: Option<stripe_types_10::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsUpi>,
    #[doc = "contains details about the Us Bank Account payment method options."]
    pub us_bank_account:
        Option<stripe_types_09::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccount>,
    #[doc = "contains details about the WeChat Pay payment method options."]
    pub wechat_pay: Option<stripe_types_08::checkout::CreateCheckoutSessionParamsPaymentMethodOptionsWechatPay>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("acss_debit", self.acss_debit.as_ref());
        form.field_opt("affirm", self.affirm.as_ref());
        form.field_opt("afterpay_clearpay", self.afterpay_clearpay.as_ref());
        form.field_opt("alipay", self.alipay.as_ref());
        form.field_opt("alma", self.alma.as_ref());
        form.field_opt("amazon_pay", self.amazon_pay.as_ref());
        form.field_opt("au_becs_debit", self.au_becs_debit.as_ref());
        form.field_opt("bacs_debit", self.bacs_debit.as_ref());
        form.field_opt("bancontact", self.bancontact.as_ref());
        form.field_opt("billie", self.billie.as_ref());
        form.field_opt("blik", self.blik.as_ref());
        form.field_opt("boleto", self.boleto.as_ref());
        form.field_opt("card", self.card.as_ref());
        form.field_opt("cashapp", self.cashapp.as_ref());
        form.field_opt("crypto", self.crypto.as_ref());
        form.field_opt("customer_balance", self.customer_balance.as_ref());
        form.field_opt("demo_pay", self.demo_pay.as_ref());
        form.field_opt("eps", self.eps.as_ref());
        form.field_opt("fpx", self.fpx.as_ref());
        form.field_opt("giropay", self.giropay.as_ref());
        form.field_opt("grabpay", self.grabpay.as_ref());
        form.field_opt("ideal", self.ideal.as_ref());
        form.field_opt("kakao_pay", self.kakao_pay.as_ref());
        form.field_opt("klarna", self.klarna.as_ref());
        form.field_opt("konbini", self.konbini.as_ref());
        form.field_opt("kr_card", self.kr_card.as_ref());
        form.field_opt("link", self.link.as_ref());
        form.field_opt("mobilepay", self.mobilepay.as_ref());
        form.field_opt("multibanco", self.multibanco.as_ref());
        form.field_opt("naver_pay", self.naver_pay.as_ref());
        form.field_opt("oxxo", self.oxxo.as_ref());
        form.field_opt("p24", self.p24.as_ref());
        form.field_opt("pay_by_bank", self.pay_by_bank.as_ref());
        form.field_opt("payco", self.payco.as_ref());
        form.field_opt("paynow", self.paynow.as_ref());
        form.field_opt("paypal", self.paypal.as_ref());
        form.field_opt("payto", self.payto.as_ref());
        form.field_opt("pix", self.pix.as_ref());
        form.field_opt("revolut_pay", self.revolut_pay.as_ref());
        form.field_opt("samsung_pay", self.samsung_pay.as_ref());
        form.field_opt("satispay", self.satispay.as_ref());
        form.field_opt("scalapay", self.scalapay.as_ref());
        form.field_opt("sepa_debit", self.sepa_debit.as_ref());
        form.field_opt("sequra", self.sequra.as_ref());
        form.field_opt("sofort", self.sofort.as_ref());
        form.field_opt("sunbit", self.sunbit.as_ref());
        form.field_opt("swish", self.swish.as_ref());
        form.field_opt("twint", self.twint.as_ref());
        form.field_opt("upi", self.upi.as_ref());
        form.field_opt("us_bank_account", self.us_bank_account.as_ref());
        form.field_opt("wechat_pay", self.wechat_pay.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsShippingOptionsItemShippingRateData {
    #[doc = "The estimated range for how long shipping will take, meant to be displayable to the customer."]
    pub delivery_estimate: Option<
        stripe_types_11::checkout::CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataDeliveryEstimate,
    >,
    #[doc = "The name of the shipping rate, meant to be displayable to the customer."]
    pub display_name: String,
    #[doc = "Describes a fixed amount to charge for shipping."]
    pub fixed_amount:
        Option<stripe_types_10::checkout::CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataFixedAmount>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Specifies whether the rate is considered inclusive of taxes or exclusive of taxes."]
    pub tax_behavior:
        Option<stripe_types_05::checkout::CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataTaxBehavior>,
    #[doc = "A [tax code](https://docs.stripe.com/tax/tax-categories) ID."]
    pub tax_code: Option<String>,
    #[doc = "The type of calculation to use on the shipping rate."]
    pub r#type: Option<stripe_types_01::checkout::CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataType>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsShippingOptionsItemShippingRateData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("delivery_estimate", self.delivery_estimate.as_ref());
        form.field("display_name", &self.display_name);
        form.field_opt("fixed_amount", self.fixed_amount.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
        form.field_opt("tax_code", self.tax_code.as_ref());
        form.field_opt("type", self.r#type.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsSubscriptionData {
    #[doc = "A non-negative decimal between 0 and 100, with at most two decimal places."]
    pub application_fee_percent: Option<f64>,
    #[doc = "A future timestamp to anchor the subscription's billing cycle for new subscriptions."]
    pub billing_cycle_anchor: Option<i64>,
    #[doc = "Configures when the subscription schedule's billing cycle anchors to a specific day of the week or month."]
    pub billing_cycle_anchor_config:
        Option<stripe_types_00::checkout::CreateCheckoutSessionParamsSubscriptionDataBillingCycleAnchorConfig>,
    #[doc = "Controls how prorations and invoices for subscriptions are calculated and orchestrated."]
    pub billing_mode: Option<stripe_types_07::checkout::CreateCheckoutSessionParamsSubscriptionDataBillingMode>,
    #[doc = "The tax rates that will apply to any subscription item that does not have `tax_rates` set."]
    pub default_tax_rates: Option<Vec<String>>,
    #[doc = "The subscription's description, meant to be displayable to the customer."]
    pub description: Option<String>,
    #[doc = "All invoices will be billed using the specified settings."]
    pub invoice_settings: Option<stripe_types_11::checkout::CreateCheckoutSessionParamsSubscriptionDataInvoiceSettings>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The account on behalf of which to charge, for each of the subscription's invoices."]
    pub on_behalf_of: Option<String>,
    #[doc = "Specifies an interval for how often to bill for any pending invoice items."]
    pub pending_invoice_item_interval:
        Option<stripe_types_09::checkout::CreateCheckoutSessionParamsSubscriptionDataPendingInvoiceItemInterval>,
    #[doc = "Determines how to handle prorations resulting from the `billing_cycle_anchor`."]
    pub proration_behavior:
        Option<stripe_types_04::checkout::CreateCheckoutSessionParamsSubscriptionDataProrationBehavior>,
    #[doc = "If specified, the funds from the subscription's invoices will be transferred to the destination and the ID of the res..."]
    pub transfer_data: Option<stripe_types_04::checkout::CreateCheckoutSessionParamsSubscriptionDataTransferData>,
    #[doc = "Unix timestamp representing the end of the trial period the customer will get before being charged for the first time."]
    pub trial_end: Option<i64>,
    #[doc = "Integer representing the number of trial period days before the customer is charged for the first time."]
    pub trial_period_days: Option<i64>,
    #[doc = "Settings related to subscription trials."]
    pub trial_settings: Option<stripe_types_08::checkout::CreateCheckoutSessionParamsSubscriptionDataTrialSettings>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsSubscriptionData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("application_fee_percent", self.application_fee_percent.as_ref());
        form.field_opt("billing_cycle_anchor", self.billing_cycle_anchor.as_ref());
        form.field_opt("billing_cycle_anchor_config", self.billing_cycle_anchor_config.as_ref());
        form.field_opt("billing_mode", self.billing_mode.as_ref());
        if let Some(values) = self.default_tax_rates.as_ref() {
            form.field_array("default_tax_rates", values.as_slice());
        }
        form.field_opt("description", self.description.as_ref());
        form.field_opt("invoice_settings", self.invoice_settings.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("on_behalf_of", self.on_behalf_of.as_ref());
        form.field_opt("pending_invoice_item_interval", self.pending_invoice_item_interval.as_ref());
        form.field_opt("proration_behavior", self.proration_behavior.as_ref());
        form.field_opt("transfer_data", self.transfer_data.as_ref());
        form.field_opt("trial_end", self.trial_end.as_ref());
        form.field_opt("trial_period_days", self.trial_period_days.as_ref());
        form.field_opt("trial_settings", self.trial_settings.as_ref());
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
pub struct PaymentPagesCheckoutSessionShippingCost {
    #[doc = "Total shipping cost before any discounts or taxes are applied."]
    #[json(default)]
    pub amount_subtotal: i64,
    #[doc = "Total tax amount applied due to shipping costs."]
    #[json(default)]
    pub amount_tax: i64,
    #[doc = "Total shipping cost after discounts and taxes are applied."]
    #[json(default)]
    pub amount_total: i64,
    #[doc = "The ID of the ShippingRate for this order."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<crate::Expandable<stripe_types_11::shipping_rates::ShippingRate>>,
    #[doc = "The taxes applied to the shipping rate."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub taxes: Option<Vec<stripe_types_07::common::LineItemsTaxAmount>>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentPagesCheckoutSessionShippingOption {
    #[doc = "A non-negative integer in cents representing how much to charge."]
    #[json(default)]
    pub shipping_amount: i64,
    #[doc = "The shipping rate."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<crate::Expandable<stripe_types_11::shipping_rates::ShippingRate>>,
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct UpdateCheckoutSessionParamsLineItemsItem {
    #[doc = "When set, provides configuration for this item’s quantity to be adjusted by the customer during Checkout."]
    pub adjustable_quantity:
        Option<stripe_types_05::checkout::UpdateCheckoutSessionParamsLineItemsItemAdjustableQuantity>,
    #[doc = "ID of an existing line item."]
    pub id: Option<String>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The ID of the [Price](https://docs.stripe.com/api/prices)."]
    pub price: Option<String>,
    #[doc = "Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline."]
    pub price_data: Option<stripe_types_11::checkout::UpdateCheckoutSessionParamsLineItemsItemPriceData>,
    #[doc = "The quantity of the line item being purchased."]
    pub quantity: Option<i64>,
    #[doc = "The [tax rates](https://docs.stripe.com/api/tax_rates) which apply to this line item."]
    pub tax_rates: Option<json::OwnedValue>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for UpdateCheckoutSessionParamsLineItemsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("adjustable_quantity", self.adjustable_quantity.as_ref());
        form.field_opt("id", self.id.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("price", self.price.as_ref());
        form.field_opt("price_data", self.price_data.as_ref());
        form.field_opt("quantity", self.quantity.as_ref());
    }
}
