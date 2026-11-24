#[cfg(feature = "financial_connections")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateFinancialConnectionSessionParamsFiltersAccountSubcategoriesItem {
    Checking,
    CreditCard,
    LineOfCredit,
    Mortgage,
    Savings,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "financial_connections")]
impl CreateFinancialConnectionSessionParamsFiltersAccountSubcategoriesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateFinancialConnectionSessionParamsFiltersAccountSubcategoriesItem::Checking => "checking",
            CreateFinancialConnectionSessionParamsFiltersAccountSubcategoriesItem::CreditCard => "credit_card",
            CreateFinancialConnectionSessionParamsFiltersAccountSubcategoriesItem::LineOfCredit => "line_of_credit",
            CreateFinancialConnectionSessionParamsFiltersAccountSubcategoriesItem::Mortgage => "mortgage",
            CreateFinancialConnectionSessionParamsFiltersAccountSubcategoriesItem::Savings => "savings",
            CreateFinancialConnectionSessionParamsFiltersAccountSubcategoriesItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "financial_connections")]
impl<'json> json::Decode<'json> for CreateFinancialConnectionSessionParamsFiltersAccountSubcategoriesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateFinancialConnectionSessionParamsFiltersAccountSubcategoriesItem::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "checking" => CreateFinancialConnectionSessionParamsFiltersAccountSubcategoriesItem::Checking,
            "credit_card" => CreateFinancialConnectionSessionParamsFiltersAccountSubcategoriesItem::CreditCard,
            "line_of_credit" => CreateFinancialConnectionSessionParamsFiltersAccountSubcategoriesItem::LineOfCredit,
            "mortgage" => CreateFinancialConnectionSessionParamsFiltersAccountSubcategoriesItem::Mortgage,
            "savings" => CreateFinancialConnectionSessionParamsFiltersAccountSubcategoriesItem::Savings,
            other => CreateFinancialConnectionSessionParamsFiltersAccountSubcategoriesItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "financial_connections")]
impl crate::params::FormEncode for CreateFinancialConnectionSessionParamsFiltersAccountSubcategoriesItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "financial_connections")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateFinancialConnectionSessionParamsPermissionsItem {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "financial_connections")]
impl CreateFinancialConnectionSessionParamsPermissionsItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateFinancialConnectionSessionParamsPermissionsItem::Balances => "balances",
            CreateFinancialConnectionSessionParamsPermissionsItem::Ownership => "ownership",
            CreateFinancialConnectionSessionParamsPermissionsItem::PaymentMethod => "payment_method",
            CreateFinancialConnectionSessionParamsPermissionsItem::Transactions => "transactions",
            CreateFinancialConnectionSessionParamsPermissionsItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "financial_connections")]
impl<'json> json::Decode<'json> for CreateFinancialConnectionSessionParamsPermissionsItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateFinancialConnectionSessionParamsPermissionsItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "balances" => CreateFinancialConnectionSessionParamsPermissionsItem::Balances,
            "ownership" => CreateFinancialConnectionSessionParamsPermissionsItem::Ownership,
            "payment_method" => CreateFinancialConnectionSessionParamsPermissionsItem::PaymentMethod,
            "transactions" => CreateFinancialConnectionSessionParamsPermissionsItem::Transactions,
            other => CreateFinancialConnectionSessionParamsPermissionsItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "financial_connections")]
impl crate::params::FormEncode for CreateFinancialConnectionSessionParamsPermissionsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
#[derive(Debug, Clone, PartialEq)]
pub enum FinancialConnectionsAccountCategory {
    Cash,
    Credit,
    Investment,
    Other,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl FinancialConnectionsAccountCategory {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            FinancialConnectionsAccountCategory::Cash => "cash",
            FinancialConnectionsAccountCategory::Credit => "credit",
            FinancialConnectionsAccountCategory::Investment => "investment",
            FinancialConnectionsAccountCategory::Other => "other",
            FinancialConnectionsAccountCategory::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl Default for FinancialConnectionsAccountCategory {
    fn default() -> Self {
        FinancialConnectionsAccountCategory::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl<'json> json::Decode<'json> for FinancialConnectionsAccountCategory {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(FinancialConnectionsAccountCategory::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "cash" => FinancialConnectionsAccountCategory::Cash,
            "credit" => FinancialConnectionsAccountCategory::Credit,
            "investment" => FinancialConnectionsAccountCategory::Investment,
            "other" => FinancialConnectionsAccountCategory::Other,
            other => FinancialConnectionsAccountCategory::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
#[derive(Debug, Clone, PartialEq)]
pub enum FinancialConnectionsAccountObject {
    FinancialConnectionsAccount,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl FinancialConnectionsAccountObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            FinancialConnectionsAccountObject::FinancialConnectionsAccount => "financial_connections.account",
            FinancialConnectionsAccountObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl Default for FinancialConnectionsAccountObject {
    fn default() -> Self {
        FinancialConnectionsAccountObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl<'json> json::Decode<'json> for FinancialConnectionsAccountObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(FinancialConnectionsAccountObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "financial_connections.account" => FinancialConnectionsAccountObject::FinancialConnectionsAccount,
            other => FinancialConnectionsAccountObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
#[derive(Debug, Clone, PartialEq)]
pub enum FinancialConnectionsAccountStatus {
    Active,
    Disconnected,
    Inactive,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl FinancialConnectionsAccountStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            FinancialConnectionsAccountStatus::Active => "active",
            FinancialConnectionsAccountStatus::Disconnected => "disconnected",
            FinancialConnectionsAccountStatus::Inactive => "inactive",
            FinancialConnectionsAccountStatus::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl Default for FinancialConnectionsAccountStatus {
    fn default() -> Self {
        FinancialConnectionsAccountStatus::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl<'json> json::Decode<'json> for FinancialConnectionsAccountStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(FinancialConnectionsAccountStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "active" => FinancialConnectionsAccountStatus::Active,
            "disconnected" => FinancialConnectionsAccountStatus::Disconnected,
            "inactive" => FinancialConnectionsAccountStatus::Inactive,
            other => FinancialConnectionsAccountStatus::Unknown(other.to_string()),
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
pub enum FinancialConnectionsSessionPrefetchItem {
    Balances,
    Ownership,
    Transactions,
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
impl FinancialConnectionsSessionPrefetchItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            FinancialConnectionsSessionPrefetchItem::Balances => "balances",
            FinancialConnectionsSessionPrefetchItem::Ownership => "ownership",
            FinancialConnectionsSessionPrefetchItem::Transactions => "transactions",
            FinancialConnectionsSessionPrefetchItem::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for FinancialConnectionsSessionPrefetchItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(FinancialConnectionsSessionPrefetchItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "balances" => FinancialConnectionsSessionPrefetchItem::Balances,
            "ownership" => FinancialConnectionsSessionPrefetchItem::Ownership,
            "transactions" => FinancialConnectionsSessionPrefetchItem::Transactions,
            other => FinancialConnectionsSessionPrefetchItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "financial_connections")]
#[derive(Debug, Clone, PartialEq)]
pub enum FinancialConnectionsTransactionObject {
    FinancialConnectionsTransaction,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "financial_connections")]
impl FinancialConnectionsTransactionObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            FinancialConnectionsTransactionObject::FinancialConnectionsTransaction => {
                "financial_connections.transaction"
            }
            FinancialConnectionsTransactionObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "financial_connections")]
impl Default for FinancialConnectionsTransactionObject {
    fn default() -> Self {
        FinancialConnectionsTransactionObject::Unknown(String::new())
    }
}

#[cfg(feature = "financial_connections")]
impl<'json> json::Decode<'json> for FinancialConnectionsTransactionObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(FinancialConnectionsTransactionObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "financial_connections.transaction" => {
                FinancialConnectionsTransactionObject::FinancialConnectionsTransaction
            }
            other => FinancialConnectionsTransactionObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "financial_connections")]
#[derive(Debug, Clone, Default)]
pub struct ListFinancialConnectionTransactionsParamsTransactedAt {
    #[doc = "Minimum value to filter by (exclusive)"]
    pub gt: Option<i64>,
    #[doc = "Minimum value to filter by (inclusive)"]
    pub gte: Option<i64>,
    #[doc = "Maximum value to filter by (exclusive)"]
    pub lt: Option<i64>,
    #[doc = "Maximum value to filter by (inclusive)"]
    pub lte: Option<i64>,
}

#[cfg(feature = "financial_connections")]
impl crate::params::FormEncode for ListFinancialConnectionTransactionsParamsTransactedAt {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("gt", self.gt.as_ref());
        form.field_opt("gte", self.gte.as_ref());
        form.field_opt("lt", self.lt.as_ref());
        form.field_opt("lte", self.lte.as_ref());
    }
}
