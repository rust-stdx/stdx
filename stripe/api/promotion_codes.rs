#[cfg(feature = "promotion_codes")]
#[doc = "Create a promotion code"]
pub async fn create_promotion_code(
    client: &crate::Client,
    params: &stripe_types_09::promotion_codes::CreatePromotionCodeParams,
) -> Result<stripe_types_14::promotion_codes::PromotionCode, crate::Error> {
    let path = crate::params::format_path("/v1/promotion_codes", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "promotion_codes")]
#[doc = "Retrieve a promotion code"]
pub async fn get_promotion_code(
    client: &crate::Client,
    promotion_code: &str,
    params: &stripe_types_01::promotion_codes::GetPromotionCodeParams,
) -> Result<stripe_types_14::promotion_codes::PromotionCode, crate::Error> {
    let path =
        crate::params::format_path("/v1/promotion_codes/{promotion_code}", &[("promotion_code", promotion_code)]);
    client.get(&path, params).await
}

#[cfg(feature = "promotion_codes")]
#[doc = "List all promotion codes"]
pub async fn list_promotion_codes(
    client: &crate::Client,
    params: &stripe_types_05::promotion_codes::ListPromotionCodesParams,
) -> Result<crate::List<stripe_types_14::promotion_codes::PromotionCode>, crate::Error> {
    let path = crate::params::format_path("/v1/promotion_codes", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "promotion_codes")]
#[doc = "Update a promotion code"]
pub async fn update_promotion_code(
    client: &crate::Client,
    promotion_code: &str,
    params: &stripe_types_10::promotion_codes::UpdatePromotionCodeParams,
) -> Result<stripe_types_14::promotion_codes::PromotionCode, crate::Error> {
    let path =
        crate::params::format_path("/v1/promotion_codes/{promotion_code}", &[("promotion_code", promotion_code)]);
    client.post_form(&path, params).await
}
