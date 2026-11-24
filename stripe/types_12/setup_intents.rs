#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone)]
pub struct ConfirmSetupIntentParamsPaymentMethodData {
    #[doc = "If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method."]
    pub acss_debit: Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataAcssDebit>,
    #[doc = "If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method."]
    pub affirm: Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataAffirm>,
    #[doc = "If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method."]
    pub afterpay_clearpay:
        Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataAfterpayClearpay>,
    #[doc = "If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method."]
    pub alipay: Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataAlipay>,
    #[doc = "This field indicates whether this payment method can be shown again to its customer in a checkout flow."]
    pub allow_redisplay:
        Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataAllowRedisplay>,
    #[doc = "If this is an Alma PaymentMethod, this hash contains details about the Alma payment method."]
    pub alma: Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataAlma>,
    #[doc = "If this is an AmazonPay PaymentMethod, this hash contains details about the AmazonPay payment method."]
    pub amazon_pay: Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataAmazonPay>,
    #[doc = "If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account."]
    pub au_becs_debit: Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataAuBecsDebit>,
    #[doc = "If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account."]
    pub bacs_debit: Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataBacsDebit>,
    #[doc = "If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method."]
    pub bancontact: Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataBancontact>,
    #[doc = "If this is a `billie` PaymentMethod, this hash contains details about the Billie payment method."]
    pub billie: Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataBillie>,
    #[doc = "Billing information associated with the PaymentMethod that may be used or required by particular types of payment met..."]
    pub billing_details:
        Option<stripe_types_07::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataBillingDetails>,
    #[doc = "If this is a `bizum` PaymentMethod, this hash contains details about the Bizum payment method."]
    pub bizum: Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataBizum>,
    #[doc = "If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method."]
    pub blik: Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataBlik>,
    #[doc = "If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method."]
    pub boleto: Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataBoleto>,
    #[doc = "If this is a `cashapp` PaymentMethod, this hash contains details about the Cash App Pay payment method."]
    pub cashapp: Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataCashapp>,
    #[doc = "If this is a Crypto PaymentMethod, this hash contains details about the Crypto payment method."]
    pub crypto: Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataCrypto>,
    #[doc = "If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method."]
    pub customer_balance:
        Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataCustomerBalance>,
    #[doc = "If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method."]
    pub eps: Option<stripe_types_07::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataEps>,
    #[doc = "If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method."]
    pub fpx: Option<stripe_types_07::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataFpx>,
    #[doc = "If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method."]
    pub giropay: Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataGiropay>,
    #[doc = "If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method."]
    pub grabpay: Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataGrabpay>,
    #[doc = "If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method."]
    pub ideal: Option<stripe_types_08::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataIdeal>,
    #[doc = "If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method."]
    pub interac_present:
        Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataInteracPresent>,
    #[doc = "If this is a `kakao_pay` PaymentMethod, this hash contains details about the Kakao Pay payment method."]
    pub kakao_pay: Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataKakaoPay>,
    #[doc = "If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method."]
    pub klarna: Option<stripe_types_08::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataKlarna>,
    #[doc = "If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method."]
    pub konbini: Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataKonbini>,
    #[doc = "If this is a `kr_card` PaymentMethod, this hash contains details about the Korean Card payment method."]
    pub kr_card: Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataKrCard>,
    #[doc = "If this is an `Link` PaymentMethod, this hash contains details about the Link payment method (Link is also known as O..."]
    pub link: Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataLink>,
    #[doc = "If this is a MB WAY PaymentMethod, this hash contains details about the MB WAY payment method."]
    pub mb_way: Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataMbWay>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "If this is a `mobilepay` PaymentMethod, this hash contains details about the MobilePay payment method."]
    pub mobilepay: Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataMobilepay>,
    #[doc = "If this is a `multibanco` PaymentMethod, this hash contains details about the Multibanco payment method."]
    pub multibanco: Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataMultibanco>,
    #[doc = "If this is a `naver_pay` PaymentMethod, this hash contains details about the Naver Pay payment method."]
    pub naver_pay: Option<stripe_types_09::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataNaverPay>,
    #[doc = "If this is an nz_bank_account PaymentMethod, this hash contains details about the nz_bank_account payment method."]
    pub nz_bank_account: Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataNzBankAccount>,
    #[doc = "If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method."]
    pub oxxo: Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataOxxo>,
    #[doc = "If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method."]
    pub p24: Option<stripe_types_07::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataP24>,
    #[doc = "If this is a `pay_by_bank` PaymentMethod, this hash contains details about the PayByBank payment method."]
    pub pay_by_bank: Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataPayByBank>,
    #[doc = "If this is a `payco` PaymentMethod, this hash contains details about the PAYCO payment method."]
    pub payco: Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataPayco>,
    #[doc = "If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method."]
    pub paynow: Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataPaynow>,
    #[doc = "If this is a `paypal` PaymentMethod, this hash contains details about the PayPal payment method."]
    pub paypal: Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataPaypal>,
    #[doc = "If this is a `paypay` PaymentMethod, this hash contains details about the PayPay payment method."]
    pub paypay: Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataPaypay>,
    #[doc = "If this is a `payto` PaymentMethod, this hash contains details about the PayTo payment method."]
    pub payto: Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataPayto>,
    #[doc = "If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method."]
    pub pix: Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataPix>,
    #[doc = "If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method."]
    pub promptpay: Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataPromptpay>,
    #[doc = "Options to configure Radar."]
    pub radar_options: Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataRadarOptions>,
    #[doc = "If this is a `revolut_pay` PaymentMethod, this hash contains details about the Revolut Pay payment method."]
    pub revolut_pay: Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataRevolutPay>,
    #[doc = "If this is a `samsung_pay` PaymentMethod, this hash contains details about the SamsungPay payment method."]
    pub samsung_pay: Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataSamsungPay>,
    #[doc = "If this is a `satispay` PaymentMethod, this hash contains details about the Satispay payment method."]
    pub satispay: Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataSatispay>,
    #[doc = "If this is a Scalapay PaymentMethod, this hash contains details about the Scalapay payment method."]
    pub scalapay: Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataScalapay>,
    #[doc = "If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account."]
    pub sepa_debit: Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataSepaDebit>,
    #[doc = "If this is a SeQura PaymentMethod, this hash contains details about the SeQura payment method."]
    pub sequra: Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataSequra>,
    #[doc = "If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method."]
    pub sofort: Option<stripe_types_07::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataSofort>,
    #[doc = "If this is a `sunbit` PaymentMethod, this hash contains details about the Sunbit payment method."]
    pub sunbit: Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataSunbit>,
    #[doc = "If this is a `swish` PaymentMethod, this hash contains details about the Swish payment method."]
    pub swish: Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataSwish>,
    #[doc = "If this is a TWINT PaymentMethod, this hash contains details about the TWINT payment method."]
    pub twint: Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataTwint>,
    #[doc = "The type of the PaymentMethod."]
    pub r#type: stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataType,
    #[doc = "If this is a `upi` PaymentMethod, this hash contains details about the UPI payment method."]
    pub upi: Option<stripe_types_11::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataUpi>,
    #[doc = "If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method."]
    pub us_bank_account: Option<stripe_types_09::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataUsBankAccount>,
    #[doc = "If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method."]
    pub wechat_pay: Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataWechatPay>,
    #[doc = "If this is a `zip` PaymentMethod, this hash contains details about the Zip payment method."]
    pub zip: Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodDataZip>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for ConfirmSetupIntentParamsPaymentMethodData {
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

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmSetupIntentParamsPaymentMethodOptionsCardThreeDSecure {
    #[doc = "The `transStatus` returned from the card Issuer’s ACS in the ARes."]
    pub ares_trans_status: Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus>,
    #[doc = "The cryptogram, also known as the 'authentication value' (AAV, CAVV or AEVV)."]
    pub cryptogram: Option<String>,
    #[doc = "The Electronic Commerce Indicator (ECI) is returned by your 3D Secure provider and indicates what degree of authentic..."]
    pub electronic_commerce_indicator: Option<stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator>,
    #[doc = "Network specific 3DS fields."]
    pub network_options: Option<stripe_types_11::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptions>,
    #[doc = "The challenge indicator (`threeDSRequestorChallengeInd`) which was requested in the AReq sent to the card Issuer's ACS."]
    pub requestor_challenge_indicator: Option<String>,
    #[doc = "For 3D Secure 1, the XID."]
    pub transaction_id: Option<String>,
    #[doc = "The version of 3D Secure that was performed."]
    pub version: Option<stripe_types_05::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsCardThreeDSecureVersion>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for ConfirmSetupIntentParamsPaymentMethodOptionsCardThreeDSecure {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ares_trans_status", self.ares_trans_status.as_ref());
        form.field_opt("cryptogram", self.cryptogram.as_ref());
        form.field_opt("electronic_commerce_indicator", self.electronic_commerce_indicator.as_ref());
        form.field_opt("network_options", self.network_options.as_ref());
        form.field_opt("requestor_challenge_indicator", self.requestor_challenge_indicator.as_ref());
        form.field_opt("transaction_id", self.transaction_id.as_ref());
        form.field_opt("version", self.version.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmSetupIntentParamsPaymentMethodOptionsUsBankAccount {
    #[doc = "Additional fields for Financial Connections Session creation"]
    pub financial_connections: Option<
        stripe_types_11::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnections,
    >,
    #[doc = "Additional fields for Mandate creation"]
    pub mandate_options:
        Option<stripe_types_07::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsUsBankAccountMandateOptions>,
    #[doc = "Additional fields for network related functions"]
    pub networks:
        Option<stripe_types_08::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsUsBankAccountNetworks>,
    #[doc = "Bank account verification method."]
    pub verification_method: Option<
        stripe_types_06::setup_intents::ConfirmSetupIntentParamsPaymentMethodOptionsUsBankAccountVerificationMethod,
    >,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for ConfirmSetupIntentParamsPaymentMethodOptionsUsBankAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("financial_connections", self.financial_connections.as_ref());
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("networks", self.networks.as_ref());
        form.field_opt("verification_method", self.verification_method.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodOptionsCard {
    #[doc = "Configuration options for setting up an eMandate for cards issued in India."]
    pub mandate_options:
        Option<stripe_types_09::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptions>,
    #[doc = "When specified, this parameter signals that a card has been collected as MOTO (Mail Order Telephone Order) and thus o..."]
    pub moto: Option<bool>,
    #[doc = "Selected network to process this SetupIntent on."]
    pub network: Option<stripe_types_04::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsCardNetwork>,
    #[doc = "We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based..."]
    pub request_three_d_secure:
        Option<stripe_types_03::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsCardRequestThreeDSecure>,
    #[doc = "If 3D Secure authentication was performed with a third-party provider, the authentication details to use for this setup."]
    pub three_d_secure:
        Option<stripe_types_11::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecure>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsCard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("moto", self.moto.as_ref());
        form.field_opt("network", self.network.as_ref());
        form.field_opt("request_three_d_secure", self.request_three_d_secure.as_ref());
        form.field_opt("three_d_secure", self.three_d_secure.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodOptionsUsBankAccount {
    #[doc = "Additional fields for Financial Connections Session creation"]
    pub financial_connections: Option<
        stripe_types_11::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnections,
    >,
    #[doc = "Additional fields for Mandate creation"]
    pub mandate_options:
        Option<stripe_types_05::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountMandateOptions>,
    #[doc = "Additional fields for network related functions"]
    pub networks:
        Option<stripe_types_06::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountNetworks>,
    #[doc = "Bank account verification method."]
    pub verification_method: Option<
        stripe_types_05::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountVerificationMethod,
    >,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsUsBankAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("financial_connections", self.financial_connections.as_ref());
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("networks", self.networks.as_ref());
        form.field_opt("verification_method", self.verification_method.as_ref());
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
pub struct SetupIntentPaymentMethodOptions {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<stripe_types_07::setup_intents::SetupIntentPaymentMethodOptionsAcssDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amazon_pay: Option<stripe_types_02::setup_intents::SetupIntentPaymentMethodOptionsAmazonPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<stripe_types_06::setup_intents::SetupIntentPaymentMethodOptionsBacsDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bizum: Option<stripe_types_00::setup_intents::SetupIntentPaymentMethodOptionsBizum>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub blik: Option<stripe_types_09::setup_intents::SetupIntentPaymentMethodOptionsBlik>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types_08::setup_intents::SetupIntentPaymentMethodOptionsCard>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card_present: Option<stripe_types_03::setup_intents::SetupIntentPaymentMethodOptionsCardPresent>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub klarna: Option<stripe_types_01::setup_intents::SetupIntentPaymentMethodOptionsKlarna>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<stripe_types_02::setup_intents::SetupIntentPaymentMethodOptionsLink>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paypal: Option<stripe_types_03::setup_intents::SetupIntentPaymentMethodOptionsPaypal>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payto: Option<stripe_types_08::setup_intents::SetupIntentPaymentMethodOptionsPayto>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pix: Option<stripe_types_09::setup_intents::SetupIntentPaymentMethodOptionsPix>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<stripe_types_05::setup_intents::SetupIntentPaymentMethodOptionsSepaDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub upi: Option<stripe_types_07::setup_intents::SetupIntentPaymentMethodOptionsUpi>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<stripe_types_11::setup_intents::SetupIntentPaymentMethodOptionsUsBankAccount>,
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone)]
pub struct UpdateSetupIntentParamsPaymentMethodData {
    #[doc = "If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method."]
    pub acss_debit: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodDataAcssDebit>,
    #[doc = "If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method."]
    pub affirm: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodDataAffirm>,
    #[doc = "If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method."]
    pub afterpay_clearpay:
        Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodDataAfterpayClearpay>,
    #[doc = "If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method."]
    pub alipay: Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodDataAlipay>,
    #[doc = "This field indicates whether this payment method can be shown again to its customer in a checkout flow."]
    pub allow_redisplay: Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodDataAllowRedisplay>,
    #[doc = "If this is an Alma PaymentMethod, this hash contains details about the Alma payment method."]
    pub alma: Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodDataAlma>,
    #[doc = "If this is an AmazonPay PaymentMethod, this hash contains details about the AmazonPay payment method."]
    pub amazon_pay: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodDataAmazonPay>,
    #[doc = "If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account."]
    pub au_becs_debit: Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodDataAuBecsDebit>,
    #[doc = "If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account."]
    pub bacs_debit: Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodDataBacsDebit>,
    #[doc = "If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method."]
    pub bancontact: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodDataBancontact>,
    #[doc = "If this is a `billie` PaymentMethod, this hash contains details about the Billie payment method."]
    pub billie: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodDataBillie>,
    #[doc = "Billing information associated with the PaymentMethod that may be used or required by particular types of payment met..."]
    pub billing_details: Option<stripe_types_07::setup_intents::UpdateSetupIntentParamsPaymentMethodDataBillingDetails>,
    #[doc = "If this is a `bizum` PaymentMethod, this hash contains details about the Bizum payment method."]
    pub bizum: Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodDataBizum>,
    #[doc = "If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method."]
    pub blik: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodDataBlik>,
    #[doc = "If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method."]
    pub boleto: Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodDataBoleto>,
    #[doc = "If this is a `cashapp` PaymentMethod, this hash contains details about the Cash App Pay payment method."]
    pub cashapp: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodDataCashapp>,
    #[doc = "If this is a Crypto PaymentMethod, this hash contains details about the Crypto payment method."]
    pub crypto: Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodDataCrypto>,
    #[doc = "If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method."]
    pub customer_balance:
        Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodDataCustomerBalance>,
    #[doc = "If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method."]
    pub eps: Option<stripe_types_09::setup_intents::UpdateSetupIntentParamsPaymentMethodDataEps>,
    #[doc = "If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method."]
    pub fpx: Option<stripe_types_08::setup_intents::UpdateSetupIntentParamsPaymentMethodDataFpx>,
    #[doc = "If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method."]
    pub giropay: Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodDataGiropay>,
    #[doc = "If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method."]
    pub grabpay: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodDataGrabpay>,
    #[doc = "If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method."]
    pub ideal: Option<stripe_types_09::setup_intents::UpdateSetupIntentParamsPaymentMethodDataIdeal>,
    #[doc = "If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method."]
    pub interac_present: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodDataInteracPresent>,
    #[doc = "If this is a `kakao_pay` PaymentMethod, this hash contains details about the Kakao Pay payment method."]
    pub kakao_pay: Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodDataKakaoPay>,
    #[doc = "If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method."]
    pub klarna: Option<stripe_types_09::setup_intents::UpdateSetupIntentParamsPaymentMethodDataKlarna>,
    #[doc = "If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method."]
    pub konbini: Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodDataKonbini>,
    #[doc = "If this is a `kr_card` PaymentMethod, this hash contains details about the Korean Card payment method."]
    pub kr_card: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodDataKrCard>,
    #[doc = "If this is an `Link` PaymentMethod, this hash contains details about the Link payment method (Link is also known as O..."]
    pub link: Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodDataLink>,
    #[doc = "If this is a MB WAY PaymentMethod, this hash contains details about the MB WAY payment method."]
    pub mb_way: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodDataMbWay>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "If this is a `mobilepay` PaymentMethod, this hash contains details about the MobilePay payment method."]
    pub mobilepay: Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodDataMobilepay>,
    #[doc = "If this is a `multibanco` PaymentMethod, this hash contains details about the Multibanco payment method."]
    pub multibanco: Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodDataMultibanco>,
    #[doc = "If this is a `naver_pay` PaymentMethod, this hash contains details about the Naver Pay payment method."]
    pub naver_pay: Option<stripe_types_08::setup_intents::UpdateSetupIntentParamsPaymentMethodDataNaverPay>,
    #[doc = "If this is an nz_bank_account PaymentMethod, this hash contains details about the nz_bank_account payment method."]
    pub nz_bank_account: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodDataNzBankAccount>,
    #[doc = "If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method."]
    pub oxxo: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodDataOxxo>,
    #[doc = "If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method."]
    pub p24: Option<stripe_types_08::setup_intents::UpdateSetupIntentParamsPaymentMethodDataP24>,
    #[doc = "If this is a `pay_by_bank` PaymentMethod, this hash contains details about the PayByBank payment method."]
    pub pay_by_bank: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodDataPayByBank>,
    #[doc = "If this is a `payco` PaymentMethod, this hash contains details about the PAYCO payment method."]
    pub payco: Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodDataPayco>,
    #[doc = "If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method."]
    pub paynow: Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodDataPaynow>,
    #[doc = "If this is a `paypal` PaymentMethod, this hash contains details about the PayPal payment method."]
    pub paypal: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodDataPaypal>,
    #[doc = "If this is a `paypay` PaymentMethod, this hash contains details about the PayPay payment method."]
    pub paypay: Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodDataPaypay>,
    #[doc = "If this is a `payto` PaymentMethod, this hash contains details about the PayTo payment method."]
    pub payto: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodDataPayto>,
    #[doc = "If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method."]
    pub pix: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodDataPix>,
    #[doc = "If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method."]
    pub promptpay: Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodDataPromptpay>,
    #[doc = "Options to configure Radar."]
    pub radar_options: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodDataRadarOptions>,
    #[doc = "If this is a `revolut_pay` PaymentMethod, this hash contains details about the Revolut Pay payment method."]
    pub revolut_pay: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodDataRevolutPay>,
    #[doc = "If this is a `samsung_pay` PaymentMethod, this hash contains details about the SamsungPay payment method."]
    pub samsung_pay: Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodDataSamsungPay>,
    #[doc = "If this is a `satispay` PaymentMethod, this hash contains details about the Satispay payment method."]
    pub satispay: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodDataSatispay>,
    #[doc = "If this is a Scalapay PaymentMethod, this hash contains details about the Scalapay payment method."]
    pub scalapay: Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodDataScalapay>,
    #[doc = "If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account."]
    pub sepa_debit: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodDataSepaDebit>,
    #[doc = "If this is a SeQura PaymentMethod, this hash contains details about the SeQura payment method."]
    pub sequra: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodDataSequra>,
    #[doc = "If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method."]
    pub sofort: Option<stripe_types_09::setup_intents::UpdateSetupIntentParamsPaymentMethodDataSofort>,
    #[doc = "If this is a `sunbit` PaymentMethod, this hash contains details about the Sunbit payment method."]
    pub sunbit: Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodDataSunbit>,
    #[doc = "If this is a `swish` PaymentMethod, this hash contains details about the Swish payment method."]
    pub swish: Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodDataSwish>,
    #[doc = "If this is a TWINT PaymentMethod, this hash contains details about the TWINT payment method."]
    pub twint: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodDataTwint>,
    #[doc = "The type of the PaymentMethod."]
    pub r#type: stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodDataType,
    #[doc = "If this is a `upi` PaymentMethod, this hash contains details about the UPI payment method."]
    pub upi: Option<stripe_types_10::setup_intents::UpdateSetupIntentParamsPaymentMethodDataUpi>,
    #[doc = "If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method."]
    pub us_bank_account: Option<stripe_types_09::setup_intents::UpdateSetupIntentParamsPaymentMethodDataUsBankAccount>,
    #[doc = "If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method."]
    pub wechat_pay: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodDataWechatPay>,
    #[doc = "If this is a `zip` PaymentMethod, this hash contains details about the Zip payment method."]
    pub zip: Option<stripe_types_06::setup_intents::UpdateSetupIntentParamsPaymentMethodDataZip>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for UpdateSetupIntentParamsPaymentMethodData {
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

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSetupIntentParamsPaymentMethodOptionsCardThreeDSecure {
    #[doc = "The `transStatus` returned from the card Issuer’s ACS in the ARes."]
    pub ares_trans_status: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus>,
    #[doc = "The cryptogram, also known as the 'authentication value' (AAV, CAVV or AEVV)."]
    pub cryptogram: Option<String>,
    #[doc = "The Electronic Commerce Indicator (ECI) is returned by your 3D Secure provider and indicates what degree of authentic..."]
    pub electronic_commerce_indicator: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator>,
    #[doc = "Network specific 3DS fields."]
    pub network_options: Option<stripe_types_10::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptions>,
    #[doc = "The challenge indicator (`threeDSRequestorChallengeInd`) which was requested in the AReq sent to the card Issuer's ACS."]
    pub requestor_challenge_indicator: Option<String>,
    #[doc = "For 3D Secure 1, the XID."]
    pub transaction_id: Option<String>,
    #[doc = "The version of 3D Secure that was performed."]
    pub version: Option<stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureVersion>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for UpdateSetupIntentParamsPaymentMethodOptionsCardThreeDSecure {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ares_trans_status", self.ares_trans_status.as_ref());
        form.field_opt("cryptogram", self.cryptogram.as_ref());
        form.field_opt("electronic_commerce_indicator", self.electronic_commerce_indicator.as_ref());
        form.field_opt("network_options", self.network_options.as_ref());
        form.field_opt("requestor_challenge_indicator", self.requestor_challenge_indicator.as_ref());
        form.field_opt("transaction_id", self.transaction_id.as_ref());
        form.field_opt("version", self.version.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSetupIntentParamsPaymentMethodOptionsUsBankAccount {
    #[doc = "Additional fields for Financial Connections Session creation"]
    pub financial_connections: Option<
        stripe_types_11::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnections,
    >,
    #[doc = "Additional fields for Mandate creation"]
    pub mandate_options:
        Option<stripe_types_09::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsUsBankAccountMandateOptions>,
    #[doc = "Additional fields for network related functions"]
    pub networks:
        Option<stripe_types_09::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsUsBankAccountNetworks>,
    #[doc = "Bank account verification method."]
    pub verification_method: Option<
        stripe_types_05::setup_intents::UpdateSetupIntentParamsPaymentMethodOptionsUsBankAccountVerificationMethod,
    >,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for UpdateSetupIntentParamsPaymentMethodOptionsUsBankAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("financial_connections", self.financial_connections.as_ref());
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("networks", self.networks.as_ref());
        form.field_opt("verification_method", self.verification_method.as_ref());
    }
}
