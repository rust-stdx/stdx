#[cfg(feature = "financial_connections")]
#[derive(Debug, Clone, Default)]
pub struct CreateFinancialConnectionSessionParamsLimits {
    #[doc = "The number of accounts that can be linked in this Session."]
    pub accounts: json::OwnedValue,
}

#[cfg(feature = "financial_connections")]
impl crate::params::FormEncode for CreateFinancialConnectionSessionParamsLimits {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        let _ = form;
    }
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
#[derive(Debug, Clone, PartialEq)]
pub enum FinancialConnectionsAccountOwnerObject {
    FinancialConnectionsAccountOwner,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl FinancialConnectionsAccountOwnerObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            FinancialConnectionsAccountOwnerObject::FinancialConnectionsAccountOwner => {
                "financial_connections.account_owner"
            }
            FinancialConnectionsAccountOwnerObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl Default for FinancialConnectionsAccountOwnerObject {
    fn default() -> Self {
        FinancialConnectionsAccountOwnerObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl<'json> json::Decode<'json> for FinancialConnectionsAccountOwnerObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(FinancialConnectionsAccountOwnerObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "financial_connections.account_owner" => {
                FinancialConnectionsAccountOwnerObject::FinancialConnectionsAccountOwner
            }
            other => FinancialConnectionsAccountOwnerObject::Unknown(other.to_string()),
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
pub enum FinancialConnectionsAccountPermissionsItem {
    Balances,
    Ownership,
    PaymentMethod,
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
impl FinancialConnectionsAccountPermissionsItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            FinancialConnectionsAccountPermissionsItem::Balances => "balances",
            FinancialConnectionsAccountPermissionsItem::Ownership => "ownership",
            FinancialConnectionsAccountPermissionsItem::PaymentMethod => "payment_method",
            FinancialConnectionsAccountPermissionsItem::Transactions => "transactions",
            FinancialConnectionsAccountPermissionsItem::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for FinancialConnectionsAccountPermissionsItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(FinancialConnectionsAccountPermissionsItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "balances" => FinancialConnectionsAccountPermissionsItem::Balances,
            "ownership" => FinancialConnectionsAccountPermissionsItem::Ownership,
            "payment_method" => FinancialConnectionsAccountPermissionsItem::PaymentMethod,
            "transactions" => FinancialConnectionsAccountPermissionsItem::Transactions,
            other => FinancialConnectionsAccountPermissionsItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
#[derive(Debug, Clone, PartialEq)]
pub enum FinancialConnectionsAccountSupportedPaymentMethodTypesItem {
    Link,
    UsBankAccount,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl FinancialConnectionsAccountSupportedPaymentMethodTypesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            FinancialConnectionsAccountSupportedPaymentMethodTypesItem::Link => "link",
            FinancialConnectionsAccountSupportedPaymentMethodTypesItem::UsBankAccount => "us_bank_account",
            FinancialConnectionsAccountSupportedPaymentMethodTypesItem::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl<'json> json::Decode<'json> for FinancialConnectionsAccountSupportedPaymentMethodTypesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(FinancialConnectionsAccountSupportedPaymentMethodTypesItem::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "link" => FinancialConnectionsAccountSupportedPaymentMethodTypesItem::Link,
            "us_bank_account" => FinancialConnectionsAccountSupportedPaymentMethodTypesItem::UsBankAccount,
            other => FinancialConnectionsAccountSupportedPaymentMethodTypesItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum FinancialConnectionsAuthorizationStatus {
    Active,
    Inactive,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl FinancialConnectionsAuthorizationStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            FinancialConnectionsAuthorizationStatus::Active => "active",
            FinancialConnectionsAuthorizationStatus::Inactive => "inactive",
            FinancialConnectionsAuthorizationStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for FinancialConnectionsAuthorizationStatus {
    fn default() -> Self {
        FinancialConnectionsAuthorizationStatus::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for FinancialConnectionsAuthorizationStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(FinancialConnectionsAuthorizationStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "active" => FinancialConnectionsAuthorizationStatus::Active,
            "inactive" => FinancialConnectionsAuthorizationStatus::Inactive,
            other => FinancialConnectionsAuthorizationStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "financial_connections")]
#[derive(Debug, Clone, PartialEq)]
pub enum FinancialConnectionsSessionObject {
    FinancialConnectionsSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "financial_connections")]
impl FinancialConnectionsSessionObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            FinancialConnectionsSessionObject::FinancialConnectionsSession => "financial_connections.session",
            FinancialConnectionsSessionObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "financial_connections")]
impl Default for FinancialConnectionsSessionObject {
    fn default() -> Self {
        FinancialConnectionsSessionObject::Unknown(String::new())
    }
}

#[cfg(feature = "financial_connections")]
impl<'json> json::Decode<'json> for FinancialConnectionsSessionObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(FinancialConnectionsSessionObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "financial_connections.session" => FinancialConnectionsSessionObject::FinancialConnectionsSession,
            other => FinancialConnectionsSessionObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "financial_connections")]
#[derive(Debug, Clone, PartialEq)]
pub enum SubscribeFinancialConnectionAccountParamsFeaturesItem {
    Transactions,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "financial_connections")]
impl SubscribeFinancialConnectionAccountParamsFeaturesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SubscribeFinancialConnectionAccountParamsFeaturesItem::Transactions => "transactions",
            SubscribeFinancialConnectionAccountParamsFeaturesItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "financial_connections")]
impl<'json> json::Decode<'json> for SubscribeFinancialConnectionAccountParamsFeaturesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SubscribeFinancialConnectionAccountParamsFeaturesItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "transactions" => SubscribeFinancialConnectionAccountParamsFeaturesItem::Transactions,
            other => SubscribeFinancialConnectionAccountParamsFeaturesItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "financial_connections")]
impl crate::params::FormEncode for SubscribeFinancialConnectionAccountParamsFeaturesItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
