#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum EventObject {
    Event,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl EventObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            EventObject::Event => "event",
            EventObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for EventObject {
    fn default() -> Self {
        EventObject::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for EventObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(EventObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "event" => EventObject::Event,
            other => EventObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct NotificationEventData {
    #[doc = "Object containing the API resource relevant to the event."]
    #[json(default)]
    pub object: std::collections::HashMap<String, json::OwnedValue>,
    #[doc = "Object containing the names of the updated attributes and their values prior to the event (only included in events of..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub previous_attributes: Option<std::collections::HashMap<String, json::OwnedValue>>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1AccountApplicationAuthorizedData {}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1AccountApplicationAuthorizedObject {
    V2CoreEvent,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1AccountApplicationAuthorizedObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1AccountApplicationAuthorizedObject::V2CoreEvent => "v2.core.event",
            V1AccountApplicationAuthorizedObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1AccountApplicationAuthorizedObject {
    fn default() -> Self {
        V1AccountApplicationAuthorizedObject::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1AccountApplicationAuthorizedObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1AccountApplicationAuthorizedObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.core.event" => V1AccountApplicationAuthorizedObject::V2CoreEvent,
            other => V1AccountApplicationAuthorizedObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1AccountApplicationAuthorizedType {
    V1AccountApplicationAuthorized,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1AccountApplicationAuthorizedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1AccountApplicationAuthorizedType::V1AccountApplicationAuthorized => "v1.account.application.authorized",
            V1AccountApplicationAuthorizedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1AccountApplicationAuthorizedType {
    fn default() -> Self {
        V1AccountApplicationAuthorizedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1AccountApplicationAuthorizedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1AccountApplicationAuthorizedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.account.application.authorized" => V1AccountApplicationAuthorizedType::V1AccountApplicationAuthorized,
            other => V1AccountApplicationAuthorizedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1AccountExternalAccountUpdatedType {
    V1AccountExternalAccountUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1AccountExternalAccountUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1AccountExternalAccountUpdatedType::V1AccountExternalAccountUpdated => {
                "v1.account.external_account.updated"
            }
            V1AccountExternalAccountUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1AccountExternalAccountUpdatedType {
    fn default() -> Self {
        V1AccountExternalAccountUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1AccountExternalAccountUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1AccountExternalAccountUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.account.external_account.updated" => {
                V1AccountExternalAccountUpdatedType::V1AccountExternalAccountUpdated
            }
            other => V1AccountExternalAccountUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1BalanceAvailableRelatedObjectType {
    Balance,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1BalanceAvailableRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1BalanceAvailableRelatedObjectType::Balance => "balance",
            V1BalanceAvailableRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1BalanceAvailableRelatedObjectType {
    fn default() -> Self {
        V1BalanceAvailableRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1BalanceAvailableRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1BalanceAvailableRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "balance" => V1BalanceAvailableRelatedObjectType::Balance,
            other => V1BalanceAvailableRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1BalanceAvailableType {
    V1BalanceAvailable,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1BalanceAvailableType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1BalanceAvailableType::V1BalanceAvailable => "v1.balance.available",
            V1BalanceAvailableType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1BalanceAvailableType {
    fn default() -> Self {
        V1BalanceAvailableType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1BalanceAvailableType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1BalanceAvailableType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.balance.available" => V1BalanceAvailableType::V1BalanceAvailable,
            other => V1BalanceAvailableType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1BillingMeterErrorReportTriggeredType {
    V1BillingMeterErrorReportTriggered,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1BillingMeterErrorReportTriggeredType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1BillingMeterErrorReportTriggeredType::V1BillingMeterErrorReportTriggered => {
                "v1.billing.meter.error_report_triggered"
            }
            V1BillingMeterErrorReportTriggeredType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1BillingMeterErrorReportTriggeredType {
    fn default() -> Self {
        V1BillingMeterErrorReportTriggeredType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1BillingMeterErrorReportTriggeredType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1BillingMeterErrorReportTriggeredType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.billing.meter.error_report_triggered" => {
                V1BillingMeterErrorReportTriggeredType::V1BillingMeterErrorReportTriggered
            }
            other => V1BillingMeterErrorReportTriggeredType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ChargeCapturedRelatedObjectType {
    Charge,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ChargeCapturedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ChargeCapturedRelatedObjectType::Charge => "charge",
            V1ChargeCapturedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ChargeCapturedRelatedObjectType {
    fn default() -> Self {
        V1ChargeCapturedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ChargeCapturedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ChargeCapturedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "charge" => V1ChargeCapturedRelatedObjectType::Charge,
            other => V1ChargeCapturedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ChargeDisputeCreatedType {
    V1ChargeDisputeCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ChargeDisputeCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ChargeDisputeCreatedType::V1ChargeDisputeCreated => "v1.charge.dispute.created",
            V1ChargeDisputeCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ChargeDisputeCreatedType {
    fn default() -> Self {
        V1ChargeDisputeCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ChargeDisputeCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ChargeDisputeCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.charge.dispute.created" => V1ChargeDisputeCreatedType::V1ChargeDisputeCreated,
            other => V1ChargeDisputeCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ChargeUpdatedType {
    V1ChargeUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ChargeUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ChargeUpdatedType::V1ChargeUpdated => "v1.charge.updated",
            V1ChargeUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ChargeUpdatedType {
    fn default() -> Self {
        V1ChargeUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ChargeUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ChargeUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.charge.updated" => V1ChargeUpdatedType::V1ChargeUpdated,
            other => V1ChargeUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CheckoutSessionCompletedType {
    V1CheckoutSessionCompleted,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CheckoutSessionCompletedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CheckoutSessionCompletedType::V1CheckoutSessionCompleted => "v1.checkout.session.completed",
            V1CheckoutSessionCompletedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CheckoutSessionCompletedType {
    fn default() -> Self {
        V1CheckoutSessionCompletedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CheckoutSessionCompletedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CheckoutSessionCompletedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.checkout.session.completed" => V1CheckoutSessionCompletedType::V1CheckoutSessionCompleted,
            other => V1CheckoutSessionCompletedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CheckoutSessionExpiredType {
    V1CheckoutSessionExpired,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CheckoutSessionExpiredType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CheckoutSessionExpiredType::V1CheckoutSessionExpired => "v1.checkout.session.expired",
            V1CheckoutSessionExpiredType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CheckoutSessionExpiredType {
    fn default() -> Self {
        V1CheckoutSessionExpiredType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CheckoutSessionExpiredType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CheckoutSessionExpiredType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.checkout.session.expired" => V1CheckoutSessionExpiredType::V1CheckoutSessionExpired,
            other => V1CheckoutSessionExpiredType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ClimateOrderCanceledRelatedObjectType {
    ClimateOrder,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ClimateOrderCanceledRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ClimateOrderCanceledRelatedObjectType::ClimateOrder => "climate.order",
            V1ClimateOrderCanceledRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ClimateOrderCanceledRelatedObjectType {
    fn default() -> Self {
        V1ClimateOrderCanceledRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ClimateOrderCanceledRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ClimateOrderCanceledRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "climate.order" => V1ClimateOrderCanceledRelatedObjectType::ClimateOrder,
            other => V1ClimateOrderCanceledRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ClimateOrderCreatedType {
    V1ClimateOrderCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ClimateOrderCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ClimateOrderCreatedType::V1ClimateOrderCreated => "v1.climate.order.created",
            V1ClimateOrderCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ClimateOrderCreatedType {
    fn default() -> Self {
        V1ClimateOrderCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ClimateOrderCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ClimateOrderCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.climate.order.created" => V1ClimateOrderCreatedType::V1ClimateOrderCreated,
            other => V1ClimateOrderCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ClimateOrderProductSubstitutedType {
    V1ClimateOrderProductSubstituted,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ClimateOrderProductSubstitutedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ClimateOrderProductSubstitutedType::V1ClimateOrderProductSubstituted => {
                "v1.climate.order.product_substituted"
            }
            V1ClimateOrderProductSubstitutedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ClimateOrderProductSubstitutedType {
    fn default() -> Self {
        V1ClimateOrderProductSubstitutedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ClimateOrderProductSubstitutedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ClimateOrderProductSubstitutedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.climate.order.product_substituted" => {
                V1ClimateOrderProductSubstitutedType::V1ClimateOrderProductSubstituted
            }
            other => V1ClimateOrderProductSubstitutedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ClimateProductCreatedType {
    V1ClimateProductCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ClimateProductCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ClimateProductCreatedType::V1ClimateProductCreated => "v1.climate.product.created",
            V1ClimateProductCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ClimateProductCreatedType {
    fn default() -> Self {
        V1ClimateProductCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ClimateProductCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ClimateProductCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.climate.product.created" => V1ClimateProductCreatedType::V1ClimateProductCreated,
            other => V1ClimateProductCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ClimateProductPricingUpdatedType {
    V1ClimateProductPricingUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ClimateProductPricingUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ClimateProductPricingUpdatedType::V1ClimateProductPricingUpdated => "v1.climate.product.pricing_updated",
            V1ClimateProductPricingUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ClimateProductPricingUpdatedType {
    fn default() -> Self {
        V1ClimateProductPricingUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ClimateProductPricingUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ClimateProductPricingUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.climate.product.pricing_updated" => V1ClimateProductPricingUpdatedType::V1ClimateProductPricingUpdated,
            other => V1ClimateProductPricingUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CreditNoteVoidedType {
    V1CreditNoteVoided,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CreditNoteVoidedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CreditNoteVoidedType::V1CreditNoteVoided => "v1.credit_note.voided",
            V1CreditNoteVoidedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CreditNoteVoidedType {
    fn default() -> Self {
        V1CreditNoteVoidedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CreditNoteVoidedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CreditNoteVoidedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.credit_note.voided" => V1CreditNoteVoidedType::V1CreditNoteVoided,
            other => V1CreditNoteVoidedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CustomerDeletedType {
    V1CustomerDeleted,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CustomerDeletedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CustomerDeletedType::V1CustomerDeleted => "v1.customer.deleted",
            V1CustomerDeletedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CustomerDeletedType {
    fn default() -> Self {
        V1CustomerDeletedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CustomerDeletedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CustomerDeletedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.customer.deleted" => V1CustomerDeletedType::V1CustomerDeleted,
            other => V1CustomerDeletedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CustomerSubscriptionTrialWillEndType {
    V1CustomerSubscriptionTrialWillEnd,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CustomerSubscriptionTrialWillEndType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CustomerSubscriptionTrialWillEndType::V1CustomerSubscriptionTrialWillEnd => {
                "v1.customer.subscription.trial_will_end"
            }
            V1CustomerSubscriptionTrialWillEndType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CustomerSubscriptionTrialWillEndType {
    fn default() -> Self {
        V1CustomerSubscriptionTrialWillEndType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CustomerSubscriptionTrialWillEndType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CustomerSubscriptionTrialWillEndType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.customer.subscription.trial_will_end" => {
                V1CustomerSubscriptionTrialWillEndType::V1CustomerSubscriptionTrialWillEnd
            }
            other => V1CustomerSubscriptionTrialWillEndType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CustomerSubscriptionUpdatedType {
    V1CustomerSubscriptionUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CustomerSubscriptionUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CustomerSubscriptionUpdatedType::V1CustomerSubscriptionUpdated => "v1.customer.subscription.updated",
            V1CustomerSubscriptionUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CustomerSubscriptionUpdatedType {
    fn default() -> Self {
        V1CustomerSubscriptionUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CustomerSubscriptionUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CustomerSubscriptionUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.customer.subscription.updated" => V1CustomerSubscriptionUpdatedType::V1CustomerSubscriptionUpdated,
            other => V1CustomerSubscriptionUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1FileCreatedType {
    V1FileCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1FileCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1FileCreatedType::V1FileCreated => "v1.file.created",
            V1FileCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1FileCreatedType {
    fn default() -> Self {
        V1FileCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1FileCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1FileCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.file.created" => V1FileCreatedType::V1FileCreated,
            other => V1FileCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IdentityVerificationSessionCanceledRelatedObjectType {
    IdentityVerificationSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IdentityVerificationSessionCanceledRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IdentityVerificationSessionCanceledRelatedObjectType::IdentityVerificationSession => {
                "identity.verification_session"
            }
            V1IdentityVerificationSessionCanceledRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IdentityVerificationSessionCanceledRelatedObjectType {
    fn default() -> Self {
        V1IdentityVerificationSessionCanceledRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IdentityVerificationSessionCanceledRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IdentityVerificationSessionCanceledRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "identity.verification_session" => {
                V1IdentityVerificationSessionCanceledRelatedObjectType::IdentityVerificationSession
            }
            other => V1IdentityVerificationSessionCanceledRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1InvoiceCreatedRelatedObjectType {
    Invoice,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1InvoiceCreatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1InvoiceCreatedRelatedObjectType::Invoice => "invoice",
            V1InvoiceCreatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1InvoiceCreatedRelatedObjectType {
    fn default() -> Self {
        V1InvoiceCreatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1InvoiceCreatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1InvoiceCreatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "invoice" => V1InvoiceCreatedRelatedObjectType::Invoice,
            other => V1InvoiceCreatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1InvoiceCreatedType {
    V1InvoiceCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1InvoiceCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1InvoiceCreatedType::V1InvoiceCreated => "v1.invoice.created",
            V1InvoiceCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1InvoiceCreatedType {
    fn default() -> Self {
        V1InvoiceCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1InvoiceCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1InvoiceCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.invoice.created" => V1InvoiceCreatedType::V1InvoiceCreated,
            other => V1InvoiceCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1InvoiceMarkedUncollectibleType {
    V1InvoiceMarkedUncollectible,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1InvoiceMarkedUncollectibleType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1InvoiceMarkedUncollectibleType::V1InvoiceMarkedUncollectible => "v1.invoice.marked_uncollectible",
            V1InvoiceMarkedUncollectibleType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1InvoiceMarkedUncollectibleType {
    fn default() -> Self {
        V1InvoiceMarkedUncollectibleType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1InvoiceMarkedUncollectibleType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1InvoiceMarkedUncollectibleType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.invoice.marked_uncollectible" => V1InvoiceMarkedUncollectibleType::V1InvoiceMarkedUncollectible,
            other => V1InvoiceMarkedUncollectibleType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1InvoicePaymentSucceededType {
    V1InvoicePaymentSucceeded,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1InvoicePaymentSucceededType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1InvoicePaymentSucceededType::V1InvoicePaymentSucceeded => "v1.invoice.payment_succeeded",
            V1InvoicePaymentSucceededType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1InvoicePaymentSucceededType {
    fn default() -> Self {
        V1InvoicePaymentSucceededType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1InvoicePaymentSucceededType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1InvoicePaymentSucceededType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.invoice.payment_succeeded" => V1InvoicePaymentSucceededType::V1InvoicePaymentSucceeded,
            other => V1InvoicePaymentSucceededType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1InvoiceitemCreatedType {
    V1InvoiceitemCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1InvoiceitemCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1InvoiceitemCreatedType::V1InvoiceitemCreated => "v1.invoiceitem.created",
            V1InvoiceitemCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1InvoiceitemCreatedType {
    fn default() -> Self {
        V1InvoiceitemCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1InvoiceitemCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1InvoiceitemCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.invoiceitem.created" => V1InvoiceitemCreatedType::V1InvoiceitemCreated,
            other => V1InvoiceitemCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IssuingCardUpdatedType {
    V1IssuingCardUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IssuingCardUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IssuingCardUpdatedType::V1IssuingCardUpdated => "v1.issuing_card.updated",
            V1IssuingCardUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IssuingCardUpdatedType {
    fn default() -> Self {
        V1IssuingCardUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IssuingCardUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IssuingCardUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.issuing_card.updated" => V1IssuingCardUpdatedType::V1IssuingCardUpdated,
            other => V1IssuingCardUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IssuingCardholderCreatedRelatedObjectType {
    IssuingCardholder,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IssuingCardholderCreatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IssuingCardholderCreatedRelatedObjectType::IssuingCardholder => "issuing.cardholder",
            V1IssuingCardholderCreatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IssuingCardholderCreatedRelatedObjectType {
    fn default() -> Self {
        V1IssuingCardholderCreatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IssuingCardholderCreatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IssuingCardholderCreatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "issuing.cardholder" => V1IssuingCardholderCreatedRelatedObjectType::IssuingCardholder,
            other => V1IssuingCardholderCreatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IssuingDisputeFundsRescindedType {
    V1IssuingDisputeFundsRescinded,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IssuingDisputeFundsRescindedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IssuingDisputeFundsRescindedType::V1IssuingDisputeFundsRescinded => "v1.issuing_dispute.funds_rescinded",
            V1IssuingDisputeFundsRescindedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IssuingDisputeFundsRescindedType {
    fn default() -> Self {
        V1IssuingDisputeFundsRescindedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IssuingDisputeFundsRescindedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IssuingDisputeFundsRescindedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.issuing_dispute.funds_rescinded" => V1IssuingDisputeFundsRescindedType::V1IssuingDisputeFundsRescinded,
            other => V1IssuingDisputeFundsRescindedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IssuingPersonalizationDesignActivatedType {
    V1IssuingPersonalizationDesignActivated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IssuingPersonalizationDesignActivatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IssuingPersonalizationDesignActivatedType::V1IssuingPersonalizationDesignActivated => {
                "v1.issuing_personalization_design.activated"
            }
            V1IssuingPersonalizationDesignActivatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IssuingPersonalizationDesignActivatedType {
    fn default() -> Self {
        V1IssuingPersonalizationDesignActivatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IssuingPersonalizationDesignActivatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IssuingPersonalizationDesignActivatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.issuing_personalization_design.activated" => {
                V1IssuingPersonalizationDesignActivatedType::V1IssuingPersonalizationDesignActivated
            }
            other => V1IssuingPersonalizationDesignActivatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IssuingPersonalizationDesignRejectedType {
    V1IssuingPersonalizationDesignRejected,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IssuingPersonalizationDesignRejectedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IssuingPersonalizationDesignRejectedType::V1IssuingPersonalizationDesignRejected => {
                "v1.issuing_personalization_design.rejected"
            }
            V1IssuingPersonalizationDesignRejectedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IssuingPersonalizationDesignRejectedType {
    fn default() -> Self {
        V1IssuingPersonalizationDesignRejectedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IssuingPersonalizationDesignRejectedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IssuingPersonalizationDesignRejectedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.issuing_personalization_design.rejected" => {
                V1IssuingPersonalizationDesignRejectedType::V1IssuingPersonalizationDesignRejected
            }
            other => V1IssuingPersonalizationDesignRejectedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IssuingTransactionPurchaseDetailsReceiptUpdatedType {
    V1IssuingTransactionPurchaseDetailsReceiptUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IssuingTransactionPurchaseDetailsReceiptUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IssuingTransactionPurchaseDetailsReceiptUpdatedType::V1IssuingTransactionPurchaseDetailsReceiptUpdated => "v1.issuing_transaction.purchase_details_receipt_updated",
            V1IssuingTransactionPurchaseDetailsReceiptUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IssuingTransactionPurchaseDetailsReceiptUpdatedType {
    fn default() -> Self {
        V1IssuingTransactionPurchaseDetailsReceiptUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IssuingTransactionPurchaseDetailsReceiptUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IssuingTransactionPurchaseDetailsReceiptUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.issuing_transaction.purchase_details_receipt_updated" => {
                V1IssuingTransactionPurchaseDetailsReceiptUpdatedType::V1IssuingTransactionPurchaseDetailsReceiptUpdated
            }
            other => V1IssuingTransactionPurchaseDetailsReceiptUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PaymentIntentCreatedType {
    V1PaymentIntentCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PaymentIntentCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PaymentIntentCreatedType::V1PaymentIntentCreated => "v1.payment_intent.created",
            V1PaymentIntentCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PaymentIntentCreatedType {
    fn default() -> Self {
        V1PaymentIntentCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PaymentIntentCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PaymentIntentCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.payment_intent.created" => V1PaymentIntentCreatedType::V1PaymentIntentCreated,
            other => V1PaymentIntentCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PaymentIntentRequiresActionType {
    V1PaymentIntentRequiresAction,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PaymentIntentRequiresActionType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PaymentIntentRequiresActionType::V1PaymentIntentRequiresAction => "v1.payment_intent.requires_action",
            V1PaymentIntentRequiresActionType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PaymentIntentRequiresActionType {
    fn default() -> Self {
        V1PaymentIntentRequiresActionType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PaymentIntentRequiresActionType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PaymentIntentRequiresActionType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.payment_intent.requires_action" => V1PaymentIntentRequiresActionType::V1PaymentIntentRequiresAction,
            other => V1PaymentIntentRequiresActionType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PaymentMethodAutomaticallyUpdatedType {
    V1PaymentMethodAutomaticallyUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PaymentMethodAutomaticallyUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PaymentMethodAutomaticallyUpdatedType::V1PaymentMethodAutomaticallyUpdated => {
                "v1.payment_method.automatically_updated"
            }
            V1PaymentMethodAutomaticallyUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PaymentMethodAutomaticallyUpdatedType {
    fn default() -> Self {
        V1PaymentMethodAutomaticallyUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PaymentMethodAutomaticallyUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PaymentMethodAutomaticallyUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.payment_method.automatically_updated" => {
                V1PaymentMethodAutomaticallyUpdatedType::V1PaymentMethodAutomaticallyUpdated
            }
            other => V1PaymentMethodAutomaticallyUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PaymentMethodDetachedType {
    V1PaymentMethodDetached,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PaymentMethodDetachedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PaymentMethodDetachedType::V1PaymentMethodDetached => "v1.payment_method.detached",
            V1PaymentMethodDetachedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PaymentMethodDetachedType {
    fn default() -> Self {
        V1PaymentMethodDetachedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PaymentMethodDetachedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PaymentMethodDetachedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.payment_method.detached" => V1PaymentMethodDetachedType::V1PaymentMethodDetached,
            other => V1PaymentMethodDetachedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PayoutCanceledType {
    V1PayoutCanceled,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PayoutCanceledType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PayoutCanceledType::V1PayoutCanceled => "v1.payout.canceled",
            V1PayoutCanceledType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PayoutCanceledType {
    fn default() -> Self {
        V1PayoutCanceledType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PayoutCanceledType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PayoutCanceledType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.payout.canceled" => V1PayoutCanceledType::V1PayoutCanceled,
            other => V1PayoutCanceledType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PayoutCreatedType {
    V1PayoutCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PayoutCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PayoutCreatedType::V1PayoutCreated => "v1.payout.created",
            V1PayoutCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PayoutCreatedType {
    fn default() -> Self {
        V1PayoutCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PayoutCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PayoutCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.payout.created" => V1PayoutCreatedType::V1PayoutCreated,
            other => V1PayoutCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PayoutReconciliationCompletedType {
    V1PayoutReconciliationCompleted,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PayoutReconciliationCompletedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PayoutReconciliationCompletedType::V1PayoutReconciliationCompleted => {
                "v1.payout.reconciliation_completed"
            }
            V1PayoutReconciliationCompletedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PayoutReconciliationCompletedType {
    fn default() -> Self {
        V1PayoutReconciliationCompletedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PayoutReconciliationCompletedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PayoutReconciliationCompletedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.payout.reconciliation_completed" => {
                V1PayoutReconciliationCompletedType::V1PayoutReconciliationCompleted
            }
            other => V1PayoutReconciliationCompletedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PlanUpdatedType {
    V1PlanUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PlanUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PlanUpdatedType::V1PlanUpdated => "v1.plan.updated",
            V1PlanUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PlanUpdatedType {
    fn default() -> Self {
        V1PlanUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PlanUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PlanUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.plan.updated" => V1PlanUpdatedType::V1PlanUpdated,
            other => V1PlanUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PriceCreatedRelatedObjectType {
    Price,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PriceCreatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PriceCreatedRelatedObjectType::Price => "price",
            V1PriceCreatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PriceCreatedRelatedObjectType {
    fn default() -> Self {
        V1PriceCreatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PriceCreatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PriceCreatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "price" => V1PriceCreatedRelatedObjectType::Price,
            other => V1PriceCreatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PriceCreatedType {
    V1PriceCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PriceCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PriceCreatedType::V1PriceCreated => "v1.price.created",
            V1PriceCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PriceCreatedType {
    fn default() -> Self {
        V1PriceCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PriceCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PriceCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.price.created" => V1PriceCreatedType::V1PriceCreated,
            other => V1PriceCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1QuoteAcceptedType {
    V1QuoteAccepted,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1QuoteAcceptedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1QuoteAcceptedType::V1QuoteAccepted => "v1.quote.accepted",
            V1QuoteAcceptedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1QuoteAcceptedType {
    fn default() -> Self {
        V1QuoteAcceptedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1QuoteAcceptedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1QuoteAcceptedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.quote.accepted" => V1QuoteAcceptedType::V1QuoteAccepted,
            other => V1QuoteAcceptedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1RefundFailedType {
    V1RefundFailed,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1RefundFailedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1RefundFailedType::V1RefundFailed => "v1.refund.failed",
            V1RefundFailedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1RefundFailedType {
    fn default() -> Self {
        V1RefundFailedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1RefundFailedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1RefundFailedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.refund.failed" => V1RefundFailedType::V1RefundFailed,
            other => V1RefundFailedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1SigmaScheduledQueryRunCreatedRelatedObjectType {
    ScheduledQueryRun,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1SigmaScheduledQueryRunCreatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1SigmaScheduledQueryRunCreatedRelatedObjectType::ScheduledQueryRun => "scheduled_query_run",
            V1SigmaScheduledQueryRunCreatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1SigmaScheduledQueryRunCreatedRelatedObjectType {
    fn default() -> Self {
        V1SigmaScheduledQueryRunCreatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1SigmaScheduledQueryRunCreatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1SigmaScheduledQueryRunCreatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "scheduled_query_run" => V1SigmaScheduledQueryRunCreatedRelatedObjectType::ScheduledQueryRun,
            other => V1SigmaScheduledQueryRunCreatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1SourceCanceledType {
    V1SourceCanceled,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1SourceCanceledType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1SourceCanceledType::V1SourceCanceled => "v1.source.canceled",
            V1SourceCanceledType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1SourceCanceledType {
    fn default() -> Self {
        V1SourceCanceledType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1SourceCanceledType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1SourceCanceledType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.source.canceled" => V1SourceCanceledType::V1SourceCanceled,
            other => V1SourceCanceledType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1SourceRefundAttributesRequiredType {
    V1SourceRefundAttributesRequired,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1SourceRefundAttributesRequiredType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1SourceRefundAttributesRequiredType::V1SourceRefundAttributesRequired => {
                "v1.source.refund_attributes_required"
            }
            V1SourceRefundAttributesRequiredType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1SourceRefundAttributesRequiredType {
    fn default() -> Self {
        V1SourceRefundAttributesRequiredType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1SourceRefundAttributesRequiredType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1SourceRefundAttributesRequiredType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.source.refund_attributes_required" => {
                V1SourceRefundAttributesRequiredType::V1SourceRefundAttributesRequired
            }
            other => V1SourceRefundAttributesRequiredType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1SubscriptionScheduleAbortedType {
    V1SubscriptionScheduleAborted,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1SubscriptionScheduleAbortedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1SubscriptionScheduleAbortedType::V1SubscriptionScheduleAborted => "v1.subscription_schedule.aborted",
            V1SubscriptionScheduleAbortedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1SubscriptionScheduleAbortedType {
    fn default() -> Self {
        V1SubscriptionScheduleAbortedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1SubscriptionScheduleAbortedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1SubscriptionScheduleAbortedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.subscription_schedule.aborted" => V1SubscriptionScheduleAbortedType::V1SubscriptionScheduleAborted,
            other => V1SubscriptionScheduleAbortedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1TaxRateCreatedType {
    V1TaxRateCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1TaxRateCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1TaxRateCreatedType::V1TaxRateCreated => "v1.tax_rate.created",
            V1TaxRateCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1TaxRateCreatedType {
    fn default() -> Self {
        V1TaxRateCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1TaxRateCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1TaxRateCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.tax_rate.created" => V1TaxRateCreatedType::V1TaxRateCreated,
            other => V1TaxRateCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1TaxSettingsUpdatedRelatedObjectType {
    TaxSettings,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1TaxSettingsUpdatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1TaxSettingsUpdatedRelatedObjectType::TaxSettings => "tax.settings",
            V1TaxSettingsUpdatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1TaxSettingsUpdatedRelatedObjectType {
    fn default() -> Self {
        V1TaxSettingsUpdatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1TaxSettingsUpdatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1TaxSettingsUpdatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "tax.settings" => V1TaxSettingsUpdatedRelatedObjectType::TaxSettings,
            other => V1TaxSettingsUpdatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1TestHelpersTestClockReadyType {
    V1TestHelpersTestClockReady,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1TestHelpersTestClockReadyType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1TestHelpersTestClockReadyType::V1TestHelpersTestClockReady => "v1.test_helpers.test_clock.ready",
            V1TestHelpersTestClockReadyType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1TestHelpersTestClockReadyType {
    fn default() -> Self {
        V1TestHelpersTestClockReadyType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1TestHelpersTestClockReadyType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1TestHelpersTestClockReadyType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.test_helpers.test_clock.ready" => V1TestHelpersTestClockReadyType::V1TestHelpersTestClockReady,
            other => V1TestHelpersTestClockReadyType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1TopupReversedType {
    V1TopupReversed,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1TopupReversedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1TopupReversedType::V1TopupReversed => "v1.topup.reversed",
            V1TopupReversedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1TopupReversedType {
    fn default() -> Self {
        V1TopupReversedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1TopupReversedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1TopupReversedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.topup.reversed" => V1TopupReversedType::V1TopupReversed,
            other => V1TopupReversedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1TransferCreatedRelatedObjectType {
    Transfer,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1TransferCreatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1TransferCreatedRelatedObjectType::Transfer => "transfer",
            V1TransferCreatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1TransferCreatedRelatedObjectType {
    fn default() -> Self {
        V1TransferCreatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1TransferCreatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1TransferCreatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "transfer" => V1TransferCreatedRelatedObjectType::Transfer,
            other => V1TransferCreatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1TransferCreatedType {
    V1TransferCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1TransferCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1TransferCreatedType::V1TransferCreated => "v1.transfer.created",
            V1TransferCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1TransferCreatedType {
    fn default() -> Self {
        V1TransferCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1TransferCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1TransferCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.transfer.created" => V1TransferCreatedType::V1TransferCreated,
            other => V1TransferCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CommerceProductCatalogImportsSucceededType {
    V2CommerceProductCatalogImportsSucceeded,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V2CommerceProductCatalogImportsSucceededType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CommerceProductCatalogImportsSucceededType::V2CommerceProductCatalogImportsSucceeded => {
                "v2.commerce.product_catalog.imports.succeeded"
            }
            V2CommerceProductCatalogImportsSucceededType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V2CommerceProductCatalogImportsSucceededType {
    fn default() -> Self {
        V2CommerceProductCatalogImportsSucceededType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V2CommerceProductCatalogImportsSucceededType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CommerceProductCatalogImportsSucceededType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.commerce.product_catalog.imports.succeeded" => {
                V2CommerceProductCatalogImportsSucceededType::V2CommerceProductCatalogImportsSucceeded
            }
            other => V2CommerceProductCatalogImportsSucceededType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountClosedType {
    V2CoreAccountClosed,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V2CoreAccountClosedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountClosedType::V2CoreAccountClosed => "v2.core.account.closed",
            V2CoreAccountClosedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V2CoreAccountClosedType {
    fn default() -> Self {
        V2CoreAccountClosedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V2CoreAccountClosedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountClosedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.core.account.closed" => V2CoreAccountClosedType::V2CoreAccountClosed,
            other => V2CoreAccountClosedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedType {
    V2CoreAccountConfigurationMerchantCapabilityStatusUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedType::V2CoreAccountConfigurationMerchantCapabilityStatusUpdated => "v2.core.account[configuration.merchant].capability_status_updated",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedType {
    fn default() -> Self {
        V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.core.account[configuration.merchant].capability_status_updated" => V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedType::V2CoreAccountConfigurationMerchantCapabilityStatusUpdated,
            other => V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountCreatedType {
    V2CoreAccountCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V2CoreAccountCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountCreatedType::V2CoreAccountCreated => "v2.core.account.created",
            V2CoreAccountCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V2CoreAccountCreatedType {
    fn default() -> Self {
        V2CoreAccountCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V2CoreAccountCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.core.account.created" => V2CoreAccountCreatedType::V2CoreAccountCreated,
            other => V2CoreAccountCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountPersonCreatedRelatedObjectType {
    V2CoreAccountPerson,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V2CoreAccountPersonCreatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountPersonCreatedRelatedObjectType::V2CoreAccountPerson => "v2.core.account_person",
            V2CoreAccountPersonCreatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V2CoreAccountPersonCreatedRelatedObjectType {
    fn default() -> Self {
        V2CoreAccountPersonCreatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V2CoreAccountPersonCreatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountPersonCreatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.core.account_person" => V2CoreAccountPersonCreatedRelatedObjectType::V2CoreAccountPerson,
            other => V2CoreAccountPersonCreatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreEventDestinationPingRelatedObjectType {
    V2CoreEventDestination,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V2CoreEventDestinationPingRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreEventDestinationPingRelatedObjectType::V2CoreEventDestination => "v2.core.event_destination",
            V2CoreEventDestinationPingRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V2CoreEventDestinationPingRelatedObjectType {
    fn default() -> Self {
        V2CoreEventDestinationPingRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V2CoreEventDestinationPingRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreEventDestinationPingRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.core.event_destination" => V2CoreEventDestinationPingRelatedObjectType::V2CoreEventDestination,
            other => V2CoreEventDestinationPingRelatedObjectType::Unknown(other.to_string()),
        })
    }
}
