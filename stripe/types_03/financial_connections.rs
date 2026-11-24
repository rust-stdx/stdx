#[cfg(feature = "financial_connections")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateFinancialConnectionSessionParamsPrefetchItem {
    Balances,
    Ownership,
    Transactions,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "financial_connections")]
impl CreateFinancialConnectionSessionParamsPrefetchItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateFinancialConnectionSessionParamsPrefetchItem::Balances => "balances",
            CreateFinancialConnectionSessionParamsPrefetchItem::Ownership => "ownership",
            CreateFinancialConnectionSessionParamsPrefetchItem::Transactions => "transactions",
            CreateFinancialConnectionSessionParamsPrefetchItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "financial_connections")]
impl<'json> json::Decode<'json> for CreateFinancialConnectionSessionParamsPrefetchItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateFinancialConnectionSessionParamsPrefetchItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "balances" => CreateFinancialConnectionSessionParamsPrefetchItem::Balances,
            "ownership" => CreateFinancialConnectionSessionParamsPrefetchItem::Ownership,
            "transactions" => CreateFinancialConnectionSessionParamsPrefetchItem::Transactions,
            other => CreateFinancialConnectionSessionParamsPrefetchItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "financial_connections")]
impl crate::params::FormEncode for CreateFinancialConnectionSessionParamsPrefetchItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "financial_connections")]
#[derive(Debug, Clone, Default)]
pub struct GetFinancialConnectionAccountParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "financial_connections")]
impl crate::params::FormEncode for GetFinancialConnectionAccountParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}

#[cfg(feature = "financial_connections")]
#[derive(Debug, Clone, Default)]
pub struct ListFinancialConnectionAccountOwnersParams {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub ownership: String,
    pub starting_after: Option<String>,
}

#[cfg(feature = "financial_connections")]
impl crate::params::FormEncode for ListFinancialConnectionAccountOwnersParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field("ownership", &self.ownership);
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}
