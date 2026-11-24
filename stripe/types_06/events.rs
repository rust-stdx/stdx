#[cfg(feature = "events")]
#[derive(Debug, Clone, Default)]
pub struct ListEventsParams {
    pub created: Option<stripe_types_04::events::ListEventsParamsCreated>,
    pub delivery_success: Option<bool>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub r#type: Option<String>,
    pub types: Option<Vec<String>>,
}

#[cfg(feature = "events")]
impl crate::params::FormEncode for ListEventsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("created", self.created.as_ref());
        form.field_opt("delivery_success", self.delivery_success.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("type", self.r#type.as_ref());
        if let Some(values) = self.types.as_ref() {
            form.field_array("types", values.as_slice());
        }
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1AccountApplicationAuthorizedReason {
    #[doc = "Information on the API request that instigated the event."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<stripe_types_04::events::V1AccountApplicationAuthorizedReasonRequest>,
    #[doc = "Event reason type."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V1AccountApplicationAuthorizedReasonType,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ApplicationFeeCreatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::events::V1ApplicationFeeCreatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ApplicationFeeCreatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1BalanceAvailableRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V1BalanceAvailableRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1BillingCreditGrantCreatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::events::V1BillingCreditGrantCreatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1BillingCreditGrantCreatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1BillingMeterCreatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::events::V1BillingMeterCreatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1BillingMeterCreatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1BillingPortalConfigurationCreatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V1BillingPortalConfigurationCreatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1BillingPortalConfigurationCreatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CapabilityUpdatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::events::V1CapabilityUpdatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CapabilityUpdatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ChargeCapturedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V1ChargeCapturedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ChargeCapturedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ChargeRefundUpdatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::events::V1ChargeRefundUpdatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ChargeRefundUpdatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ClimateOrderCanceledRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V1ClimateOrderCanceledRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ClimateOrderCanceledRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ClimateProductCreatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::events::V1ClimateProductCreatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ClimateProductCreatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CustomerCreatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V1CustomerCreatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CustomerCreatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1CustomerSubscriptionCreatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::events::V1CustomerSubscriptionCreatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1CustomerSubscriptionCreatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1InvoicePaymentPaidRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::events::V1InvoicePaymentPaidRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1InvoicePaymentPaidRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IssuingCardCreatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::events::V1IssuingCardCreatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IssuingCardCreatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IssuingDisputeClosedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::events::V1IssuingDisputeClosedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IssuingDisputeClosedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1IssuingPersonalizationDesignActivatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::events::V1IssuingPersonalizationDesignActivatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1IssuingPersonalizationDesignActivatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1MandateUpdatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V1MandateUpdatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1MandateUpdatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PersonCreatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::events::V1PersonCreatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PersonCreatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1PlanCreatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V1PlanCreatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1PlanCreatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ProductCreatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::events::V1ProductCreatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ProductCreatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1ReviewClosedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::events::V1ReviewClosedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1ReviewClosedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1SetupIntentCanceledRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::events::V1SetupIntentCanceledRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1SetupIntentCanceledRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1SigmaScheduledQueryRunCreatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V1SigmaScheduledQueryRunCreatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1SigmaScheduledQueryRunCreatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1SubscriptionScheduleAbortedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V1SubscriptionScheduleAbortedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1SubscriptionScheduleAbortedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1TaxRateCreatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::events::V1TaxRateCreatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1TaxRateCreatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1TaxSettingsUpdatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V1TaxSettingsUpdatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1TerminalReaderActionFailedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::events::V1TerminalReaderActionFailedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1TerminalReaderActionFailedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1TopupCanceledRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::events::V1TopupCanceledRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1TopupCanceledRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CommerceProductCatalogImportsFailedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::events::V2CommerceProductCatalogImportsFailedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V2CommerceProductCatalogImportsFailedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountClosedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::events::V2CoreAccountClosedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V2CoreAccountClosedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedData {
    #[doc = "Open Enum."]
    #[json(default)]
    pub updated_capability:
        stripe_types_01::events::V2CoreAccountConfigurationMerchantCapabilityStatusUpdatedDataUpdatedCapability,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedData {
    #[doc = "Open Enum."]
    #[json(default)]
    pub updated_capability:
        stripe_types_03::events::V2CoreAccountConfigurationRecipientCapabilityStatusUpdatedDataUpdatedCapability,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountPersonCreatedRelatedObject {
    #[doc = "Unique identifier for the object relevant to the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Object tag of the resource relevant to the event."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::events::V2CoreAccountPersonCreatedRelatedObjectType,
    #[doc = "URL to retrieve the resource."]
    #[json(default)]
    pub url: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V2CoreAccountPersonCreatedRelatedObject {
    fn id(&self) -> &str {
        &self.id
    }
}
