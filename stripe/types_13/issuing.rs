#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IssuingCard {
    #[doc = "The brand of the card."]
    #[json(default)]
    pub brand: String,
    #[doc = "The reason why the card was canceled."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<stripe_types_00::issuing::IssuingCardCancellationReason>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cardholder: Option<crate::issuing::IssuingCardholder>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "The card's CVC."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cvc: Option<String>,
    #[doc = "The expiration month of the card."]
    #[json(default)]
    pub exp_month: i64,
    #[doc = "The expiration year of the card."]
    #[json(default)]
    pub exp_year: i64,
    #[doc = "The financial account this card is attached to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<String>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "The last 4 digits of the card number."]
    #[json(default)]
    pub last4: String,
    #[doc = "Stripe’s assessment of whether this card’s details have been compromised."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub latest_fraud_warning: Option<stripe_types_05::issuing::IssuingCardFraudWarning>,
    #[doc = "Rules that control the lifecycle of this card, such as automatic cancellation."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_controls: Option<stripe_types_05::issuing::IssuingCardLifecycleControls>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "The full unredacted card number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_01::issuing::IssuingCardObject,
    #[doc = "The personalization design object belonging to this card."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub personalization_design: Option<crate::Expandable<stripe_types_10::issuing::IssuingPersonalizationDesign>>,
    #[doc = "The latest card that replaces this card, if any."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub replaced_by: Option<crate::Expandable<crate::issuing::IssuingCard>>,
    #[doc = "The card this card replaces, if any."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub replacement_for: Option<crate::Expandable<crate::issuing::IssuingCard>>,
    #[doc = "The reason why the previous card needed to be replaced."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub replacement_reason: Option<stripe_types_02::issuing::IssuingCardReplacementReason>,
    #[doc = "Text separate from cardholder name, printed on the card."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub second_line: Option<String>,
    #[doc = "Where and how the card will be shipped."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping: Option<stripe_types_09::issuing::IssuingCardShipping>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub spending_controls: Option<stripe_types_07::issuing::IssuingCardAuthorizationControls>,
    #[doc = "Whether authorizations can be approved on this card."]
    #[json(default)]
    pub status: stripe_types_04::issuing::IssuingCardStatus,
    #[doc = "The type of the card."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::issuing::IssuingCardType,
    #[doc = "Information relating to digital wallets (like Apple Pay and Google Pay)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub wallets: Option<stripe_types_07::issuing::IssuingCardWallets>,
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl crate::PagedItem for IssuingCard {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IssuingCardholder {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billing: Option<stripe_types_05::issuing::IssuingCardholderAddress>,
    #[doc = "Additional information about a `company` cardholder."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub company: Option<stripe_types_00::issuing::IssuingCardholderCompany>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "The cardholder's email address."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "Additional information about an `individual` cardholder."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub individual: Option<stripe_types_12::issuing::IssuingCardholderIndividual>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "The cardholder's name."]
    #[json(default)]
    pub name: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::issuing::IssuingCardholderObject,
    #[doc = "The cardholder's phone number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[doc = "The cardholder’s preferred locales (languages), ordered by preference."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub preferred_locales: Option<Vec<stripe_types_03::issuing::IssuingCardholderPreferredLocalesItem>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub requirements: Option<stripe_types_05::issuing::IssuingCardholderRequirements>,
    #[doc = "Rules that control spending across this cardholder's cards."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub spending_controls: Option<stripe_types_07::issuing::IssuingCardholderAuthorizationControls>,
    #[doc = "Specifies whether to permit authorizations on this cardholder's cards."]
    #[json(default)]
    pub status: stripe_types_04::issuing::IssuingCardholderStatus,
    #[doc = "One of `individual` or `company`."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::issuing::IssuingCardholderType,
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl crate::PagedItem for IssuingCardholder {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IssuingToken {
    #[doc = "Card associated with this token."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<crate::Expandable<crate::issuing::IssuingCard>>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "The hashed ID derived from the device ID from the card network associated with the token."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub device_fingerprint: Option<String>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "The last four digits of the token."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "The token service provider / card network associated with the token."]
    #[json(default)]
    pub network: stripe_types_00::issuing::IssuingTokenNetwork,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub network_data: Option<stripe_types_09::issuing::IssuingNetworkTokenNetworkData>,
    #[doc = "Time at which the token was last updated by the card network."]
    #[json(default)]
    pub network_updated_at: i64,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_01::issuing::IssuingTokenObject,
    #[doc = "The usage state of the token."]
    #[json(default)]
    pub status: stripe_types_04::issuing::IssuingTokenStatus,
    #[doc = "The digital wallet for this token, if one was used."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub wallet_provider: Option<stripe_types_00::issuing::IssuingTokenWalletProvider>,
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl crate::PagedItem for IssuingToken {
    fn id(&self) -> &str {
        &self.id
    }
}
