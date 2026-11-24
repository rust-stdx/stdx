#[cfg(feature = "quotes")]
#[doc = "Accept a quote"]
pub async fn accept_quote(
    client: &crate::Client,
    quote: &str,
    params: &stripe_types_05::quotes::AcceptQuoteParams,
) -> Result<stripe_types_15::quotes::Quote, crate::Error> {
    let path = crate::params::format_path("/v1/quotes/{quote}/accept", &[("quote", quote)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "quotes")]
#[doc = "Cancel a quote"]
pub async fn cancel_quote(
    client: &crate::Client,
    quote: &str,
    params: &stripe_types_06::quotes::CancelQuoteParams,
) -> Result<stripe_types_15::quotes::Quote, crate::Error> {
    let path = crate::params::format_path("/v1/quotes/{quote}/cancel", &[("quote", quote)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "quotes")]
#[doc = "Create a quote"]
pub async fn create_quote(
    client: &crate::Client,
    params: &stripe_types_12::quotes::CreateQuoteParams,
) -> Result<stripe_types_15::quotes::Quote, crate::Error> {
    let path = crate::params::format_path("/v1/quotes", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "quotes")]
#[doc = "Finalize a quote"]
pub async fn finalize_quote(
    client: &crate::Client,
    quote: &str,
    params: &stripe_types_03::quotes::FinalizeQuoteParams,
) -> Result<stripe_types_15::quotes::Quote, crate::Error> {
    let path = crate::params::format_path("/v1/quotes/{quote}/finalize", &[("quote", quote)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "quotes")]
#[doc = "Retrieve a quote"]
pub async fn get_quote(
    client: &crate::Client,
    quote: &str,
    params: &stripe_types_04::quotes::GetQuoteParams,
) -> Result<stripe_types_15::quotes::Quote, crate::Error> {
    let path = crate::params::format_path("/v1/quotes/{quote}", &[("quote", quote)]);
    client.get(&path, params).await
}

#[cfg(feature = "quotes")]
#[doc = "Download quote PDF"]
pub async fn get_quote_pdf(
    client: &crate::Client,
    quote: &str,
    params: &stripe_types_05::quotes::GetQuotePdfParams,
) -> Result<(), crate::Error> {
    let path = crate::params::format_path("/v1/quotes/{quote}/pdf", &[("quote", quote)]);
    client.get(&path, params).await
}

#[cfg(feature = "quotes")]
#[doc = "Retrieve a quote's upfront line items"]
pub async fn list_quote_computed_upfront_line_items(
    client: &crate::Client,
    quote: &str,
    params: &stripe_types_01::quotes::ListQuoteComputedUpfrontLineItemsParams,
) -> Result<crate::List<stripe_types_15::common::Item>, crate::Error> {
    let path = crate::params::format_path("/v1/quotes/{quote}/computed_upfront_line_items", &[("quote", quote)]);
    client.get(&path, params).await
}

#[cfg(feature = "quotes")]
#[doc = "Retrieve a quote's line items"]
pub async fn list_quote_line_items(
    client: &crate::Client,
    quote: &str,
    params: &stripe_types_06::quotes::ListQuoteLineItemsParams,
) -> Result<crate::List<stripe_types_15::common::Item>, crate::Error> {
    let path = crate::params::format_path("/v1/quotes/{quote}/line_items", &[("quote", quote)]);
    client.get(&path, params).await
}

#[cfg(feature = "quotes")]
#[doc = "List all quotes"]
pub async fn list_quotes(
    client: &crate::Client,
    params: &stripe_types_05::quotes::ListQuotesParams,
) -> Result<crate::List<stripe_types_15::quotes::Quote>, crate::Error> {
    let path = crate::params::format_path("/v1/quotes", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "quotes")]
#[doc = "Update a quote"]
pub async fn update_quote(
    client: &crate::Client,
    quote: &str,
    params: &stripe_types_13::quotes::UpdateQuoteParams,
) -> Result<stripe_types_15::quotes::Quote, crate::Error> {
    let path = crate::params::format_path("/v1/quotes/{quote}", &[("quote", quote)]);
    client.post_form(&path, params).await
}
