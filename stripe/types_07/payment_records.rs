#[cfg(feature = "payment_records")]
#[derive(Debug, Clone, Default)]
pub struct ReportPaymentParamsShippingDetails {
    #[doc = "The physical shipping address."]
    pub address: Option<stripe_types_05::payment_records::ReportPaymentParamsShippingDetailsAddress>,
    #[doc = "The shipping recipient's name."]
    pub name: Option<String>,
    #[doc = "The shipping recipient's phone number."]
    pub phone: Option<String>,
}

#[cfg(feature = "payment_records")]
impl crate::params::FormEncode for ReportPaymentParamsShippingDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("address", self.address.as_ref());
        form.field_opt("name", self.name.as_ref());
        form.field_opt("phone", self.phone.as_ref());
    }
}

#[cfg(feature = "payment_records")]
#[derive(Debug, Clone)]
pub struct ReportRefundParamsProcessorDetails {
    #[doc = "Information about the custom processor used to make this refund."]
    pub custom: Option<stripe_types_01::payment_records::ReportRefundParamsProcessorDetailsCustom>,
    #[doc = "The type of the processor details."]
    pub r#type: stripe_types_05::payment_records::ReportRefundParamsProcessorDetailsType,
}

#[cfg(feature = "payment_records")]
impl crate::params::FormEncode for ReportRefundParamsProcessorDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("custom", self.custom.as_ref());
        form.field("type", &self.r#type);
    }
}
