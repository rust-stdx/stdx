#[cfg(any(feature = "billing", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BillingAlert {
    #[doc = "Defines the type of the alert."]
    #[json(default)]
    pub alert_type: stripe_types_00::billing::BillingAlertAlertType,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::billing::BillingAlertObject,
    #[doc = "Status of the alert."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<stripe_types_02::billing::BillingAlertStatus>,
    #[doc = "Title of the alert."]
    #[json(default)]
    pub title: String,
    #[doc = "Encapsulates configuration of the alert to monitor usage on a specific Billing Meter..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub usage_threshold: Option<crate::billing::ThresholdsResourceUsageThresholdConfig>,
}

#[cfg(any(feature = "billing", feature = "events"))]
impl crate::PagedItem for BillingAlert {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BillingAlertTriggered {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub alert: Option<crate::billing::BillingAlert>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "ID of customer for which the alert triggered"]
    #[json(default)]
    pub customer: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_01::billing::BillingAlertTriggeredObject,
    #[doc = "The value triggering the alert"]
    #[json(default)]
    pub value: i64,
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BillingCreditBalanceSummary {
    #[doc = "The billing credit balances."]
    #[json(default)]
    pub balances: Vec<stripe_types_08::billing::CreditBalance>,
    #[doc = "The customer the balance is for."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<crate::Expandable<stripe_types_14::billing::BillingCreditBalanceSummaryCustomerExpanded>>,
    #[doc = "The account the balance is for."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_account: Option<String>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_02::billing::BillingCreditBalanceSummaryObject,
}

#[cfg(any(feature = "billing", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ThresholdsResourceUsageAlertFilter {
    #[doc = "Limit the scope of the alert to this customer ID"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<crate::Expandable<stripe_types_14::customers::Customer>>,
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::billing::ThresholdsResourceUsageAlertFilterType,
}

#[cfg(any(feature = "billing", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ThresholdsResourceUsageThresholdConfig {
    #[doc = "The filters allow limiting the scope of this usage alert."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<crate::billing::ThresholdsResourceUsageAlertFilter>>,
    #[doc = "The value at which this alert will trigger."]
    #[json(default)]
    pub gte: i64,
    #[doc = "The [Billing Meter](/api/billing/meter) ID whose usage is monitored."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub meter: Option<crate::Expandable<stripe_types_09::billing::BillingMeter>>,
    #[doc = "Defines how the alert will behave."]
    #[json(default)]
    pub recurrence: stripe_types_02::billing::ThresholdsResourceUsageThresholdConfigRecurrence,
}
