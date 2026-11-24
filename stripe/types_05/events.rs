#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct Event {
    #[doc = "The connected account that originates the event."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[doc = "The Stripe API version used to render `data` when the event was created."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    #[doc = "Authentication context needed to fetch the event or related object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<stripe_types_00::events::NotificationEventData>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::events::EventObject,
    #[doc = "Number of webhooks that haven't been successfully delivered (for example, to return a 20x response) to the URLs you s..."]
    #[json(default)]
    pub pending_webhooks: i64,
    #[doc = "Information on the API request that triggers the event."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<stripe_types_03::events::NotificationEventRequest>,
    #[doc = "Description of the event (for example, `invoice.created` or `charge.refunded`)."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::events::EventType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for Event {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1AccountUpdatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::events::V1AccountUpdatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1AccountUpdatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ApplicationFeeRefundUpdatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::events::V1ApplicationFeeRefundUpdatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ApplicationFeeRefundUpdatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1BalanceSettingsUpdatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::events::V1BalanceSettingsUpdatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1BillingAlertTriggeredRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V1BillingAlertTriggeredRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1BillingAlertTriggeredRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1BillingCreditBalanceTransactionCreatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::events::V1BillingCreditBalanceTransactionCreatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1BillingCreditBalanceTransactionCreatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItemSampleErrorsItem {
    #[doc = "The error message."]
    #[json(default)]
    pub error_message: String,
    #[doc = "The request causes the error."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<
        stripe_types_04::events::V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItemSampleErrorsItemRequest,
    >,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CashBalanceFundsAvailableRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::events::V1CashBalanceFundsAvailableRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ChargeDisputeClosedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::events::V1ChargeDisputeClosedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ChargeDisputeClosedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CheckoutSessionAsyncPaymentFailedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::events::V1CheckoutSessionAsyncPaymentFailedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CheckoutSessionAsyncPaymentFailedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CouponCreatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V1CouponCreatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CouponCreatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CreditNoteCreatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::events::V1CreditNoteCreatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CreditNoteCreatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CustomerCashBalanceTransactionCreatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V1CustomerCashBalanceTransactionCreatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CustomerCashBalanceTransactionCreatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CustomerDiscountCreatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::events::V1CustomerDiscountCreatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CustomerTaxIdCreatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::events::V1CustomerTaxIdCreatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CustomerTaxIdCreatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1FileCreatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V1FileCreatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1FileCreatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1FinancialConnectionsAccountAccountNumbersUpdatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::events::V1FinancialConnectionsAccountAccountNumbersUpdatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1FinancialConnectionsAccountAccountNumbersUpdatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IdentityVerificationSessionCanceledRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V1IdentityVerificationSessionCanceledRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IdentityVerificationSessionCanceledRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1InvoiceCreatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V1InvoiceCreatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1InvoiceCreatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1InvoiceitemCreatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V1InvoiceitemCreatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1InvoiceitemCreatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IssuingAuthorizationCreatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::events::V1IssuingAuthorizationCreatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IssuingAuthorizationCreatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IssuingCardholderCreatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V1IssuingCardholderCreatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IssuingCardholderCreatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IssuingTokenCreatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::events::V1IssuingTokenCreatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IssuingTokenCreatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IssuingTransactionCreatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::events::V1IssuingTransactionCreatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IssuingTransactionCreatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PaymentIntentAmountCapturableUpdatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::events::V1PaymentIntentAmountCapturableUpdatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PaymentIntentAmountCapturableUpdatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PaymentLinkCreatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V1PaymentLinkCreatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PaymentLinkCreatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PaymentMethodAttachedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::events::V1PaymentMethodAttachedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PaymentMethodAttachedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PayoutCanceledRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::events::V1PayoutCanceledRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PayoutCanceledRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PriceCreatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V1PriceCreatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PriceCreatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PromotionCodeCreatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::events::V1PromotionCodeCreatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PromotionCodeCreatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1QuoteAcceptedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::events::V1QuoteAcceptedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1QuoteAcceptedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1RadarEarlyFraudWarningCreatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V1RadarEarlyFraudWarningCreatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1RadarEarlyFraudWarningCreatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1SourceCanceledRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V1SourceCanceledRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1SourceCanceledRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1TestHelpersTestClockAdvancingRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::events::V1TestHelpersTestClockAdvancingRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1TestHelpersTestClockAdvancingRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1TransferCreatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V1TransferCreatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1TransferCreatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationCustomerCapabilityStatusUpdatedData {
    #[doc = "Open Enum."]
    #[json(default)]
    pub updated_capability:
        stripe_types_01::events::V2CoreAccountConfigurationCustomerCapabilityStatusUpdatedDataUpdatedCapability,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountLinkReturnedData {
    #[doc = "The ID of the v2 account."]
    #[json(default)]
    pub account_id: String,
    #[doc = "Configurations on the Account that was onboarded via the account link."]
    #[json(default)]
    pub configurations: Vec<stripe_types_03::events::V2CoreAccountLinkReturnedDataConfigurationsItem>,
    #[doc = "Open Enum."]
    #[json(default)]
    pub use_case: stripe_types_01::events::V2CoreAccountLinkReturnedDataUseCase,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreEventDestinationPingRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V2CoreEventDestinationPingRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V2CoreEventDestinationPingRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}
