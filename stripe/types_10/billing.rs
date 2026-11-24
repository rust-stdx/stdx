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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BillingBillResourceInvoicingLinesParentsInvoiceLineItemParent {
    #[doc = "Details about the invoice item that generated this line item"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub invoice_item_details:
        Option<stripe_types_09::billing::BillingBillResourceInvoicingLinesParentsInvoiceLineItemInvoiceItemParent>,
    #[doc = "Details about the subscription item that generated this line item"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub subscription_item_details:
        Option<stripe_types_09::billing::BillingBillResourceInvoicingLinesParentsInvoiceLineItemSubscriptionItemParent>,
    #[doc = "The type of parent that generated this line item"]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::billing::BillingBillResourceInvoicingLinesParentsInvoiceLineItemParentType,
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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BillingBillResourceInvoicingTaxesTax {
    #[doc = "The amount of the tax, in cents (or local equivalent)."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Whether this tax is inclusive or exclusive."]
    #[json(default)]
    pub tax_behavior: stripe_types_02::billing::BillingBillResourceInvoicingTaxesTaxTaxBehavior,
    #[doc = "Additional details about the tax rate."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax_rate_details: Option<stripe_types_07::billing::BillingBillResourceInvoicingTaxesTaxRateDetails>,
    #[doc = "The reasoning behind this tax, for example, if the product is tax exempt."]
    #[json(default)]
    pub taxability_reason: stripe_types_04::billing::BillingBillResourceInvoicingTaxesTaxTaxabilityReason,
    #[doc = "The amount on which tax is calculated, in cents (or local equivalent)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub taxable_amount: Option<i64>,
    #[doc = "The type of tax information."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::billing::BillingBillResourceInvoicingTaxesTaxType,
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone)]
pub struct CreateBillingAlertParams {
    #[doc = "The type of alert to create."]
    pub alert_type: stripe_types_05::billing::CreateBillingAlertParamsAlertType,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The title of the alert."]
    pub title: String,
    #[doc = "The configuration of the usage threshold."]
    pub usage_threshold: Option<stripe_types_09::billing::CreateBillingAlertParamsUsageThreshold>,
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for CreateBillingAlertParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("alert_type", &self.alert_type);
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("title", &self.title);
        form.field_opt("usage_threshold", self.usage_threshold.as_ref());
    }
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone)]
pub struct CreateBillingCreditGrantParamsApplicabilityConfig {
    #[doc = "Specify the scope of this applicability config."]
    pub scope: stripe_types_09::billing::CreateBillingCreditGrantParamsApplicabilityConfigScope,
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for CreateBillingCreditGrantParamsApplicabilityConfig {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("scope", &self.scope);
    }
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone)]
pub struct GetBillingCreditBalanceSummaryParams {
    pub customer: Option<String>,
    pub customer_account: Option<String>,
    pub expand: Option<Vec<String>>,
    pub filter: stripe_types_08::billing::GetBillingCreditBalanceSummaryParamsFilter,
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for GetBillingCreditBalanceSummaryParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("customer_account", self.customer_account.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("filter", &self.filter);
    }
}
