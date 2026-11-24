#[cfg(feature = "quotes")]
#[derive(Debug, Clone, Default)]
pub struct CreateQuoteParamsAutomaticTax {
    #[doc = "Controls whether Stripe will automatically compute tax on the resulting invoices or subscriptions as well as the quot..."]
    pub enabled: bool,
    #[doc = "The account that's liable for tax."]
    pub liability: Option<stripe_types_05::quotes::CreateQuoteParamsAutomaticTaxLiability>,
}

#[cfg(feature = "quotes")]
impl crate::params::FormEncode for CreateQuoteParamsAutomaticTax {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("liability", self.liability.as_ref());
    }
}

#[cfg(feature = "quotes")]
#[derive(Debug, Clone)]
pub struct CreateQuoteParamsInvoiceSettingsIssuer {
    #[doc = "The connected account being referenced when `type` is `account`."]
    pub account: Option<String>,
    #[doc = "Type of the account referenced in the request."]
    pub r#type: stripe_types_05::quotes::CreateQuoteParamsInvoiceSettingsIssuerType,
}

#[cfg(feature = "quotes")]
impl crate::params::FormEncode for CreateQuoteParamsInvoiceSettingsIssuer {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "quotes")]
#[derive(Debug, Clone)]
pub struct CreateQuoteParamsSubscriptionDataBillingMode {
    #[doc = "Configure behavior for flexible billing mode."]
    pub flexible: Option<stripe_types_06::quotes::CreateQuoteParamsSubscriptionDataBillingModeFlexible>,
    #[doc = "Controls the calculation and orchestration of prorations and invoices for subscriptions."]
    pub r#type: stripe_types_04::quotes::CreateQuoteParamsSubscriptionDataBillingModeType,
}

#[cfg(feature = "quotes")]
impl crate::params::FormEncode for CreateQuoteParamsSubscriptionDataBillingMode {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("flexible", self.flexible.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(any(feature = "events", feature = "quotes"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct QuotesResourceSubscriptionDataBillingMode {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub flexible: Option<stripe_types_05::subscriptions::SubscriptionsResourceBillingModeFlexible>,
    #[doc = "Controls how prorations and invoices for subscriptions are calculated and orchestrated."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::quotes::QuotesResourceSubscriptionDataBillingModeType,
}
