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
pub struct SetupAttempt {
    #[doc = "The value of [application](https://docs.stripe.com/api/setup_intents/object#setup_intent_object-application) on the S..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<crate::Expandable<stripe_types_05::common::Application>>,
    #[doc = "If present, the SetupIntent's payment method will be attached to the in-context Stripe Account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub attach_to_self: Option<bool>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "The value of [customer](https://docs.stripe.com/api/setup_intents/object#setup_intent_object-customer) on the SetupIn..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<crate::Expandable<crate::setup_attempts::SetupAttemptCustomerExpanded>>,
    #[doc = "The value of customer_account..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_account: Option<String>,
    #[doc = "Indicates the directions of money movement for which this payment method is intended to be used."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub flow_directions: Option<Vec<stripe_types_04::setup_attempts::SetupAttemptFlowDirectionsItem>>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::setup_attempts::SetupAttemptObject,
    #[doc = "The value of [on_behalf_of](https://docs.stripe.com/api/setup_intents/object#setup_intent_object-on_behalf_of) on the..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<crate::Expandable<crate::account::Account>>,
    #[doc = "ID of the payment method used with this SetupAttempt."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<crate::Expandable<crate::payment_methods::PaymentMethod>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_details: Option<crate::setup_attempts::SetupAttemptPaymentMethodDetails>,
    #[doc = "The error encountered during this attempt to confirm the SetupIntent, if any."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_error: Option<crate::common::ApiErrors>,
    #[doc = "ID of the SetupIntent that this attempt belongs to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_intent: Option<crate::Expandable<crate::setup_intents::SetupIntent>>,
    #[doc = "Status of this SetupAttempt, one of `requires_confirmation`, `requires_action`, `processing`, `succeeded`, `failed`, ..."]
    #[json(default)]
    pub status: String,
    #[doc = "The value of [usage](https://docs.stripe.com/api/setup_intents/object#setup_intent_object-usage) on the SetupIntent a..."]
    #[json(default)]
    pub usage: String,
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
impl crate::PagedItem for SetupAttempt {
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
pub enum SetupAttemptCustomerExpanded {
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
impl<'json> json::Decode<'json> for SetupAttemptCustomerExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "customer" => {
                SetupAttemptCustomerExpanded::Customer(Box::new(json::from_slice_owned(&json::to_vec(&value))?))
            }
            _ => SetupAttemptCustomerExpanded::Other(value),
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
pub struct SetupAttemptPaymentMethodDetails {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<stripe_types_04::setup_attempts::SetupAttemptPaymentMethodDetailsAcssDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amazon_pay: Option<stripe_types_01::setup_attempts::SetupAttemptPaymentMethodDetailsAmazonPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<stripe_types_00::setup_attempts::SetupAttemptPaymentMethodDetailsAuBecsDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<stripe_types_02::setup_attempts::SetupAttemptPaymentMethodDetailsBacsDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<crate::setup_attempts::SetupAttemptPaymentMethodDetailsBancontact>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub blik: Option<stripe_types_03::setup_attempts::SetupAttemptPaymentMethodDetailsBlik>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub boleto: Option<stripe_types_04::setup_attempts::SetupAttemptPaymentMethodDetailsBoleto>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types_07::setup_attempts::SetupAttemptPaymentMethodDetailsCard>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card_present: Option<crate::setup_attempts::SetupAttemptPaymentMethodDetailsCardPresent>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<stripe_types_01::setup_attempts::SetupAttemptPaymentMethodDetailsCashapp>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ideal: Option<crate::setup_attempts::SetupAttemptPaymentMethodDetailsIdeal>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub kakao_pay: Option<stripe_types_01::setup_attempts::SetupAttemptPaymentMethodDetailsKakaoPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub klarna: Option<stripe_types_00::setup_attempts::SetupAttemptPaymentMethodDetailsKlarna>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub kr_card: Option<stripe_types_01::setup_attempts::SetupAttemptPaymentMethodDetailsKrCard>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<stripe_types_04::setup_attempts::SetupAttemptPaymentMethodDetailsLink>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub naver_pay: Option<stripe_types_00::setup_attempts::SetupAttemptPaymentMethodDetailsNaverPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub nz_bank_account: Option<stripe_types_02::setup_attempts::SetupAttemptPaymentMethodDetailsNzBankAccount>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paypal: Option<stripe_types_02::setup_attempts::SetupAttemptPaymentMethodDetailsPaypal>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payto: Option<stripe_types_00::setup_attempts::SetupAttemptPaymentMethodDetailsPayto>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pix: Option<stripe_types_03::setup_attempts::SetupAttemptPaymentMethodDetailsPix>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub revolut_pay: Option<stripe_types_00::setup_attempts::SetupAttemptPaymentMethodDetailsRevolutPay>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub satispay:
        Option<stripe_types_02::payment_intents::PaymentFlowsPrivatePaymentMethodsSatispaySetupAttemptDetails>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<stripe_types_04::setup_attempts::SetupAttemptPaymentMethodDetailsSepaDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sofort: Option<crate::setup_attempts::SetupAttemptPaymentMethodDetailsSofort>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub twint: Option<stripe_types_01::setup_attempts::SetupAttemptPaymentMethodDetailsTwint>,
    #[doc = "The type of the payment method used in the SetupIntent (e.g., `card`)."]
    #[json(rename = "type", default)]
    pub r#type: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub upi: Option<stripe_types_02::setup_attempts::SetupAttemptPaymentMethodDetailsUpi>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<stripe_types_03::setup_attempts::SetupAttemptPaymentMethodDetailsUsBankAccount>,
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
pub struct SetupAttemptPaymentMethodDetailsBancontact {
    #[doc = "Bank code of bank associated with the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    #[doc = "Name of the bank associated with the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[doc = "Bank Identifier Code of the bank associated with the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    #[doc = "The ID of the SEPA Direct Debit PaymentMethod which was generated by this SetupAttempt."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub generated_sepa_debit: Option<crate::Expandable<crate::payment_methods::PaymentMethod>>,
    #[doc = "The mandate for the SEPA Direct Debit PaymentMethod which was generated by this SetupAttempt."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub generated_sepa_debit_mandate: Option<crate::Expandable<crate::mandates::Mandate>>,
    #[doc = "Last four characters of the IBAN."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub iban_last4: Option<String>,
    #[doc = "Preferred language of the Bancontact authorization page that the customer is redirected to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub preferred_language:
        Option<stripe_types_02::setup_attempts::SetupAttemptPaymentMethodDetailsBancontactPreferredLanguage>,
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
pub struct SetupAttemptPaymentMethodDetailsCardPresent {
    #[doc = "The ID of the Card PaymentMethod which was generated by this SetupAttempt."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub generated_card: Option<crate::Expandable<crate::payment_methods::PaymentMethod>>,
    #[doc = "Details about payments collected offline."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub offline: Option<stripe_types_06::payment_methods::PaymentMethodDetailsCardPresentOffline>,
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
pub struct SetupAttemptPaymentMethodDetailsIdeal {
    #[doc = "The customer's bank."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank: Option<stripe_types_04::setup_attempts::SetupAttemptPaymentMethodDetailsIdealBank>,
    #[doc = "The Bank Identifier Code of the customer's bank."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bic: Option<stripe_types_01::setup_attempts::SetupAttemptPaymentMethodDetailsIdealBic>,
    #[doc = "The ID of the SEPA Direct Debit PaymentMethod which was generated by this SetupAttempt."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub generated_sepa_debit: Option<crate::Expandable<crate::payment_methods::PaymentMethod>>,
    #[doc = "The mandate for the SEPA Direct Debit PaymentMethod which was generated by this SetupAttempt."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub generated_sepa_debit_mandate: Option<crate::Expandable<crate::mandates::Mandate>>,
    #[doc = "Last four characters of the IBAN."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub iban_last4: Option<String>,
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
pub struct SetupAttemptPaymentMethodDetailsSofort {
    #[doc = "Bank code of bank associated with the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    #[doc = "Name of the bank associated with the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[doc = "Bank Identifier Code of the bank associated with the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    #[doc = "The ID of the SEPA Direct Debit PaymentMethod which was generated by this SetupAttempt."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub generated_sepa_debit: Option<crate::Expandable<crate::payment_methods::PaymentMethod>>,
    #[doc = "The mandate for the SEPA Direct Debit PaymentMethod which was generated by this SetupAttempt."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub generated_sepa_debit_mandate: Option<crate::Expandable<crate::mandates::Mandate>>,
    #[doc = "Last four characters of the IBAN."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub iban_last4: Option<String>,
    #[doc = "Preferred language of the Sofort authorization page that the customer is redirected to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub preferred_language:
        Option<stripe_types_02::setup_attempts::SetupAttemptPaymentMethodDetailsBancontactPreferredLanguage>,
    #[doc = "Owner's verified full name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verified_name: Option<String>,
}
