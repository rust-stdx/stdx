#[cfg(feature = "payment_records")]
#[derive(Debug, Clone, Default)]
pub struct ReportPaymentAttemptInformationalParamsShippingDetails {
    #[doc = "The physical shipping address."]
    pub address:
        Option<stripe_types_05::payment_records::ReportPaymentAttemptInformationalParamsShippingDetailsAddress>,
    #[doc = "The shipping recipient's name."]
    pub name: Option<String>,
    #[doc = "The shipping recipient's phone number."]
    pub phone: Option<String>,
}

#[cfg(feature = "payment_records")]
impl crate::params::FormEncode for ReportPaymentAttemptInformationalParamsShippingDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("address", self.address.as_ref());
        form.field_opt("name", self.name.as_ref());
        form.field_opt("phone", self.phone.as_ref());
    }
}

#[cfg(feature = "payment_records")]
#[derive(Debug, Clone, Default)]
pub struct ReportPaymentAttemptParamsPaymentMethodDetailsBillingDetails {
    #[doc = "The billing address associated with the method of payment."]
    pub address:
        Option<stripe_types_06::payment_records::ReportPaymentAttemptParamsPaymentMethodDetailsBillingDetailsAddress>,
    #[doc = "The billing email associated with the method of payment."]
    pub email: Option<String>,
    #[doc = "The billing name associated with the method of payment."]
    pub name: Option<String>,
    #[doc = "The billing phone number associated with the method of payment."]
    pub phone: Option<String>,
}

#[cfg(feature = "payment_records")]
impl crate::params::FormEncode for ReportPaymentAttemptParamsPaymentMethodDetailsBillingDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("address", self.address.as_ref());
        form.field_opt("email", self.email.as_ref());
        form.field_opt("name", self.name.as_ref());
        form.field_opt("phone", self.phone.as_ref());
    }
}

#[cfg(feature = "payment_records")]
#[derive(Debug, Clone, Default)]
pub struct ReportPaymentAttemptParamsShippingDetails {
    #[doc = "The physical shipping address."]
    pub address: Option<stripe_types_05::payment_records::ReportPaymentAttemptParamsShippingDetailsAddress>,
    #[doc = "The shipping recipient's name."]
    pub name: Option<String>,
    #[doc = "The shipping recipient's phone number."]
    pub phone: Option<String>,
}

#[cfg(feature = "payment_records")]
impl crate::params::FormEncode for ReportPaymentAttemptParamsShippingDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("address", self.address.as_ref());
        form.field_opt("name", self.name.as_ref());
        form.field_opt("phone", self.phone.as_ref());
    }
}

#[cfg(feature = "payment_records")]
#[derive(Debug, Clone, Default)]
pub struct ReportPaymentParamsPaymentMethodDetails {
    #[doc = "The billing details associated with the method of payment."]
    pub billing_details:
        Option<stripe_types_06::payment_records::ReportPaymentParamsPaymentMethodDetailsBillingDetails>,
    #[doc = "Information about the custom (user-defined) payment method used to make this payment."]
    pub custom: Option<stripe_types_01::payment_records::ReportPaymentParamsPaymentMethodDetailsCustom>,
    #[doc = "ID of the Stripe Payment Method used to make this payment."]
    pub payment_method: Option<String>,
    #[doc = "The type of the payment method details."]
    pub r#type: Option<stripe_types_01::payment_records::ReportPaymentParamsPaymentMethodDetailsType>,
}

#[cfg(feature = "payment_records")]
impl crate::params::FormEncode for ReportPaymentParamsPaymentMethodDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("billing_details", self.billing_details.as_ref());
        form.field_opt("custom", self.custom.as_ref());
        form.field_opt("payment_method", self.payment_method.as_ref());
        form.field_opt("type", self.r#type.as_ref());
    }
}
