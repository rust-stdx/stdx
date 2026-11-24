#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BillingMeterDeactivated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_09::billing::BillingMeter>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BillingMeterUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_09::billing::BillingMeter>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ClimateProductCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_07::climate::ClimateProduct>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ClimateProductPricingUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_07::climate::ClimateProduct>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ReportingReportRunFailed {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_08::reporting::ReportingReportRun>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TestHelpersTestClockCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_08::test_helpers::TestHelpersTestClock>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TestHelpersTestClockInternalFailure {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_08::test_helpers::TestHelpersTestClock>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TestHelpersTestClockReady {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_08::test_helpers::TestHelpersTestClock>,
}
