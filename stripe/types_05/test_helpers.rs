#[cfg(feature = "test_helpers")]
pub type ActivateTestHelperIssuingPersonalizationDesignParams =
    stripe_types_03::test_helpers::ExpireTestHelperIssuingAuthorizationParams;

#[cfg(feature = "test_helpers")]
pub type CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsFleetPurchaseType =
    stripe_types_02::test_helpers::CreateTestHelperIssuingAuthorizationParamsFleetPurchaseType;

#[cfg(feature = "test_helpers")]
pub type CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsFleetReportedBreakdownFuel =
    stripe_types_03::test_helpers::CreateTestHelperIssuingAuthorizationParamsFleetReportedBreakdownFuel;

#[cfg(feature = "test_helpers")]
pub type CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsFleetReportedBreakdownNonFuel =
    stripe_types_03::test_helpers::CreateTestHelperIssuingAuthorizationParamsFleetReportedBreakdownFuel;

#[cfg(feature = "test_helpers")]
pub type CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsFleetReportedBreakdownTax =
    stripe_types_00::test_helpers::CreateTestHelperIssuingAuthorizationParamsFleetReportedBreakdownTax;

#[cfg(feature = "test_helpers")]
pub type CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsFuelUnit =
    stripe_types_03::test_helpers::CreateTestHelperIssuingAuthorizationParamsFuelUnit;

#[cfg(feature = "test_helpers")]
pub type CreateForceCaptureParamsPurchaseDetailsFleetCardholderPromptData =
    stripe_types_04::test_helpers::CreateTestHelperIssuingAuthorizationParamsFleetCardholderPromptData;

#[cfg(feature = "test_helpers")]
pub type CreateForceCaptureParamsPurchaseDetailsFleetPurchaseType =
    stripe_types_02::test_helpers::CreateTestHelperIssuingAuthorizationParamsFleetPurchaseType;

#[cfg(feature = "test_helpers")]
pub type CreateForceCaptureParamsPurchaseDetailsFlightSegmentsItem =
    stripe_types_02::test_helpers::CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsFlightSegmentsItem;

#[cfg(feature = "test_helpers")]
pub type CreateForceCaptureParamsPurchaseDetailsFuelUnit =
    stripe_types_03::test_helpers::CreateTestHelperIssuingAuthorizationParamsFuelUnit;

#[cfg(feature = "test_helpers")]
pub type CreateForceCaptureParamsPurchaseDetailsLodging =
    stripe_types_00::test_helpers::CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsLodging;

#[cfg(feature = "test_helpers")]
pub type CreateForceCaptureParamsPurchaseDetailsReceiptItem =
    stripe_types_04::test_helpers::CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsReceiptItem;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataAlipay =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataAmazonPay =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperConfirmationTokenParamsPaymentMethodDataBillingDetails {
    #[doc = "Billing address."]
    pub address: Option<
        stripe_types_02::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataBillingDetailsAddress,
    >,
    #[doc = "Email address."]
    pub email: Option<json::OwnedValue>,
    #[doc = "Full name."]
    pub name: Option<json::OwnedValue>,
    #[doc = "Billing phone number (including extension)."]
    pub phone: Option<json::OwnedValue>,
    #[doc = "Taxpayer identification number."]
    pub tax_id: Option<String>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodDataBillingDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("address", self.address.as_ref());
        form.field_opt("tax_id", self.tax_id.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataBizum =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataBlik =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataCashapp =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataCrypto =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperConfirmationTokenParamsPaymentMethodDataEps {
    #[doc = "The customer's bank."]
    pub bank: Option<stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodDataEps {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("bank", self.bank.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone)]
pub struct CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpx {
    #[doc = "Account holder type for FPX transaction"]
    pub account_holder_type: Option<
        stripe_types_03::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxAccountHolderType,
    >,
    #[doc = "The customer's bank."]
    pub bank: stripe_types_03::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxBank,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpx {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account_holder_type", self.account_holder_type.as_ref());
        form.field("bank", &self.bank);
    }
}

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataGrabpay =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataInteracPresent =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataMultibanco =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperConfirmationTokenParamsPaymentMethodDataNaverPay {
    #[doc = "Whether to use Naver Pay points or a card to fund this transaction."]
    pub funding:
        Option<stripe_types_01::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataNaverPayFunding>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodDataNaverPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("funding", self.funding.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataOxxo =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataPayByBank =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataPayco =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataPaynow =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataPaypay =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataSamsungPay =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataSatispay =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone)]
pub struct CreateTestHelperConfirmationTokenParamsPaymentMethodDataSofort {
    #[doc = "Two-letter ISO code representing the country the bank account is located in."]
    pub country: stripe_types_02::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataSofortCountry,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodDataSofort {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("country", &self.country);
    }
}

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataSunbit =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataTwint =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataUsBankAccountAccountHolderType =
    stripe_types_03::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpxAccountHolderType;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataWechatPay =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataZip =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsShippingAddress =
    stripe_types_02::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataBillingDetailsAddress;

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperIssuingAuthorizationParamsMerchantData {
    #[doc = "A categorization of the seller's type of business."]
    pub category: Option<stripe_types_02::test_helpers::CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory>,
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
impl crate::params::FormEncode for CreateTestHelperIssuingAuthorizationParamsMerchantData {
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
pub type CreateTestHelperIssuingAuthorizationParamsRiskAssessmentFraudRiskLevel =
    stripe_types_03::test_helpers::CreateTestHelperIssuingAuthorizationParamsRiskAssessmentCardTestingRiskLevel;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperIssuingAuthorizationParamsRiskAssessmentMerchantDisputeRiskLevel =
    stripe_types_03::test_helpers::CreateTestHelperIssuingAuthorizationParamsRiskAssessmentCardTestingRiskLevel;

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone)]
pub struct CreateTestHelperIssuingAuthorizationParamsVerificationDataAuthenticationExemption {
    #[doc = "The entity that requested the exemption, either the acquiring merchant or the Issuing user."]
    pub claimed_by: stripe_types_03::test_helpers::CreateTestHelperIssuingAuthorizationParamsVerificationDataAuthenticationExemptionClaimedBy,
    #[doc = "The specific exemption claimed for this authorization."]
    pub r#type: stripe_types_04::test_helpers::CreateTestHelperIssuingAuthorizationParamsVerificationDataAuthenticationExemptionType,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperIssuingAuthorizationParamsVerificationDataAuthenticationExemption {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("claimed_by", &self.claimed_by);
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperIssuingAuthorizationParamsVerificationDataCvcCheck =
    stripe_types_04::test_helpers::CreateTestHelperIssuingAuthorizationParamsVerificationDataAddressLine1Check;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperIssuingAuthorizationParamsVerificationDataExpiryCheck =
    stripe_types_04::test_helpers::CreateTestHelperIssuingAuthorizationParamsVerificationDataAddressLine1Check;

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone)]
pub struct CreateTestHelperIssuingAuthorizationParamsVerificationDataThreeDSecure {
    #[doc = "The outcome of the 3D Secure authentication request."]
    pub result:
        stripe_types_01::test_helpers::CreateTestHelperIssuingAuthorizationParamsVerificationDataThreeDSecureResult,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperIssuingAuthorizationParamsVerificationDataThreeDSecure {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("result", &self.result);
    }
}

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperTreasuryReceivedDebitParamsInitiatingPaymentMethodDetailsType =
    stripe_types_00::test_helpers::CreateTestHelperTreasuryReceivedCreditParamsInitiatingPaymentMethodDetailsType;

#[cfg(feature = "test_helpers")]
pub type CreateUnlinkedRefundParamsPurchaseDetailsFleetCardholderPromptData =
    stripe_types_04::test_helpers::CreateTestHelperIssuingAuthorizationParamsFleetCardholderPromptData;

#[cfg(feature = "test_helpers")]
pub type CreateUnlinkedRefundParamsPurchaseDetailsFleetReportedBreakdownFuel =
    stripe_types_03::test_helpers::CreateTestHelperIssuingAuthorizationParamsFleetReportedBreakdownFuel;

#[cfg(feature = "test_helpers")]
pub type CreateUnlinkedRefundParamsPurchaseDetailsFleetReportedBreakdownNonFuel =
    stripe_types_03::test_helpers::CreateTestHelperIssuingAuthorizationParamsFleetReportedBreakdownFuel;

#[cfg(feature = "test_helpers")]
pub type CreateUnlinkedRefundParamsPurchaseDetailsFuelType =
    stripe_types_02::test_helpers::CreateTestHelperIssuingAuthorizationParamsFuelType;

#[cfg(feature = "test_helpers")]
pub type CreateUnlinkedRefundParamsPurchaseDetailsLodging =
    stripe_types_00::test_helpers::CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsLodging;

#[cfg(feature = "test_helpers")]
pub type DeactivateTestHelperIssuingPersonalizationDesignParams =
    stripe_types_03::test_helpers::ExpireTestHelperIssuingAuthorizationParams;

#[cfg(feature = "test_helpers")]
pub type DeliverTestHelperIssuingCardShippingParams =
    stripe_types_03::test_helpers::ExpireTestHelperIssuingAuthorizationParams;

#[cfg(feature = "test_helpers")]
pub type FailTestHelperIssuingCardShippingParams =
    stripe_types_03::test_helpers::ExpireTestHelperIssuingAuthorizationParams;

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct FailTestHelperTreasuryInboundTransferParamsFailureDetails {
    #[doc = "Reason for the failure."]
    pub code: Option<stripe_types_01::test_helpers::FailTestHelperTreasuryInboundTransferParamsFailureDetailsCode>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for FailTestHelperTreasuryInboundTransferParamsFailureDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("code", self.code.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
pub type FailTestHelperTreasuryOutboundTransferParams =
    stripe_types_03::test_helpers::ExpireTestHelperIssuingAuthorizationParams;

#[cfg(feature = "test_helpers")]
pub type FinalizeAmountParamsFleetReportedBreakdownNonFuel =
    stripe_types_03::test_helpers::CreateTestHelperIssuingAuthorizationParamsFleetReportedBreakdownFuel;

#[cfg(feature = "test_helpers")]
pub type FinalizeAmountParamsFleetReportedBreakdownTax =
    stripe_types_00::test_helpers::CreateTestHelperIssuingAuthorizationParamsFleetReportedBreakdownTax;

#[cfg(feature = "test_helpers")]
pub type FinalizeAmountParamsFuelUnit =
    stripe_types_03::test_helpers::CreateTestHelperIssuingAuthorizationParamsFuelUnit;

#[cfg(feature = "test_helpers")]
pub type GetTestHelperTestClockParams = stripe_types_03::test_helpers::ExpireTestHelperIssuingAuthorizationParams;

#[cfg(feature = "test_helpers")]
pub type PostTestHelperTreasuryOutboundPaymentParams =
    stripe_types_03::test_helpers::ExpireTestHelperIssuingAuthorizationParams;

#[cfg(feature = "test_helpers")]
pub type PresentPaymentMethodParamsInteracPresent =
    stripe_types_02::test_helpers::PresentPaymentMethodParamsCardPresent;

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasons {
    #[doc = "The reason(s) the card logo was rejected."]
    pub card_logo: Option<Vec<stripe_types_04::test_helpers::RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCardLogoItem>>,
    #[doc = "The reason(s) the carrier text was rejected."]
    pub carrier_text: Option<Vec<stripe_types_03::test_helpers::RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasonsCarrierTextItem>>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for RejectTestHelperIssuingPersonalizationDesignParamsRejectionReasons {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.card_logo.as_ref() {
            form.field_array("card_logo", values.as_slice());
        }
        if let Some(values) = self.carrier_text.as_ref() {
            form.field_array("carrier_text", values.as_slice());
        }
    }
}

#[cfg(feature = "test_helpers")]
pub type ReturnTestHelperIssuingCardShippingParams =
    stripe_types_03::test_helpers::ExpireTestHelperIssuingAuthorizationParams;

#[cfg(feature = "test_helpers")]
pub type ReturnTestHelperTreasuryOutboundTransferParamsReturnedDetailsCode =
    stripe_types_04::test_helpers::ReturnTestHelperTreasuryOutboundPaymentParamsReturnedDetailsCode;

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct SucceedInputCollectionParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "This parameter defines the skip behavior for input collection."]
    pub skip_non_required_inputs:
        Option<stripe_types_02::test_helpers::SucceedInputCollectionParamsSkipNonRequiredInputs>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for SucceedInputCollectionParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("skip_non_required_inputs", self.skip_non_required_inputs.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
pub type UpdateTestHelperTreasuryOutboundTransferParamsTrackingDetailsUsDomesticWire =
    stripe_types_03::test_helpers::UpdateTestHelperTreasuryOutboundPaymentParamsTrackingDetailsUsDomesticWire;
