#[cfg(feature = "subscription_items")]
pub type DeleteSubscriptionItemParamsProrationBehavior =
    stripe_types_00::subscription_items::CreateSubscriptionItemParamsProrationBehavior;

#[cfg(feature = "subscription_items")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct DeletedSubscriptionItem {
    #[doc = "Always true for a deleted object"]
    #[json(default)]
    pub deleted: bool,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::subscription_items::SubscriptionItemObject,
}

#[cfg(feature = "subscription_items")]
impl crate::PagedItem for DeletedSubscriptionItem {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "subscription_items")]
pub type UpdateSubscriptionItemParamsPaymentBehavior =
    stripe_types_03::subscription_items::CreateSubscriptionItemParamsPaymentBehavior;

#[cfg(feature = "subscription_items")]
pub type UpdateSubscriptionItemParamsPriceDataRecurringInterval =
    stripe_types_03::subscription_items::CreateSubscriptionItemParamsPriceDataRecurringInterval;

#[cfg(feature = "subscription_items")]
pub type UpdateSubscriptionItemParamsPriceDataTaxBehavior =
    stripe_types_00::subscription_items::CreateSubscriptionItemParamsPriceDataTaxBehavior;

#[cfg(feature = "subscription_items")]
pub type UpdateSubscriptionItemParamsProrationBehavior =
    stripe_types_00::subscription_items::CreateSubscriptionItemParamsProrationBehavior;
