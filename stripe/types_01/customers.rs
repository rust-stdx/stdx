#[cfg(feature = "customers")]
#[derive(Debug, Clone, Default)]
pub struct CreateCustomerFundingInstructionParamsBankTransferEuBankTransfer {
    #[doc = "The desired country code of the bank account information."]
    pub country: String,
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for CreateCustomerFundingInstructionParamsBankTransferEuBankTransfer {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("country", &self.country);
    }
}

#[cfg(feature = "customers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCustomerParamsTaxExempt {
    Unnamed,
    Exempt,
    None,
    Reverse,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "customers")]
impl CreateCustomerParamsTaxExempt {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCustomerParamsTaxExempt::Unnamed => "",
            CreateCustomerParamsTaxExempt::Exempt => "exempt",
            CreateCustomerParamsTaxExempt::None => "none",
            CreateCustomerParamsTaxExempt::Reverse => "reverse",
            CreateCustomerParamsTaxExempt::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "customers")]
impl<'json> json::Decode<'json> for CreateCustomerParamsTaxExempt {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCustomerParamsTaxExempt::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreateCustomerParamsTaxExempt::Unnamed,
            "exempt" => CreateCustomerParamsTaxExempt::Exempt,
            "none" => CreateCustomerParamsTaxExempt::None,
            "reverse" => CreateCustomerParamsTaxExempt::Reverse,
            other => CreateCustomerParamsTaxExempt::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for CreateCustomerParamsTaxExempt {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "customers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCustomerParamsTaxValidateLocation {
    Deferred,
    Immediately,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "customers")]
impl CreateCustomerParamsTaxValidateLocation {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCustomerParamsTaxValidateLocation::Deferred => "deferred",
            CreateCustomerParamsTaxValidateLocation::Immediately => "immediately",
            CreateCustomerParamsTaxValidateLocation::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "customers")]
impl<'json> json::Decode<'json> for CreateCustomerParamsTaxValidateLocation {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCustomerParamsTaxValidateLocation::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "deferred" => CreateCustomerParamsTaxValidateLocation::Deferred,
            "immediately" => CreateCustomerParamsTaxValidateLocation::Immediately,
            other => CreateCustomerParamsTaxValidateLocation::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for CreateCustomerParamsTaxValidateLocation {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum CustomerBalanceTransactionObject {
    CustomerBalanceTransaction,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
impl CustomerBalanceTransactionObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CustomerBalanceTransactionObject::CustomerBalanceTransaction => "customer_balance_transaction",
            CustomerBalanceTransactionObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
impl Default for CustomerBalanceTransactionObject {
    fn default() -> Self {
        CustomerBalanceTransactionObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
impl<'json> json::Decode<'json> for CustomerBalanceTransactionObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CustomerBalanceTransactionObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "customer_balance_transaction" => CustomerBalanceTransactionObject::CustomerBalanceTransaction,
            other => CustomerBalanceTransactionObject::Unknown(other.to_string()),
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
pub enum CustomerTaxAutomaticTax {
    Failed,
    NotCollecting,
    Supported,
    UnrecognizedLocation,
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
impl CustomerTaxAutomaticTax {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CustomerTaxAutomaticTax::Failed => "failed",
            CustomerTaxAutomaticTax::NotCollecting => "not_collecting",
            CustomerTaxAutomaticTax::Supported => "supported",
            CustomerTaxAutomaticTax::UnrecognizedLocation => "unrecognized_location",
            CustomerTaxAutomaticTax::Unknown(value) => value,
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
impl Default for CustomerTaxAutomaticTax {
    fn default() -> Self {
        CustomerTaxAutomaticTax::Unknown(String::new())
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
impl<'json> json::Decode<'json> for CustomerTaxAutomaticTax {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CustomerTaxAutomaticTax::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "failed" => CustomerTaxAutomaticTax::Failed,
            "not_collecting" => CustomerTaxAutomaticTax::NotCollecting,
            "supported" => CustomerTaxAutomaticTax::Supported,
            "unrecognized_location" => CustomerTaxAutomaticTax::UnrecognizedLocation,
            other => CustomerTaxAutomaticTax::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "customers")]
#[derive(Debug, Clone, PartialEq)]
pub enum UpdateCustomerSourceParamsAccountHolderType {
    Company,
    Individual,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "customers")]
impl UpdateCustomerSourceParamsAccountHolderType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            UpdateCustomerSourceParamsAccountHolderType::Company => "company",
            UpdateCustomerSourceParamsAccountHolderType::Individual => "individual",
            UpdateCustomerSourceParamsAccountHolderType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "customers")]
impl<'json> json::Decode<'json> for UpdateCustomerSourceParamsAccountHolderType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(UpdateCustomerSourceParamsAccountHolderType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "company" => UpdateCustomerSourceParamsAccountHolderType::Company,
            "individual" => UpdateCustomerSourceParamsAccountHolderType::Individual,
            other => UpdateCustomerSourceParamsAccountHolderType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for UpdateCustomerSourceParamsAccountHolderType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
