#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone)]
pub struct CreateThreeDSecureAuthenticationParamsChannelThreeRI {
    #[doc = "ID of a prior `Authentication`."]
    pub previous_authentication: String,
    #[doc = "It provides additional information to the ACS to determine the best approach for handling a 3RI request."]
    pub r#type: stripe_types_02::three_d_secure::CreateThreeDSecureAuthenticationParamsChannelThreeRIType,
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for CreateThreeDSecureAuthenticationParamsChannelThreeRI {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("previous_authentication", &self.previous_authentication);
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone)]
pub struct CreateThreeDSecureAuthenticationParamsFlowPreferenceChallenge {
    #[doc = "Type of challenge flow you requested for this 3DS Authentication."]
    pub r#type: stripe_types_00::three_d_secure::CreateThreeDSecureAuthenticationParamsFlowPreferenceChallengeType,
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for CreateThreeDSecureAuthenticationParamsFlowPreferenceChallenge {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone)]
pub struct CreateThreeDSecureAuthenticationParamsFlowPreferenceDataShare {
    #[doc = "Type of data share only flow you requested for this 3DS Authentication."]
    pub r#type: stripe_types_03::three_d_secure::CreateThreeDSecureAuthenticationParamsFlowPreferenceDataShareType,
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for CreateThreeDSecureAuthenticationParamsFlowPreferenceDataShare {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone)]
pub struct CreateThreeDSecureAuthenticationParamsFlowPreferenceFrictionless {
    #[doc = "Type of frictionless flow you requested for this 3DS Authentication."]
    pub r#type: stripe_types_01::three_d_secure::CreateThreeDSecureAuthenticationParamsFlowPreferenceFrictionlessType,
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for CreateThreeDSecureAuthenticationParamsFlowPreferenceFrictionless {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone)]
pub struct CreateThreeDSecureAuthenticationParamsFutureUsageInstallmentExpiry {
    #[doc = "The date before which the last authorization related to this authentication will occur."]
    pub date: Option<String>,
    #[doc = "The type of expiry for the future use of this authentication."]
    pub r#type: stripe_types_00::three_d_secure::CreateThreeDSecureAuthenticationParamsFutureUsageInstallmentExpiryType,
}

#[cfg(feature = "three_d_secure")]
impl crate::params::FormEncode for CreateThreeDSecureAuthenticationParamsFutureUsageInstallmentExpiry {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("date", self.date.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "three_d_secure")]
pub type CreateThreeDSecureAuthenticationParamsFutureUsageRecurringInterval =
    stripe_types_03::three_d_secure::CreateThreeDSecureAuthenticationParamsFutureUsageInstallmentInterval;

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ThreeDSecureDetails {
    #[doc = "For authenticated transactions: how the customer was authenticated by the issuing bank."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub authentication_flow: Option<stripe_types_00::three_d_secure::ThreeDSecureDetailsAuthenticationFlow>,
    #[doc = "The Electronic Commerce Indicator (ECI)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub electronic_commerce_indicator:
        Option<stripe_types_00::three_d_secure::ThreeDSecureDetailsElectronicCommerceIndicator>,
    #[doc = "Indicates the outcome of 3D Secure authentication."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<stripe_types_03::three_d_secure::ThreeDSecureDetailsResult>,
    #[doc = "Additional information about why 3D Secure succeeded or failed based on the `result`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub result_reason: Option<stripe_types_00::three_d_secure::ThreeDSecureDetailsResultReason>,
    #[doc = "The 3D Secure 1 XID or 3D Secure 2 Directory Server Transaction ID (dsTransId) for this payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[doc = "The version of 3D Secure that was used."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<stripe_types_01::three_d_secure::ThreeDSecureDetailsVersion>,
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ThreeDSecureDetailsCharge {
    #[doc = "For authenticated transactions: how the customer was authenticated by the issuing bank."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub authentication_flow: Option<stripe_types_00::three_d_secure::ThreeDSecureDetailsAuthenticationFlow>,
    #[doc = "The Electronic Commerce Indicator (ECI)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub electronic_commerce_indicator:
        Option<stripe_types_00::three_d_secure::ThreeDSecureDetailsElectronicCommerceIndicator>,
    #[doc = "The exemption requested via 3DS and accepted by the issuer at authentication time."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub exemption_indicator: Option<stripe_types_01::three_d_secure::ThreeDSecureDetailsChargeExemptionIndicator>,
    #[doc = "Whether Stripe requested the value of `exemption_indicator` in the transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub exemption_indicator_applied: Option<bool>,
    #[doc = "Indicates the outcome of 3D Secure authentication."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<stripe_types_03::three_d_secure::ThreeDSecureDetailsResult>,
    #[doc = "Additional information about why 3D Secure succeeded or failed based on the `result`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub result_reason: Option<stripe_types_00::three_d_secure::ThreeDSecureDetailsResultReason>,
    #[doc = "The 3D Secure 1 XID or 3D Secure 2 Directory Server Transaction ID (dsTransId) for this payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[doc = "The version of 3D Secure that was used."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<stripe_types_01::three_d_secure::ThreeDSecureDetailsVersion>,
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ThreeDSecureStandaloneResourceDataShareDetails {
    #[doc = "Type of data share flow you requested for this 3DS Authentication."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::three_d_secure::ThreeDSecureStandaloneResourceDataShareDetailsType,
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ThreeDSecureStandaloneResourceExpiry {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::three_d_secure::ThreeDSecureStandaloneResourceExpiryType,
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ThreeDSecureStandaloneResourceFrictionlessDetails {
    #[doc = "Type of frictionless flow you requested for this 3DS Authentication."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::three_d_secure::ThreeDSecureDetailsChargeExemptionIndicator,
}

#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ThreeDSecureStandaloneResourceNetworkDetails {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires: Option<stripe_types_04::three_d_secure::ThreeDSecureStandaloneResourceCartesBancaires>,
}
