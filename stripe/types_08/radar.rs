#[cfg(feature = "radar")]
#[derive(Debug, Clone, Default)]
pub struct CreateRadarPaymentEvaluationParamsPaymentDetailsPaymentMethodDetails {
    #[doc = "Billing information associated with the payment evaluation."]
    pub billing_details: Option<
        stripe_types_06::radar::CreateRadarPaymentEvaluationParamsPaymentDetailsPaymentMethodDetailsBillingDetails,
    >,
    #[doc = "ID of the payment method used in this payment evaluation."]
    pub payment_method: String,
}

#[cfg(feature = "radar")]
impl crate::params::FormEncode for CreateRadarPaymentEvaluationParamsPaymentDetailsPaymentMethodDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("billing_details", self.billing_details.as_ref());
        form.field("payment_method", &self.payment_method);
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InsightsResourcesPaymentEvaluationMoneyMovementDetails {
    #[doc = "Describes card money movement details."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types_06::radar::InsightsResourcesPaymentEvaluationMoneyMovementCard>,
    #[doc = "Describes the type of money movement."]
    #[json(default)]
    pub money_movement_type:
        stripe_types_04::radar::InsightsResourcesPaymentEvaluationMoneyMovementDetailsMoneyMovementType,
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InsightsResourcesPaymentEvaluationRejected {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types_06::radar::InsightsResourcesPaymentEvaluationRejectedCard>,
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InsightsResourcesPaymentEvaluationSignals {
    #[doc = "The likelihood that this `PaymentEvaluation` results in an early fraud warning."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub early_fraud_warning: Option<stripe_types_05::radar::InsightsResourcesPaymentEvaluationSignalV2>,
    #[doc = "The likelihood that this `PaymentEvaluation` results in a dispute with reason code `fraudulent`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fraudulent_dispute: Option<stripe_types_05::radar::InsightsResourcesPaymentEvaluationSignalV2>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fraudulent_payment: Option<stripe_types_05::radar::InsightsResourcesPaymentEvaluationSignalV2>,
}
