#[cfg(any(feature = "confirmation_tokens", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConfirmationTokensResourcePaymentMethodOptionsResourceCardResourceInstallment {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<stripe_types_06::payment_methods::PaymentMethodDetailsCardInstallmentsPlan>,
}
