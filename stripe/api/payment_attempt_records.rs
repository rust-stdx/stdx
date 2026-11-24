#[cfg(feature = "payment_attempt_records")]
#[doc = "Retrieve a Payment Attempt Record"]
pub async fn get_payment_attempt_record(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_01::payment_attempt_records::GetPaymentAttemptRecordParams,
) -> Result<stripe_types_15::payment_attempt_records::PaymentAttemptRecord, crate::Error> {
    let path = crate::params::format_path("/v1/payment_attempt_records/{id}", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "payment_attempt_records")]
#[doc = "List Payment Attempt Records"]
pub async fn list_payment_attempt_records(
    client: &crate::Client,
    params: &stripe_types_03::payment_attempt_records::ListPaymentAttemptRecordsParams,
) -> Result<crate::List<stripe_types_15::payment_attempt_records::PaymentAttemptRecord>, crate::Error> {
    let path = crate::params::format_path("/v1/payment_attempt_records", &[]);
    client.get(&path, params).await
}
