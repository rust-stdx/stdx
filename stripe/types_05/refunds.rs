#[cfg(feature = "refunds")]
#[derive(Debug, Clone, Default)]
pub struct ListRefundsParams {
    pub charge: Option<String>,
    pub created: Option<stripe_types_00::refunds::ListRefundsParamsCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub payment_intent: Option<String>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "refunds")]
impl crate::params::FormEncode for ListRefundsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("charge", self.charge.as_ref());
        form.field_opt("created", self.created.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("payment_intent", self.payment_intent.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct RefundDestinationDetailsCard {
    #[doc = "Value of the reference number assigned to the refund."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[doc = "Status of the reference number on the refund."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reference_status: Option<String>,
    #[doc = "Type of the reference number assigned to the refund."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reference_type: Option<String>,
    #[doc = "The type of refund."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::refunds::RefundDestinationDetailsCardType,
}
