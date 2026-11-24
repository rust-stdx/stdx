#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsAffirmDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::CreatePaymentMethodConfigurationParamsAffirmDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsAffirmDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsApplePayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::CreatePaymentMethodConfigurationParamsApplePayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsApplePayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsBacsDebitDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::CreatePaymentMethodConfigurationParamsBacsDebitDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsBacsDebitDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsBancontactDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::CreatePaymentMethodConfigurationParamsBancontactDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsBancontactDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsBillieDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::CreatePaymentMethodConfigurationParamsBillieDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsBillieDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsCardDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::CreatePaymentMethodConfigurationParamsCardDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsCardDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsCustomerBalanceDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::CreatePaymentMethodConfigurationParamsCustomerBalanceDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsCustomerBalanceDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsGrabpayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::CreatePaymentMethodConfigurationParamsGrabpayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsGrabpayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsJcbDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::CreatePaymentMethodConfigurationParamsJcbDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsJcbDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsKlarnaDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::CreatePaymentMethodConfigurationParamsKlarnaDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsKlarnaDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsMbWayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::CreatePaymentMethodConfigurationParamsMbWayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsMbWayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsPayByBankDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::CreatePaymentMethodConfigurationParamsPayByBankDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsPayByBankDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsPaypalDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::CreatePaymentMethodConfigurationParamsPaypalDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsPaypalDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsSamsungPayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::CreatePaymentMethodConfigurationParamsSamsungPayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsSamsungPayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsSatispayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::CreatePaymentMethodConfigurationParamsSatispayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsSatispayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsSofortDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::CreatePaymentMethodConfigurationParamsSofortDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsSofortDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsSwishDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::CreatePaymentMethodConfigurationParamsSwishDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsSwishDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsUpiDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::CreatePaymentMethodConfigurationParamsUpiDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsUpiDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsWechatPayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::CreatePaymentMethodConfigurationParamsWechatPayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsWechatPayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsAlipayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::UpdatePaymentMethodConfigurationParamsAlipayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsAlipayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsAlmaDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::UpdatePaymentMethodConfigurationParamsAlmaDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsAlmaDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsApplePayLaterDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::UpdatePaymentMethodConfigurationParamsApplePayLaterDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsApplePayLaterDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsBacsDebitDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::UpdatePaymentMethodConfigurationParamsBacsDebitDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsBacsDebitDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsCryptoDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::UpdatePaymentMethodConfigurationParamsCryptoDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsCryptoDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsFpxDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::UpdatePaymentMethodConfigurationParamsFpxDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsFpxDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsGiropayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::UpdatePaymentMethodConfigurationParamsGiropayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsGiropayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsIdealDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::UpdatePaymentMethodConfigurationParamsIdealDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsIdealDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsKakaoPayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::UpdatePaymentMethodConfigurationParamsKakaoPayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsKakaoPayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsLinkDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::UpdatePaymentMethodConfigurationParamsLinkDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsLinkDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsPayByBankDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::UpdatePaymentMethodConfigurationParamsPayByBankDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsPayByBankDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsPaynowDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::UpdatePaymentMethodConfigurationParamsPaynowDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsPaynowDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsPaytoDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::UpdatePaymentMethodConfigurationParamsPaytoDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsPaytoDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsPixDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::UpdatePaymentMethodConfigurationParamsPixDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsPixDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsRevolutPayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::UpdatePaymentMethodConfigurationParamsRevolutPayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsRevolutPayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsScalapayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::UpdatePaymentMethodConfigurationParamsScalapayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsScalapayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsSequraDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::UpdatePaymentMethodConfigurationParamsSequraDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsSequraDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsWechatPayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::UpdatePaymentMethodConfigurationParamsWechatPayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsWechatPayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}
