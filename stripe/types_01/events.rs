#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ApplicationFeeCreatedRelatedObjectType {
    ApplicationFee,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ApplicationFeeCreatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ApplicationFeeCreatedRelatedObjectType::ApplicationFee => "application_fee",
            V1ApplicationFeeCreatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ApplicationFeeCreatedRelatedObjectType {
    fn default() -> Self {
        V1ApplicationFeeCreatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ApplicationFeeCreatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ApplicationFeeCreatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "application_fee" => V1ApplicationFeeCreatedRelatedObjectType::ApplicationFee,
            other => V1ApplicationFeeCreatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ApplicationFeeRefundUpdatedRelatedObjectType {
    FeeRefund,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ApplicationFeeRefundUpdatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ApplicationFeeRefundUpdatedRelatedObjectType::FeeRefund => "fee_refund",
            V1ApplicationFeeRefundUpdatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ApplicationFeeRefundUpdatedRelatedObjectType {
    fn default() -> Self {
        V1ApplicationFeeRefundUpdatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ApplicationFeeRefundUpdatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ApplicationFeeRefundUpdatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "fee_refund" => V1ApplicationFeeRefundUpdatedRelatedObjectType::FeeRefund,
            other => V1ApplicationFeeRefundUpdatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ApplicationFeeRefundedType {
    V1ApplicationFeeRefunded,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ApplicationFeeRefundedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ApplicationFeeRefundedType::V1ApplicationFeeRefunded => "v1.application_fee.refunded",
            V1ApplicationFeeRefundedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ApplicationFeeRefundedType {
    fn default() -> Self {
        V1ApplicationFeeRefundedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ApplicationFeeRefundedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ApplicationFeeRefundedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.application_fee.refunded" => V1ApplicationFeeRefundedType::V1ApplicationFeeRefunded,
            other => V1ApplicationFeeRefundedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1BillingMeterCreatedType {
    V1BillingMeterCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1BillingMeterCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1BillingMeterCreatedType::V1BillingMeterCreated => "v1.billing.meter.created",
            V1BillingMeterCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1BillingMeterCreatedType {
    fn default() -> Self {
        V1BillingMeterCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1BillingMeterCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1BillingMeterCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.billing.meter.created" => V1BillingMeterCreatedType::V1BillingMeterCreated,
            other => V1BillingMeterCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItemCode {
    ArchivedMeter,
    MeterEventCustomerNotFound,
    MeterEventDimensionCountTooHigh,
    MeterEventInvalidValue,
    MeterEventNoCustomerDefined,
    MeterEventValueTooManyDigits,
    MissingDimensionPayloadKeys,
    NoMeter,
    TimestampInFuture,
    TimestampTooFarInPast,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItemCode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItemCode::ArchivedMeter => "archived_meter",
            V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItemCode::MeterEventCustomerNotFound => {
                "meter_event_customer_not_found"
            }
            V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItemCode::MeterEventDimensionCountTooHigh => {
                "meter_event_dimension_count_too_high"
            }
            V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItemCode::MeterEventInvalidValue => {
                "meter_event_invalid_value"
            }
            V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItemCode::MeterEventNoCustomerDefined => {
                "meter_event_no_customer_defined"
            }
            V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItemCode::MeterEventValueTooManyDigits => {
                "meter_event_value_too_many_digits"
            }
            V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItemCode::MissingDimensionPayloadKeys => {
                "missing_dimension_payload_keys"
            }
            V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItemCode::NoMeter => "no_meter",
            V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItemCode::TimestampInFuture => "timestamp_in_future",
            V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItemCode::TimestampTooFarInPast => {
                "timestamp_too_far_in_past"
            }
            V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItemCode::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItemCode {
    fn default() -> Self {
        V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItemCode::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItemCode {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItemCode::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "archived_meter" => V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItemCode::ArchivedMeter,
            "meter_event_customer_not_found" => {
                V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItemCode::MeterEventCustomerNotFound
            }
            "meter_event_dimension_count_too_high" => {
                V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItemCode::MeterEventDimensionCountTooHigh
            }
            "meter_event_invalid_value" => {
                V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItemCode::MeterEventInvalidValue
            }
            "meter_event_no_customer_defined" => {
                V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItemCode::MeterEventNoCustomerDefined
            }
            "meter_event_value_too_many_digits" => {
                V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItemCode::MeterEventValueTooManyDigits
            }
            "missing_dimension_payload_keys" => {
                V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItemCode::MissingDimensionPayloadKeys
            }
            "no_meter" => V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItemCode::NoMeter,
            "timestamp_in_future" => V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItemCode::TimestampInFuture,
            "timestamp_too_far_in_past" => {
                V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItemCode::TimestampTooFarInPast
            }
            other => V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItemCode::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ChargeDisputeClosedRelatedObjectType {
    Dispute,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ChargeDisputeClosedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ChargeDisputeClosedRelatedObjectType::Dispute => "dispute",
            V1ChargeDisputeClosedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ChargeDisputeClosedRelatedObjectType {
    fn default() -> Self {
        V1ChargeDisputeClosedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ChargeDisputeClosedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ChargeDisputeClosedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "dispute" => V1ChargeDisputeClosedRelatedObjectType::Dispute,
            other => V1ChargeDisputeClosedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ChargeRefundedType {
    V1ChargeRefunded,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ChargeRefundedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ChargeRefundedType::V1ChargeRefunded => "v1.charge.refunded",
            V1ChargeRefundedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ChargeRefundedType {
    fn default() -> Self {
        V1ChargeRefundedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ChargeRefundedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ChargeRefundedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.charge.refunded" => V1ChargeRefundedType::V1ChargeRefunded,
            other => V1ChargeRefundedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ClimateOrderCanceledType {
    V1ClimateOrderCanceled,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ClimateOrderCanceledType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ClimateOrderCanceledType::V1ClimateOrderCanceled => "v1.climate.order.canceled",
            V1ClimateOrderCanceledType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ClimateOrderCanceledType {
    fn default() -> Self {
        V1ClimateOrderCanceledType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ClimateOrderCanceledType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ClimateOrderCanceledType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.climate.order.canceled" => V1ClimateOrderCanceledType::V1ClimateOrderCanceled,
            other => V1ClimateOrderCanceledType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ClimateOrderDelayedType {
    V1ClimateOrderDelayed,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ClimateOrderDelayedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ClimateOrderDelayedType::V1ClimateOrderDelayed => "v1.climate.order.delayed",
            V1ClimateOrderDelayedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ClimateOrderDelayedType {
    fn default() -> Self {
        V1ClimateOrderDelayedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ClimateOrderDelayedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ClimateOrderDelayedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.climate.order.delayed" => V1ClimateOrderDelayedType::V1ClimateOrderDelayed,
            other => V1ClimateOrderDelayedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ClimateOrderDeliveredType {
    V1ClimateOrderDelivered,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ClimateOrderDeliveredType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ClimateOrderDeliveredType::V1ClimateOrderDelivered => "v1.climate.order.delivered",
            V1ClimateOrderDeliveredType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ClimateOrderDeliveredType {
    fn default() -> Self {
        V1ClimateOrderDeliveredType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ClimateOrderDeliveredType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ClimateOrderDeliveredType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.climate.order.delivered" => V1ClimateOrderDeliveredType::V1ClimateOrderDelivered,
            other => V1ClimateOrderDeliveredType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CouponCreatedType {
    V1CouponCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CouponCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CouponCreatedType::V1CouponCreated => "v1.coupon.created",
            V1CouponCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CouponCreatedType {
    fn default() -> Self {
        V1CouponCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CouponCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CouponCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.coupon.created" => V1CouponCreatedType::V1CouponCreated,
            other => V1CouponCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CreditNoteCreatedRelatedObjectType {
    CreditNote,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CreditNoteCreatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CreditNoteCreatedRelatedObjectType::CreditNote => "credit_note",
            V1CreditNoteCreatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CreditNoteCreatedRelatedObjectType {
    fn default() -> Self {
        V1CreditNoteCreatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CreditNoteCreatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CreditNoteCreatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "credit_note" => V1CreditNoteCreatedRelatedObjectType::CreditNote,
            other => V1CreditNoteCreatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CustomerCashBalanceTransactionCreatedType {
    V1CustomerCashBalanceTransactionCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CustomerCashBalanceTransactionCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CustomerCashBalanceTransactionCreatedType::V1CustomerCashBalanceTransactionCreated => {
                "v1.customer_cash_balance_transaction.created"
            }
            V1CustomerCashBalanceTransactionCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CustomerCashBalanceTransactionCreatedType {
    fn default() -> Self {
        V1CustomerCashBalanceTransactionCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CustomerCashBalanceTransactionCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CustomerCashBalanceTransactionCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.customer_cash_balance_transaction.created" => {
                V1CustomerCashBalanceTransactionCreatedType::V1CustomerCashBalanceTransactionCreated
            }
            other => V1CustomerCashBalanceTransactionCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CustomerDiscountDeletedType {
    V1CustomerDiscountDeleted,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CustomerDiscountDeletedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CustomerDiscountDeletedType::V1CustomerDiscountDeleted => "v1.customer.discount.deleted",
            V1CustomerDiscountDeletedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CustomerDiscountDeletedType {
    fn default() -> Self {
        V1CustomerDiscountDeletedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CustomerDiscountDeletedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CustomerDiscountDeletedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.customer.discount.deleted" => V1CustomerDiscountDeletedType::V1CustomerDiscountDeleted,
            other => V1CustomerDiscountDeletedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CustomerSubscriptionPendingUpdateExpiredType {
    V1CustomerSubscriptionPendingUpdateExpired,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CustomerSubscriptionPendingUpdateExpiredType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CustomerSubscriptionPendingUpdateExpiredType::V1CustomerSubscriptionPendingUpdateExpired => {
                "v1.customer.subscription.pending_update_expired"
            }
            V1CustomerSubscriptionPendingUpdateExpiredType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CustomerSubscriptionPendingUpdateExpiredType {
    fn default() -> Self {
        V1CustomerSubscriptionPendingUpdateExpiredType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CustomerSubscriptionPendingUpdateExpiredType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CustomerSubscriptionPendingUpdateExpiredType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.customer.subscription.pending_update_expired" => {
                V1CustomerSubscriptionPendingUpdateExpiredType::V1CustomerSubscriptionPendingUpdateExpired
            }
            other => V1CustomerSubscriptionPendingUpdateExpiredType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CustomerTaxIdCreatedRelatedObjectType {
    TaxId,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CustomerTaxIdCreatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CustomerTaxIdCreatedRelatedObjectType::TaxId => "tax_id",
            V1CustomerTaxIdCreatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CustomerTaxIdCreatedRelatedObjectType {
    fn default() -> Self {
        V1CustomerTaxIdCreatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CustomerTaxIdCreatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CustomerTaxIdCreatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "tax_id" => V1CustomerTaxIdCreatedRelatedObjectType::TaxId,
            other => V1CustomerTaxIdCreatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CustomerTaxIdCreatedType {
    V1CustomerTaxIdCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CustomerTaxIdCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CustomerTaxIdCreatedType::V1CustomerTaxIdCreated => "v1.customer.tax_id.created",
            V1CustomerTaxIdCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CustomerTaxIdCreatedType {
    fn default() -> Self {
        V1CustomerTaxIdCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CustomerTaxIdCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CustomerTaxIdCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.customer.tax_id.created" => V1CustomerTaxIdCreatedType::V1CustomerTaxIdCreated,
            other => V1CustomerTaxIdCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CustomerUpdatedType {
    V1CustomerUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CustomerUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CustomerUpdatedType::V1CustomerUpdated => "v1.customer.updated",
            V1CustomerUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CustomerUpdatedType {
    fn default() -> Self {
        V1CustomerUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CustomerUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CustomerUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.customer.updated" => V1CustomerUpdatedType::V1CustomerUpdated,
            other => V1CustomerUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1FinancialConnectionsAccountAccountNumbersUpdatedType {
    V1FinancialConnectionsAccountAccountNumbersUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1FinancialConnectionsAccountAccountNumbersUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1FinancialConnectionsAccountAccountNumbersUpdatedType::V1FinancialConnectionsAccountAccountNumbersUpdated => "v1.financial_connections.account.account_numbers_updated",
            V1FinancialConnectionsAccountAccountNumbersUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1FinancialConnectionsAccountAccountNumbersUpdatedType {
    fn default() -> Self {
        V1FinancialConnectionsAccountAccountNumbersUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1FinancialConnectionsAccountAccountNumbersUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1FinancialConnectionsAccountAccountNumbersUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.financial_connections.account.account_numbers_updated" => V1FinancialConnectionsAccountAccountNumbersUpdatedType::V1FinancialConnectionsAccountAccountNumbersUpdated,
            other => V1FinancialConnectionsAccountAccountNumbersUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1FinancialConnectionsAccountReactivatedType {
    V1FinancialConnectionsAccountReactivated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1FinancialConnectionsAccountReactivatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1FinancialConnectionsAccountReactivatedType::V1FinancialConnectionsAccountReactivated => {
                "v1.financial_connections.account.reactivated"
            }
            V1FinancialConnectionsAccountReactivatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1FinancialConnectionsAccountReactivatedType {
    fn default() -> Self {
        V1FinancialConnectionsAccountReactivatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1FinancialConnectionsAccountReactivatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1FinancialConnectionsAccountReactivatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.financial_connections.account.reactivated" => {
                V1FinancialConnectionsAccountReactivatedType::V1FinancialConnectionsAccountReactivated
            }
            other => V1FinancialConnectionsAccountReactivatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1FinancialConnectionsAccountRefreshedBalanceType {
    V1FinancialConnectionsAccountRefreshedBalance,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1FinancialConnectionsAccountRefreshedBalanceType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1FinancialConnectionsAccountRefreshedBalanceType::V1FinancialConnectionsAccountRefreshedBalance => {
                "v1.financial_connections.account.refreshed_balance"
            }
            V1FinancialConnectionsAccountRefreshedBalanceType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1FinancialConnectionsAccountRefreshedBalanceType {
    fn default() -> Self {
        V1FinancialConnectionsAccountRefreshedBalanceType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1FinancialConnectionsAccountRefreshedBalanceType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1FinancialConnectionsAccountRefreshedBalanceType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.financial_connections.account.refreshed_balance" => {
                V1FinancialConnectionsAccountRefreshedBalanceType::V1FinancialConnectionsAccountRefreshedBalance
            }
            other => V1FinancialConnectionsAccountRefreshedBalanceType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1FinancialConnectionsAccountRefreshedTransactionsType {
    V1FinancialConnectionsAccountRefreshedTransactions,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1FinancialConnectionsAccountRefreshedTransactionsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1FinancialConnectionsAccountRefreshedTransactionsType::V1FinancialConnectionsAccountRefreshedTransactions => "v1.financial_connections.account.refreshed_transactions",
            V1FinancialConnectionsAccountRefreshedTransactionsType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1FinancialConnectionsAccountRefreshedTransactionsType {
    fn default() -> Self {
        V1FinancialConnectionsAccountRefreshedTransactionsType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1FinancialConnectionsAccountRefreshedTransactionsType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1FinancialConnectionsAccountRefreshedTransactionsType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.financial_connections.account.refreshed_transactions" => V1FinancialConnectionsAccountRefreshedTransactionsType::V1FinancialConnectionsAccountRefreshedTransactions,
            other => V1FinancialConnectionsAccountRefreshedTransactionsType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1FinancialConnectionsAccountUpcomingAccountNumberExpiryType {
    V1FinancialConnectionsAccountUpcomingAccountNumberExpiry,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1FinancialConnectionsAccountUpcomingAccountNumberExpiryType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1FinancialConnectionsAccountUpcomingAccountNumberExpiryType::V1FinancialConnectionsAccountUpcomingAccountNumberExpiry => "v1.financial_connections.account.upcoming_account_number_expiry",
            V1FinancialConnectionsAccountUpcomingAccountNumberExpiryType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1FinancialConnectionsAccountUpcomingAccountNumberExpiryType {
    fn default() -> Self {
        V1FinancialConnectionsAccountUpcomingAccountNumberExpiryType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1FinancialConnectionsAccountUpcomingAccountNumberExpiryType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1FinancialConnectionsAccountUpcomingAccountNumberExpiryType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.financial_connections.account.upcoming_account_number_expiry" => V1FinancialConnectionsAccountUpcomingAccountNumberExpiryType::V1FinancialConnectionsAccountUpcomingAccountNumberExpiry,
            other => V1FinancialConnectionsAccountUpcomingAccountNumberExpiryType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IdentityVerificationSessionCanceledType {
    V1IdentityVerificationSessionCanceled,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IdentityVerificationSessionCanceledType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IdentityVerificationSessionCanceledType::V1IdentityVerificationSessionCanceled => {
                "v1.identity.verification_session.canceled"
            }
            V1IdentityVerificationSessionCanceledType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IdentityVerificationSessionCanceledType {
    fn default() -> Self {
        V1IdentityVerificationSessionCanceledType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IdentityVerificationSessionCanceledType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IdentityVerificationSessionCanceledType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.identity.verification_session.canceled" => {
                V1IdentityVerificationSessionCanceledType::V1IdentityVerificationSessionCanceled
            }
            other => V1IdentityVerificationSessionCanceledType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IdentityVerificationSessionVerifiedType {
    V1IdentityVerificationSessionVerified,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IdentityVerificationSessionVerifiedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IdentityVerificationSessionVerifiedType::V1IdentityVerificationSessionVerified => {
                "v1.identity.verification_session.verified"
            }
            V1IdentityVerificationSessionVerifiedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IdentityVerificationSessionVerifiedType {
    fn default() -> Self {
        V1IdentityVerificationSessionVerifiedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IdentityVerificationSessionVerifiedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IdentityVerificationSessionVerifiedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.identity.verification_session.verified" => {
                V1IdentityVerificationSessionVerifiedType::V1IdentityVerificationSessionVerified
            }
            other => V1IdentityVerificationSessionVerifiedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1InvoiceFinalizationFailedType {
    V1InvoiceFinalizationFailed,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1InvoiceFinalizationFailedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1InvoiceFinalizationFailedType::V1InvoiceFinalizationFailed => "v1.invoice.finalization_failed",
            V1InvoiceFinalizationFailedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1InvoiceFinalizationFailedType {
    fn default() -> Self {
        V1InvoiceFinalizationFailedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1InvoiceFinalizationFailedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1InvoiceFinalizationFailedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.invoice.finalization_failed" => V1InvoiceFinalizationFailedType::V1InvoiceFinalizationFailed,
            other => V1InvoiceFinalizationFailedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1InvoicePaymentAttemptRequiredType {
    V1InvoicePaymentAttemptRequired,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1InvoicePaymentAttemptRequiredType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1InvoicePaymentAttemptRequiredType::V1InvoicePaymentAttemptRequired => {
                "v1.invoice.payment_attempt_required"
            }
            V1InvoicePaymentAttemptRequiredType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1InvoicePaymentAttemptRequiredType {
    fn default() -> Self {
        V1InvoicePaymentAttemptRequiredType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1InvoicePaymentAttemptRequiredType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1InvoicePaymentAttemptRequiredType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.invoice.payment_attempt_required" => {
                V1InvoicePaymentAttemptRequiredType::V1InvoicePaymentAttemptRequired
            }
            other => V1InvoicePaymentAttemptRequiredType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1InvoicePaymentPaidRelatedObjectType {
    InvoicePayment,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1InvoicePaymentPaidRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1InvoicePaymentPaidRelatedObjectType::InvoicePayment => "invoice_payment",
            V1InvoicePaymentPaidRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1InvoicePaymentPaidRelatedObjectType {
    fn default() -> Self {
        V1InvoicePaymentPaidRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1InvoicePaymentPaidRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1InvoicePaymentPaidRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "invoice_payment" => V1InvoicePaymentPaidRelatedObjectType::InvoicePayment,
            other => V1InvoicePaymentPaidRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1InvoiceWillBeDueType {
    V1InvoiceWillBeDue,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1InvoiceWillBeDueType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1InvoiceWillBeDueType::V1InvoiceWillBeDue => "v1.invoice.will_be_due",
            V1InvoiceWillBeDueType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1InvoiceWillBeDueType {
    fn default() -> Self {
        V1InvoiceWillBeDueType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1InvoiceWillBeDueType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1InvoiceWillBeDueType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.invoice.will_be_due" => V1InvoiceWillBeDueType::V1InvoiceWillBeDue,
            other => V1InvoiceWillBeDueType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IssuingCardCreatedType {
    V1IssuingCardCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IssuingCardCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IssuingCardCreatedType::V1IssuingCardCreated => "v1.issuing_card.created",
            V1IssuingCardCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IssuingCardCreatedType {
    fn default() -> Self {
        V1IssuingCardCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IssuingCardCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IssuingCardCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.issuing_card.created" => V1IssuingCardCreatedType::V1IssuingCardCreated,
            other => V1IssuingCardCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IssuingCardholderCreatedType {
    V1IssuingCardholderCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IssuingCardholderCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IssuingCardholderCreatedType::V1IssuingCardholderCreated => "v1.issuing_cardholder.created",
            V1IssuingCardholderCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IssuingCardholderCreatedType {
    fn default() -> Self {
        V1IssuingCardholderCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IssuingCardholderCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IssuingCardholderCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.issuing_cardholder.created" => V1IssuingCardholderCreatedType::V1IssuingCardholderCreated,
            other => V1IssuingCardholderCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IssuingPersonalizationDesignDeactivatedType {
    V1IssuingPersonalizationDesignDeactivated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IssuingPersonalizationDesignDeactivatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IssuingPersonalizationDesignDeactivatedType::V1IssuingPersonalizationDesignDeactivated => {
                "v1.issuing_personalization_design.deactivated"
            }
            V1IssuingPersonalizationDesignDeactivatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IssuingPersonalizationDesignDeactivatedType {
    fn default() -> Self {
        V1IssuingPersonalizationDesignDeactivatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IssuingPersonalizationDesignDeactivatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IssuingPersonalizationDesignDeactivatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.issuing_personalization_design.deactivated" => {
                V1IssuingPersonalizationDesignDeactivatedType::V1IssuingPersonalizationDesignDeactivated
            }
            other => V1IssuingPersonalizationDesignDeactivatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IssuingTransactionCreatedRelatedObjectType {
    IssuingTransaction,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IssuingTransactionCreatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IssuingTransactionCreatedRelatedObjectType::IssuingTransaction => "issuing.transaction",
            V1IssuingTransactionCreatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IssuingTransactionCreatedRelatedObjectType {
    fn default() -> Self {
        V1IssuingTransactionCreatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IssuingTransactionCreatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IssuingTransactionCreatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "issuing.transaction" => V1IssuingTransactionCreatedRelatedObjectType::IssuingTransaction,
            other => V1IssuingTransactionCreatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IssuingTransactionUpdatedType {
    V1IssuingTransactionUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IssuingTransactionUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IssuingTransactionUpdatedType::V1IssuingTransactionUpdated => "v1.issuing_transaction.updated",
            V1IssuingTransactionUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IssuingTransactionUpdatedType {
    fn default() -> Self {
        V1IssuingTransactionUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IssuingTransactionUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IssuingTransactionUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.issuing_transaction.updated" => V1IssuingTransactionUpdatedType::V1IssuingTransactionUpdated,
            other => V1IssuingTransactionUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1MandateUpdatedType {
    V1MandateUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1MandateUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1MandateUpdatedType::V1MandateUpdated => "v1.mandate.updated",
            V1MandateUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1MandateUpdatedType {
    fn default() -> Self {
        V1MandateUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1MandateUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1MandateUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.mandate.updated" => V1MandateUpdatedType::V1MandateUpdated,
            other => V1MandateUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PaymentIntentAmountCapturableUpdatedRelatedObjectType {
    PaymentIntent,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PaymentIntentAmountCapturableUpdatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PaymentIntentAmountCapturableUpdatedRelatedObjectType::PaymentIntent => "payment_intent",
            V1PaymentIntentAmountCapturableUpdatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PaymentIntentAmountCapturableUpdatedRelatedObjectType {
    fn default() -> Self {
        V1PaymentIntentAmountCapturableUpdatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PaymentIntentAmountCapturableUpdatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PaymentIntentAmountCapturableUpdatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "payment_intent" => V1PaymentIntentAmountCapturableUpdatedRelatedObjectType::PaymentIntent,
            other => V1PaymentIntentAmountCapturableUpdatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PaymentIntentPartiallyFundedType {
    V1PaymentIntentPartiallyFunded,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PaymentIntentPartiallyFundedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PaymentIntentPartiallyFundedType::V1PaymentIntentPartiallyFunded => "v1.payment_intent.partially_funded",
            V1PaymentIntentPartiallyFundedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PaymentIntentPartiallyFundedType {
    fn default() -> Self {
        V1PaymentIntentPartiallyFundedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PaymentIntentPartiallyFundedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PaymentIntentPartiallyFundedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.payment_intent.partially_funded" => V1PaymentIntentPartiallyFundedType::V1PaymentIntentPartiallyFunded,
            other => V1PaymentIntentPartiallyFundedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PaymentIntentPaymentFailedType {
    V1PaymentIntentPaymentFailed,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PaymentIntentPaymentFailedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PaymentIntentPaymentFailedType::V1PaymentIntentPaymentFailed => "v1.payment_intent.payment_failed",
            V1PaymentIntentPaymentFailedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PaymentIntentPaymentFailedType {
    fn default() -> Self {
        V1PaymentIntentPaymentFailedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PaymentIntentPaymentFailedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PaymentIntentPaymentFailedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.payment_intent.payment_failed" => V1PaymentIntentPaymentFailedType::V1PaymentIntentPaymentFailed,
            other => V1PaymentIntentPaymentFailedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PaymentIntentSucceededType {
    V1PaymentIntentSucceeded,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PaymentIntentSucceededType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PaymentIntentSucceededType::V1PaymentIntentSucceeded => "v1.payment_intent.succeeded",
            V1PaymentIntentSucceededType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PaymentIntentSucceededType {
    fn default() -> Self {
        V1PaymentIntentSucceededType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PaymentIntentSucceededType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PaymentIntentSucceededType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.payment_intent.succeeded" => V1PaymentIntentSucceededType::V1PaymentIntentSucceeded,
            other => V1PaymentIntentSucceededType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PaymentMethodAttachedRelatedObjectType {
    PaymentMethod,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PaymentMethodAttachedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PaymentMethodAttachedRelatedObjectType::PaymentMethod => "payment_method",
            V1PaymentMethodAttachedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PaymentMethodAttachedRelatedObjectType {
    fn default() -> Self {
        V1PaymentMethodAttachedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PaymentMethodAttachedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PaymentMethodAttachedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "payment_method" => V1PaymentMethodAttachedRelatedObjectType::PaymentMethod,
            other => V1PaymentMethodAttachedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PaymentMethodAttachedType {
    V1PaymentMethodAttached,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PaymentMethodAttachedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PaymentMethodAttachedType::V1PaymentMethodAttached => "v1.payment_method.attached",
            V1PaymentMethodAttachedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PaymentMethodAttachedType {
    fn default() -> Self {
        V1PaymentMethodAttachedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PaymentMethodAttachedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PaymentMethodAttachedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.payment_method.attached" => V1PaymentMethodAttachedType::V1PaymentMethodAttached,
            other => V1PaymentMethodAttachedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PayoutCanceledRelatedObjectType {
    Payout,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PayoutCanceledRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PayoutCanceledRelatedObjectType::Payout => "payout",
            V1PayoutCanceledRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PayoutCanceledRelatedObjectType {
    fn default() -> Self {
        V1PayoutCanceledRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PayoutCanceledRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PayoutCanceledRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "payout" => V1PayoutCanceledRelatedObjectType::Payout,
            other => V1PayoutCanceledRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PayoutPaidType {
    V1PayoutPaid,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PayoutPaidType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PayoutPaidType::V1PayoutPaid => "v1.payout.paid",
            V1PayoutPaidType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PayoutPaidType {
    fn default() -> Self {
        V1PayoutPaidType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PayoutPaidType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PayoutPaidType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.payout.paid" => V1PayoutPaidType::V1PayoutPaid,
            other => V1PayoutPaidType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PayoutUpdatedType {
    V1PayoutUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PayoutUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PayoutUpdatedType::V1PayoutUpdated => "v1.payout.updated",
            V1PayoutUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PayoutUpdatedType {
    fn default() -> Self {
        V1PayoutUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PayoutUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PayoutUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.payout.updated" => V1PayoutUpdatedType::V1PayoutUpdated,
            other => V1PayoutUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PlanCreatedType {
    V1PlanCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PlanCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PlanCreatedType::V1PlanCreated => "v1.plan.created",
            V1PlanCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PlanCreatedType {
    fn default() -> Self {
        V1PlanCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PlanCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PlanCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.plan.created" => V1PlanCreatedType::V1PlanCreated,
            other => V1PlanCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PriceDeletedType {
    V1PriceDeleted,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PriceDeletedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PriceDeletedType::V1PriceDeleted => "v1.price.deleted",
            V1PriceDeletedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PriceDeletedType {
    fn default() -> Self {
        V1PriceDeletedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PriceDeletedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PriceDeletedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.price.deleted" => V1PriceDeletedType::V1PriceDeleted,
            other => V1PriceDeletedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PromotionCodeCreatedRelatedObjectType {
    PromotionCode,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PromotionCodeCreatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PromotionCodeCreatedRelatedObjectType::PromotionCode => "promotion_code",
            V1PromotionCodeCreatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PromotionCodeCreatedRelatedObjectType {
    fn default() -> Self {
        V1PromotionCodeCreatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PromotionCodeCreatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PromotionCodeCreatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "promotion_code" => V1PromotionCodeCreatedRelatedObjectType::PromotionCode,
            other => V1PromotionCodeCreatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PromotionCodeCreatedType {
    V1PromotionCodeCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PromotionCodeCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PromotionCodeCreatedType::V1PromotionCodeCreated => "v1.promotion_code.created",
            V1PromotionCodeCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PromotionCodeCreatedType {
    fn default() -> Self {
        V1PromotionCodeCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PromotionCodeCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PromotionCodeCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.promotion_code.created" => V1PromotionCodeCreatedType::V1PromotionCodeCreated,
            other => V1PromotionCodeCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1QuoteCanceledType {
    V1QuoteCanceled,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1QuoteCanceledType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1QuoteCanceledType::V1QuoteCanceled => "v1.quote.canceled",
            V1QuoteCanceledType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1QuoteCanceledType {
    fn default() -> Self {
        V1QuoteCanceledType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1QuoteCanceledType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1QuoteCanceledType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.quote.canceled" => V1QuoteCanceledType::V1QuoteCanceled,
            other => V1QuoteCanceledType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1QuoteFinalizedType {
    V1QuoteFinalized,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1QuoteFinalizedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1QuoteFinalizedType::V1QuoteFinalized => "v1.quote.finalized",
            V1QuoteFinalizedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1QuoteFinalizedType {
    fn default() -> Self {
        V1QuoteFinalizedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1QuoteFinalizedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1QuoteFinalizedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.quote.finalized" => V1QuoteFinalizedType::V1QuoteFinalized,
            other => V1QuoteFinalizedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ReviewClosedRelatedObjectType {
    Review,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ReviewClosedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ReviewClosedRelatedObjectType::Review => "review",
            V1ReviewClosedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ReviewClosedRelatedObjectType {
    fn default() -> Self {
        V1ReviewClosedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ReviewClosedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ReviewClosedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "review" => V1ReviewClosedRelatedObjectType::Review,
            other => V1ReviewClosedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ReviewClosedType {
    V1ReviewClosed,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ReviewClosedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ReviewClosedType::V1ReviewClosed => "v1.review.closed",
            V1ReviewClosedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ReviewClosedType {
    fn default() -> Self {
        V1ReviewClosedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ReviewClosedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ReviewClosedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.review.closed" => V1ReviewClosedType::V1ReviewClosed,
            other => V1ReviewClosedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1SourceChargeableType {
    V1SourceChargeable,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1SourceChargeableType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1SourceChargeableType::V1SourceChargeable => "v1.source.chargeable",
            V1SourceChargeableType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1SourceChargeableType {
    fn default() -> Self {
        V1SourceChargeableType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1SourceChargeableType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1SourceChargeableType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.source.chargeable" => V1SourceChargeableType::V1SourceChargeable,
            other => V1SourceChargeableType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1SubscriptionScheduleCanceledType {
    V1SubscriptionScheduleCanceled,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1SubscriptionScheduleCanceledType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1SubscriptionScheduleCanceledType::V1SubscriptionScheduleCanceled => "v1.subscription_schedule.canceled",
            V1SubscriptionScheduleCanceledType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1SubscriptionScheduleCanceledType {
    fn default() -> Self {
        V1SubscriptionScheduleCanceledType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1SubscriptionScheduleCanceledType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1SubscriptionScheduleCanceledType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.subscription_schedule.canceled" => V1SubscriptionScheduleCanceledType::V1SubscriptionScheduleCanceled,
            other => V1SubscriptionScheduleCanceledType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1SubscriptionScheduleCompletedType {
    V1SubscriptionScheduleCompleted,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1SubscriptionScheduleCompletedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1SubscriptionScheduleCompletedType::V1SubscriptionScheduleCompleted => {
                "v1.subscription_schedule.completed"
            }
            V1SubscriptionScheduleCompletedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1SubscriptionScheduleCompletedType {
    fn default() -> Self {
        V1SubscriptionScheduleCompletedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1SubscriptionScheduleCompletedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1SubscriptionScheduleCompletedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.subscription_schedule.completed" => {
                V1SubscriptionScheduleCompletedType::V1SubscriptionScheduleCompleted
            }
            other => V1SubscriptionScheduleCompletedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1SubscriptionScheduleCreatedType {
    V1SubscriptionScheduleCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1SubscriptionScheduleCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1SubscriptionScheduleCreatedType::V1SubscriptionScheduleCreated => "v1.subscription_schedule.created",
            V1SubscriptionScheduleCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1SubscriptionScheduleCreatedType {
    fn default() -> Self {
        V1SubscriptionScheduleCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1SubscriptionScheduleCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1SubscriptionScheduleCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.subscription_schedule.created" => V1SubscriptionScheduleCreatedType::V1SubscriptionScheduleCreated,
            other => V1SubscriptionScheduleCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1TaxSettingsUpdatedType {
    V1TaxSettingsUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1TaxSettingsUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1TaxSettingsUpdatedType::V1TaxSettingsUpdated => "v1.tax.settings.updated",
            V1TaxSettingsUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1TaxSettingsUpdatedType {
    fn default() -> Self {
        V1TaxSettingsUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1TaxSettingsUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1TaxSettingsUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.tax.settings.updated" => V1TaxSettingsUpdatedType::V1TaxSettingsUpdated,
            other => V1TaxSettingsUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1TerminalReaderActionUpdatedType {
    V1TerminalReaderActionUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1TerminalReaderActionUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1TerminalReaderActionUpdatedType::V1TerminalReaderActionUpdated => "v1.terminal.reader.action_updated",
            V1TerminalReaderActionUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1TerminalReaderActionUpdatedType {
    fn default() -> Self {
        V1TerminalReaderActionUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1TerminalReaderActionUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1TerminalReaderActionUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.terminal.reader.action_updated" => V1TerminalReaderActionUpdatedType::V1TerminalReaderActionUpdated,
            other => V1TerminalReaderActionUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountConfigurationCustomerCapabilityStatusUpdatedDataUpdatedCapability {
    AutomaticIndirectTax,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V2CoreAccountConfigurationCustomerCapabilityStatusUpdatedDataUpdatedCapability {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountConfigurationCustomerCapabilityStatusUpdatedDataUpdatedCapability::AutomaticIndirectTax => {
                "automatic_indirect_tax"
            }
            V2CoreAccountConfigurationCustomerCapabilityStatusUpdatedDataUpdatedCapability::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V2CoreAccountConfigurationCustomerCapabilityStatusUpdatedDataUpdatedCapability {
    fn default() -> Self {
        V2CoreAccountConfigurationCustomerCapabilityStatusUpdatedDataUpdatedCapability::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V2CoreAccountConfigurationCustomerCapabilityStatusUpdatedDataUpdatedCapability {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                V2CoreAccountConfigurationCustomerCapabilityStatusUpdatedDataUpdatedCapability::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "automatic_indirect_tax" => {
                V2CoreAccountConfigurationCustomerCapabilityStatusUpdatedDataUpdatedCapability::AutomaticIndirectTax
            }
            other => V2CoreAccountConfigurationCustomerCapabilityStatusUpdatedDataUpdatedCapability::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability {
    AchDebitPayments,
    AcssDebitPayments,
    AffirmPayments,
    AfterpayClearpayPayments,
    AlmaPayments,
    AmazonPayPayments,
    AuBecsDebitPayments,
    BacsDebitPayments,
    BancontactPayments,
    BlikPayments,
    BlikRecurringPayments,
    BoletoPayments,
    CardPayments,
    CartesBancairesPayments,
    CashappPayments,
    EpsPayments,
    FpxPayments,
    GbBankTransferPayments,
    GrabpayPayments,
    IdealPayments,
    JcbPayments,
    JpBankTransferPayments,
    KakaoPayPayments,
    KlarnaPayments,
    KonbiniPayments,
    KrCardPayments,
    LinkPayments,
    MobilepayPayments,
    MultibancoPayments,
    MxBankTransferPayments,
    NaverPayPayments,
    OxxoPayments,
    P24Payments,
    PaycoPayments,
    PaynowPayments,
    StripeBalancePayouts,
    PayByBankPayments,
    PromptpayPayments,
    RevolutPayPayments,
    SamsungPayPayments,
    SatispayPayments,
    SepaBankTransferPayments,
    SepaDebitPayments,
    SequraPayments,
    SunbitPayments,
    SwishPayments,
    TwintPayments,
    UsBankTransferPayments,
    ZipPayments,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::AchDebitPayments => "ach_debit_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::AcssDebitPayments => "acss_debit_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::AffirmPayments => "affirm_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::AfterpayClearpayPayments => "afterpay_clearpay_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::AlmaPayments => "alma_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::AmazonPayPayments => "amazon_pay_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::AuBecsDebitPayments => "au_becs_debit_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::BacsDebitPayments => "bacs_debit_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::BancontactPayments => "bancontact_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::BlikPayments => "blik_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::BlikRecurringPayments => "blik_recurring_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::BoletoPayments => "boleto_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::CardPayments => "card_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::CartesBancairesPayments => "cartes_bancaires_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::CashappPayments => "cashapp_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::EpsPayments => "eps_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::FpxPayments => "fpx_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::GbBankTransferPayments => "gb_bank_transfer_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::GrabpayPayments => "grabpay_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::IdealPayments => "ideal_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::JcbPayments => "jcb_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::JpBankTransferPayments => "jp_bank_transfer_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::KakaoPayPayments => "kakao_pay_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::KlarnaPayments => "klarna_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::KonbiniPayments => "konbini_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::KrCardPayments => "kr_card_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::LinkPayments => "link_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::MobilepayPayments => "mobilepay_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::MultibancoPayments => "multibanco_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::MxBankTransferPayments => "mx_bank_transfer_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::NaverPayPayments => "naver_pay_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::OxxoPayments => "oxxo_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::P24Payments => "p24_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::PaycoPayments => "payco_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::PaynowPayments => "paynow_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::StripeBalancePayouts => "stripe_balance.payouts",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::PayByBankPayments => "pay_by_bank_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::PromptpayPayments => "promptpay_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::RevolutPayPayments => "revolut_pay_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::SamsungPayPayments => "samsung_pay_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::SatispayPayments => "satispay_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::SepaBankTransferPayments => "sepa_bank_transfer_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::SepaDebitPayments => "sepa_debit_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::SequraPayments => "sequra_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::SunbitPayments => "sunbit_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::SwishPayments => "swish_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::TwintPayments => "twint_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::UsBankTransferPayments => "us_bank_transfer_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::ZipPayments => "zip_payments",
            V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability {
    fn default() -> Self {
        V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ach_debit_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::AchDebitPayments
            }
            "acss_debit_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::AcssDebitPayments
            }
            "affirm_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::AffirmPayments
            }
            "afterpay_clearpay_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::AfterpayClearpayPayments
            }
            "alma_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::AlmaPayments
            }
            "amazon_pay_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::AmazonPayPayments
            }
            "au_becs_debit_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::AuBecsDebitPayments
            }
            "bacs_debit_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::BacsDebitPayments
            }
            "bancontact_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::BancontactPayments
            }
            "blik_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::BlikPayments
            }
            "blik_recurring_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::BlikRecurringPayments
            }
            "boleto_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::BoletoPayments
            }
            "card_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::CardPayments
            }
            "cartes_bancaires_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::CartesBancairesPayments
            }
            "cashapp_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::CashappPayments
            }
            "eps_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::EpsPayments
            }
            "fpx_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::FpxPayments
            }
            "gb_bank_transfer_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::GbBankTransferPayments
            }
            "grabpay_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::GrabpayPayments
            }
            "ideal_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::IdealPayments
            }
            "jcb_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::JcbPayments
            }
            "jp_bank_transfer_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::JpBankTransferPayments
            }
            "kakao_pay_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::KakaoPayPayments
            }
            "klarna_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::KlarnaPayments
            }
            "konbini_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::KonbiniPayments
            }
            "kr_card_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::KrCardPayments
            }
            "link_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::LinkPayments
            }
            "mobilepay_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::MobilepayPayments
            }
            "multibanco_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::MultibancoPayments
            }
            "mx_bank_transfer_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::MxBankTransferPayments
            }
            "naver_pay_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::NaverPayPayments
            }
            "oxxo_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::OxxoPayments
            }
            "p24_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::P24Payments
            }
            "payco_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::PaycoPayments
            }
            "paynow_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::PaynowPayments
            }
            "stripe_balance.payouts" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::StripeBalancePayouts
            }
            "pay_by_bank_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::PayByBankPayments
            }
            "promptpay_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::PromptpayPayments
            }
            "revolut_pay_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::RevolutPayPayments
            }
            "samsung_pay_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::SamsungPayPayments
            }
            "satispay_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::SatispayPayments
            }
            "sepa_bank_transfer_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::SepaBankTransferPayments
            }
            "sepa_debit_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::SepaDebitPayments
            }
            "sequra_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::SequraPayments
            }
            "sunbit_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::SunbitPayments
            }
            "swish_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::SwishPayments
            }
            "twint_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::TwintPayments
            }
            "us_bank_transfer_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::UsBankTransferPayments
            }
            "zip_payments" => {
                V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::ZipPayments
            }
            other => V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountConfigurationMerchantUpdatedType {
    V2CoreAccountConfigurationMerchantUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V2CoreAccountConfigurationMerchantUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountConfigurationMerchantUpdatedType::V2CoreAccountConfigurationMerchantUpdated => {
                "v2.core.account[configuration.merchant].updated"
            }
            V2CoreAccountConfigurationMerchantUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V2CoreAccountConfigurationMerchantUpdatedType {
    fn default() -> Self {
        V2CoreAccountConfigurationMerchantUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V2CoreAccountConfigurationMerchantUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountConfigurationMerchantUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.core.account[configuration.merchant].updated" => {
                V2CoreAccountConfigurationMerchantUpdatedType::V2CoreAccountConfigurationMerchantUpdated
            }
            other => V2CoreAccountConfigurationMerchantUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountLinkReturnedDataUseCase {
    AccountOnboarding,
    AccountUpdate,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V2CoreAccountLinkReturnedDataUseCase {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountLinkReturnedDataUseCase::AccountOnboarding => "account_onboarding",
            V2CoreAccountLinkReturnedDataUseCase::AccountUpdate => "account_update",
            V2CoreAccountLinkReturnedDataUseCase::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V2CoreAccountLinkReturnedDataUseCase {
    fn default() -> Self {
        V2CoreAccountLinkReturnedDataUseCase::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V2CoreAccountLinkReturnedDataUseCase {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountLinkReturnedDataUseCase::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account_onboarding" => V2CoreAccountLinkReturnedDataUseCase::AccountOnboarding,
            "account_update" => V2CoreAccountLinkReturnedDataUseCase::AccountUpdate,
            other => V2CoreAccountLinkReturnedDataUseCase::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountPersonCreatedData {
    #[doc = "The ID of the v2 account."]
    #[json(default)]
    pub account_id: String,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountPersonCreatedType {
    V2CoreAccountPersonCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V2CoreAccountPersonCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountPersonCreatedType::V2CoreAccountPersonCreated => "v2.core.account_person.created",
            V2CoreAccountPersonCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V2CoreAccountPersonCreatedType {
    fn default() -> Self {
        V2CoreAccountPersonCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V2CoreAccountPersonCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountPersonCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.core.account_person.created" => V2CoreAccountPersonCreatedType::V2CoreAccountPersonCreated,
            other => V2CoreAccountPersonCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountPersonUpdatedType {
    V2CoreAccountPersonUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V2CoreAccountPersonUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountPersonUpdatedType::V2CoreAccountPersonUpdated => "v2.core.account_person.updated",
            V2CoreAccountPersonUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V2CoreAccountPersonUpdatedType {
    fn default() -> Self {
        V2CoreAccountPersonUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V2CoreAccountPersonUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountPersonUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.core.account_person.updated" => V2CoreAccountPersonUpdatedType::V2CoreAccountPersonUpdated,
            other => V2CoreAccountPersonUpdatedType::Unknown(other.to_string()),
        })
    }
}
