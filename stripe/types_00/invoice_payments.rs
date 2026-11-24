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
pub enum InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry {
    BE,
    DE,
    ES,
    FR,
    IE,
    NL,
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
impl InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry::BE => "BE",
            InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry::DE => "DE",
            InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry::ES => "ES",
            InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry::FR => "FR",
            InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry::IE => "IE",
            InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry::NL => "NL",
            InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry::Unknown(value) => value,
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
impl Default for InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry {
    fn default() -> Self {
        InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry::Unknown(String::new())
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
impl<'json> json::Decode<'json> for InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "BE" => InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry::BE,
            "DE" => InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry::DE,
            "ES" => InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry::ES,
            "FR" => InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry::FR,
            "IE" => InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry::IE,
            "NL" => InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry::NL,
            other => {
                InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry::Unknown(other.to_string())
            }
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
pub enum InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsFiltersAccountSubcategoriesItem {
    Checking,
    Savings,
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
impl InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsFiltersAccountSubcategoriesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsFiltersAccountSubcategoriesItem::Checking => {
                "checking"
            }
            InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsFiltersAccountSubcategoriesItem::Savings => {
                "savings"
            }
            InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsFiltersAccountSubcategoriesItem::Unknown(
                value,
            ) => value,
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
impl<'json> json::Decode<'json>
    for InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsFiltersAccountSubcategoriesItem
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsFiltersAccountSubcategoriesItem::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "checking" => {
                InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsFiltersAccountSubcategoriesItem::Checking
            }
            "savings" => {
                InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsFiltersAccountSubcategoriesItem::Savings
            }
            other => {
                InvoicePaymentMethodOptionsUsBankAccountLinkedAccountOptionsFiltersAccountSubcategoriesItem::Unknown(
                    other.to_string(),
                )
            }
        })
    }
}

#[cfg(feature = "invoice_payments")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListInvoicePaymentsParamsStatus {
    Canceled,
    Open,
    Paid,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoice_payments")]
impl ListInvoicePaymentsParamsStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListInvoicePaymentsParamsStatus::Canceled => "canceled",
            ListInvoicePaymentsParamsStatus::Open => "open",
            ListInvoicePaymentsParamsStatus::Paid => "paid",
            ListInvoicePaymentsParamsStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoice_payments")]
impl<'json> json::Decode<'json> for ListInvoicePaymentsParamsStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListInvoicePaymentsParamsStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "canceled" => ListInvoicePaymentsParamsStatus::Canceled,
            "open" => ListInvoicePaymentsParamsStatus::Open,
            "paid" => ListInvoicePaymentsParamsStatus::Paid,
            other => ListInvoicePaymentsParamsStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "invoice_payments")]
impl crate::params::FormEncode for ListInvoicePaymentsParamsStatus {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
