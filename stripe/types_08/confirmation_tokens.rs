#[cfg(any(feature = "confirmation_tokens", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConfirmationTokensResourceMandateData {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_acceptance:
        Option<stripe_types_05::confirmation_tokens::ConfirmationTokensResourceMandateDataResourceCustomerAcceptance>,
}
