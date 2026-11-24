#[cfg(feature = "promotion_codes")]
#[derive(Debug, Clone)]
pub struct CreatePromotionCodeParams {
    #[doc = "Whether the promotion code is currently active."]
    pub active: Option<bool>,
    #[doc = "The customer-facing code."]
    pub code: Option<String>,
    #[doc = "The customer who can use this promotion code."]
    pub customer: Option<String>,
    #[doc = "The account representing the customer who can use this promotion code."]
    pub customer_account: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The timestamp at which this promotion code will expire."]
    pub expires_at: Option<i64>,
    #[doc = "A positive integer specifying the number of times the promotion code can be redeemed."]
    pub max_redemptions: Option<i64>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The promotion referenced by this promotion code."]
    pub promotion: stripe_types_06::promotion_codes::CreatePromotionCodeParamsPromotion,
    #[doc = "Settings that restrict the redemption of the promotion code."]
    pub restrictions: Option<stripe_types_05::promotion_codes::CreatePromotionCodeParamsRestrictions>,
}

#[cfg(feature = "promotion_codes")]
impl crate::params::FormEncode for CreatePromotionCodeParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("active", self.active.as_ref());
        form.field_opt("code", self.code.as_ref());
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("customer_account", self.customer_account.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("expires_at", self.expires_at.as_ref());
        form.field_opt("max_redemptions", self.max_redemptions.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field("promotion", &self.promotion);
        form.field_opt("restrictions", self.restrictions.as_ref());
    }
}

#[cfg(feature = "promotion_codes")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePromotionCodeParamsRestrictions {
    #[doc = "Promotion codes defined in each available currency option."]
    pub currency_options: Option<
        std::collections::HashMap<
            String,
            stripe_types_06::promotion_codes::UpdatePromotionCodeParamsRestrictionsCurrencyOptionsValue,
        >,
    >,
}

#[cfg(feature = "promotion_codes")]
impl crate::params::FormEncode for UpdatePromotionCodeParamsRestrictions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.currency_options.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("currency_options", key, value);
            }
        }
    }
}
