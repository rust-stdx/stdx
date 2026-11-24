#[cfg(feature = "sources")]
#[derive(Debug, Clone)]
pub struct UpdateSourceParamsSourceOrderShipping {
    #[doc = "Shipping address."]
    pub address: stripe_types_05::sources::UpdateSourceParamsSourceOrderShippingAddress,
    #[doc = "The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc."]
    pub carrier: Option<String>,
    #[doc = "Recipient name."]
    pub name: Option<String>,
    #[doc = "Recipient phone (including extension)."]
    pub phone: Option<String>,
    #[doc = "The tracking number for a physical product, obtained from the delivery service."]
    pub tracking_number: Option<String>,
}

#[cfg(feature = "sources")]
impl crate::params::FormEncode for UpdateSourceParamsSourceOrderShipping {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("address", &self.address);
        form.field_opt("carrier", self.carrier.as_ref());
        form.field_opt("name", self.name.as_ref());
        form.field_opt("phone", self.phone.as_ref());
        form.field_opt("tracking_number", self.tracking_number.as_ref());
    }
}
