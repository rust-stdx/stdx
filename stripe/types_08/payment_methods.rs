#[cfg(any(feature = "events", feature = "financial_connections"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BankConnectionsResourceAccountStatusDetails {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<
        stripe_types_06::payment_methods::BankConnectionsResourceAccountStatusDetailsApiResourceActiveStatusDetails,
    >,
}

#[cfg(feature = "payment_methods")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodParamsUsBankAccount {
    #[doc = "Account holder type: individual or company."]
    pub account_holder_type:
        Option<stripe_types_05::payment_methods::CreatePaymentMethodParamsUsBankAccountAccountHolderType>,
    #[doc = "Account number of the bank account."]
    pub account_number: Option<String>,
    #[doc = "Account type: checkings or savings."]
    pub account_type: Option<stripe_types_02::payment_methods::CreatePaymentMethodParamsUsBankAccountAccountType>,
    #[doc = "The ID of a Financial Connections Account to use as a payment method."]
    pub financial_connections_account: Option<String>,
    #[doc = "Routing number of the bank account."]
    pub routing_number: Option<String>,
}

#[cfg(feature = "payment_methods")]
impl crate::params::FormEncode for CreatePaymentMethodParamsUsBankAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account_holder_type", self.account_holder_type.as_ref());
        form.field_opt("account_number", self.account_number.as_ref());
        form.field_opt("account_type", self.account_type.as_ref());
        form.field_opt("financial_connections_account", self.financial_connections_account.as_ref());
        form.field_opt("routing_number", self.routing_number.as_ref());
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
pub struct PaymentMethodCardPresent {
    #[doc = "Card brand."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[doc = "The [product code](https://stripe.com/docs/card-product-codes) that identifies the specific program or product associ..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub brand_product: Option<String>,
    #[doc = "The cardholder name as read from the card, in [ISO 7813](https://en.wikipedia.org/wiki/ISO/IEC_7813) format."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cardholder_name: Option<String>,
    #[doc = "Two-letter ISO code representing the country of the card."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "A high-level description of the type of cards issued in this range."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
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
    #[doc = "Issuer identification number of the card."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub iin: Option<String>,
    #[doc = "The name of the card's issuing bank."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[doc = "The last four digits of the card."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[doc = "Contains information about card networks that can be used to process the payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub networks: Option<stripe_types_00::payment_methods::PaymentMethodCardPresentNetworks>,
    #[doc = "Details about payment methods collected offline."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub offline: Option<stripe_types_06::payment_methods::PaymentMethodDetailsCardPresentOffline>,
    #[doc = "The languages that the issuing bank recommends using for localizing any customer-facing text, as read from the card."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub preferred_locales: Option<Vec<String>>,
    #[doc = "How card details were read in this transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub read_method: Option<stripe_types_00::payment_methods::PaymentMethodCardPresentReadMethod>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub wallet: Option<stripe_types_05::payment_intents::PaymentFlowsPrivatePaymentMethodsCardPresentCommonWallet>,
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
pub struct PaymentMethodCardWallet {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amex_express_checkout: Option<stripe_types_03::payment_methods::PaymentMethodCardWalletAmexExpressCheckout>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<stripe_types_03::payment_methods::PaymentMethodCardWalletApplePay>,
    #[doc = "(For tokenized numbers only.) The last four digits of the device account number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub dynamic_last4: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<stripe_types_03::payment_methods::PaymentMethodCardWalletGooglePay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<stripe_types_04::payment_methods::PaymentMethodCardWalletLink>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub masterpass: Option<stripe_types_05::payment_methods::PaymentMethodCardWalletMasterpass>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub samsung_pay: Option<stripe_types_00::payment_methods::PaymentMethodCardWalletSamsungPay>,
    #[doc = "The type of the card wallet, one of `amex_express_checkout`, `apple_pay`, `google_pay`, `masterpass`, `samsung_pay`, ..."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::payment_methods::PaymentMethodCardWalletType,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub visa_checkout: Option<stripe_types_06::payment_methods::PaymentMethodCardWalletVisaCheckout>,
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
pub struct PaymentMethodDetailsCardPresent {
    #[doc = "The authorized amount"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_authorized: Option<i64>,
    #[doc = "Card brand."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[doc = "The [product code](https://stripe.com/docs/card-product-codes) that identifies the specific program or product associ..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub brand_product: Option<String>,
    #[doc = "When using manual capture, a future timestamp after which the charge will be automatically refunded if uncaptured."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub capture_before: Option<i64>,
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
    #[doc = "Whether this [PaymentIntent](https://docs.stripe.com/api/payment_intents) is eligible for incremental authorizations."]
    #[json(default)]
    pub incremental_authorization_supported: bool,
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
    #[doc = "Details about payments collected offline."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub offline: Option<stripe_types_06::payment_methods::PaymentMethodDetailsCardPresentOffline>,
    #[doc = "Defines whether the authorized amount can be over-captured or not"]
    #[json(default)]
    pub overcapture_supported: bool,
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
    pub receipt: Option<stripe_types_06::payment_methods::PaymentMethodDetailsCardPresentReceipt>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub wallet: Option<stripe_types_05::payment_intents::PaymentFlowsPrivatePaymentMethodsCardPresentCommonWallet>,
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
pub struct PaymentMethodDetailsKonbini {
    #[doc = "If the payment succeeded, this contains the details of the convenience store where the payment was completed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<stripe_types_06::payment_intents::PaymentFlowsPrivatePaymentMethodsKonbiniDetailsResourceStore>,
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
pub struct PaymentMethodDetailsPaymentRecordRevolutPay {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub funding: Option<stripe_types_05::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodRevolutPayDetailsResourceFunding>,
    #[doc = "The Revolut Pay transaction ID associated with this payment."]
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
pub struct PaymentMethodDetailsRevolutPay {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub funding: Option<stripe_types_05::payment_methods::RevolutPayUnderlyingPaymentMethodFundingDetails>,
    #[doc = "The Revolut Pay transaction ID associated with this payment."]
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
pub struct PaymentMethodOptionsPix {
    #[doc = "Determines if the amount includes the IOF tax."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_includes_iof:
        Option<stripe_types_04::payment_methods::PaymentMethodOptionsMandateOptionsPixAmountIncludesIof>,
    #[doc = "The number of seconds (between 10 and 1209600) after which Pix payment will expire."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub expires_after_seconds: Option<i64>,
    #[doc = "The timestamp at which the Pix expires."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<stripe_types_05::payment_methods::PaymentMethodOptionsMandateOptionsPix>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_types_03::payment_methods::PaymentMethodOptionsAlipaySetupFutureUsage>,
}

#[cfg(feature = "payment_methods")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodParamsBillingDetails {
    #[doc = "Billing address."]
    pub address: Option<stripe_types_05::payment_methods::UpdatePaymentMethodParamsBillingDetailsAddress>,
    #[doc = "Email address."]
    pub email: Option<json::OwnedValue>,
    #[doc = "Full name."]
    pub name: Option<json::OwnedValue>,
    #[doc = "Billing phone number (including extension)."]
    pub phone: Option<json::OwnedValue>,
    #[doc = "Taxpayer identification number."]
    pub tax_id: Option<String>,
}

#[cfg(feature = "payment_methods")]
impl crate::params::FormEncode for UpdatePaymentMethodParamsBillingDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("address", self.address.as_ref());
        form.field_opt("tax_id", self.tax_id.as_ref());
    }
}
