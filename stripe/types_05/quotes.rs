#[cfg(feature = "quotes")]
pub type AcceptQuoteParams = stripe_types_04::quotes::GetQuoteParams;

#[cfg(feature = "quotes")]
#[derive(Debug, Clone)]
pub struct CreateQuoteParamsAutomaticTaxLiability {
    #[doc = "The connected account being referenced when `type` is `account`."]
    pub account: Option<String>,
    #[doc = "Type of the account referenced in the request."]
    pub r#type: stripe_types_00::quotes::CreateQuoteParamsAutomaticTaxLiabilityType,
}

#[cfg(feature = "quotes")]
impl crate::params::FormEncode for CreateQuoteParamsAutomaticTaxLiability {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "quotes")]
pub type CreateQuoteParamsInvoiceSettingsIssuerType =
    stripe_types_00::quotes::CreateQuoteParamsAutomaticTaxLiabilityType;

#[cfg(feature = "quotes")]
#[derive(Debug, Clone)]
pub struct CreateQuoteParamsLineItemsItemPriceDataRecurring {
    #[doc = "Specifies billing frequency."]
    pub interval: stripe_types_01::quotes::CreateQuoteParamsLineItemsItemPriceDataRecurringInterval,
    #[doc = "The number of intervals between subscription billings."]
    pub interval_count: Option<i64>,
}

#[cfg(feature = "quotes")]
impl crate::params::FormEncode for CreateQuoteParamsLineItemsItemPriceDataRecurring {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("interval", &self.interval);
        form.field_opt("interval_count", self.interval_count.as_ref());
    }
}

#[cfg(feature = "quotes")]
pub type GetQuotePdfParams = stripe_types_04::quotes::GetQuoteParams;

#[cfg(feature = "quotes")]
#[derive(Debug, Clone, Default)]
pub struct ListQuotesParams {
    pub customer: Option<String>,
    pub customer_account: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<stripe_types_03::quotes::ListQuotesParamsStatus>,
    pub test_clock: Option<String>,
}

#[cfg(feature = "quotes")]
impl crate::params::FormEncode for ListQuotesParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("customer_account", self.customer_account.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("status", self.status.as_ref());
        form.field_opt("test_clock", self.test_clock.as_ref());
    }
}

#[cfg(feature = "quotes")]
pub type UpdateQuoteParamsInvoiceSettingsIssuerType =
    stripe_types_00::quotes::CreateQuoteParamsAutomaticTaxLiabilityType;

#[cfg(feature = "quotes")]
pub type UpdateQuoteParamsTransferData = stripe_types_01::quotes::CreateQuoteParamsTransferData;
