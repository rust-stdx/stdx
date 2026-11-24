#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreateInvoiceParamsAutomaticTax {
    #[doc = "Whether Stripe automatically computes tax on this invoice."]
    pub enabled: bool,
    #[doc = "The account that's liable for tax."]
    pub liability: Option<stripe_types_05::invoices::CreateInvoiceParamsAutomaticTaxLiability>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsAutomaticTax {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("liability", self.liability.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone)]
pub struct CreateInvoiceParamsIssuer {
    #[doc = "The connected account being referenced when `type` is `account`."]
    pub account: Option<String>,
    #[doc = "Type of the account referenced in the request."]
    pub r#type: stripe_types_06::invoices::CreateInvoiceParamsIssuerType,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsIssuer {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallments {
    #[doc = "Setting to true enables installments for this invoice."]
    pub enabled: Option<bool>,
    #[doc = "The selected installment plan to use for this invoice."]
    pub plan:
        Option<stripe_types_06::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlan>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallments {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("enabled", self.enabled.as_ref());
        form.field_opt("plan", self.plan.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCustomerBalance {
    #[doc = "Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`."]
    pub bank_transfer: Option<
        stripe_types_05::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer,
    >,
    #[doc = "The funding method type to be used when there are not enough funds in the customer balance."]
    pub funding_type: Option<String>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsCustomerBalance {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("bank_transfer", self.bank_transfer.as_ref());
        form.field_opt("funding_type", self.funding_type.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPayto {
    #[doc = "Additional fields for Mandate creation."]
    pub mandate_options:
        Option<stripe_types_05::invoices::CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptions>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPayto {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone)]
pub struct CreatePreviewParamsAutomaticTaxLiability {
    #[doc = "The connected account being referenced when `type` is `account`."]
    pub account: Option<String>,
    #[doc = "Type of the account referenced in the request."]
    pub r#type: stripe_types_05::invoices::CreatePreviewParamsAutomaticTaxLiabilityType,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsAutomaticTaxLiability {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreatePreviewParamsInvoiceItemsItemPriceData {
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "The ID of the [Product](https://docs.stripe.com/api/products) that this [Price](https://docs.stripe.com/api/prices) w..."]
    pub product: String,
    #[doc = "Only required if a default tax behavior..."]
    pub tax_behavior: Option<stripe_types_06::invoices::CreatePreviewParamsInvoiceItemsItemPriceDataTaxBehavior>,
    #[doc = "A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge."]
    pub unit_amount: Option<i64>,
    #[doc = "Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places."]
    pub unit_amount_decimal: Option<String>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsInvoiceItemsItemPriceData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("currency", &self.currency);
        form.field("product", &self.product);
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
        form.field_opt("unit_amount", self.unit_amount.as_ref());
        form.field_opt("unit_amount_decimal", self.unit_amount_decimal.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone)]
pub struct CreatePreviewParamsScheduleDetailsBillingMode {
    #[doc = "Configure behavior for flexible billing mode."]
    pub flexible: Option<stripe_types_06::invoices::CreatePreviewParamsScheduleDetailsBillingModeFlexible>,
    #[doc = "Controls the calculation and orchestration of prorations and invoices for subscriptions."]
    pub r#type: stripe_types_01::invoices::CreatePreviewParamsScheduleDetailsBillingModeType,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsScheduleDetailsBillingMode {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("flexible", self.flexible.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone)]
pub struct CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriod {
    #[doc = "End of the invoice item period."]
    pub end: stripe_types_05::invoices::CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodEnd,
    #[doc = "Start of the invoice item period."]
    pub start: stripe_types_06::invoices::CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriodStart,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPeriod {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("end", &self.end);
        form.field("start", &self.start);
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPriceData {
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "The ID of the [Product](https://docs.stripe.com/api/products) that this [Price](https://docs.stripe.com/api/prices) w..."]
    pub product: String,
    #[doc = "Only required if a default tax behavior..."]
    pub tax_behavior: Option<
        stripe_types_05::invoices::CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPriceDataTaxBehavior,
    >,
    #[doc = "A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge or a negati..."]
    pub unit_amount: Option<i64>,
    #[doc = "Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places."]
    pub unit_amount_decimal: Option<String>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemPriceData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("currency", &self.currency);
        form.field("product", &self.product);
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
        form.field_opt("unit_amount", self.unit_amount.as_ref());
        form.field_opt("unit_amount_decimal", self.unit_amount_decimal.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone)]
pub struct CreatePreviewParamsScheduleDetailsPhasesItemAutomaticTaxLiability {
    #[doc = "The connected account being referenced when `type` is `account`."]
    pub account: Option<String>,
    #[doc = "Type of the account referenced in the request."]
    pub r#type: stripe_types_06::invoices::CreatePreviewParamsScheduleDetailsPhasesItemAutomaticTaxLiabilityType,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsScheduleDetailsPhasesItemAutomaticTaxLiability {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreatePreviewParamsSubscriptionDetailsPauseBillFor {
    #[doc = "Controls when to bill for metered usage in the current period."]
    pub outstanding_usage_through:
        Option<stripe_types_05::invoices::CreatePreviewParamsSubscriptionDetailsPauseBillForOutstandingUsageThrough>,
    #[doc = "Controls when to credit for unused time on licensed items."]
    pub unused_time_from:
        Option<stripe_types_06::invoices::CreatePreviewParamsSubscriptionDetailsPauseBillForUnusedTimeFrom>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsSubscriptionDetailsPauseBillFor {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("outstanding_usage_through", self.outstanding_usage_through.as_ref());
        form.field_opt("unused_time_from", self.unused_time_from.as_ref());
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
#[derive(Debug, Clone)]
pub enum InvoiceApplicationExpanded {
    /// Expanded `application`.
    Application(Box<stripe_types_05::common::Application>),
    /// An object type not known to this version of the crate.
    Other(json::OwnedValue),
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl<'json> json::Decode<'json> for InvoiceApplicationExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "application" => {
                InvoiceApplicationExpanded::Application(Box::new(json::from_slice_owned(&json::to_vec(&value))?))
            }
            _ => InvoiceApplicationExpanded::Other(value),
        })
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct RemoveLinesParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub invoice_metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The line items to remove."]
    pub lines: Vec<stripe_types_06::invoices::RemoveLinesParamsLinesItem>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for RemoveLinesParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.invoice_metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("invoice_metadata", key, value);
            }
        }
        for (index, item) in self.lines.iter().enumerate() {
            form.field_indexed("lines", index, item);
        }
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct UpdateInvoiceParamsRenderingPdf {
    #[doc = "Page size for invoice PDF."]
    pub page_size: Option<stripe_types_06::invoices::UpdateInvoiceParamsRenderingPdfPageSize>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for UpdateInvoiceParamsRenderingPdf {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("page_size", self.page_size.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone)]
pub struct UpdateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMaximum {
    #[doc = "A unit of time."]
    pub unit: stripe_types_06::invoices::UpdateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit,
    #[doc = "Must be greater than 0."]
    pub value: i64,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for UpdateInvoiceParamsShippingCostShippingRateDataDeliveryEstimateMaximum {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("unit", &self.unit);
        form.field("value", &self.value);
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct UpdateInvoiceParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsValue {
    #[doc = "A non-negative integer in cents representing how much to charge."]
    pub amount: i64,
    #[doc = "Specifies whether the rate is considered inclusive of taxes or exclusive of taxes."]
    pub tax_behavior: Option<stripe_types_06::invoices::UpdateInvoiceParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsValueTaxBehavior>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for UpdateInvoiceParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsValue {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        form.field_opt("tax_behavior", self.tax_behavior.as_ref());
    }
}
