#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateForceCaptureParamsPurchaseDetailsFleet {
    #[doc = "Answers to prompts presented to the cardholder at the point of sale."]
    pub cardholder_prompt_data:
        Option<stripe_types_05::test_helpers::CreateForceCaptureParamsPurchaseDetailsFleetCardholderPromptData>,
    #[doc = "The type of purchase."]
    pub purchase_type: Option<stripe_types_05::test_helpers::CreateForceCaptureParamsPurchaseDetailsFleetPurchaseType>,
    #[doc = "More information about the total amount."]
    pub reported_breakdown:
        Option<stripe_types_07::test_helpers::CreateForceCaptureParamsPurchaseDetailsFleetReportedBreakdown>,
    #[doc = "The type of fuel service."]
    pub service_type: Option<stripe_types_06::test_helpers::CreateForceCaptureParamsPurchaseDetailsFleetServiceType>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateForceCaptureParamsPurchaseDetailsFleet {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("cardholder_prompt_data", self.cardholder_prompt_data.as_ref());
        form.field_opt("purchase_type", self.purchase_type.as_ref());
        form.field_opt("reported_breakdown", self.reported_breakdown.as_ref());
        form.field_opt("service_type", self.service_type.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperIssuingAuthorizationParamsRiskAssessment {
    #[doc = "Stripe's assessment of this authorization's likelihood of being card testing activity."]
    pub card_testing_risk:
        Option<stripe_types_06::test_helpers::CreateTestHelperIssuingAuthorizationParamsRiskAssessmentCardTestingRisk>,
    #[doc = "Stripe’s assessment of this authorization’s likelihood to be fraudulent."]
    pub fraud_risk:
        Option<stripe_types_09::test_helpers::CreateTestHelperIssuingAuthorizationParamsRiskAssessmentFraudRisk>,
    #[doc = "The dispute risk of the merchant (the seller on a purchase) on an authorization based on all Stripe Issuing activity."]
    pub merchant_dispute_risk: Option<
        stripe_types_09::test_helpers::CreateTestHelperIssuingAuthorizationParamsRiskAssessmentMerchantDisputeRisk,
    >,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperIssuingAuthorizationParamsRiskAssessment {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("card_testing_risk", self.card_testing_risk.as_ref());
        form.field_opt("fraud_risk", self.fraud_risk.as_ref());
        form.field_opt("merchant_dispute_risk", self.merchant_dispute_risk.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct FinalizeAmountParamsFleet {
    #[doc = "Answers to prompts presented to the cardholder at the point of sale."]
    pub cardholder_prompt_data: Option<stripe_types_06::test_helpers::FinalizeAmountParamsFleetCardholderPromptData>,
    #[doc = "The type of purchase."]
    pub purchase_type: Option<stripe_types_06::test_helpers::FinalizeAmountParamsFleetPurchaseType>,
    #[doc = "More information about the total amount."]
    pub reported_breakdown: Option<stripe_types_07::test_helpers::FinalizeAmountParamsFleetReportedBreakdown>,
    #[doc = "The type of fuel service."]
    pub service_type: Option<stripe_types_06::test_helpers::FinalizeAmountParamsFleetServiceType>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for FinalizeAmountParamsFleet {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("cardholder_prompt_data", self.cardholder_prompt_data.as_ref());
        form.field_opt("purchase_type", self.purchase_type.as_ref());
        form.field_opt("reported_breakdown", self.reported_breakdown.as_ref());
        form.field_opt("service_type", self.service_type.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone)]
pub struct UpdateTestHelperTreasuryOutboundTransferParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Details about network-specific tracking information."]
    pub tracking_details: stripe_types_07::test_helpers::UpdateTestHelperTreasuryOutboundTransferParamsTrackingDetails,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for UpdateTestHelperTreasuryOutboundTransferParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("tracking_details", &self.tracking_details);
    }
}
