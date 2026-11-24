#[cfg(feature = "coupons")]
#[doc = "Create a coupon"]
pub async fn create_coupon(
    client: &crate::Client,
    params: &stripe_types_05::coupons::CreateCouponParams,
) -> Result<stripe_types_06::coupons::Coupon, crate::Error> {
    let path = crate::params::format_path("/v1/coupons", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "coupons")]
#[doc = "Delete a coupon"]
pub async fn delete_coupon(
    client: &crate::Client,
    coupon: &str,
    params: &crate::params::Empty,
) -> Result<stripe_types_06::coupons::DeletedCoupon, crate::Error> {
    let path = crate::params::format_path("/v1/coupons/{coupon}", &[("coupon", coupon)]);
    client.delete_form(&path, params).await
}

#[cfg(feature = "coupons")]
#[doc = "Retrieve a coupon"]
pub async fn get_coupon(
    client: &crate::Client,
    coupon: &str,
    params: &stripe_types_00::coupons::GetCouponParams,
) -> Result<stripe_types_06::coupons::Coupon, crate::Error> {
    let path = crate::params::format_path("/v1/coupons/{coupon}", &[("coupon", coupon)]);
    client.get(&path, params).await
}

#[cfg(feature = "coupons")]
#[doc = "List all coupons"]
pub async fn list_coupons(
    client: &crate::Client,
    params: &stripe_types_06::coupons::ListCouponsParams,
) -> Result<crate::List<stripe_types_06::coupons::Coupon>, crate::Error> {
    let path = crate::params::format_path("/v1/coupons", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "coupons")]
#[doc = "Update a coupon"]
pub async fn update_coupon(
    client: &crate::Client,
    coupon: &str,
    params: &stripe_types_08::coupons::UpdateCouponParams,
) -> Result<stripe_types_06::coupons::Coupon, crate::Error> {
    let path = crate::params::format_path("/v1/coupons/{coupon}", &[("coupon", coupon)]);
    client.post_form(&path, params).await
}
