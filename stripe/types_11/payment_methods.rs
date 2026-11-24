#[cfg(feature = "payment_methods")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodParams {
    #[doc = "If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method."]
    pub acss_debit: Option<stripe_types_01::payment_methods::CreatePaymentMethodParamsAcssDebit>,
    #[doc = "If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method."]
    pub affirm: Option<stripe_types_04::payment_methods::CreatePaymentMethodParamsAffirm>,
    #[doc = "If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method."]
    pub afterpay_clearpay: Option<stripe_types_06::payment_methods::CreatePaymentMethodParamsAfterpayClearpay>,
    #[doc = "If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method."]
    pub alipay: Option<stripe_types_06::payment_methods::CreatePaymentMethodParamsAlipay>,
    #[doc = "This field indicates whether this payment method can be shown again to its customer in a checkout flow."]
    pub allow_redisplay: Option<stripe_types_06::payment_methods::CreatePaymentMethodParamsAllowRedisplay>,
    #[doc = "If this is an Alma PaymentMethod, this hash contains details about the Alma payment method."]
    pub alma: Option<stripe_types_05::payment_methods::CreatePaymentMethodParamsAlma>,
    #[doc = "If this is an AmazonPay PaymentMethod, this hash contains details about the AmazonPay payment method."]
    pub amazon_pay: Option<stripe_types_05::payment_methods::CreatePaymentMethodParamsAmazonPay>,
    #[doc = "If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account."]
    pub au_becs_debit: Option<stripe_types_04::payment_methods::CreatePaymentMethodParamsAuBecsDebit>,
    #[doc = "If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account."]
    pub bacs_debit: Option<stripe_types_04::payment_methods::CreatePaymentMethodParamsBacsDebit>,
    #[doc = "If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method."]
    pub bancontact: Option<stripe_types_05::payment_methods::CreatePaymentMethodParamsBancontact>,
    #[doc = "If this is a `billie` PaymentMethod, this hash contains details about the Billie payment method."]
    pub billie: Option<stripe_types_05::payment_methods::CreatePaymentMethodParamsBillie>,
    #[doc = "Billing information associated with the PaymentMethod that may be used or required by particular types of payment met..."]
    pub billing_details: Option<stripe_types_05::payment_methods::CreatePaymentMethodParamsBillingDetails>,
    #[doc = "If this is a `bizum` PaymentMethod, this hash contains details about the Bizum payment method."]
    pub bizum: Option<stripe_types_06::payment_methods::CreatePaymentMethodParamsBizum>,
    #[doc = "If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method."]
    pub blik: Option<stripe_types_06::payment_methods::CreatePaymentMethodParamsBlik>,
    #[doc = "If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method."]
    pub boleto: Option<stripe_types_01::payment_methods::CreatePaymentMethodParamsBoleto>,
    #[doc = "If this is a `card` PaymentMethod, this hash contains the user's card details."]
    pub card: Option<json::OwnedValue>,
    #[doc = "If this is a `cashapp` PaymentMethod, this hash contains details about the Cash App Pay payment method."]
    pub cashapp: Option<stripe_types_05::payment_methods::CreatePaymentMethodParamsCashapp>,
    #[doc = "If this is a Crypto PaymentMethod, this hash contains details about the Crypto payment method."]
    pub crypto: Option<stripe_types_06::payment_methods::CreatePaymentMethodParamsCrypto>,
    #[doc = "If this is a `custom` PaymentMethod, this hash contains details about the Custom payment method."]
    pub custom: Option<stripe_types_03::payment_methods::CreatePaymentMethodParamsCustom>,
    #[doc = "The `Customer` to whom the original PaymentMethod is attached."]
    pub customer: Option<String>,
    #[doc = "If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method."]
    pub customer_balance: Option<stripe_types_06::payment_methods::CreatePaymentMethodParamsCustomerBalance>,
    #[doc = "If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method."]
    pub eps: Option<stripe_types_05::payment_methods::CreatePaymentMethodParamsEps>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method."]
    pub fpx: Option<stripe_types_05::payment_methods::CreatePaymentMethodParamsFpx>,
    #[doc = "If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method."]
    pub giropay: Option<stripe_types_06::payment_methods::CreatePaymentMethodParamsGiropay>,
    #[doc = "If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method."]
    pub grabpay: Option<stripe_types_05::payment_methods::CreatePaymentMethodParamsGrabpay>,
    #[doc = "If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method."]
    pub ideal: Option<stripe_types_06::payment_methods::CreatePaymentMethodParamsIdeal>,
    #[doc = "If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method."]
    pub interac_present: Option<stripe_types_05::payment_methods::CreatePaymentMethodParamsInteracPresent>,
    #[doc = "If this is a `kakao_pay` PaymentMethod, this hash contains details about the Kakao Pay payment method."]
    pub kakao_pay: Option<stripe_types_06::payment_methods::CreatePaymentMethodParamsKakaoPay>,
    #[doc = "If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method."]
    pub klarna: Option<stripe_types_05::payment_methods::CreatePaymentMethodParamsKlarna>,
    #[doc = "If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method."]
    pub konbini: Option<stripe_types_06::payment_methods::CreatePaymentMethodParamsKonbini>,
    #[doc = "If this is a `kr_card` PaymentMethod, this hash contains details about the Korean Card payment method."]
    pub kr_card: Option<stripe_types_05::payment_methods::CreatePaymentMethodParamsKrCard>,
    #[doc = "If this is an `Link` PaymentMethod, this hash contains details about the Link payment method (Link is also known as O..."]
    pub link: Option<stripe_types_05::payment_methods::CreatePaymentMethodParamsLink>,
    #[doc = "If this is a MB WAY PaymentMethod, this hash contains details about the MB WAY payment method."]
    pub mb_way: Option<stripe_types_05::payment_methods::CreatePaymentMethodParamsMbWay>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "If this is a `mobilepay` PaymentMethod, this hash contains details about the MobilePay payment method."]
    pub mobilepay: Option<stripe_types_06::payment_methods::CreatePaymentMethodParamsMobilepay>,
    #[doc = "If this is a `multibanco` PaymentMethod, this hash contains details about the Multibanco payment method."]
    pub multibanco: Option<stripe_types_06::payment_methods::CreatePaymentMethodParamsMultibanco>,
    #[doc = "If this is a `naver_pay` PaymentMethod, this hash contains details about the Naver Pay payment method."]
    pub naver_pay: Option<stripe_types_05::payment_methods::CreatePaymentMethodParamsNaverPay>,
    #[doc = "If this is an nz_bank_account PaymentMethod, this hash contains details about the nz_bank_account payment method."]
    pub nz_bank_account: Option<stripe_types_03::payment_methods::CreatePaymentMethodParamsNzBankAccount>,
    #[doc = "If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method."]
    pub oxxo: Option<stripe_types_06::payment_methods::CreatePaymentMethodParamsOxxo>,
    #[doc = "If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method."]
    pub p24: Option<stripe_types_06::payment_methods::CreatePaymentMethodParamsP24>,
    #[doc = "If this is a `pay_by_bank` PaymentMethod, this hash contains details about the PayByBank payment method."]
    pub pay_by_bank: Option<stripe_types_05::payment_methods::CreatePaymentMethodParamsPayByBank>,
    #[doc = "If this is a `payco` PaymentMethod, this hash contains details about the PAYCO payment method."]
    pub payco: Option<stripe_types_06::payment_methods::CreatePaymentMethodParamsPayco>,
    #[doc = "The PaymentMethod to share."]
    pub payment_method: Option<String>,
    #[doc = "If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method."]
    pub paynow: Option<stripe_types_06::payment_methods::CreatePaymentMethodParamsPaynow>,
    #[doc = "If this is a `paypal` PaymentMethod, this hash contains details about the PayPal payment method."]
    pub paypal: Option<stripe_types_05::payment_methods::CreatePaymentMethodParamsPaypal>,
    #[doc = "If this is a `paypay` PaymentMethod, this hash contains details about the PayPay payment method."]
    pub paypay: Option<stripe_types_06::payment_methods::CreatePaymentMethodParamsPaypay>,
    #[doc = "If this is a `payto` PaymentMethod, this hash contains details about the PayTo payment method."]
    pub payto: Option<stripe_types_04::payment_methods::CreatePaymentMethodParamsPayto>,
    #[doc = "If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method."]
    pub pix: Option<stripe_types_06::payment_methods::CreatePaymentMethodParamsPix>,
    #[doc = "If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method."]
    pub promptpay: Option<stripe_types_06::payment_methods::CreatePaymentMethodParamsPromptpay>,
    #[doc = "Options to configure Radar."]
    pub radar_options: Option<stripe_types_03::payment_methods::CreatePaymentMethodParamsRadarOptions>,
    #[doc = "If this is a `revolut_pay` PaymentMethod, this hash contains details about the Revolut Pay payment method."]
    pub revolut_pay: Option<stripe_types_05::payment_methods::CreatePaymentMethodParamsRevolutPay>,
    #[doc = "If this is a `samsung_pay` PaymentMethod, this hash contains details about the SamsungPay payment method."]
    pub samsung_pay: Option<stripe_types_06::payment_methods::CreatePaymentMethodParamsSamsungPay>,
    #[doc = "If this is a `satispay` PaymentMethod, this hash contains details about the Satispay payment method."]
    pub satispay: Option<stripe_types_05::payment_methods::CreatePaymentMethodParamsSatispay>,
    #[doc = "If this is a Scalapay PaymentMethod, this hash contains details about the Scalapay payment method."]
    pub scalapay: Option<stripe_types_06::payment_methods::CreatePaymentMethodParamsScalapay>,
    #[doc = "If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account."]
    pub sepa_debit: Option<stripe_types_02::payment_methods::CreatePaymentMethodParamsSepaDebit>,
    #[doc = "If this is a SeQura PaymentMethod, this hash contains details about the SeQura payment method."]
    pub sequra: Option<stripe_types_05::payment_methods::CreatePaymentMethodParamsSequra>,
    #[doc = "If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method."]
    pub sofort: Option<stripe_types_05::payment_methods::CreatePaymentMethodParamsSofort>,
    #[doc = "If this is a `sunbit` PaymentMethod, this hash contains details about the Sunbit payment method."]
    pub sunbit: Option<stripe_types_06::payment_methods::CreatePaymentMethodParamsSunbit>,
    #[doc = "If this is a `swish` PaymentMethod, this hash contains details about the Swish payment method."]
    pub swish: Option<stripe_types_05::payment_methods::CreatePaymentMethodParamsSwish>,
    #[doc = "If this is a TWINT PaymentMethod, this hash contains details about the TWINT payment method."]
    pub twint: Option<stripe_types_06::payment_methods::CreatePaymentMethodParamsTwint>,
    #[doc = "The type of the PaymentMethod."]
    pub r#type: Option<stripe_types_06::payment_methods::CreatePaymentMethodParamsType>,
    #[doc = "If this is a `upi` PaymentMethod, this hash contains details about the UPI payment method."]
    pub upi: Option<stripe_types_09::payment_methods::CreatePaymentMethodParamsUpi>,
    #[doc = "If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method."]
    pub us_bank_account: Option<stripe_types_08::payment_methods::CreatePaymentMethodParamsUsBankAccount>,
    #[doc = "If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method."]
    pub wechat_pay: Option<stripe_types_05::payment_methods::CreatePaymentMethodParamsWechatPay>,
    #[doc = "If this is a `zip` PaymentMethod, this hash contains details about the Zip payment method."]
    pub zip: Option<stripe_types_05::payment_methods::CreatePaymentMethodParamsZip>,
}

#[cfg(feature = "payment_methods")]
impl crate::params::FormEncode for CreatePaymentMethodParams {
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
        form.field_opt("custom", self.custom.as_ref());
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("customer_balance", self.customer_balance.as_ref());
        form.field_opt("eps", self.eps.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
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
        form.field_opt("payment_method", self.payment_method.as_ref());
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
        form.field_opt("type", self.r#type.as_ref());
        form.field_opt("upi", self.upi.as_ref());
        form.field_opt("us_bank_account", self.us_bank_account.as_ref());
        form.field_opt("wechat_pay", self.wechat_pay.as_ref());
        form.field_opt("zip", self.zip.as_ref());
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
pub struct PaymentMethodOptionsCustomerBalance {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_transfer: Option<stripe_types_07::payment_methods::PaymentMethodOptionsCustomerBalanceBankTransfer>,
    #[doc = "The funding method type to be used when there are not enough funds in the customer balance."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub funding_type: Option<stripe_types_01::payment_methods::PaymentMethodOptionsCustomerBalanceFundingType>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_types_04::payment_methods::PaymentMethodOptionsAffirmSetupFutureUsage>,
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
pub struct PaymentMethodUsBankAccount {
    #[doc = "Account holder type: individual or company."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<stripe_types_02::payment_methods::PaymentMethodDetailsAchDebitAccountHolderType>,
    #[doc = "Account type: checkings or savings."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_type:
        Option<stripe_types_00::payment_methods::PaymentMethodDetailsPaymentRecordUsBankAccountAccountType>,
    #[doc = "The name of the bank."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[doc = "The ID of the Financial Connections Account used to create the payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub financial_connections_account: Option<String>,
    #[doc = "Uniquely identifies this particular bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[doc = "Last four digits of the bank account number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[doc = "Contains information about US bank account networks that can be used."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub networks: Option<stripe_types_06::common::UsBankAccountNetworks>,
    #[doc = "Routing number of the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
    #[doc = "Contains information about the future reusability of this PaymentMethod."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<stripe_types_09::payment_methods::PaymentMethodUsBankAccountStatusDetails>,
}

#[cfg(feature = "payment_methods")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodParams {
    #[doc = "This field indicates whether this payment method can be shown again to its customer in a checkout flow."]
    pub allow_redisplay: Option<stripe_types_05::payment_methods::UpdatePaymentMethodParamsAllowRedisplay>,
    #[doc = "Billing information associated with the PaymentMethod that may be used or required by particular types of payment met..."]
    pub billing_details: Option<stripe_types_08::payment_methods::UpdatePaymentMethodParamsBillingDetails>,
    #[doc = "If this is a `card` PaymentMethod, this hash contains the user's card details."]
    pub card: Option<stripe_types_07::payment_methods::UpdatePaymentMethodParamsCard>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method."]
    pub us_bank_account: Option<stripe_types_07::payment_methods::UpdatePaymentMethodParamsUsBankAccount>,
}

#[cfg(feature = "payment_methods")]
impl crate::params::FormEncode for UpdatePaymentMethodParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("allow_redisplay", self.allow_redisplay.as_ref());
        form.field_opt("billing_details", self.billing_details.as_ref());
        form.field_opt("card", self.card.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("us_bank_account", self.us_bank_account.as_ref());
    }
}
