#[cfg(feature = "test_helpers")]
pub type CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsFleetCardholderPromptData =
    stripe_types_04::test_helpers::CreateTestHelperIssuingAuthorizationParamsFleetCardholderPromptData;

#[cfg(feature = "test_helpers")]
pub type CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsFleetServiceType =
    stripe_types_00::test_helpers::CreateTestHelperIssuingAuthorizationParamsFleetServiceType;

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsFlight {
    #[doc = "The time that the flight departed."]
    pub departure_at: Option<i64>,
    #[doc = "The name of the passenger."]
    pub passenger_name: Option<String>,
    #[doc = "Whether the ticket is refundable."]
    pub refundable: Option<bool>,
    #[doc = "The legs of the trip."]
    pub segments: Option<
        Vec<
            stripe_types_02::test_helpers::CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsFlightSegmentsItem,
        >,
    >,
    #[doc = "The travel agency that issued the ticket."]
    pub travel_agency: Option<String>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsFlight {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("departure_at", self.departure_at.as_ref());
        form.field_opt("passenger_name", self.passenger_name.as_ref());
        form.field_opt("refundable", self.refundable.as_ref());
        if let Some(values) = self.segments.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("segments", index, item);
            }
        }
        form.field_opt("travel_agency", self.travel_agency.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
pub type CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsFuelType =
    stripe_types_02::test_helpers::CreateTestHelperIssuingAuthorizationParamsFuelType;

#[cfg(feature = "test_helpers")]
pub type CreateForceCaptureParamsMerchantDataCategory =
    stripe_types_02::test_helpers::CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory;

#[cfg(feature = "test_helpers")]
pub type CreateForceCaptureParamsPurchaseDetailsFleetReportedBreakdownFuel =
    stripe_types_03::test_helpers::CreateTestHelperIssuingAuthorizationParamsFleetReportedBreakdownFuel;

#[cfg(feature = "test_helpers")]
pub type CreateForceCaptureParamsPurchaseDetailsFleetReportedBreakdownNonFuel =
    stripe_types_03::test_helpers::CreateTestHelperIssuingAuthorizationParamsFleetReportedBreakdownFuel;

#[cfg(feature = "test_helpers")]
pub type CreateForceCaptureParamsPurchaseDetailsFleetReportedBreakdownTax =
    stripe_types_00::test_helpers::CreateTestHelperIssuingAuthorizationParamsFleetReportedBreakdownTax;

#[cfg(feature = "test_helpers")]
pub type CreateForceCaptureParamsPurchaseDetailsFleetServiceType =
    stripe_types_00::test_helpers::CreateTestHelperIssuingAuthorizationParamsFleetServiceType;

#[cfg(feature = "test_helpers")]
pub type CreateForceCaptureParamsPurchaseDetailsFuelType =
    stripe_types_02::test_helpers::CreateTestHelperIssuingAuthorizationParamsFuelType;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataAfterpayClearpay =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataAlma =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataBancontact =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataBillie =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataCustomerBalance =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataGiropay =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdeal {
    #[doc = "The customer's bank."]
    pub bank: Option<stripe_types_03::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdealBank>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdeal {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("bank", self.bank.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataKakaoPay =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperConfirmationTokenParamsPaymentMethodDataKlarna {
    #[doc = "Customer's date of birth"]
    pub dob: Option<stripe_types_04::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataKlarnaDob>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodDataKlarna {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("dob", self.dob.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataKonbini =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataKrCard =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataLink =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataMbWay =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataMobilepay =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24 {
    #[doc = "The customer's bank."]
    pub bank: Option<stripe_types_01::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24Bank>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24 {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("bank", self.bank.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataPaypal =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataPix =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataPromptpay =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataRevolutPay =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataScalapay =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataSequra =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperConfirmationTokenParamsPaymentMethodDataSwish =
    stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm;

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperConfirmationTokenParamsPaymentMethodDataUpiMandateOptions {
    #[doc = "Amount to be charged for future payments."]
    pub amount: Option<i64>,
    #[doc = "One of `fixed` or `maximum`."]
    pub amount_type: Option<stripe_types_01::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataUpiMandateOptionsAmountType>,
    #[doc = "A description of the mandate or subscription that is meant to be displayed to the customer."]
    pub description: Option<String>,
    #[doc = "End date of the mandate or subscription."]
    pub end_date: Option<i64>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodDataUpiMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("amount_type", self.amount_type.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field_opt("end_date", self.end_date.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone)]
pub struct CreateTestHelperConfirmationTokenParamsPaymentMethodOptionsCardInstallmentsPlan {
    #[doc = "For `fixed_count` installment plans, this is required."]
    pub count: Option<i64>,
    #[doc = "For `fixed_count` installment plans, this is required."]
    pub interval: Option<stripe_types_01::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodOptionsCardInstallmentsPlanInterval>,
    #[doc = "Type of installment plan, one of `fixed_count`, `bonus`, or `revolving`."]
    pub r#type: stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodOptionsCardInstallmentsPlanType,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodOptionsCardInstallmentsPlan {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("count", self.count.as_ref());
        form.field_opt("interval", self.interval.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperIssuingAuthorizationParamsFleetReportedBreakdownNonFuel =
    stripe_types_03::test_helpers::CreateTestHelperIssuingAuthorizationParamsFleetReportedBreakdownFuel;

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperIssuingAuthorizationParamsFuel {
    #[doc = "[Conexxus Payment System Product Code](https://www.conexxus.org/conexxus-payment-system-product-codes) identifying th..."]
    pub industry_product_code: Option<String>,
    #[doc = "The quantity of `unit`s of fuel that was dispensed, represented as a decimal string with at most 12 decimal places."]
    pub quantity_decimal: Option<String>,
    #[doc = "The type of fuel that was purchased."]
    pub r#type: Option<stripe_types_02::test_helpers::CreateTestHelperIssuingAuthorizationParamsFuelType>,
    #[doc = "The units for `quantity_decimal`."]
    pub unit: Option<stripe_types_03::test_helpers::CreateTestHelperIssuingAuthorizationParamsFuelUnit>,
    #[doc = "The cost in cents per each unit of fuel, represented as a decimal string with at most 12 decimal places."]
    pub unit_cost_decimal: Option<String>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperIssuingAuthorizationParamsFuel {
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
pub struct CreateTestHelperIssuingAuthorizationParamsRiskAssessmentCardTestingRisk {
    #[doc = "The % of declines due to a card number not existing in the past hour, taking place at the same merchant."]
    pub invalid_account_number_decline_rate_past_hour: Option<i64>,
    #[doc = "The % of declines due to incorrect verification data (like CVV or expiry) in the past hour, taking place at the same ..."]
    pub invalid_credentials_decline_rate_past_hour: Option<i64>,
    #[doc = "The likelihood that this authorization is associated with card testing activity."]
    pub level:
        stripe_types_03::test_helpers::CreateTestHelperIssuingAuthorizationParamsRiskAssessmentCardTestingRiskLevel,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperIssuingAuthorizationParamsRiskAssessmentCardTestingRisk {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt(
            "invalid_account_number_decline_rate_past_hour",
            self.invalid_account_number_decline_rate_past_hour.as_ref(),
        );
        form.field_opt(
            "invalid_credentials_decline_rate_past_hour",
            self.invalid_credentials_decline_rate_past_hour.as_ref(),
        );
        form.field("level", &self.level);
    }
}

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperIssuingAuthorizationParamsVerificationDataAddressPostalCodeCheck =
    stripe_types_04::test_helpers::CreateTestHelperIssuingAuthorizationParamsVerificationDataAddressLine1Check;

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone)]
pub struct CreateTestHelperTreasuryReceivedCreditParamsInitiatingPaymentMethodDetails {
    #[doc = "The source type."]
    pub r#type: stripe_types_00::test_helpers::CreateTestHelperTreasuryReceivedCreditParamsInitiatingPaymentMethodDetailsType,
    #[doc = "Optional fields for `us_bank_account`."]
    pub us_bank_account: Option<stripe_types_03::test_helpers::CreateTestHelperTreasuryReceivedCreditParamsInitiatingPaymentMethodDetailsUsBankAccount>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperTreasuryReceivedCreditParamsInitiatingPaymentMethodDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
        form.field_opt("us_bank_account", self.us_bank_account.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
pub type CreateTestHelperTreasuryReceivedDebitParamsInitiatingPaymentMethodDetailsUsBankAccount = stripe_types_03::test_helpers::CreateTestHelperTreasuryReceivedCreditParamsInitiatingPaymentMethodDetailsUsBankAccount;

#[cfg(feature = "test_helpers")]
pub type CreateUnlinkedRefundParamsMerchantDataCategory =
    stripe_types_02::test_helpers::CreateTestHelperIssuingAuthorizationParamsMerchantDataCategory;

#[cfg(feature = "test_helpers")]
pub type CreateUnlinkedRefundParamsPurchaseDetailsFleetPurchaseType =
    stripe_types_02::test_helpers::CreateTestHelperIssuingAuthorizationParamsFleetPurchaseType;

#[cfg(feature = "test_helpers")]
pub type CreateUnlinkedRefundParamsPurchaseDetailsFleetReportedBreakdownTax =
    stripe_types_00::test_helpers::CreateTestHelperIssuingAuthorizationParamsFleetReportedBreakdownTax;

#[cfg(feature = "test_helpers")]
pub type CreateUnlinkedRefundParamsPurchaseDetailsFleetServiceType =
    stripe_types_00::test_helpers::CreateTestHelperIssuingAuthorizationParamsFleetServiceType;

#[cfg(feature = "test_helpers")]
pub type CreateUnlinkedRefundParamsPurchaseDetailsFlightSegmentsItem =
    stripe_types_02::test_helpers::CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsFlightSegmentsItem;

#[cfg(feature = "test_helpers")]
pub type CreateUnlinkedRefundParamsPurchaseDetailsFuelUnit =
    stripe_types_03::test_helpers::CreateTestHelperIssuingAuthorizationParamsFuelUnit;

#[cfg(feature = "test_helpers")]
pub type CreateUnlinkedRefundParamsPurchaseDetailsReceiptItem =
    stripe_types_04::test_helpers::CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsReceiptItem;

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct DeletedTestHelpersTestClock {
    #[doc = "Always true for a deleted object"]
    #[json(default)]
    pub deleted: bool,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::test_helpers::TestHelpersTestClockObject,
}

#[cfg(feature = "test_helpers")]
impl crate::PagedItem for DeletedTestHelpersTestClock {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "test_helpers")]
pub type ExpireTestHelperRefundParams = stripe_types_03::test_helpers::ExpireTestHelperIssuingAuthorizationParams;

#[cfg(feature = "test_helpers")]
pub type FailTestHelperTreasuryOutboundPaymentParams =
    stripe_types_03::test_helpers::ExpireTestHelperIssuingAuthorizationParams;

#[cfg(feature = "test_helpers")]
pub type FinalizeAmountParamsFleetCardholderPromptData =
    stripe_types_04::test_helpers::CreateTestHelperIssuingAuthorizationParamsFleetCardholderPromptData;

#[cfg(feature = "test_helpers")]
pub type FinalizeAmountParamsFleetPurchaseType =
    stripe_types_02::test_helpers::CreateTestHelperIssuingAuthorizationParamsFleetPurchaseType;

#[cfg(feature = "test_helpers")]
pub type FinalizeAmountParamsFleetReportedBreakdownFuel =
    stripe_types_03::test_helpers::CreateTestHelperIssuingAuthorizationParamsFleetReportedBreakdownFuel;

#[cfg(feature = "test_helpers")]
pub type FinalizeAmountParamsFleetServiceType =
    stripe_types_00::test_helpers::CreateTestHelperIssuingAuthorizationParamsFleetServiceType;

#[cfg(feature = "test_helpers")]
pub type FinalizeAmountParamsFuelType =
    stripe_types_02::test_helpers::CreateTestHelperIssuingAuthorizationParamsFuelType;

#[cfg(feature = "test_helpers")]
pub type PostTestHelperTreasuryOutboundTransferParams =
    stripe_types_03::test_helpers::ExpireTestHelperIssuingAuthorizationParams;

#[cfg(feature = "test_helpers")]
pub type ReturnTestHelperTreasuryInboundTransferParams =
    stripe_types_03::test_helpers::ExpireTestHelperIssuingAuthorizationParams;

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct ReturnTestHelperTreasuryOutboundPaymentParamsReturnedDetails {
    #[doc = "The return code to be set on the OutboundPayment object."]
    pub code: Option<stripe_types_04::test_helpers::ReturnTestHelperTreasuryOutboundPaymentParamsReturnedDetailsCode>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for ReturnTestHelperTreasuryOutboundPaymentParamsReturnedDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("code", self.code.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
pub type ShipTestHelperIssuingCardShippingParams =
    stripe_types_03::test_helpers::ExpireTestHelperIssuingAuthorizationParams;

#[cfg(feature = "test_helpers")]
pub type SubmitTestHelperIssuingCardShippingParams =
    stripe_types_03::test_helpers::ExpireTestHelperIssuingAuthorizationParams;

#[cfg(feature = "test_helpers")]
pub type SucceedTestHelperTreasuryInboundTransferParams =
    stripe_types_03::test_helpers::ExpireTestHelperIssuingAuthorizationParams;

#[cfg(feature = "test_helpers")]
pub type TimeoutInputCollectionParams = stripe_types_03::test_helpers::ExpireTestHelperIssuingAuthorizationParams;

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone)]
pub struct UpdateTestHelperTreasuryOutboundPaymentParamsTrackingDetails {
    #[doc = "ACH network tracking details."]
    pub ach: Option<stripe_types_00::test_helpers::UpdateTestHelperTreasuryOutboundPaymentParamsTrackingDetailsAch>,
    #[doc = "The US bank account network used to send funds."]
    pub r#type: stripe_types_00::test_helpers::UpdateTestHelperTreasuryOutboundPaymentParamsTrackingDetailsType,
    #[doc = "US domestic wire network tracking details."]
    pub us_domestic_wire: Option<
        stripe_types_03::test_helpers::UpdateTestHelperTreasuryOutboundPaymentParamsTrackingDetailsUsDomesticWire,
    >,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for UpdateTestHelperTreasuryOutboundPaymentParamsTrackingDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ach", self.ach.as_ref());
        form.field("type", &self.r#type);
        form.field_opt("us_domestic_wire", self.us_domestic_wire.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
pub type UpdateTestHelperTreasuryOutboundTransferParamsTrackingDetailsAch =
    stripe_types_00::test_helpers::UpdateTestHelperTreasuryOutboundPaymentParamsTrackingDetailsAch;

#[cfg(feature = "test_helpers")]
pub type UpdateTestHelperTreasuryOutboundTransferParamsTrackingDetailsType =
    stripe_types_00::test_helpers::UpdateTestHelperTreasuryOutboundPaymentParamsTrackingDetailsType;
