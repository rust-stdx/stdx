#[cfg(feature = "events")]
#[derive(Debug, Clone, Default)]
pub struct GetEventParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "events")]
impl crate::params::FormEncode for GetEventParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ApplicationFeeRefundUpdatedType {
    V1ApplicationFeeRefundUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ApplicationFeeRefundUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ApplicationFeeRefundUpdatedType::V1ApplicationFeeRefundUpdated => "v1.application_fee.refund.updated",
            V1ApplicationFeeRefundUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ApplicationFeeRefundUpdatedType {
    fn default() -> Self {
        V1ApplicationFeeRefundUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ApplicationFeeRefundUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ApplicationFeeRefundUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.application_fee.refund.updated" => V1ApplicationFeeRefundUpdatedType::V1ApplicationFeeRefundUpdated,
            other => V1ApplicationFeeRefundUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1BillingCreditGrantUpdatedType {
    V1BillingCreditGrantUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1BillingCreditGrantUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1BillingCreditGrantUpdatedType::V1BillingCreditGrantUpdated => "v1.billing.credit_grant.updated",
            V1BillingCreditGrantUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1BillingCreditGrantUpdatedType {
    fn default() -> Self {
        V1BillingCreditGrantUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1BillingCreditGrantUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1BillingCreditGrantUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.billing.credit_grant.updated" => V1BillingCreditGrantUpdatedType::V1BillingCreditGrantUpdated,
            other => V1BillingCreditGrantUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1BillingMeterDeactivatedType {
    V1BillingMeterDeactivated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1BillingMeterDeactivatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1BillingMeterDeactivatedType::V1BillingMeterDeactivated => "v1.billing.meter.deactivated",
            V1BillingMeterDeactivatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1BillingMeterDeactivatedType {
    fn default() -> Self {
        V1BillingMeterDeactivatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1BillingMeterDeactivatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1BillingMeterDeactivatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.billing.meter.deactivated" => V1BillingMeterDeactivatedType::V1BillingMeterDeactivated,
            other => V1BillingMeterDeactivatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1BillingMeterReactivatedType {
    V1BillingMeterReactivated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1BillingMeterReactivatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1BillingMeterReactivatedType::V1BillingMeterReactivated => "v1.billing.meter.reactivated",
            V1BillingMeterReactivatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1BillingMeterReactivatedType {
    fn default() -> Self {
        V1BillingMeterReactivatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1BillingMeterReactivatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1BillingMeterReactivatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.billing.meter.reactivated" => V1BillingMeterReactivatedType::V1BillingMeterReactivated,
            other => V1BillingMeterReactivatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CashBalanceFundsAvailableRelatedObjectType {
    CashBalance,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CashBalanceFundsAvailableRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CashBalanceFundsAvailableRelatedObjectType::CashBalance => "cash_balance",
            V1CashBalanceFundsAvailableRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CashBalanceFundsAvailableRelatedObjectType {
    fn default() -> Self {
        V1CashBalanceFundsAvailableRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CashBalanceFundsAvailableRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CashBalanceFundsAvailableRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "cash_balance" => V1CashBalanceFundsAvailableRelatedObjectType::CashBalance,
            other => V1CashBalanceFundsAvailableRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ChargeCapturedType {
    V1ChargeCaptured,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ChargeCapturedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ChargeCapturedType::V1ChargeCaptured => "v1.charge.captured",
            V1ChargeCapturedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ChargeCapturedType {
    fn default() -> Self {
        V1ChargeCapturedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ChargeCapturedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ChargeCapturedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.charge.captured" => V1ChargeCapturedType::V1ChargeCaptured,
            other => V1ChargeCapturedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ChargeDisputeClosedType {
    V1ChargeDisputeClosed,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ChargeDisputeClosedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ChargeDisputeClosedType::V1ChargeDisputeClosed => "v1.charge.dispute.closed",
            V1ChargeDisputeClosedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ChargeDisputeClosedType {
    fn default() -> Self {
        V1ChargeDisputeClosedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ChargeDisputeClosedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ChargeDisputeClosedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.charge.dispute.closed" => V1ChargeDisputeClosedType::V1ChargeDisputeClosed,
            other => V1ChargeDisputeClosedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ChargeExpiredType {
    V1ChargeExpired,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ChargeExpiredType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ChargeExpiredType::V1ChargeExpired => "v1.charge.expired",
            V1ChargeExpiredType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ChargeExpiredType {
    fn default() -> Self {
        V1ChargeExpiredType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ChargeExpiredType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ChargeExpiredType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.charge.expired" => V1ChargeExpiredType::V1ChargeExpired,
            other => V1ChargeExpiredType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ChargeSucceededType {
    V1ChargeSucceeded,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ChargeSucceededType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ChargeSucceededType::V1ChargeSucceeded => "v1.charge.succeeded",
            V1ChargeSucceededType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ChargeSucceededType {
    fn default() -> Self {
        V1ChargeSucceededType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ChargeSucceededType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ChargeSucceededType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.charge.succeeded" => V1ChargeSucceededType::V1ChargeSucceeded,
            other => V1ChargeSucceededType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CheckoutSessionAsyncPaymentFailedRelatedObjectType {
    CheckoutSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CheckoutSessionAsyncPaymentFailedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CheckoutSessionAsyncPaymentFailedRelatedObjectType::CheckoutSession => "checkout.session",
            V1CheckoutSessionAsyncPaymentFailedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CheckoutSessionAsyncPaymentFailedRelatedObjectType {
    fn default() -> Self {
        V1CheckoutSessionAsyncPaymentFailedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CheckoutSessionAsyncPaymentFailedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CheckoutSessionAsyncPaymentFailedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "checkout.session" => V1CheckoutSessionAsyncPaymentFailedRelatedObjectType::CheckoutSession,
            other => V1CheckoutSessionAsyncPaymentFailedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CheckoutSessionAsyncPaymentFailedType {
    V1CheckoutSessionAsyncPaymentFailed,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CheckoutSessionAsyncPaymentFailedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CheckoutSessionAsyncPaymentFailedType::V1CheckoutSessionAsyncPaymentFailed => {
                "v1.checkout.session.async_payment_failed"
            }
            V1CheckoutSessionAsyncPaymentFailedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CheckoutSessionAsyncPaymentFailedType {
    fn default() -> Self {
        V1CheckoutSessionAsyncPaymentFailedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CheckoutSessionAsyncPaymentFailedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CheckoutSessionAsyncPaymentFailedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.checkout.session.async_payment_failed" => {
                V1CheckoutSessionAsyncPaymentFailedType::V1CheckoutSessionAsyncPaymentFailed
            }
            other => V1CheckoutSessionAsyncPaymentFailedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ClimateProductCreatedRelatedObjectType {
    ClimateProduct,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ClimateProductCreatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ClimateProductCreatedRelatedObjectType::ClimateProduct => "climate.product",
            V1ClimateProductCreatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ClimateProductCreatedRelatedObjectType {
    fn default() -> Self {
        V1ClimateProductCreatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ClimateProductCreatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ClimateProductCreatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "climate.product" => V1ClimateProductCreatedRelatedObjectType::ClimateProduct,
            other => V1ClimateProductCreatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CouponUpdatedType {
    V1CouponUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CouponUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CouponUpdatedType::V1CouponUpdated => "v1.coupon.updated",
            V1CouponUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CouponUpdatedType {
    fn default() -> Self {
        V1CouponUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CouponUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CouponUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.coupon.updated" => V1CouponUpdatedType::V1CouponUpdated,
            other => V1CouponUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CreditNoteUpdatedType {
    V1CreditNoteUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CreditNoteUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CreditNoteUpdatedType::V1CreditNoteUpdated => "v1.credit_note.updated",
            V1CreditNoteUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CreditNoteUpdatedType {
    fn default() -> Self {
        V1CreditNoteUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CreditNoteUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CreditNoteUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.credit_note.updated" => V1CreditNoteUpdatedType::V1CreditNoteUpdated,
            other => V1CreditNoteUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CustomerDiscountCreatedRelatedObjectType {
    Discount,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CustomerDiscountCreatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CustomerDiscountCreatedRelatedObjectType::Discount => "discount",
            V1CustomerDiscountCreatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CustomerDiscountCreatedRelatedObjectType {
    fn default() -> Self {
        V1CustomerDiscountCreatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CustomerDiscountCreatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CustomerDiscountCreatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "discount" => V1CustomerDiscountCreatedRelatedObjectType::Discount,
            other => V1CustomerDiscountCreatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CustomerSubscriptionCreatedType {
    V1CustomerSubscriptionCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CustomerSubscriptionCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CustomerSubscriptionCreatedType::V1CustomerSubscriptionCreated => "v1.customer.subscription.created",
            V1CustomerSubscriptionCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CustomerSubscriptionCreatedType {
    fn default() -> Self {
        V1CustomerSubscriptionCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CustomerSubscriptionCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CustomerSubscriptionCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.customer.subscription.created" => V1CustomerSubscriptionCreatedType::V1CustomerSubscriptionCreated,
            other => V1CustomerSubscriptionCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CustomerTaxIdDeletedType {
    V1CustomerTaxIdDeleted,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CustomerTaxIdDeletedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CustomerTaxIdDeletedType::V1CustomerTaxIdDeleted => "v1.customer.tax_id.deleted",
            V1CustomerTaxIdDeletedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CustomerTaxIdDeletedType {
    fn default() -> Self {
        V1CustomerTaxIdDeletedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CustomerTaxIdDeletedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CustomerTaxIdDeletedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.customer.tax_id.deleted" => V1CustomerTaxIdDeletedType::V1CustomerTaxIdDeleted,
            other => V1CustomerTaxIdDeletedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1EntitlementsActiveEntitlementSummaryUpdatedType {
    V1EntitlementsActiveEntitlementSummaryUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1EntitlementsActiveEntitlementSummaryUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1EntitlementsActiveEntitlementSummaryUpdatedType::V1EntitlementsActiveEntitlementSummaryUpdated => {
                "v1.entitlements.active_entitlement_summary.updated"
            }
            V1EntitlementsActiveEntitlementSummaryUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1EntitlementsActiveEntitlementSummaryUpdatedType {
    fn default() -> Self {
        V1EntitlementsActiveEntitlementSummaryUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1EntitlementsActiveEntitlementSummaryUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1EntitlementsActiveEntitlementSummaryUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.entitlements.active_entitlement_summary.updated" => {
                V1EntitlementsActiveEntitlementSummaryUpdatedType::V1EntitlementsActiveEntitlementSummaryUpdated
            }
            other => V1EntitlementsActiveEntitlementSummaryUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1FinancialConnectionsAccountAccountNumbersUpdatedRelatedObjectType {
    FinancialConnectionsAccount,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1FinancialConnectionsAccountAccountNumbersUpdatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1FinancialConnectionsAccountAccountNumbersUpdatedRelatedObjectType::FinancialConnectionsAccount => {
                "financial_connections.account"
            }
            V1FinancialConnectionsAccountAccountNumbersUpdatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1FinancialConnectionsAccountAccountNumbersUpdatedRelatedObjectType {
    fn default() -> Self {
        V1FinancialConnectionsAccountAccountNumbersUpdatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1FinancialConnectionsAccountAccountNumbersUpdatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1FinancialConnectionsAccountAccountNumbersUpdatedRelatedObjectType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "financial_connections.account" => {
                V1FinancialConnectionsAccountAccountNumbersUpdatedRelatedObjectType::FinancialConnectionsAccount
            }
            other => V1FinancialConnectionsAccountAccountNumbersUpdatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1FinancialConnectionsAccountRefreshedOwnershipType {
    V1FinancialConnectionsAccountRefreshedOwnership,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1FinancialConnectionsAccountRefreshedOwnershipType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1FinancialConnectionsAccountRefreshedOwnershipType::V1FinancialConnectionsAccountRefreshedOwnership => {
                "v1.financial_connections.account.refreshed_ownership"
            }
            V1FinancialConnectionsAccountRefreshedOwnershipType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1FinancialConnectionsAccountRefreshedOwnershipType {
    fn default() -> Self {
        V1FinancialConnectionsAccountRefreshedOwnershipType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1FinancialConnectionsAccountRefreshedOwnershipType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1FinancialConnectionsAccountRefreshedOwnershipType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.financial_connections.account.refreshed_ownership" => {
                V1FinancialConnectionsAccountRefreshedOwnershipType::V1FinancialConnectionsAccountRefreshedOwnership
            }
            other => V1FinancialConnectionsAccountRefreshedOwnershipType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IdentityVerificationSessionRedactedType {
    V1IdentityVerificationSessionRedacted,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IdentityVerificationSessionRedactedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IdentityVerificationSessionRedactedType::V1IdentityVerificationSessionRedacted => {
                "v1.identity.verification_session.redacted"
            }
            V1IdentityVerificationSessionRedactedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IdentityVerificationSessionRedactedType {
    fn default() -> Self {
        V1IdentityVerificationSessionRedactedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IdentityVerificationSessionRedactedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IdentityVerificationSessionRedactedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.identity.verification_session.redacted" => {
                V1IdentityVerificationSessionRedactedType::V1IdentityVerificationSessionRedacted
            }
            other => V1IdentityVerificationSessionRedactedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IdentityVerificationSessionRequiresInputType {
    V1IdentityVerificationSessionRequiresInput,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IdentityVerificationSessionRequiresInputType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IdentityVerificationSessionRequiresInputType::V1IdentityVerificationSessionRequiresInput => {
                "v1.identity.verification_session.requires_input"
            }
            V1IdentityVerificationSessionRequiresInputType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IdentityVerificationSessionRequiresInputType {
    fn default() -> Self {
        V1IdentityVerificationSessionRequiresInputType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IdentityVerificationSessionRequiresInputType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IdentityVerificationSessionRequiresInputType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.identity.verification_session.requires_input" => {
                V1IdentityVerificationSessionRequiresInputType::V1IdentityVerificationSessionRequiresInput
            }
            other => V1IdentityVerificationSessionRequiresInputType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1InvoiceFinalizedType {
    V1InvoiceFinalized,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1InvoiceFinalizedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1InvoiceFinalizedType::V1InvoiceFinalized => "v1.invoice.finalized",
            V1InvoiceFinalizedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1InvoiceFinalizedType {
    fn default() -> Self {
        V1InvoiceFinalizedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1InvoiceFinalizedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1InvoiceFinalizedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.invoice.finalized" => V1InvoiceFinalizedType::V1InvoiceFinalized,
            other => V1InvoiceFinalizedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1InvoiceOverpaidType {
    V1InvoiceOverpaid,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1InvoiceOverpaidType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1InvoiceOverpaidType::V1InvoiceOverpaid => "v1.invoice.overpaid",
            V1InvoiceOverpaidType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1InvoiceOverpaidType {
    fn default() -> Self {
        V1InvoiceOverpaidType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1InvoiceOverpaidType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1InvoiceOverpaidType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.invoice.overpaid" => V1InvoiceOverpaidType::V1InvoiceOverpaid,
            other => V1InvoiceOverpaidType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1InvoicePaymentFailedType {
    V1InvoicePaymentFailed,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1InvoicePaymentFailedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1InvoicePaymentFailedType::V1InvoicePaymentFailed => "v1.invoice.payment_failed",
            V1InvoicePaymentFailedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1InvoicePaymentFailedType {
    fn default() -> Self {
        V1InvoicePaymentFailedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1InvoicePaymentFailedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1InvoicePaymentFailedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.invoice.payment_failed" => V1InvoicePaymentFailedType::V1InvoicePaymentFailed,
            other => V1InvoicePaymentFailedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1InvoicePaymentPaidType {
    V1InvoicePaymentPaid,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1InvoicePaymentPaidType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1InvoicePaymentPaidType::V1InvoicePaymentPaid => "v1.invoice_payment.paid",
            V1InvoicePaymentPaidType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1InvoicePaymentPaidType {
    fn default() -> Self {
        V1InvoicePaymentPaidType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1InvoicePaymentPaidType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1InvoicePaymentPaidType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.invoice_payment.paid" => V1InvoicePaymentPaidType::V1InvoicePaymentPaid,
            other => V1InvoicePaymentPaidType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1InvoiceUpdatedType {
    V1InvoiceUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1InvoiceUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1InvoiceUpdatedType::V1InvoiceUpdated => "v1.invoice.updated",
            V1InvoiceUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1InvoiceUpdatedType {
    fn default() -> Self {
        V1InvoiceUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1InvoiceUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1InvoiceUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.invoice.updated" => V1InvoiceUpdatedType::V1InvoiceUpdated,
            other => V1InvoiceUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IssuingAuthorizationCreatedRelatedObjectType {
    IssuingAuthorization,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IssuingAuthorizationCreatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IssuingAuthorizationCreatedRelatedObjectType::IssuingAuthorization => "issuing.authorization",
            V1IssuingAuthorizationCreatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IssuingAuthorizationCreatedRelatedObjectType {
    fn default() -> Self {
        V1IssuingAuthorizationCreatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IssuingAuthorizationCreatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IssuingAuthorizationCreatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "issuing.authorization" => V1IssuingAuthorizationCreatedRelatedObjectType::IssuingAuthorization,
            other => V1IssuingAuthorizationCreatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IssuingAuthorizationCreatedType {
    V1IssuingAuthorizationCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IssuingAuthorizationCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IssuingAuthorizationCreatedType::V1IssuingAuthorizationCreated => "v1.issuing_authorization.created",
            V1IssuingAuthorizationCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IssuingAuthorizationCreatedType {
    fn default() -> Self {
        V1IssuingAuthorizationCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IssuingAuthorizationCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IssuingAuthorizationCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.issuing_authorization.created" => V1IssuingAuthorizationCreatedType::V1IssuingAuthorizationCreated,
            other => V1IssuingAuthorizationCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IssuingCardCreatedRelatedObjectType {
    IssuingCard,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IssuingCardCreatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IssuingCardCreatedRelatedObjectType::IssuingCard => "issuing.card",
            V1IssuingCardCreatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IssuingCardCreatedRelatedObjectType {
    fn default() -> Self {
        V1IssuingCardCreatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IssuingCardCreatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IssuingCardCreatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "issuing.card" => V1IssuingCardCreatedRelatedObjectType::IssuingCard,
            other => V1IssuingCardCreatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IssuingCardholderUpdatedType {
    V1IssuingCardholderUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IssuingCardholderUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IssuingCardholderUpdatedType::V1IssuingCardholderUpdated => "v1.issuing_cardholder.updated",
            V1IssuingCardholderUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IssuingCardholderUpdatedType {
    fn default() -> Self {
        V1IssuingCardholderUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IssuingCardholderUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IssuingCardholderUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.issuing_cardholder.updated" => V1IssuingCardholderUpdatedType::V1IssuingCardholderUpdated,
            other => V1IssuingCardholderUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IssuingDisputeCreatedType {
    V1IssuingDisputeCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IssuingDisputeCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IssuingDisputeCreatedType::V1IssuingDisputeCreated => "v1.issuing_dispute.created",
            V1IssuingDisputeCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IssuingDisputeCreatedType {
    fn default() -> Self {
        V1IssuingDisputeCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IssuingDisputeCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IssuingDisputeCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.issuing_dispute.created" => V1IssuingDisputeCreatedType::V1IssuingDisputeCreated,
            other => V1IssuingDisputeCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IssuingDisputeSubmittedType {
    V1IssuingDisputeSubmitted,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IssuingDisputeSubmittedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IssuingDisputeSubmittedType::V1IssuingDisputeSubmitted => "v1.issuing_dispute.submitted",
            V1IssuingDisputeSubmittedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IssuingDisputeSubmittedType {
    fn default() -> Self {
        V1IssuingDisputeSubmittedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IssuingDisputeSubmittedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IssuingDisputeSubmittedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.issuing_dispute.submitted" => V1IssuingDisputeSubmittedType::V1IssuingDisputeSubmitted,
            other => V1IssuingDisputeSubmittedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IssuingPersonalizationDesignUpdatedType {
    V1IssuingPersonalizationDesignUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IssuingPersonalizationDesignUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IssuingPersonalizationDesignUpdatedType::V1IssuingPersonalizationDesignUpdated => {
                "v1.issuing_personalization_design.updated"
            }
            V1IssuingPersonalizationDesignUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IssuingPersonalizationDesignUpdatedType {
    fn default() -> Self {
        V1IssuingPersonalizationDesignUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IssuingPersonalizationDesignUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IssuingPersonalizationDesignUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.issuing_personalization_design.updated" => {
                V1IssuingPersonalizationDesignUpdatedType::V1IssuingPersonalizationDesignUpdated
            }
            other => V1IssuingPersonalizationDesignUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IssuingTokenUpdatedType {
    V1IssuingTokenUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IssuingTokenUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IssuingTokenUpdatedType::V1IssuingTokenUpdated => "v1.issuing_token.updated",
            V1IssuingTokenUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IssuingTokenUpdatedType {
    fn default() -> Self {
        V1IssuingTokenUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IssuingTokenUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IssuingTokenUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.issuing_token.updated" => V1IssuingTokenUpdatedType::V1IssuingTokenUpdated,
            other => V1IssuingTokenUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PaymentIntentAmountCapturableUpdatedType {
    V1PaymentIntentAmountCapturableUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PaymentIntentAmountCapturableUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PaymentIntentAmountCapturableUpdatedType::V1PaymentIntentAmountCapturableUpdated => {
                "v1.payment_intent.amount_capturable_updated"
            }
            V1PaymentIntentAmountCapturableUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PaymentIntentAmountCapturableUpdatedType {
    fn default() -> Self {
        V1PaymentIntentAmountCapturableUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PaymentIntentAmountCapturableUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PaymentIntentAmountCapturableUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.payment_intent.amount_capturable_updated" => {
                V1PaymentIntentAmountCapturableUpdatedType::V1PaymentIntentAmountCapturableUpdated
            }
            other => V1PaymentIntentAmountCapturableUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PersonDeletedType {
    V1PersonDeleted,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PersonDeletedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PersonDeletedType::V1PersonDeleted => "v1.person.deleted",
            V1PersonDeletedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PersonDeletedType {
    fn default() -> Self {
        V1PersonDeletedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PersonDeletedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PersonDeletedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.person.deleted" => V1PersonDeletedType::V1PersonDeleted,
            other => V1PersonDeletedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PlanDeletedType {
    V1PlanDeleted,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PlanDeletedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PlanDeletedType::V1PlanDeleted => "v1.plan.deleted",
            V1PlanDeletedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PlanDeletedType {
    fn default() -> Self {
        V1PlanDeletedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PlanDeletedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PlanDeletedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.plan.deleted" => V1PlanDeletedType::V1PlanDeleted,
            other => V1PlanDeletedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ProductCreatedRelatedObjectType {
    Product,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ProductCreatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ProductCreatedRelatedObjectType::Product => "product",
            V1ProductCreatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ProductCreatedRelatedObjectType {
    fn default() -> Self {
        V1ProductCreatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ProductCreatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ProductCreatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "product" => V1ProductCreatedRelatedObjectType::Product,
            other => V1ProductCreatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ProductUpdatedType {
    V1ProductUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ProductUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ProductUpdatedType::V1ProductUpdated => "v1.product.updated",
            V1ProductUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ProductUpdatedType {
    fn default() -> Self {
        V1ProductUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ProductUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ProductUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.product.updated" => V1ProductUpdatedType::V1ProductUpdated,
            other => V1ProductUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PromotionCodeUpdatedType {
    V1PromotionCodeUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PromotionCodeUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PromotionCodeUpdatedType::V1PromotionCodeUpdated => "v1.promotion_code.updated",
            V1PromotionCodeUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PromotionCodeUpdatedType {
    fn default() -> Self {
        V1PromotionCodeUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PromotionCodeUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PromotionCodeUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.promotion_code.updated" => V1PromotionCodeUpdatedType::V1PromotionCodeUpdated,
            other => V1PromotionCodeUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1RadarEarlyFraudWarningUpdatedType {
    V1RadarEarlyFraudWarningUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1RadarEarlyFraudWarningUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1RadarEarlyFraudWarningUpdatedType::V1RadarEarlyFraudWarningUpdated => {
                "v1.radar.early_fraud_warning.updated"
            }
            V1RadarEarlyFraudWarningUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1RadarEarlyFraudWarningUpdatedType {
    fn default() -> Self {
        V1RadarEarlyFraudWarningUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1RadarEarlyFraudWarningUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1RadarEarlyFraudWarningUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.radar.early_fraud_warning.updated" => {
                V1RadarEarlyFraudWarningUpdatedType::V1RadarEarlyFraudWarningUpdated
            }
            other => V1RadarEarlyFraudWarningUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1RefundUpdatedType {
    V1RefundUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1RefundUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1RefundUpdatedType::V1RefundUpdated => "v1.refund.updated",
            V1RefundUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1RefundUpdatedType {
    fn default() -> Self {
        V1RefundUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1RefundUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1RefundUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.refund.updated" => V1RefundUpdatedType::V1RefundUpdated,
            other => V1RefundUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ReviewOpenedType {
    V1ReviewOpened,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ReviewOpenedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ReviewOpenedType::V1ReviewOpened => "v1.review.opened",
            V1ReviewOpenedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ReviewOpenedType {
    fn default() -> Self {
        V1ReviewOpenedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ReviewOpenedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ReviewOpenedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.review.opened" => V1ReviewOpenedType::V1ReviewOpened,
            other => V1ReviewOpenedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1SetupIntentCreatedType {
    V1SetupIntentCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1SetupIntentCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1SetupIntentCreatedType::V1SetupIntentCreated => "v1.setup_intent.created",
            V1SetupIntentCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1SetupIntentCreatedType {
    fn default() -> Self {
        V1SetupIntentCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1SetupIntentCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1SetupIntentCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.setup_intent.created" => V1SetupIntentCreatedType::V1SetupIntentCreated,
            other => V1SetupIntentCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1SetupIntentRequiresActionType {
    V1SetupIntentRequiresAction,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1SetupIntentRequiresActionType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1SetupIntentRequiresActionType::V1SetupIntentRequiresAction => "v1.setup_intent.requires_action",
            V1SetupIntentRequiresActionType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1SetupIntentRequiresActionType {
    fn default() -> Self {
        V1SetupIntentRequiresActionType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1SetupIntentRequiresActionType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1SetupIntentRequiresActionType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.setup_intent.requires_action" => V1SetupIntentRequiresActionType::V1SetupIntentRequiresAction,
            other => V1SetupIntentRequiresActionType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1SetupIntentSetupFailedType {
    V1SetupIntentSetupFailed,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1SetupIntentSetupFailedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1SetupIntentSetupFailedType::V1SetupIntentSetupFailed => "v1.setup_intent.setup_failed",
            V1SetupIntentSetupFailedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1SetupIntentSetupFailedType {
    fn default() -> Self {
        V1SetupIntentSetupFailedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1SetupIntentSetupFailedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1SetupIntentSetupFailedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.setup_intent.setup_failed" => V1SetupIntentSetupFailedType::V1SetupIntentSetupFailed,
            other => V1SetupIntentSetupFailedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1SubscriptionScheduleExpiringType {
    V1SubscriptionScheduleExpiring,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1SubscriptionScheduleExpiringType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1SubscriptionScheduleExpiringType::V1SubscriptionScheduleExpiring => "v1.subscription_schedule.expiring",
            V1SubscriptionScheduleExpiringType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1SubscriptionScheduleExpiringType {
    fn default() -> Self {
        V1SubscriptionScheduleExpiringType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1SubscriptionScheduleExpiringType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1SubscriptionScheduleExpiringType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.subscription_schedule.expiring" => V1SubscriptionScheduleExpiringType::V1SubscriptionScheduleExpiring,
            other => V1SubscriptionScheduleExpiringType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1SubscriptionScheduleUpdatedType {
    V1SubscriptionScheduleUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1SubscriptionScheduleUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1SubscriptionScheduleUpdatedType::V1SubscriptionScheduleUpdated => "v1.subscription_schedule.updated",
            V1SubscriptionScheduleUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1SubscriptionScheduleUpdatedType {
    fn default() -> Self {
        V1SubscriptionScheduleUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1SubscriptionScheduleUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1SubscriptionScheduleUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.subscription_schedule.updated" => V1SubscriptionScheduleUpdatedType::V1SubscriptionScheduleUpdated,
            other => V1SubscriptionScheduleUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1TerminalReaderActionFailedType {
    V1TerminalReaderActionFailed,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1TerminalReaderActionFailedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1TerminalReaderActionFailedType::V1TerminalReaderActionFailed => "v1.terminal.reader.action_failed",
            V1TerminalReaderActionFailedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1TerminalReaderActionFailedType {
    fn default() -> Self {
        V1TerminalReaderActionFailedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1TerminalReaderActionFailedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1TerminalReaderActionFailedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.terminal.reader.action_failed" => V1TerminalReaderActionFailedType::V1TerminalReaderActionFailed,
            other => V1TerminalReaderActionFailedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1TerminalReaderActionSucceededType {
    V1TerminalReaderActionSucceeded,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1TerminalReaderActionSucceededType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1TerminalReaderActionSucceededType::V1TerminalReaderActionSucceeded => {
                "v1.terminal.reader.action_succeeded"
            }
            V1TerminalReaderActionSucceededType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1TerminalReaderActionSucceededType {
    fn default() -> Self {
        V1TerminalReaderActionSucceededType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1TerminalReaderActionSucceededType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1TerminalReaderActionSucceededType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.terminal.reader.action_succeeded" => {
                V1TerminalReaderActionSucceededType::V1TerminalReaderActionSucceeded
            }
            other => V1TerminalReaderActionSucceededType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1TestHelpersTestClockAdvancingRelatedObjectType {
    TestHelpersTestClock,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1TestHelpersTestClockAdvancingRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1TestHelpersTestClockAdvancingRelatedObjectType::TestHelpersTestClock => "test_helpers.test_clock",
            V1TestHelpersTestClockAdvancingRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1TestHelpersTestClockAdvancingRelatedObjectType {
    fn default() -> Self {
        V1TestHelpersTestClockAdvancingRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1TestHelpersTestClockAdvancingRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1TestHelpersTestClockAdvancingRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "test_helpers.test_clock" => V1TestHelpersTestClockAdvancingRelatedObjectType::TestHelpersTestClock,
            other => V1TestHelpersTestClockAdvancingRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1TopupCanceledRelatedObjectType {
    Topup,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1TopupCanceledRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1TopupCanceledRelatedObjectType::Topup => "topup",
            V1TopupCanceledRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1TopupCanceledRelatedObjectType {
    fn default() -> Self {
        V1TopupCanceledRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1TopupCanceledRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1TopupCanceledRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "topup" => V1TopupCanceledRelatedObjectType::Topup,
            other => V1TopupCanceledRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CommerceProductCatalogImportsFailedChanges {
    #[doc = "Updated values of properties that the event changed."]
    #[json(default)]
    pub after: json::OwnedValue,
    #[doc = "Values of properties before the event changes."]
    #[json(default)]
    pub before: json::OwnedValue,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CommerceProductCatalogImportsSucceededWithErrorsType {
    V2CommerceProductCatalogImportsSucceededWithErrors,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V2CommerceProductCatalogImportsSucceededWithErrorsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CommerceProductCatalogImportsSucceededWithErrorsType::V2CommerceProductCatalogImportsSucceededWithErrors => "v2.commerce.product_catalog.imports.succeeded_with_errors",
            V2CommerceProductCatalogImportsSucceededWithErrorsType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V2CommerceProductCatalogImportsSucceededWithErrorsType {
    fn default() -> Self {
        V2CommerceProductCatalogImportsSucceededWithErrorsType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V2CommerceProductCatalogImportsSucceededWithErrorsType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CommerceProductCatalogImportsSucceededWithErrorsType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.commerce.product_catalog.imports.succeeded_with_errors" => V2CommerceProductCatalogImportsSucceededWithErrorsType::V2CommerceProductCatalogImportsSucceededWithErrors,
            other => V2CommerceProductCatalogImportsSucceededWithErrorsType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountClosedRelatedObjectType {
    V2CoreAccount,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V2CoreAccountClosedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountClosedRelatedObjectType::V2CoreAccount => "v2.core.account",
            V2CoreAccountClosedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V2CoreAccountClosedRelatedObjectType {
    fn default() -> Self {
        V2CoreAccountClosedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V2CoreAccountClosedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountClosedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.core.account" => V2CoreAccountClosedRelatedObjectType::V2CoreAccount,
            other => V2CoreAccountClosedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedType {
    V2CoreAccountConfigurationRecipientCapabilityStatusUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedType::V2CoreAccountConfigurationRecipientCapabilityStatusUpdated => "v2.core.account[configuration.recipient].capability_status_updated",
            V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedType {
    fn default() -> Self {
        V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.core.account[configuration.recipient].capability_status_updated" => V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedType::V2CoreAccountConfigurationRecipientCapabilityStatusUpdated,
            other => V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountPersonDeletedType {
    V2CoreAccountPersonDeleted,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V2CoreAccountPersonDeletedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountPersonDeletedType::V2CoreAccountPersonDeleted => "v2.core.account_person.deleted",
            V2CoreAccountPersonDeletedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V2CoreAccountPersonDeletedType {
    fn default() -> Self {
        V2CoreAccountPersonDeletedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V2CoreAccountPersonDeletedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountPersonDeletedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.core.account_person.deleted" => V2CoreAccountPersonDeletedType::V2CoreAccountPersonDeleted,
            other => V2CoreAccountPersonDeletedType::Unknown(other.to_string()),
        })
    }
}
