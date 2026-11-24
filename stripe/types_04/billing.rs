#[cfg(any(feature = "events", feature = "invoiceitems"))]
#[derive(Debug, Clone, PartialEq)]
pub enum BillingBillResourceInvoiceItemParentsInvoiceItemParentType {
    SubscriptionDetails,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "invoiceitems"))]
impl BillingBillResourceInvoiceItemParentsInvoiceItemParentType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BillingBillResourceInvoiceItemParentsInvoiceItemParentType::SubscriptionDetails => "subscription_details",
            BillingBillResourceInvoiceItemParentsInvoiceItemParentType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "invoiceitems"))]
impl Default for BillingBillResourceInvoiceItemParentsInvoiceItemParentType {
    fn default() -> Self {
        BillingBillResourceInvoiceItemParentsInvoiceItemParentType::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "invoiceitems"))]
impl<'json> json::Decode<'json> for BillingBillResourceInvoiceItemParentsInvoiceItemParentType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BillingBillResourceInvoiceItemParentsInvoiceItemParentType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "subscription_details" => BillingBillResourceInvoiceItemParentsInvoiceItemParentType::SubscriptionDetails,
            other => BillingBillResourceInvoiceItemParentsInvoiceItemParentType::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "invoiceitems"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BillingBillResourceInvoiceItemParentsInvoiceItemSubscriptionParent {
    #[doc = "The subscription that generated this invoice item"]
    #[json(default)]
    pub subscription: String,
    #[doc = "The subscription item that generated this invoice item"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub subscription_item: Option<String>,
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
pub enum BillingBillResourceInvoicingTaxesTaxTaxabilityReason {
    CustomerExempt,
    NotAvailable,
    NotCollecting,
    NotSubjectToTax,
    NotSupported,
    PortionProductExempt,
    PortionReducedRated,
    PortionStandardRated,
    ProductExempt,
    ProductExemptHoliday,
    ProportionallyRated,
    ReducedRated,
    ReverseCharge,
    StandardRated,
    TaxableBasisReduced,
    ZeroRated,
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
impl BillingBillResourceInvoicingTaxesTaxTaxabilityReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::CustomerExempt => "customer_exempt",
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::NotAvailable => "not_available",
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::NotCollecting => "not_collecting",
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::NotSubjectToTax => "not_subject_to_tax",
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::NotSupported => "not_supported",
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::PortionProductExempt => "portion_product_exempt",
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::PortionReducedRated => "portion_reduced_rated",
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::PortionStandardRated => "portion_standard_rated",
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::ProductExempt => "product_exempt",
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::ProductExemptHoliday => "product_exempt_holiday",
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::ProportionallyRated => "proportionally_rated",
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::ReducedRated => "reduced_rated",
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::ReverseCharge => "reverse_charge",
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::StandardRated => "standard_rated",
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::TaxableBasisReduced => "taxable_basis_reduced",
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::ZeroRated => "zero_rated",
            BillingBillResourceInvoicingTaxesTaxTaxabilityReason::Unknown(value) => value,
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
impl Default for BillingBillResourceInvoicingTaxesTaxTaxabilityReason {
    fn default() -> Self {
        BillingBillResourceInvoicingTaxesTaxTaxabilityReason::Unknown(String::new())
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
impl<'json> json::Decode<'json> for BillingBillResourceInvoicingTaxesTaxTaxabilityReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BillingBillResourceInvoicingTaxesTaxTaxabilityReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "customer_exempt" => BillingBillResourceInvoicingTaxesTaxTaxabilityReason::CustomerExempt,
            "not_available" => BillingBillResourceInvoicingTaxesTaxTaxabilityReason::NotAvailable,
            "not_collecting" => BillingBillResourceInvoicingTaxesTaxTaxabilityReason::NotCollecting,
            "not_subject_to_tax" => BillingBillResourceInvoicingTaxesTaxTaxabilityReason::NotSubjectToTax,
            "not_supported" => BillingBillResourceInvoicingTaxesTaxTaxabilityReason::NotSupported,
            "portion_product_exempt" => BillingBillResourceInvoicingTaxesTaxTaxabilityReason::PortionProductExempt,
            "portion_reduced_rated" => BillingBillResourceInvoicingTaxesTaxTaxabilityReason::PortionReducedRated,
            "portion_standard_rated" => BillingBillResourceInvoicingTaxesTaxTaxabilityReason::PortionStandardRated,
            "product_exempt" => BillingBillResourceInvoicingTaxesTaxTaxabilityReason::ProductExempt,
            "product_exempt_holiday" => BillingBillResourceInvoicingTaxesTaxTaxabilityReason::ProductExemptHoliday,
            "proportionally_rated" => BillingBillResourceInvoicingTaxesTaxTaxabilityReason::ProportionallyRated,
            "reduced_rated" => BillingBillResourceInvoicingTaxesTaxTaxabilityReason::ReducedRated,
            "reverse_charge" => BillingBillResourceInvoicingTaxesTaxTaxabilityReason::ReverseCharge,
            "standard_rated" => BillingBillResourceInvoicingTaxesTaxTaxabilityReason::StandardRated,
            "taxable_basis_reduced" => BillingBillResourceInvoicingTaxesTaxTaxabilityReason::TaxableBasisReduced,
            "zero_rated" => BillingBillResourceInvoicingTaxesTaxTaxabilityReason::ZeroRated,
            other => BillingBillResourceInvoicingTaxesTaxTaxabilityReason::Unknown(other.to_string()),
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
pub enum BillingCreditBalanceTransactionObject {
    BillingCreditBalanceTransaction,
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
impl BillingCreditBalanceTransactionObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BillingCreditBalanceTransactionObject::BillingCreditBalanceTransaction => {
                "billing.credit_balance_transaction"
            }
            BillingCreditBalanceTransactionObject::Unknown(value) => value,
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
impl Default for BillingCreditBalanceTransactionObject {
    fn default() -> Self {
        BillingCreditBalanceTransactionObject::Unknown(String::new())
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
impl<'json> json::Decode<'json> for BillingCreditBalanceTransactionObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BillingCreditBalanceTransactionObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "billing.credit_balance_transaction" => {
                BillingCreditBalanceTransactionObject::BillingCreditBalanceTransaction
            }
            other => BillingCreditBalanceTransactionObject::Unknown(other.to_string()),
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
pub enum BillingCreditGrantsResourceBalanceCreditType {
    CreditsApplicationInvoiceVoided,
    CreditsGranted,
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
impl BillingCreditGrantsResourceBalanceCreditType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BillingCreditGrantsResourceBalanceCreditType::CreditsApplicationInvoiceVoided => {
                "credits_application_invoice_voided"
            }
            BillingCreditGrantsResourceBalanceCreditType::CreditsGranted => "credits_granted",
            BillingCreditGrantsResourceBalanceCreditType::Unknown(value) => value,
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
impl Default for BillingCreditGrantsResourceBalanceCreditType {
    fn default() -> Self {
        BillingCreditGrantsResourceBalanceCreditType::Unknown(String::new())
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
impl<'json> json::Decode<'json> for BillingCreditGrantsResourceBalanceCreditType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BillingCreditGrantsResourceBalanceCreditType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "credits_application_invoice_voided" => {
                BillingCreditGrantsResourceBalanceCreditType::CreditsApplicationInvoiceVoided
            }
            "credits_granted" => BillingCreditGrantsResourceBalanceCreditType::CreditsGranted,
            other => BillingCreditGrantsResourceBalanceCreditType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone, Default)]
pub struct CreateBillingCreditGrantParamsAmountMonetary {
    #[doc = "Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) of the `value` parameter."]
    pub currency: String,
    #[doc = "A positive integer representing the amount of the credit grant."]
    pub value: i64,
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for CreateBillingCreditGrantParamsAmountMonetary {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("currency", &self.currency);
        form.field("value", &self.value);
    }
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone, Default)]
pub struct CreateBillingFeedbackOptionParams {
    #[doc = "The text of the feedback option, which customers see when canceling."]
    pub description: String,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for CreateBillingFeedbackOptionParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("description", &self.description);
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone, PartialEq)]
pub enum GetBillingCreditBalanceSummaryParamsFilterType {
    ApplicabilityScope,
    CreditGrant,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "billing")]
impl GetBillingCreditBalanceSummaryParamsFilterType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            GetBillingCreditBalanceSummaryParamsFilterType::ApplicabilityScope => "applicability_scope",
            GetBillingCreditBalanceSummaryParamsFilterType::CreditGrant => "credit_grant",
            GetBillingCreditBalanceSummaryParamsFilterType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "billing")]
impl<'json> json::Decode<'json> for GetBillingCreditBalanceSummaryParamsFilterType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(GetBillingCreditBalanceSummaryParamsFilterType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "applicability_scope" => GetBillingCreditBalanceSummaryParamsFilterType::ApplicabilityScope,
            "credit_grant" => GetBillingCreditBalanceSummaryParamsFilterType::CreditGrant,
            other => GetBillingCreditBalanceSummaryParamsFilterType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for GetBillingCreditBalanceSummaryParamsFilterType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone, Default)]
pub struct ListBillingCreditBalanceTransactionsParams {
    pub credit_grant: Option<String>,
    pub customer: Option<String>,
    pub customer_account: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for ListBillingCreditBalanceTransactionsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("credit_grant", self.credit_grant.as_ref());
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("customer_account", self.customer_account.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListBillingFeedbackOptionsParamsStatus {
    Active,
    Inactive,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "billing")]
impl ListBillingFeedbackOptionsParamsStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListBillingFeedbackOptionsParamsStatus::Active => "active",
            ListBillingFeedbackOptionsParamsStatus::Inactive => "inactive",
            ListBillingFeedbackOptionsParamsStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "billing")]
impl<'json> json::Decode<'json> for ListBillingFeedbackOptionsParamsStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListBillingFeedbackOptionsParamsStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "active" => ListBillingFeedbackOptionsParamsStatus::Active,
            "inactive" => ListBillingFeedbackOptionsParamsStatus::Inactive,
            other => ListBillingFeedbackOptionsParamsStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for ListBillingFeedbackOptionsParamsStatus {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
