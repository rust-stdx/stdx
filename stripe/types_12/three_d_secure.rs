#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone)]
pub struct CreateThreeDSecureAuthenticationParamsFutureUsage {
    #[doc = "Parameters related to an installment payment."]
    pub installment:
        Option<stripe_types_09::three_d_secure::CreateThreeDSecureAuthenticationParamsFutureUsageInstallment>,
    #[doc = "Parameters related to a recurring payment."]
    pub recurring: Option<stripe_types_10::three_d_secure::CreateThreeDSecureAuthenticationParamsFutureUsageRecurring>,
    #[doc = "The type of future usage declared for this 3DS Authentication"]
    pub r#type: stripe_types_00::three_d_secure::CreateThreeDSecureAuthenticationParamsFutureUsageType,
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for CreateThreeDSecureAuthenticationParamsFutureUsage {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("installment", self.installment.as_ref());
        form.field_opt("recurring", self.recurring.as_ref());
        form.field("type", &self.r#type);
    }
}
