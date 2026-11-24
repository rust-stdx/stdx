#[cfg(feature = "radar")]
#[derive(Debug, Clone)]
pub struct CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetails {
    #[doc = "Describes card money movement details."]
    pub card: Option<stripe_types_05::radar::CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsCard>,
    #[doc = "Describes the type of money movement."]
    pub money_movement_type:
        stripe_types_01::radar::CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsMoneyMovementType,
}

#[cfg(feature = "radar")]
impl crate::params::FormEncode for CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("card", self.card.as_ref());
        form.field("money_movement_type", &self.money_movement_type);
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InsightsResourcesPaymentEvaluationEvent {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub dispute_opened: Option<stripe_types_06::radar::InsightsResourcesPaymentEvaluationDisputeOpened>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub early_fraud_warning_received:
        Option<stripe_types_05::radar::InsightsResourcesPaymentEvaluationEarlyFraudWarningReceived>,
    #[doc = "Timestamp when the event occurred."]
    #[json(default)]
    pub occurred_at: i64,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refunded: Option<stripe_types_05::radar::InsightsResourcesPaymentEvaluationRefunded>,
    #[doc = "Indicates the type of event attached to the payment evaluation."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::radar::InsightsResourcesPaymentEvaluationEventType,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub user_intervention_raised:
        Option<stripe_types_05::radar::InsightsResourcesPaymentEvaluationUserInterventionRaised>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub user_intervention_resolved:
        Option<stripe_types_05::radar::InsightsResourcesPaymentEvaluationUserInterventionResolved>,
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, Default)]
pub struct ListRadarValueListsParams {
    pub alias: Option<String>,
    pub contains: Option<String>,
    pub created: Option<stripe_types_05::radar::ListRadarValueListsParamsCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "radar")]
impl crate::params::FormEncode for ListRadarValueListsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("alias", self.alias.as_ref());
        form.field_opt("contains", self.contains.as_ref());
        form.field_opt("created", self.created.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}
