#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CaptureTestHelperIssuingAuthorizationParamsPurchaseDetails {
    #[doc = "Fleet-specific information for transactions using Fleet cards."]
    pub fleet: Option<stripe_types_10::test_helpers::CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsFleet>,
    #[doc = "Information about the flight that was purchased with this transaction."]
    pub flight: Option<stripe_types_06::test_helpers::CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsFlight>,
    #[doc = "Information about fuel that was purchased with this transaction."]
    pub fuel: Option<stripe_types_08::test_helpers::CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsFuel>,
    #[doc = "Information about lodging that was purchased with this transaction."]
    pub lodging:
        Option<stripe_types_00::test_helpers::CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsLodging>,
    #[doc = "The line items in the purchase."]
    pub receipt: Option<
        Vec<stripe_types_04::test_helpers::CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsReceiptItem>,
    >,
    #[doc = "A merchant-specific order number."]
    pub reference: Option<String>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CaptureTestHelperIssuingAuthorizationParamsPurchaseDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("fleet", self.fleet.as_ref());
        form.field_opt("flight", self.flight.as_ref());
        form.field_opt("fuel", self.fuel.as_ref());
        form.field_opt("lodging", self.lodging.as_ref());
        if let Some(values) = self.receipt.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("receipt", index, item);
            }
        }
        form.field_opt("reference", self.reference.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateForceCaptureParamsPurchaseDetails {
    #[doc = "Fleet-specific information for transactions using Fleet cards."]
    pub fleet: Option<stripe_types_11::test_helpers::CreateForceCaptureParamsPurchaseDetailsFleet>,
    #[doc = "Information about the flight that was purchased with this transaction."]
    pub flight: Option<stripe_types_07::test_helpers::CreateForceCaptureParamsPurchaseDetailsFlight>,
    #[doc = "Information about fuel that was purchased with this transaction."]
    pub fuel: Option<stripe_types_07::test_helpers::CreateForceCaptureParamsPurchaseDetailsFuel>,
    #[doc = "Information about lodging that was purchased with this transaction."]
    pub lodging: Option<stripe_types_05::test_helpers::CreateForceCaptureParamsPurchaseDetailsLodging>,
    #[doc = "The line items in the purchase."]
    pub receipt: Option<Vec<stripe_types_05::test_helpers::CreateForceCaptureParamsPurchaseDetailsReceiptItem>>,
    #[doc = "A merchant-specific order number."]
    pub reference: Option<String>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateForceCaptureParamsPurchaseDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("fleet", self.fleet.as_ref());
        form.field_opt("flight", self.flight.as_ref());
        form.field_opt("fuel", self.fuel.as_ref());
        form.field_opt("lodging", self.lodging.as_ref());
        if let Some(values) = self.receipt.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("receipt", index, item);
            }
        }
        form.field_opt("reference", self.reference.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperConfirmationTokenParamsPaymentMethodOptions {
    #[doc = "Configuration for any card payments confirmed using this ConfirmationToken."]
    pub card: Option<stripe_types_10::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodOptionsCard>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("card", self.card.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperIssuingAuthorizationParams {
    #[doc = "The total amount to attempt to authorize."]
    pub amount: Option<i64>,
    #[doc = "Detailed breakdown of amount components."]
    pub amount_details: Option<stripe_types_03::test_helpers::CreateTestHelperIssuingAuthorizationParamsAmountDetails>,
    #[doc = "How the card details were provided."]
    pub authorization_method:
        Option<stripe_types_02::test_helpers::CreateTestHelperIssuingAuthorizationParamsAuthorizationMethod>,
    #[doc = "Card associated with this authorization."]
    pub card: String,
    #[doc = "The currency of the authorization."]
    pub currency: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Fleet-specific information for authorizations using Fleet cards."]
    pub fleet: Option<stripe_types_10::test_helpers::CreateTestHelperIssuingAuthorizationParamsFleet>,
    #[doc = "Probability that this transaction can be disputed in the event of fraud."]
    pub fraud_disputability_likelihood:
        Option<stripe_types_00::test_helpers::CreateTestHelperIssuingAuthorizationParamsFraudDisputabilityLikelihood>,
    #[doc = "Information about fuel that was purchased with this transaction."]
    pub fuel: Option<stripe_types_06::test_helpers::CreateTestHelperIssuingAuthorizationParamsFuel>,
    #[doc = "If set `true`, you may provide amount..."]
    pub is_amount_controllable: Option<bool>,
    #[doc = "The total amount to attempt to authorize."]
    pub merchant_amount: Option<i64>,
    #[doc = "The currency of the authorization."]
    pub merchant_currency: Option<String>,
    #[doc = "Details about the seller (grocery store, e-commerce website, etc.) where the card authorization happened."]
    pub merchant_data: Option<stripe_types_05::test_helpers::CreateTestHelperIssuingAuthorizationParamsMerchantData>,
    #[doc = "Details about the authorization, such as identifiers, set by the card network."]
    pub network_data: Option<stripe_types_02::test_helpers::CreateTestHelperIssuingAuthorizationParamsNetworkData>,
    #[doc = "Stripe’s assessment of the fraud risk for this authorization."]
    pub risk_assessment:
        Option<stripe_types_11::test_helpers::CreateTestHelperIssuingAuthorizationParamsRiskAssessment>,
    #[doc = "Verifications that Stripe performed on information that the cardholder provided to the merchant."]
    pub verification_data:
        Option<stripe_types_07::test_helpers::CreateTestHelperIssuingAuthorizationParamsVerificationData>,
    #[doc = "The digital wallet used for this transaction."]
    pub wallet: Option<stripe_types_04::test_helpers::CreateTestHelperIssuingAuthorizationParamsWallet>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperIssuingAuthorizationParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("amount_details", self.amount_details.as_ref());
        form.field_opt("authorization_method", self.authorization_method.as_ref());
        form.field("card", &self.card);
        form.field_opt("currency", self.currency.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("fleet", self.fleet.as_ref());
        form.field_opt("fraud_disputability_likelihood", self.fraud_disputability_likelihood.as_ref());
        form.field_opt("fuel", self.fuel.as_ref());
        form.field_opt("is_amount_controllable", self.is_amount_controllable.as_ref());
        form.field_opt("merchant_amount", self.merchant_amount.as_ref());
        form.field_opt("merchant_currency", self.merchant_currency.as_ref());
        form.field_opt("merchant_data", self.merchant_data.as_ref());
        form.field_opt("network_data", self.network_data.as_ref());
        form.field_opt("risk_assessment", self.risk_assessment.as_ref());
        form.field_opt("verification_data", self.verification_data.as_ref());
        form.field_opt("wallet", self.wallet.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateUnlinkedRefundParamsPurchaseDetails {
    #[doc = "Fleet-specific information for transactions using Fleet cards."]
    pub fleet: Option<stripe_types_10::test_helpers::CreateUnlinkedRefundParamsPurchaseDetailsFleet>,
    #[doc = "Information about the flight that was purchased with this transaction."]
    pub flight: Option<stripe_types_08::test_helpers::CreateUnlinkedRefundParamsPurchaseDetailsFlight>,
    #[doc = "Information about fuel that was purchased with this transaction."]
    pub fuel: Option<stripe_types_09::test_helpers::CreateUnlinkedRefundParamsPurchaseDetailsFuel>,
    #[doc = "Information about lodging that was purchased with this transaction."]
    pub lodging: Option<stripe_types_05::test_helpers::CreateUnlinkedRefundParamsPurchaseDetailsLodging>,
    #[doc = "The line items in the purchase."]
    pub receipt: Option<Vec<stripe_types_06::test_helpers::CreateUnlinkedRefundParamsPurchaseDetailsReceiptItem>>,
    #[doc = "A merchant-specific order number."]
    pub reference: Option<String>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateUnlinkedRefundParamsPurchaseDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("fleet", self.fleet.as_ref());
        form.field_opt("flight", self.flight.as_ref());
        form.field_opt("fuel", self.fuel.as_ref());
        form.field_opt("lodging", self.lodging.as_ref());
        if let Some(values) = self.receipt.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("receipt", index, item);
            }
        }
        form.field_opt("reference", self.reference.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct FinalizeAmountParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The final authorization amount that will be captured by the merchant."]
    pub final_amount: i64,
    #[doc = "Fleet-specific information for authorizations using Fleet cards."]
    pub fleet: Option<stripe_types_11::test_helpers::FinalizeAmountParamsFleet>,
    #[doc = "Information about fuel that was purchased with this transaction."]
    pub fuel: Option<stripe_types_09::test_helpers::FinalizeAmountParamsFuel>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for FinalizeAmountParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("final_amount", &self.final_amount);
        form.field_opt("fleet", self.fleet.as_ref());
        form.field_opt("fuel", self.fuel.as_ref());
    }
}
