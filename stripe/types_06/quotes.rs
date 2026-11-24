#[cfg(feature = "quotes")]
pub type CancelQuoteParams = stripe_types_04::quotes::GetQuoteParams;

#[cfg(feature = "quotes")]
#[derive(Debug, Clone, Default)]
pub struct CreateQuoteParamsSubscriptionDataBillingModeFlexible {
    #[doc = "Controls how invoices and invoice items display proration amounts and discount amounts."]
    pub proration_discounts:
        Option<stripe_types_02::quotes::CreateQuoteParamsSubscriptionDataBillingModeFlexibleProrationDiscounts>,
}

#[cfg(feature = "quotes")]
impl crate::params::FormEncode for CreateQuoteParamsSubscriptionDataBillingModeFlexible {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("proration_discounts", self.proration_discounts.as_ref());
    }
}

#[cfg(feature = "quotes")]
pub type ListQuoteLineItemsParams = stripe_types_01::quotes::ListQuoteComputedUpfrontLineItemsParams;

#[cfg(feature = "quotes")]
pub type UpdateQuoteParamsAutomaticTaxLiabilityType =
    stripe_types_00::quotes::CreateQuoteParamsAutomaticTaxLiabilityType;

#[cfg(feature = "quotes")]
pub type UpdateQuoteParamsCollectionMethod = stripe_types_01::quotes::CreateQuoteParamsCollectionMethod;

#[cfg(feature = "quotes")]
pub type UpdateQuoteParamsLineItemsItemPriceDataRecurringInterval =
    stripe_types_01::quotes::CreateQuoteParamsLineItemsItemPriceDataRecurringInterval;

#[cfg(feature = "quotes")]
pub type UpdateQuoteParamsLineItemsItemPriceDataTaxBehavior =
    stripe_types_04::quotes::CreateQuoteParamsLineItemsItemPriceDataTaxBehavior;
