#[cfg(feature = "treasury")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataType {
    FinancialAccount,
    UsBankAccount,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "treasury")]
impl CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataType::FinancialAccount => {
                "financial_account"
            }
            CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataType::UsBankAccount => "us_bank_account",
            CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "treasury")]
impl<'json> json::Decode<'json> for CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "financial_account" => {
                CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataType::FinancialAccount
            }
            "us_bank_account" => CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataType::UsBankAccount,
            other => CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodOptionsUsBankAccountNetwork {
    Ach,
    UsDomesticWire,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "treasury")]
impl CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodOptionsUsBankAccountNetwork {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodOptionsUsBankAccountNetwork::Ach => "ach",
            CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodOptionsUsBankAccountNetwork::UsDomesticWire => {
                "us_domestic_wire"
            }
            CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodOptionsUsBankAccountNetwork::Unknown(value) => {
                value
            }
        }
    }
}

#[cfg(feature = "treasury")]
impl<'json> json::Decode<'json>
    for CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodOptionsUsBankAccountNetwork
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodOptionsUsBankAccountNetwork::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ach" => CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodOptionsUsBankAccountNetwork::Ach,
            "us_domestic_wire" => {
                CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodOptionsUsBankAccountNetwork::UsDomesticWire
            }
            other => CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodOptionsUsBankAccountNetwork::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode
    for CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodOptionsUsBankAccountNetwork
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    Company,
    Individual,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType::Company => "company",
            InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType::Individual => "individual",
            InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "company" => InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType::Company,
            "individual" => InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType::Individual,
            other => InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    Ach,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InboundTransfersPaymentMethodDetailsUsBankAccountNetwork::Ach => "ach",
            InboundTransfersPaymentMethodDetailsUsBankAccountNetwork::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl Default for InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    fn default() -> Self {
        InboundTransfersPaymentMethodDetailsUsBankAccountNetwork::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for InboundTransfersPaymentMethodDetailsUsBankAccountNetwork {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InboundTransfersPaymentMethodDetailsUsBankAccountNetwork::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ach" => InboundTransfersPaymentMethodDetailsUsBankAccountNetwork::Ach,
            other => InboundTransfersPaymentMethodDetailsUsBankAccountNetwork::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListTreasuryDebitReversalsParamsResolution {
    Lost,
    Won,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "treasury")]
impl ListTreasuryDebitReversalsParamsResolution {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListTreasuryDebitReversalsParamsResolution::Lost => "lost",
            ListTreasuryDebitReversalsParamsResolution::Won => "won",
            ListTreasuryDebitReversalsParamsResolution::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "treasury")]
impl<'json> json::Decode<'json> for ListTreasuryDebitReversalsParamsResolution {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListTreasuryDebitReversalsParamsResolution::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "lost" => ListTreasuryDebitReversalsParamsResolution::Lost,
            "won" => ListTreasuryDebitReversalsParamsResolution::Won,
            other => ListTreasuryDebitReversalsParamsResolution::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for ListTreasuryDebitReversalsParamsResolution {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListTreasuryDebitReversalsParamsStatus {
    Canceled,
    Completed,
    Processing,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "treasury")]
impl ListTreasuryDebitReversalsParamsStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListTreasuryDebitReversalsParamsStatus::Canceled => "canceled",
            ListTreasuryDebitReversalsParamsStatus::Completed => "completed",
            ListTreasuryDebitReversalsParamsStatus::Processing => "processing",
            ListTreasuryDebitReversalsParamsStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "treasury")]
impl<'json> json::Decode<'json> for ListTreasuryDebitReversalsParamsStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListTreasuryDebitReversalsParamsStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "canceled" => ListTreasuryDebitReversalsParamsStatus::Canceled,
            "completed" => ListTreasuryDebitReversalsParamsStatus::Completed,
            "processing" => ListTreasuryDebitReversalsParamsStatus::Processing,
            other => ListTreasuryDebitReversalsParamsStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for ListTreasuryDebitReversalsParamsStatus {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListTreasuryOutboundPaymentsParamsStatus {
    Canceled,
    Failed,
    Posted,
    Processing,
    Returned,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "treasury")]
impl ListTreasuryOutboundPaymentsParamsStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListTreasuryOutboundPaymentsParamsStatus::Canceled => "canceled",
            ListTreasuryOutboundPaymentsParamsStatus::Failed => "failed",
            ListTreasuryOutboundPaymentsParamsStatus::Posted => "posted",
            ListTreasuryOutboundPaymentsParamsStatus::Processing => "processing",
            ListTreasuryOutboundPaymentsParamsStatus::Returned => "returned",
            ListTreasuryOutboundPaymentsParamsStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "treasury")]
impl<'json> json::Decode<'json> for ListTreasuryOutboundPaymentsParamsStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListTreasuryOutboundPaymentsParamsStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "canceled" => ListTreasuryOutboundPaymentsParamsStatus::Canceled,
            "failed" => ListTreasuryOutboundPaymentsParamsStatus::Failed,
            "posted" => ListTreasuryOutboundPaymentsParamsStatus::Posted,
            "processing" => ListTreasuryOutboundPaymentsParamsStatus::Processing,
            "returned" => ListTreasuryOutboundPaymentsParamsStatus::Returned,
            other => ListTreasuryOutboundPaymentsParamsStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for ListTreasuryOutboundPaymentsParamsStatus {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListTreasuryReceivedCreditsParamsLinkedFlowsSourceFlowType {
    CreditReversal,
    Other,
    OutboundPayment,
    OutboundTransfer,
    Payout,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "treasury")]
impl ListTreasuryReceivedCreditsParamsLinkedFlowsSourceFlowType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListTreasuryReceivedCreditsParamsLinkedFlowsSourceFlowType::CreditReversal => "credit_reversal",
            ListTreasuryReceivedCreditsParamsLinkedFlowsSourceFlowType::Other => "other",
            ListTreasuryReceivedCreditsParamsLinkedFlowsSourceFlowType::OutboundPayment => "outbound_payment",
            ListTreasuryReceivedCreditsParamsLinkedFlowsSourceFlowType::OutboundTransfer => "outbound_transfer",
            ListTreasuryReceivedCreditsParamsLinkedFlowsSourceFlowType::Payout => "payout",
            ListTreasuryReceivedCreditsParamsLinkedFlowsSourceFlowType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "treasury")]
impl<'json> json::Decode<'json> for ListTreasuryReceivedCreditsParamsLinkedFlowsSourceFlowType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListTreasuryReceivedCreditsParamsLinkedFlowsSourceFlowType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "credit_reversal" => ListTreasuryReceivedCreditsParamsLinkedFlowsSourceFlowType::CreditReversal,
            "other" => ListTreasuryReceivedCreditsParamsLinkedFlowsSourceFlowType::Other,
            "outbound_payment" => ListTreasuryReceivedCreditsParamsLinkedFlowsSourceFlowType::OutboundPayment,
            "outbound_transfer" => ListTreasuryReceivedCreditsParamsLinkedFlowsSourceFlowType::OutboundTransfer,
            "payout" => ListTreasuryReceivedCreditsParamsLinkedFlowsSourceFlowType::Payout,
            other => ListTreasuryReceivedCreditsParamsLinkedFlowsSourceFlowType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for ListTreasuryReceivedCreditsParamsLinkedFlowsSourceFlowType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
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
pub enum ReserveTransactionObject {
    ReserveTransaction,
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
impl ReserveTransactionObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ReserveTransactionObject::ReserveTransaction => "reserve_transaction",
            ReserveTransactionObject::Unknown(value) => value,
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
impl Default for ReserveTransactionObject {
    fn default() -> Self {
        ReserveTransactionObject::Unknown(String::new())
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
impl<'json> json::Decode<'json> for ReserveTransactionObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ReserveTransactionObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "reserve_transaction" => ReserveTransactionObject::ReserveTransaction,
            other => ReserveTransactionObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryDebitReversalNetwork {
    Ach,
    Card,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl TreasuryDebitReversalNetwork {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryDebitReversalNetwork::Ach => "ach",
            TreasuryDebitReversalNetwork::Card => "card",
            TreasuryDebitReversalNetwork::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl Default for TreasuryDebitReversalNetwork {
    fn default() -> Self {
        TreasuryDebitReversalNetwork::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryDebitReversalNetwork {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryDebitReversalNetwork::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ach" => TreasuryDebitReversalNetwork::Ach,
            "card" => TreasuryDebitReversalNetwork::Card,
            other => TreasuryDebitReversalNetwork::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryFinancialAccountObject {
    TreasuryFinancialAccount,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "treasury"))]
impl TreasuryFinancialAccountObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryFinancialAccountObject::TreasuryFinancialAccount => "treasury.financial_account",
            TreasuryFinancialAccountObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
impl Default for TreasuryFinancialAccountObject {
    fn default() -> Self {
        TreasuryFinancialAccountObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryFinancialAccountObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryFinancialAccountObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "treasury.financial_account" => TreasuryFinancialAccountObject::TreasuryFinancialAccount,
            other => TreasuryFinancialAccountObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryFinancialAccountStatus {
    Closed,
    Open,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "treasury"))]
impl TreasuryFinancialAccountStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryFinancialAccountStatus::Closed => "closed",
            TreasuryFinancialAccountStatus::Open => "open",
            TreasuryFinancialAccountStatus::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
impl Default for TreasuryFinancialAccountStatus {
    fn default() -> Self {
        TreasuryFinancialAccountStatus::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryFinancialAccountStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryFinancialAccountStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "closed" => TreasuryFinancialAccountStatus::Closed,
            "open" => TreasuryFinancialAccountStatus::Open,
            other => TreasuryFinancialAccountStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryFinancialAccountsResourceAbaRecord {
    #[doc = "The name of the person or business that owns the bank account."]
    #[json(default)]
    pub account_holder_name: String,
    #[doc = "The account number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    #[doc = "The last four characters of the account number."]
    #[json(default)]
    pub account_number_last4: String,
    #[doc = "Name of the bank."]
    #[json(default)]
    pub bank_name: String,
    #[doc = "Routing number for the account."]
    #[json(default)]
    pub routing_number: String,
}

#[cfg(any(feature = "events", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows {
    Restricted,
    Unrestricted,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "treasury"))]
impl TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows::Restricted => "restricted",
            TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows::Unrestricted => "unrestricted",
            TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "restricted" => TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows::Restricted,
            "unrestricted" => TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows::Unrestricted,
            other => TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryInboundTransferStatus {
    Canceled,
    Failed,
    Processing,
    Succeeded,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl TreasuryInboundTransferStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryInboundTransferStatus::Canceled => "canceled",
            TreasuryInboundTransferStatus::Failed => "failed",
            TreasuryInboundTransferStatus::Processing => "processing",
            TreasuryInboundTransferStatus::Succeeded => "succeeded",
            TreasuryInboundTransferStatus::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl Default for TreasuryInboundTransferStatus {
    fn default() -> Self {
        TreasuryInboundTransferStatus::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryInboundTransferStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryInboundTransferStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "canceled" => TreasuryInboundTransferStatus::Canceled,
            "failed" => TreasuryInboundTransferStatus::Failed,
            "processing" => TreasuryInboundTransferStatus::Processing,
            "succeeded" => TreasuryInboundTransferStatus::Succeeded,
            other => TreasuryInboundTransferStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows {
    #[doc = "If funds for this flow were returned after the flow went to the `succeeded` state, this field contains a reference to..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub received_debit: Option<String>,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryOutboundPaymentStatus {
    Canceled,
    Failed,
    Posted,
    Processing,
    Returned,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl TreasuryOutboundPaymentStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryOutboundPaymentStatus::Canceled => "canceled",
            TreasuryOutboundPaymentStatus::Failed => "failed",
            TreasuryOutboundPaymentStatus::Posted => "posted",
            TreasuryOutboundPaymentStatus::Processing => "processing",
            TreasuryOutboundPaymentStatus::Returned => "returned",
            TreasuryOutboundPaymentStatus::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl Default for TreasuryOutboundPaymentStatus {
    fn default() -> Self {
        TreasuryOutboundPaymentStatus::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryOutboundPaymentStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryOutboundPaymentStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "canceled" => TreasuryOutboundPaymentStatus::Canceled,
            "failed" => TreasuryOutboundPaymentStatus::Failed,
            "posted" => TreasuryOutboundPaymentStatus::Posted,
            "processing" => TreasuryOutboundPaymentStatus::Processing,
            "returned" => TreasuryOutboundPaymentStatus::Returned,
            other => TreasuryOutboundPaymentStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitions {
    #[doc = "Timestamp describing when an OutboundPayment changed status to `canceled`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<i64>,
    #[doc = "Timestamp describing when an OutboundPayment changed status to `failed`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<i64>,
    #[doc = "Timestamp describing when an OutboundPayment changed status to `posted`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub posted_at: Option<i64>,
    #[doc = "Timestamp describing when an OutboundPayment changed status to `returned`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub returned_at: Option<i64>,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryOutboundTransferObject {
    TreasuryOutboundTransfer,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl TreasuryOutboundTransferObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryOutboundTransferObject::TreasuryOutboundTransfer => "treasury.outbound_transfer",
            TreasuryOutboundTransferObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl Default for TreasuryOutboundTransferObject {
    fn default() -> Self {
        TreasuryOutboundTransferObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryOutboundTransferObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryOutboundTransferObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "treasury.outbound_transfer" => TreasuryOutboundTransferObject::TreasuryOutboundTransfer,
            other => TreasuryOutboundTransferObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryOutboundTransfersResourceAchTrackingDetails {
    #[doc = "ACH trace ID of the OutboundTransfer for transfers sent over the `ach` network."]
    #[json(default)]
    pub trace_id: String,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryReceivedCreditNetwork {
    Ach,
    Card,
    Rtp,
    Stripe,
    UsDomesticWire,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl TreasuryReceivedCreditNetwork {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryReceivedCreditNetwork::Ach => "ach",
            TreasuryReceivedCreditNetwork::Card => "card",
            TreasuryReceivedCreditNetwork::Rtp => "rtp",
            TreasuryReceivedCreditNetwork::Stripe => "stripe",
            TreasuryReceivedCreditNetwork::UsDomesticWire => "us_domestic_wire",
            TreasuryReceivedCreditNetwork::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl Default for TreasuryReceivedCreditNetwork {
    fn default() -> Self {
        TreasuryReceivedCreditNetwork::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryReceivedCreditNetwork {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryReceivedCreditNetwork::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ach" => TreasuryReceivedCreditNetwork::Ach,
            "card" => TreasuryReceivedCreditNetwork::Card,
            "rtp" => TreasuryReceivedCreditNetwork::Rtp,
            "stripe" => TreasuryReceivedCreditNetwork::Stripe,
            "us_domestic_wire" => TreasuryReceivedCreditNetwork::UsDomesticWire,
            other => TreasuryReceivedCreditNetwork::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryReceivedCreditObject {
    TreasuryReceivedCredit,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl TreasuryReceivedCreditObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryReceivedCreditObject::TreasuryReceivedCredit => "treasury.received_credit",
            TreasuryReceivedCreditObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl Default for TreasuryReceivedCreditObject {
    fn default() -> Self {
        TreasuryReceivedCreditObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryReceivedCreditObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryReceivedCreditObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "treasury.received_credit" => TreasuryReceivedCreditObject::TreasuryReceivedCredit,
            other => TreasuryReceivedCreditObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryReceivedCreditsResourceStatusTransitions {
    #[doc = "Timestamp describing when the CreditReversal changed status to `posted`"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub posted_at: Option<i64>,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryReceivedDebitObject {
    TreasuryReceivedDebit,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl TreasuryReceivedDebitObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryReceivedDebitObject::TreasuryReceivedDebit => "treasury.received_debit",
            TreasuryReceivedDebitObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl Default for TreasuryReceivedDebitObject {
    fn default() -> Self {
        TreasuryReceivedDebitObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryReceivedDebitObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryReceivedDebitObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "treasury.received_debit" => TreasuryReceivedDebitObject::TreasuryReceivedDebit,
            other => TreasuryReceivedDebitObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TreasuryTransactionStatus {
    Open,
    Posted,
    Void,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl TreasuryTransactionStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TreasuryTransactionStatus::Open => "open",
            TreasuryTransactionStatus::Posted => "posted",
            TreasuryTransactionStatus::Void => "void",
            TreasuryTransactionStatus::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl Default for TreasuryTransactionStatus {
    fn default() -> Self {
        TreasuryTransactionStatus::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl<'json> json::Decode<'json> for TreasuryTransactionStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TreasuryTransactionStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "open" => TreasuryTransactionStatus::Open,
            "posted" => TreasuryTransactionStatus::Posted,
            "void" => TreasuryTransactionStatus::Void,
            other => TreasuryTransactionStatus::Unknown(other.to_string()),
        })
    }
}
