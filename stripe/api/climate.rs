#[cfg(feature = "climate")]
#[doc = "Cancel an order"]
pub async fn cancel_climate_order(
    client: &crate::Client,
    order: &str,
    params: &stripe_types_06::climate::CancelClimateOrderParams,
) -> Result<stripe_types_10::climate::ClimateOrder, crate::Error> {
    let path = crate::params::format_path("/v1/climate/orders/{order}/cancel", &[("order", order)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "climate")]
#[doc = "Create an order"]
pub async fn create_climate_order(
    client: &crate::Client,
    params: &stripe_types_06::climate::CreateClimateOrderParams,
) -> Result<stripe_types_10::climate::ClimateOrder, crate::Error> {
    let path = crate::params::format_path("/v1/climate/orders", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "climate")]
#[doc = "Retrieve an order"]
pub async fn get_climate_order(
    client: &crate::Client,
    order: &str,
    params: &stripe_types_04::climate::GetClimateOrderParams,
) -> Result<stripe_types_10::climate::ClimateOrder, crate::Error> {
    let path = crate::params::format_path("/v1/climate/orders/{order}", &[("order", order)]);
    client.get(&path, params).await
}

#[cfg(feature = "climate")]
#[doc = "Retrieve a product"]
pub async fn get_climate_product(
    client: &crate::Client,
    product: &str,
    params: &stripe_types_05::climate::GetClimateProductParams,
) -> Result<stripe_types_07::climate::ClimateProduct, crate::Error> {
    let path = crate::params::format_path("/v1/climate/products/{product}", &[("product", product)]);
    client.get(&path, params).await
}

#[cfg(feature = "climate")]
#[doc = "Retrieve a supplier"]
pub async fn get_climate_supplier(
    client: &crate::Client,
    supplier: &str,
    params: &stripe_types_06::climate::GetClimateSupplierParams,
) -> Result<stripe_types_06::climate::ClimateSupplier, crate::Error> {
    let path = crate::params::format_path("/v1/climate/suppliers/{supplier}", &[("supplier", supplier)]);
    client.get(&path, params).await
}

#[cfg(feature = "climate")]
#[doc = "List orders"]
pub async fn list_climate_orders(
    client: &crate::Client,
    params: &stripe_types_02::climate::ListClimateOrdersParams,
) -> Result<crate::List<stripe_types_10::climate::ClimateOrder>, crate::Error> {
    let path = crate::params::format_path("/v1/climate/orders", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "climate")]
#[doc = "List products"]
pub async fn list_climate_products(
    client: &crate::Client,
    params: &stripe_types_06::climate::ListClimateProductsParams,
) -> Result<crate::List<stripe_types_07::climate::ClimateProduct>, crate::Error> {
    let path = crate::params::format_path("/v1/climate/products", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "climate")]
#[doc = "List suppliers"]
pub async fn list_climate_suppliers(
    client: &crate::Client,
    params: &stripe_types_05::climate::ListClimateSuppliersParams,
) -> Result<crate::List<stripe_types_06::climate::ClimateSupplier>, crate::Error> {
    let path = crate::params::format_path("/v1/climate/suppliers", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "climate")]
#[doc = "Update an order"]
pub async fn update_climate_order(
    client: &crate::Client,
    order: &str,
    params: &stripe_types_06::climate::UpdateClimateOrderParams,
) -> Result<stripe_types_10::climate::ClimateOrder, crate::Error> {
    let path = crate::params::format_path("/v1/climate/orders/{order}", &[("order", order)]);
    client.post_form(&path, params).await
}
