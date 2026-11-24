#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsAlmaDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::CreatePaymentMethodConfigurationParamsAlmaDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsAlmaDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsApplePayLaterDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::CreatePaymentMethodConfigurationParamsApplePayLaterDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsApplePayLaterDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsAuBecsDebitDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::CreatePaymentMethodConfigurationParamsAuBecsDebitDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsAuBecsDebitDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsBizumDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::CreatePaymentMethodConfigurationParamsBizumDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsBizumDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsBoletoDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::CreatePaymentMethodConfigurationParamsBoletoDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsBoletoDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsCartesBancairesDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::CreatePaymentMethodConfigurationParamsCartesBancairesDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsCartesBancairesDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsCashappDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::CreatePaymentMethodConfigurationParamsCashappDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsCashappDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsEpsDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::CreatePaymentMethodConfigurationParamsEpsDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsEpsDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsFrMealVoucherConecsDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::CreatePaymentMethodConfigurationParamsFrMealVoucherConecsDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsFrMealVoucherConecsDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsGooglePayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::CreatePaymentMethodConfigurationParamsGooglePayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsGooglePayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsKakaoPayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::CreatePaymentMethodConfigurationParamsKakaoPayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsKakaoPayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsKonbiniDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::CreatePaymentMethodConfigurationParamsKonbiniDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsKonbiniDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsKrCardDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::CreatePaymentMethodConfigurationParamsKrCardDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsKrCardDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsLinkDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::CreatePaymentMethodConfigurationParamsLinkDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsLinkDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsMultibancoDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::CreatePaymentMethodConfigurationParamsMultibancoDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsMultibancoDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsP24DisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::CreatePaymentMethodConfigurationParamsP24DisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsP24DisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsPaycoDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::CreatePaymentMethodConfigurationParamsPaycoDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsPaycoDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsPaypayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::CreatePaymentMethodConfigurationParamsPaypayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsPaypayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsPromptpayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::CreatePaymentMethodConfigurationParamsPromptpayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsPromptpayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsScalapayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::CreatePaymentMethodConfigurationParamsScalapayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsScalapayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsSunbitDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::CreatePaymentMethodConfigurationParamsSunbitDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsSunbitDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsTwintDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::CreatePaymentMethodConfigurationParamsTwintDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsTwintDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsUsBankAccountDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::CreatePaymentMethodConfigurationParamsUsBankAccountDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsUsBankAccountDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsZipDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::CreatePaymentMethodConfigurationParamsZipDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsZipDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsAcssDebitDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::UpdatePaymentMethodConfigurationParamsAcssDebitDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsAcssDebitDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsAfterpayClearpayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::UpdatePaymentMethodConfigurationParamsAfterpayClearpayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsAfterpayClearpayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsAuBecsDebitDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::UpdatePaymentMethodConfigurationParamsAuBecsDebitDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsAuBecsDebitDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsBancontactDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::UpdatePaymentMethodConfigurationParamsBancontactDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsBancontactDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsBillieDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::UpdatePaymentMethodConfigurationParamsBillieDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsBillieDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsBlikDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::UpdatePaymentMethodConfigurationParamsBlikDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsBlikDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsCustomerBalanceDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::UpdatePaymentMethodConfigurationParamsCustomerBalanceDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsCustomerBalanceDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsGooglePayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::UpdatePaymentMethodConfigurationParamsGooglePayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsGooglePayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsGrabpayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::UpdatePaymentMethodConfigurationParamsGrabpayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsGrabpayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsJcbDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::UpdatePaymentMethodConfigurationParamsJcbDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsJcbDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsKlarnaDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::UpdatePaymentMethodConfigurationParamsKlarnaDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsKlarnaDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsMbWayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::UpdatePaymentMethodConfigurationParamsMbWayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsMbWayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsNaverPayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::UpdatePaymentMethodConfigurationParamsNaverPayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsNaverPayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsOxxoDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::UpdatePaymentMethodConfigurationParamsOxxoDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsOxxoDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsPaypalDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::UpdatePaymentMethodConfigurationParamsPaypalDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsPaypalDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsPromptpayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::UpdatePaymentMethodConfigurationParamsPromptpayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsPromptpayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsSamsungPayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::UpdatePaymentMethodConfigurationParamsSamsungPayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsSamsungPayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsSofortDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::UpdatePaymentMethodConfigurationParamsSofortDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsSofortDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsSwishDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::UpdatePaymentMethodConfigurationParamsSwishDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsSwishDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsUpiDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::UpdatePaymentMethodConfigurationParamsUpiDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsUpiDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsUsBankAccountDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::UpdatePaymentMethodConfigurationParamsUsBankAccountDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsUsBankAccountDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}
