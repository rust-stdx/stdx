#[cfg(feature = "payment_method_domains")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentMethodDomain {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amazon_pay: Option<stripe_types_06::payment_method_domains::PaymentMethodDomainResourcePaymentMethodStatus>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<stripe_types_06::payment_method_domains::PaymentMethodDomainResourcePaymentMethodStatus>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "The domain name that this payment method domain object represents."]
    #[json(default)]
    pub domain_name: String,
    #[doc = "Whether this payment method domain is enabled."]
    #[json(default)]
    pub enabled: bool,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<stripe_types_06::payment_method_domains::PaymentMethodDomainResourcePaymentMethodStatus>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub klarna: Option<stripe_types_06::payment_method_domains::PaymentMethodDomainResourcePaymentMethodStatus>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<stripe_types_06::payment_method_domains::PaymentMethodDomainResourcePaymentMethodStatus>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::payment_method_domains::PaymentMethodDomainObject,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paypal: Option<stripe_types_06::payment_method_domains::PaymentMethodDomainResourcePaymentMethodStatus>,
}

#[cfg(feature = "payment_method_domains")]
impl crate::PagedItem for PaymentMethodDomain {
    fn id(&self) -> &str {
        &self.id
    }
}
