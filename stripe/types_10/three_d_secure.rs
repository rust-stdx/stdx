#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone)]
pub struct CreateThreeDSecureAuthenticationParamsFutureUsageRecurring {
    #[doc = "A non-negative integer representing the future authorizations' amount in the smallest currency unit..."]
    pub amount: i64,
    #[doc = "Information about the expiry of the future usage of this authentication."]
    pub expiry: stripe_types_07::three_d_secure::CreateThreeDSecureAuthenticationParamsFutureUsageRecurringExpiry,
    #[doc = "The unit of time for `interval_count`."]
    pub interval:
        Option<stripe_types_06::three_d_secure::CreateThreeDSecureAuthenticationParamsFutureUsageRecurringInterval>,
    #[doc = "The minimum number of time intervals between authorizations."]
    pub interval_count: Option<i64>,
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for CreateThreeDSecureAuthenticationParamsFutureUsageRecurring {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        form.field("expiry", &self.expiry);
        form.field_opt("interval", self.interval.as_ref());
        form.field_opt("interval_count", self.interval_count.as_ref());
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ThreeDSecureStandaloneResourceFutureUsage {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub installment: Option<stripe_types_07::three_d_secure::ThreeDSecureStandaloneResourceInstallment>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub recurring: Option<stripe_types_07::three_d_secure::ThreeDSecureStandaloneResourceRecurring>,
    #[doc = "The type of future usage declared for this 3DS Authentication."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::three_d_secure::ThreeDSecureStandaloneResourceFutureUsageType,
}
