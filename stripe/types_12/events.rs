#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BalanceAvailable {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_11::balance::Balance>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ClimateOrderCanceled {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_10::climate::ClimateOrder>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ClimateOrderCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_10::climate::ClimateOrder>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ClimateOrderDelayed {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_10::climate::ClimateOrder>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ClimateOrderDelivered {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_10::climate::ClimateOrder>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ClimateOrderProductSubstituted {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_10::climate::ClimateOrder>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct EntitlementsActiveEntitlementSummaryUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_10::entitlements::EntitlementsActiveEntitlementSummary>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct FinancialConnectionsAuthorizationExpectedDeactivationDateUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_10::financial_connections::FinancialConnectionsAuthorization>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct FinancialConnectionsAuthorizationUpcomingDeactivation {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_10::financial_connections::FinancialConnectionsAuthorization>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IssuingPersonalizationDesignActivated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_10::issuing::IssuingPersonalizationDesign>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IssuingPersonalizationDesignDeactivated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_10::issuing::IssuingPersonalizationDesign>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IssuingPersonalizationDesignRejected {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_10::issuing::IssuingPersonalizationDesign>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IssuingPersonalizationDesignUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_10::issuing::IssuingPersonalizationDesign>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PriceCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_11::prices::Price>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PriceDeleted {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_11::prices::Price>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PriceUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_11::prices::Price>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ProductCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_11::products::Product>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ProductDeleted {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_11::products::Product>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ProductUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_11::products::Product>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SourceCanceled {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_10::sources::Source>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SourceChargeable {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_10::sources::Source>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SourceFailed {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_10::sources::Source>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SourceRefundAttributesRequired {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_10::sources::Source>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1BillingMeterErrorReportTriggeredData {
    #[doc = "Extra field included in the event's `data` when fetched from /v2/events."]
    #[json(default)]
    pub developer_message_summary: String,
    #[doc = "This contains information about why meter error happens."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<stripe_types_11::events::V1BillingMeterErrorReportTriggeredDataReason>,
    #[doc = "The end of the window that is encapsulated by this summary."]
    #[json(default)]
    pub validation_end: String,
    #[doc = "The start of the window that is encapsulated by this summary."]
    #[json(default)]
    pub validation_start: String,
}
