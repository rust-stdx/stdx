#[cfg(feature = "sources")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSourceParamsFlow {
    CodeVerification,
    None,
    Receiver,
    Redirect,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "sources")]
impl CreateSourceParamsFlow {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSourceParamsFlow::CodeVerification => "code_verification",
            CreateSourceParamsFlow::None => "none",
            CreateSourceParamsFlow::Receiver => "receiver",
            CreateSourceParamsFlow::Redirect => "redirect",
            CreateSourceParamsFlow::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "sources")]
impl<'json> json::Decode<'json> for CreateSourceParamsFlow {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSourceParamsFlow::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "code_verification" => CreateSourceParamsFlow::CodeVerification,
            "none" => CreateSourceParamsFlow::None,
            "receiver" => CreateSourceParamsFlow::Receiver,
            "redirect" => CreateSourceParamsFlow::Redirect,
            other => CreateSourceParamsFlow::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "sources")]
impl crate::params::FormEncode for CreateSourceParamsFlow {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "sources")]
#[derive(Debug, Clone, Default)]
pub struct CreateSourceParamsRedirect {
    #[doc = "The URL you provide to redirect the customer back to you after they authenticated their payment."]
    pub return_url: String,
}

#[cfg(feature = "sources")]
impl crate::params::FormEncode for CreateSourceParamsRedirect {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("return_url", &self.return_url);
    }
}

#[cfg(feature = "sources")]
#[derive(Debug, Clone, Default)]
pub struct GetSourceParams {
    pub client_secret: Option<String>,
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "sources")]
impl crate::params::FormEncode for GetSourceParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("client_secret", self.client_secret.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
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
    feature = "sources",
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
pub struct SourceOrderItem {
    #[doc = "The amount (price) for this order item."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[doc = "This currency of this order item."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "Human-readable description for this order item."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The ID of the associated object for this line item."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[doc = "The quantity of this order item."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[doc = "The type of this order item."]
    #[json(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[cfg(any(feature = "events", feature = "sources"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SourceTransactionGbpCreditTransferData {
    #[doc = "Bank account fingerprint associated with the Stripe owned bank account receiving the transfer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[doc = "The credit transfer rails the sender used to push this transfer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub funding_method: Option<String>,
    #[doc = "Last 4 digits of sender account number associated with the transfer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[doc = "Sender entered arbitrary information about the transfer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[doc = "Sender account number associated with the transfer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sender_account_number: Option<String>,
    #[doc = "Sender name associated with the transfer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
    #[doc = "Sender sort code associated with the transfer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sender_sort_code: Option<String>,
}

#[cfg(any(feature = "events", feature = "sources"))]
#[derive(Debug, Clone, PartialEq)]
pub enum SourceTransactionObject {
    SourceTransaction,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "sources"))]
impl SourceTransactionObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SourceTransactionObject::SourceTransaction => "source_transaction",
            SourceTransactionObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "sources"))]
impl Default for SourceTransactionObject {
    fn default() -> Self {
        SourceTransactionObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "sources"))]
impl<'json> json::Decode<'json> for SourceTransactionObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SourceTransactionObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "source_transaction" => SourceTransactionObject::SourceTransaction,
            other => SourceTransactionObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "sources"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SourceTransactionSepaCreditTransferData {
    #[doc = "Reference associated with the transfer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[doc = "Sender's bank account IBAN."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sender_iban: Option<String>,
    #[doc = "Sender's name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
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
    feature = "sources",
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
pub struct SourceTypeAchDebit {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
    #[json(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
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
    feature = "sources",
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
pub struct SourceTypeIdeal {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub iban_last4: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
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
    feature = "sources",
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
pub struct SourceTypeSepaDebit {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub branch_code: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate_reference: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate_url: Option<String>,
}
