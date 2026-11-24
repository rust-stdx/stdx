#[cfg(feature = "promotion_codes")]
#[derive(Debug, Clone, Default)]
pub struct CreatePromotionCodeParamsRestrictions {
    #[doc = "Promotion codes defined in each available currency option."]
    pub currency_options: Option<
        std::collections::HashMap<
            String,
            stripe_types_01::promotion_codes::CreatePromotionCodeParamsRestrictionsCurrencyOptionsValue,
        >,
    >,
    #[doc = "A Boolean indicating if the Promotion Code should only be redeemed for Customers without any successful payments or i..."]
    pub first_time_transaction: Option<bool>,
    #[doc = "Minimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work)."]
    pub minimum_amount: Option<i64>,
    #[doc = "Three-letter [ISO code](https://stripe.com/docs/currencies) for minimum_amount"]
    pub minimum_amount_currency: Option<String>,
}

#[cfg(feature = "promotion_codes")]
impl crate::params::FormEncode for CreatePromotionCodeParamsRestrictions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.currency_options.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("currency_options", key, value);
            }
        }
        form.field_opt("first_time_transaction", self.first_time_transaction.as_ref());
        form.field_opt("minimum_amount", self.minimum_amount.as_ref());
        form.field_opt("minimum_amount_currency", self.minimum_amount_currency.as_ref());
    }
}

#[cfg(feature = "promotion_codes")]
#[derive(Debug, Clone, Default)]
pub struct ListPromotionCodesParams {
    pub active: Option<bool>,
    pub code: Option<String>,
    pub coupon: Option<String>,
    pub created: Option<stripe_types_01::promotion_codes::ListPromotionCodesParamsCreated>,
    pub customer: Option<String>,
    pub customer_account: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "promotion_codes")]
impl crate::params::FormEncode for ListPromotionCodesParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("active", self.active.as_ref());
        form.field_opt("code", self.code.as_ref());
        form.field_opt("coupon", self.coupon.as_ref());
        form.field_opt("created", self.created.as_ref());
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("customer_account", self.customer_account.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}
