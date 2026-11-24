#[cfg(feature = "billing")]
#[doc = "Activate a billing alert"]
pub async fn activate_billing_alert(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_06::billing::ActivateBillingAlertParams,
) -> Result<stripe_types_15::billing::BillingAlert, crate::Error> {
    let path = crate::params::format_path("/v1/billing/alerts/{id}/activate", &[("id", id)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "billing")]
#[doc = "Archive a billing alert"]
pub async fn archive_billing_alert(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_06::billing::ArchiveBillingAlertParams,
) -> Result<stripe_types_15::billing::BillingAlert, crate::Error> {
    let path = crate::params::format_path("/v1/billing/alerts/{id}/archive", &[("id", id)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "billing")]
#[doc = "Create a billing alert"]
pub async fn create_billing_alert(
    client: &crate::Client,
    params: &stripe_types_10::billing::CreateBillingAlertParams,
) -> Result<stripe_types_15::billing::BillingAlert, crate::Error> {
    let path = crate::params::format_path("/v1/billing/alerts", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "billing")]
#[doc = "Create a credit grant"]
pub async fn create_billing_credit_grant(
    client: &crate::Client,
    params: &stripe_types_12::billing::CreateBillingCreditGrantParams,
) -> Result<stripe_types_14::billing::BillingCreditGrant, crate::Error> {
    let path = crate::params::format_path("/v1/billing/credit_grants", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "billing")]
#[doc = "Create a feedback option"]
pub async fn create_billing_feedback_option(
    client: &crate::Client,
    params: &stripe_types_04::billing::CreateBillingFeedbackOptionParams,
) -> Result<stripe_types_06::billing::BillingFeedbackOption, crate::Error> {
    let path = crate::params::format_path("/v1/billing/feedback_options", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "billing")]
#[doc = "Create a billing meter"]
pub async fn create_billing_meter(
    client: &crate::Client,
    params: &stripe_types_09::billing::CreateBillingMeterParams,
) -> Result<stripe_types_09::billing::BillingMeter, crate::Error> {
    let path = crate::params::format_path("/v1/billing/meters", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "billing")]
#[doc = "Create a billing meter event"]
pub async fn create_billing_meter_event(
    client: &crate::Client,
    params: &stripe_types_03::billing::CreateBillingMeterEventParams,
) -> Result<stripe_types_06::billing::BillingMeterEvent, crate::Error> {
    let path = crate::params::format_path("/v1/billing/meter_events", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "billing")]
#[doc = "Create a billing meter event adjustment"]
pub async fn create_billing_meter_event_adjustment(
    client: &crate::Client,
    params: &stripe_types_05::billing::CreateBillingMeterEventAdjustmentParams,
) -> Result<stripe_types_06::billing::BillingMeterEventAdjustment, crate::Error> {
    let path = crate::params::format_path("/v1/billing/meter_event_adjustments", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "billing")]
#[doc = "Deactivate a billing alert"]
pub async fn deactivate_billing_alert(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_05::billing::DeactivateBillingAlertParams,
) -> Result<stripe_types_15::billing::BillingAlert, crate::Error> {
    let path = crate::params::format_path("/v1/billing/alerts/{id}/deactivate", &[("id", id)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "billing")]
#[doc = "Deactivate a feedback option"]
pub async fn deactivate_billing_feedback_option(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_05::billing::DeactivateBillingFeedbackOptionParams,
) -> Result<stripe_types_06::billing::BillingFeedbackOption, crate::Error> {
    let path = crate::params::format_path("/v1/billing/feedback_options/{id}/deactivate", &[("id", id)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "billing")]
#[doc = "Deactivate a billing meter"]
pub async fn deactivate_billing_meter(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_06::billing::DeactivateBillingMeterParams,
) -> Result<stripe_types_09::billing::BillingMeter, crate::Error> {
    let path = crate::params::format_path("/v1/billing/meters/{id}/deactivate", &[("id", id)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "billing")]
#[doc = "Expire a credit grant"]
pub async fn expire_billing_credit_grant(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_06::billing::ExpireBillingCreditGrantParams,
) -> Result<stripe_types_14::billing::BillingCreditGrant, crate::Error> {
    let path = crate::params::format_path("/v1/billing/credit_grants/{id}/expire", &[("id", id)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "billing")]
#[doc = "Retrieve a billing alert"]
pub async fn get_billing_alert(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_00::billing::GetBillingAlertParams,
) -> Result<stripe_types_15::billing::BillingAlert, crate::Error> {
    let path = crate::params::format_path("/v1/billing/alerts/{id}", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "billing")]
#[doc = "Retrieve the credit balance summary for a customer"]
pub async fn get_billing_credit_balance_summary(
    client: &crate::Client,
    params: &stripe_types_10::billing::GetBillingCreditBalanceSummaryParams,
) -> Result<stripe_types_15::billing::BillingCreditBalanceSummary, crate::Error> {
    let path = crate::params::format_path("/v1/billing/credit_balance_summary", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "billing")]
#[doc = "Retrieve a credit balance transaction"]
pub async fn get_billing_credit_balance_transaction(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_06::billing::GetBillingCreditBalanceTransactionParams,
) -> Result<stripe_types_14::billing::BillingCreditBalanceTransaction, crate::Error> {
    let path = crate::params::format_path("/v1/billing/credit_balance_transactions/{id}", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "billing")]
#[doc = "Retrieve a credit grant"]
pub async fn get_billing_credit_grant(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_05::billing::GetBillingCreditGrantParams,
) -> Result<stripe_types_14::billing::BillingCreditGrant, crate::Error> {
    let path = crate::params::format_path("/v1/billing/credit_grants/{id}", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "billing")]
#[doc = "Retrieve a feedback option"]
pub async fn get_billing_feedback_option(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_05::billing::GetBillingFeedbackOptionParams,
) -> Result<stripe_types_06::billing::BillingFeedbackOption, crate::Error> {
    let path = crate::params::format_path("/v1/billing/feedback_options/{id}", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "billing")]
#[doc = "Retrieve a billing meter"]
pub async fn get_billing_meter(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_06::billing::GetBillingMeterParams,
) -> Result<stripe_types_09::billing::BillingMeter, crate::Error> {
    let path = crate::params::format_path("/v1/billing/meters/{id}", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "billing")]
#[doc = "List billing alerts"]
pub async fn list_billing_alerts(
    client: &crate::Client,
    params: &stripe_types_06::billing::ListBillingAlertsParams,
) -> Result<crate::List<stripe_types_15::billing::BillingAlert>, crate::Error> {
    let path = crate::params::format_path("/v1/billing/alerts", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "billing")]
#[doc = "List credit balance transactions"]
pub async fn list_billing_credit_balance_transactions(
    client: &crate::Client,
    params: &stripe_types_04::billing::ListBillingCreditBalanceTransactionsParams,
) -> Result<crate::List<stripe_types_14::billing::BillingCreditBalanceTransaction>, crate::Error> {
    let path = crate::params::format_path("/v1/billing/credit_balance_transactions", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "billing")]
#[doc = "List credit grants"]
pub async fn list_billing_credit_grants(
    client: &crate::Client,
    params: &stripe_types_00::billing::ListBillingCreditGrantsParams,
) -> Result<crate::List<stripe_types_14::billing::BillingCreditGrant>, crate::Error> {
    let path = crate::params::format_path("/v1/billing/credit_grants", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "billing")]
#[doc = "List all feedback options"]
pub async fn list_billing_feedback_options(
    client: &crate::Client,
    params: &stripe_types_05::billing::ListBillingFeedbackOptionsParams,
) -> Result<crate::List<stripe_types_06::billing::BillingFeedbackOption>, crate::Error> {
    let path = crate::params::format_path("/v1/billing/feedback_options", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "billing")]
#[doc = "List billing meter event summaries"]
pub async fn list_billing_meter_event_summaries(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_08::billing::ListBillingMeterEventSummariesParams,
) -> Result<crate::List<stripe_types_05::billing::BillingMeterEventSummary>, crate::Error> {
    let path = crate::params::format_path("/v1/billing/meters/{id}/event_summaries", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "billing")]
#[doc = "List billing meters"]
pub async fn list_billing_meters(
    client: &crate::Client,
    params: &stripe_types_07::billing::ListBillingMetersParams,
) -> Result<crate::List<stripe_types_09::billing::BillingMeter>, crate::Error> {
    let path = crate::params::format_path("/v1/billing/meters", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "billing")]
#[doc = "Reactivate a billing meter"]
pub async fn reactivate_billing_meter(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_05::billing::ReactivateBillingMeterParams,
) -> Result<stripe_types_09::billing::BillingMeter, crate::Error> {
    let path = crate::params::format_path("/v1/billing/meters/{id}/reactivate", &[("id", id)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "billing")]
#[doc = "Update a credit grant"]
pub async fn update_billing_credit_grant(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_03::billing::UpdateBillingCreditGrantParams,
) -> Result<stripe_types_14::billing::BillingCreditGrant, crate::Error> {
    let path = crate::params::format_path("/v1/billing/credit_grants/{id}", &[("id", id)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "billing")]
#[doc = "Update a feedback option"]
pub async fn update_billing_feedback_option(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_03::billing::UpdateBillingFeedbackOptionParams,
) -> Result<stripe_types_06::billing::BillingFeedbackOption, crate::Error> {
    let path = crate::params::format_path("/v1/billing/feedback_options/{id}", &[("id", id)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "billing")]
#[doc = "Update a billing meter"]
pub async fn update_billing_meter(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_02::billing::UpdateBillingMeterParams,
) -> Result<stripe_types_09::billing::BillingMeter, crate::Error> {
    let path = crate::params::format_path("/v1/billing/meters/{id}", &[("id", id)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "billing")]
#[doc = "Void a credit grant"]
pub async fn void_billing_credit_grant(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_06::billing::VoidBillingCreditGrantParams,
) -> Result<stripe_types_14::billing::BillingCreditGrant, crate::Error> {
    let path = crate::params::format_path("/v1/billing/credit_grants/{id}/void", &[("id", id)]);
    client.post_form(&path, params).await
}
