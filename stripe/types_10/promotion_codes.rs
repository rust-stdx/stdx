#[cfg(feature = "promotion_codes")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePromotionCodeParams {
    #[doc = "Whether the promotion code is currently active."]
    pub active: Option<bool>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Settings that restrict the redemption of the promotion code."]
    pub restrictions: Option<stripe_types_09::promotion_codes::UpdatePromotionCodeParamsRestrictions>,
}

#[cfg(feature = "promotion_codes")]
impl crate::params::FormEncode for UpdatePromotionCodeParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("active", self.active.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("restrictions", self.restrictions.as_ref());
    }
}
