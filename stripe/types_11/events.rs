#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BillingMeterCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_09::billing::BillingMeter>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BillingMeterReactivated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_09::billing::BillingMeter>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CashBalanceFundsAvailable {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_08::customers::CashBalance>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ReportingReportRunSucceeded {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_08::reporting::ReportingReportRun>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SigmaScheduledQueryRunCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_09::sigma::ScheduledQueryRun>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxSettingsUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_08::tax::TaxSettings>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TestHelpersTestClockAdvancing {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_08::test_helpers::TestHelpersTestClock>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TestHelpersTestClockDeleted {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_08::test_helpers::TestHelpersTestClock>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1BillingMeterErrorReportTriggeredDataReason {
    #[doc = "The total error count within this window."]
    #[json(default)]
    pub error_count: i64,
    #[doc = "The error details."]
    #[json(default)]
    pub error_types: Vec<stripe_types_08::events::V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItem>,
}
