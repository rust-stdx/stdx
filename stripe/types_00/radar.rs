#[cfg(feature = "radar")]
#[derive(Debug, Clone, PartialEq)]
pub enum InsightsResourcesPaymentEvaluationDisputeOpenedReason {
    AccountNotAvailable,
    CreditNotProcessed,
    CustomerInitiated,
    Duplicate,
    Fraudulent,
    General,
    Noncompliant,
    ProductNotReceived,
    ProductUnacceptable,
    SubscriptionCanceled,
    Unrecognized,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "radar")]
impl InsightsResourcesPaymentEvaluationDisputeOpenedReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InsightsResourcesPaymentEvaluationDisputeOpenedReason::AccountNotAvailable => "account_not_available",
            InsightsResourcesPaymentEvaluationDisputeOpenedReason::CreditNotProcessed => "credit_not_processed",
            InsightsResourcesPaymentEvaluationDisputeOpenedReason::CustomerInitiated => "customer_initiated",
            InsightsResourcesPaymentEvaluationDisputeOpenedReason::Duplicate => "duplicate",
            InsightsResourcesPaymentEvaluationDisputeOpenedReason::Fraudulent => "fraudulent",
            InsightsResourcesPaymentEvaluationDisputeOpenedReason::General => "general",
            InsightsResourcesPaymentEvaluationDisputeOpenedReason::Noncompliant => "noncompliant",
            InsightsResourcesPaymentEvaluationDisputeOpenedReason::ProductNotReceived => "product_not_received",
            InsightsResourcesPaymentEvaluationDisputeOpenedReason::ProductUnacceptable => "product_unacceptable",
            InsightsResourcesPaymentEvaluationDisputeOpenedReason::SubscriptionCanceled => "subscription_canceled",
            InsightsResourcesPaymentEvaluationDisputeOpenedReason::Unrecognized => "unrecognized",
            InsightsResourcesPaymentEvaluationDisputeOpenedReason::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "radar")]
impl Default for InsightsResourcesPaymentEvaluationDisputeOpenedReason {
    fn default() -> Self {
        InsightsResourcesPaymentEvaluationDisputeOpenedReason::Unknown(String::new())
    }
}

#[cfg(feature = "radar")]
impl<'json> json::Decode<'json> for InsightsResourcesPaymentEvaluationDisputeOpenedReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InsightsResourcesPaymentEvaluationDisputeOpenedReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account_not_available" => InsightsResourcesPaymentEvaluationDisputeOpenedReason::AccountNotAvailable,
            "credit_not_processed" => InsightsResourcesPaymentEvaluationDisputeOpenedReason::CreditNotProcessed,
            "customer_initiated" => InsightsResourcesPaymentEvaluationDisputeOpenedReason::CustomerInitiated,
            "duplicate" => InsightsResourcesPaymentEvaluationDisputeOpenedReason::Duplicate,
            "fraudulent" => InsightsResourcesPaymentEvaluationDisputeOpenedReason::Fraudulent,
            "general" => InsightsResourcesPaymentEvaluationDisputeOpenedReason::General,
            "noncompliant" => InsightsResourcesPaymentEvaluationDisputeOpenedReason::Noncompliant,
            "product_not_received" => InsightsResourcesPaymentEvaluationDisputeOpenedReason::ProductNotReceived,
            "product_unacceptable" => InsightsResourcesPaymentEvaluationDisputeOpenedReason::ProductUnacceptable,
            "subscription_canceled" => InsightsResourcesPaymentEvaluationDisputeOpenedReason::SubscriptionCanceled,
            "unrecognized" => InsightsResourcesPaymentEvaluationDisputeOpenedReason::Unrecognized,
            other => InsightsResourcesPaymentEvaluationDisputeOpenedReason::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, PartialEq)]
pub enum InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedFraudType {
    MadeWithLostCard,
    MadeWithStolenCard,
    Other,
    UnauthorizedUseOfCard,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "radar")]
impl InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedFraudType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedFraudType::MadeWithLostCard => {
                "made_with_lost_card"
            }
            InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedFraudType::MadeWithStolenCard => {
                "made_with_stolen_card"
            }
            InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedFraudType::Other => "other",
            InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedFraudType::UnauthorizedUseOfCard => {
                "unauthorized_use_of_card"
            }
            InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedFraudType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "radar")]
impl Default for InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedFraudType {
    fn default() -> Self {
        InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedFraudType::Unknown(String::new())
    }
}

#[cfg(feature = "radar")]
impl<'json> json::Decode<'json> for InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedFraudType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedFraudType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "made_with_lost_card" => {
                InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedFraudType::MadeWithLostCard
            }
            "made_with_stolen_card" => {
                InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedFraudType::MadeWithStolenCard
            }
            "other" => InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedFraudType::Other,
            "unauthorized_use_of_card" => {
                InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedFraudType::UnauthorizedUseOfCard
            }
            other => InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedFraudType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, PartialEq)]
pub enum InsightsResourcesPaymentEvaluationOutcomeType {
    Failed,
    MerchantBlocked,
    Rejected,
    Rerouted,
    Succeeded,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "radar")]
impl InsightsResourcesPaymentEvaluationOutcomeType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InsightsResourcesPaymentEvaluationOutcomeType::Failed => "failed",
            InsightsResourcesPaymentEvaluationOutcomeType::MerchantBlocked => "merchant_blocked",
            InsightsResourcesPaymentEvaluationOutcomeType::Rejected => "rejected",
            InsightsResourcesPaymentEvaluationOutcomeType::Rerouted => "rerouted",
            InsightsResourcesPaymentEvaluationOutcomeType::Succeeded => "succeeded",
            InsightsResourcesPaymentEvaluationOutcomeType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "radar")]
impl Default for InsightsResourcesPaymentEvaluationOutcomeType {
    fn default() -> Self {
        InsightsResourcesPaymentEvaluationOutcomeType::Unknown(String::new())
    }
}

#[cfg(feature = "radar")]
impl<'json> json::Decode<'json> for InsightsResourcesPaymentEvaluationOutcomeType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InsightsResourcesPaymentEvaluationOutcomeType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "failed" => InsightsResourcesPaymentEvaluationOutcomeType::Failed,
            "merchant_blocked" => InsightsResourcesPaymentEvaluationOutcomeType::MerchantBlocked,
            "rejected" => InsightsResourcesPaymentEvaluationOutcomeType::Rejected,
            "rerouted" => InsightsResourcesPaymentEvaluationOutcomeType::Rerouted,
            "succeeded" => InsightsResourcesPaymentEvaluationOutcomeType::Succeeded,
            other => InsightsResourcesPaymentEvaluationOutcomeType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, PartialEq)]
pub enum InsightsResourcesPaymentEvaluationRefundedReason {
    Duplicate,
    Fraudulent,
    Other,
    RequestedByCustomer,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "radar")]
impl InsightsResourcesPaymentEvaluationRefundedReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InsightsResourcesPaymentEvaluationRefundedReason::Duplicate => "duplicate",
            InsightsResourcesPaymentEvaluationRefundedReason::Fraudulent => "fraudulent",
            InsightsResourcesPaymentEvaluationRefundedReason::Other => "other",
            InsightsResourcesPaymentEvaluationRefundedReason::RequestedByCustomer => "requested_by_customer",
            InsightsResourcesPaymentEvaluationRefundedReason::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "radar")]
impl Default for InsightsResourcesPaymentEvaluationRefundedReason {
    fn default() -> Self {
        InsightsResourcesPaymentEvaluationRefundedReason::Unknown(String::new())
    }
}

#[cfg(feature = "radar")]
impl<'json> json::Decode<'json> for InsightsResourcesPaymentEvaluationRefundedReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InsightsResourcesPaymentEvaluationRefundedReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "duplicate" => InsightsResourcesPaymentEvaluationRefundedReason::Duplicate,
            "fraudulent" => InsightsResourcesPaymentEvaluationRefundedReason::Fraudulent,
            "other" => InsightsResourcesPaymentEvaluationRefundedReason::Other,
            "requested_by_customer" => InsightsResourcesPaymentEvaluationRefundedReason::RequestedByCustomer,
            other => InsightsResourcesPaymentEvaluationRefundedReason::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, PartialEq)]
pub enum InsightsResourcesPaymentEvaluationRejectedCardReason {
    AuthenticationFailed,
    DoNotHonor,
    Expired,
    IncorrectCvc,
    IncorrectNumber,
    IncorrectPostalCode,
    InsufficientFunds,
    InvalidAccount,
    LostCard,
    Other,
    ProcessingError,
    ReportedStolen,
    TryAgainLater,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "radar")]
impl InsightsResourcesPaymentEvaluationRejectedCardReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InsightsResourcesPaymentEvaluationRejectedCardReason::AuthenticationFailed => "authentication_failed",
            InsightsResourcesPaymentEvaluationRejectedCardReason::DoNotHonor => "do_not_honor",
            InsightsResourcesPaymentEvaluationRejectedCardReason::Expired => "expired",
            InsightsResourcesPaymentEvaluationRejectedCardReason::IncorrectCvc => "incorrect_cvc",
            InsightsResourcesPaymentEvaluationRejectedCardReason::IncorrectNumber => "incorrect_number",
            InsightsResourcesPaymentEvaluationRejectedCardReason::IncorrectPostalCode => "incorrect_postal_code",
            InsightsResourcesPaymentEvaluationRejectedCardReason::InsufficientFunds => "insufficient_funds",
            InsightsResourcesPaymentEvaluationRejectedCardReason::InvalidAccount => "invalid_account",
            InsightsResourcesPaymentEvaluationRejectedCardReason::LostCard => "lost_card",
            InsightsResourcesPaymentEvaluationRejectedCardReason::Other => "other",
            InsightsResourcesPaymentEvaluationRejectedCardReason::ProcessingError => "processing_error",
            InsightsResourcesPaymentEvaluationRejectedCardReason::ReportedStolen => "reported_stolen",
            InsightsResourcesPaymentEvaluationRejectedCardReason::TryAgainLater => "try_again_later",
            InsightsResourcesPaymentEvaluationRejectedCardReason::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "radar")]
impl Default for InsightsResourcesPaymentEvaluationRejectedCardReason {
    fn default() -> Self {
        InsightsResourcesPaymentEvaluationRejectedCardReason::Unknown(String::new())
    }
}

#[cfg(feature = "radar")]
impl<'json> json::Decode<'json> for InsightsResourcesPaymentEvaluationRejectedCardReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InsightsResourcesPaymentEvaluationRejectedCardReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "authentication_failed" => InsightsResourcesPaymentEvaluationRejectedCardReason::AuthenticationFailed,
            "do_not_honor" => InsightsResourcesPaymentEvaluationRejectedCardReason::DoNotHonor,
            "expired" => InsightsResourcesPaymentEvaluationRejectedCardReason::Expired,
            "incorrect_cvc" => InsightsResourcesPaymentEvaluationRejectedCardReason::IncorrectCvc,
            "incorrect_number" => InsightsResourcesPaymentEvaluationRejectedCardReason::IncorrectNumber,
            "incorrect_postal_code" => InsightsResourcesPaymentEvaluationRejectedCardReason::IncorrectPostalCode,
            "insufficient_funds" => InsightsResourcesPaymentEvaluationRejectedCardReason::InsufficientFunds,
            "invalid_account" => InsightsResourcesPaymentEvaluationRejectedCardReason::InvalidAccount,
            "lost_card" => InsightsResourcesPaymentEvaluationRejectedCardReason::LostCard,
            "other" => InsightsResourcesPaymentEvaluationRejectedCardReason::Other,
            "processing_error" => InsightsResourcesPaymentEvaluationRejectedCardReason::ProcessingError,
            "reported_stolen" => InsightsResourcesPaymentEvaluationRejectedCardReason::ReportedStolen,
            "try_again_later" => InsightsResourcesPaymentEvaluationRejectedCardReason::TryAgainLater,
            other => InsightsResourcesPaymentEvaluationRejectedCardReason::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, PartialEq)]
pub enum InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome {
    Abandoned,
    Failed,
    Passed,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "radar")]
impl InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome::Abandoned => "abandoned",
            InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome::Failed => "failed",
            InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome::Passed => "passed",
            InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "radar")]
impl<'json> json::Decode<'json> for InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "abandoned" => InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome::Abandoned,
            "failed" => InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome::Failed,
            "passed" => InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome::Passed,
            other => InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, PartialEq)]
pub enum RadarPaymentEvaluationRecommendedAction {
    Block,
    Continue,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "radar")]
impl RadarPaymentEvaluationRecommendedAction {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            RadarPaymentEvaluationRecommendedAction::Block => "block",
            RadarPaymentEvaluationRecommendedAction::Continue => "continue",
            RadarPaymentEvaluationRecommendedAction::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "radar")]
impl Default for RadarPaymentEvaluationRecommendedAction {
    fn default() -> Self {
        RadarPaymentEvaluationRecommendedAction::Unknown(String::new())
    }
}

#[cfg(feature = "radar")]
impl<'json> json::Decode<'json> for RadarPaymentEvaluationRecommendedAction {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(RadarPaymentEvaluationRecommendedAction::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "block" => RadarPaymentEvaluationRecommendedAction::Block,
            "continue" => RadarPaymentEvaluationRecommendedAction::Continue,
            other => RadarPaymentEvaluationRecommendedAction::Unknown(other.to_string()),
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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct RadarRadarOptions {
    #[doc = "A [Radar Session](https://docs.stripe.com/radar/radar-session) is a snapshot of the browser metadata and device detai..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct Rule {
    #[doc = "The action taken on the payment."]
    #[json(default)]
    pub action: String,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "The predicate to evaluate the payment against."]
    #[json(default)]
    pub predicate: String,
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
impl crate::PagedItem for Rule {
    fn id(&self) -> &str {
        &self.id
    }
}
