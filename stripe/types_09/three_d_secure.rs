#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone)]
pub struct CreateThreeDSecureAuthenticationParamsChannel {
    #[doc = "Contains additional details about the browser details you collected."]
    pub browser: Option<stripe_types_02::three_d_secure::CreateThreeDSecureAuthenticationParamsChannelBrowser>,
    #[doc = "Contains additional details about the 3DS Requestor Initiated (3RI) channel."]
    pub three_r_i: Option<stripe_types_06::three_d_secure::CreateThreeDSecureAuthenticationParamsChannelThreeRI>,
    #[doc = "Type of channel you would prefer to use for this 3DS Authentication."]
    pub r#type: stripe_types_01::three_d_secure::CreateThreeDSecureAuthenticationParamsChannelType,
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for CreateThreeDSecureAuthenticationParamsChannel {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("browser", self.browser.as_ref());
        form.field_opt("three_r_i", self.three_r_i.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone)]
pub struct CreateThreeDSecureAuthenticationParamsFutureUsageInstallment {
    #[doc = "A non-negative integer representing the future authorizations' amount in the smallest currency unit..."]
    pub amount: i64,
    #[doc = "Information about the expiry of the future usage of this authentication."]
    pub expiry: stripe_types_06::three_d_secure::CreateThreeDSecureAuthenticationParamsFutureUsageInstallmentExpiry,
    #[doc = "The unit of time for `interval_count`."]
    pub interval:
        Option<stripe_types_03::three_d_secure::CreateThreeDSecureAuthenticationParamsFutureUsageInstallmentInterval>,
    #[doc = "The minimum number of time intervals between authorizations."]
    pub interval_count: Option<i64>,
    #[doc = "The maximum number of installments."]
    pub number: i64,
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for CreateThreeDSecureAuthenticationParamsFutureUsageInstallment {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        form.field("expiry", &self.expiry);
        form.field_opt("interval", self.interval.as_ref());
        form.field_opt("interval_count", self.interval_count.as_ref());
        form.field("number", &self.number);
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone)]
pub struct CreateThreeDSecureAuthenticationParamsPaymentMethodData {
    #[doc = "Billing information associated with the PaymentMethod that may be used or required by particular types of payment met..."]
    pub billing_details:
        Option<stripe_types_05::three_d_secure::CreateThreeDSecureAuthenticationParamsPaymentMethodDataBillingDetails>,
    pub card: json::OwnedValue,
    #[doc = "The type of the PaymentMethod."]
    pub r#type: stripe_types_02::three_d_secure::CreateThreeDSecureAuthenticationParamsPaymentMethodDataType,
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for CreateThreeDSecureAuthenticationParamsPaymentMethodData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("billing_details", self.billing_details.as_ref());
        form.field("type", &self.r#type);
    }
}
