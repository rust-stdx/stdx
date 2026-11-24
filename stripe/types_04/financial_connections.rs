#[cfg(feature = "financial_connections")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateFinancialConnectionSessionParamsManualEntryMode {
    Automatic,
    Disabled,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "financial_connections")]
impl CreateFinancialConnectionSessionParamsManualEntryMode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateFinancialConnectionSessionParamsManualEntryMode::Automatic => "automatic",
            CreateFinancialConnectionSessionParamsManualEntryMode::Disabled => "disabled",
            CreateFinancialConnectionSessionParamsManualEntryMode::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "financial_connections")]
impl<'json> json::Decode<'json> for CreateFinancialConnectionSessionParamsManualEntryMode {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateFinancialConnectionSessionParamsManualEntryMode::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "automatic" => CreateFinancialConnectionSessionParamsManualEntryMode::Automatic,
            "disabled" => CreateFinancialConnectionSessionParamsManualEntryMode::Disabled,
            other => CreateFinancialConnectionSessionParamsManualEntryMode::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "financial_connections")]
impl crate::params::FormEncode for CreateFinancialConnectionSessionParamsManualEntryMode {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
#[derive(Debug, Clone, PartialEq)]
pub enum FinancialConnectionsAccountOwnershipObject {
    FinancialConnectionsAccountOwnership,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl FinancialConnectionsAccountOwnershipObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            FinancialConnectionsAccountOwnershipObject::FinancialConnectionsAccountOwnership => {
                "financial_connections.account_ownership"
            }
            FinancialConnectionsAccountOwnershipObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl Default for FinancialConnectionsAccountOwnershipObject {
    fn default() -> Self {
        FinancialConnectionsAccountOwnershipObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl<'json> json::Decode<'json> for FinancialConnectionsAccountOwnershipObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(FinancialConnectionsAccountOwnershipObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "financial_connections.account_ownership" => {
                FinancialConnectionsAccountOwnershipObject::FinancialConnectionsAccountOwnership
            }
            other => FinancialConnectionsAccountOwnershipObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
#[derive(Debug, Clone, PartialEq)]
pub enum FinancialConnectionsAccountSubcategory {
    Checking,
    CreditCard,
    LineOfCredit,
    Mortgage,
    Other,
    Savings,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl FinancialConnectionsAccountSubcategory {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            FinancialConnectionsAccountSubcategory::Checking => "checking",
            FinancialConnectionsAccountSubcategory::CreditCard => "credit_card",
            FinancialConnectionsAccountSubcategory::LineOfCredit => "line_of_credit",
            FinancialConnectionsAccountSubcategory::Mortgage => "mortgage",
            FinancialConnectionsAccountSubcategory::Other => "other",
            FinancialConnectionsAccountSubcategory::Savings => "savings",
            FinancialConnectionsAccountSubcategory::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl Default for FinancialConnectionsAccountSubcategory {
    fn default() -> Self {
        FinancialConnectionsAccountSubcategory::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl<'json> json::Decode<'json> for FinancialConnectionsAccountSubcategory {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(FinancialConnectionsAccountSubcategory::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "checking" => FinancialConnectionsAccountSubcategory::Checking,
            "credit_card" => FinancialConnectionsAccountSubcategory::CreditCard,
            "line_of_credit" => FinancialConnectionsAccountSubcategory::LineOfCredit,
            "mortgage" => FinancialConnectionsAccountSubcategory::Mortgage,
            "other" => FinancialConnectionsAccountSubcategory::Other,
            "savings" => FinancialConnectionsAccountSubcategory::Savings,
            other => FinancialConnectionsAccountSubcategory::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum FinancialConnectionsAuthorizationObject {
    FinancialConnectionsAuthorization,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl FinancialConnectionsAuthorizationObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            FinancialConnectionsAuthorizationObject::FinancialConnectionsAuthorization => {
                "financial_connections.authorization"
            }
            FinancialConnectionsAuthorizationObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for FinancialConnectionsAuthorizationObject {
    fn default() -> Self {
        FinancialConnectionsAuthorizationObject::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for FinancialConnectionsAuthorizationObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(FinancialConnectionsAuthorizationObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "financial_connections.authorization" => {
                FinancialConnectionsAuthorizationObject::FinancialConnectionsAuthorization
            }
            other => FinancialConnectionsAuthorizationObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "financial_connections")]
#[derive(Debug, Clone, Default)]
pub struct ListFinancialConnectionTransactionsParamsTransactionRefresh {
    #[doc = "Return results where the transactions were created or updated by a refresh that took place after this refresh (non-in..."]
    pub after: String,
}

#[cfg(feature = "financial_connections")]
impl crate::params::FormEncode for ListFinancialConnectionTransactionsParamsTransactionRefresh {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("after", &self.after);
    }
}

#[cfg(feature = "financial_connections")]
#[derive(Debug, Clone, PartialEq)]
pub enum RefreshFinancialConnectionAccountParamsFeaturesItem {
    Balance,
    Ownership,
    Transactions,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "financial_connections")]
impl RefreshFinancialConnectionAccountParamsFeaturesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            RefreshFinancialConnectionAccountParamsFeaturesItem::Balance => "balance",
            RefreshFinancialConnectionAccountParamsFeaturesItem::Ownership => "ownership",
            RefreshFinancialConnectionAccountParamsFeaturesItem::Transactions => "transactions",
            RefreshFinancialConnectionAccountParamsFeaturesItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "financial_connections")]
impl<'json> json::Decode<'json> for RefreshFinancialConnectionAccountParamsFeaturesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(RefreshFinancialConnectionAccountParamsFeaturesItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "balance" => RefreshFinancialConnectionAccountParamsFeaturesItem::Balance,
            "ownership" => RefreshFinancialConnectionAccountParamsFeaturesItem::Ownership,
            "transactions" => RefreshFinancialConnectionAccountParamsFeaturesItem::Transactions,
            other => RefreshFinancialConnectionAccountParamsFeaturesItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "financial_connections")]
impl crate::params::FormEncode for RefreshFinancialConnectionAccountParamsFeaturesItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
