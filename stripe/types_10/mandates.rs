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
pub struct MandatePaymentMethodDetails {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<stripe_types_06::mandates::MandateAcssDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amazon_pay: Option<stripe_types_03::mandates::MandateAmazonPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<stripe_types_02::mandates::MandateAuBecsDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<stripe_types_06::mandates::MandateBacsDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub blik: Option<stripe_types_05::mandates::MandateBlik>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types_07::payment_methods::CardMandatePaymentMethodDetails>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<stripe_types_02::mandates::MandateCashapp>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub kakao_pay: Option<stripe_types_00::mandates::MandateKakaoPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub klarna: Option<stripe_types_02::mandates::MandateKlarna>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub kr_card: Option<stripe_types_04::mandates::MandateKrCard>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<stripe_types_01::mandates::MandateLink>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub naver_pay: Option<stripe_types_01::mandates::MandateNaverPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub nz_bank_account: Option<stripe_types_00::mandates::MandateNzBankAccount>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paypal: Option<stripe_types_01::mandates::MandatePaypal>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payto: Option<stripe_types_05::mandates::MandatePayto>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pix: Option<stripe_types_05::mandates::MandatePix>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub revolut_pay: Option<stripe_types_01::mandates::MandateRevolutPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<stripe_types_01::mandates::MandateSepaDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub twint: Option<stripe_types_00::mandates::MandateTwint>,
    #[doc = "This mandate corresponds with a specific payment method type."]
    #[json(rename = "type", default)]
    pub r#type: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub upi: Option<stripe_types_05::mandates::MandateUpi>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<stripe_types_05::mandates::MandateUsBankAccount>,
}
