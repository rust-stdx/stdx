#[cfg(feature = "coupons")]
#[derive(Debug, Clone, Default)]
pub struct UpdateCouponParams {
    #[doc = "Coupons defined in each available currency option (only supported if the coupon is amount-based)."]
    pub currency_options:
        Option<std::collections::HashMap<String, stripe_types_06::coupons::UpdateCouponParamsCurrencyOptionsValue>>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Name of the coupon displayed to customers on, for instance invoices, or receipts."]
    pub name: Option<String>,
}

#[cfg(feature = "coupons")]
impl crate::params::FormEncode for UpdateCouponParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.currency_options.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("currency_options", key, value);
            }
        }
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("name", self.name.as_ref());
    }
}
