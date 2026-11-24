#[cfg(feature = "payment_method_domains")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentMethodDomainResourcePaymentMethodStatusDetails {
    #[doc = "The error message associated with the status of the payment method on the domain."]
    #[json(default)]
    pub error_message: String,
}
