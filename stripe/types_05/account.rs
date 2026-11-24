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
pub struct AccountCapabilities {
    #[doc = "The status of the Canadian pre-authorized debits payments capability of the account, or whether the account can direc..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub acss_debit_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the Affirm capability of the account, or whether the account can directly process Affirm charges."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub affirm_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the Afterpay Clearpay capability of the account, or whether the account can directly process Afterpay C..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the Alma capability of the account, or whether the account can directly process Alma payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub alma_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the AmazonPay capability of the account, or whether the account can directly process AmazonPay payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amazon_pay_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the `app_distribution` capability of the account, or whether the platform can distribute apps to other ..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub app_distribution: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the BECS Direct Debit (AU) payments capability of the account, or whether the account can directly proc..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub au_becs_debit_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the Bacs Direct Debits payments capability of the account, or whether the account can directly process ..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bacs_debit_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the Bancontact payments capability of the account, or whether the account can directly process Banconta..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bancontact_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the customer_balance payments capability of the account, or whether the account can directly process cu..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_transfer_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the Billie capability of the account, or whether the account can directly process Billie payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billie_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the Bizum capability of the account, or whether the account can directly process Bizum payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bizum_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the blik payments capability of the account, or whether the account can directly process blik charges."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub blik_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the BLIK recurring payments capability of the account, or whether the account can accept recurring and ..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub blik_recurring_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the boleto payments capability of the account, or whether the account can directly process boleto charges."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub boleto_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the card issuing capability of the account, or whether you can use Issuing to distribute funds on cards"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the card payments capability of the account, or whether the account can directly process credit and deb..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the Cartes Bancaires payments capability of the account, or whether the account can directly process Ca..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the Cash App Pay capability of the account, or whether the account can directly process Cash App Pay pa..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cashapp_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the Crypto capability of the account, or whether the account can directly process Crypto payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub crypto_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the EPS payments capability of the account, or whether the account can directly process EPS charges."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub eps_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the FPX payments capability of the account, or whether the account can directly process FPX charges."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fpx_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the GB customer_balance payments (GBP currency) capability of the account, or whether the account can d..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub gb_bank_transfer_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the giropay payments capability of the account, or whether the account can directly process giropay cha..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub giropay_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the GrabPay payments capability of the account, or whether the account can directly process GrabPay cha..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub grabpay_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the iDEAL payments capability of the account, or whether the account can directly process iDEAL charges."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ideal_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the india_international_payments capability of the account, or whether the account can process internat..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub india_international_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the JCB payments capability of the account, or whether the account (Japan only) can directly process JC..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub jcb_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the Japanese customer_balance payments (JPY currency) capability of the account, or whether the account..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub jp_bank_transfer_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the KakaoPay capability of the account, or whether the account can directly process KakaoPay payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub kakao_pay_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the Klarna payments capability of the account, or whether the account can directly process Klarna charges."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub klarna_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the konbini payments capability of the account, or whether the account can directly process konbini cha..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub konbini_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the KrCard capability of the account, or whether the account can directly process KrCard payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub kr_card_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the legacy payments capability of the account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub legacy_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the link_payments capability of the account, or whether the account can directly process Link charges."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub link_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the MB WAY payments capability of the account, or whether the account can directly process MB WAY charges."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mb_way_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the MobilePay capability of the account, or whether the account can directly process MobilePay charges."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mobilepay_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the Multibanco payments capability of the account, or whether the account can directly process Multiban..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub multibanco_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the Mexican customer_balance payments (MXN currency) capability of the account, or whether the account ..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mx_bank_transfer_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the NaverPay capability of the account, or whether the account can directly process NaverPay payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub naver_pay_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the New Zealand BECS Direct Debit payments capability of the account, or whether the account can direct..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub nz_bank_account_becs_debit_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the OXXO payments capability of the account, or whether the account can directly process OXXO charges."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub oxxo_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the P24 payments capability of the account, or whether the account can directly process P24 charges."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub p24_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the pay_by_bank payments capability of the account, or whether the account can directly process pay_by_..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pay_by_bank_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the Payco capability of the account, or whether the account can directly process Payco payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payco_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the paynow payments capability of the account, or whether the account can directly process paynow charges."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paynow_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the Paypay capability of the account, or whether the account can directly process Paypay payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paypay_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the PayTo capability of the account, or whether the account can directly process PayTo charges."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payto_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the pix payments capability of the account, or whether the account can directly process pix charges."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pix_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the promptpay payments capability of the account, or whether the account can directly process promptpay..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub promptpay_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the RevolutPay capability of the account, or whether the account can directly process RevolutPay payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub revolut_pay_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the SamsungPay capability of the account, or whether the account can directly process SamsungPay payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub samsung_pay_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the Satispay capability of the account, or whether the account can directly process Satispay payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub satispay_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the Scalapay capability of the account, or whether the account can directly process Scalapay payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub scalapay_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the SEPA customer_balance payments (EUR currency) capability of the account, or whether the account can..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sepa_bank_transfer_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the SEPA Direct Debits payments capability of the account, or whether the account can directly process ..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sepa_debit_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the SeQura capability of the account, or whether the account can directly process SeQura payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sequra_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the Sofort payments capability of the account, or whether the account can directly process Sofort charges."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sofort_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the Sunbit capability of the account, or whether the account can directly process Sunbit payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sunbit_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the Swish capability of the account, or whether the account can directly process Swish payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub swish_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the tax reporting 1099-K (US) capability of the account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax_reporting_us_1099_k: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the tax reporting 1099-MISC (US) capability of the account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax_reporting_us_1099_misc: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the transfers capability of the account, or whether your platform can transfer funds to the account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transfers: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the banking capability, or whether the account can have bank accounts."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub treasury: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the TWINT capability of the account, or whether the account can directly process TWINT charges."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub twint_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the upi payments capability of the account, or whether the account can directly process upi charges."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub upi_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the US bank account ACH payments capability of the account, or whether the account can directly process..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub us_bank_account_ach_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the US customer_balance payments (USD currency) capability of the account, or whether the account can d..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub us_bank_transfer_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
    #[doc = "The status of the Zip capability of the account, or whether the account can directly process Zip charges."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub zip_payments: Option<stripe_types_01::account::AccountCapabilitiesAcssDebitPayments>,
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
pub struct AccountCardIssuingSettings {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<stripe_types_02::payment_methods::CardIssuingAccountTermsOfService>,
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct DeletedAccount {
    #[doc = "Always true for a deleted object"]
    #[json(default)]
    pub deleted: bool,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::account::AccountObject,
}

#[cfg(feature = "accounts")]
impl crate::PagedItem for DeletedAccount {
    fn id(&self) -> &str {
        &self.id
    }
}
