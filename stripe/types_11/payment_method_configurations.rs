#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsAfterpayClearpay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_09::payment_method_configurations::CreatePaymentMethodConfigurationParamsAfterpayClearpayDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsAfterpayClearpay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsAlipay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_09::payment_method_configurations::CreatePaymentMethodConfigurationParamsAlipayDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsAlipay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsAmazonPay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_09::payment_method_configurations::CreatePaymentMethodConfigurationParamsAmazonPayDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsAmazonPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsApplePay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_07::payment_method_configurations::CreatePaymentMethodConfigurationParamsApplePayDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsApplePay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsApplePayLater {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_08::payment_method_configurations::CreatePaymentMethodConfigurationParamsApplePayLaterDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsApplePayLater {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsBancontact {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_07::payment_method_configurations::CreatePaymentMethodConfigurationParamsBancontactDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsBancontact {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsBlik {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_09::payment_method_configurations::CreatePaymentMethodConfigurationParamsBlikDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsBlik {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsBoleto {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_08::payment_method_configurations::CreatePaymentMethodConfigurationParamsBoletoDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsBoleto {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsCashapp {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_08::payment_method_configurations::CreatePaymentMethodConfigurationParamsCashappDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsCashapp {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsCustomerBalance {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_07::payment_method_configurations::CreatePaymentMethodConfigurationParamsCustomerBalanceDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsCustomerBalance {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsFpx {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_09::payment_method_configurations::CreatePaymentMethodConfigurationParamsFpxDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsFpx {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsFrMealVoucherConecs {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_08::payment_method_configurations::CreatePaymentMethodConfigurationParamsFrMealVoucherConecsDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsFrMealVoucherConecs {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsGooglePay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_08::payment_method_configurations::CreatePaymentMethodConfigurationParamsGooglePayDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsGooglePay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsGrabpay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_07::payment_method_configurations::CreatePaymentMethodConfigurationParamsGrabpayDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsGrabpay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsIdeal {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_09::payment_method_configurations::CreatePaymentMethodConfigurationParamsIdealDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsIdeal {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsKlarna {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_07::payment_method_configurations::CreatePaymentMethodConfigurationParamsKlarnaDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsKlarna {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsLink {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_08::payment_method_configurations::CreatePaymentMethodConfigurationParamsLinkDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsLink {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsNaverPay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_09::payment_method_configurations::CreatePaymentMethodConfigurationParamsNaverPayDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsNaverPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsP24 {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_08::payment_method_configurations::CreatePaymentMethodConfigurationParamsP24DisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsP24 {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsPayByBank {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_07::payment_method_configurations::CreatePaymentMethodConfigurationParamsPayByBankDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsPayByBank {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsPayco {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_08::payment_method_configurations::CreatePaymentMethodConfigurationParamsPaycoDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsPayco {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsPaynow {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_09::payment_method_configurations::CreatePaymentMethodConfigurationParamsPaynowDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsPaynow {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsPaypay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_08::payment_method_configurations::CreatePaymentMethodConfigurationParamsPaypayDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsPaypay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsRevolutPay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_09::payment_method_configurations::CreatePaymentMethodConfigurationParamsRevolutPayDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsRevolutPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsScalapay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_08::payment_method_configurations::CreatePaymentMethodConfigurationParamsScalapayDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsScalapay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsSepaDebit {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_09::payment_method_configurations::CreatePaymentMethodConfigurationParamsSepaDebitDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsSepaDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsSofort {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_07::payment_method_configurations::CreatePaymentMethodConfigurationParamsSofortDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsSofort {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsSwish {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_07::payment_method_configurations::CreatePaymentMethodConfigurationParamsSwishDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsSwish {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsUpi {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_07::payment_method_configurations::CreatePaymentMethodConfigurationParamsUpiDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsUpi {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsUsBankAccount {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_08::payment_method_configurations::CreatePaymentMethodConfigurationParamsUsBankAccountDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsUsBankAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsAcssDebit {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_08::payment_method_configurations::UpdatePaymentMethodConfigurationParamsAcssDebitDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsAcssDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsAffirm {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_09::payment_method_configurations::UpdatePaymentMethodConfigurationParamsAffirmDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsAffirm {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsAlma {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_07::payment_method_configurations::UpdatePaymentMethodConfigurationParamsAlmaDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsAlma {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsAuBecsDebit {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_08::payment_method_configurations::UpdatePaymentMethodConfigurationParamsAuBecsDebitDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsAuBecsDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsBacsDebit {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_07::payment_method_configurations::UpdatePaymentMethodConfigurationParamsBacsDebitDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsBacsDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsBancontact {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_08::payment_method_configurations::UpdatePaymentMethodConfigurationParamsBancontactDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsBancontact {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsBillie {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_08::payment_method_configurations::UpdatePaymentMethodConfigurationParamsBillieDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsBillie {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsBizum {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_09::payment_method_configurations::UpdatePaymentMethodConfigurationParamsBizumDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsBizum {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsCard {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_09::payment_method_configurations::UpdatePaymentMethodConfigurationParamsCardDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsCard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsCashapp {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_09::payment_method_configurations::UpdatePaymentMethodConfigurationParamsCashappDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsCashapp {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsCrypto {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_07::payment_method_configurations::UpdatePaymentMethodConfigurationParamsCryptoDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsCrypto {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsCustomerBalance {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_08::payment_method_configurations::UpdatePaymentMethodConfigurationParamsCustomerBalanceDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsCustomerBalance {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsEps {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_09::payment_method_configurations::UpdatePaymentMethodConfigurationParamsEpsDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsEps {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsGrabpay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_08::payment_method_configurations::UpdatePaymentMethodConfigurationParamsGrabpayDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsGrabpay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsJcb {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_08::payment_method_configurations::UpdatePaymentMethodConfigurationParamsJcbDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsJcb {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsKakaoPay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_07::payment_method_configurations::UpdatePaymentMethodConfigurationParamsKakaoPayDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsKakaoPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsKrCard {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_09::payment_method_configurations::UpdatePaymentMethodConfigurationParamsKrCardDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsKrCard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsMbWay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_08::payment_method_configurations::UpdatePaymentMethodConfigurationParamsMbWayDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsMbWay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsMobilepay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_09::payment_method_configurations::UpdatePaymentMethodConfigurationParamsMobilepayDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsMobilepay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsNzBankAccount {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_09::payment_method_configurations::UpdatePaymentMethodConfigurationParamsNzBankAccountDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsNzBankAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsOxxo {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_08::payment_method_configurations::UpdatePaymentMethodConfigurationParamsOxxoDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsOxxo {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsPaypal {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_08::payment_method_configurations::UpdatePaymentMethodConfigurationParamsPaypalDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsPaypal {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsPayto {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_07::payment_method_configurations::UpdatePaymentMethodConfigurationParamsPaytoDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsPayto {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsPix {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_07::payment_method_configurations::UpdatePaymentMethodConfigurationParamsPixDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsPix {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsPromptpay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_08::payment_method_configurations::UpdatePaymentMethodConfigurationParamsPromptpayDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsPromptpay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsRevolutPay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_07::payment_method_configurations::UpdatePaymentMethodConfigurationParamsRevolutPayDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsRevolutPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsSatispay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_09::payment_method_configurations::UpdatePaymentMethodConfigurationParamsSatispayDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsSatispay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsSequra {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_07::payment_method_configurations::UpdatePaymentMethodConfigurationParamsSequraDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsSequra {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsSunbit {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_09::payment_method_configurations::UpdatePaymentMethodConfigurationParamsSunbitDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsSunbit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsTwint {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_09::payment_method_configurations::UpdatePaymentMethodConfigurationParamsTwintDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsTwint {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsWechatPay {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_07::payment_method_configurations::UpdatePaymentMethodConfigurationParamsWechatPayDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsWechatPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsZip {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<
        stripe_types_09::payment_method_configurations::UpdatePaymentMethodConfigurationParamsZipDisplayPreference,
    >,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsZip {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}
