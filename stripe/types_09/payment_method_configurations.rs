#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsAcssDebit {
    #[doc = "Whether or not the payment method should be displayed."]
    pub display_preference: Option<stripe_types_05::payment_method_configurations::CreatePaymentMethodConfigurationParamsAcssDebitDisplayPreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsAcssDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_preference", self.display_preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsAfterpayClearpayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::CreatePaymentMethodConfigurationParamsAfterpayClearpayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsAfterpayClearpayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsAlipayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::CreatePaymentMethodConfigurationParamsAlipayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsAlipayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsAmazonPayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::CreatePaymentMethodConfigurationParamsAmazonPayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsAmazonPayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsBlikDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::CreatePaymentMethodConfigurationParamsBlikDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsBlikDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsCryptoDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::CreatePaymentMethodConfigurationParamsCryptoDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsCryptoDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsFpxDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::CreatePaymentMethodConfigurationParamsFpxDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsFpxDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsGiropayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::CreatePaymentMethodConfigurationParamsGiropayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsGiropayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsIdealDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::CreatePaymentMethodConfigurationParamsIdealDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsIdealDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsMobilepayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::CreatePaymentMethodConfigurationParamsMobilepayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsMobilepayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsNaverPayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::CreatePaymentMethodConfigurationParamsNaverPayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsNaverPayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsNzBankAccountDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::CreatePaymentMethodConfigurationParamsNzBankAccountDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsNzBankAccountDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsOxxoDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::CreatePaymentMethodConfigurationParamsOxxoDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsOxxoDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsPaynowDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::CreatePaymentMethodConfigurationParamsPaynowDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsPaynowDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsPaytoDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::CreatePaymentMethodConfigurationParamsPaytoDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsPaytoDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsPixDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::CreatePaymentMethodConfigurationParamsPixDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsPixDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsRevolutPayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::CreatePaymentMethodConfigurationParamsRevolutPayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsRevolutPayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsSepaDebitDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::CreatePaymentMethodConfigurationParamsSepaDebitDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsSepaDebitDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParamsSequraDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::CreatePaymentMethodConfigurationParamsSequraDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParamsSequraDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsAffirmDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::UpdatePaymentMethodConfigurationParamsAffirmDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsAffirmDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsAmazonPayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::UpdatePaymentMethodConfigurationParamsAmazonPayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsAmazonPayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsApplePayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::UpdatePaymentMethodConfigurationParamsApplePayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsApplePayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsBizumDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::UpdatePaymentMethodConfigurationParamsBizumDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsBizumDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsBoletoDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::UpdatePaymentMethodConfigurationParamsBoletoDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsBoletoDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsCardDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::UpdatePaymentMethodConfigurationParamsCardDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsCardDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsCartesBancairesDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::UpdatePaymentMethodConfigurationParamsCartesBancairesDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsCartesBancairesDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsCashappDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::UpdatePaymentMethodConfigurationParamsCashappDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsCashappDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsEpsDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::UpdatePaymentMethodConfigurationParamsEpsDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsEpsDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsFrMealVoucherConecsDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::UpdatePaymentMethodConfigurationParamsFrMealVoucherConecsDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsFrMealVoucherConecsDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsKonbiniDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::UpdatePaymentMethodConfigurationParamsKonbiniDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsKonbiniDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsKrCardDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::UpdatePaymentMethodConfigurationParamsKrCardDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsKrCardDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsMobilepayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::UpdatePaymentMethodConfigurationParamsMobilepayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsMobilepayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsMultibancoDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::UpdatePaymentMethodConfigurationParamsMultibancoDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsMultibancoDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsNzBankAccountDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::UpdatePaymentMethodConfigurationParamsNzBankAccountDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsNzBankAccountDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsP24DisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::UpdatePaymentMethodConfigurationParamsP24DisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsP24DisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsPaycoDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::UpdatePaymentMethodConfigurationParamsPaycoDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsPaycoDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsPaypayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::UpdatePaymentMethodConfigurationParamsPaypayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsPaypayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsSatispayDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::UpdatePaymentMethodConfigurationParamsSatispayDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsSatispayDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsSepaDebitDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::UpdatePaymentMethodConfigurationParamsSepaDebitDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsSepaDebitDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsSunbitDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_05::payment_method_configurations::UpdatePaymentMethodConfigurationParamsSunbitDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsSunbitDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsTwintDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::UpdatePaymentMethodConfigurationParamsTwintDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsTwintDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParamsZipDisplayPreference {
    #[doc = "The account's preference for whether or not to display this payment method."]
    pub preference: Option<stripe_types_06::payment_method_configurations::UpdatePaymentMethodConfigurationParamsZipDisplayPreferencePreference>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParamsZipDisplayPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("preference", self.preference.as_ref());
    }
}
