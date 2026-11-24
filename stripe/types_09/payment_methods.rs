#[cfg(feature = "payment_methods")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodParamsUpi {
    #[doc = "Configuration options for setting up an eMandate"]
    pub mandate_options: Option<stripe_types_05::payment_methods::CreatePaymentMethodParamsUpiMandateOptions>,
}

#[cfg(feature = "payment_methods")]
impl crate::params::FormEncode for CreatePaymentMethodParamsUpi {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentMethodConfigResourcePaymentMethodProperties {
    #[doc = "Whether this payment method may be offered at checkout."]
    #[json(default)]
    pub available: bool,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub display_preference: Option<stripe_types_05::payment_methods::PaymentMethodConfigResourceDisplayPreference>,
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
pub struct PaymentMethodDetailsCardWallet {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amex_express_checkout:
        Option<stripe_types_01::payment_methods::PaymentMethodDetailsCardWalletAmexExpressCheckout>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<stripe_types_02::payment_methods::PaymentMethodDetailsCardWalletApplePay>,
    #[doc = "(For tokenized numbers only.) The last four digits of the device account number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub dynamic_last4: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<stripe_types_03::payment_methods::PaymentMethodDetailsCardWalletGooglePay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<stripe_types_01::payment_methods::PaymentMethodDetailsCardWalletLink>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub masterpass: Option<stripe_types_05::payment_methods::PaymentMethodDetailsCardWalletMasterpass>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub samsung_pay: Option<stripe_types_00::payment_methods::PaymentMethodDetailsCardWalletSamsungPay>,
    #[doc = "The type of the card wallet, one of `amex_express_checkout`, `apple_pay`, `google_pay`, `masterpass`, `samsung_pay`, ..."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::payment_methods::PaymentMethodCardWalletType,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub visa_checkout: Option<stripe_types_05::payment_methods::PaymentMethodDetailsCardWalletVisaCheckout>,
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
pub struct PaymentMethodDetailsInteracPresent {
    #[doc = "Card brand."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[doc = "The cardholder name as read from the card, in [ISO 7813](https://en.wikipedia.org/wiki/ISO/IEC_7813) format."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cardholder_name: Option<String>,
    #[doc = "Two-letter ISO code representing the country of the card."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "A high-level description of the type of cards issued in this range."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Authorization response cryptogram."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub emv_auth_data: Option<String>,
    #[doc = "Two-digit number representing the card's expiration month."]
    #[json(default)]
    pub exp_month: i64,
    #[doc = "Four-digit number representing the card's expiration year."]
    #[json(default)]
    pub exp_year: i64,
    #[doc = "Uniquely identifies this particular card number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[doc = "Card funding type."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub funding: Option<String>,
    #[doc = "ID of a card PaymentMethod generated from the card_present PaymentMethod that may be attached to a Customer for futur..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub generated_card: Option<String>,
    #[doc = "Issuer identification number of the card."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub iin: Option<String>,
    #[doc = "The name of the card's issuing bank."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[doc = "The last four digits of the card."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[doc = "ID of the [location](https://docs.stripe.com/api/terminal/locations) that this transaction's reader is assigned to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Identifies which network this charge was processed on."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[doc = "This is used by the financial networks to identify a transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub network_transaction_id: Option<String>,
    #[doc = "The languages that the issuing bank recommends using for localizing any customer-facing text, as read from the card."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub preferred_locales: Option<Vec<String>>,
    #[doc = "How card details were read in this transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub read_method: Option<stripe_types_00::payment_methods::PaymentMethodCardPresentReadMethod>,
    #[doc = "ID of the [reader](https://docs.stripe.com/api/terminal/readers) this transaction was made on."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reader: Option<String>,
    #[doc = "A collection of fields required to be displayed on receipts."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub receipt: Option<stripe_types_05::payment_methods::PaymentMethodDetailsInteracPresentReceipt>,
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
pub struct PaymentMethodDetailsKlarna {
    #[doc = "ID of the [location](https://docs.stripe.com/api/terminal/locations) that this transaction's reader is assigned to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The payer details for this transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payer_details: Option<stripe_types_06::payment_methods::KlarnaPayerDetails>,
    #[doc = "The Klarna payment method used for this transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_category: Option<String>,
    #[doc = "Preferred language of the Klarna authorization page that the customer is redirected to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub preferred_locale: Option<String>,
    #[doc = "ID of the [reader](https://docs.stripe.com/api/terminal/readers) this transaction was made on."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reader: Option<String>,
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
pub struct PaymentMethodDetailsPaymentRecordAmazonPay {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub funding: Option<stripe_types_06::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodAmazonPayDetailsResourceFunding>,
    #[doc = "The Amazon Pay transaction ID associated with this payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
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
pub struct PaymentMethodDetailsPaymentRecordKonbini {
    #[doc = "If the payment succeeded, this contains the details of the convenience store where the payment was completed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<stripe_types_05::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKonbiniDetailsResourceStore>,
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
pub struct PaymentMethodOptionsBillie {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<stripe_types_04::payment_methods::PaymentMethodOptionsAffirmCaptureMethod>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub company_details: Option<stripe_types_05::payment_intents::PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetails>,
    #[doc = "An identifier or reference that this payment corresponds to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
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
pub struct PaymentMethodUsBankAccountStatusDetails {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub blocked: Option<stripe_types_05::payment_methods::PaymentMethodUsBankAccountBlocked>,
}
