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
pub struct RefundDestinationDetails {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub affirm: Option<stripe_types_02::common::DestinationDetailsUnimplemented>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<stripe_types_02::common::DestinationDetailsUnimplemented>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub alipay: Option<stripe_types_02::common::DestinationDetailsUnimplemented>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub alma: Option<stripe_types_02::common::DestinationDetailsUnimplemented>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amazon_pay: Option<stripe_types_02::common::DestinationDetailsUnimplemented>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub au_bank_transfer: Option<stripe_types_02::common::DestinationDetailsUnimplemented>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub blik: Option<stripe_types_04::refunds::RefundDestinationDetailsBlik>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub br_bank_transfer: Option<stripe_types_00::refunds::RefundDestinationDetailsBrBankTransfer>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types_05::refunds::RefundDestinationDetailsCard>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<stripe_types_02::common::DestinationDetailsUnimplemented>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub crypto: Option<stripe_types_00::refunds::RefundDestinationDetailsCrypto>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_cash_balance: Option<stripe_types_02::common::DestinationDetailsUnimplemented>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub eps: Option<stripe_types_02::common::DestinationDetailsUnimplemented>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub eu_bank_transfer: Option<stripe_types_04::refunds::RefundDestinationDetailsEuBankTransfer>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub gb_bank_transfer: Option<stripe_types_00::refunds::RefundDestinationDetailsGbBankTransfer>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub giropay: Option<stripe_types_02::common::DestinationDetailsUnimplemented>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<stripe_types_02::common::DestinationDetailsUnimplemented>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub jp_bank_transfer: Option<stripe_types_01::refunds::RefundDestinationDetailsJpBankTransfer>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub klarna: Option<stripe_types_02::common::DestinationDetailsUnimplemented>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mb_way: Option<stripe_types_00::refunds::RefundDestinationDetailsMbWay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub multibanco: Option<stripe_types_03::refunds::RefundDestinationDetailsMultibanco>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mx_bank_transfer: Option<stripe_types_02::refunds::RefundDestinationDetailsMxBankTransfer>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub nz_bank_transfer: Option<stripe_types_02::common::DestinationDetailsUnimplemented>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub p24: Option<stripe_types_02::refunds::RefundDestinationDetailsP24>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paynow: Option<stripe_types_02::common::DestinationDetailsUnimplemented>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paypal: Option<stripe_types_02::refunds::RefundDestinationDetailsPaypal>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pix: Option<stripe_types_02::common::DestinationDetailsUnimplemented>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub revolut: Option<stripe_types_02::common::DestinationDetailsUnimplemented>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub scalapay: Option<stripe_types_02::common::DestinationDetailsUnimplemented>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sofort: Option<stripe_types_02::common::DestinationDetailsUnimplemented>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub swish: Option<stripe_types_01::refunds::RefundDestinationDetailsSwish>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub th_bank_transfer: Option<stripe_types_03::refunds::RefundDestinationDetailsThBankTransfer>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub twint: Option<stripe_types_02::common::DestinationDetailsUnimplemented>,
    #[doc = "The type of transaction-specific details of the payment method used in the refund (e.g., `card`)."]
    #[json(rename = "type", default)]
    pub r#type: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub us_bank_transfer: Option<stripe_types_04::refunds::RefundDestinationDetailsUsBankTransfer>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<stripe_types_02::common::DestinationDetailsUnimplemented>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub zip: Option<stripe_types_02::common::DestinationDetailsUnimplemented>,
}
