#[cfg(feature = "three_d_secure")]
pub type CreateThreeDSecureAuthenticationParamsFutureUsageRecurringExpiryType =
    stripe_types_00::three_d_secure::CreateThreeDSecureAuthenticationParamsFutureUsageInstallmentExpiryType;

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, Default)]
pub struct CreateThreeDSecureAuthenticationParamsPaymentMethodDataBillingDetails {
    #[doc = "Billing address."]
    pub address: Option<
        stripe_types_03::three_d_secure::CreateThreeDSecureAuthenticationParamsPaymentMethodDataBillingDetailsAddress,
    >,
    #[doc = "Email address."]
    pub email: Option<String>,
    #[doc = "Full name."]
    pub name: Option<String>,
    #[doc = "Billing phone number (including extension)."]
    pub phone: Option<String>,
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for CreateThreeDSecureAuthenticationParamsPaymentMethodDataBillingDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("address", self.address.as_ref());
        form.field_opt("email", self.email.as_ref());
        form.field_opt("name", self.name.as_ref());
        form.field_opt("phone", self.phone.as_ref());
    }
}

#[cfg(feature = "three_d_secure")]
pub type CreateThreeDSecureAuthenticationParamsShippingAddress =
    stripe_types_03::three_d_secure::CreateThreeDSecureAuthenticationParamsPaymentMethodDataBillingDetailsAddress;

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, Default)]
pub struct ListThreeDSecureAuthenticationsParams {
    pub created: Option<stripe_types_02::three_d_secure::ListThreeDSecureAuthenticationsParamsCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<stripe_types_04::three_d_secure::ListThreeDSecureAuthenticationsParamsStatus>,
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for ListThreeDSecureAuthenticationsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("created", self.created.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("status", self.status.as_ref());
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ThreeDSecureStandaloneResourceChallengeDetails {
    #[doc = "Type of challenge flow you requested for this 3DS Authentication."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::three_d_secure::ThreeDSecureStandaloneResourceChallengeDetailsType,
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ThreeDSecureStandaloneResourceThreeRiDetails {
    #[doc = "ID of the previous initial authenticated 3DS Authentication object."]
    #[json(default)]
    pub previous_authentication: String,
    #[doc = "Type of the 3RI Authentication."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::three_d_secure::ThreeDSecureStandaloneResourceThreeRiDetailsType,
}
