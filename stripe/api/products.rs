#[cfg(feature = "products")]
#[doc = "Create a product"]
pub async fn create_product(
    client: &crate::Client,
    params: &stripe_types_11::products::CreateProductParams,
) -> Result<stripe_types_11::products::Product, crate::Error> {
    let path = crate::params::format_path("/v1/products", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "products")]
#[doc = "Attach a feature to a product"]
pub async fn create_product_feature(
    client: &crate::Client,
    product: &str,
    params: &stripe_types_03::products::CreateProductFeatureParams,
) -> Result<stripe_types_07::products::ProductFeature, crate::Error> {
    let path = crate::params::format_path("/v1/products/{product}/features", &[("product", product)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "products")]
#[doc = "Delete a product"]
pub async fn delete_product(
    client: &crate::Client,
    id: &str,
    params: &crate::params::Empty,
) -> Result<stripe_types_06::products::DeletedProduct, crate::Error> {
    let path = crate::params::format_path("/v1/products/{id}", &[("id", id)]);
    client.delete_form(&path, params).await
}

#[cfg(feature = "products")]
#[doc = "Remove a feature from a product"]
pub async fn delete_product_feature(
    client: &crate::Client,
    product: &str,
    id: &str,
    params: &crate::params::Empty,
) -> Result<stripe_types_06::products::DeletedProductFeature, crate::Error> {
    let path = crate::params::format_path("/v1/products/{product}/features/{id}", &[("product", product), ("id", id)]);
    client.delete_form(&path, params).await
}

#[cfg(feature = "products")]
#[doc = "Retrieve a product"]
pub async fn get_product(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_01::products::GetProductParams,
) -> Result<stripe_types_11::products::Product, crate::Error> {
    let path = crate::params::format_path("/v1/products/{id}", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "products")]
#[doc = "Retrieve a product_feature"]
pub async fn get_product_feature(
    client: &crate::Client,
    product: &str,
    id: &str,
    params: &stripe_types_05::products::GetProductFeatureParams,
) -> Result<stripe_types_07::products::ProductFeature, crate::Error> {
    let path = crate::params::format_path("/v1/products/{product}/features/{id}", &[("product", product), ("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "products")]
#[doc = "List all features attached to a product"]
pub async fn list_product_features(
    client: &crate::Client,
    product: &str,
    params: &stripe_types_00::products::ListProductFeaturesParams,
) -> Result<crate::List<stripe_types_07::products::ProductFeature>, crate::Error> {
    let path = crate::params::format_path("/v1/products/{product}/features", &[("product", product)]);
    client.get(&path, params).await
}

#[cfg(feature = "products")]
#[doc = "List all products"]
pub async fn list_products(
    client: &crate::Client,
    params: &stripe_types_06::products::ListProductsParams,
) -> Result<crate::List<stripe_types_11::products::Product>, crate::Error> {
    let path = crate::params::format_path("/v1/products", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "products")]
#[doc = "Search products"]
pub async fn search_products(
    client: &crate::Client,
    params: &stripe_types_01::products::SearchProductsParams,
) -> Result<crate::List<stripe_types_11::products::Product>, crate::Error> {
    let path = crate::params::format_path("/v1/products/search", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "products")]
#[doc = "Update a product"]
pub async fn update_product(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_08::products::UpdateProductParams,
) -> Result<stripe_types_11::products::Product, crate::Error> {
    let path = crate::params::format_path("/v1/products/{id}", &[("id", id)]);
    client.post_form(&path, params).await
}
