#[cfg(feature = "radar")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InsightsResourcesPaymentEvaluationPaymentDetails {
    #[doc = "Amount intended to be collected by this payment."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "An arbitrary string attached to the object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Details about the payment's customer presence and type."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub money_movement_details: Option<stripe_types_08::radar::InsightsResourcesPaymentEvaluationMoneyMovementDetails>,
    #[doc = "Details about the payment method used for the payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_details: Option<crate::radar::InsightsResourcesPaymentEvaluationPaymentMethodDetails>,
    #[doc = "Shipping details for the payment evaluation."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping_details: Option<stripe_types_05::radar::InsightsResourcesPaymentEvaluationShipping>,
    #[doc = "Payment statement descriptor."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InsightsResourcesPaymentEvaluationPaymentMethodDetails {
    #[doc = "Billing information associated with the payment evaluation."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<stripe_types_06::radar::InsightsResourcesPaymentEvaluationBillingDetails>,
    #[doc = "The payment method used in this payment evaluation."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<crate::Expandable<stripe_types_14::payment_methods::PaymentMethod>>,
}

#[cfg(any(feature = "events", feature = "radar"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct RadarEarlyFraudWarning {
    #[doc = "An EFW is actionable if it has not received a dispute and has not been fully refunded."]
    #[json(default)]
    pub actionable: bool,
    #[doc = "ID of the charge this early fraud warning is for, optionally expanded."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub charge: Option<crate::Expandable<stripe_types_14::charges::Charge>>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "The type of fraud labelled by the issuer."]
    #[json(default)]
    pub fraud_type: String,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::radar::RadarEarlyFraudWarningObject,
    #[doc = "ID of the Payment Intent this early fraud warning is for, optionally expanded."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<crate::Expandable<stripe_types_14::payment_intents::PaymentIntent>>,
}

#[cfg(any(feature = "events", feature = "radar"))]
impl crate::PagedItem for RadarEarlyFraudWarning {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct RadarPaymentEvaluation {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub client_device_metadata_details:
        Option<stripe_types_03::radar::InsightsResourcesPaymentEvaluationClientDeviceMetadata>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created_at: i64,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_details: Option<stripe_types_01::radar::InsightsResourcesPaymentEvaluationCustomerDetails>,
    #[doc = "Event information associated with the payment evaluation, such as refunds, dispute, early fraud warnings, or user int..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<stripe_types_07::radar::InsightsResourcesPaymentEvaluationEvent>>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::radar::RadarPaymentEvaluationObject,
    #[doc = "Indicates the final outcome for the payment evaluation."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<stripe_types_11::radar::InsightsResourcesPaymentEvaluationOutcome>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_details: Option<crate::radar::InsightsResourcesPaymentEvaluationPaymentDetails>,
    #[doc = "Recommended action based on the score of the `fraudulent_payment` signal."]
    #[json(default)]
    pub recommended_action: stripe_types_00::radar::RadarPaymentEvaluationRecommendedAction,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub signals: Option<stripe_types_08::radar::InsightsResourcesPaymentEvaluationSignals>,
}

#[cfg(feature = "radar")]
impl crate::PagedItem for RadarPaymentEvaluation {
    fn id(&self) -> &str {
        &self.id
    }
}
