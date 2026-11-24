#[cfg(feature = "customers")]
#[derive(Debug, Clone, Default)]
pub struct CreateCustomerBalanceTransactionParams {
    #[doc = "The integer amount in **cents (or local equivalent)** to apply to the customer's credit balance."]
    pub amount: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "An arbitrary string attached to the object."]
    pub description: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for CreateCustomerBalanceTransactionParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        form.field("currency", &self.currency);
        form.field_opt("description", self.description.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
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
#[derive(Debug, Clone, PartialEq)]
pub enum CustomerAcceptanceType {
    Offline,
    Online,
    /// A value not known to this version of the crate.
    Unknown(String),
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
impl CustomerAcceptanceType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CustomerAcceptanceType::Offline => "offline",
            CustomerAcceptanceType::Online => "online",
            CustomerAcceptanceType::Unknown(value) => value,
        }
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
impl Default for CustomerAcceptanceType {
    fn default() -> Self {
        CustomerAcceptanceType::Unknown(String::new())
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
impl<'json> json::Decode<'json> for CustomerAcceptanceType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CustomerAcceptanceType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "offline" => CustomerAcceptanceType::Offline,
            "online" => CustomerAcceptanceType::Online,
            other => CustomerAcceptanceType::Unknown(other.to_string()),
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
#[derive(Debug, Clone, PartialEq)]
pub enum CustomerBalanceCustomerBalanceSettingsReconciliationMode {
    Automatic,
    Manual,
    /// A value not known to this version of the crate.
    Unknown(String),
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
impl CustomerBalanceCustomerBalanceSettingsReconciliationMode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CustomerBalanceCustomerBalanceSettingsReconciliationMode::Automatic => "automatic",
            CustomerBalanceCustomerBalanceSettingsReconciliationMode::Manual => "manual",
            CustomerBalanceCustomerBalanceSettingsReconciliationMode::Unknown(value) => value,
        }
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
impl Default for CustomerBalanceCustomerBalanceSettingsReconciliationMode {
    fn default() -> Self {
        CustomerBalanceCustomerBalanceSettingsReconciliationMode::Unknown(String::new())
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
impl<'json> json::Decode<'json> for CustomerBalanceCustomerBalanceSettingsReconciliationMode {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CustomerBalanceCustomerBalanceSettingsReconciliationMode::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "automatic" => CustomerBalanceCustomerBalanceSettingsReconciliationMode::Automatic,
            "manual" => CustomerBalanceCustomerBalanceSettingsReconciliationMode::Manual,
            other => CustomerBalanceCustomerBalanceSettingsReconciliationMode::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum CustomerBalanceTransactionType {
    Adjustment,
    AppliedToInvoice,
    CheckoutSessionSubscriptionPayment,
    CheckoutSessionSubscriptionPaymentCanceled,
    CreditNote,
    Initial,
    InvoiceOverpaid,
    InvoiceTooLarge,
    InvoiceTooSmall,
    Migration,
    UnappliedFromInvoice,
    UnspentReceiverCredit,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
impl CustomerBalanceTransactionType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CustomerBalanceTransactionType::Adjustment => "adjustment",
            CustomerBalanceTransactionType::AppliedToInvoice => "applied_to_invoice",
            CustomerBalanceTransactionType::CheckoutSessionSubscriptionPayment => {
                "checkout_session_subscription_payment"
            }
            CustomerBalanceTransactionType::CheckoutSessionSubscriptionPaymentCanceled => {
                "checkout_session_subscription_payment_canceled"
            }
            CustomerBalanceTransactionType::CreditNote => "credit_note",
            CustomerBalanceTransactionType::Initial => "initial",
            CustomerBalanceTransactionType::InvoiceOverpaid => "invoice_overpaid",
            CustomerBalanceTransactionType::InvoiceTooLarge => "invoice_too_large",
            CustomerBalanceTransactionType::InvoiceTooSmall => "invoice_too_small",
            CustomerBalanceTransactionType::Migration => "migration",
            CustomerBalanceTransactionType::UnappliedFromInvoice => "unapplied_from_invoice",
            CustomerBalanceTransactionType::UnspentReceiverCredit => "unspent_receiver_credit",
            CustomerBalanceTransactionType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
impl Default for CustomerBalanceTransactionType {
    fn default() -> Self {
        CustomerBalanceTransactionType::Unknown(String::new())
    }
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
impl<'json> json::Decode<'json> for CustomerBalanceTransactionType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CustomerBalanceTransactionType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "adjustment" => CustomerBalanceTransactionType::Adjustment,
            "applied_to_invoice" => CustomerBalanceTransactionType::AppliedToInvoice,
            "checkout_session_subscription_payment" => {
                CustomerBalanceTransactionType::CheckoutSessionSubscriptionPayment
            }
            "checkout_session_subscription_payment_canceled" => {
                CustomerBalanceTransactionType::CheckoutSessionSubscriptionPaymentCanceled
            }
            "credit_note" => CustomerBalanceTransactionType::CreditNote,
            "initial" => CustomerBalanceTransactionType::Initial,
            "invoice_overpaid" => CustomerBalanceTransactionType::InvoiceOverpaid,
            "invoice_too_large" => CustomerBalanceTransactionType::InvoiceTooLarge,
            "invoice_too_small" => CustomerBalanceTransactionType::InvoiceTooSmall,
            "migration" => CustomerBalanceTransactionType::Migration,
            "unapplied_from_invoice" => CustomerBalanceTransactionType::UnappliedFromInvoice,
            "unspent_receiver_credit" => CustomerBalanceTransactionType::UnspentReceiverCredit,
            other => CustomerBalanceTransactionType::Unknown(other.to_string()),
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
#[derive(Debug, Clone, PartialEq)]
pub enum CustomerTaxLocationSource {
    BillingAddress,
    IpAddress,
    PaymentMethod,
    ShippingDestination,
    /// A value not known to this version of the crate.
    Unknown(String),
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
impl CustomerTaxLocationSource {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CustomerTaxLocationSource::BillingAddress => "billing_address",
            CustomerTaxLocationSource::IpAddress => "ip_address",
            CustomerTaxLocationSource::PaymentMethod => "payment_method",
            CustomerTaxLocationSource::ShippingDestination => "shipping_destination",
            CustomerTaxLocationSource::Unknown(value) => value,
        }
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
impl Default for CustomerTaxLocationSource {
    fn default() -> Self {
        CustomerTaxLocationSource::Unknown(String::new())
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
impl<'json> json::Decode<'json> for CustomerTaxLocationSource {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CustomerTaxLocationSource::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "billing_address" => CustomerTaxLocationSource::BillingAddress,
            "ip_address" => CustomerTaxLocationSource::IpAddress,
            "payment_method" => CustomerTaxLocationSource::PaymentMethod,
            "shipping_destination" => CustomerTaxLocationSource::ShippingDestination,
            other => CustomerTaxLocationSource::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "customers")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListCustomerPaymentMethodsParamsType {
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    Alma,
    AmazonPay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Billie,
    Bizum,
    Blik,
    Boleto,
    Card,
    Cashapp,
    Crypto,
    Custom,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    KakaoPay,
    Klarna,
    Konbini,
    KrCard,
    Link,
    MbWay,
    Mobilepay,
    Multibanco,
    NaverPay,
    NzBankAccount,
    Oxxo,
    P24,
    PayByBank,
    Payco,
    Paynow,
    Paypal,
    Paypay,
    Payto,
    Pix,
    Promptpay,
    RevolutPay,
    SamsungPay,
    Satispay,
    Scalapay,
    SepaDebit,
    Sequra,
    Sofort,
    Sunbit,
    Swish,
    Twint,
    Upi,
    UsBankAccount,
    WechatPay,
    Zip,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "customers")]
impl ListCustomerPaymentMethodsParamsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListCustomerPaymentMethodsParamsType::AcssDebit => "acss_debit",
            ListCustomerPaymentMethodsParamsType::Affirm => "affirm",
            ListCustomerPaymentMethodsParamsType::AfterpayClearpay => "afterpay_clearpay",
            ListCustomerPaymentMethodsParamsType::Alipay => "alipay",
            ListCustomerPaymentMethodsParamsType::Alma => "alma",
            ListCustomerPaymentMethodsParamsType::AmazonPay => "amazon_pay",
            ListCustomerPaymentMethodsParamsType::AuBecsDebit => "au_becs_debit",
            ListCustomerPaymentMethodsParamsType::BacsDebit => "bacs_debit",
            ListCustomerPaymentMethodsParamsType::Bancontact => "bancontact",
            ListCustomerPaymentMethodsParamsType::Billie => "billie",
            ListCustomerPaymentMethodsParamsType::Bizum => "bizum",
            ListCustomerPaymentMethodsParamsType::Blik => "blik",
            ListCustomerPaymentMethodsParamsType::Boleto => "boleto",
            ListCustomerPaymentMethodsParamsType::Card => "card",
            ListCustomerPaymentMethodsParamsType::Cashapp => "cashapp",
            ListCustomerPaymentMethodsParamsType::Crypto => "crypto",
            ListCustomerPaymentMethodsParamsType::Custom => "custom",
            ListCustomerPaymentMethodsParamsType::CustomerBalance => "customer_balance",
            ListCustomerPaymentMethodsParamsType::Eps => "eps",
            ListCustomerPaymentMethodsParamsType::Fpx => "fpx",
            ListCustomerPaymentMethodsParamsType::Giropay => "giropay",
            ListCustomerPaymentMethodsParamsType::Grabpay => "grabpay",
            ListCustomerPaymentMethodsParamsType::Ideal => "ideal",
            ListCustomerPaymentMethodsParamsType::KakaoPay => "kakao_pay",
            ListCustomerPaymentMethodsParamsType::Klarna => "klarna",
            ListCustomerPaymentMethodsParamsType::Konbini => "konbini",
            ListCustomerPaymentMethodsParamsType::KrCard => "kr_card",
            ListCustomerPaymentMethodsParamsType::Link => "link",
            ListCustomerPaymentMethodsParamsType::MbWay => "mb_way",
            ListCustomerPaymentMethodsParamsType::Mobilepay => "mobilepay",
            ListCustomerPaymentMethodsParamsType::Multibanco => "multibanco",
            ListCustomerPaymentMethodsParamsType::NaverPay => "naver_pay",
            ListCustomerPaymentMethodsParamsType::NzBankAccount => "nz_bank_account",
            ListCustomerPaymentMethodsParamsType::Oxxo => "oxxo",
            ListCustomerPaymentMethodsParamsType::P24 => "p24",
            ListCustomerPaymentMethodsParamsType::PayByBank => "pay_by_bank",
            ListCustomerPaymentMethodsParamsType::Payco => "payco",
            ListCustomerPaymentMethodsParamsType::Paynow => "paynow",
            ListCustomerPaymentMethodsParamsType::Paypal => "paypal",
            ListCustomerPaymentMethodsParamsType::Paypay => "paypay",
            ListCustomerPaymentMethodsParamsType::Payto => "payto",
            ListCustomerPaymentMethodsParamsType::Pix => "pix",
            ListCustomerPaymentMethodsParamsType::Promptpay => "promptpay",
            ListCustomerPaymentMethodsParamsType::RevolutPay => "revolut_pay",
            ListCustomerPaymentMethodsParamsType::SamsungPay => "samsung_pay",
            ListCustomerPaymentMethodsParamsType::Satispay => "satispay",
            ListCustomerPaymentMethodsParamsType::Scalapay => "scalapay",
            ListCustomerPaymentMethodsParamsType::SepaDebit => "sepa_debit",
            ListCustomerPaymentMethodsParamsType::Sequra => "sequra",
            ListCustomerPaymentMethodsParamsType::Sofort => "sofort",
            ListCustomerPaymentMethodsParamsType::Sunbit => "sunbit",
            ListCustomerPaymentMethodsParamsType::Swish => "swish",
            ListCustomerPaymentMethodsParamsType::Twint => "twint",
            ListCustomerPaymentMethodsParamsType::Upi => "upi",
            ListCustomerPaymentMethodsParamsType::UsBankAccount => "us_bank_account",
            ListCustomerPaymentMethodsParamsType::WechatPay => "wechat_pay",
            ListCustomerPaymentMethodsParamsType::Zip => "zip",
            ListCustomerPaymentMethodsParamsType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "customers")]
impl<'json> json::Decode<'json> for ListCustomerPaymentMethodsParamsType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListCustomerPaymentMethodsParamsType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "acss_debit" => ListCustomerPaymentMethodsParamsType::AcssDebit,
            "affirm" => ListCustomerPaymentMethodsParamsType::Affirm,
            "afterpay_clearpay" => ListCustomerPaymentMethodsParamsType::AfterpayClearpay,
            "alipay" => ListCustomerPaymentMethodsParamsType::Alipay,
            "alma" => ListCustomerPaymentMethodsParamsType::Alma,
            "amazon_pay" => ListCustomerPaymentMethodsParamsType::AmazonPay,
            "au_becs_debit" => ListCustomerPaymentMethodsParamsType::AuBecsDebit,
            "bacs_debit" => ListCustomerPaymentMethodsParamsType::BacsDebit,
            "bancontact" => ListCustomerPaymentMethodsParamsType::Bancontact,
            "billie" => ListCustomerPaymentMethodsParamsType::Billie,
            "bizum" => ListCustomerPaymentMethodsParamsType::Bizum,
            "blik" => ListCustomerPaymentMethodsParamsType::Blik,
            "boleto" => ListCustomerPaymentMethodsParamsType::Boleto,
            "card" => ListCustomerPaymentMethodsParamsType::Card,
            "cashapp" => ListCustomerPaymentMethodsParamsType::Cashapp,
            "crypto" => ListCustomerPaymentMethodsParamsType::Crypto,
            "custom" => ListCustomerPaymentMethodsParamsType::Custom,
            "customer_balance" => ListCustomerPaymentMethodsParamsType::CustomerBalance,
            "eps" => ListCustomerPaymentMethodsParamsType::Eps,
            "fpx" => ListCustomerPaymentMethodsParamsType::Fpx,
            "giropay" => ListCustomerPaymentMethodsParamsType::Giropay,
            "grabpay" => ListCustomerPaymentMethodsParamsType::Grabpay,
            "ideal" => ListCustomerPaymentMethodsParamsType::Ideal,
            "kakao_pay" => ListCustomerPaymentMethodsParamsType::KakaoPay,
            "klarna" => ListCustomerPaymentMethodsParamsType::Klarna,
            "konbini" => ListCustomerPaymentMethodsParamsType::Konbini,
            "kr_card" => ListCustomerPaymentMethodsParamsType::KrCard,
            "link" => ListCustomerPaymentMethodsParamsType::Link,
            "mb_way" => ListCustomerPaymentMethodsParamsType::MbWay,
            "mobilepay" => ListCustomerPaymentMethodsParamsType::Mobilepay,
            "multibanco" => ListCustomerPaymentMethodsParamsType::Multibanco,
            "naver_pay" => ListCustomerPaymentMethodsParamsType::NaverPay,
            "nz_bank_account" => ListCustomerPaymentMethodsParamsType::NzBankAccount,
            "oxxo" => ListCustomerPaymentMethodsParamsType::Oxxo,
            "p24" => ListCustomerPaymentMethodsParamsType::P24,
            "pay_by_bank" => ListCustomerPaymentMethodsParamsType::PayByBank,
            "payco" => ListCustomerPaymentMethodsParamsType::Payco,
            "paynow" => ListCustomerPaymentMethodsParamsType::Paynow,
            "paypal" => ListCustomerPaymentMethodsParamsType::Paypal,
            "paypay" => ListCustomerPaymentMethodsParamsType::Paypay,
            "payto" => ListCustomerPaymentMethodsParamsType::Payto,
            "pix" => ListCustomerPaymentMethodsParamsType::Pix,
            "promptpay" => ListCustomerPaymentMethodsParamsType::Promptpay,
            "revolut_pay" => ListCustomerPaymentMethodsParamsType::RevolutPay,
            "samsung_pay" => ListCustomerPaymentMethodsParamsType::SamsungPay,
            "satispay" => ListCustomerPaymentMethodsParamsType::Satispay,
            "scalapay" => ListCustomerPaymentMethodsParamsType::Scalapay,
            "sepa_debit" => ListCustomerPaymentMethodsParamsType::SepaDebit,
            "sequra" => ListCustomerPaymentMethodsParamsType::Sequra,
            "sofort" => ListCustomerPaymentMethodsParamsType::Sofort,
            "sunbit" => ListCustomerPaymentMethodsParamsType::Sunbit,
            "swish" => ListCustomerPaymentMethodsParamsType::Swish,
            "twint" => ListCustomerPaymentMethodsParamsType::Twint,
            "upi" => ListCustomerPaymentMethodsParamsType::Upi,
            "us_bank_account" => ListCustomerPaymentMethodsParamsType::UsBankAccount,
            "wechat_pay" => ListCustomerPaymentMethodsParamsType::WechatPay,
            "zip" => ListCustomerPaymentMethodsParamsType::Zip,
            other => ListCustomerPaymentMethodsParamsType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for ListCustomerPaymentMethodsParamsType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "customers")]
#[derive(Debug, Clone, Default)]
pub struct ListCustomersParamsCreated {
    #[doc = "Minimum value to filter by (exclusive)"]
    pub gt: Option<i64>,
    #[doc = "Minimum value to filter by (inclusive)"]
    pub gte: Option<i64>,
    #[doc = "Maximum value to filter by (exclusive)"]
    pub lt: Option<i64>,
    #[doc = "Maximum value to filter by (inclusive)"]
    pub lte: Option<i64>,
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for ListCustomersParamsCreated {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("gt", self.gt.as_ref());
        form.field_opt("gte", self.gte.as_ref());
        form.field_opt("lt", self.lt.as_ref());
        form.field_opt("lte", self.lte.as_ref());
    }
}
