#[cfg(feature = "financial_connections")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateFinancialConnectionSessionParamsAccountHolderType {
    Account,
    Customer,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "financial_connections")]
impl CreateFinancialConnectionSessionParamsAccountHolderType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateFinancialConnectionSessionParamsAccountHolderType::Account => "account",
            CreateFinancialConnectionSessionParamsAccountHolderType::Customer => "customer",
            CreateFinancialConnectionSessionParamsAccountHolderType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "financial_connections")]
impl<'json> json::Decode<'json> for CreateFinancialConnectionSessionParamsAccountHolderType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateFinancialConnectionSessionParamsAccountHolderType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account" => CreateFinancialConnectionSessionParamsAccountHolderType::Account,
            "customer" => CreateFinancialConnectionSessionParamsAccountHolderType::Customer,
            other => CreateFinancialConnectionSessionParamsAccountHolderType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "financial_connections")]
impl crate::params::FormEncode for CreateFinancialConnectionSessionParamsAccountHolderType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "financial_connections")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateFinancialConnectionSessionParamsFiltersRequirePaymentMethodSupport {
    All,
    AtLeastOne,
    None,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "financial_connections")]
impl CreateFinancialConnectionSessionParamsFiltersRequirePaymentMethodSupport {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateFinancialConnectionSessionParamsFiltersRequirePaymentMethodSupport::All => "all",
            CreateFinancialConnectionSessionParamsFiltersRequirePaymentMethodSupport::AtLeastOne => "at_least_one",
            CreateFinancialConnectionSessionParamsFiltersRequirePaymentMethodSupport::None => "none",
            CreateFinancialConnectionSessionParamsFiltersRequirePaymentMethodSupport::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "financial_connections")]
impl<'json> json::Decode<'json> for CreateFinancialConnectionSessionParamsFiltersRequirePaymentMethodSupport {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateFinancialConnectionSessionParamsFiltersRequirePaymentMethodSupport::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "all" => CreateFinancialConnectionSessionParamsFiltersRequirePaymentMethodSupport::All,
            "at_least_one" => CreateFinancialConnectionSessionParamsFiltersRequirePaymentMethodSupport::AtLeastOne,
            "none" => CreateFinancialConnectionSessionParamsFiltersRequirePaymentMethodSupport::None,
            other => {
                CreateFinancialConnectionSessionParamsFiltersRequirePaymentMethodSupport::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "financial_connections")]
impl crate::params::FormEncode for CreateFinancialConnectionSessionParamsFiltersRequirePaymentMethodSupport {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
#[derive(Debug, Clone, PartialEq)]
pub enum FinancialConnectionsAccountSubscriptionsItem {
    Transactions,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl FinancialConnectionsAccountSubscriptionsItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            FinancialConnectionsAccountSubscriptionsItem::Transactions => "transactions",
            FinancialConnectionsAccountSubscriptionsItem::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl<'json> json::Decode<'json> for FinancialConnectionsAccountSubscriptionsItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(FinancialConnectionsAccountSubscriptionsItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "transactions" => FinancialConnectionsAccountSubscriptionsItem::Transactions,
            other => FinancialConnectionsAccountSubscriptionsItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "financial_connections")]
#[derive(Debug, Clone, PartialEq)]
pub enum FinancialConnectionsTransactionStatus {
    Pending,
    Posted,
    Void,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "financial_connections")]
impl FinancialConnectionsTransactionStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            FinancialConnectionsTransactionStatus::Pending => "pending",
            FinancialConnectionsTransactionStatus::Posted => "posted",
            FinancialConnectionsTransactionStatus::Void => "void",
            FinancialConnectionsTransactionStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "financial_connections")]
impl Default for FinancialConnectionsTransactionStatus {
    fn default() -> Self {
        FinancialConnectionsTransactionStatus::Unknown(String::new())
    }
}

#[cfg(feature = "financial_connections")]
impl<'json> json::Decode<'json> for FinancialConnectionsTransactionStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(FinancialConnectionsTransactionStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "pending" => FinancialConnectionsTransactionStatus::Pending,
            "posted" => FinancialConnectionsTransactionStatus::Posted,
            "void" => FinancialConnectionsTransactionStatus::Void,
            other => FinancialConnectionsTransactionStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "financial_connections")]
#[derive(Debug, Clone, Default)]
pub struct ListFinancialConnectionAccountsParamsAccountHolder {
    #[doc = "The ID of the Stripe account whose accounts you will retrieve."]
    pub account: Option<String>,
    #[doc = "The ID of the Stripe customer whose accounts you will retrieve."]
    pub customer: Option<String>,
    #[doc = "The ID of the Account representing a customer whose accounts you will retrieve."]
    pub customer_account: Option<String>,
}

#[cfg(feature = "financial_connections")]
impl crate::params::FormEncode for ListFinancialConnectionAccountsParamsAccountHolder {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("customer_account", self.customer_account.as_ref());
    }
}
