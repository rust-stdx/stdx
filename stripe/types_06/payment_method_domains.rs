#[cfg(feature = "payment_method_domains")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentMethodDomainResourcePaymentMethodStatus {
    #[doc = "The status of the payment method on the domain."]
    #[json(default)]
    pub status: stripe_types_00::payment_method_domains::PaymentMethodDomainResourcePaymentMethodStatusStatus,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub status_details:
        Option<stripe_types_02::payment_method_domains::PaymentMethodDomainResourcePaymentMethodStatusDetails>,
}
