#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsAffirm {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_07::payment_method_configurations::CreatePaymentMethodConfigurationParamsAffirmDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsAffirm {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsAlma {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_08::payment_method_configurations::CreatePaymentMethodConfigurationParamsAlmaDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsAlma {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsAuBecsDebit {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_08::payment_method_configurations::CreatePaymentMethodConfigurationParamsAuBecsDebitDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsAuBecsDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsBacsDebit {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_07::payment_method_configurations::CreatePaymentMethodConfigurationParamsBacsDebitDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsBacsDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsBillie {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_07::payment_method_configurations::CreatePaymentMethodConfigurationParamsBillieDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsBillie {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsBizum {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_08::payment_method_configurations::CreatePaymentMethodConfigurationParamsBizumDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsBizum {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsCard {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_07::payment_method_configurations::CreatePaymentMethodConfigurationParamsCardDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsCard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsCartesBancaires {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_08::payment_method_configurations::CreatePaymentMethodConfigurationParamsCartesBancairesDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsCartesBancaires {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsCrypto {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_09::payment_method_configurations::CreatePaymentMethodConfigurationParamsCryptoDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsCrypto {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsEps {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_08::payment_method_configurations::CreatePaymentMethodConfigurationParamsEpsDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsEps {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsGiropay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_09::payment_method_configurations::CreatePaymentMethodConfigurationParamsGiropayDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsGiropay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsJcb {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_07::payment_method_configurations::CreatePaymentMethodConfigurationParamsJcbDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsJcb {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsKakaoPay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_08::payment_method_configurations::CreatePaymentMethodConfigurationParamsKakaoPayDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsKakaoPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsKonbini {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_08::payment_method_configurations::CreatePaymentMethodConfigurationParamsKonbiniDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsKonbini {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsKrCard {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_08::payment_method_configurations::CreatePaymentMethodConfigurationParamsKrCardDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsKrCard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsMbWay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_07::payment_method_configurations::CreatePaymentMethodConfigurationParamsMbWayDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsMbWay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsMobilepay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_09::payment_method_configurations::CreatePaymentMethodConfigurationParamsMobilepayDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsMobilepay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsMultibanco {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_08::payment_method_configurations::CreatePaymentMethodConfigurationParamsMultibancoDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsMultibanco {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsNzBankAccount {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_09::payment_method_configurations::CreatePaymentMethodConfigurationParamsNzBankAccountDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsNzBankAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsOxxo {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_09::payment_method_configurations::CreatePaymentMethodConfigurationParamsOxxoDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsOxxo {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsPaypal {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_07::payment_method_configurations::CreatePaymentMethodConfigurationParamsPaypalDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsPaypal {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsPayto {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_09::payment_method_configurations::CreatePaymentMethodConfigurationParamsPaytoDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsPayto {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsPix {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_09::payment_method_configurations::CreatePaymentMethodConfigurationParamsPixDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsPix {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsPromptpay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_08::payment_method_configurations::CreatePaymentMethodConfigurationParamsPromptpayDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsPromptpay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsSamsungPay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_07::payment_method_configurations::CreatePaymentMethodConfigurationParamsSamsungPayDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsSamsungPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsSatispay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_07::payment_method_configurations::CreatePaymentMethodConfigurationParamsSatispayDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsSatispay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsSequra {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_09::payment_method_configurations::CreatePaymentMethodConfigurationParamsSequraDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsSequra {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsSunbit {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_08::payment_method_configurations::CreatePaymentMethodConfigurationParamsSunbitDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsSunbit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsTwint {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_08::payment_method_configurations::CreatePaymentMethodConfigurationParamsTwintDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsTwint {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsWechatPay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_07::payment_method_configurations::CreatePaymentMethodConfigurationParamsWechatPayDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsWechatPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsZip {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_08::payment_method_configurations::CreatePaymentMethodConfigurationParamsZipDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsZip {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentMethodConfiguration {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[doc = "Whether the configuration can be used for new payments."]
    #[json(default)]
    pub active: bool,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub affirm: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub alipay: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub alma: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amazon_pay: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[doc = "For child configs, the Connect application associated with the configuration."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billie: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bizum: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub blik: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub boleto: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub crypto: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub eps: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fpx: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub giropay: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ideal: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[doc = "The default configuration is used whenever a payment method configuration is not specified."]
    #[json(default)]
    pub is_default: bool,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub jcb: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub kakao_pay: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub klarna: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub konbini: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub kr_card: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mb_way: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mobilepay: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub multibanco: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[doc = "The configuration's name."]
    #[json(default)]
    pub name: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub naver_pay: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub nz_bank_account: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::payment_method_configurations::PaymentMethodConfigurationObject,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub p24: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[doc = "For child configs, the configuration's parent configuration."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pay_by_bank: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payco: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paynow: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paypal: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paypay: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payto: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pix: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub revolut_pay: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub samsung_pay: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub satispay: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub scalapay: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sequra: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sofort: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sunbit: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub swish: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub twint: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub upi: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub zip: Option<stripe_types_09::payment_methods::PaymentMethodConfigResourcePaymentMethodProperties>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::PagedItem for PaymentMethodConfiguration {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsAfterpayClearpay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_08::payment_method_configurations::UpdatePaymentMethodConfigurationParamsAfterpayClearpayDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsAfterpayClearpay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsAlipay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_07::payment_method_configurations::UpdatePaymentMethodConfigurationParamsAlipayDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsAlipay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsAmazonPay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_09::payment_method_configurations::UpdatePaymentMethodConfigurationParamsAmazonPayDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsAmazonPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsApplePay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_09::payment_method_configurations::UpdatePaymentMethodConfigurationParamsApplePayDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsApplePay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsApplePayLater {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_07::payment_method_configurations::UpdatePaymentMethodConfigurationParamsApplePayLaterDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsApplePayLater {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsBlik {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_08::payment_method_configurations::UpdatePaymentMethodConfigurationParamsBlikDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsBlik {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsBoleto {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_09::payment_method_configurations::UpdatePaymentMethodConfigurationParamsBoletoDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsBoleto {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsCartesBancaires {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_09::payment_method_configurations::UpdatePaymentMethodConfigurationParamsCartesBancairesDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsCartesBancaires {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsFpx {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_07::payment_method_configurations::UpdatePaymentMethodConfigurationParamsFpxDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsFpx {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsFrMealVoucherConecs {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_09::payment_method_configurations::UpdatePaymentMethodConfigurationParamsFrMealVoucherConecsDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsFrMealVoucherConecs {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsGiropay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_07::payment_method_configurations::UpdatePaymentMethodConfigurationParamsGiropayDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsGiropay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsGooglePay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_08::payment_method_configurations::UpdatePaymentMethodConfigurationParamsGooglePayDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsGooglePay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsIdeal {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_07::payment_method_configurations::UpdatePaymentMethodConfigurationParamsIdealDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsIdeal {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsKlarna {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_08::payment_method_configurations::UpdatePaymentMethodConfigurationParamsKlarnaDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsKlarna {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsKonbini {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_09::payment_method_configurations::UpdatePaymentMethodConfigurationParamsKonbiniDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsKonbini {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsLink {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_07::payment_method_configurations::UpdatePaymentMethodConfigurationParamsLinkDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsLink {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsMultibanco {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_09::payment_method_configurations::UpdatePaymentMethodConfigurationParamsMultibancoDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsMultibanco {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsNaverPay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_08::payment_method_configurations::UpdatePaymentMethodConfigurationParamsNaverPayDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsNaverPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsP24 {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_09::payment_method_configurations::UpdatePaymentMethodConfigurationParamsP24DisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsP24 {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsPayByBank {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_07::payment_method_configurations::UpdatePaymentMethodConfigurationParamsPayByBankDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsPayByBank {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsPayco {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_09::payment_method_configurations::UpdatePaymentMethodConfigurationParamsPaycoDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsPayco {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsPaynow {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_07::payment_method_configurations::UpdatePaymentMethodConfigurationParamsPaynowDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsPaynow {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsPaypay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_09::payment_method_configurations::UpdatePaymentMethodConfigurationParamsPaypayDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsPaypay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsSamsungPay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_08::payment_method_configurations::UpdatePaymentMethodConfigurationParamsSamsungPayDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsSamsungPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsScalapay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_07::payment_method_configurations::UpdatePaymentMethodConfigurationParamsScalapayDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsScalapay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsSepaDebit {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_09::payment_method_configurations::UpdatePaymentMethodConfigurationParamsSepaDebitDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsSepaDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsSofort {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_08::payment_method_configurations::UpdatePaymentMethodConfigurationParamsSofortDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsSofort {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsSwish {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_08::payment_method_configurations::UpdatePaymentMethodConfigurationParamsSwishDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsSwish {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsUpi {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_08::payment_method_configurations::UpdatePaymentMethodConfigurationParamsUpiDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsUpi {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsUsBankAccount {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_08::payment_method_configurations::UpdatePaymentMethodConfigurationParamsUsBankAccountDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsUsBankAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}
