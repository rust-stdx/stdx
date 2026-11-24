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
    feature = "file_links",
    feature = "files",
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
    feature = "reporting",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "sigma",
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
pub struct FileLink {
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Returns if the link is already expired."]
    #[json(default)]
    pub expired: bool,
    #[doc = "Time that the link expires."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[doc = "The file object this link points to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<crate::Expandable<crate::files::File>>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_01::file_links::FileLinkObject,
    #[doc = "The publicly accessible URL to download the file."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
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
    feature = "file_links",
    feature = "files",
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
    feature = "reporting",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "sigma",
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
impl crate::PagedItem for FileLink {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "file_links")]
#[derive(Debug, Clone, Default)]
pub struct ListFileLinksParams {
    pub created: Option<stripe_types_03::file_links::ListFileLinksParamsCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub expired: Option<bool>,
    pub file: Option<String>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "file_links")]
impl crate::params::FormEncode for ListFileLinksParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("created", self.created.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("expired", self.expired.as_ref());
        form.field_opt("file", self.file.as_ref());
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}
