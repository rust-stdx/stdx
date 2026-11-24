#[cfg(feature = "reviews")]
#[doc = "Approve a review"]
pub async fn approve_review(
    client: &crate::Client,
    review: &str,
    params: &stripe_types_05::reviews::ApproveReviewParams,
) -> Result<stripe_types_14::reviews::Review, crate::Error> {
    let path = crate::params::format_path("/v1/reviews/{review}/approve", &[("review", review)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "reviews")]
#[doc = "Retrieve a review"]
pub async fn get_review(
    client: &crate::Client,
    review: &str,
    params: &stripe_types_04::reviews::GetReviewParams,
) -> Result<stripe_types_14::reviews::Review, crate::Error> {
    let path = crate::params::format_path("/v1/reviews/{review}", &[("review", review)]);
    client.get(&path, params).await
}

#[cfg(feature = "reviews")]
#[doc = "List all open reviews"]
pub async fn list_reviews(
    client: &crate::Client,
    params: &stripe_types_05::reviews::ListReviewsParams,
) -> Result<crate::List<stripe_types_14::reviews::Review>, crate::Error> {
    let path = crate::params::format_path("/v1/reviews", &[]);
    client.get(&path, params).await
}
