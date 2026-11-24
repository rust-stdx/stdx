#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsFleetReportedBreakdown {
    #[doc = "Breakdown of fuel portion of the purchase."]
    pub fuel: Option<stripe_types_05::test_helpers::CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsFleetReportedBreakdownFuel>,
    #[doc = "Breakdown of non-fuel portion of the purchase."]
    pub non_fuel: Option<stripe_types_05::test_helpers::CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsFleetReportedBreakdownNonFuel>,
    #[doc = "Information about tax included in this transaction."]
    pub tax: Option<stripe_types_05::test_helpers::CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsFleetReportedBreakdownTax>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsFleetReportedBreakdown {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("fuel", self.fuel.as_ref());
        form.field_opt("non_fuel", self.non_fuel.as_ref());
        form.field_opt("tax", self.tax.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateForceCaptureParamsMerchantData {
    #[doc = "A categorization of the seller's type of business."]
    pub category: Option<stripe_types_06::test_helpers::CreateForceCaptureParamsMerchantDataCategory>,
    #[doc = "City where the seller is located"]
    pub city: Option<String>,
    #[doc = "Country where the seller is located"]
    pub country: Option<String>,
    #[doc = "Name of the seller"]
    pub name: Option<String>,
    #[doc = "Identifier assigned to the seller by the card network."]
    pub network_id: Option<String>,
    #[doc = "Postal code where the seller is located"]
    pub postal_code: Option<String>,
    #[doc = "State where the seller is located"]
    pub state: Option<String>,
    #[doc = "An ID assigned by the seller to the location of the sale."]
    pub terminal_id: Option<String>,
    #[doc = "URL provided by the merchant on a 3DS request"]
    pub url: Option<String>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateForceCaptureParamsMerchantData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("category", self.category.as_ref());
        form.field_opt("city", self.city.as_ref());
        form.field_opt("country", self.country.as_ref());
        form.field_opt("name", self.name.as_ref());
        form.field_opt("network_id", self.network_id.as_ref());
        form.field_opt("postal_code", self.postal_code.as_ref());
        form.field_opt("state", self.state.as_ref());
        form.field_opt("terminal_id", self.terminal_id.as_ref());
        form.field_opt("url", self.url.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperConfirmationTokenParamsPaymentMethodDataUpi {
    #[doc = "Configuration options for setting up an eMandate"]
    pub mandate_options: Option<
        stripe_types_06::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataUpiMandateOptions,
    >,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodDataUpi {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone)]
pub struct CreateTestHelperIssuingAuthorizationParamsRiskAssessmentFraudRisk {
    #[doc = "Stripe’s assessment of the likelihood of fraud on an authorization."]
    pub level: stripe_types_05::test_helpers::CreateTestHelperIssuingAuthorizationParamsRiskAssessmentFraudRiskLevel,
    #[doc = "Stripe’s numerical model score assessing the likelihood of fraudulent activity."]
    pub score: Option<f64>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperIssuingAuthorizationParamsRiskAssessmentFraudRisk {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("level", &self.level);
        form.field_opt("score", self.score.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone)]
pub struct CreateTestHelperIssuingAuthorizationParamsRiskAssessmentMerchantDisputeRisk {
    #[doc = "The dispute rate observed across all Stripe Issuing authorizations for this merchant."]
    pub dispute_rate: Option<i64>,
    #[doc = "The likelihood that authorizations from this merchant will result in a dispute based on their history on Stripe Issuing."]
    pub level:
        stripe_types_05::test_helpers::CreateTestHelperIssuingAuthorizationParamsRiskAssessmentMerchantDisputeRiskLevel,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperIssuingAuthorizationParamsRiskAssessmentMerchantDisputeRisk {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("dispute_rate", self.dispute_rate.as_ref());
        form.field("level", &self.level);
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateUnlinkedRefundParamsPurchaseDetailsFleetReportedBreakdown {
    #[doc = "Breakdown of fuel portion of the purchase."]
    pub fuel:
        Option<stripe_types_05::test_helpers::CreateUnlinkedRefundParamsPurchaseDetailsFleetReportedBreakdownFuel>,
    #[doc = "Breakdown of non-fuel portion of the purchase."]
    pub non_fuel:
        Option<stripe_types_05::test_helpers::CreateUnlinkedRefundParamsPurchaseDetailsFleetReportedBreakdownNonFuel>,
    #[doc = "Information about tax included in this transaction."]
    pub tax: Option<stripe_types_06::test_helpers::CreateUnlinkedRefundParamsPurchaseDetailsFleetReportedBreakdownTax>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateUnlinkedRefundParamsPurchaseDetailsFleetReportedBreakdown {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("fuel", self.fuel.as_ref());
        form.field_opt("non_fuel", self.non_fuel.as_ref());
        form.field_opt("tax", self.tax.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateUnlinkedRefundParamsPurchaseDetailsFuel {
    #[doc = "[Conexxus Payment System Product Code](https://www.conexxus.org/conexxus-payment-system-product-codes) identifying th..."]
    pub industry_product_code: Option<String>,
    #[doc = "The quantity of `unit`s of fuel that was dispensed, represented as a decimal string with at most 12 decimal places."]
    pub quantity_decimal: Option<String>,
    #[doc = "The type of fuel that was purchased."]
    pub r#type: Option<stripe_types_05::test_helpers::CreateUnlinkedRefundParamsPurchaseDetailsFuelType>,
    #[doc = "The units for `quantity_decimal`."]
    pub unit: Option<stripe_types_06::test_helpers::CreateUnlinkedRefundParamsPurchaseDetailsFuelUnit>,
    #[doc = "The cost in cents per each unit of fuel, represented as a decimal string with at most 12 decimal places."]
    pub unit_cost_decimal: Option<String>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateUnlinkedRefundParamsPurchaseDetailsFuel {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("industry_product_code", self.industry_product_code.as_ref());
        form.field_opt("quantity_decimal", self.quantity_decimal.as_ref());
        form.field_opt("type", self.r#type.as_ref());
        form.field_opt("unit", self.unit.as_ref());
        form.field_opt("unit_cost_decimal", self.unit_cost_decimal.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct FinalizeAmountParamsFuel {
    #[doc = "[Conexxus Payment System Product Code](https://www.conexxus.org/conexxus-payment-system-product-codes) identifying th..."]
    pub industry_product_code: Option<String>,
    #[doc = "The quantity of `unit`s of fuel that was dispensed, represented as a decimal string with at most 12 decimal places."]
    pub quantity_decimal: Option<String>,
    #[doc = "The type of fuel that was purchased."]
    pub r#type: Option<stripe_types_06::test_helpers::FinalizeAmountParamsFuelType>,
    #[doc = "The units for `quantity_decimal`."]
    pub unit: Option<stripe_types_05::test_helpers::FinalizeAmountParamsFuelUnit>,
    #[doc = "The cost in cents per each unit of fuel, represented as a decimal string with at most 12 decimal places."]
    pub unit_cost_decimal: Option<String>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for FinalizeAmountParamsFuel {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("industry_product_code", self.industry_product_code.as_ref());
        form.field_opt("quantity_decimal", self.quantity_decimal.as_ref());
        form.field_opt("type", self.r#type.as_ref());
        form.field_opt("unit", self.unit.as_ref());
        form.field_opt("unit_cost_decimal", self.unit_cost_decimal.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone)]
pub struct RejectTestHelperIssuingPersonalizationDesignParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The reason(s) the personalization design was rejected."]
    pub rejection_reasons:
        stripe_types_05::test_helpers::RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasons,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for RejectTestHelperIssuingPersonalizationDesignParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("rejection_reasons", &self.rejection_reasons);
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct ReturnTestHelperTreasuryOutboundPaymentParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Optional hash to set the return code."]
    pub returned_details:
        Option<stripe_types_06::test_helpers::ReturnTestHelperTreasuryOutboundPaymentParamsReturnedDetails>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for ReturnTestHelperTreasuryOutboundPaymentParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("returned_details", self.returned_details.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct ReturnTestHelperTreasuryOutboundTransferParamsReturnedDetails {
    #[doc = "Reason for the return."]
    pub code: Option<stripe_types_05::test_helpers::ReturnTestHelperTreasuryOutboundTransferParamsReturnedDetailsCode>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for ReturnTestHelperTreasuryOutboundTransferParamsReturnedDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("code", self.code.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone)]
pub struct UpdateTestHelperTreasuryOutboundPaymentParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Details about network-specific tracking information."]
    pub tracking_details: stripe_types_06::test_helpers::UpdateTestHelperTreasuryOutboundPaymentParamsTrackingDetails,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for UpdateTestHelperTreasuryOutboundPaymentParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("tracking_details", &self.tracking_details);
    }
}
