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
pub struct Payout {
    #[doc = "The amount (in cents (or local equivalent)) that transfers to your bank account or debit card."]
    #[json(default)]
    pub amount: i64,
    #[doc = "The application fee (if any) for the payout."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub application_fee: Option<crate::Expandable<crate::application_fees::ApplicationFee>>,
    #[doc = "The amount of the application fee (if any) requested for the payout."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    #[doc = "Date that you can expect the payout to arrive in the bank."]
    #[json(default)]
    pub arrival_date: i64,
    #[doc = "Returns `true` if the payout is created by an automated payout schedule..."]
    #[json(default)]
    pub automatic: bool,
    #[doc = "ID of the balance transaction that describes the impact of this payout on your account balance."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<crate::Expandable<crate::balance_transactions::BalanceTransaction>>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "An arbitrary string attached to the object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "ID of the bank account or card the payout is sent to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<crate::Expandable<crate::payouts::PayoutDestinationExpanded>>,
    #[doc = "If the payout fails or cancels, this is the ID of the balance transaction that reverses the initial balance transacti..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub failure_balance_transaction: Option<crate::Expandable<crate::balance_transactions::BalanceTransaction>>,
    #[doc = "Error code that provides a reason for a payout failure, if available."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[doc = "Message that provides the reason for a payout failure, if available."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The method used to send this payout, which can be `standard` or `instant`."]
    #[json(default)]
    pub method: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::payouts::PayoutObject,
    #[doc = "If the payout reverses another, this is the ID of the original payout."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub original_payout: Option<crate::Expandable<crate::payouts::Payout>>,
    #[doc = "ID of the v2 FinancialAccount the funds are sent to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payout_method: Option<String>,
    #[doc = "If `completed`, you can use the Balance Transactions API..."]
    #[json(default)]
    pub reconciliation_status: stripe_types_00::payouts::PayoutReconciliationStatus,
    #[doc = "If the payout reverses, this is the ID of the payout that reverses this payout."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reversed_by: Option<crate::Expandable<crate::payouts::Payout>>,
    #[doc = "The source balance this payout came from, which can be one of the following: `card`, `fpx`, or `bank_account`."]
    #[json(default)]
    pub source_type: String,
    #[doc = "Extra information about a payout that displays on the user's bank statement."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    #[doc = "Current status of the payout: `paid`, `pending`, `in_transit`, `canceled` or `failed`."]
    #[json(default)]
    pub status: String,
    #[doc = "A value that generates from the beneficiary's bank that allows users to track payouts with their bank."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<stripe_types_02::payouts::PayoutsTraceId>,
    #[doc = "Can be `bank_account` or `card`."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::payouts::PayoutType,
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
impl crate::PagedItem for Payout {
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
#[derive(Debug, Clone)]
pub enum PayoutDestinationExpanded {
    /// Expanded `external_account`.
    ExternalAccount(Box<crate::accounts::ExternalAccount>),
    /// Expanded `deleted_external_account`.
    DeletedExternalAccount(Box<stripe_types_09::accounts::DeletedExternalAccount>),
    /// An object type not known to this version of the crate.
    Other(json::OwnedValue),
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
impl<'json> json::Decode<'json> for PayoutDestinationExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "external_account" => {
                PayoutDestinationExpanded::ExternalAccount(Box::new(json::from_slice_owned(&json::to_vec(&value))?))
            }
            "deleted_external_account" => PayoutDestinationExpanded::DeletedExternalAccount(Box::new(
                json::from_slice_owned(&json::to_vec(&value))?,
            )),
            _ => PayoutDestinationExpanded::Other(value),
        })
    }
}
