#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CapturePaymentIntentParams {
    #[doc = "Provides industry-specific information about the amount."]
    pub amount_details: Option<stripe_types_08::payment_intents::CapturePaymentIntentParamsAmountDetails>,
    #[doc = "The amount to capture from the PaymentIntent, which must be less than or equal to the original amount."]
    pub amount_to_capture: Option<i64>,
    #[doc = "The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the..."]
    pub application_fee_amount: Option<i64>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Defaults to `true`."]
    pub final_capture: Option<bool>,
    #[doc = "Automations to be run during the PaymentIntent lifecycle"]
    pub hooks: Option<stripe_types_10::payment_intents::CapturePaymentIntentParamsHooks>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Provides industry-specific information about the charge."]
    pub payment_details: Option<stripe_types_05::payment_intents::CapturePaymentIntentParamsPaymentDetails>,
    #[doc = "Text that appears on the customer's statement as the statement descriptor for a non-card charge."]
    pub statement_descriptor: Option<String>,
    #[doc = "Provides information about a card charge."]
    pub statement_descriptor_suffix: Option<String>,
    #[doc = "The parameters that you can use to automatically create a transfer after the payment is captured."]
    pub transfer_data: Option<stripe_types_01::payment_intents::CapturePaymentIntentParamsTransferData>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CapturePaymentIntentParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount_details", self.amount_details.as_ref());
        form.field_opt("amount_to_capture", self.amount_to_capture.as_ref());
        form.field_opt("application_fee_amount", self.application_fee_amount.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("final_capture", self.final_capture.as_ref());
        form.field_opt("hooks", self.hooks.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("payment_details", self.payment_details.as_ref());
        form.field_opt("statement_descriptor", self.statement_descriptor.as_ref());
        form.field_opt("statement_descriptor_suffix", self.statement_descriptor_suffix.as_ref());
        form.field_opt("transfer_data", self.transfer_data.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone)]
pub struct ConfirmPaymentIntentParamsPaymentMethodData {
    #[doc = "If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method."]
    pub acss_debit: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataAcssDebit>,
    #[doc = "If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method."]
    pub affirm: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataAffirm>,
    #[doc = "If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method."]
    pub afterpay_clearpay:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataAfterpayClearpay>,
    #[doc = "If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method."]
    pub alipay: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataAlipay>,
    #[doc = "This field indicates whether this payment method can be shown again to its customer in a checkout flow."]
    pub allow_redisplay:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataAllowRedisplay>,
    #[doc = "If this is an Alma PaymentMethod, this hash contains details about the Alma payment method."]
    pub alma: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataAlma>,
    #[doc = "If this is an AmazonPay PaymentMethod, this hash contains details about the AmazonPay payment method."]
    pub amazon_pay: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataAmazonPay>,
    #[doc = "If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account."]
    pub au_becs_debit: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataAuBecsDebit>,
    #[doc = "If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account."]
    pub bacs_debit: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataBacsDebit>,
    #[doc = "If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method."]
    pub bancontact: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataBancontact>,
    #[doc = "If this is a `billie` PaymentMethod, this hash contains details about the Billie payment method."]
    pub billie: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataBillie>,
    #[doc = "Billing information associated with the PaymentMethod that may be used or required by particular types of payment met..."]
    pub billing_details:
        Option<stripe_types_09::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataBillingDetails>,
    #[doc = "If this is a `bizum` PaymentMethod, this hash contains details about the Bizum payment method."]
    pub bizum: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataBizum>,
    #[doc = "If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method."]
    pub blik: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataBlik>,
    #[doc = "If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method."]
    pub boleto: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataBoleto>,
    #[doc = "If this is a `cashapp` PaymentMethod, this hash contains details about the Cash App Pay payment method."]
    pub cashapp: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataCashapp>,
    #[doc = "If this is a Crypto PaymentMethod, this hash contains details about the Crypto payment method."]
    pub crypto: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataCrypto>,
    #[doc = "If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method."]
    pub customer_balance:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataCustomerBalance>,
    #[doc = "If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method."]
    pub eps: Option<stripe_types_07::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataEps>,
    #[doc = "If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method."]
    pub fpx: Option<stripe_types_08::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataFpx>,
    #[doc = "If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method."]
    pub giropay: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataGiropay>,
    #[doc = "If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method."]
    pub grabpay: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataGrabpay>,
    #[doc = "If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method."]
    pub ideal: Option<stripe_types_07::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataIdeal>,
    #[doc = "If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method."]
    pub interac_present:
        Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataInteracPresent>,
    #[doc = "If this is a `kakao_pay` PaymentMethod, this hash contains details about the Kakao Pay payment method."]
    pub kakao_pay: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataKakaoPay>,
    #[doc = "If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method."]
    pub klarna: Option<stripe_types_07::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataKlarna>,
    #[doc = "If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method."]
    pub konbini: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataKonbini>,
    #[doc = "If this is a `kr_card` PaymentMethod, this hash contains details about the Korean Card payment method."]
    pub kr_card: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataKrCard>,
    #[doc = "If this is an `Link` PaymentMethod, this hash contains details about the Link payment method (Link is also known as O..."]
    pub link: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataLink>,
    #[doc = "If this is a MB WAY PaymentMethod, this hash contains details about the MB WAY payment method."]
    pub mb_way: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataMbWay>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "If this is a `mobilepay` PaymentMethod, this hash contains details about the MobilePay payment method."]
    pub mobilepay: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataMobilepay>,
    #[doc = "If this is a `multibanco` PaymentMethod, this hash contains details about the Multibanco payment method."]
    pub multibanco: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataMultibanco>,
    #[doc = "If this is a `naver_pay` PaymentMethod, this hash contains details about the Naver Pay payment method."]
    pub naver_pay: Option<stripe_types_09::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataNaverPay>,
    #[doc = "If this is an nz_bank_account PaymentMethod, this hash contains details about the nz_bank_account payment method."]
    pub nz_bank_account:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataNzBankAccount>,
    #[doc = "If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method."]
    pub oxxo: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataOxxo>,
    #[doc = "If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method."]
    pub p24: Option<stripe_types_09::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataP24>,
    #[doc = "If this is a `pay_by_bank` PaymentMethod, this hash contains details about the PayByBank payment method."]
    pub pay_by_bank: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataPayByBank>,
    #[doc = "If this is a `payco` PaymentMethod, this hash contains details about the PAYCO payment method."]
    pub payco: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataPayco>,
    #[doc = "If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method."]
    pub paynow: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataPaynow>,
    #[doc = "If this is a `paypal` PaymentMethod, this hash contains details about the PayPal payment method."]
    pub paypal: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataPaypal>,
    #[doc = "If this is a `paypay` PaymentMethod, this hash contains details about the PayPay payment method."]
    pub paypay: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataPaypay>,
    #[doc = "If this is a `payto` PaymentMethod, this hash contains details about the PayTo payment method."]
    pub payto: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataPayto>,
    #[doc = "If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method."]
    pub pix: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataPix>,
    #[doc = "If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method."]
    pub promptpay: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataPromptpay>,
    #[doc = "Options to configure Radar."]
    pub radar_options:
        Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataRadarOptions>,
    #[doc = "If this is a `revolut_pay` PaymentMethod, this hash contains details about the Revolut Pay payment method."]
    pub revolut_pay: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataRevolutPay>,
    #[doc = "If this is a `samsung_pay` PaymentMethod, this hash contains details about the SamsungPay payment method."]
    pub samsung_pay: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataSamsungPay>,
    #[doc = "If this is a `satispay` PaymentMethod, this hash contains details about the Satispay payment method."]
    pub satispay: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataSatispay>,
    #[doc = "If this is a Scalapay PaymentMethod, this hash contains details about the Scalapay payment method."]
    pub scalapay: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataScalapay>,
    #[doc = "If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account."]
    pub sepa_debit: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataSepaDebit>,
    #[doc = "If this is a SeQura PaymentMethod, this hash contains details about the SeQura payment method."]
    pub sequra: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataSequra>,
    #[doc = "If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method."]
    pub sofort: Option<stripe_types_07::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataSofort>,
    #[doc = "If this is a `sunbit` PaymentMethod, this hash contains details about the Sunbit payment method."]
    pub sunbit: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataSunbit>,
    #[doc = "If this is a `swish` PaymentMethod, this hash contains details about the Swish payment method."]
    pub swish: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataSwish>,
    #[doc = "If this is a TWINT PaymentMethod, this hash contains details about the TWINT payment method."]
    pub twint: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataTwint>,
    #[doc = "The type of the PaymentMethod."]
    pub r#type: stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataType,
    #[doc = "If this is a `upi` PaymentMethod, this hash contains details about the UPI payment method."]
    pub upi: Option<stripe_types_10::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataUpi>,
    #[doc = "If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method."]
    pub us_bank_account:
        Option<stripe_types_09::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataUsBankAccount>,
    #[doc = "If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method."]
    pub wechat_pay: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataWechatPay>,
    #[doc = "If this is a `zip` PaymentMethod, this hash contains details about the Zip payment method."]
    pub zip: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodDataZip>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("acss_debit", self.acss_debit.as_ref());
        form.field_opt("affirm", self.affirm.as_ref());
        form.field_opt("afterpay_clearpay", self.afterpay_clearpay.as_ref());
        form.field_opt("alipay", self.alipay.as_ref());
        form.field_opt("allow_redisplay", self.allow_redisplay.as_ref());
        form.field_opt("alma", self.alma.as_ref());
        form.field_opt("amazon_pay", self.amazon_pay.as_ref());
        form.field_opt("au_becs_debit", self.au_becs_debit.as_ref());
        form.field_opt("bacs_debit", self.bacs_debit.as_ref());
        form.field_opt("bancontact", self.bancontact.as_ref());
        form.field_opt("billie", self.billie.as_ref());
        form.field_opt("billing_details", self.billing_details.as_ref());
        form.field_opt("bizum", self.bizum.as_ref());
        form.field_opt("blik", self.blik.as_ref());
        form.field_opt("boleto", self.boleto.as_ref());
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
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
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
        form.field_opt("radar_options", self.radar_options.as_ref());
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
        form.field("type", &self.r#type);
        form.field_opt("upi", self.upi.as_ref());
        form.field_opt("us_bank_account", self.us_bank_account.as_ref());
        form.field_opt("wechat_pay", self.wechat_pay.as_ref());
        form.field_opt("zip", self.zip.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsCardThreeDSecure {
    #[doc = "The `transStatus` returned from the card Issuer’s ACS in the ARes."]
    pub ares_trans_status: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus>,
    #[doc = "The cryptogram, also known as the 'authentication value' (AAV, CAVV or AEVV)."]
    pub cryptogram: String,
    #[doc = "The Electronic Commerce Indicator (ECI) is returned by your 3D Secure provider and indicates what degree of authentic..."]
    pub electronic_commerce_indicator: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator>,
    #[doc = "The exemption requested via 3DS and accepted by the issuer at authentication time."]
    pub exemption_indicator: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCardThreeDSecureExemptionIndicator>,
    #[doc = "Network specific 3DS fields."]
    pub network_options: Option<stripe_types_11::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptions>,
    #[doc = "The challenge indicator (`threeDSRequestorChallengeInd`) which was requested in the AReq sent to the card Issuer's ACS."]
    pub requestor_challenge_indicator: Option<String>,
    #[doc = "For 3D Secure 1, the XID."]
    pub transaction_id: String,
    #[doc = "The version of 3D Secure that was performed."]
    pub version: stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCardThreeDSecureVersion,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsCardThreeDSecure {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ares_trans_status", self.ares_trans_status.as_ref());
        form.field("cryptogram", &self.cryptogram);
        form.field_opt("electronic_commerce_indicator", self.electronic_commerce_indicator.as_ref());
        form.field_opt("exemption_indicator", self.exemption_indicator.as_ref());
        form.field_opt("network_options", self.network_options.as_ref());
        form.field_opt("requestor_challenge_indicator", self.requestor_challenge_indicator.as_ref());
        form.field("transaction_id", &self.transaction_id);
        form.field("version", &self.version);
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsUsBankAccount {
    #[doc = "Additional fields for Financial Connections Session creation"]
    pub financial_connections: Option<stripe_types_11::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnections>,
    #[doc = "Additional fields for Mandate creation"]
    pub mandate_options: Option<stripe_types_08::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsUsBankAccountMandateOptions>,
    #[doc = "Additional fields for network related functions"]
    pub networks: Option<stripe_types_09::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsUsBankAccountNetworks>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsUsBankAccountSetupFutureUsage>,
    #[doc = "Controls when Stripe will attempt to debit the funds from the customer's account."]
    pub target_date: Option<String>,
    #[doc = "The purpose of the transaction."]
    pub transaction_purpose: Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsUsBankAccountTransactionPurpose>,
    #[doc = "Bank account verification method."]
    pub verification_method: Option<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsUsBankAccountVerificationMethod>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsUsBankAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("financial_connections", self.financial_connections.as_ref());
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("networks", self.networks.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("target_date", self.target_date.as_ref());
        form.field_opt("transaction_purpose", self.transaction_purpose.as_ref());
        form.field_opt("verification_method", self.verification_method.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsCard {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method: Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardCaptureMethod>,
    #[doc = "A single-use `cvc_update` Token that represents a card CVC value."]
    pub cvc_token: Option<String>,
    #[doc = "Installment configuration for payments attempted on this PaymentIntent."]
    pub installments: Option<stripe_types_08::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardInstallments>,
    #[doc = "Configuration options for setting up an eMandate for cards issued in India."]
    pub mandate_options: Option<stripe_types_09::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptions>,
    #[doc = "When specified, this parameter indicates that a transaction will be marked as MOTO (Mail Order Telephone Order) and t..."]
    pub moto: Option<bool>,
    #[doc = "Selected network to process this PaymentIntent on."]
    pub network: Option<stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork>,
    #[doc = "Request ability to capture beyond the standard authorization validity window..."]
    pub request_extended_authorization: Option<stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardRequestExtendedAuthorization>,
    #[doc = "Request ability to [increment the authorization](https://docs.stripe.com/payments/incremental-authorization) for this..."]
    pub request_incremental_authorization: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardRequestIncrementalAuthorization>,
    #[doc = "Request ability to make [multiple captures](https://docs.stripe.com/payments/multicapture) for this PaymentIntent."]
    pub request_multicapture: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardRequestMulticapture>,
    #[doc = "Request ability to [overcapture](https://docs.stripe.com/payments/overcapture) for this PaymentIntent."]
    pub request_overcapture: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardRequestOvercapture>,
    #[doc = "We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based..."]
    pub request_three_d_secure: Option<stripe_types_03::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardRequestThreeDSecure>,
    #[doc = "When enabled, using a card that is attached to a customer will require the CVC to be provided again (i.e."]
    pub require_cvc_recollection: Option<bool>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardSetupFutureUsage>,
    #[doc = "Provides information about a card payment that customers see on their statements."]
    pub statement_descriptor_suffix_kana: Option<json::OwnedValue>,
    #[doc = "Provides information about a card payment that customers see on their statements."]
    pub statement_descriptor_suffix_kanji: Option<json::OwnedValue>,
    #[doc = "If 3D Secure authentication was performed with a third-party provider, the authentication details to use for this pay..."]
    pub three_d_secure: Option<stripe_types_10::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecure>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsCard {
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
pub struct CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccount {
    #[doc = "Additional fields for Financial Connections Session creation"]
    pub financial_connections: Option<stripe_types_10::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnections>,
    #[doc = "Additional fields for Mandate creation"]
    pub mandate_options: Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountMandateOptions>,
    #[doc = "Additional fields for network related functions"]
    pub networks: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountNetworks>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountSetupFutureUsage>,
    #[doc = "Controls when Stripe will attempt to debit the funds from the customer's account."]
    pub target_date: Option<String>,
    #[doc = "The purpose of the transaction."]
    pub transaction_purpose: Option<stripe_types_01::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountTransactionPurpose>,
    #[doc = "Bank account verification method."]
    pub verification_method: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountVerificationMethod>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("financial_connections", self.financial_connections.as_ref());
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("networks", self.networks.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("target_date", self.target_date.as_ref());
        form.field_opt("transaction_purpose", self.transaction_purpose.as_ref());
        form.field_opt("verification_method", self.verification_method.as_ref());
    }
}

#[cfg(feature = "customers")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct FundingInstructions {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_transfer: Option<stripe_types_11::payment_intents::FundingInstructionsBankTransfer>,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "The `funding_type` of the returned instructions"]
    #[json(default)]
    pub funding_type: stripe_types_02::payment_intents::FundingInstructionsFundingType,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::payment_intents::FundingInstructionsObject,
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct IncrementAuthorizationParams {
    #[doc = "The updated total amount that you intend to collect from the cardholder."]
    pub amount: i64,
    #[doc = "Provides industry-specific information about the amount."]
    pub amount_details: Option<stripe_types_07::payment_intents::IncrementAuthorizationParamsAmountDetails>,
    #[doc = "The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the..."]
    pub application_fee_amount: Option<i64>,
    #[doc = "An arbitrary string attached to the object."]
    pub description: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Automations to be run during the PaymentIntent lifecycle"]
    pub hooks: Option<stripe_types_11::payment_intents::IncrementAuthorizationParamsHooks>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Provides industry-specific information about the charge."]
    pub payment_details: Option<stripe_types_06::payment_intents::IncrementAuthorizationParamsPaymentDetails>,
    #[doc = "Text that appears on the customer's statement as the statement descriptor for a non-card or card charge."]
    pub statement_descriptor: Option<String>,
    #[doc = "The parameters used to automatically create a transfer after the payment is captured."]
    pub transfer_data: Option<stripe_types_05::payment_intents::IncrementAuthorizationParamsTransferData>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for IncrementAuthorizationParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        form.field_opt("amount_details", self.amount_details.as_ref());
        form.field_opt("application_fee_amount", self.application_fee_amount.as_ref());
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
        form.field_opt("statement_descriptor", self.statement_descriptor.as_ref());
        form.field_opt("transfer_data", self.transfer_data.as_ref());
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
pub struct PaymentFlowsAmountDetails {
    #[doc = "The total discount applied on the transaction represented in the smallest currency unit..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub discount_amount: Option<i64>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<stripe_types_06::payment_intents::PaymentFlowsAmountDetailsResourceError>,
    #[doc = "A list of line items, each containing information about a product in the PaymentIntent."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub line_items: Option<crate::List<stripe_types_10::payment_intents::PaymentIntentAmountDetailsLineItem>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping: Option<stripe_types_02::payment_intents::PaymentFlowsAmountDetailsResourceShipping>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax: Option<stripe_types_01::payment_intents::PaymentFlowsAmountDetailsResourceTax>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tip: Option<stripe_types_02::payment_intents::PaymentFlowsAmountDetailsClientResourceTip>,
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
pub struct PaymentIntentNextAction {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub alipay_handle_redirect: Option<stripe_types_00::payment_intents::PaymentIntentNextActionAlipayHandleRedirect>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub blik_authorize: Option<stripe_types_02::payment_intents::PaymentIntentNextActionBlikAuthorize>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub boleto_display_details: Option<stripe_types_01::payment_intents::PaymentIntentNextActionBoleto>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card_await_notification: Option<stripe_types_02::payment_intents::PaymentIntentNextActionCardAwaitNotification>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cashapp_handle_redirect_or_display_qr_code:
        Option<stripe_types_05::payment_intents::PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub display_bank_transfer_instructions:
        Option<stripe_types_11::payment_intents::PaymentIntentNextActionDisplayBankTransferInstructions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub klarna_display_qr_code: Option<stripe_types_01::payment_intents::PaymentIntentNextActionKlarnaDisplayQrCode>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub konbini_display_details: Option<stripe_types_08::payment_intents::PaymentIntentNextActionKonbini>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub multibanco_display_details:
        Option<stripe_types_03::payment_intents::PaymentIntentNextActionDisplayMultibancoDetails>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub oxxo_display_details: Option<stripe_types_01::payment_intents::PaymentIntentNextActionDisplayOxxoDetails>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paynow_display_qr_code: Option<stripe_types_03::payment_intents::PaymentIntentNextActionPaynowDisplayQrCode>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pix_display_qr_code: Option<stripe_types_03::payment_intents::PaymentIntentNextActionPixDisplayQrCode>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub promptpay_display_qr_code:
        Option<stripe_types_04::payment_intents::PaymentIntentNextActionPromptpayDisplayQrCode>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub redirect_to_url: Option<stripe_types_04::payment_intents::PaymentIntentNextActionRedirectToUrl>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub swish_handle_redirect_or_display_qr_code:
        Option<stripe_types_05::payment_intents::PaymentIntentNextActionSwishHandleRedirectOrDisplayQrCode>,
    #[doc = "Type of the next action to perform."]
    #[json(rename = "type", default)]
    pub r#type: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub upi_handle_redirect_or_display_qr_code:
        Option<stripe_types_06::payment_intents::PaymentIntentNextActionUpiHandleRedirectOrDisplayQrCode>,
    #[doc = "When confirming a PaymentIntent with Stripe.js, Stripe.js depends on the contents of this dictionary to invoke authen..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub use_stripe_sdk: Option<std::collections::HashMap<String, json::OwnedValue>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verify_with_microdeposits:
        Option<stripe_types_05::payment_intents::PaymentIntentNextActionVerifyWithMicrodeposits>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub wechat_pay_display_qr_code:
        Option<stripe_types_04::payment_intents::PaymentIntentNextActionWechatPayDisplayQrCode>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub wechat_pay_redirect_to_android_app:
        Option<stripe_types_03::payment_intents::PaymentIntentNextActionWechatPayRedirectToAndroidApp>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub wechat_pay_redirect_to_ios_app:
        Option<stripe_types_02::payment_intents::PaymentIntentNextActionWechatPayRedirectToIosApp>,
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
pub struct PaymentIntentPaymentMethodOptions {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<stripe_types_09::payment_intents::PaymentIntentPaymentMethodOptionsAcssDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub affirm: Option<stripe_types_06::payment_methods::PaymentMethodOptionsAffirm>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<stripe_types_05::payment_methods::PaymentMethodOptionsAfterpayClearpay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub alipay: Option<stripe_types_05::payment_methods::PaymentMethodOptionsAlipay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub alma: Option<stripe_types_05::payment_methods::PaymentMethodOptionsAlma>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amazon_pay: Option<stripe_types_05::payment_methods::PaymentMethodOptionsAmazonPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<stripe_types_06::payment_intents::PaymentIntentPaymentMethodOptionsAuBecsDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<stripe_types_06::payment_intents::PaymentIntentPaymentMethodOptionsBacsDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<stripe_types_06::payment_methods::PaymentMethodOptionsBancontact>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billie: Option<stripe_types_09::payment_methods::PaymentMethodOptionsBillie>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bizum: Option<stripe_types_04::payment_methods::PaymentMethodOptionsBizum>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub blik: Option<stripe_types_08::payment_intents::PaymentIntentPaymentMethodOptionsBlik>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub boleto: Option<stripe_types_06::payment_methods::PaymentMethodOptionsBoleto>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types_10::payment_intents::PaymentIntentPaymentMethodOptionsCard>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card_present: Option<stripe_types_07::payment_methods::PaymentMethodOptionsCardPresent>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<stripe_types_06::payment_methods::PaymentMethodOptionsCashapp>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub crypto: Option<stripe_types_06::payment_methods::PaymentMethodOptionsCrypto>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<stripe_types_11::payment_methods::PaymentMethodOptionsCustomerBalance>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub eps: Option<stripe_types_06::payment_intents::PaymentIntentPaymentMethodOptionsEps>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fpx: Option<stripe_types_06::payment_methods::PaymentMethodOptionsFpx>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub giropay: Option<stripe_types_06::payment_methods::PaymentMethodOptionsGiropay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<stripe_types_05::payment_methods::PaymentMethodOptionsGrabpay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ideal: Option<stripe_types_05::payment_methods::PaymentMethodOptionsIdeal>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<stripe_types_04::payment_methods::PaymentMethodOptionsInteracPresent>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub kakao_pay:
        Option<stripe_types_05::payment_intents::PaymentFlowsPrivatePaymentMethodsKakaoPayPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub klarna: Option<stripe_types_06::payment_methods::PaymentMethodOptionsKlarna>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub konbini: Option<stripe_types_06::payment_methods::PaymentMethodOptionsKonbini>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub kr_card: Option<stripe_types_05::payment_methods::PaymentMethodOptionsKrCard>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<stripe_types_06::payment_intents::PaymentIntentPaymentMethodOptionsLink>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mb_way: Option<stripe_types_06::payment_methods::PaymentMethodOptionsMbWay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mobilepay: Option<stripe_types_05::payment_intents::PaymentIntentPaymentMethodOptionsMobilepay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub multibanco: Option<stripe_types_06::payment_methods::PaymentMethodOptionsMultibanco>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub naver_pay:
        Option<stripe_types_06::payment_intents::PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub nz_bank_account: Option<stripe_types_05::payment_intents::PaymentIntentPaymentMethodOptionsNzBankAccount>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<stripe_types_06::payment_methods::PaymentMethodOptionsOxxo>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub p24: Option<stripe_types_05::payment_methods::PaymentMethodOptionsP24>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pay_by_bank: Option<stripe_types_00::payment_methods::PaymentMethodOptionsPayByBank>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payco: Option<stripe_types_05::payment_intents::PaymentFlowsPrivatePaymentMethodsPaycoPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paynow: Option<stripe_types_05::payment_methods::PaymentMethodOptionsPaynow>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paypal: Option<stripe_types_06::payment_methods::PaymentMethodOptionsPaypal>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paypay: Option<stripe_types_02::payment_methods::PaymentMethodOptionsPaypay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payto: Option<stripe_types_07::payment_intents::PaymentIntentPaymentMethodOptionsPayto>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pix: Option<stripe_types_08::payment_methods::PaymentMethodOptionsPix>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<stripe_types_05::payment_methods::PaymentMethodOptionsPromptpay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub revolut_pay: Option<stripe_types_06::payment_methods::PaymentMethodOptionsRevolutPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub samsung_pay:
        Option<stripe_types_06::payment_intents::PaymentFlowsPrivatePaymentMethodsSamsungPayPaymentMethodOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub satispay: Option<stripe_types_06::payment_methods::PaymentMethodOptionsSatispay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub scalapay: Option<stripe_types_06::payment_methods::PaymentMethodOptionsScalapay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<stripe_types_05::payment_intents::PaymentIntentPaymentMethodOptionsSepaDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sequra: Option<stripe_types_06::payment_methods::PaymentMethodOptionsSequra>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sofort: Option<stripe_types_05::payment_methods::PaymentMethodOptionsSofort>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sunbit: Option<stripe_types_05::payment_methods::PaymentMethodOptionsSunbit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub swish: Option<stripe_types_06::payment_intents::PaymentIntentPaymentMethodOptionsSwish>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub twint: Option<stripe_types_05::payment_methods::PaymentMethodOptionsTwint>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub upi: Option<stripe_types_05::payment_methods::PaymentMethodOptionsUpi>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<stripe_types_10::payment_intents::PaymentIntentPaymentMethodOptionsUsBankAccount>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<stripe_types_06::payment_methods::PaymentMethodOptionsWechatPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub zip: Option<stripe_types_06::payment_methods::PaymentMethodOptionsZip>,
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone)]
pub struct UpdatePaymentIntentParamsPaymentMethodData {
    #[doc = "If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method."]
    pub acss_debit: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataAcssDebit>,
    #[doc = "If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method."]
    pub affirm: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataAffirm>,
    #[doc = "If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method."]
    pub afterpay_clearpay:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataAfterpayClearpay>,
    #[doc = "If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method."]
    pub alipay: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataAlipay>,
    #[doc = "This field indicates whether this payment method can be shown again to its customer in a checkout flow."]
    pub allow_redisplay:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataAllowRedisplay>,
    #[doc = "If this is an Alma PaymentMethod, this hash contains details about the Alma payment method."]
    pub alma: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataAlma>,
    #[doc = "If this is an AmazonPay PaymentMethod, this hash contains details about the AmazonPay payment method."]
    pub amazon_pay: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataAmazonPay>,
    #[doc = "If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account."]
    pub au_becs_debit: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataAuBecsDebit>,
    #[doc = "If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account."]
    pub bacs_debit: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataBacsDebit>,
    #[doc = "If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method."]
    pub bancontact: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataBancontact>,
    #[doc = "If this is a `billie` PaymentMethod, this hash contains details about the Billie payment method."]
    pub billie: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataBillie>,
    #[doc = "Billing information associated with the PaymentMethod that may be used or required by particular types of payment met..."]
    pub billing_details:
        Option<stripe_types_07::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataBillingDetails>,
    #[doc = "If this is a `bizum` PaymentMethod, this hash contains details about the Bizum payment method."]
    pub bizum: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataBizum>,
    #[doc = "If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method."]
    pub blik: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataBlik>,
    #[doc = "If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method."]
    pub boleto: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataBoleto>,
    #[doc = "If this is a `cashapp` PaymentMethod, this hash contains details about the Cash App Pay payment method."]
    pub cashapp: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataCashapp>,
    #[doc = "If this is a Crypto PaymentMethod, this hash contains details about the Crypto payment method."]
    pub crypto: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataCrypto>,
    #[doc = "If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method."]
    pub customer_balance:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataCustomerBalance>,
    #[doc = "If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method."]
    pub eps: Option<stripe_types_09::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataEps>,
    #[doc = "If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method."]
    pub fpx: Option<stripe_types_08::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataFpx>,
    #[doc = "If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method."]
    pub giropay: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataGiropay>,
    #[doc = "If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method."]
    pub grabpay: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataGrabpay>,
    #[doc = "If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method."]
    pub ideal: Option<stripe_types_07::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataIdeal>,
    #[doc = "If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method."]
    pub interac_present:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataInteracPresent>,
    #[doc = "If this is a `kakao_pay` PaymentMethod, this hash contains details about the Kakao Pay payment method."]
    pub kakao_pay: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataKakaoPay>,
    #[doc = "If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method."]
    pub klarna: Option<stripe_types_09::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataKlarna>,
    #[doc = "If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method."]
    pub konbini: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataKonbini>,
    #[doc = "If this is a `kr_card` PaymentMethod, this hash contains details about the Korean Card payment method."]
    pub kr_card: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataKrCard>,
    #[doc = "If this is an `Link` PaymentMethod, this hash contains details about the Link payment method (Link is also known as O..."]
    pub link: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataLink>,
    #[doc = "If this is a MB WAY PaymentMethod, this hash contains details about the MB WAY payment method."]
    pub mb_way: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataMbWay>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "If this is a `mobilepay` PaymentMethod, this hash contains details about the MobilePay payment method."]
    pub mobilepay: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataMobilepay>,
    #[doc = "If this is a `multibanco` PaymentMethod, this hash contains details about the Multibanco payment method."]
    pub multibanco: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataMultibanco>,
    #[doc = "If this is a `naver_pay` PaymentMethod, this hash contains details about the Naver Pay payment method."]
    pub naver_pay: Option<stripe_types_09::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataNaverPay>,
    #[doc = "If this is an nz_bank_account PaymentMethod, this hash contains details about the nz_bank_account payment method."]
    pub nz_bank_account:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataNzBankAccount>,
    #[doc = "If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method."]
    pub oxxo: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataOxxo>,
    #[doc = "If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method."]
    pub p24: Option<stripe_types_07::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataP24>,
    #[doc = "If this is a `pay_by_bank` PaymentMethod, this hash contains details about the PayByBank payment method."]
    pub pay_by_bank: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataPayByBank>,
    #[doc = "If this is a `payco` PaymentMethod, this hash contains details about the PAYCO payment method."]
    pub payco: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataPayco>,
    #[doc = "If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method."]
    pub paynow: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataPaynow>,
    #[doc = "If this is a `paypal` PaymentMethod, this hash contains details about the PayPal payment method."]
    pub paypal: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataPaypal>,
    #[doc = "If this is a `paypay` PaymentMethod, this hash contains details about the PayPay payment method."]
    pub paypay: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataPaypay>,
    #[doc = "If this is a `payto` PaymentMethod, this hash contains details about the PayTo payment method."]
    pub payto: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataPayto>,
    #[doc = "If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method."]
    pub pix: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataPix>,
    #[doc = "If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method."]
    pub promptpay: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataPromptpay>,
    #[doc = "Options to configure Radar."]
    pub radar_options: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataRadarOptions>,
    #[doc = "If this is a `revolut_pay` PaymentMethod, this hash contains details about the Revolut Pay payment method."]
    pub revolut_pay: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataRevolutPay>,
    #[doc = "If this is a `samsung_pay` PaymentMethod, this hash contains details about the SamsungPay payment method."]
    pub samsung_pay: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataSamsungPay>,
    #[doc = "If this is a `satispay` PaymentMethod, this hash contains details about the Satispay payment method."]
    pub satispay: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataSatispay>,
    #[doc = "If this is a Scalapay PaymentMethod, this hash contains details about the Scalapay payment method."]
    pub scalapay: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataScalapay>,
    #[doc = "If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account."]
    pub sepa_debit: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataSepaDebit>,
    #[doc = "If this is a SeQura PaymentMethod, this hash contains details about the SeQura payment method."]
    pub sequra: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataSequra>,
    #[doc = "If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method."]
    pub sofort: Option<stripe_types_09::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataSofort>,
    #[doc = "If this is a `sunbit` PaymentMethod, this hash contains details about the Sunbit payment method."]
    pub sunbit: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataSunbit>,
    #[doc = "If this is a `swish` PaymentMethod, this hash contains details about the Swish payment method."]
    pub swish: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataSwish>,
    #[doc = "If this is a TWINT PaymentMethod, this hash contains details about the TWINT payment method."]
    pub twint: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataTwint>,
    #[doc = "The type of the PaymentMethod."]
    pub r#type: stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataType,
    #[doc = "If this is a `upi` PaymentMethod, this hash contains details about the UPI payment method."]
    pub upi: Option<stripe_types_10::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataUpi>,
    #[doc = "If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method."]
    pub us_bank_account:
        Option<stripe_types_08::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataUsBankAccount>,
    #[doc = "If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method."]
    pub wechat_pay: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataWechatPay>,
    #[doc = "If this is a `zip` PaymentMethod, this hash contains details about the Zip payment method."]
    pub zip: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodDataZip>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("acss_debit", self.acss_debit.as_ref());
        form.field_opt("affirm", self.affirm.as_ref());
        form.field_opt("afterpay_clearpay", self.afterpay_clearpay.as_ref());
        form.field_opt("alipay", self.alipay.as_ref());
        form.field_opt("allow_redisplay", self.allow_redisplay.as_ref());
        form.field_opt("alma", self.alma.as_ref());
        form.field_opt("amazon_pay", self.amazon_pay.as_ref());
        form.field_opt("au_becs_debit", self.au_becs_debit.as_ref());
        form.field_opt("bacs_debit", self.bacs_debit.as_ref());
        form.field_opt("bancontact", self.bancontact.as_ref());
        form.field_opt("billie", self.billie.as_ref());
        form.field_opt("billing_details", self.billing_details.as_ref());
        form.field_opt("bizum", self.bizum.as_ref());
        form.field_opt("blik", self.blik.as_ref());
        form.field_opt("boleto", self.boleto.as_ref());
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
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
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
        form.field_opt("radar_options", self.radar_options.as_ref());
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
        form.field("type", &self.r#type);
        form.field_opt("upi", self.upi.as_ref());
        form.field_opt("us_bank_account", self.us_bank_account.as_ref());
        form.field_opt("wechat_pay", self.wechat_pay.as_ref());
        form.field_opt("zip", self.zip.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecure {
    #[doc = "The `transStatus` returned from the card Issuer’s ACS in the ARes."]
    pub ares_trans_status: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus>,
    #[doc = "The cryptogram, also known as the 'authentication value' (AAV, CAVV or AEVV)."]
    pub cryptogram: String,
    #[doc = "The Electronic Commerce Indicator (ECI) is returned by your 3D Secure provider and indicates what degree of authentic..."]
    pub electronic_commerce_indicator: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator>,
    #[doc = "The exemption requested via 3DS and accepted by the issuer at authentication time."]
    pub exemption_indicator: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureExemptionIndicator>,
    #[doc = "Network specific 3DS fields."]
    pub network_options: Option<stripe_types_10::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptions>,
    #[doc = "The challenge indicator (`threeDSRequestorChallengeInd`) which was requested in the AReq sent to the card Issuer's ACS."]
    pub requestor_challenge_indicator: Option<String>,
    #[doc = "For 3D Secure 1, the XID."]
    pub transaction_id: String,
    #[doc = "The version of 3D Secure that was performed."]
    pub version: stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureVersion,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecure {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ares_trans_status", self.ares_trans_status.as_ref());
        form.field("cryptogram", &self.cryptogram);
        form.field_opt("electronic_commerce_indicator", self.electronic_commerce_indicator.as_ref());
        form.field_opt("exemption_indicator", self.exemption_indicator.as_ref());
        form.field_opt("network_options", self.network_options.as_ref());
        form.field_opt("requestor_challenge_indicator", self.requestor_challenge_indicator.as_ref());
        form.field("transaction_id", &self.transaction_id);
        form.field("version", &self.version);
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsUsBankAccount {
    #[doc = "Additional fields for Financial Connections Session creation"]
    pub financial_connections: Option<stripe_types_11::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnections>,
    #[doc = "Additional fields for Mandate creation"]
    pub mandate_options: Option<stripe_types_09::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsUsBankAccountMandateOptions>,
    #[doc = "Additional fields for network related functions"]
    pub networks: Option<stripe_types_07::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsUsBankAccountNetworks>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsUsBankAccountSetupFutureUsage>,
    #[doc = "Controls when Stripe will attempt to debit the funds from the customer's account."]
    pub target_date: Option<String>,
    #[doc = "The purpose of the transaction."]
    pub transaction_purpose: Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsUsBankAccountTransactionPurpose>,
    #[doc = "Bank account verification method."]
    pub verification_method: Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsUsBankAccountVerificationMethod>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsUsBankAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("financial_connections", self.financial_connections.as_ref());
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("networks", self.networks.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("target_date", self.target_date.as_ref());
        form.field_opt("transaction_purpose", self.transaction_purpose.as_ref());
        form.field_opt("verification_method", self.verification_method.as_ref());
    }
}
