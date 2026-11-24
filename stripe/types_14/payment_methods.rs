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
pub struct BankAccount {
    #[doc = "The account this bank account belongs to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account: Option<crate::Expandable<crate::account::Account>>,
    #[doc = "The name of the person or business that owns the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<String>,
    #[doc = "The type of entity that holds the account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<String>,
    #[doc = "The bank account type."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[doc = "A set of available payout methods for this bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub available_payout_methods: Option<Vec<stripe_types_01::payment_methods::BankAccountAvailablePayoutMethodsItem>>,
    #[doc = "Name of the bank associated with the routing number (e.g., `WELLS FARGO`)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[doc = "Two-letter ISO code representing the country the bank account is located in."]
    #[json(default)]
    pub country: String,
    #[doc = "Three-letter [ISO code for the currency](https://stripe.com/docs/payouts) paid out to the bank account."]
    #[json(default)]
    pub currency: String,
    #[doc = "The ID of the customer that the bank account is associated with."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<crate::Expandable<crate::payment_methods::BankAccountCustomerExpanded>>,
    #[doc = "Whether this bank account is the default external account for its currency."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub default_for_currency: Option<bool>,
    #[doc = "Uniquely identifies this particular bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[doc = "Information about the upcoming new requirements for the bank account..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub future_requirements: Option<stripe_types_07::accounts::ExternalAccountRequirements>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "The last four digits of the bank account number."]
    #[json(default)]
    pub last4: String,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::payment_methods::BankAccountObject,
    #[doc = "Information about the requirements for the bank account, including what information needs to be collected."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub requirements: Option<stripe_types_07::accounts::ExternalAccountRequirements>,
    #[doc = "The routing transit number for the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
    #[doc = "For bank accounts, possible values are `new`, `validated`, `verified`, `verification_failed`, `tokenized_account_numb..."]
    #[json(default)]
    pub status: String,
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
impl crate::PagedItem for BankAccount {
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
pub enum BankAccountCustomerExpanded {
    /// Expanded `customer`.
    Customer(Box<crate::customers::Customer>),
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
impl<'json> json::Decode<'json> for BankAccountCustomerExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "customer" => {
                BankAccountCustomerExpanded::Customer(Box::new(json::from_slice_owned(&json::to_vec(&value))?))
            }
            _ => BankAccountCustomerExpanded::Other(value),
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
pub struct Card {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account: Option<crate::Expandable<crate::account::Account>>,
    #[doc = "City/District/Suburb/Town/Village."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address_city: Option<String>,
    #[doc = "Billing address country, if provided when creating card."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address_country: Option<String>,
    #[doc = "Address line 1 (Street address/PO Box/Company name)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,
    #[doc = "If `address_line1` was provided, results of the check: `pass`, `fail`, `unavailable`, or `unchecked`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address_line1_check: Option<String>,
    #[doc = "Address line 2 (Apartment/Suite/Unit/Building)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,
    #[doc = "State/County/Province/Region."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address_state: Option<String>,
    #[doc = "ZIP or postal code."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address_zip: Option<String>,
    #[doc = "If `address_zip` was provided, results of the check: `pass`, `fail`, `unavailable`, or `unchecked`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address_zip_check: Option<String>,
    #[doc = "This field indicates whether this payment method can be shown again to its customer in a checkout flow."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub allow_redisplay: Option<stripe_types_03::payment_methods::CardAllowRedisplay>,
    #[doc = "A set of available payout methods for this card."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub available_payout_methods: Option<Vec<stripe_types_01::payment_methods::BankAccountAvailablePayoutMethodsItem>>,
    #[doc = "Card brand."]
    #[json(default)]
    pub brand: String,
    #[doc = "Two-letter ISO code representing the country of the card."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "Three-letter [ISO code for currency](https://www.iso.org/iso-4217-currency-codes.html) in lowercase."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "The customer that this card belongs to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<crate::Expandable<crate::payment_methods::BankAccountCustomerExpanded>>,
    #[doc = "If a CVC was provided, results of the check: `pass`, `fail`, `unavailable`, or `unchecked`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cvc_check: Option<String>,
    #[doc = "Whether this card is the default external account for its currency."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub default_for_currency: Option<bool>,
    #[doc = "A high-level description of the type of cards issued in this range."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "(For tokenized numbers only.) The last four digits of the device account number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub dynamic_last4: Option<String>,
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
    #[json(default)]
    pub funding: String,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "Issuer identification number of the card."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub iin: Option<String>,
    #[doc = "The name of the card's issuing bank."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[doc = "The last four digits of the card."]
    #[json(default)]
    pub last4: String,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Cardholder name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub networks: Option<stripe_types_04::tokens::TokenCardNetworks>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::payment_methods::AmazonPayUnderlyingPaymentMethodFundingDetailsType,
    #[doc = "Status of a card based on the card issuer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub regulated_status: Option<stripe_types_03::payment_methods::CardRegulatedStatus>,
    #[doc = "For external accounts that are cards, possible values are `new` and `errored`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "If the card number is tokenized, this is the method that was used."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tokenization_method: Option<String>,
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
impl crate::PagedItem for Card {
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
pub struct PaymentMethod {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<stripe_types_03::payment_methods::PaymentMethodAcssDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub affirm: Option<stripe_types_00::payment_methods::PaymentMethodAffirm>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<stripe_types_00::payment_methods::PaymentMethodAfterpayClearpay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub alipay: Option<stripe_types_03::payment_intents::PaymentFlowsPrivatePaymentMethodsAlipay>,
    #[doc = "This field indicates whether this payment method can be shown again to its customer in a checkout flow."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub allow_redisplay: Option<stripe_types_03::payment_methods::CardAllowRedisplay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub alma: Option<stripe_types_02::payment_methods::PaymentMethodAlma>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amazon_pay: Option<stripe_types_04::payment_methods::PaymentMethodAmazonPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<stripe_types_03::payment_methods::PaymentMethodAuBecsDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<stripe_types_02::payment_methods::PaymentMethodBacsDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<stripe_types_00::payment_methods::PaymentMethodBancontact>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billie: Option<stripe_types_01::payment_methods::PaymentMethodBillie>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<stripe_types_05::billing::BillingDetails>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bizum: Option<stripe_types_00::payment_methods::PaymentMethodBizum>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub blik: Option<stripe_types_02::payment_methods::PaymentMethodBlik>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub boleto: Option<stripe_types_02::payment_methods::PaymentMethodBoleto>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<crate::payment_methods::PaymentMethodCard>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card_present: Option<stripe_types_08::payment_methods::PaymentMethodCardPresent>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<stripe_types_04::payment_methods::PaymentMethodCashapp>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub crypto: Option<stripe_types_02::payment_methods::PaymentMethodCrypto>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<stripe_types_05::payment_methods::PaymentMethodCustom>,
    #[doc = "The ID of the Customer to which this PaymentMethod is saved."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<crate::Expandable<crate::customers::Customer>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_account: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<stripe_types_01::payment_methods::PaymentMethodCustomerBalance>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub eps: Option<stripe_types_06::payment_methods::PaymentMethodEps>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fpx: Option<stripe_types_05::payment_methods::PaymentMethodFpx>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub giropay: Option<stripe_types_01::payment_methods::PaymentMethodGiropay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<stripe_types_02::payment_methods::PaymentMethodGrabpay>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ideal: Option<stripe_types_06::payment_methods::PaymentMethodIdeal>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<stripe_types_06::payment_methods::PaymentMethodInteracPresent>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub kakao_pay: Option<stripe_types_04::payment_methods::PaymentMethodKakaoPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub klarna: Option<stripe_types_06::payment_methods::PaymentMethodKlarna>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub konbini: Option<stripe_types_03::payment_methods::PaymentMethodKonbini>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub kr_card: Option<stripe_types_05::payment_methods::PaymentMethodKrCard>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<stripe_types_04::payment_methods::PaymentMethodLink>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mb_way: Option<stripe_types_03::payment_methods::PaymentMethodMbWay>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mobilepay: Option<stripe_types_01::payment_methods::PaymentMethodMobilepay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub multibanco: Option<stripe_types_04::payment_methods::PaymentMethodMultibanco>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub naver_pay: Option<stripe_types_05::payment_methods::PaymentMethodNaverPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub nz_bank_account: Option<stripe_types_02::payment_methods::PaymentMethodNzBankAccount>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::payment_methods::PaymentMethodObject,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<stripe_types_01::payment_methods::PaymentMethodOxxo>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub p24: Option<stripe_types_05::payment_methods::PaymentMethodP24>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pay_by_bank: Option<stripe_types_02::payment_methods::PaymentMethodPayByBank>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payco: Option<stripe_types_04::payment_methods::PaymentMethodPayco>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paynow: Option<stripe_types_03::payment_methods::PaymentMethodPaynow>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paypal: Option<stripe_types_01::payment_methods::PaymentMethodPaypal>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paypay: Option<stripe_types_04::payment_methods::PaymentMethodPaypay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payto: Option<stripe_types_01::payment_methods::PaymentMethodPayto>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pix: Option<stripe_types_03::payment_methods::PaymentMethodPix>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<stripe_types_00::payment_methods::PaymentMethodPromptpay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<stripe_types_00::radar::RadarRadarOptions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub revolut_pay: Option<stripe_types_01::payment_methods::PaymentMethodRevolutPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub samsung_pay: Option<stripe_types_02::payment_methods::PaymentMethodSamsungPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub satispay: Option<stripe_types_01::payment_methods::PaymentMethodSatispay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub scalapay: Option<stripe_types_02::payment_methods::PaymentMethodScalapay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<crate::payment_methods::PaymentMethodSepaDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sequra: Option<stripe_types_00::payment_methods::PaymentMethodSequra>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sofort: Option<stripe_types_03::payment_methods::PaymentMethodSofort>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sunbit: Option<stripe_types_01::payment_methods::PaymentMethodSunbit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub swish: Option<stripe_types_03::payment_methods::PaymentMethodSwish>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub twint: Option<stripe_types_04::payment_methods::PaymentMethodTwint>,
    #[doc = "The type of the PaymentMethod."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::payment_methods::PaymentMethodType,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub upi: Option<stripe_types_03::payment_methods::PaymentMethodUpi>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<stripe_types_11::payment_methods::PaymentMethodUsBankAccount>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<stripe_types_03::payment_methods::PaymentMethodWechatPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub zip: Option<stripe_types_04::payment_methods::PaymentMethodZip>,
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
impl crate::PagedItem for PaymentMethod {
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
pub struct PaymentMethodCard {
    #[doc = "Card brand."]
    #[json(default)]
    pub brand: String,
    #[doc = "Checks on Card address and CVC if provided."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub checks: Option<stripe_types_04::payment_methods::PaymentMethodCardChecks>,
    #[doc = "Two-letter ISO code representing the country of the card."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "A high-level description of the type of cards issued in this range."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The brand to use when displaying the card, this accounts for customer's brand choice on dual-branded cards."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub display_brand: Option<String>,
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
    #[json(default)]
    pub funding: String,
    #[doc = "Details of the original PaymentMethod that created this object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub generated_from: Option<crate::payment_methods::PaymentMethodCardGeneratedCard>,
    #[doc = "Issuer identification number of the card."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub iin: Option<String>,
    #[doc = "The name of the card's issuing bank."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[doc = "The last four digits of the card."]
    #[json(default)]
    pub last4: String,
    #[doc = "Contains information about card networks that can be used to process the payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub networks: Option<stripe_types_04::payment_methods::Networks>,
    #[doc = "Status of a card based on the card issuer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub regulated_status: Option<stripe_types_03::payment_methods::CardRegulatedStatus>,
    #[doc = "Contains details on how this Card may be used for 3D Secure authentication."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub three_d_secure_usage: Option<stripe_types_00::three_d_secure::ThreeDSecureUsage>,
    #[doc = "If this Card is part of a card wallet, this contains the details of the card wallet."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub wallet: Option<stripe_types_08::payment_methods::PaymentMethodCardWallet>,
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
pub struct PaymentMethodCardGeneratedCard {
    #[doc = "The charge that created this object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub charge: Option<String>,
    #[doc = "Transaction-specific details of the payment method used in the payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_details: Option<stripe_types_10::payment_methods::CardGeneratedFromPaymentMethodDetails>,
    #[doc = "The ID of the SetupAttempt that generated this PaymentMethod, if any."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_attempt: Option<crate::Expandable<crate::setup_attempts::SetupAttempt>>,
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
pub struct PaymentMethodDetails {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ach_credit_transfer: Option<stripe_types_00::payment_methods::PaymentMethodDetailsAchCreditTransfer>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ach_debit: Option<stripe_types_06::payment_methods::PaymentMethodDetailsAchDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<stripe_types_04::payment_methods::PaymentMethodDetailsAcssDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub affirm: Option<stripe_types_00::payment_methods::PaymentMethodDetailsAffirm>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<stripe_types_03::payment_methods::PaymentMethodDetailsAfterpayClearpay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub alipay: Option<stripe_types_00::payment_methods::PaymentMethodDetailsAlipay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub alma: Option<stripe_types_06::payment_methods::PaymentMethodDetailsAlma>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amazon_pay: Option<stripe_types_07::payment_methods::PaymentMethodDetailsAmazonPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<stripe_types_04::payment_methods::PaymentMethodDetailsAuBecsDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<stripe_types_04::payment_methods::PaymentMethodDetailsBacsDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<crate::payment_methods::PaymentMethodDetailsBancontact>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billie: Option<stripe_types_03::payment_methods::PaymentMethodDetailsBillie>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bizum: Option<stripe_types_00::payment_methods::PaymentMethodDetailsBizum>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub blik: Option<stripe_types_01::payment_methods::PaymentMethodDetailsBlik>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub boleto: Option<stripe_types_03::payment_methods::PaymentMethodDetailsBoleto>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<crate::payment_methods::PaymentMethodDetailsCard>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card_present: Option<stripe_types_08::payment_methods::PaymentMethodDetailsCardPresent>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<stripe_types_01::payment_methods::PaymentMethodDetailsCashapp>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub crypto: Option<stripe_types_05::payment_methods::PaymentMethodDetailsCrypto>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<stripe_types_01::payment_methods::PaymentMethodDetailsCustomerBalance>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub eps: Option<stripe_types_05::payment_methods::PaymentMethodDetailsEps>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fpx: Option<stripe_types_06::payment_methods::PaymentMethodDetailsFpx>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub giropay: Option<stripe_types_03::payment_methods::PaymentMethodDetailsGiropay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<stripe_types_03::payment_methods::PaymentMethodDetailsGrabpay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ideal: Option<crate::payment_methods::PaymentMethodDetailsIdeal>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<stripe_types_09::payment_methods::PaymentMethodDetailsInteracPresent>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub kakao_pay: Option<stripe_types_02::payment_methods::PaymentMethodDetailsKakaoPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub klarna: Option<stripe_types_09::payment_methods::PaymentMethodDetailsKlarna>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub konbini: Option<stripe_types_08::payment_methods::PaymentMethodDetailsKonbini>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub kr_card: Option<stripe_types_06::payment_methods::PaymentMethodDetailsKrCard>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<stripe_types_00::payment_methods::PaymentMethodDetailsLink>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mb_way: Option<stripe_types_03::payment_methods::PaymentMethodDetailsMbWay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mobilepay: Option<stripe_types_05::payment_methods::PaymentMethodDetailsMobilepay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub multibanco: Option<stripe_types_03::payment_methods::PaymentMethodDetailsMultibanco>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub naver_pay: Option<stripe_types_04::payment_methods::PaymentMethodDetailsNaverPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub nz_bank_account: Option<stripe_types_03::payment_methods::PaymentMethodDetailsNzBankAccount>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<stripe_types_03::payment_methods::PaymentMethodDetailsOxxo>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub p24: Option<stripe_types_06::payment_methods::PaymentMethodDetailsP24>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pay_by_bank: Option<stripe_types_01::payment_methods::PaymentMethodDetailsPayByBank>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payco: Option<stripe_types_01::payment_methods::PaymentMethodDetailsPayco>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paynow: Option<stripe_types_01::payment_methods::PaymentMethodDetailsPaynow>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paypal: Option<stripe_types_07::payment_methods::PaymentMethodDetailsPaypal>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paypay: Option<stripe_types_02::payment_methods::PaymentMethodDetailsPaypay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payto: Option<stripe_types_03::payment_methods::PaymentMethodDetailsPayto>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pix: Option<stripe_types_01::payment_methods::PaymentMethodDetailsPix>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<stripe_types_01::payment_methods::PaymentMethodDetailsPromptpay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub revolut_pay: Option<stripe_types_08::payment_methods::PaymentMethodDetailsRevolutPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub samsung_pay: Option<stripe_types_01::payment_methods::PaymentMethodDetailsSamsungPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub satispay: Option<stripe_types_02::payment_methods::PaymentMethodDetailsSatispay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub scalapay: Option<stripe_types_04::payment_methods::PaymentMethodDetailsScalapay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sepa_credit_transfer: Option<stripe_types_01::payment_methods::PaymentMethodDetailsSepaCreditTransfer>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<stripe_types_00::payment_methods::PaymentMethodDetailsSepaDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sequra: Option<stripe_types_04::payment_methods::PaymentMethodDetailsSequra>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sofort: Option<crate::payment_methods::PaymentMethodDetailsSofort>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub stripe_account: Option<stripe_types_00::payment_methods::PaymentMethodDetailsStripeAccount>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sunbit: Option<stripe_types_00::payment_methods::PaymentMethodDetailsSunbit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub swish: Option<stripe_types_02::payment_methods::PaymentMethodDetailsSwish>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub twint: Option<stripe_types_04::payment_methods::PaymentMethodDetailsTwint>,
    #[doc = "The type of transaction-specific details of the payment method used in the payment."]
    #[json(rename = "type", default)]
    pub r#type: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub upi: Option<stripe_types_01::payment_methods::PaymentMethodDetailsUpi>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<crate::payment_methods::PaymentMethodDetailsUsBankAccount>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub wechat: Option<stripe_types_03::payment_methods::PaymentMethodDetailsWechat>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<stripe_types_02::payment_methods::PaymentMethodDetailsWechatPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub zip: Option<stripe_types_03::payment_methods::PaymentMethodDetailsZip>,
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
pub struct PaymentMethodDetailsBancontact {
    #[doc = "Bank code of bank associated with the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    #[doc = "Name of the bank associated with the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[doc = "Bank Identifier Code of the bank associated with the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    #[doc = "The ID of the SEPA Direct Debit PaymentMethod which was generated by this Charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub generated_sepa_debit: Option<crate::Expandable<crate::payment_methods::PaymentMethod>>,
    #[doc = "The mandate for the SEPA Direct Debit PaymentMethod which was generated by this Charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub generated_sepa_debit_mandate: Option<crate::Expandable<crate::mandates::Mandate>>,
    #[doc = "Last four characters of the IBAN."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub iban_last4: Option<String>,
    #[doc = "Preferred language of the Bancontact authorization page that the customer is redirected to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<stripe_types_00::payment_methods::PaymentMethodDetailsBancontactPreferredLanguage>,
    #[doc = "Owner's verified full name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verified_name: Option<String>,
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
pub struct PaymentMethodDetailsCard {
    #[doc = "The authorized amount."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_authorized: Option<i64>,
    #[doc = "Authorization code on the charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<String>,
    #[doc = "Card brand."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[doc = "When using manual capture, a future timestamp at which the charge will be automatically refunded if uncaptured."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub capture_before: Option<i64>,
    #[doc = "Check results by Card networks on Card address and CVC at time of payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub checks: Option<stripe_types_03::payment_methods::PaymentMethodDetailsCardChecks>,
    #[doc = "Two-letter ISO code representing the country of the card."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "A high-level description of the type of cards issued in this range."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The Electronic Commerce Indicator (ECI) returned by the card network in the authorization response."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub electronic_commerce_indicator: Option<String>,
    #[doc = "Two-digit number representing the card's expiration month."]
    #[json(default)]
    pub exp_month: i64,
    #[doc = "Four-digit number representing the card's expiration year."]
    #[json(default)]
    pub exp_year: i64,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub extended_authorization: Option<stripe_types_05::payment_intents::PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorization>,
    #[doc = "Uniquely identifies this particular card number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[doc = "Card funding type."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub funding: Option<String>,
    #[doc = "Issuer identification number of the card."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub iin: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub incremental_authorization: Option<stripe_types_06::payment_intents::PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorization>,
    #[doc = "Installment details for this payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub installments: Option<stripe_types_07::payment_methods::PaymentMethodDetailsCardInstallments>,
    #[doc = "The name of the card's issuing bank."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[doc = "The last four digits of the card."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[doc = "ID of the mandate used to make this payment or created by it."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate: Option<crate::Expandable<crate::mandates::Mandate>>,
    #[doc = "True if this payment was marked as MOTO and out of scope for SCA."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub moto: Option<bool>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub multicapture: Option<stripe_types_05::payment_intents::PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticapture>,
    #[doc = "Identifies which network this charge was processed on."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[doc = "If this card has network token credentials, this contains the details of the network token credentials."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub network_token: Option<stripe_types_01::payment_methods::PaymentMethodDetailsCardNetworkToken>,
    #[doc = "This is used by the financial networks to identify a transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub network_transaction_id: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub overcapture: Option<stripe_types_06::payment_intents::PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesOvercaptureOvercapture>,
    #[doc = "Status of a card based on the card issuer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub regulated_status: Option<stripe_types_03::payment_methods::CardRegulatedStatus>,
    #[doc = "Populated if this transaction used 3D Secure authentication."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<stripe_types_06::three_d_secure::ThreeDSecureDetailsCharge>,
    #[doc = "Transaction Link ID (TLID) is a unique identifier for a transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transaction_link_id: Option<String>,
    #[doc = "If this Card is part of a card wallet, this contains the details of the card wallet."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub wallet: Option<stripe_types_09::payment_methods::PaymentMethodDetailsCardWallet>,
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
pub struct PaymentMethodDetailsIdeal {
    #[doc = "The customer's bank."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank: Option<stripe_types_03::payment_methods::PaymentMethodDetailsIdealBank>,
    #[doc = "The Bank Identifier Code of the customer's bank."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bic: Option<stripe_types_03::payment_methods::PaymentMethodDetailsIdealBic>,
    #[doc = "The ID of the SEPA Direct Debit PaymentMethod which was generated by this Charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub generated_sepa_debit: Option<crate::Expandable<crate::payment_methods::PaymentMethod>>,
    #[doc = "The mandate for the SEPA Direct Debit PaymentMethod which was generated by this Charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub generated_sepa_debit_mandate: Option<crate::Expandable<crate::mandates::Mandate>>,
    #[doc = "Last four characters of the IBAN."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub iban_last4: Option<String>,
    #[doc = "Unique transaction ID generated by iDEAL."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[doc = "Owner's verified full name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verified_name: Option<String>,
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
pub struct PaymentMethodDetailsPaymentRecordBancontact {
    #[doc = "Bank code of bank associated with the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    #[doc = "Name of the bank associated with the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[doc = "Bank Identifier Code of the bank associated with the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    #[doc = "The ID of the SEPA Direct Debit PaymentMethod which was generated by this Charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub generated_sepa_debit: Option<crate::Expandable<crate::payment_methods::PaymentMethod>>,
    #[doc = "The mandate for the SEPA Direct Debit PaymentMethod which was generated by this Charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub generated_sepa_debit_mandate: Option<crate::Expandable<crate::mandates::Mandate>>,
    #[doc = "Last four characters of the IBAN."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub iban_last4: Option<String>,
    #[doc = "Preferred language of the Bancontact authorization page that the customer is redirected to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<stripe_types_00::payment_methods::PaymentMethodDetailsBancontactPreferredLanguage>,
    #[doc = "Owner's verified full name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verified_name: Option<String>,
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
pub struct PaymentMethodDetailsPaymentRecordIdeal {
    #[doc = "The customer's bank."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank: Option<stripe_types_03::payment_methods::PaymentMethodDetailsIdealBank>,
    #[doc = "The Bank Identifier Code of the customer's bank."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bic: Option<stripe_types_03::payment_methods::PaymentMethodDetailsIdealBic>,
    #[doc = "The ID of the SEPA Direct Debit PaymentMethod which was generated by this Charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub generated_sepa_debit: Option<crate::Expandable<crate::payment_methods::PaymentMethod>>,
    #[doc = "The mandate for the SEPA Direct Debit PaymentMethod which was generated by this Charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub generated_sepa_debit_mandate: Option<crate::Expandable<crate::mandates::Mandate>>,
    #[doc = "Last four characters of the IBAN."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub iban_last4: Option<String>,
    #[doc = "Unique transaction ID generated by iDEAL."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[doc = "Owner's verified full name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verified_name: Option<String>,
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
pub struct PaymentMethodDetailsPaymentRecordSofort {
    #[doc = "Bank code of bank associated with the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    #[doc = "Name of the bank associated with the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[doc = "Bank Identifier Code of the bank associated with the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    #[doc = "Two-letter ISO code representing the country the bank account is located in."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "The ID of the SEPA Direct Debit PaymentMethod which was generated by this Charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub generated_sepa_debit: Option<crate::Expandable<crate::payment_methods::PaymentMethod>>,
    #[doc = "The mandate for the SEPA Direct Debit PaymentMethod which was generated by this Charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub generated_sepa_debit_mandate: Option<crate::Expandable<crate::mandates::Mandate>>,
    #[doc = "Last four characters of the IBAN."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub iban_last4: Option<String>,
    #[doc = "Preferred language of the SOFORT authorization page that the customer is redirected to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub preferred_language:
        Option<stripe_types_01::payment_methods::PaymentMethodDetailsPaymentRecordSofortPreferredLanguage>,
    #[doc = "Owner's verified full name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verified_name: Option<String>,
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
pub struct PaymentMethodDetailsPaymentRecordUsBankAccount {
    #[doc = "The type of entity that holds the account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<stripe_types_02::payment_methods::PaymentMethodDetailsAchDebitAccountHolderType>,
    #[doc = "The type of the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_type:
        Option<stripe_types_00::payment_methods::PaymentMethodDetailsPaymentRecordUsBankAccountAccountType>,
    #[doc = "Name of the bank associated with the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[doc = "Estimated date to debit the customer's bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub expected_debit_date: Option<String>,
    #[doc = "Uniquely identifies this particular bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[doc = "Last four digits of the bank account number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[doc = "ID of the mandate used to make this payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate: Option<crate::Expandable<crate::mandates::Mandate>>,
    #[doc = "The ACH payment reference for this transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_reference: Option<String>,
    #[doc = "The routing number for the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
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
pub struct PaymentMethodDetailsSofort {
    #[doc = "Bank code of bank associated with the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    #[doc = "Name of the bank associated with the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[doc = "Bank Identifier Code of the bank associated with the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    #[doc = "Two-letter ISO code representing the country the bank account is located in."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "The ID of the SEPA Direct Debit PaymentMethod which was generated by this Charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub generated_sepa_debit: Option<crate::Expandable<crate::payment_methods::PaymentMethod>>,
    #[doc = "The mandate for the SEPA Direct Debit PaymentMethod which was generated by this Charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub generated_sepa_debit_mandate: Option<crate::Expandable<crate::mandates::Mandate>>,
    #[doc = "Last four characters of the IBAN."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub iban_last4: Option<String>,
    #[doc = "Preferred language of the SOFORT authorization page that the customer is redirected to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub preferred_language:
        Option<stripe_types_01::payment_methods::PaymentMethodDetailsPaymentRecordSofortPreferredLanguage>,
    #[doc = "Owner's verified full name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verified_name: Option<String>,
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
pub struct PaymentMethodDetailsUsBankAccount {
    #[doc = "Account holder type: individual or company."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<stripe_types_02::payment_methods::PaymentMethodDetailsAchDebitAccountHolderType>,
    #[doc = "Account type: checkings or savings."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_type:
        Option<stripe_types_00::payment_methods::PaymentMethodDetailsPaymentRecordUsBankAccountAccountType>,
    #[doc = "Name of the bank associated with the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[doc = "Estimated date to debit the customer's bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub expected_debit_date: Option<String>,
    #[doc = "Uniquely identifies this particular bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[doc = "Last four digits of the bank account number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[doc = "ID of the mandate used to make this payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate: Option<crate::Expandable<crate::mandates::Mandate>>,
    #[doc = "Reference number to locate ACH payments with customer's bank."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_reference: Option<String>,
    #[doc = "Routing number of the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
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
pub struct PaymentMethodSepaDebit {
    #[doc = "Bank code of bank associated with the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    #[doc = "Branch code of bank associated with the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub branch_code: Option<String>,
    #[doc = "Two-letter ISO code representing the country the bank account is located in."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "Uniquely identifies this particular bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[doc = "Information about the object that generated this PaymentMethod."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub generated_from: Option<crate::payment_methods::SepaDebitGeneratedFrom>,
    #[doc = "Last four characters of the IBAN."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
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
pub struct SepaDebitGeneratedFrom {
    #[doc = "The ID of the Charge that generated this PaymentMethod, if any."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub charge: Option<crate::Expandable<crate::charges::Charge>>,
    #[doc = "The ID of the SetupAttempt that generated this PaymentMethod, if any."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_attempt: Option<crate::Expandable<crate::setup_attempts::SetupAttempt>>,
}
