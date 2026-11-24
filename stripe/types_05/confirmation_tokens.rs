#[cfg(any(feature = "confirmation_tokens", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConfirmationTokensResourceMandateDataResourceCustomerAcceptance {
    #[doc = "If this is a Mandate accepted online, this hash contains details about the online acceptance."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub online: Option<stripe_types_04::confirmation_tokens::ConfirmationTokensResourceMandateDataResourceCustomerAcceptanceResourceOnline>,
    #[doc = "The type of customer acceptance information included with the Mandate."]
    #[json(rename = "type", default)]
    pub r#type: String,
}

#[cfg(any(feature = "confirmation_tokens", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConfirmationTokensResourceShipping {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_01::common::Address>,
    #[doc = "Recipient name."]
    #[json(default)]
    pub name: String,
    #[doc = "Recipient phone (including extension)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}
