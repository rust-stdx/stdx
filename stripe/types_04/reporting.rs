#[cfg(feature = "reporting")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateReportingReportRunParamsParametersReportingCategory {
    Advance,
    AdvanceFunding,
    AnticipationRepayment,
    Charge,
    ChargeFailure,
    ClimateOrderPurchase,
    ClimateOrderRefund,
    ConnectCollectionTransfer,
    ConnectReservedFunds,
    Contribution,
    Dispute,
    DisputeReversal,
    Fee,
    FinancingPaydown,
    FinancingPaydownReversal,
    FinancingPayout,
    FinancingPayoutReversal,
    IssuingAuthorizationHold,
    IssuingAuthorizationRelease,
    IssuingDispute,
    IssuingTransaction,
    NetworkCost,
    OtherAdjustment,
    PartialCaptureReversal,
    Payout,
    PayoutReversal,
    PlatformEarning,
    PlatformEarningRefund,
    Refund,
    RefundFailure,
    RiskReservedFunds,
    Tax,
    Topup,
    TopupReversal,
    Transfer,
    TransferReversal,
    UnreconciledCustomerFunds,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "reporting")]
impl CreateReportingReportRunParamsParametersReportingCategory {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateReportingReportRunParamsParametersReportingCategory::Advance => "advance",
            CreateReportingReportRunParamsParametersReportingCategory::AdvanceFunding => "advance_funding",
            CreateReportingReportRunParamsParametersReportingCategory::AnticipationRepayment => {
                "anticipation_repayment"
            }
            CreateReportingReportRunParamsParametersReportingCategory::Charge => "charge",
            CreateReportingReportRunParamsParametersReportingCategory::ChargeFailure => "charge_failure",
            CreateReportingReportRunParamsParametersReportingCategory::ClimateOrderPurchase => "climate_order_purchase",
            CreateReportingReportRunParamsParametersReportingCategory::ClimateOrderRefund => "climate_order_refund",
            CreateReportingReportRunParamsParametersReportingCategory::ConnectCollectionTransfer => {
                "connect_collection_transfer"
            }
            CreateReportingReportRunParamsParametersReportingCategory::ConnectReservedFunds => "connect_reserved_funds",
            CreateReportingReportRunParamsParametersReportingCategory::Contribution => "contribution",
            CreateReportingReportRunParamsParametersReportingCategory::Dispute => "dispute",
            CreateReportingReportRunParamsParametersReportingCategory::DisputeReversal => "dispute_reversal",
            CreateReportingReportRunParamsParametersReportingCategory::Fee => "fee",
            CreateReportingReportRunParamsParametersReportingCategory::FinancingPaydown => "financing_paydown",
            CreateReportingReportRunParamsParametersReportingCategory::FinancingPaydownReversal => {
                "financing_paydown_reversal"
            }
            CreateReportingReportRunParamsParametersReportingCategory::FinancingPayout => "financing_payout",
            CreateReportingReportRunParamsParametersReportingCategory::FinancingPayoutReversal => {
                "financing_payout_reversal"
            }
            CreateReportingReportRunParamsParametersReportingCategory::IssuingAuthorizationHold => {
                "issuing_authorization_hold"
            }
            CreateReportingReportRunParamsParametersReportingCategory::IssuingAuthorizationRelease => {
                "issuing_authorization_release"
            }
            CreateReportingReportRunParamsParametersReportingCategory::IssuingDispute => "issuing_dispute",
            CreateReportingReportRunParamsParametersReportingCategory::IssuingTransaction => "issuing_transaction",
            CreateReportingReportRunParamsParametersReportingCategory::NetworkCost => "network_cost",
            CreateReportingReportRunParamsParametersReportingCategory::OtherAdjustment => "other_adjustment",
            CreateReportingReportRunParamsParametersReportingCategory::PartialCaptureReversal => {
                "partial_capture_reversal"
            }
            CreateReportingReportRunParamsParametersReportingCategory::Payout => "payout",
            CreateReportingReportRunParamsParametersReportingCategory::PayoutReversal => "payout_reversal",
            CreateReportingReportRunParamsParametersReportingCategory::PlatformEarning => "platform_earning",
            CreateReportingReportRunParamsParametersReportingCategory::PlatformEarningRefund => {
                "platform_earning_refund"
            }
            CreateReportingReportRunParamsParametersReportingCategory::Refund => "refund",
            CreateReportingReportRunParamsParametersReportingCategory::RefundFailure => "refund_failure",
            CreateReportingReportRunParamsParametersReportingCategory::RiskReservedFunds => "risk_reserved_funds",
            CreateReportingReportRunParamsParametersReportingCategory::Tax => "tax",
            CreateReportingReportRunParamsParametersReportingCategory::Topup => "topup",
            CreateReportingReportRunParamsParametersReportingCategory::TopupReversal => "topup_reversal",
            CreateReportingReportRunParamsParametersReportingCategory::Transfer => "transfer",
            CreateReportingReportRunParamsParametersReportingCategory::TransferReversal => "transfer_reversal",
            CreateReportingReportRunParamsParametersReportingCategory::UnreconciledCustomerFunds => {
                "unreconciled_customer_funds"
            }
            CreateReportingReportRunParamsParametersReportingCategory::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "reporting")]
impl<'json> json::Decode<'json> for CreateReportingReportRunParamsParametersReportingCategory {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateReportingReportRunParamsParametersReportingCategory::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "advance" => CreateReportingReportRunParamsParametersReportingCategory::Advance,
            "advance_funding" => CreateReportingReportRunParamsParametersReportingCategory::AdvanceFunding,
            "anticipation_repayment" => {
                CreateReportingReportRunParamsParametersReportingCategory::AnticipationRepayment
            }
            "charge" => CreateReportingReportRunParamsParametersReportingCategory::Charge,
            "charge_failure" => CreateReportingReportRunParamsParametersReportingCategory::ChargeFailure,
            "climate_order_purchase" => CreateReportingReportRunParamsParametersReportingCategory::ClimateOrderPurchase,
            "climate_order_refund" => CreateReportingReportRunParamsParametersReportingCategory::ClimateOrderRefund,
            "connect_collection_transfer" => {
                CreateReportingReportRunParamsParametersReportingCategory::ConnectCollectionTransfer
            }
            "connect_reserved_funds" => CreateReportingReportRunParamsParametersReportingCategory::ConnectReservedFunds,
            "contribution" => CreateReportingReportRunParamsParametersReportingCategory::Contribution,
            "dispute" => CreateReportingReportRunParamsParametersReportingCategory::Dispute,
            "dispute_reversal" => CreateReportingReportRunParamsParametersReportingCategory::DisputeReversal,
            "fee" => CreateReportingReportRunParamsParametersReportingCategory::Fee,
            "financing_paydown" => CreateReportingReportRunParamsParametersReportingCategory::FinancingPaydown,
            "financing_paydown_reversal" => {
                CreateReportingReportRunParamsParametersReportingCategory::FinancingPaydownReversal
            }
            "financing_payout" => CreateReportingReportRunParamsParametersReportingCategory::FinancingPayout,
            "financing_payout_reversal" => {
                CreateReportingReportRunParamsParametersReportingCategory::FinancingPayoutReversal
            }
            "issuing_authorization_hold" => {
                CreateReportingReportRunParamsParametersReportingCategory::IssuingAuthorizationHold
            }
            "issuing_authorization_release" => {
                CreateReportingReportRunParamsParametersReportingCategory::IssuingAuthorizationRelease
            }
            "issuing_dispute" => CreateReportingReportRunParamsParametersReportingCategory::IssuingDispute,
            "issuing_transaction" => CreateReportingReportRunParamsParametersReportingCategory::IssuingTransaction,
            "network_cost" => CreateReportingReportRunParamsParametersReportingCategory::NetworkCost,
            "other_adjustment" => CreateReportingReportRunParamsParametersReportingCategory::OtherAdjustment,
            "partial_capture_reversal" => {
                CreateReportingReportRunParamsParametersReportingCategory::PartialCaptureReversal
            }
            "payout" => CreateReportingReportRunParamsParametersReportingCategory::Payout,
            "payout_reversal" => CreateReportingReportRunParamsParametersReportingCategory::PayoutReversal,
            "platform_earning" => CreateReportingReportRunParamsParametersReportingCategory::PlatformEarning,
            "platform_earning_refund" => {
                CreateReportingReportRunParamsParametersReportingCategory::PlatformEarningRefund
            }
            "refund" => CreateReportingReportRunParamsParametersReportingCategory::Refund,
            "refund_failure" => CreateReportingReportRunParamsParametersReportingCategory::RefundFailure,
            "risk_reserved_funds" => CreateReportingReportRunParamsParametersReportingCategory::RiskReservedFunds,
            "tax" => CreateReportingReportRunParamsParametersReportingCategory::Tax,
            "topup" => CreateReportingReportRunParamsParametersReportingCategory::Topup,
            "topup_reversal" => CreateReportingReportRunParamsParametersReportingCategory::TopupReversal,
            "transfer" => CreateReportingReportRunParamsParametersReportingCategory::Transfer,
            "transfer_reversal" => CreateReportingReportRunParamsParametersReportingCategory::TransferReversal,
            "unreconciled_customer_funds" => {
                CreateReportingReportRunParamsParametersReportingCategory::UnreconciledCustomerFunds
            }
            other => CreateReportingReportRunParamsParametersReportingCategory::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "reporting")]
impl crate::params::FormEncode for CreateReportingReportRunParamsParametersReportingCategory {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
