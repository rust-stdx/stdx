#[cfg(feature = "payment_method_domains")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodDomainParams {
    #[doc = "The domain name that this payment method domain object represents."]
    pub domain_name: String,
    #[doc = "Whether this payment method domain is enabled."]
    pub enabled: Option<bool>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "payment_method_domains")]
impl crate::params::FormEncode for CreatePaymentMethodDomainParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("domain_name", &self.domain_name);
        form.field_opt("enabled", self.enabled.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}

#[cfg(feature = "payment_method_domains")]
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentMethodDomainObject {
    PaymentMethodDomain,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_method_domains")]
impl PaymentMethodDomainObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentMethodDomainObject::PaymentMethodDomain => "payment_method_domain",
            PaymentMethodDomainObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_method_domains")]
impl Default for PaymentMethodDomainObject {
    fn default() -> Self {
        PaymentMethodDomainObject::Unknown(String::new())
    }
}

#[cfg(feature = "payment_method_domains")]
impl<'json> json::Decode<'json> for PaymentMethodDomainObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentMethodDomainObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "payment_method_domain" => PaymentMethodDomainObject::PaymentMethodDomain,
            other => PaymentMethodDomainObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_method_domains")]
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentMethodDomainResourcePaymentMethodStatusStatus {
    Active,
    Inactive,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_method_domains")]
impl PaymentMethodDomainResourcePaymentMethodStatusStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentMethodDomainResourcePaymentMethodStatusStatus::Active => "active",
            PaymentMethodDomainResourcePaymentMethodStatusStatus::Inactive => "inactive",
            PaymentMethodDomainResourcePaymentMethodStatusStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_method_domains")]
impl Default for PaymentMethodDomainResourcePaymentMethodStatusStatus {
    fn default() -> Self {
        PaymentMethodDomainResourcePaymentMethodStatusStatus::Unknown(String::new())
    }
}

#[cfg(feature = "payment_method_domains")]
impl<'json> json::Decode<'json> for PaymentMethodDomainResourcePaymentMethodStatusStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentMethodDomainResourcePaymentMethodStatusStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "active" => PaymentMethodDomainResourcePaymentMethodStatusStatus::Active,
            "inactive" => PaymentMethodDomainResourcePaymentMethodStatusStatus::Inactive,
            other => PaymentMethodDomainResourcePaymentMethodStatusStatus::Unknown(other.to_string()),
        })
    }
}
