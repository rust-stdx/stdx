#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct NotificationEventRequest {
    #[doc = "ID of the API request that caused the event."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The idempotency key transmitted during the request, if any."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1AccountApplicationAuthorizedReasonType {
    Request,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1AccountApplicationAuthorizedReasonType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1AccountApplicationAuthorizedReasonType::Request => "request",
            V1AccountApplicationAuthorizedReasonType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1AccountApplicationAuthorizedReasonType {
    fn default() -> Self {
        V1AccountApplicationAuthorizedReasonType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1AccountApplicationAuthorizedReasonType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1AccountApplicationAuthorizedReasonType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "request" => V1AccountApplicationAuthorizedReasonType::Request,
            other => V1AccountApplicationAuthorizedReasonType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1AccountApplicationDeauthorizedType {
    V1AccountApplicationDeauthorized,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1AccountApplicationDeauthorizedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1AccountApplicationDeauthorizedType::V1AccountApplicationDeauthorized => {
                "v1.account.application.deauthorized"
            }
            V1AccountApplicationDeauthorizedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1AccountApplicationDeauthorizedType {
    fn default() -> Self {
        V1AccountApplicationDeauthorizedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1AccountApplicationDeauthorizedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1AccountApplicationDeauthorizedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.account.application.deauthorized" => {
                V1AccountApplicationDeauthorizedType::V1AccountApplicationDeauthorized
            }
            other => V1AccountApplicationDeauthorizedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1AccountExternalAccountDeletedType {
    V1AccountExternalAccountDeleted,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1AccountExternalAccountDeletedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1AccountExternalAccountDeletedType::V1AccountExternalAccountDeleted => {
                "v1.account.external_account.deleted"
            }
            V1AccountExternalAccountDeletedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1AccountExternalAccountDeletedType {
    fn default() -> Self {
        V1AccountExternalAccountDeletedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1AccountExternalAccountDeletedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1AccountExternalAccountDeletedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.account.external_account.deleted" => {
                V1AccountExternalAccountDeletedType::V1AccountExternalAccountDeleted
            }
            other => V1AccountExternalAccountDeletedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1AccountUpdatedType {
    V1AccountUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1AccountUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1AccountUpdatedType::V1AccountUpdated => "v1.account.updated",
            V1AccountUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1AccountUpdatedType {
    fn default() -> Self {
        V1AccountUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1AccountUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1AccountUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.account.updated" => V1AccountUpdatedType::V1AccountUpdated,
            other => V1AccountUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ApplicationFeeCreatedType {
    V1ApplicationFeeCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ApplicationFeeCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ApplicationFeeCreatedType::V1ApplicationFeeCreated => "v1.application_fee.created",
            V1ApplicationFeeCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ApplicationFeeCreatedType {
    fn default() -> Self {
        V1ApplicationFeeCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ApplicationFeeCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ApplicationFeeCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.application_fee.created" => V1ApplicationFeeCreatedType::V1ApplicationFeeCreated,
            other => V1ApplicationFeeCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1BalanceSettingsUpdatedType {
    V1BalanceSettingsUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1BalanceSettingsUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1BalanceSettingsUpdatedType::V1BalanceSettingsUpdated => "v1.balance_settings.updated",
            V1BalanceSettingsUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1BalanceSettingsUpdatedType {
    fn default() -> Self {
        V1BalanceSettingsUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1BalanceSettingsUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1BalanceSettingsUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.balance_settings.updated" => V1BalanceSettingsUpdatedType::V1BalanceSettingsUpdated,
            other => V1BalanceSettingsUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1BillingAlertTriggeredRelatedObjectType {
    BillingAlert,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1BillingAlertTriggeredRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1BillingAlertTriggeredRelatedObjectType::BillingAlert => "billing.alert",
            V1BillingAlertTriggeredRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1BillingAlertTriggeredRelatedObjectType {
    fn default() -> Self {
        V1BillingAlertTriggeredRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1BillingAlertTriggeredRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1BillingAlertTriggeredRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "billing.alert" => V1BillingAlertTriggeredRelatedObjectType::BillingAlert,
            other => V1BillingAlertTriggeredRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1BillingCreditBalanceTransactionCreatedType {
    V1BillingCreditBalanceTransactionCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1BillingCreditBalanceTransactionCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1BillingCreditBalanceTransactionCreatedType::V1BillingCreditBalanceTransactionCreated => {
                "v1.billing.credit_balance_transaction.created"
            }
            V1BillingCreditBalanceTransactionCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1BillingCreditBalanceTransactionCreatedType {
    fn default() -> Self {
        V1BillingCreditBalanceTransactionCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1BillingCreditBalanceTransactionCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1BillingCreditBalanceTransactionCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.billing.credit_balance_transaction.created" => {
                V1BillingCreditBalanceTransactionCreatedType::V1BillingCreditBalanceTransactionCreated
            }
            other => V1BillingCreditBalanceTransactionCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1BillingCreditGrantCreatedType {
    V1BillingCreditGrantCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1BillingCreditGrantCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1BillingCreditGrantCreatedType::V1BillingCreditGrantCreated => "v1.billing.credit_grant.created",
            V1BillingCreditGrantCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1BillingCreditGrantCreatedType {
    fn default() -> Self {
        V1BillingCreditGrantCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1BillingCreditGrantCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1BillingCreditGrantCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.billing.credit_grant.created" => V1BillingCreditGrantCreatedType::V1BillingCreditGrantCreated,
            other => V1BillingCreditGrantCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1BillingMeterNoMeterFoundType {
    V1BillingMeterNoMeterFound,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1BillingMeterNoMeterFoundType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1BillingMeterNoMeterFoundType::V1BillingMeterNoMeterFound => "v1.billing.meter.no_meter_found",
            V1BillingMeterNoMeterFoundType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1BillingMeterNoMeterFoundType {
    fn default() -> Self {
        V1BillingMeterNoMeterFoundType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1BillingMeterNoMeterFoundType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1BillingMeterNoMeterFoundType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.billing.meter.no_meter_found" => V1BillingMeterNoMeterFoundType::V1BillingMeterNoMeterFound,
            other => V1BillingMeterNoMeterFoundType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1BillingMeterUpdatedType {
    V1BillingMeterUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1BillingMeterUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1BillingMeterUpdatedType::V1BillingMeterUpdated => "v1.billing.meter.updated",
            V1BillingMeterUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1BillingMeterUpdatedType {
    fn default() -> Self {
        V1BillingMeterUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1BillingMeterUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1BillingMeterUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.billing.meter.updated" => V1BillingMeterUpdatedType::V1BillingMeterUpdated,
            other => V1BillingMeterUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1BillingPortalConfigurationCreatedRelatedObjectType {
    BillingPortalConfiguration,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1BillingPortalConfigurationCreatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1BillingPortalConfigurationCreatedRelatedObjectType::BillingPortalConfiguration => {
                "billing_portal.configuration"
            }
            V1BillingPortalConfigurationCreatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1BillingPortalConfigurationCreatedRelatedObjectType {
    fn default() -> Self {
        V1BillingPortalConfigurationCreatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1BillingPortalConfigurationCreatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1BillingPortalConfigurationCreatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "billing_portal.configuration" => {
                V1BillingPortalConfigurationCreatedRelatedObjectType::BillingPortalConfiguration
            }
            other => V1BillingPortalConfigurationCreatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1BillingPortalConfigurationCreatedType {
    V1BillingPortalConfigurationCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1BillingPortalConfigurationCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1BillingPortalConfigurationCreatedType::V1BillingPortalConfigurationCreated => {
                "v1.billing_portal.configuration.created"
            }
            V1BillingPortalConfigurationCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1BillingPortalConfigurationCreatedType {
    fn default() -> Self {
        V1BillingPortalConfigurationCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1BillingPortalConfigurationCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1BillingPortalConfigurationCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.billing_portal.configuration.created" => {
                V1BillingPortalConfigurationCreatedType::V1BillingPortalConfigurationCreated
            }
            other => V1BillingPortalConfigurationCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1BillingPortalConfigurationUpdatedType {
    V1BillingPortalConfigurationUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1BillingPortalConfigurationUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1BillingPortalConfigurationUpdatedType::V1BillingPortalConfigurationUpdated => {
                "v1.billing_portal.configuration.updated"
            }
            V1BillingPortalConfigurationUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1BillingPortalConfigurationUpdatedType {
    fn default() -> Self {
        V1BillingPortalConfigurationUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1BillingPortalConfigurationUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1BillingPortalConfigurationUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.billing_portal.configuration.updated" => {
                V1BillingPortalConfigurationUpdatedType::V1BillingPortalConfigurationUpdated
            }
            other => V1BillingPortalConfigurationUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1BillingPortalSessionCreatedType {
    V1BillingPortalSessionCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1BillingPortalSessionCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1BillingPortalSessionCreatedType::V1BillingPortalSessionCreated => "v1.billing_portal.session.created",
            V1BillingPortalSessionCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1BillingPortalSessionCreatedType {
    fn default() -> Self {
        V1BillingPortalSessionCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1BillingPortalSessionCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1BillingPortalSessionCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.billing_portal.session.created" => V1BillingPortalSessionCreatedType::V1BillingPortalSessionCreated,
            other => V1BillingPortalSessionCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CashBalanceFundsAvailableType {
    V1CashBalanceFundsAvailable,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CashBalanceFundsAvailableType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CashBalanceFundsAvailableType::V1CashBalanceFundsAvailable => "v1.cash_balance.funds_available",
            V1CashBalanceFundsAvailableType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CashBalanceFundsAvailableType {
    fn default() -> Self {
        V1CashBalanceFundsAvailableType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CashBalanceFundsAvailableType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CashBalanceFundsAvailableType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.cash_balance.funds_available" => V1CashBalanceFundsAvailableType::V1CashBalanceFundsAvailable,
            other => V1CashBalanceFundsAvailableType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ChargeFailedType {
    V1ChargeFailed,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ChargeFailedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ChargeFailedType::V1ChargeFailed => "v1.charge.failed",
            V1ChargeFailedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ChargeFailedType {
    fn default() -> Self {
        V1ChargeFailedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ChargeFailedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ChargeFailedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.charge.failed" => V1ChargeFailedType::V1ChargeFailed,
            other => V1ChargeFailedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ChargePendingType {
    V1ChargePending,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ChargePendingType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ChargePendingType::V1ChargePending => "v1.charge.pending",
            V1ChargePendingType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ChargePendingType {
    fn default() -> Self {
        V1ChargePendingType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ChargePendingType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ChargePendingType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.charge.pending" => V1ChargePendingType::V1ChargePending,
            other => V1ChargePendingType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CouponCreatedRelatedObjectType {
    Coupon,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CouponCreatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CouponCreatedRelatedObjectType::Coupon => "coupon",
            V1CouponCreatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CouponCreatedRelatedObjectType {
    fn default() -> Self {
        V1CouponCreatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CouponCreatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CouponCreatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "coupon" => V1CouponCreatedRelatedObjectType::Coupon,
            other => V1CouponCreatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CustomerCashBalanceTransactionCreatedRelatedObjectType {
    CustomerCashBalanceTransaction,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CustomerCashBalanceTransactionCreatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CustomerCashBalanceTransactionCreatedRelatedObjectType::CustomerCashBalanceTransaction => {
                "customer_cash_balance_transaction"
            }
            V1CustomerCashBalanceTransactionCreatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CustomerCashBalanceTransactionCreatedRelatedObjectType {
    fn default() -> Self {
        V1CustomerCashBalanceTransactionCreatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CustomerCashBalanceTransactionCreatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CustomerCashBalanceTransactionCreatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "customer_cash_balance_transaction" => {
                V1CustomerCashBalanceTransactionCreatedRelatedObjectType::CustomerCashBalanceTransaction
            }
            other => V1CustomerCashBalanceTransactionCreatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CustomerCreatedRelatedObjectType {
    Customer,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CustomerCreatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CustomerCreatedRelatedObjectType::Customer => "customer",
            V1CustomerCreatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CustomerCreatedRelatedObjectType {
    fn default() -> Self {
        V1CustomerCreatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CustomerCreatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CustomerCreatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "customer" => V1CustomerCreatedRelatedObjectType::Customer,
            other => V1CustomerCreatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CustomerCreatedType {
    V1CustomerCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CustomerCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CustomerCreatedType::V1CustomerCreated => "v1.customer.created",
            V1CustomerCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CustomerCreatedType {
    fn default() -> Self {
        V1CustomerCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CustomerCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CustomerCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.customer.created" => V1CustomerCreatedType::V1CustomerCreated,
            other => V1CustomerCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CustomerDiscountUpdatedType {
    V1CustomerDiscountUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CustomerDiscountUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CustomerDiscountUpdatedType::V1CustomerDiscountUpdated => "v1.customer.discount.updated",
            V1CustomerDiscountUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CustomerDiscountUpdatedType {
    fn default() -> Self {
        V1CustomerDiscountUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CustomerDiscountUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CustomerDiscountUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.customer.discount.updated" => V1CustomerDiscountUpdatedType::V1CustomerDiscountUpdated,
            other => V1CustomerDiscountUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CustomerSubscriptionDeletedType {
    V1CustomerSubscriptionDeleted,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CustomerSubscriptionDeletedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CustomerSubscriptionDeletedType::V1CustomerSubscriptionDeleted => "v1.customer.subscription.deleted",
            V1CustomerSubscriptionDeletedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CustomerSubscriptionDeletedType {
    fn default() -> Self {
        V1CustomerSubscriptionDeletedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CustomerSubscriptionDeletedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CustomerSubscriptionDeletedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.customer.subscription.deleted" => V1CustomerSubscriptionDeletedType::V1CustomerSubscriptionDeleted,
            other => V1CustomerSubscriptionDeletedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CustomerTaxIdUpdatedType {
    V1CustomerTaxIdUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CustomerTaxIdUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CustomerTaxIdUpdatedType::V1CustomerTaxIdUpdated => "v1.customer.tax_id.updated",
            V1CustomerTaxIdUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CustomerTaxIdUpdatedType {
    fn default() -> Self {
        V1CustomerTaxIdUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CustomerTaxIdUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CustomerTaxIdUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.customer.tax_id.updated" => V1CustomerTaxIdUpdatedType::V1CustomerTaxIdUpdated,
            other => V1CustomerTaxIdUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1FileCreatedRelatedObjectType {
    File,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1FileCreatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1FileCreatedRelatedObjectType::File => "file",
            V1FileCreatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1FileCreatedRelatedObjectType {
    fn default() -> Self {
        V1FileCreatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1FileCreatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1FileCreatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "file" => V1FileCreatedRelatedObjectType::File,
            other => V1FileCreatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1FinancialConnectionsAccountCreatedType {
    V1FinancialConnectionsAccountCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1FinancialConnectionsAccountCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1FinancialConnectionsAccountCreatedType::V1FinancialConnectionsAccountCreated => {
                "v1.financial_connections.account.created"
            }
            V1FinancialConnectionsAccountCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1FinancialConnectionsAccountCreatedType {
    fn default() -> Self {
        V1FinancialConnectionsAccountCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1FinancialConnectionsAccountCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1FinancialConnectionsAccountCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.financial_connections.account.created" => {
                V1FinancialConnectionsAccountCreatedType::V1FinancialConnectionsAccountCreated
            }
            other => V1FinancialConnectionsAccountCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1FinancialConnectionsAccountExpectedDeactivationDateUpdatedType {
    V1FinancialConnectionsAccountExpectedDeactivationDateUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1FinancialConnectionsAccountExpectedDeactivationDateUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1FinancialConnectionsAccountExpectedDeactivationDateUpdatedType::V1FinancialConnectionsAccountExpectedDeactivationDateUpdated => "v1.financial_connections.account.expected_deactivation_date_updated",
            V1FinancialConnectionsAccountExpectedDeactivationDateUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1FinancialConnectionsAccountExpectedDeactivationDateUpdatedType {
    fn default() -> Self {
        V1FinancialConnectionsAccountExpectedDeactivationDateUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1FinancialConnectionsAccountExpectedDeactivationDateUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1FinancialConnectionsAccountExpectedDeactivationDateUpdatedType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.financial_connections.account.expected_deactivation_date_updated" => V1FinancialConnectionsAccountExpectedDeactivationDateUpdatedType::V1FinancialConnectionsAccountExpectedDeactivationDateUpdated,
            other => V1FinancialConnectionsAccountExpectedDeactivationDateUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1InvoiceOverdueType {
    V1InvoiceOverdue,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1InvoiceOverdueType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1InvoiceOverdueType::V1InvoiceOverdue => "v1.invoice.overdue",
            V1InvoiceOverdueType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1InvoiceOverdueType {
    fn default() -> Self {
        V1InvoiceOverdueType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1InvoiceOverdueType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1InvoiceOverdueType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.invoice.overdue" => V1InvoiceOverdueType::V1InvoiceOverdue,
            other => V1InvoiceOverdueType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1InvoicePaidType {
    V1InvoicePaid,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1InvoicePaidType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1InvoicePaidType::V1InvoicePaid => "v1.invoice.paid",
            V1InvoicePaidType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1InvoicePaidType {
    fn default() -> Self {
        V1InvoicePaidType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1InvoicePaidType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1InvoicePaidType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.invoice.paid" => V1InvoicePaidType::V1InvoicePaid,
            other => V1InvoicePaidType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1InvoiceUpcomingType {
    V1InvoiceUpcoming,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1InvoiceUpcomingType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1InvoiceUpcomingType::V1InvoiceUpcoming => "v1.invoice.upcoming",
            V1InvoiceUpcomingType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1InvoiceUpcomingType {
    fn default() -> Self {
        V1InvoiceUpcomingType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1InvoiceUpcomingType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1InvoiceUpcomingType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.invoice.upcoming" => V1InvoiceUpcomingType::V1InvoiceUpcoming,
            other => V1InvoiceUpcomingType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1InvoiceVoidedType {
    V1InvoiceVoided,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1InvoiceVoidedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1InvoiceVoidedType::V1InvoiceVoided => "v1.invoice.voided",
            V1InvoiceVoidedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1InvoiceVoidedType {
    fn default() -> Self {
        V1InvoiceVoidedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1InvoiceVoidedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1InvoiceVoidedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.invoice.voided" => V1InvoiceVoidedType::V1InvoiceVoided,
            other => V1InvoiceVoidedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1InvoiceitemCreatedRelatedObjectType {
    Invoiceitem,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1InvoiceitemCreatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1InvoiceitemCreatedRelatedObjectType::Invoiceitem => "invoiceitem",
            V1InvoiceitemCreatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1InvoiceitemCreatedRelatedObjectType {
    fn default() -> Self {
        V1InvoiceitemCreatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1InvoiceitemCreatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1InvoiceitemCreatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "invoiceitem" => V1InvoiceitemCreatedRelatedObjectType::Invoiceitem,
            other => V1InvoiceitemCreatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IssuingAuthorizationRequestType {
    V1IssuingAuthorizationRequest,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IssuingAuthorizationRequestType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IssuingAuthorizationRequestType::V1IssuingAuthorizationRequest => "v1.issuing_authorization.request",
            V1IssuingAuthorizationRequestType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IssuingAuthorizationRequestType {
    fn default() -> Self {
        V1IssuingAuthorizationRequestType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IssuingAuthorizationRequestType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IssuingAuthorizationRequestType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.issuing_authorization.request" => V1IssuingAuthorizationRequestType::V1IssuingAuthorizationRequest,
            other => V1IssuingAuthorizationRequestType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IssuingDisputeClosedType {
    V1IssuingDisputeClosed,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IssuingDisputeClosedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IssuingDisputeClosedType::V1IssuingDisputeClosed => "v1.issuing_dispute.closed",
            V1IssuingDisputeClosedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IssuingDisputeClosedType {
    fn default() -> Self {
        V1IssuingDisputeClosedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IssuingDisputeClosedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IssuingDisputeClosedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.issuing_dispute.closed" => V1IssuingDisputeClosedType::V1IssuingDisputeClosed,
            other => V1IssuingDisputeClosedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IssuingDisputeUpdatedType {
    V1IssuingDisputeUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IssuingDisputeUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IssuingDisputeUpdatedType::V1IssuingDisputeUpdated => "v1.issuing_dispute.updated",
            V1IssuingDisputeUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IssuingDisputeUpdatedType {
    fn default() -> Self {
        V1IssuingDisputeUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IssuingDisputeUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IssuingDisputeUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.issuing_dispute.updated" => V1IssuingDisputeUpdatedType::V1IssuingDisputeUpdated,
            other => V1IssuingDisputeUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IssuingTokenCreatedType {
    V1IssuingTokenCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IssuingTokenCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IssuingTokenCreatedType::V1IssuingTokenCreated => "v1.issuing_token.created",
            V1IssuingTokenCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IssuingTokenCreatedType {
    fn default() -> Self {
        V1IssuingTokenCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IssuingTokenCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IssuingTokenCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.issuing_token.created" => V1IssuingTokenCreatedType::V1IssuingTokenCreated,
            other => V1IssuingTokenCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1MandateUpdatedRelatedObjectType {
    Mandate,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1MandateUpdatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1MandateUpdatedRelatedObjectType::Mandate => "mandate",
            V1MandateUpdatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1MandateUpdatedRelatedObjectType {
    fn default() -> Self {
        V1MandateUpdatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1MandateUpdatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1MandateUpdatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "mandate" => V1MandateUpdatedRelatedObjectType::Mandate,
            other => V1MandateUpdatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PaymentLinkCreatedRelatedObjectType {
    PaymentLink,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PaymentLinkCreatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PaymentLinkCreatedRelatedObjectType::PaymentLink => "payment_link",
            V1PaymentLinkCreatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PaymentLinkCreatedRelatedObjectType {
    fn default() -> Self {
        V1PaymentLinkCreatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PaymentLinkCreatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PaymentLinkCreatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "payment_link" => V1PaymentLinkCreatedRelatedObjectType::PaymentLink,
            other => V1PaymentLinkCreatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PaymentLinkCreatedType {
    V1PaymentLinkCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PaymentLinkCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PaymentLinkCreatedType::V1PaymentLinkCreated => "v1.payment_link.created",
            V1PaymentLinkCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PaymentLinkCreatedType {
    fn default() -> Self {
        V1PaymentLinkCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PaymentLinkCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PaymentLinkCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.payment_link.created" => V1PaymentLinkCreatedType::V1PaymentLinkCreated,
            other => V1PaymentLinkCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PersonUpdatedType {
    V1PersonUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PersonUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PersonUpdatedType::V1PersonUpdated => "v1.person.updated",
            V1PersonUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PersonUpdatedType {
    fn default() -> Self {
        V1PersonUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PersonUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PersonUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.person.updated" => V1PersonUpdatedType::V1PersonUpdated,
            other => V1PersonUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PlanCreatedRelatedObjectType {
    Plan,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PlanCreatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PlanCreatedRelatedObjectType::Plan => "plan",
            V1PlanCreatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PlanCreatedRelatedObjectType {
    fn default() -> Self {
        V1PlanCreatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PlanCreatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PlanCreatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "plan" => V1PlanCreatedRelatedObjectType::Plan,
            other => V1PlanCreatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PriceUpdatedType {
    V1PriceUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PriceUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PriceUpdatedType::V1PriceUpdated => "v1.price.updated",
            V1PriceUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PriceUpdatedType {
    fn default() -> Self {
        V1PriceUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PriceUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PriceUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.price.updated" => V1PriceUpdatedType::V1PriceUpdated,
            other => V1PriceUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ProductDeletedType {
    V1ProductDeleted,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ProductDeletedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ProductDeletedType::V1ProductDeleted => "v1.product.deleted",
            V1ProductDeletedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ProductDeletedType {
    fn default() -> Self {
        V1ProductDeletedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ProductDeletedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ProductDeletedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.product.deleted" => V1ProductDeletedType::V1ProductDeleted,
            other => V1ProductDeletedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1RadarEarlyFraudWarningCreatedRelatedObjectType {
    RadarEarlyFraudWarning,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1RadarEarlyFraudWarningCreatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1RadarEarlyFraudWarningCreatedRelatedObjectType::RadarEarlyFraudWarning => "radar.early_fraud_warning",
            V1RadarEarlyFraudWarningCreatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1RadarEarlyFraudWarningCreatedRelatedObjectType {
    fn default() -> Self {
        V1RadarEarlyFraudWarningCreatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1RadarEarlyFraudWarningCreatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1RadarEarlyFraudWarningCreatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "radar.early_fraud_warning" => V1RadarEarlyFraudWarningCreatedRelatedObjectType::RadarEarlyFraudWarning,
            other => V1RadarEarlyFraudWarningCreatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1SetupIntentSucceededType {
    V1SetupIntentSucceeded,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1SetupIntentSucceededType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1SetupIntentSucceededType::V1SetupIntentSucceeded => "v1.setup_intent.succeeded",
            V1SetupIntentSucceededType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1SetupIntentSucceededType {
    fn default() -> Self {
        V1SetupIntentSucceededType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1SetupIntentSucceededType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1SetupIntentSucceededType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.setup_intent.succeeded" => V1SetupIntentSucceededType::V1SetupIntentSucceeded,
            other => V1SetupIntentSucceededType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1SourceCanceledRelatedObjectType {
    Source,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1SourceCanceledRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1SourceCanceledRelatedObjectType::Source => "source",
            V1SourceCanceledRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1SourceCanceledRelatedObjectType {
    fn default() -> Self {
        V1SourceCanceledRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1SourceCanceledRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1SourceCanceledRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "source" => V1SourceCanceledRelatedObjectType::Source,
            other => V1SourceCanceledRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1SourceFailedType {
    V1SourceFailed,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1SourceFailedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1SourceFailedType::V1SourceFailed => "v1.source.failed",
            V1SourceFailedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1SourceFailedType {
    fn default() -> Self {
        V1SourceFailedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1SourceFailedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1SourceFailedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.source.failed" => V1SourceFailedType::V1SourceFailed,
            other => V1SourceFailedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1SubscriptionScheduleAbortedRelatedObjectType {
    SubscriptionSchedule,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1SubscriptionScheduleAbortedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1SubscriptionScheduleAbortedRelatedObjectType::SubscriptionSchedule => "subscription_schedule",
            V1SubscriptionScheduleAbortedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1SubscriptionScheduleAbortedRelatedObjectType {
    fn default() -> Self {
        V1SubscriptionScheduleAbortedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1SubscriptionScheduleAbortedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1SubscriptionScheduleAbortedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "subscription_schedule" => V1SubscriptionScheduleAbortedRelatedObjectType::SubscriptionSchedule,
            other => V1SubscriptionScheduleAbortedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1TestHelpersTestClockAdvancingType {
    V1TestHelpersTestClockAdvancing,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1TestHelpersTestClockAdvancingType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1TestHelpersTestClockAdvancingType::V1TestHelpersTestClockAdvancing => {
                "v1.test_helpers.test_clock.advancing"
            }
            V1TestHelpersTestClockAdvancingType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1TestHelpersTestClockAdvancingType {
    fn default() -> Self {
        V1TestHelpersTestClockAdvancingType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1TestHelpersTestClockAdvancingType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1TestHelpersTestClockAdvancingType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.test_helpers.test_clock.advancing" => {
                V1TestHelpersTestClockAdvancingType::V1TestHelpersTestClockAdvancing
            }
            other => V1TestHelpersTestClockAdvancingType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1TestHelpersTestClockCreatedType {
    V1TestHelpersTestClockCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1TestHelpersTestClockCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1TestHelpersTestClockCreatedType::V1TestHelpersTestClockCreated => "v1.test_helpers.test_clock.created",
            V1TestHelpersTestClockCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1TestHelpersTestClockCreatedType {
    fn default() -> Self {
        V1TestHelpersTestClockCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1TestHelpersTestClockCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1TestHelpersTestClockCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.test_helpers.test_clock.created" => V1TestHelpersTestClockCreatedType::V1TestHelpersTestClockCreated,
            other => V1TestHelpersTestClockCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1TestHelpersTestClockInternalFailureType {
    V1TestHelpersTestClockInternalFailure,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1TestHelpersTestClockInternalFailureType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1TestHelpersTestClockInternalFailureType::V1TestHelpersTestClockInternalFailure => {
                "v1.test_helpers.test_clock.internal_failure"
            }
            V1TestHelpersTestClockInternalFailureType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1TestHelpersTestClockInternalFailureType {
    fn default() -> Self {
        V1TestHelpersTestClockInternalFailureType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1TestHelpersTestClockInternalFailureType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1TestHelpersTestClockInternalFailureType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.test_helpers.test_clock.internal_failure" => {
                V1TestHelpersTestClockInternalFailureType::V1TestHelpersTestClockInternalFailure
            }
            other => V1TestHelpersTestClockInternalFailureType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1TopupCanceledType {
    V1TopupCanceled,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1TopupCanceledType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1TopupCanceledType::V1TopupCanceled => "v1.topup.canceled",
            V1TopupCanceledType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1TopupCanceledType {
    fn default() -> Self {
        V1TopupCanceledType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1TopupCanceledType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1TopupCanceledType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.topup.canceled" => V1TopupCanceledType::V1TopupCanceled,
            other => V1TopupCanceledType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1TopupFailedType {
    V1TopupFailed,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1TopupFailedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1TopupFailedType::V1TopupFailed => "v1.topup.failed",
            V1TopupFailedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1TopupFailedType {
    fn default() -> Self {
        V1TopupFailedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1TopupFailedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1TopupFailedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.topup.failed" => V1TopupFailedType::V1TopupFailed,
            other => V1TopupFailedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CommerceProductCatalogImportsFailedRelatedObjectType {
    V2CommerceProductCatalogImport,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V2CommerceProductCatalogImportsFailedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CommerceProductCatalogImportsFailedRelatedObjectType::V2CommerceProductCatalogImport => {
                "v2.commerce.product_catalog_import"
            }
            V2CommerceProductCatalogImportsFailedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V2CommerceProductCatalogImportsFailedRelatedObjectType {
    fn default() -> Self {
        V2CommerceProductCatalogImportsFailedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V2CommerceProductCatalogImportsFailedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CommerceProductCatalogImportsFailedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.commerce.product_catalog_import" => {
                V2CommerceProductCatalogImportsFailedRelatedObjectType::V2CommerceProductCatalogImport
            }
            other => V2CommerceProductCatalogImportsFailedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CommerceProductCatalogImportsFailedType {
    V2CommerceProductCatalogImportsFailed,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V2CommerceProductCatalogImportsFailedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CommerceProductCatalogImportsFailedType::V2CommerceProductCatalogImportsFailed => {
                "v2.commerce.product_catalog.imports.failed"
            }
            V2CommerceProductCatalogImportsFailedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V2CommerceProductCatalogImportsFailedType {
    fn default() -> Self {
        V2CommerceProductCatalogImportsFailedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V2CommerceProductCatalogImportsFailedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CommerceProductCatalogImportsFailedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.commerce.product_catalog.imports.failed" => {
                V2CommerceProductCatalogImportsFailedType::V2CommerceProductCatalogImportsFailed
            }
            other => V2CommerceProductCatalogImportsFailedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountConfigurationCustomerCapabilityStatusUpdatedType {
    V2CoreAccountConfigurationCustomerCapabilityStatusUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V2CoreAccountConfigurationCustomerCapabilityStatusUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountConfigurationCustomerCapabilityStatusUpdatedType::V2CoreAccountConfigurationCustomerCapabilityStatusUpdated => "v2.core.account[configuration.customer].capability_status_updated",
            V2CoreAccountConfigurationCustomerCapabilityStatusUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V2CoreAccountConfigurationCustomerCapabilityStatusUpdatedType {
    fn default() -> Self {
        V2CoreAccountConfigurationCustomerCapabilityStatusUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V2CoreAccountConfigurationCustomerCapabilityStatusUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountConfigurationCustomerCapabilityStatusUpdatedType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.core.account[configuration.customer].capability_status_updated" => V2CoreAccountConfigurationCustomerCapabilityStatusUpdatedType::V2CoreAccountConfigurationCustomerCapabilityStatusUpdated,
            other => V2CoreAccountConfigurationCustomerCapabilityStatusUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedDataUpdatedCapability {
    BankAccountsLocal,
    BankAccountsWire,
    Cards,
    StripeBalancePayouts,
    StripeBalanceStripeTransfers,
    StripeTransfers,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedDataUpdatedCapability {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedDataUpdatedCapability::BankAccountsLocal => "bank_accounts.local",
            V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedDataUpdatedCapability::BankAccountsWire => "bank_accounts.wire",
            V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedDataUpdatedCapability::Cards => "cards",
            V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedDataUpdatedCapability::StripeBalancePayouts => "stripe_balance.payouts",
            V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedDataUpdatedCapability::StripeBalanceStripeTransfers => "stripe_balance.stripe_transfers",
            V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedDataUpdatedCapability::StripeTransfers => "stripe.transfers",
            V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedDataUpdatedCapability::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedDataUpdatedCapability {
    fn default() -> Self {
        V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedDataUpdatedCapability::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedDataUpdatedCapability {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedDataUpdatedCapability::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "bank_accounts.local" => V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedDataUpdatedCapability::BankAccountsLocal,
            "bank_accounts.wire" => V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedDataUpdatedCapability::BankAccountsWire,
            "cards" => V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedDataUpdatedCapability::Cards,
            "stripe_balance.payouts" => V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedDataUpdatedCapability::StripeBalancePayouts,
            "stripe_balance.stripe_transfers" => V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedDataUpdatedCapability::StripeBalanceStripeTransfers,
            "stripe.transfers" => V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedDataUpdatedCapability::StripeTransfers,
            other => V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedDataUpdatedCapability::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountConfigurationRecipientUpdatedType {
    V2CoreAccountConfigurationRecipientUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V2CoreAccountConfigurationRecipientUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountConfigurationRecipientUpdatedType::V2CoreAccountConfigurationRecipientUpdated => {
                "v2.core.account[configuration.recipient].updated"
            }
            V2CoreAccountConfigurationRecipientUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V2CoreAccountConfigurationRecipientUpdatedType {
    fn default() -> Self {
        V2CoreAccountConfigurationRecipientUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V2CoreAccountConfigurationRecipientUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountConfigurationRecipientUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.core.account[configuration.recipient].updated" => {
                V2CoreAccountConfigurationRecipientUpdatedType::V2CoreAccountConfigurationRecipientUpdated
            }
            other => V2CoreAccountConfigurationRecipientUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountCreatedChanges {
    #[doc = "Updated values of properties that the event changed."]
    #[json(default)]
    pub after: json::OwnedValue,
    #[doc = "Values of properties before the event changes."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub before: Option<json::OwnedValue>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountFutureRequirementsUpdatedType {
    V2CoreAccountFutureRequirementsUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V2CoreAccountFutureRequirementsUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountFutureRequirementsUpdatedType::V2CoreAccountFutureRequirementsUpdated => {
                "v2.core.account[future_requirements].updated"
            }
            V2CoreAccountFutureRequirementsUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V2CoreAccountFutureRequirementsUpdatedType {
    fn default() -> Self {
        V2CoreAccountFutureRequirementsUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V2CoreAccountFutureRequirementsUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountFutureRequirementsUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.core.account[future_requirements].updated" => {
                V2CoreAccountFutureRequirementsUpdatedType::V2CoreAccountFutureRequirementsUpdated
            }
            other => V2CoreAccountFutureRequirementsUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountLinkReturnedDataConfigurationsItem {
    Customer,
    Merchant,
    Recipient,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V2CoreAccountLinkReturnedDataConfigurationsItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountLinkReturnedDataConfigurationsItem::Customer => "customer",
            V2CoreAccountLinkReturnedDataConfigurationsItem::Merchant => "merchant",
            V2CoreAccountLinkReturnedDataConfigurationsItem::Recipient => "recipient",
            V2CoreAccountLinkReturnedDataConfigurationsItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V2CoreAccountLinkReturnedDataConfigurationsItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountLinkReturnedDataConfigurationsItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "customer" => V2CoreAccountLinkReturnedDataConfigurationsItem::Customer,
            "merchant" => V2CoreAccountLinkReturnedDataConfigurationsItem::Merchant,
            "recipient" => V2CoreAccountLinkReturnedDataConfigurationsItem::Recipient,
            other => V2CoreAccountLinkReturnedDataConfigurationsItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountUpdatedType {
    V2CoreAccountUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V2CoreAccountUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountUpdatedType::V2CoreAccountUpdated => "v2.core.account.updated",
            V2CoreAccountUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V2CoreAccountUpdatedType {
    fn default() -> Self {
        V2CoreAccountUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V2CoreAccountUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.core.account.updated" => V2CoreAccountUpdatedType::V2CoreAccountUpdated,
            other => V2CoreAccountUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreEventDestinationPingType {
    V2CoreEventDestinationPing,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V2CoreEventDestinationPingType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreEventDestinationPingType::V2CoreEventDestinationPing => "v2.core.event_destination.ping",
            V2CoreEventDestinationPingType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V2CoreEventDestinationPingType {
    fn default() -> Self {
        V2CoreEventDestinationPingType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V2CoreEventDestinationPingType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreEventDestinationPingType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.core.event_destination.ping" => V2CoreEventDestinationPingType::V2CoreEventDestinationPing,
            other => V2CoreEventDestinationPingType::Unknown(other.to_string()),
        })
    }
}
