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
pub struct Customer {
    #[doc = "The customer's billing address."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_01::common::Address>,
    #[doc = "The current balance, if any, that's stored on the customer in their default currency."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub balance: Option<i64>,
    #[doc = "The customer's business name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    #[doc = "The current funds being held by Stripe on behalf of the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cash_balance: Option<stripe_types_08::customers::CashBalance>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) the customer can be charged in for recur..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "The ID of an Account representing a customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_account: Option<String>,
    #[doc = "ID of the default payment source for the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub default_source: Option<crate::Expandable<crate::common::PaymentSource>>,
    #[doc = "Tracks the most recent state change on any invoice belonging to the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub delinquent: Option<bool>,
    #[doc = "An arbitrary string attached to the object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Describes the current discount active on the customer, if there is one."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub discount: Option<crate::coupons::Discount>,
    #[doc = "The customer's email address."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "The customer's individual name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub individual_name: Option<String>,
    #[doc = "The current multi-currency balances, if any, that's stored on the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub invoice_credit_balance: Option<std::collections::HashMap<String, i64>>,
    #[doc = "The prefix for the customer used to generate unique invoice numbers."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub invoice_prefix: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<crate::invoices::InvoiceSettingCustomerSetting>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The customer's full name or business name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The suffix of the customer's next invoice number (for example, 0001)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub next_invoice_sequence: Option<i64>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_02::customers::CustomerObject,
    #[doc = "The customer's phone number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "The customer's preferred locales (languages), ordered by preference."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub preferred_locales: Option<Vec<String>>,
    #[doc = "Mailing and shipping address for the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping: Option<stripe_types_05::common::Shipping>,
    #[doc = "The customer's payment sources, if any."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sources: Option<crate::List<crate::common::PaymentSource>>,
    #[doc = "The customer's current subscriptions, if any."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<crate::List<crate::subscriptions::Subscription>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax: Option<stripe_types_08::customers::CustomerTax>,
    #[doc = "Describes the customer's tax exemption status, which is `none`, `exempt`, or `reverse`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<stripe_types_02::customers::CustomerTaxExempt>,
    #[doc = "The customer's tax IDs."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax_ids: Option<crate::List<crate::tax_ids::TaxId>>,
    #[doc = "ID of the test clock that this customer belongs to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<crate::Expandable<stripe_types_08::test_helpers::TestHelpersTestClock>>,
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
impl crate::PagedItem for Customer {
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
pub struct CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft {
    #[doc = "The [Balance Transaction](https://docs.stripe.com/api/balance_transactions/object) that corresponds to funds taken ou..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<crate::Expandable<crate::balance_transactions::BalanceTransaction>>,
    #[doc = "The [Cash Balance Transaction](https://docs.stripe.com/api/cash_balance_transactions/object) that brought the custome..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub linked_transaction: Option<crate::Expandable<crate::customers::CustomerCashBalanceTransaction>>,
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
pub struct CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction {
    #[doc = "The [Payment Intent](https://docs.stripe.com/api/payment_intents/object) that funds were applied to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<crate::Expandable<crate::payment_intents::PaymentIntent>>,
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
pub struct CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction {
    #[doc = "The [Refund](https://docs.stripe.com/api/refunds/object) that moved these funds into the customer's cash balance."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refund: Option<crate::Expandable<crate::refunds::Refund>>,
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
pub struct CustomerBalanceResourceCashBalanceTransactionResourceTransferredToBalance {
    #[doc = "The [Balance Transaction](https://docs.stripe.com/api/balance_transactions/object) that corresponds to funds transfer..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<crate::Expandable<crate::balance_transactions::BalanceTransaction>>,
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
pub struct CustomerBalanceResourceCashBalanceTransactionResourceUnappliedFromPaymentTransaction {
    #[doc = "The [Payment Intent](https://docs.stripe.com/api/payment_intents/object) that funds were unapplied from."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<crate::Expandable<crate::payment_intents::PaymentIntent>>,
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
pub struct CustomerCashBalanceTransaction {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub adjusted_for_overdraft:
        Option<crate::customers::CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub applied_to_payment:
        Option<crate::customers::CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "The customer whose available cash balance changed as a result of this transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<crate::Expandable<crate::customers::Customer>>,
    #[doc = "The ID of an Account representing a customer whose available cash balance changed as a result of this transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_account: Option<String>,
    #[doc = "The total available cash balance for the specified currency after this transaction was applied."]
    #[json(default)]
    pub ending_balance: i64,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub funded:
        Option<stripe_types_10::customers::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "The amount by which the cash balance changed, represented in the smallest currency unit..."]
    #[json(default)]
    pub net_amount: i64,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::customers::CustomerCashBalanceTransactionObject,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refunded_from_payment:
        Option<crate::customers::CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transferred_to_balance:
        Option<crate::customers::CustomerBalanceResourceCashBalanceTransactionResourceTransferredToBalance>,
    #[doc = "The type of the cash balance transaction."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::customers::CustomerCashBalanceTransactionType,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub unapplied_from_payment:
        Option<crate::customers::CustomerBalanceResourceCashBalanceTransactionResourceUnappliedFromPaymentTransaction>,
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
impl crate::PagedItem for CustomerCashBalanceTransaction {
    fn id(&self) -> &str {
        &self.id
    }
}
