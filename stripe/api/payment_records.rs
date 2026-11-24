#[cfg(feature = "payment_records")]
#[doc = "Retrieve a Payment Record"]
pub async fn get_payment_record(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_04::payment_records::GetPaymentRecordParams,
) -> Result<stripe_types_14::payment_records::PaymentRecord, crate::Error> {
    let path = crate::params::format_path("/v1/payment_records/{id}", &[("id", id)]);
    client.get(&path, params).await
}

#[cfg(feature = "payment_records")]
#[doc = "List Payment Records"]
pub async fn list_payment_records(
    client: &crate::Client,
    params: &stripe_types_02::payment_records::ListPaymentRecordsParams,
) -> Result<crate::List<stripe_types_14::payment_records::PaymentRecord>, crate::Error> {
    let path = crate::params::format_path("/v1/payment_records", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "payment_records")]
#[doc = "Report a payment"]
pub async fn report_payment(
    client: &crate::Client,
    params: &stripe_types_11::payment_records::ReportPaymentParams,
) -> Result<stripe_types_14::payment_records::PaymentRecord, crate::Error> {
    let path = crate::params::format_path("/v1/payment_records/report_payment", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "payment_records")]
#[doc = "Report a payment attempt"]
pub async fn report_payment_attempt(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_12::payment_records::ReportPaymentAttemptParams,
) -> Result<stripe_types_14::payment_records::PaymentRecord, crate::Error> {
    let path = crate::params::format_path("/v1/payment_records/{id}/report_payment_attempt", &[("id", id)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "payment_records")]
#[doc = "Report payment attempt canceled"]
pub async fn report_payment_attempt_canceled(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_03::payment_records::ReportPaymentAttemptCanceledParams,
) -> Result<stripe_types_14::payment_records::PaymentRecord, crate::Error> {
    let path = crate::params::format_path("/v1/payment_records/{id}/report_payment_attempt_canceled", &[("id", id)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "payment_records")]
#[doc = "Report payment attempt failed"]
pub async fn report_payment_attempt_failed(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_00::payment_records::ReportPaymentAttemptFailedParams,
) -> Result<stripe_types_14::payment_records::PaymentRecord, crate::Error> {
    let path = crate::params::format_path("/v1/payment_records/{id}/report_payment_attempt_failed", &[("id", id)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "payment_records")]
#[doc = "Report payment attempt guaranteed"]
pub async fn report_payment_attempt_guaranteed(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_04::payment_records::ReportPaymentAttemptGuaranteedParams,
) -> Result<stripe_types_14::payment_records::PaymentRecord, crate::Error> {
    let path = crate::params::format_path("/v1/payment_records/{id}/report_payment_attempt_guaranteed", &[("id", id)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "payment_records")]
#[doc = "Report payment attempt informational"]
pub async fn report_payment_attempt_informational(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_10::payment_records::ReportPaymentAttemptInformationalParams,
) -> Result<stripe_types_14::payment_records::PaymentRecord, crate::Error> {
    let path =
        crate::params::format_path("/v1/payment_records/{id}/report_payment_attempt_informational", &[("id", id)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "payment_records")]
#[doc = "Report a refund"]
pub async fn report_refund(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_11::payment_records::ReportRefundParams,
) -> Result<stripe_types_14::payment_records::PaymentRecord, crate::Error> {
    let path = crate::params::format_path("/v1/payment_records/{id}/report_refund", &[("id", id)]);
    client.post_form(&path, params).await
}
