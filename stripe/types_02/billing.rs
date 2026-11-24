#[cfg(any(feature = "billing", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum BillingAlertStatus {
    Active,
    Archived,
    Inactive,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "billing", feature = "events"))]
impl BillingAlertStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BillingAlertStatus::Active => "active",
            BillingAlertStatus::Archived => "archived",
            BillingAlertStatus::Inactive => "inactive",
            BillingAlertStatus::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "billing", feature = "events"))]
impl<'json> json::Decode<'json> for BillingAlertStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BillingAlertStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "active" => BillingAlertStatus::Active,
            "archived" => BillingAlertStatus::Archived,
            "inactive" => BillingAlertStatus::Inactive,
            other => BillingAlertStatus::Unknown(other.to_string()),
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
pub enum BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType {
    InvoiceItemDetails,
    SubscriptionItemDetails,
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
impl BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType::InvoiceItemDetails => {
                "invoice_item_details"
            }
            BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType::SubscriptionItemDetails => {
                "subscription_item_details"
            }
            BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType::Unknown(value) => value,
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
impl Default for BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType {
    fn default() -> Self {
        BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType::Unknown(String::new())
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
impl<'json> json::Decode<'json> for BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "invoice_item_details" => {
                BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType::InvoiceItemDetails
            }
            "subscription_item_details" => {
                BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType::SubscriptionItemDetails
            }
            other => BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType::Unknown(other.to_string()),
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
pub struct BillingBillResourceInvoicingParentsInvoiceQuoteParent {
    #[doc = "The quote that generated this invoice"]
    #[json(default)]
    pub quote: String,
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
pub enum BillingBillResourceInvoicingTaxesTaxTaxBehavior {
    Exclusive,
    Inclusive,
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
impl BillingBillResourceInvoicingTaxesTaxTaxBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BillingBillResourceInvoicingTaxesTaxTaxBehavior::Exclusive => "exclusive",
            BillingBillResourceInvoicingTaxesTaxTaxBehavior::Inclusive => "inclusive",
            BillingBillResourceInvoicingTaxesTaxTaxBehavior::Unknown(value) => value,
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
impl Default for BillingBillResourceInvoicingTaxesTaxTaxBehavior {
    fn default() -> Self {
        BillingBillResourceInvoicingTaxesTaxTaxBehavior::Unknown(String::new())
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
impl<'json> json::Decode<'json> for BillingBillResourceInvoicingTaxesTaxTaxBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BillingBillResourceInvoicingTaxesTaxTaxBehavior::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "exclusive" => BillingBillResourceInvoicingTaxesTaxTaxBehavior::Exclusive,
            "inclusive" => BillingBillResourceInvoicingTaxesTaxTaxBehavior::Inclusive,
            other => BillingBillResourceInvoicingTaxesTaxTaxBehavior::Unknown(other.to_string()),
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
pub enum BillingBillResourceInvoicingTaxesTaxType {
    TaxRateDetails,
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
impl BillingBillResourceInvoicingTaxesTaxType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BillingBillResourceInvoicingTaxesTaxType::TaxRateDetails => "tax_rate_details",
            BillingBillResourceInvoicingTaxesTaxType::Unknown(value) => value,
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
impl Default for BillingBillResourceInvoicingTaxesTaxType {
    fn default() -> Self {
        BillingBillResourceInvoicingTaxesTaxType::Unknown(String::new())
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
impl<'json> json::Decode<'json> for BillingBillResourceInvoicingTaxesTaxType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BillingBillResourceInvoicingTaxesTaxType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "tax_rate_details" => BillingBillResourceInvoicingTaxesTaxType::TaxRateDetails,
            other => BillingBillResourceInvoicingTaxesTaxType::Unknown(other.to_string()),
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
pub struct BillingClocksResourceStatusDetailsAdvancingStatusDetails {
    #[doc = "The `frozen_time` that the Test Clock is advancing towards."]
    #[json(default)]
    pub target_frozen_time: i64,
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone, PartialEq)]
pub enum BillingCreditBalanceSummaryObject {
    BillingCreditBalanceSummary,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "billing")]
impl BillingCreditBalanceSummaryObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BillingCreditBalanceSummaryObject::BillingCreditBalanceSummary => "billing.credit_balance_summary",
            BillingCreditBalanceSummaryObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "billing")]
impl Default for BillingCreditBalanceSummaryObject {
    fn default() -> Self {
        BillingCreditBalanceSummaryObject::Unknown(String::new())
    }
}

#[cfg(feature = "billing")]
impl<'json> json::Decode<'json> for BillingCreditBalanceSummaryObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BillingCreditBalanceSummaryObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "billing.credit_balance_summary" => BillingCreditBalanceSummaryObject::BillingCreditBalanceSummary,
            other => BillingCreditBalanceSummaryObject::Unknown(other.to_string()),
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
pub enum BillingCreditBalanceTransactionType {
    Credit,
    Debit,
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
impl BillingCreditBalanceTransactionType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BillingCreditBalanceTransactionType::Credit => "credit",
            BillingCreditBalanceTransactionType::Debit => "debit",
            BillingCreditBalanceTransactionType::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for BillingCreditBalanceTransactionType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BillingCreditBalanceTransactionType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "credit" => BillingCreditBalanceTransactionType::Credit,
            "debit" => BillingCreditBalanceTransactionType::Debit,
            other => BillingCreditBalanceTransactionType::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "billing_portal",
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
pub enum BillingFeedbackOptionObject {
    BillingFeedbackOption,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "billing_portal",
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
impl BillingFeedbackOptionObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BillingFeedbackOptionObject::BillingFeedbackOption => "billing.feedback_option",
            BillingFeedbackOptionObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "billing_portal",
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
impl Default for BillingFeedbackOptionObject {
    fn default() -> Self {
        BillingFeedbackOptionObject::Unknown(String::new())
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "billing_portal",
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
impl<'json> json::Decode<'json> for BillingFeedbackOptionObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BillingFeedbackOptionObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "billing.feedback_option" => BillingFeedbackOptionObject::BillingFeedbackOption,
            other => BillingFeedbackOptionObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "billing", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum BillingMeterEventTimeWindow {
    Day,
    Hour,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "billing", feature = "events"))]
impl BillingMeterEventTimeWindow {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BillingMeterEventTimeWindow::Day => "day",
            BillingMeterEventTimeWindow::Hour => "hour",
            BillingMeterEventTimeWindow::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "billing", feature = "events"))]
impl<'json> json::Decode<'json> for BillingMeterEventTimeWindow {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BillingMeterEventTimeWindow::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "day" => BillingMeterEventTimeWindow::Day,
            "hour" => BillingMeterEventTimeWindow::Hour,
            other => BillingMeterEventTimeWindow::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "billing", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum BillingMeterResourceAggregationSettingsFormula {
    Count,
    Last,
    Sum,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "billing", feature = "events"))]
impl BillingMeterResourceAggregationSettingsFormula {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BillingMeterResourceAggregationSettingsFormula::Count => "count",
            BillingMeterResourceAggregationSettingsFormula::Last => "last",
            BillingMeterResourceAggregationSettingsFormula::Sum => "sum",
            BillingMeterResourceAggregationSettingsFormula::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "billing", feature = "events"))]
impl Default for BillingMeterResourceAggregationSettingsFormula {
    fn default() -> Self {
        BillingMeterResourceAggregationSettingsFormula::Unknown(String::new())
    }
}

#[cfg(any(feature = "billing", feature = "events"))]
impl<'json> json::Decode<'json> for BillingMeterResourceAggregationSettingsFormula {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BillingMeterResourceAggregationSettingsFormula::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "count" => BillingMeterResourceAggregationSettingsFormula::Count,
            "last" => BillingMeterResourceAggregationSettingsFormula::Last,
            "sum" => BillingMeterResourceAggregationSettingsFormula::Sum,
            other => BillingMeterResourceAggregationSettingsFormula::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BillingMeterResourceBillingMeterEventAdjustmentCancel {
    #[doc = "Unique identifier for the event."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateBillingAlertParamsUsageThresholdFiltersItemType {
    Customer,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "billing")]
impl CreateBillingAlertParamsUsageThresholdFiltersItemType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateBillingAlertParamsUsageThresholdFiltersItemType::Customer => "customer",
            CreateBillingAlertParamsUsageThresholdFiltersItemType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "billing")]
impl<'json> json::Decode<'json> for CreateBillingAlertParamsUsageThresholdFiltersItemType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateBillingAlertParamsUsageThresholdFiltersItemType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "customer" => CreateBillingAlertParamsUsageThresholdFiltersItemType::Customer,
            other => CreateBillingAlertParamsUsageThresholdFiltersItemType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for CreateBillingAlertParamsUsageThresholdFiltersItemType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateBillingCreditGrantParamsAmountType {
    Monetary,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "billing")]
impl CreateBillingCreditGrantParamsAmountType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateBillingCreditGrantParamsAmountType::Monetary => "monetary",
            CreateBillingCreditGrantParamsAmountType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "billing")]
impl<'json> json::Decode<'json> for CreateBillingCreditGrantParamsAmountType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateBillingCreditGrantParamsAmountType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "monetary" => CreateBillingCreditGrantParamsAmountType::Monetary,
            other => CreateBillingCreditGrantParamsAmountType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for CreateBillingCreditGrantParamsAmountType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateBillingMeterParamsCustomerMappingType {
    ById,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "billing")]
impl CreateBillingMeterParamsCustomerMappingType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateBillingMeterParamsCustomerMappingType::ById => "by_id",
            CreateBillingMeterParamsCustomerMappingType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "billing")]
impl<'json> json::Decode<'json> for CreateBillingMeterParamsCustomerMappingType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateBillingMeterParamsCustomerMappingType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "by_id" => CreateBillingMeterParamsCustomerMappingType::ById,
            other => CreateBillingMeterParamsCustomerMappingType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for CreateBillingMeterParamsCustomerMappingType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateBillingMeterParamsDefaultAggregationFormula {
    Count,
    Last,
    Sum,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "billing")]
impl CreateBillingMeterParamsDefaultAggregationFormula {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateBillingMeterParamsDefaultAggregationFormula::Count => "count",
            CreateBillingMeterParamsDefaultAggregationFormula::Last => "last",
            CreateBillingMeterParamsDefaultAggregationFormula::Sum => "sum",
            CreateBillingMeterParamsDefaultAggregationFormula::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "billing")]
impl<'json> json::Decode<'json> for CreateBillingMeterParamsDefaultAggregationFormula {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateBillingMeterParamsDefaultAggregationFormula::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "count" => CreateBillingMeterParamsDefaultAggregationFormula::Count,
            "last" => CreateBillingMeterParamsDefaultAggregationFormula::Last,
            "sum" => CreateBillingMeterParamsDefaultAggregationFormula::Sum,
            other => CreateBillingMeterParamsDefaultAggregationFormula::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for CreateBillingMeterParamsDefaultAggregationFormula {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListBillingAlertsParamsAlertType {
    UsageThreshold,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "billing")]
impl ListBillingAlertsParamsAlertType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListBillingAlertsParamsAlertType::UsageThreshold => "usage_threshold",
            ListBillingAlertsParamsAlertType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "billing")]
impl<'json> json::Decode<'json> for ListBillingAlertsParamsAlertType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListBillingAlertsParamsAlertType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "usage_threshold" => ListBillingAlertsParamsAlertType::UsageThreshold,
            other => ListBillingAlertsParamsAlertType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for ListBillingAlertsParamsAlertType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(any(feature = "billing", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum ThresholdsResourceUsageThresholdConfigRecurrence {
    OneTime,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "billing", feature = "events"))]
impl ThresholdsResourceUsageThresholdConfigRecurrence {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ThresholdsResourceUsageThresholdConfigRecurrence::OneTime => "one_time",
            ThresholdsResourceUsageThresholdConfigRecurrence::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "billing", feature = "events"))]
impl Default for ThresholdsResourceUsageThresholdConfigRecurrence {
    fn default() -> Self {
        ThresholdsResourceUsageThresholdConfigRecurrence::Unknown(String::new())
    }
}

#[cfg(any(feature = "billing", feature = "events"))]
impl<'json> json::Decode<'json> for ThresholdsResourceUsageThresholdConfigRecurrence {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ThresholdsResourceUsageThresholdConfigRecurrence::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "one_time" => ThresholdsResourceUsageThresholdConfigRecurrence::OneTime,
            other => ThresholdsResourceUsageThresholdConfigRecurrence::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone, Default)]
pub struct UpdateBillingMeterParams {
    #[doc = "The meter’s name."]
    pub display_name: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for UpdateBillingMeterParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("display_name", self.display_name.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}
