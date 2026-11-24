#[cfg(feature = "v2_commerce")]
#[doc = "Create a Product Catalog Import"]
pub async fn create_commerce_product_catalog_import(
    client: &crate::Client,
    params: &stripe_types_06::v2_commerce::CreateCommerceProductCatalogImportParams,
) -> Result<stripe_types_12::v2_commerce::V2CommerceProductCatalogImport, crate::Error> {
    let path = crate::params::format_path("/v2/commerce/product_catalog/imports", &[]);
    client.post_json(&path, params).await
}

#[cfg(feature = "v2_commerce")]
#[doc = "Retrieve a Product Catalog Import"]
pub async fn get_commerce_product_catalog_import(
    client: &crate::Client,
    id: &str,
    params: &crate::params::Empty,
) -> Result<stripe_types_12::v2_commerce::V2CommerceProductCatalogImport, crate::Error> {
    let path = crate::params::format_path("/v2/commerce/product_catalog/imports/{id}", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "v2_commerce")]
#[doc = "List Product Catalog Imports"]
pub async fn list_commerce_product_catalog_imports(
    client: &crate::Client,
    params: &stripe_types_05::v2_commerce::ListCommerceProductCatalogImportsParams,
) -> Result<crate::List<stripe_types_12::v2_commerce::V2CommerceProductCatalogImport>, crate::Error> {
    let path = crate::params::format_path("/v2/commerce/product_catalog/imports", &[]);
    client.get(&path, params).await
}
