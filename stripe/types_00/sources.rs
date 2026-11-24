#[cfg(feature = "sources")]
#[derive(Debug, Clone, Default)]
pub struct CreateSourceParamsMandateAcceptanceOffline {
    #[doc = "An email to contact you with if a copy of the mandate is requested, required if `type` is `offline`."]
    pub contact_email: String,
}

#[cfg(feature = "sources")]
impl crate::params::FormEncode for CreateSourceParamsMandateAcceptanceOffline {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("contact_email", &self.contact_email);
    }
}

#[cfg(feature = "sources")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSourceParamsMandateAcceptanceType {
    Offline,
    Online,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "sources")]
impl CreateSourceParamsMandateAcceptanceType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSourceParamsMandateAcceptanceType::Offline => "offline",
            CreateSourceParamsMandateAcceptanceType::Online => "online",
            CreateSourceParamsMandateAcceptanceType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "sources")]
impl<'json> json::Decode<'json> for CreateSourceParamsMandateAcceptanceType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSourceParamsMandateAcceptanceType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "offline" => CreateSourceParamsMandateAcceptanceType::Offline,
            "online" => CreateSourceParamsMandateAcceptanceType::Online,
            other => CreateSourceParamsMandateAcceptanceType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "sources")]
impl crate::params::FormEncode for CreateSourceParamsMandateAcceptanceType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "sources")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSourceParamsReceiverRefundAttributesMethod {
    Email,
    Manual,
    None,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "sources")]
impl CreateSourceParamsReceiverRefundAttributesMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSourceParamsReceiverRefundAttributesMethod::Email => "email",
            CreateSourceParamsReceiverRefundAttributesMethod::Manual => "manual",
            CreateSourceParamsReceiverRefundAttributesMethod::None => "none",
            CreateSourceParamsReceiverRefundAttributesMethod::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "sources")]
impl<'json> json::Decode<'json> for CreateSourceParamsReceiverRefundAttributesMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSourceParamsReceiverRefundAttributesMethod::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "email" => CreateSourceParamsReceiverRefundAttributesMethod::Email,
            "manual" => CreateSourceParamsReceiverRefundAttributesMethod::Manual,
            "none" => CreateSourceParamsReceiverRefundAttributesMethod::None,
            other => CreateSourceParamsReceiverRefundAttributesMethod::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "sources")]
impl crate::params::FormEncode for CreateSourceParamsReceiverRefundAttributesMethod {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "sources")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSourceParamsSourceOrderItemsItemType {
    Discount,
    Shipping,
    Sku,
    Tax,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "sources")]
impl CreateSourceParamsSourceOrderItemsItemType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSourceParamsSourceOrderItemsItemType::Discount => "discount",
            CreateSourceParamsSourceOrderItemsItemType::Shipping => "shipping",
            CreateSourceParamsSourceOrderItemsItemType::Sku => "sku",
            CreateSourceParamsSourceOrderItemsItemType::Tax => "tax",
            CreateSourceParamsSourceOrderItemsItemType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "sources")]
impl<'json> json::Decode<'json> for CreateSourceParamsSourceOrderItemsItemType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSourceParamsSourceOrderItemsItemType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "discount" => CreateSourceParamsSourceOrderItemsItemType::Discount,
            "shipping" => CreateSourceParamsSourceOrderItemsItemType::Shipping,
            "sku" => CreateSourceParamsSourceOrderItemsItemType::Sku,
            "tax" => CreateSourceParamsSourceOrderItemsItemType::Tax,
            other => CreateSourceParamsSourceOrderItemsItemType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "sources")]
impl crate::params::FormEncode for CreateSourceParamsSourceOrderItemsItemType {
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
pub struct SourceCodeVerificationFlow {
    #[doc = "The number of attempts remaining to authenticate the source object with a verification code."]
    #[json(default)]
    pub attempts_remaining: i64,
    #[doc = "The status of the code verification, either `pending` (awaiting verification, `attempts_remaining` should be greater ..."]
    #[json(default)]
    pub status: String,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SourceMandateNotificationBacsDebitData {
    #[doc = "Last 4 digits of the account number associated with the debit."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum SourceMandateNotificationObject {
    SourceMandateNotification,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl SourceMandateNotificationObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SourceMandateNotificationObject::SourceMandateNotification => "source_mandate_notification",
            SourceMandateNotificationObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for SourceMandateNotificationObject {
    fn default() -> Self {
        SourceMandateNotificationObject::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for SourceMandateNotificationObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SourceMandateNotificationObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "source_mandate_notification" => SourceMandateNotificationObject::SourceMandateNotification,
            other => SourceMandateNotificationObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "sources"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SourceTransactionPaperCheckData {
    #[doc = "Time at which the deposited funds will be available for use."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub available_at: Option<String>,
    #[doc = "Comma-separated list of invoice IDs associated with the paper check."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub invoices: Option<String>,
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
#[derive(Debug, Clone, PartialEq)]
pub enum SourceType {
    AchCreditTransfer,
    AchDebit,
    AcssDebit,
    Alipay,
    AuBecsDebit,
    Bancontact,
    Card,
    CardPresent,
    Eps,
    Giropay,
    Ideal,
    Klarna,
    Multibanco,
    P24,
    SepaCreditTransfer,
    SepaDebit,
    Sofort,
    ThreeDSecure,
    Wechat,
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
impl SourceType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SourceType::AchCreditTransfer => "ach_credit_transfer",
            SourceType::AchDebit => "ach_debit",
            SourceType::AcssDebit => "acss_debit",
            SourceType::Alipay => "alipay",
            SourceType::AuBecsDebit => "au_becs_debit",
            SourceType::Bancontact => "bancontact",
            SourceType::Card => "card",
            SourceType::CardPresent => "card_present",
            SourceType::Eps => "eps",
            SourceType::Giropay => "giropay",
            SourceType::Ideal => "ideal",
            SourceType::Klarna => "klarna",
            SourceType::Multibanco => "multibanco",
            SourceType::P24 => "p24",
            SourceType::SepaCreditTransfer => "sepa_credit_transfer",
            SourceType::SepaDebit => "sepa_debit",
            SourceType::Sofort => "sofort",
            SourceType::ThreeDSecure => "three_d_secure",
            SourceType::Wechat => "wechat",
            SourceType::Unknown(value) => value,
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
impl Default for SourceType {
    fn default() -> Self {
        SourceType::Unknown(String::new())
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
impl<'json> json::Decode<'json> for SourceType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SourceType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ach_credit_transfer" => SourceType::AchCreditTransfer,
            "ach_debit" => SourceType::AchDebit,
            "acss_debit" => SourceType::AcssDebit,
            "alipay" => SourceType::Alipay,
            "au_becs_debit" => SourceType::AuBecsDebit,
            "bancontact" => SourceType::Bancontact,
            "card" => SourceType::Card,
            "card_present" => SourceType::CardPresent,
            "eps" => SourceType::Eps,
            "giropay" => SourceType::Giropay,
            "ideal" => SourceType::Ideal,
            "klarna" => SourceType::Klarna,
            "multibanco" => SourceType::Multibanco,
            "p24" => SourceType::P24,
            "sepa_credit_transfer" => SourceType::SepaCreditTransfer,
            "sepa_debit" => SourceType::SepaDebit,
            "sofort" => SourceType::Sofort,
            "three_d_secure" => SourceType::ThreeDSecure,
            "wechat" => SourceType::Wechat,
            other => SourceType::Unknown(other.to_string()),
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
pub struct SourceTypeAchCreditTransfer {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_name: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_type: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refund_routing_number: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub swift_code: Option<String>,
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
pub struct SourceTypeMultibanco {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_address_city: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_address_country: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_address_line1: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_address_line2: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_address_postal_code: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_address_state: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refund_account_holder_name: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refund_iban: Option<String>,
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
pub struct SourceTypeThreeDSecure {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address_line1_check: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address_zip_check: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub authenticated: Option<bool>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cvc_check: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub dynamic_last4: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<i64>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<i64>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub funding: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub iin: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tokenization_method: Option<String>,
}
