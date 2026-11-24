#[cfg(feature = "three_d_secure")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ThreeDSecureAuthentication {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub acquirer_details: Option<stripe_types_04::three_d_secure::ThreeDSecureStandaloneResourceAcquirerDetails>,
    #[doc = "The amount for this 3DS Authentication."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[doc = "The URL for presenting a challenge to your cardholder, present if status is requires_challenge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub challenge_url: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<stripe_types_08::three_d_secure::ThreeDSecureStandaloneResourceChannel>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "The 3DS directory server with which this 3DS Authentication was processed."]
    #[json(default)]
    pub directory_server: stripe_types_02::three_d_secure::ThreeDSecureAuthenticationDirectoryServer,
    #[doc = "The URL for performing issuer fingerprinting, present if fingerprinting is supported for the given payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fingerprinting_url: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub flow_preference: Option<stripe_types_07::three_d_secure::ThreeDSecureStandaloneResourceFlowPreference>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub future_usage: Option<stripe_types_10::three_d_secure::ThreeDSecureStandaloneResourceFutureUsage>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Indicates whether this 3DS Authentication is being performed for a payment or non-payment use case."]
    #[json(default)]
    pub message_category: stripe_types_01::three_d_secure::ThreeDSecureAuthenticationMessageCategory,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::three_d_secure::ThreeDSecureAuthenticationObject,
    #[doc = "The outcome of this 3DS Authentication."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<stripe_types_04::three_d_secure::ThreeDSecureAuthenticationOutcome>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub outcome_details: Option<stripe_types_07::three_d_secure::ThreeDSecureStandaloneResourceOutcomeDetails>,
    #[doc = "ID of the payment method (a PaymentMethod object) to attach to this 3DS Authentication."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<crate::Expandable<stripe_types_14::payment_methods::PaymentMethod>>,
    #[doc = "The reason for invoking this 3DS Authentication."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<stripe_types_04::three_d_secure::ThreeDSecureAuthenticationReason>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<stripe_types_00::three_d_secure::ThreeDSecureStandaloneResourceShippingAddress>,
    #[doc = "Status of this Authentication."]
    #[json(default)]
    pub status: stripe_types_03::three_d_secure::ThreeDSecureAuthenticationStatus,
}

#[cfg(feature = "three_d_secure")]
impl crate::PagedItem for ThreeDSecureAuthentication {
    fn id(&self) -> &str {
        &self.id
    }
}
