#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateForceCaptureParamsPurchaseDetailsFleetReportedBreakdown {
    #[doc = "Breakdown of fuel portion of the purchase."]
    pub fuel: Option<stripe_types_06::test_helpers::CreateForceCaptureParamsPurchaseDetailsFleetReportedBreakdownFuel>,
    #[doc = "Breakdown of non-fuel portion of the purchase."]
    pub non_fuel:
        Option<stripe_types_06::test_helpers::CreateForceCaptureParamsPurchaseDetailsFleetReportedBreakdownNonFuel>,
    #[doc = "Information about tax included in this transaction."]
    pub tax: Option<stripe_types_06::test_helpers::CreateForceCaptureParamsPurchaseDetailsFleetReportedBreakdownTax>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateForceCaptureParamsPurchaseDetailsFleetReportedBreakdown {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("fuel", self.fuel.as_ref());
        form.field_opt("non_fuel", self.non_fuel.as_ref());
        form.field_opt("tax", self.tax.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateForceCaptureParamsPurchaseDetailsFlight {
    #[doc = "The time that the flight departed."]
    pub departure_at: Option<i64>,
    #[doc = "The name of the passenger."]
    pub passenger_name: Option<String>,
    #[doc = "Whether the ticket is refundable."]
    pub refundable: Option<bool>,
    #[doc = "The legs of the trip."]
    pub segments: Option<Vec<stripe_types_05::test_helpers::CreateForceCaptureParamsPurchaseDetailsFlightSegmentsItem>>,
    #[doc = "The travel agency that issued the ticket."]
    pub travel_agency: Option<String>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateForceCaptureParamsPurchaseDetailsFlight {
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
#[derive(Debug, Clone, Default)]
pub struct CreateForceCaptureParamsPurchaseDetailsFuel {
    #[doc = "[Conexxus Payment System Product Code](https://www.conexxus.org/conexxus-payment-system-product-codes) identifying th..."]
    pub industry_product_code: Option<String>,
    #[doc = "The quantity of `unit`s of fuel that was dispensed, represented as a decimal string with at most 12 decimal places."]
    pub quantity_decimal: Option<String>,
    #[doc = "The type of fuel that was purchased."]
    pub r#type: Option<stripe_types_06::test_helpers::CreateForceCaptureParamsPurchaseDetailsFuelType>,
    #[doc = "The units for `quantity_decimal`."]
    pub unit: Option<stripe_types_05::test_helpers::CreateForceCaptureParamsPurchaseDetailsFuelUnit>,
    #[doc = "The cost in cents per each unit of fuel, represented as a decimal string with at most 12 decimal places."]
    pub unit_cost_decimal: Option<String>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateForceCaptureParamsPurchaseDetailsFuel {
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
pub struct CreateTestHelperConfirmationTokenParamsPaymentMethodDataUsBankAccount {
    #[doc = "Account holder type: individual or company."]
    pub account_holder_type: Option<stripe_types_05::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataUsBankAccountAccountHolderType>,
    #[doc = "Account number of the bank account."]
    pub account_number: Option<String>,
    #[doc = "Account type: checkings or savings."]
    pub account_type: Option<stripe_types_02::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataUsBankAccountAccountType>,
    #[doc = "The ID of a Financial Connections Account to use as a payment method."]
    pub financial_connections_account: Option<String>,
    #[doc = "Routing number of the bank account."]
    pub routing_number: Option<String>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodDataUsBankAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account_holder_type", self.account_holder_type.as_ref());
        form.field_opt("account_number", self.account_number.as_ref());
        form.field_opt("account_type", self.account_type.as_ref());
        form.field_opt("financial_connections_account", self.financial_connections_account.as_ref());
        form.field_opt("routing_number", self.routing_number.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone)]
pub struct CreateTestHelperConfirmationTokenParamsPaymentMethodOptionsCardInstallments {
    #[doc = "The selected installment plan to use for this payment attempt."]
    pub plan:
        stripe_types_06::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodOptionsCardInstallmentsPlan,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodOptionsCardInstallments {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("plan", &self.plan);
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone)]
pub struct CreateTestHelperConfirmationTokenParamsShipping {
    #[doc = "Shipping address"]
    pub address: stripe_types_05::test_helpers::CreateTestHelperConfirmationTokenParamsShippingAddress,
    #[doc = "Recipient name."]
    pub name: String,
    #[doc = "Recipient phone (including extension)"]
    pub phone: Option<json::OwnedValue>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsShipping {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("address", &self.address);
        form.field("name", &self.name);
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperIssuingAuthorizationParamsVerificationData {
    #[doc = "Whether the cardholder provided an address first line and if it matched the cardholder’s `billing.address.line1`."]
    pub address_line1_check: Option<stripe_types_04::test_helpers::CreateTestHelperIssuingAuthorizationParamsVerificationDataAddressLine1Check>,
    #[doc = "Whether the cardholder provided a postal code and if it matched the cardholder’s `billing.address.postal_code`."]
    pub address_postal_code_check: Option<stripe_types_06::test_helpers::CreateTestHelperIssuingAuthorizationParamsVerificationDataAddressPostalCodeCheck>,
    #[doc = "The exemption applied to this authorization."]
    pub authentication_exemption: Option<stripe_types_05::test_helpers::CreateTestHelperIssuingAuthorizationParamsVerificationDataAuthenticationExemption>,
    #[doc = "Whether the cardholder provided a CVC and if it matched Stripe’s record."]
    pub cvc_check: Option<stripe_types_05::test_helpers::CreateTestHelperIssuingAuthorizationParamsVerificationDataCvcCheck>,
    #[doc = "Whether the cardholder provided an expiry date and if it matched Stripe’s record."]
    pub expiry_check: Option<stripe_types_05::test_helpers::CreateTestHelperIssuingAuthorizationParamsVerificationDataExpiryCheck>,
    #[doc = "3D Secure details."]
    pub three_d_secure: Option<stripe_types_05::test_helpers::CreateTestHelperIssuingAuthorizationParamsVerificationDataThreeDSecure>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperIssuingAuthorizationParamsVerificationData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("address_line1_check", self.address_line1_check.as_ref());
        form.field_opt("address_postal_code_check", self.address_postal_code_check.as_ref());
        form.field_opt("authentication_exemption", self.authentication_exemption.as_ref());
        form.field_opt("cvc_check", self.cvc_check.as_ref());
        form.field_opt("expiry_check", self.expiry_check.as_ref());
        form.field_opt("three_d_secure", self.three_d_secure.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone)]
pub struct CreateTestHelperTreasuryReceivedCreditParams {
    #[doc = "Amount (in cents) to be transferred."]
    pub amount: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "An arbitrary string attached to the object."]
    pub description: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The FinancialAccount to send funds to."]
    pub financial_account: String,
    #[doc = "Initiating payment method details for the object."]
    pub initiating_payment_method_details: Option<
        stripe_types_06::test_helpers::CreateTestHelperTreasuryReceivedCreditParamsInitiatingPaymentMethodDetails,
    >,
    #[doc = "Specifies the network rails to be used."]
    pub network: stripe_types_00::test_helpers::CreateTestHelperTreasuryReceivedCreditParamsNetwork,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperTreasuryReceivedCreditParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        form.field("currency", &self.currency);
        form.field_opt("description", self.description.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("financial_account", &self.financial_account);
        form.field_opt(
            "initiating_payment_method_details",
            self.initiating_payment_method_details.as_ref(),
        );
        form.field("network", &self.network);
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct FinalizeAmountParamsFleetReportedBreakdown {
    #[doc = "Breakdown of fuel portion of the purchase."]
    pub fuel: Option<stripe_types_06::test_helpers::FinalizeAmountParamsFleetReportedBreakdownFuel>,
    #[doc = "Breakdown of non-fuel portion of the purchase."]
    pub non_fuel: Option<stripe_types_05::test_helpers::FinalizeAmountParamsFleetReportedBreakdownNonFuel>,
    #[doc = "Information about tax included in this transaction."]
    pub tax: Option<stripe_types_05::test_helpers::FinalizeAmountParamsFleetReportedBreakdownTax>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for FinalizeAmountParamsFleetReportedBreakdown {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("fuel", self.fuel.as_ref());
        form.field_opt("non_fuel", self.non_fuel.as_ref());
        form.field_opt("tax", self.tax.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone)]
pub struct UpdateTestHelperTreasuryOutboundTransferParamsTrackingDetails {
    #[doc = "ACH network tracking details."]
    pub ach: Option<stripe_types_06::test_helpers::UpdateTestHelperTreasuryOutboundTransferParamsTrackingDetailsAch>,
    #[doc = "The US bank account network used to send funds."]
    pub r#type: stripe_types_06::test_helpers::UpdateTestHelperTreasuryOutboundTransferParamsTrackingDetailsType,
    #[doc = "US domestic wire network tracking details."]
    pub us_domestic_wire: Option<
        stripe_types_05::test_helpers::UpdateTestHelperTreasuryOutboundTransferParamsTrackingDetailsUsDomesticWire,
    >,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for UpdateTestHelperTreasuryOutboundTransferParamsTrackingDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ach", self.ach.as_ref());
        form.field("type", &self.r#type);
        form.field_opt("us_domestic_wire", self.us_domestic_wire.as_ref());
    }
}
