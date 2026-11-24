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
pub struct IssuingAuthorization {
    #[doc = "The total amount that was authorized or rejected."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Detailed breakdown of amount components."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_details: Option<stripe_types_01::issuing::IssuingAuthorizationAmountDetails>,
    #[doc = "Whether the authorization has been approved."]
    #[json(default)]
    pub approved: bool,
    #[doc = "How the card details were provided."]
    #[json(default)]
    pub authorization_method: stripe_types_04::issuing::IssuingAuthorizationAuthorizationMethod,
    #[doc = "List of balance transactions associated with this authorization."]
    #[json(default)]
    pub balance_transactions: Vec<crate::balance_transactions::BalanceTransaction>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types_13::issuing::IssuingCard>,
    #[doc = "Whether the card was present at the point of sale for the authorization."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card_presence: Option<stripe_types_01::issuing::IssuingAuthorizationCardPresence>,
    #[doc = "The cardholder to whom this authorization belongs."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cardholder: Option<crate::Expandable<stripe_types_13::issuing::IssuingCardholder>>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "The currency of the cardholder."]
    #[json(default)]
    pub currency: String,
    #[doc = "Fleet-specific information for authorizations using Fleet cards."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fleet: Option<stripe_types_08::issuing::IssuingAuthorizationFleetData>,
    #[doc = "Fraud challenges sent to the cardholder, if this authorization was declined for fraud risk reasons."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fraud_challenges: Option<Vec<stripe_types_05::issuing::IssuingAuthorizationFraudChallenge>>,
    #[doc = "Information about fuel that was purchased with this transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fuel: Option<stripe_types_06::issuing::IssuingAuthorizationFuelData>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "The total amount that was authorized or rejected."]
    #[json(default)]
    pub merchant_amount: i64,
    #[doc = "The local currency that was presented to the cardholder for the authorization."]
    #[json(default)]
    pub merchant_currency: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub merchant_data: Option<stripe_types_02::issuing::IssuingAuthorizationMerchantData>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "Details about the authorization, such as identifiers, set by the card network."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub network_data: Option<stripe_types_04::issuing::IssuingAuthorizationNetworkData>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::issuing::IssuingAuthorizationObject,
    #[doc = "The pending authorization request."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pending_request: Option<stripe_types_05::issuing::IssuingAuthorizationPendingRequest>,
    #[doc = "History of every time a `pending_request` authorization was approved/declined, either by you directly or by Stripe (e.g."]
    #[json(default)]
    pub request_history: Vec<stripe_types_06::issuing::IssuingAuthorizationRequest>,
    #[doc = "The current status of the authorization in its lifecycle."]
    #[json(default)]
    pub status: stripe_types_03::issuing::IssuingAuthorizationStatus,
    #[doc = "[Token](https://docs.stripe.com/api/issuing/tokens/object) object used for this authorization."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<crate::Expandable<stripe_types_13::issuing::IssuingToken>>,
    #[doc = "List of [transactions](https://docs.stripe.com/api/issuing/transactions) associated with this authorization."]
    #[json(default)]
    pub transactions: Vec<crate::issuing::IssuingTransaction>,
    #[doc = "[Treasury](https://docs.stripe.com/api/treasury) details related to this authorization if it was created on a Financ..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub treasury: Option<stripe_types_02::issuing::IssuingAuthorizationTreasury>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verification_data: Option<stripe_types_07::issuing::IssuingAuthorizationVerificationData>,
    #[doc = "Whether the authorization bypassed fraud risk checks because the cardholder has previously completed a fraud challeng..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verified_by_fraud_challenge: Option<bool>,
    #[doc = "The digital wallet used for this transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
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
impl crate::PagedItem for IssuingAuthorization {
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
pub struct IssuingDispute {
    #[doc = "Disputed amount in the card's currency and in the smallest currency unit..."]
    #[json(default)]
    pub amount: i64,
    #[doc = "List of balance transactions associated with the dispute."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub balance_transactions: Option<Vec<crate::balance_transactions::BalanceTransaction>>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "The currency the `transaction` was made in."]
    #[json(default)]
    pub currency: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub evidence: Option<stripe_types_11::issuing::IssuingDisputeEvidence>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "The enum that describes the dispute loss outcome."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub loss_reason: Option<stripe_types_02::issuing::IssuingDisputeLossReason>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::issuing::IssuingDisputeObject,
    #[doc = "Current status of the dispute."]
    #[json(default)]
    pub status: stripe_types_03::issuing::IssuingDisputeStatus,
    #[doc = "The transaction being disputed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transaction: Option<crate::Expandable<crate::issuing::IssuingTransaction>>,
    #[doc = "[Treasury](https://docs.stripe.com/api/treasury) details related to this dispute if it was created on a FinancialAcc..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub treasury: Option<stripe_types_03::issuing::IssuingDisputeTreasury>,
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
impl crate::PagedItem for IssuingDispute {
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
pub struct IssuingTransaction {
    #[doc = "The transaction amount, which will be reflected in your balance."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Detailed breakdown of amount components."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_details: Option<stripe_types_04::issuing::IssuingTransactionAmountDetails>,
    #[doc = "The `Authorization` object that led to this transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub authorization: Option<crate::Expandable<crate::issuing::IssuingAuthorization>>,
    #[doc = "ID of the [balance transaction](https://docs.stripe.com/api/balance_transactions) associated with this transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<crate::Expandable<crate::balance_transactions::BalanceTransaction>>,
    #[doc = "The card used to make this transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<crate::Expandable<stripe_types_13::issuing::IssuingCard>>,
    #[doc = "The cardholder to whom this transaction belongs."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cardholder: Option<crate::Expandable<stripe_types_13::issuing::IssuingCardholder>>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "If you've disputed the transaction, the ID of the dispute."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub dispute: Option<crate::Expandable<crate::issuing::IssuingDispute>>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "The amount that the merchant will receive, denominated in `merchant_currency` and in the smallest currency unit..."]
    #[json(default)]
    pub merchant_amount: i64,
    #[doc = "The currency with which the merchant is taking payment."]
    #[json(default)]
    pub merchant_currency: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub merchant_data: Option<stripe_types_02::issuing::IssuingAuthorizationMerchantData>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "Details about the transaction, such as processing dates, set by the card network."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub network_data: Option<stripe_types_00::issuing::IssuingTransactionNetworkData>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_01::issuing::IssuingTransactionObject,
    #[doc = "Additional purchase information that is optionally provided by the merchant."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub purchase_details: Option<stripe_types_10::issuing::IssuingTransactionPurchaseDetails>,
    #[doc = "[Token](https://docs.stripe.com/api/issuing/tokens/object) object used for this transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<crate::Expandable<stripe_types_13::issuing::IssuingToken>>,
    #[doc = "[Treasury](https://docs.stripe.com/api/treasury) details related to this transaction if it was created on a Financia..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub treasury: Option<stripe_types_04::issuing::IssuingTransactionTreasury>,
    #[doc = "The nature of the transaction."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::issuing::IssuingTransactionType,
    #[doc = "The digital wallet used for this transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub wallet: Option<stripe_types_00::issuing::IssuingTokenWalletProvider>,
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
impl crate::PagedItem for IssuingTransaction {
    fn id(&self) -> &str {
        &self.id
    }
}
