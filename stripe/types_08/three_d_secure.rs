#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone)]
pub struct CreateThreeDSecureAuthenticationParamsFlowPreference {
    #[doc = "Contains additional details about your challenge flow preference for this 3DS Authentication."]
    pub challenge:
        Option<stripe_types_06::three_d_secure::CreateThreeDSecureAuthenticationParamsFlowPreferenceChallenge>,
    #[doc = "Contains additional details about your data share only flow preference for this 3DS Authentication."]
    pub data_share:
        Option<stripe_types_06::three_d_secure::CreateThreeDSecureAuthenticationParamsFlowPreferenceDataShare>,
    #[doc = "Contains additional details about your frictionless flow preference for this 3DS Authentication."]
    pub frictionless:
        Option<stripe_types_06::three_d_secure::CreateThreeDSecureAuthenticationParamsFlowPreferenceFrictionless>,
    #[doc = "Type of flow you requested for this 3DS Authentication."]
    pub r#type: stripe_types_02::three_d_secure::CreateThreeDSecureAuthenticationParamsFlowPreferenceType,
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for CreateThreeDSecureAuthenticationParamsFlowPreference {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("challenge", self.challenge.as_ref());
        form.field_opt("data_share", self.data_share.as_ref());
        form.field_opt("frictionless", self.frictionless.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ThreeDSecureStandaloneResourceChannel {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub browser: Option<stripe_types_03::three_d_secure::ThreeDSecureStandaloneResourceBrowserDetails>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub three_r_i: Option<stripe_types_05::three_d_secure::ThreeDSecureStandaloneResourceThreeRiDetails>,
    #[doc = "Type of channel you would prefer to use for this 3DS Authentication."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::three_d_secure::ThreeDSecureStandaloneResourceChannelType,
}
