#[cfg(any(feature = "confirmation_tokens", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConfirmationTokensResourcePaymentMethodOptionsResourceCard {
    #[doc = "The `cvc_update` Token collected from the Payment Element."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cvc_token: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub installments: Option<stripe_types_09::confirmation_tokens::ConfirmationTokensResourcePaymentMethodOptionsResourceCardResourceInstallment>,
}
