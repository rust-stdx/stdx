#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BankConnectionsResourceAuthorizationStatusDetails {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<stripe_types_05::payment_methods::BankConnectionsResourceAuthorizationStatusDetailsApiResourceActiveStatusDetails>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub inactive: Option<stripe_types_06::payment_methods::BankConnectionsResourceAuthorizationStatusDetailsApiResourceInactiveStatusDetails>,
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
pub struct CardMandatePaymentMethodDetails {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub india: Option<stripe_types_05::payment_methods::CardMandatePaymentMethodDetailsIndia>,
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
pub struct PaymentMethodDetailsAmazonPay {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub funding: Option<stripe_types_06::payment_methods::AmazonPayUnderlyingPaymentMethodFundingDetails>,
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
pub struct PaymentMethodDetailsCardInstallments {
    #[doc = "Installment plan selected for the payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<stripe_types_06::payment_methods::PaymentMethodDetailsCardInstallmentsPlan>,
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
pub struct PaymentMethodDetailsPaymentRecordKlarna {
    #[doc = "ID of the [location](https://docs.stripe.com/api/terminal/locations) that this transaction's reader is assigned to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The payer details for this transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payer_details: Option<stripe_types_05::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodKlarnaDetailsResourcePayerDetails>,
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
pub struct PaymentMethodDetailsPaypal {
    #[doc = "Two-letter ISO code representing the buyer's country."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "Owner's email."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payer_email: Option<String>,
    #[doc = "PayPal account PayerID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payer_id: Option<String>,
    #[doc = "Owner's full name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payer_name: Option<String>,
    #[doc = "The level of protection offered as defined by PayPal Seller Protection for Merchants, for this transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub seller_protection: Option<stripe_types_05::payment_methods::PaypalSellerProtection>,
    #[doc = "A unique ID generated by PayPal for this transaction."]
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
pub struct PaymentMethodOptionsCardInstallments {
    #[doc = "Installment plans that may be selected for this PaymentIntent."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub available_plans: Option<Vec<stripe_types_06::payment_methods::PaymentMethodDetailsCardInstallmentsPlan>>,
    #[doc = "Whether Installments are enabled for this PaymentIntent."]
    #[json(default)]
    pub enabled: bool,
    #[doc = "Installment plan selected for this PaymentIntent."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<stripe_types_06::payment_methods::PaymentMethodDetailsCardInstallmentsPlan>,
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
pub struct PaymentMethodOptionsCardPresent {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<stripe_types_04::payment_methods::PaymentMethodOptionsCardPresentCaptureMethod>,
    #[doc = "Request ability to capture this payment beyond the standard authorization validity window..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub request_extended_authorization: Option<bool>,
    #[doc = "Request ability to [increment](https://docs.stripe.com/terminal/features/incremental-authorizations) this PaymentInte..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub request_incremental_authorization_support: Option<bool>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub routing: Option<stripe_types_06::payment_methods::PaymentMethodOptionsCardPresentRouting>,
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
pub struct PaymentMethodOptionsCustomerBalanceBankTransfer {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub eu_bank_transfer: Option<stripe_types_06::payment_methods::PaymentMethodOptionsCustomerBalanceEuBankAccount>,
    #[doc = "List of address types that should be returned in the financial_addresses response."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub requested_address_types: Option<
        Vec<stripe_types_01::payment_methods::PaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem>,
    >,
    #[doc = "The bank transfer type that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_trans..."]
    #[json(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<stripe_types_01::payment_methods::PaymentMethodOptionsCustomerBalanceBankTransferType>,
}

#[cfg(feature = "payment_methods")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodParamsCard {
    #[doc = "Two-digit number representing the card's expiration month."]
    pub exp_month: Option<i64>,
    #[doc = "Four-digit number representing the card's expiration year."]
    pub exp_year: Option<i64>,
    #[doc = "Contains information about card networks used to process the payment."]
    pub networks: Option<stripe_types_05::payment_methods::UpdatePaymentMethodParamsCardNetworks>,
}

#[cfg(feature = "payment_methods")]
impl crate::params::FormEncode for UpdatePaymentMethodParamsCard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("exp_month", self.exp_month.as_ref());
        form.field_opt("exp_year", self.exp_year.as_ref());
        form.field_opt("networks", self.networks.as_ref());
    }
}

#[cfg(feature = "payment_methods")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodParamsUsBankAccount {
    #[doc = "Bank account holder type."]
    pub account_holder_type:
        Option<stripe_types_06::payment_methods::UpdatePaymentMethodParamsUsBankAccountAccountHolderType>,
    #[doc = "Bank account type."]
    pub account_type: Option<stripe_types_06::payment_methods::UpdatePaymentMethodParamsUsBankAccountAccountType>,
}

#[cfg(feature = "payment_methods")]
impl crate::params::FormEncode for UpdatePaymentMethodParamsUsBankAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account_holder_type", self.account_holder_type.as_ref());
        form.field_opt("account_type", self.account_type.as_ref());
    }
}
