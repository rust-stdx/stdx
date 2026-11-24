#[cfg(any(feature = "confirmation_tokens", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConfirmationTokensResourcePaymentMethodOptions {
    #[doc = "This hash contains the card payment method options."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types_11::confirmation_tokens::ConfirmationTokensResourcePaymentMethodOptionsResourceCard>,
}
