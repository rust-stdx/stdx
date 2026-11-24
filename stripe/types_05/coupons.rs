#[cfg(feature = "coupons")]
#[derive(Debug, Clone, Default)]
pub struct CreateCouponParams {
    #[doc = "A positive integer representing the amount to subtract from an invoice total (required if `percent_off` is not passed)."]
    pub amount_off: Option<i64>,
    #[doc = "A hash containing directions for what this Coupon will apply discounts to."]
    pub applies_to: Option<stripe_types_02::coupons::CreateCouponParamsAppliesTo>,
    #[doc = "Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) of the `amount_off` parameter (required ..."]
    pub currency: Option<String>,
    #[doc = "Coupons defined in each available currency option (only supported if `amount_off` is passed)."]
    pub currency_options:
        Option<std::collections::HashMap<String, stripe_types_03::coupons::CreateCouponParamsCurrencyOptionsValue>>,
    #[doc = "Specifies how long the discount will be in effect if used on a subscription."]
    pub duration: Option<stripe_types_03::coupons::CreateCouponParamsDuration>,
    #[doc = "Required only if `duration` is `repeating`, in which case it must be a positive integer that specifies the number of ..."]
    pub duration_in_months: Option<i64>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Unique string of your choice that will be used to identify this coupon when applying it to a customer."]
    pub id: Option<String>,
    #[doc = "A positive integer specifying the number of times the coupon can be redeemed before it's no longer valid."]
    pub max_redemptions: Option<i64>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Name of the coupon displayed to customers on, for instance invoices, or receipts."]
    pub name: Option<String>,
    #[doc = "A positive float larger than 0, and smaller or equal to 100, that represents the discount the coupon will apply (requ..."]
    pub percent_off: Option<f64>,
    #[doc = "Unix timestamp specifying the last time at which the coupon can be redeemed (cannot be set to more than 5 years in th..."]
    pub redeem_by: Option<i64>,
}

#[cfg(feature = "coupons")]
impl crate::params::FormEncode for CreateCouponParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount_off", self.amount_off.as_ref());
        form.field_opt("applies_to", self.applies_to.as_ref());
        form.field_opt("currency", self.currency.as_ref());
        if let Some(values) = self.currency_options.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("currency_options", key, value);
            }
        }
        form.field_opt("duration", self.duration.as_ref());
        form.field_opt("duration_in_months", self.duration_in_months.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("id", self.id.as_ref());
        form.field_opt("max_redemptions", self.max_redemptions.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("name", self.name.as_ref());
        form.field_opt("percent_off", self.percent_off.as_ref());
        form.field_opt("redeem_by", self.redeem_by.as_ref());
    }
}
