#[cfg(feature = "subscription_items")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionItemParamsBillingThresholds {
    #[doc = "Number of units that meets the billing threshold to advance the subscription to a new billing period (e.g., it takes ..."]
    pub usage_gte: i64,
}

#[cfg(feature = "subscription_items")]
impl crate::params::FormEncode for CreateSubscriptionItemParamsBillingThresholds {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("usage_gte", &self.usage_gte);
    }
}

#[cfg(feature = "subscription_items")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionItemParamsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscription_items")]
impl CreateSubscriptionItemParamsPriceDataTaxBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionItemParamsPriceDataTaxBehavior::Exclusive => "exclusive",
            CreateSubscriptionItemParamsPriceDataTaxBehavior::Inclusive => "inclusive",
            CreateSubscriptionItemParamsPriceDataTaxBehavior::Unspecified => "unspecified",
            CreateSubscriptionItemParamsPriceDataTaxBehavior::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscription_items")]
impl<'json> json::Decode<'json> for CreateSubscriptionItemParamsPriceDataTaxBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSubscriptionItemParamsPriceDataTaxBehavior::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "exclusive" => CreateSubscriptionItemParamsPriceDataTaxBehavior::Exclusive,
            "inclusive" => CreateSubscriptionItemParamsPriceDataTaxBehavior::Inclusive,
            "unspecified" => CreateSubscriptionItemParamsPriceDataTaxBehavior::Unspecified,
            other => CreateSubscriptionItemParamsPriceDataTaxBehavior::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscription_items")]
impl crate::params::FormEncode for CreateSubscriptionItemParamsPriceDataTaxBehavior {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscription_items")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionItemParamsProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscription_items")]
impl CreateSubscriptionItemParamsProrationBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionItemParamsProrationBehavior::AlwaysInvoice => "always_invoice",
            CreateSubscriptionItemParamsProrationBehavior::CreateProrations => "create_prorations",
            CreateSubscriptionItemParamsProrationBehavior::None => "none",
            CreateSubscriptionItemParamsProrationBehavior::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscription_items")]
impl<'json> json::Decode<'json> for CreateSubscriptionItemParamsProrationBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSubscriptionItemParamsProrationBehavior::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "always_invoice" => CreateSubscriptionItemParamsProrationBehavior::AlwaysInvoice,
            "create_prorations" => CreateSubscriptionItemParamsProrationBehavior::CreateProrations,
            "none" => CreateSubscriptionItemParamsProrationBehavior::None,
            other => CreateSubscriptionItemParamsProrationBehavior::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscription_items")]
impl crate::params::FormEncode for CreateSubscriptionItemParamsProrationBehavior {
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
pub enum SubscriptionItemObject {
    SubscriptionItem,
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
impl SubscriptionItemObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionItemObject::SubscriptionItem => "subscription_item",
            SubscriptionItemObject::Unknown(value) => value,
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
impl Default for SubscriptionItemObject {
    fn default() -> Self {
        SubscriptionItemObject::Unknown(String::new())
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
impl<'json> json::Decode<'json> for SubscriptionItemObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SubscriptionItemObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "subscription_item" => SubscriptionItemObject::SubscriptionItem,
            other => SubscriptionItemObject::Unknown(other.to_string()),
        })
    }
}
