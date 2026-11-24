#[cfg(feature = "subscription_items")]
#[derive(Debug, Clone)]
pub struct CreateSubscriptionItemParamsPriceData {
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "The ID of the [Product](https://docs.stripe.com/api/products) that this [Price](https://docs.stripe.com/api/prices) w..."]
    pub product: String,
    #[doc = "The recurring components of a price such as `interval` and `interval_count`."]
    pub recurring: stripe_types_06::subscription_items::CreateSubscriptionItemParamsPriceDataRecurring,
    #[doc = "Only required if a default tax behavior..."]
    pub tax_behavior: Option<stripe_types_00::subscription_items::CreateSubscriptionItemParamsPriceDataTaxBehavior>,
    #[doc = "A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge."]
    pub unit_amount: Option<i64>,
    #[doc = "Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places."]
    pub unit_amount_decimal: Option<String>,
}

#[cfg(feature = "subscription_items")]
impl crate::params::FormEncode for CreateSubscriptionItemParamsPriceData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("currency", &self.currency);
        form.field("product", &self.product);
        form.field("recurring", &self.recurring);
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
        form.field_opt("unit_amount", self.unit_amount.as_ref());
        form.field_opt("unit_amount_decimal", self.unit_amount_decimal.as_ref());
    }
}

#[cfg(feature = "subscription_items")]
#[derive(Debug, Clone, Default)]
pub struct DeleteSubscriptionItemParams {
    #[doc = "Delete all usage for the given subscription item."]
    pub clear_usage: Option<bool>,
    #[doc = "Controls how Stripe handles payment when a subscription update requires payment and `collection_method=charge_automat..."]
    pub payment_behavior: Option<stripe_types_06::subscription_items::DeleteSubscriptionItemParamsPaymentBehavior>,
    #[doc = "Determines how to handle [prorations](https://docs.stripe.com/billing/subscriptions/prorations) when the billing cycl..."]
    pub proration_behavior: Option<stripe_types_05::subscription_items::DeleteSubscriptionItemParamsProrationBehavior>,
    #[doc = "If set, the proration will be calculated as though the subscription was updated at the given time."]
    pub proration_date: Option<i64>,
}

#[cfg(feature = "subscription_items")]
impl crate::params::FormEncode for DeleteSubscriptionItemParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("clear_usage", self.clear_usage.as_ref());
        form.field_opt("payment_behavior", self.payment_behavior.as_ref());
        form.field_opt("proration_behavior", self.proration_behavior.as_ref());
        form.field_opt("proration_date", self.proration_date.as_ref());
    }
}
