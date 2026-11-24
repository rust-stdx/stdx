#[cfg(feature = "radar")]
#[derive(Debug, Clone)]
pub struct CreateRadarPaymentEvaluationParamsPaymentDetails {
    #[doc = "The intended amount to collect with this payment."]
    pub amount: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "An arbitrary string attached to the object."]
    pub description: Option<String>,
    #[doc = "Details about the payment's customer presence and type."]
    pub money_movement_details:
        Option<stripe_types_07::radar::CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetails>,
    #[doc = "Details about the payment method to use for the payment."]
    pub payment_method_details:
        stripe_types_08::radar::CreateRadarPaymentEvaluationParamsPaymentDetailsPaymentMethodDetails,
    #[doc = "Shipping details for the payment evaluation."]
    pub shipping_details:
        Option<stripe_types_09::radar::CreateRadarPaymentEvaluationParamsPaymentDetailsShippingDetails>,
    #[doc = "Payment statement descriptor."]
    pub statement_descriptor: Option<String>,
}

#[cfg(feature = "radar")]
impl crate::params::FormEncode for CreateRadarPaymentEvaluationParamsPaymentDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        form.field("currency", &self.currency);
        form.field_opt("description", self.description.as_ref());
        form.field_opt("money_movement_details", self.money_movement_details.as_ref());
        form.field("payment_method_details", &self.payment_method_details);
        form.field_opt("shipping_details", self.shipping_details.as_ref());
        form.field_opt("statement_descriptor", self.statement_descriptor.as_ref());
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InsightsResourcesPaymentEvaluationOutcome {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub merchant_blocked: Option<stripe_types_05::radar::InsightsResourcesPaymentEvaluationMerchantBlocked>,
    #[doc = "The PaymentIntent ID associated with the payment evaluation."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_intent_id: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub rejected: Option<stripe_types_08::radar::InsightsResourcesPaymentEvaluationRejected>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub succeeded: Option<stripe_types_09::radar::InsightsResourcesPaymentEvaluationSucceeded>,
    #[doc = "Indicates the outcome of the payment evaluation."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::radar::InsightsResourcesPaymentEvaluationOutcomeType,
}
