#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone)]
pub struct CreateThreeDSecureAuthenticationParamsFutureUsageRecurringExpiry {
    #[doc = "The date before which the last authorization related to this authentication will occur."]
    pub date: Option<String>,
    #[doc = "The type of expiry for the future use of this authentication."]
    pub r#type: stripe_types_05::three_d_secure::CreateThreeDSecureAuthenticationParamsFutureUsageRecurringExpiryType,
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for CreateThreeDSecureAuthenticationParamsFutureUsageRecurringExpiry {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("date", self.date.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ThreeDSecureStandaloneResourceFlowPreference {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub challenge: Option<stripe_types_05::three_d_secure::ThreeDSecureStandaloneResourceChallengeDetails>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub data_share: Option<stripe_types_06::three_d_secure::ThreeDSecureStandaloneResourceDataShareDetails>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub frictionless: Option<stripe_types_06::three_d_secure::ThreeDSecureStandaloneResourceFrictionlessDetails>,
    #[doc = "Type of flow you requested for this 3DS Authentication."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::three_d_secure::ThreeDSecureStandaloneResourceFlowPreferenceType,
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ThreeDSecureStandaloneResourceInstallment {
    #[doc = "A non-negative integer representing the amount in the [smallest currency unit](/currencies#zero-decimal)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub expiry: Option<stripe_types_06::three_d_secure::ThreeDSecureStandaloneResourceExpiry>,
    #[doc = "The unit of time for `interval_count`."]
    #[json(default)]
    pub interval: stripe_types_00::three_d_secure::ThreeDSecureStandaloneResourceInstallmentInterval,
    #[doc = "The minimum number of time intervals between authorizations."]
    #[json(default)]
    pub interval_count: i64,
    #[doc = "The maximum number of installments."]
    #[json(default)]
    pub number: i64,
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ThreeDSecureStandaloneResourceOutcomeDetails {
    #[doc = "Universally unique transaction identifier assigned by the issuer to identify the transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub acs_transaction_id: Option<String>,
    #[doc = "The Authentication Response Message (ARes) is the issuer's response to the AReq message."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ares: Option<String>,
    #[doc = "TransStatus field on the ARes"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ares_trans_status:
        Option<stripe_types_04::three_d_secure::ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus>,
    #[doc = "A 28-character Base64 string proving that 3DS was completed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cryptogram: Option<String>,
    #[doc = "The 3DS2 Directory Server Transaction ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ds_transaction_id: Option<String>,
    #[doc = "Electronic Commerce Indicator provided by the issuer to indicate the result of this 3DS Authentication."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub eci: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub network_details: Option<stripe_types_06::three_d_secure::ThreeDSecureStandaloneResourceNetworkDetails>,
    #[doc = "The 3DS protocol version used for this 3DS Authentication."]
    #[json(default)]
    pub protocol_version: stripe_types_01::three_d_secure::ThreeDSecureStandaloneResourceOutcomeDetailsProtocolVersion,
    #[doc = "The indicator provided to the issuer by Stripe in the AReq that indicates whether a challenge is requested for this A..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub requestor_challenge_indicator: Option<
        stripe_types_02::three_d_secure::ThreeDSecureStandaloneResourceOutcomeDetailsRequestorChallengeIndicator,
    >,
    #[doc = "The Results Request Message (RReq) communicates the results of the authentication or verification."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub rreq: Option<String>,
    #[doc = "TransStatus field on the RReq"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub rreq_trans_status:
        Option<stripe_types_04::three_d_secure::ThreeDSecureStandaloneResourceOutcomeDetailsAresTransStatus>,
    #[doc = "Universally unique transaction identifier assigned by Stripe to identify the transaction."]
    #[json(default)]
    pub three_ds_server_transaction_id: String,
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ThreeDSecureStandaloneResourceRecurring {
    #[doc = "A non-negative integer representing the amount in the [smallest currency unit](/currencies#zero-decimal)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub expiry: Option<stripe_types_06::three_d_secure::ThreeDSecureStandaloneResourceExpiry>,
    #[doc = "The unit of time for `interval_count`."]
    #[json(default)]
    pub interval: stripe_types_00::three_d_secure::ThreeDSecureStandaloneResourceInstallmentInterval,
    #[doc = "The minimum number of time intervals between authorizations."]
    #[json(default)]
    pub interval_count: i64,
}
