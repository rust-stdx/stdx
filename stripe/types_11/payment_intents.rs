#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsHooks {
    #[doc = "Arguments passed in automations"]
    pub inputs: Option<stripe_types_07::payment_intents::ConfirmPaymentIntentParamsHooksInputs>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsHooks {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("inputs", self.inputs.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsAcssDebit {
    #[doc = "Additional fields for Mandate creation"]
    pub mandate_options:
        Option<stripe_types_07::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptions>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage,
    >,
    #[doc = "Controls when Stripe will attempt to debit the funds from the customer's account."]
    pub target_date: Option<String>,
    #[doc = "Bank account verification method."]
    pub verification_method: Option<
        stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsAcssDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
        form.field_opt("target_date", self.target_date.as_ref());
        form.field_opt("verification_method", self.verification_method.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptions {
    #[doc = "Cartes Bancaires-specific 3DS fields."]
    pub cartes_bancaires: Option<stripe_types_09::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancaires>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("cartes_bancaires", self.cartes_bancaires.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsUpi {
    #[doc = "Configuration options for setting up an eMandate"]
    pub mandate_options:
        Option<stripe_types_08::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsUpiMandateOptions>,
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsUpiSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsUpi {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnections {
    #[doc = "Provide filters for the linked accounts that the customer can select for the payment method."]
    pub filters: Option<stripe_types_09::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsFilters>,
    #[doc = "The list of permissions to request."]
    pub permissions: Option<Vec<stripe_types_05::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem>>,
    #[doc = "List of data features that you would like to retrieve upon account creation."]
    pub prefetch: Option<Vec<stripe_types_06::payment_intents::ConfirmPaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem>>,
    #[doc = "For webview integrations only."]
    pub return_url: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ConfirmPaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnections {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("filters", self.filters.as_ref());
        if let Some(values) = self.permissions.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("permissions", index, item);
            }
        }
        if let Some(values) = self.prefetch.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("prefetch", index, item);
            }
        }
        form.field_opt("return_url", self.return_url.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone)]
pub struct CreatePaymentIntentParamsPaymentMethodData {
    #[doc = "If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method."]
    pub acss_debit: Option<stripe_types_03::payment_intents::CreatePaymentIntentParamsPaymentMethodDataAcssDebit>,
    #[doc = "If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method."]
    pub affirm: Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodDataAffirm>,
    #[doc = "If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method."]
    pub afterpay_clearpay:
        Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodDataAfterpayClearpay>,
    #[doc = "If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method."]
    pub alipay: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodDataAlipay>,
    #[doc = "This field indicates whether this payment method can be shown again to its customer in a checkout flow."]
    pub allow_redisplay:
        Option<stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodDataAllowRedisplay>,
    #[doc = "If this is an Alma PaymentMethod, this hash contains details about the Alma payment method."]
    pub alma: Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodDataAlma>,
    #[doc = "If this is an AmazonPay PaymentMethod, this hash contains details about the AmazonPay payment method."]
    pub amazon_pay: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodDataAmazonPay>,
    #[doc = "If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account."]
    pub au_becs_debit: Option<stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodDataAuBecsDebit>,
    #[doc = "If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account."]
    pub bacs_debit: Option<stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodDataBacsDebit>,
    #[doc = "If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method."]
    pub bancontact: Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodDataBancontact>,
    #[doc = "If this is a `billie` PaymentMethod, this hash contains details about the Billie payment method."]
    pub billie: Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodDataBillie>,
    #[doc = "Billing information associated with the PaymentMethod that may be used or required by particular types of payment met..."]
    pub billing_details:
        Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodDataBillingDetails>,
    #[doc = "If this is a `bizum` PaymentMethod, this hash contains details about the Bizum payment method."]
    pub bizum: Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodDataBizum>,
    #[doc = "If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method."]
    pub blik: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodDataBlik>,
    #[doc = "If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method."]
    pub boleto: Option<stripe_types_03::payment_intents::CreatePaymentIntentParamsPaymentMethodDataBoleto>,
    #[doc = "If this is a `cashapp` PaymentMethod, this hash contains details about the Cash App Pay payment method."]
    pub cashapp: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodDataCashapp>,
    #[doc = "If this is a Crypto PaymentMethod, this hash contains details about the Crypto payment method."]
    pub crypto: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodDataCrypto>,
    #[doc = "If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method."]
    pub customer_balance:
        Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodDataCustomerBalance>,
    #[doc = "If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method."]
    pub eps: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodDataEps>,
    #[doc = "If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method."]
    pub fpx: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodDataFpx>,
    #[doc = "If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method."]
    pub giropay: Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodDataGiropay>,
    #[doc = "If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method."]
    pub grabpay: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodDataGrabpay>,
    #[doc = "If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method."]
    pub ideal: Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodDataIdeal>,
    #[doc = "If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method."]
    pub interac_present:
        Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodDataInteracPresent>,
    #[doc = "If this is a `kakao_pay` PaymentMethod, this hash contains details about the Kakao Pay payment method."]
    pub kakao_pay: Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodDataKakaoPay>,
    #[doc = "If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method."]
    pub klarna: Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodDataKlarna>,
    #[doc = "If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method."]
    pub konbini: Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodDataKonbini>,
    #[doc = "If this is a `kr_card` PaymentMethod, this hash contains details about the Korean Card payment method."]
    pub kr_card: Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodDataKrCard>,
    #[doc = "If this is an `Link` PaymentMethod, this hash contains details about the Link payment method (Link is also known as O..."]
    pub link: Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodDataLink>,
    #[doc = "If this is a MB WAY PaymentMethod, this hash contains details about the MB WAY payment method."]
    pub mb_way: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodDataMbWay>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "If this is a `mobilepay` PaymentMethod, this hash contains details about the MobilePay payment method."]
    pub mobilepay: Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodDataMobilepay>,
    #[doc = "If this is a `multibanco` PaymentMethod, this hash contains details about the Multibanco payment method."]
    pub multibanco: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodDataMultibanco>,
    #[doc = "If this is a `naver_pay` PaymentMethod, this hash contains details about the Naver Pay payment method."]
    pub naver_pay: Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodDataNaverPay>,
    #[doc = "If this is an nz_bank_account PaymentMethod, this hash contains details about the nz_bank_account payment method."]
    pub nz_bank_account:
        Option<stripe_types_01::payment_intents::CreatePaymentIntentParamsPaymentMethodDataNzBankAccount>,
    #[doc = "If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method."]
    pub oxxo: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodDataOxxo>,
    #[doc = "If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method."]
    pub p24: Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodDataP24>,
    #[doc = "If this is a `pay_by_bank` PaymentMethod, this hash contains details about the PayByBank payment method."]
    pub pay_by_bank: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodDataPayByBank>,
    #[doc = "If this is a `payco` PaymentMethod, this hash contains details about the PAYCO payment method."]
    pub payco: Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodDataPayco>,
    #[doc = "If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method."]
    pub paynow: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodDataPaynow>,
    #[doc = "If this is a `paypal` PaymentMethod, this hash contains details about the PayPal payment method."]
    pub paypal: Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodDataPaypal>,
    #[doc = "If this is a `paypay` PaymentMethod, this hash contains details about the PayPay payment method."]
    pub paypay: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodDataPaypay>,
    #[doc = "If this is a `payto` PaymentMethod, this hash contains details about the PayTo payment method."]
    pub payto: Option<stripe_types_03::payment_intents::CreatePaymentIntentParamsPaymentMethodDataPayto>,
    #[doc = "If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method."]
    pub pix: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodDataPix>,
    #[doc = "If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method."]
    pub promptpay: Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodDataPromptpay>,
    #[doc = "Options to configure Radar."]
    pub radar_options: Option<stripe_types_03::payment_intents::CreatePaymentIntentParamsPaymentMethodDataRadarOptions>,
    #[doc = "If this is a `revolut_pay` PaymentMethod, this hash contains details about the Revolut Pay payment method."]
    pub revolut_pay: Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodDataRevolutPay>,
    #[doc = "If this is a `samsung_pay` PaymentMethod, this hash contains details about the SamsungPay payment method."]
    pub samsung_pay: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodDataSamsungPay>,
    #[doc = "If this is a `satispay` PaymentMethod, this hash contains details about the Satispay payment method."]
    pub satispay: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodDataSatispay>,
    #[doc = "If this is a Scalapay PaymentMethod, this hash contains details about the Scalapay payment method."]
    pub scalapay: Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodDataScalapay>,
    #[doc = "If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account."]
    pub sepa_debit: Option<stripe_types_03::payment_intents::CreatePaymentIntentParamsPaymentMethodDataSepaDebit>,
    #[doc = "If this is a SeQura PaymentMethod, this hash contains details about the SeQura payment method."]
    pub sequra: Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodDataSequra>,
    #[doc = "If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method."]
    pub sofort: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodDataSofort>,
    #[doc = "If this is a `sunbit` PaymentMethod, this hash contains details about the Sunbit payment method."]
    pub sunbit: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodDataSunbit>,
    #[doc = "If this is a `swish` PaymentMethod, this hash contains details about the Swish payment method."]
    pub swish: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodDataSwish>,
    #[doc = "If this is a TWINT PaymentMethod, this hash contains details about the TWINT payment method."]
    pub twint: Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodDataTwint>,
    #[doc = "The type of the PaymentMethod."]
    pub r#type: stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodDataType,
    #[doc = "If this is a `upi` PaymentMethod, this hash contains details about the UPI payment method."]
    pub upi: Option<stripe_types_08::payment_intents::CreatePaymentIntentParamsPaymentMethodDataUpi>,
    #[doc = "If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method."]
    pub us_bank_account:
        Option<stripe_types_07::payment_intents::CreatePaymentIntentParamsPaymentMethodDataUsBankAccount>,
    #[doc = "If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method."]
    pub wechat_pay: Option<stripe_types_06::payment_intents::CreatePaymentIntentParamsPaymentMethodDataWechatPay>,
    #[doc = "If this is a `zip` PaymentMethod, this hash contains details about the Zip payment method."]
    pub zip: Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodDataZip>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodData {
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
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsUpi {
    #[doc = "Configuration options for setting up an eMandate"]
    pub mandate_options:
        Option<stripe_types_08::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsUpiMandateOptions>,
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsUpiSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsUpi {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "customers")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct FundingInstructionsBankTransfer {
    #[doc = "The country of the bank account to fund"]
    #[json(default)]
    pub country: String,
    #[doc = "A list of financial addresses that can be used to fund a particular balance"]
    #[json(default)]
    pub financial_addresses: Vec<stripe_types_07::payment_intents::FundingInstructionsBankTransferFinancialAddress>,
    #[doc = "The bank_transfer type"]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::payment_intents::FundingInstructionsBankTransferType,
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct IncrementAuthorizationParamsHooks {
    #[doc = "Arguments passed in automations"]
    pub inputs: Option<stripe_types_09::payment_intents::IncrementAuthorizationParamsHooksInputs>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for IncrementAuthorizationParamsHooks {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("inputs", self.inputs.as_ref());
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
pub struct PaymentIntentNextActionDisplayBankTransferInstructions {
    #[doc = "The remaining amount that needs to be transferred to complete the payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_remaining: Option<i64>,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "A list of financial addresses that can be used to fund the customer balance"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub financial_addresses:
        Option<Vec<stripe_types_07::payment_intents::FundingInstructionsBankTransferFinancialAddress>>,
    #[doc = "A link to a hosted page that guides your customer through completing the transfer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub hosted_instructions_url: Option<String>,
    #[doc = "A string identifying this payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[doc = "Type of bank transfer"]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::payment_intents::PaymentIntentNextActionDisplayBankTransferInstructionsType,
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsBillie {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsBillieCaptureMethod>,
    #[doc = "Registration details about the buyer's organization."]
    pub company_details:
        Option<stripe_types_08::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetails>,
    #[doc = "An identifier or reference that this payment corresponds to."]
    pub reference: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsBillie {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("company_details", self.company_details.as_ref());
        form.field_opt("reference", self.reference.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsCardInstallments {
    #[doc = "Setting to true enables installments for this PaymentIntent."]
    pub enabled: Option<bool>,
    #[doc = "The selected installment plan to use for this payment attempt."]
    pub plan:
        Option<stripe_types_07::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlan>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsCardInstallments {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("enabled", self.enabled.as_ref());
        form.field_opt("plan", self.plan.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsCardPresent {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCardPresentCaptureMethod>,
    #[doc = "Request ability to capture this payment beyond the standard authorization validity window..."]
    pub request_extended_authorization: Option<bool>,
    #[doc = "Request ability to [increment](https://docs.stripe.com/terminal/features/incremental-authorizations) this PaymentInte..."]
    pub request_incremental_authorization_support: Option<bool>,
    #[doc = "Network routing priority on co-branded EMV cards supporting domestic debit and international card schemes."]
    pub routing:
        Option<stripe_types_08::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCardPresentRouting>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsCardPresent {
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
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsCustomerBalance {
    #[doc = "Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`."]
    pub bank_transfer: Option<
        stripe_types_07::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransfer,
    >,
    #[doc = "The funding method type to be used when there are not enough funds in the customer balance."]
    pub funding_type: Option<
        stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceFundingType,
    >,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage: Option<
        stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceSetupFutureUsage,
    >,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsCustomerBalance {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("bank_transfer", self.bank_transfer.as_ref());
        form.field_opt("funding_type", self.funding_type.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsPayto {
    #[doc = "Additional fields for Mandate creation."]
    pub mandate_options:
        Option<stripe_types_09::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptions>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsPaytoSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsPayto {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsPix {
    #[doc = "Determines if the amount includes the IOF tax."]
    pub amount_includes_iof:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsPixAmountIncludesIof>,
    #[doc = "The number of seconds (between 10 and 1209600) after which Pix payment will expire."]
    pub expires_after_seconds: Option<i64>,
    #[doc = "The timestamp at which the Pix expires (between 10 and 1209600 seconds in the future)."]
    pub expires_at: Option<i64>,
    #[doc = "Additional fields for mandate creation."]
    pub mandate_options:
        Option<stripe_types_08::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsPixMandateOptions>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsPixSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsPix {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount_includes_iof", self.amount_includes_iof.as_ref());
        form.field_opt("expires_after_seconds", self.expires_after_seconds.as_ref());
        form.field_opt("expires_at", self.expires_at.as_ref());
        form.field_opt("mandate_options", self.mandate_options.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnections {
    #[doc = "Provide filters for the linked accounts that the customer can select for the payment method."]
    pub filters: Option<stripe_types_08::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsFilters>,
    #[doc = "The list of permissions to request."]
    pub permissions: Option<Vec<stripe_types_06::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem>>,
    #[doc = "List of data features that you would like to retrieve upon account creation."]
    pub prefetch: Option<Vec<stripe_types_05::payment_intents::UpdatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem>>,
    #[doc = "For webview integrations only."]
    pub return_url: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnections {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("filters", self.filters.as_ref());
        if let Some(values) = self.permissions.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("permissions", index, item);
            }
        }
        if let Some(values) = self.prefetch.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("prefetch", index, item);
            }
        }
        form.field_opt("return_url", self.return_url.as_ref());
    }
}
