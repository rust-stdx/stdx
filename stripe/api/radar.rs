#[cfg(feature = "radar")]
#[doc = "Create a Payment Evaluation"]
pub async fn create_radar_payment_evaluation(
    client: &crate::Client,
    params: &stripe_types_12::radar::CreateRadarPaymentEvaluationParams,
) -> Result<stripe_types_15::radar::RadarPaymentEvaluation, crate::Error> {
    let path = crate::params::format_path("/v1/radar/payment_evaluations", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "radar")]
#[doc = "Create a value list"]
pub async fn create_radar_value_list(
    client: &crate::Client,
    params: &stripe_types_05::radar::CreateRadarValueListParams,
) -> Result<stripe_types_09::radar::RadarValueList, crate::Error> {
    let path = crate::params::format_path("/v1/radar/value_lists", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "radar")]
#[doc = "Create a value list item"]
pub async fn create_radar_value_list_item(
    client: &crate::Client,
    params: &stripe_types_02::radar::CreateRadarValueListItemParams,
) -> Result<stripe_types_06::radar::RadarValueListItem, crate::Error> {
    let path = crate::params::format_path("/v1/radar/value_list_items", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "radar")]
#[doc = "Delete a value list"]
pub async fn delete_radar_value_list(
    client: &crate::Client,
    value_list: &str,
    params: &crate::params::Empty,
) -> Result<stripe_types_06::radar::DeletedRadarValueList, crate::Error> {
    let path = crate::params::format_path("/v1/radar/value_lists/{value_list}", &[("value_list", value_list)]);
    client.delete_form(&path, params).await
}

#[cfg(feature = "radar")]
#[doc = "Delete a value list item"]
pub async fn delete_radar_value_list_item(
    client: &crate::Client,
    item: &str,
    params: &crate::params::Empty,
) -> Result<stripe_types_05::radar::DeletedRadarValueListItem, crate::Error> {
    let path = crate::params::format_path("/v1/radar/value_list_items/{item}", &[("item", item)]);
    client.delete_form(&path, params).await
}

#[cfg(feature = "radar")]
#[doc = "Retrieve an early fraud warning"]
pub async fn get_radar_early_fraud_warning(
    client: &crate::Client,
    early_fraud_warning: &str,
    params: &stripe_types_02::radar::GetRadarEarlyFraudWarningParams,
) -> Result<stripe_types_15::radar::RadarEarlyFraudWarning, crate::Error> {
    let path = crate::params::format_path(
        "/v1/radar/early_fraud_warnings/{early_fraud_warning}",
        &[("early_fraud_warning", early_fraud_warning)],
    );
    client.get(&path, params).await
}

#[cfg(feature = "radar")]
#[doc = "Retrieve a value list"]
pub async fn get_radar_value_list(
    client: &crate::Client,
    value_list: &str,
    params: &stripe_types_05::radar::GetRadarValueListParams,
) -> Result<stripe_types_09::radar::RadarValueList, crate::Error> {
    let path = crate::params::format_path("/v1/radar/value_lists/{value_list}", &[("value_list", value_list)]);
    client.get(&path, params).await
}

#[cfg(feature = "radar")]
#[doc = "Retrieve a value list item"]
pub async fn get_radar_value_list_item(
    client: &crate::Client,
    item: &str,
    params: &stripe_types_06::radar::GetRadarValueListItemParams,
) -> Result<stripe_types_06::radar::RadarValueListItem, crate::Error> {
    let path = crate::params::format_path("/v1/radar/value_list_items/{item}", &[("item", item)]);
    client.get(&path, params).await
}

#[cfg(feature = "radar")]
#[doc = "List all early fraud warnings"]
pub async fn list_radar_early_fraud_warnings(
    client: &crate::Client,
    params: &stripe_types_06::radar::ListRadarEarlyFraudWarningsParams,
) -> Result<crate::List<stripe_types_15::radar::RadarEarlyFraudWarning>, crate::Error> {
    let path = crate::params::format_path("/v1/radar/early_fraud_warnings", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "radar")]
#[doc = "List all value list items"]
pub async fn list_radar_value_list_items(
    client: &crate::Client,
    params: &stripe_types_09::radar::ListRadarValueListItemsParams,
) -> Result<crate::List<stripe_types_06::radar::RadarValueListItem>, crate::Error> {
    let path = crate::params::format_path("/v1/radar/value_list_items", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "radar")]
#[doc = "List all value lists"]
pub async fn list_radar_value_lists(
    client: &crate::Client,
    params: &stripe_types_07::radar::ListRadarValueListsParams,
) -> Result<crate::List<stripe_types_09::radar::RadarValueList>, crate::Error> {
    let path = crate::params::format_path("/v1/radar/value_lists", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "radar")]
#[doc = "Update a value list"]
pub async fn update_radar_value_list(
    client: &crate::Client,
    value_list: &str,
    params: &stripe_types_01::radar::UpdateRadarValueListParams,
) -> Result<stripe_types_09::radar::RadarValueList, crate::Error> {
    let path = crate::params::format_path("/v1/radar/value_lists/{value_list}", &[("value_list", value_list)]);
    client.post_form(&path, params).await
}
