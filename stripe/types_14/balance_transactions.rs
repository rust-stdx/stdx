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
pub struct BalanceTransaction {
    #[doc = "Gross amount of this transaction (in cents (or local equivalent))."]
    #[json(default)]
    pub amount: i64,
    #[doc = "The date that the transaction's net funds become available in the Stripe balance."]
    #[json(default)]
    pub available_on: i64,
    #[doc = "The balance that this transaction impacts."]
    #[json(default)]
    pub balance_type: stripe_types_02::balance_transactions::BalanceTransactionBalanceType,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "An arbitrary string attached to the object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "If applicable, this transaction uses an exchange rate."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub exchange_rate: Option<f64>,
    #[doc = "Fees (in cents (or local equivalent)) paid for this transaction."]
    #[json(default)]
    pub fee: i64,
    #[doc = "Detailed breakdown of fees (in cents (or local equivalent)) paid for this transaction."]
    #[json(default)]
    pub fee_details: Vec<stripe_types_00::common::Fee>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "Net impact to a Stripe balance (in cents (or local equivalent))."]
    #[json(default)]
    pub net: i64,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::balance_transactions::BalanceTransactionObject,
    #[doc = "Learn more about how [reporting categories](https://stripe.com/docs/reports/reporting-categories) can help you unders..."]
    #[json(default)]
    pub reporting_category: String,
    #[doc = "This transaction relates to the Stripe object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<crate::Expandable<crate::balance_transactions::BalanceTransactionSource>>,
    #[doc = "The transaction's net funds status in the Stripe balance, which are either `available` or `pending`."]
    #[json(default)]
    pub status: String,
    #[doc = "Transaction type: `tax_fund`, `adjustment`, `advance`, `advance_funding`, `anticipation_repayment`, `application_fee`..."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::balance_transactions::BalanceTransactionType,
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
impl crate::PagedItem for BalanceTransaction {
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
pub enum BalanceTransactionSource {
    /// Expanded `application_fee`.
    ApplicationFee(Box<crate::application_fees::ApplicationFee>),
    /// Expanded `charge`.
    Charge(Box<crate::charges::Charge>),
    /// Expanded `connect_collection_transfer`.
    ConnectCollectionTransfer(Box<crate::accounts::ConnectCollectionTransfer>),
    /// Expanded `customer_cash_balance_transaction`.
    CustomerCashBalanceTransaction(Box<crate::customers::CustomerCashBalanceTransaction>),
    /// Expanded `dispute`.
    Dispute(Box<crate::disputes::Dispute>),
    /// Expanded `fee_refund`.
    FeeRefund(Box<crate::common::FeeRefund>),
    /// Expanded `issuing.authorization`.
    IssuingAuthorization(Box<crate::issuing::IssuingAuthorization>),
    /// Expanded `issuing.dispute`.
    IssuingDispute(Box<crate::issuing::IssuingDispute>),
    /// Expanded `issuing.transaction`.
    IssuingTransaction(Box<crate::issuing::IssuingTransaction>),
    /// Expanded `payout`.
    Payout(Box<crate::payouts::Payout>),
    /// Expanded `refund`.
    Refund(Box<crate::refunds::Refund>),
    /// Expanded `reserve_transaction`.
    ReserveTransaction(Box<stripe_types_05::treasury::ReserveTransaction>),
    /// Expanded `tax_deducted_at_source`.
    TaxDeductedAtSource(Box<stripe_types_06::tax::TaxDeductedAtSource>),
    /// Expanded `topup`.
    Topup(Box<crate::topups::Topup>),
    /// Expanded `transfer`.
    Transfer(Box<crate::transfers::Transfer>),
    /// Expanded `transfer_reversal`.
    TransferReversal(Box<crate::transfers::TransferReversal>),
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
impl<'json> json::Decode<'json> for BalanceTransactionSource {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "application_fee" => {
                BalanceTransactionSource::ApplicationFee(Box::new(json::from_slice_owned(&json::to_vec(&value))?))
            }
            "charge" => BalanceTransactionSource::Charge(Box::new(json::from_slice_owned(&json::to_vec(&value))?)),
            "connect_collection_transfer" => BalanceTransactionSource::ConnectCollectionTransfer(Box::new(
                json::from_slice_owned(&json::to_vec(&value))?,
            )),
            "customer_cash_balance_transaction" => BalanceTransactionSource::CustomerCashBalanceTransaction(Box::new(
                json::from_slice_owned(&json::to_vec(&value))?,
            )),
            "dispute" => BalanceTransactionSource::Dispute(Box::new(json::from_slice_owned(&json::to_vec(&value))?)),
            "fee_refund" => {
                BalanceTransactionSource::FeeRefund(Box::new(json::from_slice_owned(&json::to_vec(&value))?))
            }
            "issuing.authorization" => {
                BalanceTransactionSource::IssuingAuthorization(Box::new(json::from_slice_owned(&json::to_vec(&value))?))
            }
            "issuing.dispute" => {
                BalanceTransactionSource::IssuingDispute(Box::new(json::from_slice_owned(&json::to_vec(&value))?))
            }
            "issuing.transaction" => {
                BalanceTransactionSource::IssuingTransaction(Box::new(json::from_slice_owned(&json::to_vec(&value))?))
            }
            "payout" => BalanceTransactionSource::Payout(Box::new(json::from_slice_owned(&json::to_vec(&value))?)),
            "refund" => BalanceTransactionSource::Refund(Box::new(json::from_slice_owned(&json::to_vec(&value))?)),
            "reserve_transaction" => {
                BalanceTransactionSource::ReserveTransaction(Box::new(json::from_slice_owned(&json::to_vec(&value))?))
            }
            "tax_deducted_at_source" => {
                BalanceTransactionSource::TaxDeductedAtSource(Box::new(json::from_slice_owned(&json::to_vec(&value))?))
            }
            "topup" => BalanceTransactionSource::Topup(Box::new(json::from_slice_owned(&json::to_vec(&value))?)),
            "transfer" => BalanceTransactionSource::Transfer(Box::new(json::from_slice_owned(&json::to_vec(&value))?)),
            "transfer_reversal" => {
                BalanceTransactionSource::TransferReversal(Box::new(json::from_slice_owned(&json::to_vec(&value))?))
            }
            _ => BalanceTransactionSource::Other(value),
        })
    }
}
