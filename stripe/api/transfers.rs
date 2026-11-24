#[cfg(feature = "transfers")]
#[doc = "Create a transfer"]
pub async fn create_transfer(
    client: &crate::Client,
    params: &stripe_types_06::transfers::CreateTransferParams,
) -> Result<stripe_types_14::transfers::Transfer, crate::Error> {
    let path = crate::params::format_path("/v1/transfers", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "transfers")]
#[doc = "Create a transfer reversal"]
pub async fn create_transfer_reversal(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_04::transfers::CreateTransferReversalParams,
) -> Result<stripe_types_14::transfers::TransferReversal, crate::Error> {
    let path = crate::params::format_path("/v1/transfers/{id}/reversals", &[("id", id)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "transfers")]
#[doc = "Retrieve a transfer"]
pub async fn get_transfer(
    client: &crate::Client,
    transfer: &str,
    params: &stripe_types_02::transfers::GetTransferParams,
) -> Result<stripe_types_14::transfers::Transfer, crate::Error> {
    let path = crate::params::format_path("/v1/transfers/{transfer}", &[("transfer", transfer)]);
    client.get(&path, params).await
}

#[cfg(feature = "transfers")]
#[doc = "Retrieve a reversal"]
pub async fn get_transfer_reversal(
    client: &crate::Client,
    transfer: &str,
    id: &str,
    params: &stripe_types_05::transfers::GetTransferReversalParams,
) -> Result<stripe_types_14::transfers::TransferReversal, crate::Error> {
    let path =
        crate::params::format_path("/v1/transfers/{transfer}/reversals/{id}", &[("transfer", transfer), ("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "transfers")]
#[doc = "List all reversals"]
pub async fn list_transfer_reversals(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_04::transfers::ListTransferReversalsParams,
) -> Result<crate::List<stripe_types_14::transfers::TransferReversal>, crate::Error> {
    let path = crate::params::format_path("/v1/transfers/{id}/reversals", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "transfers")]
#[doc = "List all transfers"]
pub async fn list_transfers(
    client: &crate::Client,
    params: &stripe_types_05::transfers::ListTransfersParams,
) -> Result<crate::List<stripe_types_14::transfers::Transfer>, crate::Error> {
    let path = crate::params::format_path("/v1/transfers", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "transfers")]
#[doc = "Update a transfer"]
pub async fn update_transfer(
    client: &crate::Client,
    transfer: &str,
    params: &stripe_types_03::transfers::UpdateTransferParams,
) -> Result<stripe_types_14::transfers::Transfer, crate::Error> {
    let path = crate::params::format_path("/v1/transfers/{transfer}", &[("transfer", transfer)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "transfers")]
#[doc = "Update a reversal"]
pub async fn update_transfer_reversal(
    client: &crate::Client,
    transfer: &str,
    id: &str,
    params: &stripe_types_02::transfers::UpdateTransferReversalParams,
) -> Result<stripe_types_14::transfers::TransferReversal, crate::Error> {
    let path =
        crate::params::format_path("/v1/transfers/{transfer}/reversals/{id}", &[("transfer", transfer), ("id", id)]);
    client.post_form(&path, params).await
}
