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
pub struct ApiErrors {
    #[doc = "For card errors resulting from a card issuer decline, a short string indicating how to proceed with an error..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub advice_code: Option<String>,
    #[doc = "For card errors, the ID of the failed charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub charge: Option<String>,
    #[doc = "For some errors that could be handled programmatically, a short string indicating the error code..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<stripe_types_00::common::ApiErrorsCode>,
    #[doc = "For card errors resulting from a card issuer decline, a short string indicating the card issuer's reason for the dec..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub decline_code: Option<String>,
    #[doc = "A URL to more information about the [error code](https://docs.stripe.com/error-codes) reported."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub doc_url: Option<String>,
    #[doc = "A human-readable message providing more details about the error."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "For card errors resulting from a card issuer decline, a 2 digit code which indicates the advice given to merchant by ..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub network_advice_code: Option<String>,
    #[doc = "For payments declined by the network, an alphanumeric code which indicates the reason the payment failed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub network_decline_code: Option<String>,
    #[doc = "If the error is parameter-specific, the parameter related to the error."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<Box<crate::payment_intents::PaymentIntent>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<crate::payment_methods::PaymentMethod>,
    #[doc = "If the error is specific to the type of payment method, the payment method type that had a problem."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<String>,
    #[doc = "A URL to the request log entry in your dashboard."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub request_log_url: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_intent: Option<Box<crate::setup_intents::SetupIntent>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<crate::common::PaymentSource>,
    #[doc = "The type of error returned."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::common::ApiErrorsType,
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
pub struct AutomaticTax {
    #[doc = "If Stripe disabled automatic tax, this enum describes why."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<stripe_types_02::common::AutomaticTaxDisabledReason>,
    #[doc = "Whether Stripe automatically computes tax on this invoice."]
    #[json(default)]
    pub enabled: bool,
    #[doc = "The account that's liable for tax."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub liability: Option<crate::accounts::ConnectAccountReference>,
    #[doc = "The tax provider powering automatic tax."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "The status of the most recent automated tax calculation for this invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<stripe_types_02::common::AutomaticTaxStatus>,
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
pub struct DiscountsResourceDiscountAmount {
    #[doc = "The amount, in cents (or local equivalent), of the discount."]
    #[json(default)]
    pub amount: i64,
    #[doc = "The discount that was applied to get this discount amount."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub discount: Option<crate::Expandable<crate::common::DiscountsResourceDiscountAmountDiscountExpanded>>,
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
pub enum DiscountsResourceDiscountAmountDiscountExpanded {
    /// Expanded `discount`.
    Discount(Box<crate::coupons::Discount>),
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
impl<'json> json::Decode<'json> for DiscountsResourceDiscountAmountDiscountExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "discount" => DiscountsResourceDiscountAmountDiscountExpanded::Discount(Box::new(json::from_slice_owned(
                &json::to_vec(&value),
            )?)),
            _ => DiscountsResourceDiscountAmountDiscountExpanded::Other(value),
        })
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
pub struct DiscountsResourceStackableDiscountWithDiscountEnd {
    #[doc = "ID of the coupon to create a new discount for."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub coupon: Option<crate::Expandable<stripe_types_06::coupons::Coupon>>,
    #[doc = "ID of an existing discount on the object (or one of its ancestors) to reuse."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub discount: Option<crate::Expandable<crate::coupons::Discount>>,
    #[doc = "ID of the promotion code to create a new discount for."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<crate::Expandable<crate::promotion_codes::PromotionCode>>,
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
pub struct FeeRefund {
    #[doc = "Amount, in cents (or local equivalent)."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Balance transaction that describes the impact on your account balance."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<crate::Expandable<crate::balance_transactions::BalanceTransaction>>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "ID of the application fee that was refunded."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fee: Option<crate::Expandable<crate::application_fees::ApplicationFee>>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::common::FeeRefundObject,
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
impl crate::PagedItem for FeeRefund {
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
pub enum PaymentSource {
    /// Expanded `account`.
    Account(Box<crate::account::Account>),
    /// Expanded `bank_account`.
    BankAccount(Box<crate::payment_methods::BankAccount>),
    /// Expanded `card`.
    Card(Box<crate::payment_methods::Card>),
    /// Expanded `source`.
    Source(Box<stripe_types_10::sources::Source>),
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
impl<'json> json::Decode<'json> for PaymentSource {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "account" => PaymentSource::Account(Box::new(json::from_slice_owned(&json::to_vec(&value))?)),
            "bank_account" => PaymentSource::BankAccount(Box::new(json::from_slice_owned(&json::to_vec(&value))?)),
            "card" => PaymentSource::Card(Box::new(json::from_slice_owned(&json::to_vec(&value))?)),
            "source" => PaymentSource::Source(Box::new(json::from_slice_owned(&json::to_vec(&value))?)),
            _ => PaymentSource::Other(value),
        })
    }
}
