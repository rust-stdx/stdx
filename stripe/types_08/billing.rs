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
pub struct BillingCreditGrantsResourceApplicabilityConfig {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<stripe_types_06::billing::BillingCreditGrantsResourceScope>,
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CreditBalance {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub available_balance: Option<stripe_types_05::billing::BillingCreditGrantsResourceAmount>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ledger_balance: Option<stripe_types_05::billing::BillingCreditGrantsResourceAmount>,
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone)]
pub struct GetBillingCreditBalanceSummaryParamsFilter {
    #[doc = "The billing credit applicability scope for which to fetch credit balance summary."]
    pub applicability_scope:
        Option<stripe_types_06::billing::GetBillingCreditBalanceSummaryParamsFilterApplicabilityScope>,
    #[doc = "The credit grant for which to fetch credit balance summary."]
    pub credit_grant: Option<String>,
    #[doc = "Specify the type of this filter."]
    pub r#type: stripe_types_04::billing::GetBillingCreditBalanceSummaryParamsFilterType,
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for GetBillingCreditBalanceSummaryParamsFilter {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("applicability_scope", self.applicability_scope.as_ref());
        form.field_opt("credit_grant", self.credit_grant.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "billing")]
#[derive(Debug, Clone, Default)]
pub struct ListBillingMeterEventSummariesParams {
    pub customer: String,
    pub end_time: i64,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub start_time: i64,
    pub starting_after: Option<String>,
    pub value_grouping_window:
        Option<stripe_types_05::billing::ListBillingMeterEventSummariesParamsValueGroupingWindow>,
}

#[cfg(feature = "billing")]
impl crate::params::FormEncode for ListBillingMeterEventSummariesParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("customer", &self.customer);
        form.field("end_time", &self.end_time);
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field("start_time", &self.start_time);
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("value_grouping_window", self.value_grouping_window.as_ref());
    }
}
