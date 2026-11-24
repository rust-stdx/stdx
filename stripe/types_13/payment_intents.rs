#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParams {
    #[doc = "The list of payment method types allowed for use with this payment."]
    pub allowed_payment_method_types:
        Option<Vec<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsAllowedPaymentMethodTypesItem>>,
    #[doc = "Provides industry-specific information about the amount."]
    pub amount_details: Option<stripe_types_08::payment_intents::ConfirmPaymentIntentParamsAmountDetails>,
    #[doc = "Amount to confirm on the PaymentIntent."]
    pub amount_to_confirm: Option<i64>,
    #[doc = "Controls when the funds will be captured from the customer's account."]
    pub capture_method: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsCaptureMethod>,
    #[doc = "ID of the ConfirmationToken used to confirm this PaymentIntent."]
    pub confirmation_token: Option<String>,
    #[doc = "Set to `true` to fail the payment attempt if the PaymentIntent transitions into `requires_action`."]
    pub error_on_requires_action: Option<bool>,
    #[doc = "The list of payment method types to exclude from use with this payment."]
    pub excluded_payment_method_types: Option<json::OwnedValue>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Automations to be run during the PaymentIntent lifecycle"]
    pub hooks: Option<stripe_types_11::payment_intents::ConfirmPaymentIntentParamsHooks>,
    #[doc = "ID of the mandate that's used for this payment."]
    pub mandate: Option<String>,
    pub mandate_data: Option<json::OwnedValue>,
    #[doc = "Set to `true` to indicate that the customer isn't in your checkout flow during this payment attempt and can't authent..."]
    pub off_session: Option<json::OwnedValue>,
    #[doc = "Provides industry-specific information about the charge."]
    pub payment_details: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentDetails>,
    #[doc = "ID of the payment method (a PaymentMethod, Card, or compatible Source..."]
    pub payment_method: Option<String>,
    #[doc = "If provided, this hash will be used to create a PaymentMethod."]
    pub payment_method_data: Option<stripe_types_12::payment_intents::ConfirmPaymentIntentParamsPaymentMethodData>,
    #[doc = "Payment method-specific configuration for this PaymentIntent."]
    pub payment_method_options: Option<crate::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptions>,
    #[doc = "Options to configure Radar."]
    pub radar_options: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsRadarOptions>,
    #[doc = "Email address that the receipt for the resulting payment will be sent to."]
    pub receipt_email: Option<json::OwnedValue>,
    #[doc = "The URL to redirect your customer back to after they authenticate or cancel their payment on the payment method's app..."]
    pub return_url: Option<String>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsSetupFutureUsage>,
    #[doc = "Shipping information for this PaymentIntent."]
    pub shipping: Option<stripe_types_08::payment_intents::ConfirmPaymentIntentParamsShipping>,
    #[doc = "Set to `true` when confirming server-side and using Stripe.js, iOS, or Android client-side SDKs to handle the next ac..."]
    pub use_stripe_sdk: Option<bool>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.allowed_payment_method_types.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("allowed_payment_method_types", index, item);
            }
        }
        form.field_opt("amount_details", self.amount_details.as_ref());
        form.field_opt("amount_to_confirm", self.amount_to_confirm.as_ref());
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("confirmation_token", self.confirmation_token.as_ref());
        form.field_opt("error_on_requires_action", self.error_on_requires_action.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("hooks", self.hooks.as_ref());
        form.field_opt("mandate", self.mandate.as_ref());
        form.field_opt("payment_details", self.payment_details.as_ref());
        form.field_opt("payment_method", self.payment_method.as_ref());
        form.field_opt("payment_method_data", self.payment_method_data.as_ref());
        form.field_opt("payment_method_options", self.payment_method_options.as_ref());
        form.field_opt("radar_options", self.radar_options.as_ref());
        form.field_opt("return_url", self.return_url.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("shipping", self.shipping.as_ref());
        form.field_opt("use_stripe_sdk", self.use_stripe_sdk.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptions {
    #[doc = "If this is a `acss_debit` PaymentMethod, this sub-hash contains details about the ACSS Debit payment method options."]
    pub acss_debit: Option<stripe_types_11::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsAcssDebit>,
    #[doc = "If this is an `affirm` PaymentMethod, this sub-hash contains details about the Affirm payment method options."]
    pub affirm: Option<stripe_types_07::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsAffirm>,
    #[doc = "If this is a `afterpay_clearpay` PaymentMethod, this sub-hash contains details about the Afterpay Clearpay payment me..."]
    pub afterpay_clearpay:
        Option<stripe_types_07::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsAfterpayClearpay>,
    #[doc = "If this is a `alipay` PaymentMethod, this sub-hash contains details about the Alipay payment method options."]
    pub alipay: Option<stripe_types_09::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsAlipay>,
    #[doc = "If this is a `alma` PaymentMethod, this sub-hash contains details about the Alma payment method options."]
    pub alma: Option<stripe_types_07::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsAlma>,
    #[doc = "If this is a `amazon_pay` PaymentMethod, this sub-hash contains details about the Amazon Pay payment method options."]
    pub amazon_pay: Option<stripe_types_08::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsAmazonPay>,
    #[doc = "If this is a `au_becs_debit` PaymentMethod, this sub-hash contains details about the AU BECS Direct Debit payment met..."]
    pub au_becs_debit:
        Option<stripe_types_08::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsAuBecsDebit>,
    #[doc = "If this is a `bacs_debit` PaymentMethod, this sub-hash contains details about the BACS Debit payment method options."]
    pub bacs_debit: Option<stripe_types_07::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsBacsDebit>,
    #[doc = "If this is a `bancontact` PaymentMethod, this sub-hash contains details about the Bancontact payment method options."]
    pub bancontact: Option<stripe_types_07::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsBancontact>,
    #[doc = "If this is a `billie` PaymentMethod, this sub-hash contains details about the Billie payment method options."]
    pub billie: Option<stripe_types_10::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsBillie>,
    #[doc = "If this is a `bizum` PaymentMethod, this sub-hash contains details about the Bizum payment method options."]
    pub bizum: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsBizum>,
    #[doc = "If this is a `blik` PaymentMethod, this sub-hash contains details about the BLIK payment method options."]
    pub blik: Option<stripe_types_09::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsBlik>,
    #[doc = "If this is a `boleto` PaymentMethod, this sub-hash contains details about the Boleto payment method options."]
    pub boleto: Option<stripe_types_08::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsBoleto>,
    #[doc = "Configuration for any card payments attempted on this PaymentIntent."]
    pub card: Option<crate::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCard>,
    #[doc = "If this is a `card_present` PaymentMethod, this sub-hash contains details about the Card Present payment method options."]
    pub card_present:
        Option<stripe_types_10::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCardPresent>,
    #[doc = "If this is a `cashapp` PaymentMethod, this sub-hash contains details about the Cash App Pay payment method options."]
    pub cashapp: Option<stripe_types_07::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCashapp>,
    #[doc = "If this is a `crypto` PaymentMethod, this sub-hash contains details about the Crypto payment method options."]
    pub crypto: Option<stripe_types_09::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCrypto>,
    #[doc = "If this is a `customer balance` PaymentMethod, this sub-hash contains details about the customer balance payment meth..."]
    pub customer_balance:
        Option<stripe_types_10::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCustomerBalance>,
    #[doc = "If this is a `eps` PaymentMethod, this sub-hash contains details about the EPS payment method options."]
    pub eps: Option<stripe_types_07::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsEps>,
    #[doc = "If this is a `fpx` PaymentMethod, this sub-hash contains details about the FPX payment method options."]
    pub fpx: Option<stripe_types_07::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsFpx>,
    #[doc = "If this is a `giropay` PaymentMethod, this sub-hash contains details about the Giropay payment method options."]
    pub giropay: Option<stripe_types_07::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsGiropay>,
    #[doc = "If this is a `grabpay` PaymentMethod, this sub-hash contains details about the Grabpay payment method options."]
    pub grabpay: Option<stripe_types_08::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsGrabpay>,
    #[doc = "If this is a `ideal` PaymentMethod, this sub-hash contains details about the Ideal payment method options."]
    pub ideal: Option<stripe_types_09::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsIdeal>,
    #[doc = "If this is a `interac_present` PaymentMethod, this sub-hash contains details about the Card Present payment method op..."]
    pub interac_present:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsInteracPresent>,
    #[doc = "If this is a `kakao_pay` PaymentMethod, this sub-hash contains details about the Kakao Pay payment method options."]
    pub kakao_pay: Option<stripe_types_09::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsKakaoPay>,
    #[doc = "If this is a `klarna` PaymentMethod, this sub-hash contains details about the Klarna payment method options."]
    pub klarna: Option<stripe_types_10::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsKlarna>,
    #[doc = "If this is a `konbini` PaymentMethod, this sub-hash contains details about the Konbini payment method options."]
    pub konbini: Option<stripe_types_07::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsKonbini>,
    #[doc = "If this is a `kr_card` PaymentMethod, this sub-hash contains details about the KR Card payment method options."]
    pub kr_card: Option<stripe_types_07::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsKrCard>,
    #[doc = "If this is a `link` PaymentMethod, this sub-hash contains details about the Link payment method options (Link is also..."]
    pub link: Option<stripe_types_08::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsLink>,
    #[doc = "If this is a `mb_way` PaymentMethod, this sub-hash contains details about the MB WAY payment method options."]
    pub mb_way: Option<stripe_types_07::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsMbWay>,
    #[doc = "If this is a `MobilePay` PaymentMethod, this sub-hash contains details about the MobilePay payment method options."]
    pub mobilepay: Option<stripe_types_07::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsMobilepay>,
    #[doc = "If this is a `multibanco` PaymentMethod, this sub-hash contains details about the Multibanco payment method options."]
    pub multibanco: Option<stripe_types_07::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsMultibanco>,
    #[doc = "If this is a `naver_pay` PaymentMethod, this sub-hash contains details about the Naver Pay payment method options."]
    pub naver_pay: Option<stripe_types_07::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsNaverPay>,
    #[doc = "If this is a `nz_bank_account` PaymentMethod, this sub-hash contains details about the NZ BECS Direct Debit payment m..."]
    pub nz_bank_account:
        Option<stripe_types_07::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsNzBankAccount>,
    #[doc = "If this is a `oxxo` PaymentMethod, this sub-hash contains details about the OXXO payment method options."]
    pub oxxo: Option<stripe_types_08::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsOxxo>,
    #[doc = "If this is a `p24` PaymentMethod, this sub-hash contains details about the Przelewy24 payment method options."]
    pub p24: Option<stripe_types_09::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsP24>,
    #[doc = "If this is a `pay_by_bank` PaymentMethod, this sub-hash contains details about the PayByBank payment method options."]
    pub pay_by_bank: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsPayByBank>,
    #[doc = "If this is a `payco` PaymentMethod, this sub-hash contains details about the PAYCO payment method options."]
    pub payco: Option<stripe_types_07::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsPayco>,
    #[doc = "If this is a `paynow` PaymentMethod, this sub-hash contains details about the PayNow payment method options."]
    pub paynow: Option<stripe_types_07::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsPaynow>,
    #[doc = "If this is a `paypal` PaymentMethod, this sub-hash contains details about the PayPal payment method options."]
    pub paypal: Option<stripe_types_09::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsPaypal>,
    #[doc = "If this is a `paypay` PaymentMethod, this sub-hash contains details about the PayPay payment method options."]
    pub paypay: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsPaypay>,
    #[doc = "If this is a `payto` PaymentMethod, this sub-hash contains details about the PayTo payment method options."]
    pub payto: Option<stripe_types_10::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsPayto>,
    #[doc = "If this is a `pix` PaymentMethod, this sub-hash contains details about the Pix payment method options."]
    pub pix: Option<stripe_types_10::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsPix>,
    #[doc = "If this is a `promptpay` PaymentMethod, this sub-hash contains details about the PromptPay payment method options."]
    pub promptpay: Option<stripe_types_09::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsPromptpay>,
    #[doc = "If this is a `revolut_pay` PaymentMethod, this sub-hash contains details about the Revolut Pay payment method options."]
    pub revolut_pay: Option<stripe_types_09::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsRevolutPay>,
    #[doc = "If this is a `samsung_pay` PaymentMethod, this sub-hash contains details about the Samsung Pay payment method options."]
    pub samsung_pay: Option<stripe_types_08::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsSamsungPay>,
    #[doc = "If this is a `satispay` PaymentMethod, this sub-hash contains details about the Satispay payment method options."]
    pub satispay: Option<stripe_types_09::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsSatispay>,
    #[doc = "If this is a `scalapay` PaymentMethod, this sub-hash contains details about the ScalaPay payment method options."]
    pub scalapay: Option<stripe_types_08::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsScalapay>,
    #[doc = "If this is a `sepa_debit` PaymentIntent, this sub-hash contains details about the SEPA Debit payment method options."]
    pub sepa_debit: Option<stripe_types_08::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsSepaDebit>,
    #[doc = "If this is a `sequra` PaymentMethod, this sub-hash contains details about the SeQura payment method options."]
    pub sequra: Option<stripe_types_09::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsSequra>,
    #[doc = "If this is a `sofort` PaymentMethod, this sub-hash contains details about the SOFORT payment method options."]
    pub sofort: Option<stripe_types_08::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsSofort>,
    #[doc = "If this is a `sunbit` PaymentMethod, this sub-hash contains details about the Sunbit payment method options."]
    pub sunbit: Option<stripe_types_08::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsSunbit>,
    #[doc = "If this is a `Swish` PaymentMethod, this sub-hash contains details about the Swish payment method options."]
    pub swish: Option<stripe_types_09::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsSwish>,
    #[doc = "If this is a `twint` PaymentMethod, this sub-hash contains details about the TWINT payment method options."]
    pub twint: Option<stripe_types_08::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsTwint>,
    #[doc = "If this is a `upi` PaymentIntent, this sub-hash contains details about the UPI payment method options."]
    pub upi: Option<stripe_types_11::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsUpi>,
    #[doc = "If this is a `us_bank_account` PaymentMethod, this sub-hash contains details about the US bank account payment method..."]
    pub us_bank_account:
        Option<stripe_types_12::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsUsBankAccount>,
    #[doc = "If this is a `wechat_pay` PaymentMethod, this sub-hash contains details about the WeChat Pay payment method options."]
    pub wechat_pay: Option<stripe_types_07::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsWechatPay>,
    #[doc = "If this is a `zip` PaymentMethod, this sub-hash contains details about the Zip payment method options."]
    pub zip: Option<stripe_types_08::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsZip>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptions {
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
        form.field_opt("bizum", self.bizum.as_ref());
        form.field_opt("blik", self.blik.as_ref());
        form.field_opt("boleto", self.boleto.as_ref());
        form.field_opt("card", self.card.as_ref());
        form.field_opt("card_present", self.card_present.as_ref());
        form.field_opt("cashapp", self.cashapp.as_ref());
        form.field_opt("crypto", self.crypto.as_ref());
        form.field_opt("customer_balance", self.customer_balance.as_ref());
        form.field_opt("eps", self.eps.as_ref());
        form.field_opt("fpx", self.fpx.as_ref());
        form.field_opt("giropay", self.giropay.as_ref());
        form.field_opt("grabpay", self.grabpay.as_ref());
        form.field_opt("ideal", self.ideal.as_ref());
        form.field_opt("interac_present", self.interac_present.as_ref());
        form.field_opt("kakao_pay", self.kakao_pay.as_ref());
        form.field_opt("klarna", self.klarna.as_ref());
        form.field_opt("konbini", self.konbini.as_ref());
        form.field_opt("kr_card", self.kr_card.as_ref());
        form.field_opt("link", self.link.as_ref());
        form.field_opt("mb_way", self.mb_way.as_ref());
        form.field_opt("mobilepay", self.mobilepay.as_ref());
        form.field_opt("multibanco", self.multibanco.as_ref());
        form.field_opt("naver_pay", self.naver_pay.as_ref());
        form.field_opt("nz_bank_account", self.nz_bank_account.as_ref());
        form.field_opt("oxxo", self.oxxo.as_ref());
        form.field_opt("p24", self.p24.as_ref());
        form.field_opt("pay_by_bank", self.pay_by_bank.as_ref());
        form.field_opt("payco", self.payco.as_ref());
        form.field_opt("paynow", self.paynow.as_ref());
        form.field_opt("paypal", self.paypal.as_ref());
        form.field_opt("paypay", self.paypay.as_ref());
        form.field_opt("payto", self.payto.as_ref());
        form.field_opt("pix", self.pix.as_ref());
        form.field_opt("promptpay", self.promptpay.as_ref());
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
        form.field_opt("zip", self.zip.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsCard {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCardCaptureMethod>,
    #[doc = "A single-use `cvc_update` Token that represents a card CVC value."]
    pub cvc_token: Option<String>,
    #[doc = "Installment configuration for payments attempted on this PaymentIntent."]
    pub installments: Option<stripe_types_10::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCardInstallments>,
    #[doc = "Configuration options for setting up an eMandate for cards issued in India."]
    pub mandate_options: Option<stripe_types_08::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCardMandateOptions>,
    #[doc = "When specified, this parameter indicates that a transaction will be marked as MOTO (Mail Order Telephone Order) and t..."]
    pub moto: Option<bool>,
    #[doc = "Selected network to process this PaymentIntent on."]
    pub network: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCardNetwork>,
    #[doc = "Request ability to capture beyond the standard authorization validity window..."]
    pub request_extended_authorization: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCardRequestExtendedAuthorization>,
    #[doc = "Request ability to [increment the authorization](https://docs.stripe.com/payments/incremental-authorization) for this..."]
    pub request_incremental_authorization: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCardRequestIncrementalAuthorization>,
    #[doc = "Request ability to make [multiple captures](https://docs.stripe.com/payments/multicapture) for this PaymentIntent."]
    pub request_multicapture: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCardRequestMulticapture>,
    #[doc = "Request ability to [overcapture](https://docs.stripe.com/payments/overcapture) for this PaymentIntent."]
    pub request_overcapture: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCardRequestOvercapture>,
    #[doc = "We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based..."]
    pub request_three_d_secure: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCardRequestThreeDSecure>,
    #[doc = "When enabled, using a card that is attached to a customer will require the CVC to be provided again (i.e."]
    pub require_cvc_recollection: Option<bool>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCardSetupFutureUsage>,
    #[doc = "Provides information about a card payment that customers see on their statements."]
    pub statement_descriptor_suffix_kana: Option<json::OwnedValue>,
    #[doc = "Provides information about a card payment that customers see on their statements."]
    pub statement_descriptor_suffix_kanji: Option<json::OwnedValue>,
    #[doc = "If 3D Secure authentication was performed with a third-party provider, the authentication details to use for this pay..."]
    pub three_d_secure: Option<stripe_types_12::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCardThreeDSecure>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsCard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("cvc_token", self.cvc_token.as_ref());
        form.field_opt("installments", self.installments.as_ref());
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("moto", self.moto.as_ref());
        form.field_opt("network", self.network.as_ref());
        form.field_opt("request_extended_authorization", self.request_extended_authorization.as_ref());
        form.field_opt(
            "request_incremental_authorization",
            self.request_incremental_authorization.as_ref(),
        );
        form.field_opt("request_multicapture", self.request_multicapture.as_ref());
        form.field_opt("request_overcapture", self.request_overcapture.as_ref());
        form.field_opt("request_three_d_secure", self.request_three_d_secure.as_ref());
        form.field_opt("require_cvc_recollection", self.require_cvc_recollection.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("three_d_secure", self.three_d_secure.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParams {
    #[doc = "The list of payment method types allowed for use with this payment."]
    pub allowed_payment_method_types:
        Option<Vec<stripe_types_04::payment_intents::CreatePaymentIntentParamsAllowedPaymentMethodTypesItem>>,
    #[doc = "Amount intended to be collected by this PaymentIntent."]
    pub amount: i64,
    #[doc = "Provides industry-specific information about the amount."]
    pub amount_details: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsAmountDetails>,
    #[doc = "The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the..."]
    pub application_fee_amount: Option<i64>,
    #[doc = "When you enable this parameter, this PaymentIntent accepts payment methods that you enable in the Dashboard and that ..."]
    pub automatic_payment_methods:
        Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsAutomaticPaymentMethods>,
    #[doc = "Controls when the funds will be captured from the customer's account."]
    pub capture_method: Option<stripe_types_03::payment_intents::CreatePaymentIntentParamsCaptureMethod>,
    #[doc = "Set to `true` to attempt to [confirm this PaymentIntent](https://docs.stripe.com/api/payment_intents/confirm) immedia..."]
    pub confirm: Option<bool>,
    #[doc = "Describes whether we can confirm this PaymentIntent automatically, or if it requires customer action to confirm the p..."]
    pub confirmation_method: Option<stripe_types_02::payment_intents::CreatePaymentIntentParamsConfirmationMethod>,
    #[doc = "ID of the ConfirmationToken used to confirm this PaymentIntent."]
    pub confirmation_token: Option<String>,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "ID of the Customer this PaymentIntent belongs to, if one exists."]
    pub customer: Option<String>,
    #[doc = "ID of the Account representing the customer that this PaymentIntent belongs to, if one exists."]
    pub customer_account: Option<String>,
    #[doc = "An arbitrary string attached to the object."]
    pub description: Option<String>,
    #[doc = "Set to `true` to fail the payment attempt if the PaymentIntent transitions into `requires_action`."]
    pub error_on_requires_action: Option<bool>,
    #[doc = "The list of payment method types to exclude from use with this payment."]
    pub excluded_payment_method_types:
        Option<Vec<stripe_types_00::payment_intents::CreatePaymentIntentParamsExcludedPaymentMethodTypesItem>>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Automations to be run during the PaymentIntent lifecycle"]
    pub hooks: Option<stripe_types_07::payment_intents::CreatePaymentIntentParamsHooks>,
    #[doc = "ID of the mandate that's used for this payment."]
    pub mandate: Option<String>,
    #[doc = "This hash contains details about the Mandate to create."]
    pub mandate_data: Option<stripe_types_09::payment_intents::CreatePaymentIntentParamsMandateData>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Set to `true` to indicate that the customer isn't in your checkout flow during this payment attempt and can't authent..."]
    pub off_session: Option<json::OwnedValue>,
    #[doc = "The Stripe account ID that these funds are intended for."]
    pub on_behalf_of: Option<String>,
    #[doc = "Provides industry-specific information about the charge."]
    pub payment_details: Option<stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentDetails>,
    #[doc = "ID of the payment method (a PaymentMethod, Card, or compatible Source..."]
    pub payment_method: Option<String>,
    #[doc = "The ID of the [payment method configuration](https://docs.stripe.com/api/payment_method_configurations) to use with t..."]
    pub payment_method_configuration: Option<String>,
    #[doc = "If provided, this hash will be used to create a PaymentMethod."]
    pub payment_method_data: Option<stripe_types_11::payment_intents::CreatePaymentIntentParamsPaymentMethodData>,
    #[doc = "Payment method-specific configuration for this PaymentIntent."]
    pub payment_method_options: Option<crate::payment_intents::CreatePaymentIntentParamsPaymentMethodOptions>,
    #[doc = "Options to configure Radar."]
    pub radar_options: Option<stripe_types_03::payment_intents::CreatePaymentIntentParamsRadarOptions>,
    #[doc = "Email address to send the receipt to."]
    pub receipt_email: Option<String>,
    #[doc = "The URL to redirect your customer back to after they authenticate or cancel their payment on the payment method's app..."]
    pub return_url: Option<String>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<stripe_types_00::payment_intents::CreatePaymentIntentParamsSetupFutureUsage>,
    #[doc = "Shipping information for this PaymentIntent."]
    pub shipping: Option<stripe_types_07::payment_intents::CreatePaymentIntentParamsShipping>,
    #[doc = "Text that appears on the customer's statement as the statement descriptor for a non-card charge."]
    pub statement_descriptor: Option<String>,
    #[doc = "Provides information about a card charge."]
    pub statement_descriptor_suffix: Option<String>,
    #[doc = "The parameters that you can use to automatically create a Transfer."]
    pub transfer_data: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsTransferData>,
    #[doc = "A string that identifies the resulting payment as part of a group."]
    pub transfer_group: Option<String>,
    #[doc = "Set to `true` when confirming server-side and using Stripe.js, iOS, or Android client-side SDKs to handle the next ac..."]
    pub use_stripe_sdk: Option<bool>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.allowed_payment_method_types.as_ref() {
            form.field_array("allowed_payment_method_types", values.as_slice());
        }
        form.field("amount", &self.amount);
        form.field_opt("amount_details", self.amount_details.as_ref());
        form.field_opt("application_fee_amount", self.application_fee_amount.as_ref());
        form.field_opt("automatic_payment_methods", self.automatic_payment_methods.as_ref());
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("confirm", self.confirm.as_ref());
        form.field_opt("confirmation_method", self.confirmation_method.as_ref());
        form.field_opt("confirmation_token", self.confirmation_token.as_ref());
        form.field("currency", &self.currency);
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("customer_account", self.customer_account.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field_opt("error_on_requires_action", self.error_on_requires_action.as_ref());
        if let Some(values) = self.excluded_payment_method_types.as_ref() {
            form.field_array("excluded_payment_method_types", values.as_slice());
        }
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("hooks", self.hooks.as_ref());
        form.field_opt("mandate", self.mandate.as_ref());
        form.field_opt("mandate_data", self.mandate_data.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("on_behalf_of", self.on_behalf_of.as_ref());
        form.field_opt("payment_details", self.payment_details.as_ref());
        form.field_opt("payment_method", self.payment_method.as_ref());
        form.field_opt("payment_method_configuration", self.payment_method_configuration.as_ref());
        form.field_opt("payment_method_data", self.payment_method_data.as_ref());
        form.field_opt("payment_method_options", self.payment_method_options.as_ref());
        form.field_opt("radar_options", self.radar_options.as_ref());
        form.field_opt("receipt_email", self.receipt_email.as_ref());
        form.field_opt("return_url", self.return_url.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("shipping", self.shipping.as_ref());
        form.field_opt("statement_descriptor", self.statement_descriptor.as_ref());
        form.field_opt("statement_descriptor_suffix", self.statement_descriptor_suffix.as_ref());
        form.field_opt("transfer_data", self.transfer_data.as_ref());
        form.field_opt("transfer_group", self.transfer_group.as_ref());
        form.field_opt("use_stripe_sdk", self.use_stripe_sdk.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptions {
    #[doc = "If this is a `acss_debit` PaymentMethod, this sub-hash contains details about the ACSS Debit payment method options."]
    pub acss_debit: Option<stripe_types_08::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAcssDebit>,
    #[doc = "If this is an `affirm` PaymentMethod, this sub-hash contains details about the Affirm payment method options."]
    pub affirm: Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirm>,
    #[doc = "If this is a `afterpay_clearpay` PaymentMethod, this sub-hash contains details about the Afterpay Clearpay payment me..."]
    pub afterpay_clearpay:
        Option<stripe_types_09::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAfterpayClearpay>,
    #[doc = "If this is a `alipay` PaymentMethod, this sub-hash contains details about the Alipay payment method options."]
    pub alipay: Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAlipay>,
    #[doc = "If this is a `alma` PaymentMethod, this sub-hash contains details about the Alma payment method options."]
    pub alma: Option<stripe_types_07::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAlma>,
    #[doc = "If this is a `amazon_pay` PaymentMethod, this sub-hash contains details about the Amazon Pay payment method options."]
    pub amazon_pay: Option<stripe_types_08::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAmazonPay>,
    #[doc = "If this is a `au_becs_debit` PaymentMethod, this sub-hash contains details about the AU BECS Direct Debit payment met..."]
    pub au_becs_debit:
        Option<stripe_types_09::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAuBecsDebit>,
    #[doc = "If this is a `bacs_debit` PaymentMethod, this sub-hash contains details about the BACS Debit payment method options."]
    pub bacs_debit: Option<stripe_types_08::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsBacsDebit>,
    #[doc = "If this is a `bancontact` PaymentMethod, this sub-hash contains details about the Bancontact payment method options."]
    pub bancontact: Option<stripe_types_09::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsBancontact>,
    #[doc = "If this is a `billie` PaymentMethod, this sub-hash contains details about the Billie payment method options."]
    pub billie: Option<stripe_types_10::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsBillie>,
    #[doc = "If this is a `bizum` PaymentMethod, this sub-hash contains details about the Bizum payment method options."]
    pub bizum: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsBizum>,
    #[doc = "If this is a `blik` PaymentMethod, this sub-hash contains details about the BLIK payment method options."]
    pub blik: Option<stripe_types_09::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsBlik>,
    #[doc = "If this is a `boleto` PaymentMethod, this sub-hash contains details about the Boleto payment method options."]
    pub boleto: Option<stripe_types_07::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsBoleto>,
    #[doc = "Configuration for any card payments attempted on this PaymentIntent."]
    pub card: Option<stripe_types_12::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCard>,
    #[doc = "If this is a `card_present` PaymentMethod, this sub-hash contains details about the Card Present payment method options."]
    pub card_present:
        Option<stripe_types_07::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardPresent>,
    #[doc = "If this is a `cashapp` PaymentMethod, this sub-hash contains details about the Cash App Pay payment method options."]
    pub cashapp: Option<stripe_types_07::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCashapp>,
    #[doc = "If this is a `crypto` PaymentMethod, this sub-hash contains details about the Crypto payment method options."]
    pub crypto: Option<stripe_types_09::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCrypto>,
    #[doc = "If this is a `customer balance` PaymentMethod, this sub-hash contains details about the customer balance payment meth..."]
    pub customer_balance:
        Option<stripe_types_09::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalance>,
    #[doc = "If this is a `eps` PaymentMethod, this sub-hash contains details about the EPS payment method options."]
    pub eps: Option<stripe_types_09::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsEps>,
    #[doc = "If this is a `fpx` PaymentMethod, this sub-hash contains details about the FPX payment method options."]
    pub fpx: Option<stripe_types_07::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsFpx>,
    #[doc = "If this is a `giropay` PaymentMethod, this sub-hash contains details about the Giropay payment method options."]
    pub giropay: Option<stripe_types_08::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsGiropay>,
    #[doc = "If this is a `grabpay` PaymentMethod, this sub-hash contains details about the Grabpay payment method options."]
    pub grabpay: Option<stripe_types_09::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsGrabpay>,
    #[doc = "If this is a `ideal` PaymentMethod, this sub-hash contains details about the Ideal payment method options."]
    pub ideal: Option<stripe_types_09::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsIdeal>,
    #[doc = "If this is a `interac_present` PaymentMethod, this sub-hash contains details about the Card Present payment method op..."]
    pub interac_present:
        Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsInteracPresent>,
    #[doc = "If this is a `kakao_pay` PaymentMethod, this sub-hash contains details about the Kakao Pay payment method options."]
    pub kakao_pay: Option<stripe_types_07::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsKakaoPay>,
    #[doc = "If this is a `klarna` PaymentMethod, this sub-hash contains details about the Klarna payment method options."]
    pub klarna: Option<stripe_types_09::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsKlarna>,
    #[doc = "If this is a `konbini` PaymentMethod, this sub-hash contains details about the Konbini payment method options."]
    pub konbini: Option<stripe_types_09::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsKonbini>,
    #[doc = "If this is a `kr_card` PaymentMethod, this sub-hash contains details about the KR Card payment method options."]
    pub kr_card: Option<stripe_types_08::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsKrCard>,
    #[doc = "If this is a `link` PaymentMethod, this sub-hash contains details about the Link payment method options (Link is also..."]
    pub link: Option<stripe_types_08::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsLink>,
    #[doc = "If this is a `mb_way` PaymentMethod, this sub-hash contains details about the MB WAY payment method options."]
    pub mb_way: Option<stripe_types_08::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsMbWay>,
    #[doc = "If this is a `MobilePay` PaymentMethod, this sub-hash contains details about the MobilePay payment method options."]
    pub mobilepay: Option<stripe_types_07::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsMobilepay>,
    #[doc = "If this is a `multibanco` PaymentMethod, this sub-hash contains details about the Multibanco payment method options."]
    pub multibanco: Option<stripe_types_09::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsMultibanco>,
    #[doc = "If this is a `naver_pay` PaymentMethod, this sub-hash contains details about the Naver Pay payment method options."]
    pub naver_pay: Option<stripe_types_09::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsNaverPay>,
    #[doc = "If this is a `nz_bank_account` PaymentMethod, this sub-hash contains details about the NZ BECS Direct Debit payment m..."]
    pub nz_bank_account:
        Option<stripe_types_09::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsNzBankAccount>,
    #[doc = "If this is a `oxxo` PaymentMethod, this sub-hash contains details about the OXXO payment method options."]
    pub oxxo: Option<stripe_types_09::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsOxxo>,
    #[doc = "If this is a `p24` PaymentMethod, this sub-hash contains details about the Przelewy24 payment method options."]
    pub p24: Option<stripe_types_08::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsP24>,
    #[doc = "If this is a `pay_by_bank` PaymentMethod, this sub-hash contains details about the PayByBank payment method options."]
    pub pay_by_bank: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPayByBank>,
    #[doc = "If this is a `payco` PaymentMethod, this sub-hash contains details about the PAYCO payment method options."]
    pub payco: Option<stripe_types_07::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPayco>,
    #[doc = "If this is a `paynow` PaymentMethod, this sub-hash contains details about the PayNow payment method options."]
    pub paynow: Option<stripe_types_07::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPaynow>,
    #[doc = "If this is a `paypal` PaymentMethod, this sub-hash contains details about the PayPal payment method options."]
    pub paypal: Option<stripe_types_09::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPaypal>,
    #[doc = "If this is a `paypay` PaymentMethod, this sub-hash contains details about the PayPay payment method options."]
    pub paypay: Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPaypay>,
    #[doc = "If this is a `payto` PaymentMethod, this sub-hash contains details about the PayTo payment method options."]
    pub payto: Option<stripe_types_07::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPayto>,
    #[doc = "If this is a `pix` PaymentMethod, this sub-hash contains details about the Pix payment method options."]
    pub pix: Option<stripe_types_10::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPix>,
    #[doc = "If this is a `promptpay` PaymentMethod, this sub-hash contains details about the PromptPay payment method options."]
    pub promptpay: Option<stripe_types_08::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPromptpay>,
    #[doc = "If this is a `revolut_pay` PaymentMethod, this sub-hash contains details about the Revolut Pay payment method options."]
    pub revolut_pay: Option<stripe_types_08::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsRevolutPay>,
    #[doc = "If this is a `samsung_pay` PaymentMethod, this sub-hash contains details about the Samsung Pay payment method options."]
    pub samsung_pay: Option<stripe_types_09::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsSamsungPay>,
    #[doc = "If this is a `satispay` PaymentMethod, this sub-hash contains details about the Satispay payment method options."]
    pub satispay: Option<stripe_types_08::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsSatispay>,
    #[doc = "If this is a `scalapay` PaymentMethod, this sub-hash contains details about the ScalaPay payment method options."]
    pub scalapay: Option<stripe_types_07::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsScalapay>,
    #[doc = "If this is a `sepa_debit` PaymentIntent, this sub-hash contains details about the SEPA Debit payment method options."]
    pub sepa_debit: Option<stripe_types_07::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsSepaDebit>,
    #[doc = "If this is a `sequra` PaymentMethod, this sub-hash contains details about the SeQura payment method options."]
    pub sequra: Option<stripe_types_09::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsSequra>,
    #[doc = "If this is a `sofort` PaymentMethod, this sub-hash contains details about the SOFORT payment method options."]
    pub sofort: Option<stripe_types_08::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsSofort>,
    #[doc = "If this is a `sunbit` PaymentMethod, this sub-hash contains details about the Sunbit payment method options."]
    pub sunbit: Option<stripe_types_07::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsSunbit>,
    #[doc = "If this is a `Swish` PaymentMethod, this sub-hash contains details about the Swish payment method options."]
    pub swish: Option<stripe_types_07::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsSwish>,
    #[doc = "If this is a `twint` PaymentMethod, this sub-hash contains details about the TWINT payment method options."]
    pub twint: Option<stripe_types_07::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsTwint>,
    #[doc = "If this is a `upi` PaymentIntent, this sub-hash contains details about the UPI payment method options."]
    pub upi: Option<stripe_types_11::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsUpi>,
    #[doc = "If this is a `us_bank_account` PaymentMethod, this sub-hash contains details about the US bank account payment method..."]
    pub us_bank_account:
        Option<stripe_types_12::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccount>,
    #[doc = "If this is a `wechat_pay` PaymentMethod, this sub-hash contains details about the WeChat Pay payment method options."]
    pub wechat_pay: Option<stripe_types_07::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsWechatPay>,
    #[doc = "If this is a `zip` PaymentMethod, this sub-hash contains details about the Zip payment method options."]
    pub zip: Option<stripe_types_08::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsZip>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptions {
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
        form.field_opt("bizum", self.bizum.as_ref());
        form.field_opt("blik", self.blik.as_ref());
        form.field_opt("boleto", self.boleto.as_ref());
        form.field_opt("card", self.card.as_ref());
        form.field_opt("card_present", self.card_present.as_ref());
        form.field_opt("cashapp", self.cashapp.as_ref());
        form.field_opt("crypto", self.crypto.as_ref());
        form.field_opt("customer_balance", self.customer_balance.as_ref());
        form.field_opt("eps", self.eps.as_ref());
        form.field_opt("fpx", self.fpx.as_ref());
        form.field_opt("giropay", self.giropay.as_ref());
        form.field_opt("grabpay", self.grabpay.as_ref());
        form.field_opt("ideal", self.ideal.as_ref());
        form.field_opt("interac_present", self.interac_present.as_ref());
        form.field_opt("kakao_pay", self.kakao_pay.as_ref());
        form.field_opt("klarna", self.klarna.as_ref());
        form.field_opt("konbini", self.konbini.as_ref());
        form.field_opt("kr_card", self.kr_card.as_ref());
        form.field_opt("link", self.link.as_ref());
        form.field_opt("mb_way", self.mb_way.as_ref());
        form.field_opt("mobilepay", self.mobilepay.as_ref());
        form.field_opt("multibanco", self.multibanco.as_ref());
        form.field_opt("naver_pay", self.naver_pay.as_ref());
        form.field_opt("nz_bank_account", self.nz_bank_account.as_ref());
        form.field_opt("oxxo", self.oxxo.as_ref());
        form.field_opt("p24", self.p24.as_ref());
        form.field_opt("pay_by_bank", self.pay_by_bank.as_ref());
        form.field_opt("payco", self.payco.as_ref());
        form.field_opt("paynow", self.paynow.as_ref());
        form.field_opt("paypal", self.paypal.as_ref());
        form.field_opt("paypay", self.paypay.as_ref());
        form.field_opt("payto", self.payto.as_ref());
        form.field_opt("pix", self.pix.as_ref());
        form.field_opt("promptpay", self.promptpay.as_ref());
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
        form.field_opt("zip", self.zip.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParams {
    #[doc = "The list of payment method types allowed for use with this payment."]
    pub allowed_payment_method_types:
        Option<Vec<stripe_types_06::payment_intents::UpdatePaymentIntentParamsAllowedPaymentMethodTypesItem>>,
    #[doc = "Amount intended to be collected by this PaymentIntent."]
    pub amount: Option<i64>,
    #[doc = "Provides industry-specific information about the amount."]
    pub amount_details: Option<stripe_types_09::payment_intents::UpdatePaymentIntentParamsAmountDetails>,
    #[doc = "The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the..."]
    pub application_fee_amount: Option<json::OwnedValue>,
    #[doc = "Controls when the funds will be captured from the customer's account."]
    pub capture_method: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsCaptureMethod>,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: Option<String>,
    #[doc = "ID of the Customer this PaymentIntent belongs to, if one exists."]
    pub customer: Option<String>,
    #[doc = "ID of the Account representing the customer that this PaymentIntent belongs to, if one exists."]
    pub customer_account: Option<String>,
    #[doc = "An arbitrary string attached to the object."]
    pub description: Option<String>,
    #[doc = "The list of payment method types to exclude from use with this payment."]
    pub excluded_payment_method_types: Option<json::OwnedValue>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Automations to be run during the PaymentIntent lifecycle"]
    pub hooks: Option<stripe_types_10::payment_intents::UpdatePaymentIntentParamsHooks>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Provides industry-specific information about the charge."]
    pub payment_details: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentDetails>,
    #[doc = "ID of the payment method (a PaymentMethod, Card, or compatible Source..."]
    pub payment_method: Option<String>,
    #[doc = "The ID of the [payment method configuration](https://docs.stripe.com/api/payment_method_configurations) to use with t..."]
    pub payment_method_configuration: Option<String>,
    #[doc = "If provided, this hash will be used to create a PaymentMethod."]
    pub payment_method_data: Option<stripe_types_12::payment_intents::UpdatePaymentIntentParamsPaymentMethodData>,
    #[doc = "Payment-method-specific configuration for this PaymentIntent."]
    pub payment_method_options: Option<crate::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptions>,
    #[doc = "Email address that the receipt for the resulting payment will be sent to."]
    pub receipt_email: Option<json::OwnedValue>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<stripe_types_04::payment_intents::UpdatePaymentIntentParamsSetupFutureUsage>,
    #[doc = "Shipping information for this PaymentIntent."]
    pub shipping: Option<stripe_types_09::payment_intents::UpdatePaymentIntentParamsShipping>,
    #[doc = "Text that appears on the customer's statement as the statement descriptor for a non-card charge."]
    pub statement_descriptor: Option<String>,
    #[doc = "Provides information about a card charge."]
    pub statement_descriptor_suffix: Option<String>,
    #[doc = "Use this parameter to automatically create a Transfer when the payment succeeds."]
    pub transfer_data: Option<stripe_types_07::payment_intents::UpdatePaymentIntentParamsTransferData>,
    #[doc = "A string that identifies the resulting payment as part of a group."]
    pub transfer_group: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.allowed_payment_method_types.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("allowed_payment_method_types", index, item);
            }
        }
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("amount_details", self.amount_details.as_ref());
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("currency", self.currency.as_ref());
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("customer_account", self.customer_account.as_ref());
        form.field_opt("description", self.description.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("hooks", self.hooks.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("payment_details", self.payment_details.as_ref());
        form.field_opt("payment_method", self.payment_method.as_ref());
        form.field_opt("payment_method_configuration", self.payment_method_configuration.as_ref());
        form.field_opt("payment_method_data", self.payment_method_data.as_ref());
        form.field_opt("payment_method_options", self.payment_method_options.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("shipping", self.shipping.as_ref());
        form.field_opt("statement_descriptor", self.statement_descriptor.as_ref());
        form.field_opt("statement_descriptor_suffix", self.statement_descriptor_suffix.as_ref());
        form.field_opt("transfer_data", self.transfer_data.as_ref());
        form.field_opt("transfer_group", self.transfer_group.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptions {
    #[doc = "If this is a `acss_debit` PaymentMethod, this sub-hash contains details about the ACSS Debit payment method options."]
    pub acss_debit: Option<stripe_types_10::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsAcssDebit>,
    #[doc = "If this is an `affirm` PaymentMethod, this sub-hash contains details about the Affirm payment method options."]
    pub affirm: Option<stripe_types_09::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsAffirm>,
    #[doc = "If this is a `afterpay_clearpay` PaymentMethod, this sub-hash contains details about the Afterpay Clearpay payment me..."]
    pub afterpay_clearpay:
        Option<stripe_types_08::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsAfterpayClearpay>,
    #[doc = "If this is a `alipay` PaymentMethod, this sub-hash contains details about the Alipay payment method options."]
    pub alipay: Option<stripe_types_08::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsAlipay>,
    #[doc = "If this is a `alma` PaymentMethod, this sub-hash contains details about the Alma payment method options."]
    pub alma: Option<stripe_types_08::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsAlma>,
    #[doc = "If this is a `amazon_pay` PaymentMethod, this sub-hash contains details about the Amazon Pay payment method options."]
    pub amazon_pay: Option<stripe_types_09::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsAmazonPay>,
    #[doc = "If this is a `au_becs_debit` PaymentMethod, this sub-hash contains details about the AU BECS Direct Debit payment met..."]
    pub au_becs_debit:
        Option<stripe_types_07::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsAuBecsDebit>,
    #[doc = "If this is a `bacs_debit` PaymentMethod, this sub-hash contains details about the BACS Debit payment method options."]
    pub bacs_debit: Option<stripe_types_09::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsBacsDebit>,
    #[doc = "If this is a `bancontact` PaymentMethod, this sub-hash contains details about the Bancontact payment method options."]
    pub bancontact: Option<stripe_types_08::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsBancontact>,
    #[doc = "If this is a `billie` PaymentMethod, this sub-hash contains details about the Billie payment method options."]
    pub billie: Option<stripe_types_11::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsBillie>,
    #[doc = "If this is a `bizum` PaymentMethod, this sub-hash contains details about the Bizum payment method options."]
    pub bizum: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsBizum>,
    #[doc = "If this is a `blik` PaymentMethod, this sub-hash contains details about the BLIK payment method options."]
    pub blik: Option<stripe_types_07::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsBlik>,
    #[doc = "If this is a `boleto` PaymentMethod, this sub-hash contains details about the Boleto payment method options."]
    pub boleto: Option<stripe_types_08::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsBoleto>,
    #[doc = "Configuration for any card payments attempted on this PaymentIntent."]
    pub card: Option<crate::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCard>,
    #[doc = "If this is a `card_present` PaymentMethod, this sub-hash contains details about the Card Present payment method options."]
    pub card_present:
        Option<stripe_types_11::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCardPresent>,
    #[doc = "If this is a `cashapp` PaymentMethod, this sub-hash contains details about the Cash App Pay payment method options."]
    pub cashapp: Option<stripe_types_09::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCashapp>,
    #[doc = "If this is a `crypto` PaymentMethod, this sub-hash contains details about the Crypto payment method options."]
    pub crypto: Option<stripe_types_09::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCrypto>,
    #[doc = "If this is a `customer balance` PaymentMethod, this sub-hash contains details about the customer balance payment meth..."]
    pub customer_balance:
        Option<stripe_types_11::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCustomerBalance>,
    #[doc = "If this is a `eps` PaymentMethod, this sub-hash contains details about the EPS payment method options."]
    pub eps: Option<stripe_types_09::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsEps>,
    #[doc = "If this is a `fpx` PaymentMethod, this sub-hash contains details about the FPX payment method options."]
    pub fpx: Option<stripe_types_07::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsFpx>,
    #[doc = "If this is a `giropay` PaymentMethod, this sub-hash contains details about the Giropay payment method options."]
    pub giropay: Option<stripe_types_07::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsGiropay>,
    #[doc = "If this is a `grabpay` PaymentMethod, this sub-hash contains details about the Grabpay payment method options."]
    pub grabpay: Option<stripe_types_08::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsGrabpay>,
    #[doc = "If this is a `ideal` PaymentMethod, this sub-hash contains details about the Ideal payment method options."]
    pub ideal: Option<stripe_types_09::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsIdeal>,
    #[doc = "If this is a `interac_present` PaymentMethod, this sub-hash contains details about the Card Present payment method op..."]
    pub interac_present:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsInteracPresent>,
    #[doc = "If this is a `kakao_pay` PaymentMethod, this sub-hash contains details about the Kakao Pay payment method options."]
    pub kakao_pay: Option<stripe_types_07::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsKakaoPay>,
    #[doc = "If this is a `klarna` PaymentMethod, this sub-hash contains details about the Klarna payment method options."]
    pub klarna: Option<stripe_types_10::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsKlarna>,
    #[doc = "If this is a `konbini` PaymentMethod, this sub-hash contains details about the Konbini payment method options."]
    pub konbini: Option<stripe_types_08::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsKonbini>,
    #[doc = "If this is a `kr_card` PaymentMethod, this sub-hash contains details about the KR Card payment method options."]
    pub kr_card: Option<stripe_types_08::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsKrCard>,
    #[doc = "If this is a `link` PaymentMethod, this sub-hash contains details about the Link payment method options (Link is also..."]
    pub link: Option<stripe_types_09::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsLink>,
    #[doc = "If this is a `mb_way` PaymentMethod, this sub-hash contains details about the MB WAY payment method options."]
    pub mb_way: Option<stripe_types_08::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsMbWay>,
    #[doc = "If this is a `MobilePay` PaymentMethod, this sub-hash contains details about the MobilePay payment method options."]
    pub mobilepay: Option<stripe_types_08::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsMobilepay>,
    #[doc = "If this is a `multibanco` PaymentMethod, this sub-hash contains details about the Multibanco payment method options."]
    pub multibanco: Option<stripe_types_07::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsMultibanco>,
    #[doc = "If this is a `naver_pay` PaymentMethod, this sub-hash contains details about the Naver Pay payment method options."]
    pub naver_pay: Option<stripe_types_09::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsNaverPay>,
    #[doc = "If this is a `nz_bank_account` PaymentMethod, this sub-hash contains details about the NZ BECS Direct Debit payment m..."]
    pub nz_bank_account:
        Option<stripe_types_08::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsNzBankAccount>,
    #[doc = "If this is a `oxxo` PaymentMethod, this sub-hash contains details about the OXXO payment method options."]
    pub oxxo: Option<stripe_types_07::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsOxxo>,
    #[doc = "If this is a `p24` PaymentMethod, this sub-hash contains details about the Przelewy24 payment method options."]
    pub p24: Option<stripe_types_09::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsP24>,
    #[doc = "If this is a `pay_by_bank` PaymentMethod, this sub-hash contains details about the PayByBank payment method options."]
    pub pay_by_bank: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsPayByBank>,
    #[doc = "If this is a `payco` PaymentMethod, this sub-hash contains details about the PAYCO payment method options."]
    pub payco: Option<stripe_types_09::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsPayco>,
    #[doc = "If this is a `paynow` PaymentMethod, this sub-hash contains details about the PayNow payment method options."]
    pub paynow: Option<stripe_types_07::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsPaynow>,
    #[doc = "If this is a `paypal` PaymentMethod, this sub-hash contains details about the PayPal payment method options."]
    pub paypal: Option<stripe_types_07::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsPaypal>,
    #[doc = "If this is a `paypay` PaymentMethod, this sub-hash contains details about the PayPay payment method options."]
    pub paypay: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsPaypay>,
    #[doc = "If this is a `payto` PaymentMethod, this sub-hash contains details about the PayTo payment method options."]
    pub payto: Option<stripe_types_11::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsPayto>,
    #[doc = "If this is a `pix` PaymentMethod, this sub-hash contains details about the Pix payment method options."]
    pub pix: Option<stripe_types_11::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsPix>,
    #[doc = "If this is a `promptpay` PaymentMethod, this sub-hash contains details about the PromptPay payment method options."]
    pub promptpay: Option<stripe_types_09::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsPromptpay>,
    #[doc = "If this is a `revolut_pay` PaymentMethod, this sub-hash contains details about the Revolut Pay payment method options."]
    pub revolut_pay: Option<stripe_types_07::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsRevolutPay>,
    #[doc = "If this is a `samsung_pay` PaymentMethod, this sub-hash contains details about the Samsung Pay payment method options."]
    pub samsung_pay: Option<stripe_types_08::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsSamsungPay>,
    #[doc = "If this is a `satispay` PaymentMethod, this sub-hash contains details about the Satispay payment method options."]
    pub satispay: Option<stripe_types_08::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsSatispay>,
    #[doc = "If this is a `scalapay` PaymentMethod, this sub-hash contains details about the ScalaPay payment method options."]
    pub scalapay: Option<stripe_types_08::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsScalapay>,
    #[doc = "If this is a `sepa_debit` PaymentIntent, this sub-hash contains details about the SEPA Debit payment method options."]
    pub sepa_debit: Option<stripe_types_08::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsSepaDebit>,
    #[doc = "If this is a `sequra` PaymentMethod, this sub-hash contains details about the SeQura payment method options."]
    pub sequra: Option<stripe_types_09::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsSequra>,
    #[doc = "If this is a `sofort` PaymentMethod, this sub-hash contains details about the SOFORT payment method options."]
    pub sofort: Option<stripe_types_07::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsSofort>,
    #[doc = "If this is a `sunbit` PaymentMethod, this sub-hash contains details about the Sunbit payment method options."]
    pub sunbit: Option<stripe_types_07::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsSunbit>,
    #[doc = "If this is a `Swish` PaymentMethod, this sub-hash contains details about the Swish payment method options."]
    pub swish: Option<stripe_types_08::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsSwish>,
    #[doc = "If this is a `twint` PaymentMethod, this sub-hash contains details about the TWINT payment method options."]
    pub twint: Option<stripe_types_07::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsTwint>,
    #[doc = "If this is a `upi` PaymentIntent, this sub-hash contains details about the UPI payment method options."]
    pub upi: Option<stripe_types_10::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsUpi>,
    #[doc = "If this is a `us_bank_account` PaymentMethod, this sub-hash contains details about the US bank account payment method..."]
    pub us_bank_account:
        Option<stripe_types_12::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsUsBankAccount>,
    #[doc = "If this is a `wechat_pay` PaymentMethod, this sub-hash contains details about the WeChat Pay payment method options."]
    pub wechat_pay: Option<stripe_types_08::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsWechatPay>,
    #[doc = "If this is a `zip` PaymentMethod, this sub-hash contains details about the Zip payment method options."]
    pub zip: Option<stripe_types_08::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsZip>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptions {
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
        form.field_opt("bizum", self.bizum.as_ref());
        form.field_opt("blik", self.blik.as_ref());
        form.field_opt("boleto", self.boleto.as_ref());
        form.field_opt("card", self.card.as_ref());
        form.field_opt("card_present", self.card_present.as_ref());
        form.field_opt("cashapp", self.cashapp.as_ref());
        form.field_opt("crypto", self.crypto.as_ref());
        form.field_opt("customer_balance", self.customer_balance.as_ref());
        form.field_opt("eps", self.eps.as_ref());
        form.field_opt("fpx", self.fpx.as_ref());
        form.field_opt("giropay", self.giropay.as_ref());
        form.field_opt("grabpay", self.grabpay.as_ref());
        form.field_opt("ideal", self.ideal.as_ref());
        form.field_opt("interac_present", self.interac_present.as_ref());
        form.field_opt("kakao_pay", self.kakao_pay.as_ref());
        form.field_opt("klarna", self.klarna.as_ref());
        form.field_opt("konbini", self.konbini.as_ref());
        form.field_opt("kr_card", self.kr_card.as_ref());
        form.field_opt("link", self.link.as_ref());
        form.field_opt("mb_way", self.mb_way.as_ref());
        form.field_opt("mobilepay", self.mobilepay.as_ref());
        form.field_opt("multibanco", self.multibanco.as_ref());
        form.field_opt("naver_pay", self.naver_pay.as_ref());
        form.field_opt("nz_bank_account", self.nz_bank_account.as_ref());
        form.field_opt("oxxo", self.oxxo.as_ref());
        form.field_opt("p24", self.p24.as_ref());
        form.field_opt("pay_by_bank", self.pay_by_bank.as_ref());
        form.field_opt("payco", self.payco.as_ref());
        form.field_opt("paynow", self.paynow.as_ref());
        form.field_opt("paypal", self.paypal.as_ref());
        form.field_opt("paypay", self.paypay.as_ref());
        form.field_opt("payto", self.payto.as_ref());
        form.field_opt("pix", self.pix.as_ref());
        form.field_opt("promptpay", self.promptpay.as_ref());
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
        form.field_opt("zip", self.zip.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsCard {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCardCaptureMethod>,
    #[doc = "A single-use `cvc_update` Token that represents a card CVC value."]
    pub cvc_token: Option<String>,
    #[doc = "Installment configuration for payments attempted on this PaymentIntent."]
    pub installments: Option<stripe_types_11::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCardInstallments>,
    #[doc = "Configuration options for setting up an eMandate for cards issued in India."]
    pub mandate_options: Option<stripe_types_09::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCardMandateOptions>,
    #[doc = "When specified, this parameter indicates that a transaction will be marked as MOTO (Mail Order Telephone Order) and t..."]
    pub moto: Option<bool>,
    #[doc = "Selected network to process this PaymentIntent on."]
    pub network: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCardNetwork>,
    #[doc = "Request ability to capture beyond the standard authorization validity window..."]
    pub request_extended_authorization: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCardRequestExtendedAuthorization>,
    #[doc = "Request ability to [increment the authorization](https://docs.stripe.com/payments/incremental-authorization) for this..."]
    pub request_incremental_authorization: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCardRequestIncrementalAuthorization>,
    #[doc = "Request ability to make [multiple captures](https://docs.stripe.com/payments/multicapture) for this PaymentIntent."]
    pub request_multicapture: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCardRequestMulticapture>,
    #[doc = "Request ability to [overcapture](https://docs.stripe.com/payments/overcapture) for this PaymentIntent."]
    pub request_overcapture: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCardRequestOvercapture>,
    #[doc = "We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based..."]
    pub request_three_d_secure: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCardRequestThreeDSecure>,
    #[doc = "When enabled, using a card that is attached to a customer will require the CVC to be provided again (i.e."]
    pub require_cvc_recollection: Option<bool>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCardSetupFutureUsage>,
    #[doc = "Provides information about a card payment that customers see on their statements."]
    pub statement_descriptor_suffix_kana: Option<json::OwnedValue>,
    #[doc = "Provides information about a card payment that customers see on their statements."]
    pub statement_descriptor_suffix_kanji: Option<json::OwnedValue>,
    #[doc = "If 3D Secure authentication was performed with a third-party provider, the authentication details to use for this pay..."]
    pub three_d_secure: Option<stripe_types_12::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecure>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsCard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("cvc_token", self.cvc_token.as_ref());
        form.field_opt("installments", self.installments.as_ref());
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("moto", self.moto.as_ref());
        form.field_opt("network", self.network.as_ref());
        form.field_opt("request_extended_authorization", self.request_extended_authorization.as_ref());
        form.field_opt(
            "request_incremental_authorization",
            self.request_incremental_authorization.as_ref(),
        );
        form.field_opt("request_multicapture", self.request_multicapture.as_ref());
        form.field_opt("request_overcapture", self.request_overcapture.as_ref());
        form.field_opt("request_three_d_secure", self.request_three_d_secure.as_ref());
        form.field_opt("require_cvc_recollection", self.require_cvc_recollection.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("three_d_secure", self.three_d_secure.as_ref());
    }
}
