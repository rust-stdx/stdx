#[cfg(feature = "plans")]
#[doc = "Create a plan"]
pub async fn create_plan(
    client: &crate::Client,
    params: &stripe_types_07::plans::CreatePlanParams,
) -> Result<stripe_types_13::plans::Plan, crate::Error> {
    let path = crate::params::format_path("/v1/plans", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "plans")]
#[doc = "Delete a plan"]
pub async fn delete_plan(
    client: &crate::Client,
    plan: &str,
    params: &crate::params::Empty,
) -> Result<stripe_types_06::plans::DeletedPlan, crate::Error> {
    let path = crate::params::format_path("/v1/plans/{plan}", &[("plan", plan)]);
    client.delete_form(&path, params).await
}

#[cfg(feature = "plans")]
#[doc = "Retrieve a plan"]
pub async fn get_plan(
    client: &crate::Client,
    plan: &str,
    params: &stripe_types_00::plans::GetPlanParams,
) -> Result<stripe_types_13::plans::Plan, crate::Error> {
    let path = crate::params::format_path("/v1/plans/{plan}", &[("plan", plan)]);
    client.get(&path, params).await
}

#[cfg(feature = "plans")]
#[doc = "List all plans"]
pub async fn list_plans(
    client: &crate::Client,
    params: &stripe_types_06::plans::ListPlansParams,
) -> Result<crate::List<stripe_types_13::plans::Plan>, crate::Error> {
    let path = crate::params::format_path("/v1/plans", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "plans")]
#[doc = "Update a plan"]
pub async fn update_plan(
    client: &crate::Client,
    plan: &str,
    params: &stripe_types_04::plans::UpdatePlanParams,
) -> Result<stripe_types_13::plans::Plan, crate::Error> {
    let path = crate::params::format_path("/v1/plans/{plan}", &[("plan", plan)]);
    client.post_form(&path, params).await
}
