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
pub struct File {
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "The file expires and isn't available at this time in epoch seconds."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[doc = "The suitable name for saving the file to a filesystem."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "A list of [file links](https://docs.stripe.com/api#file_links) that point at this file."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<crate::List<crate::file_links::FileLink>>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::files::FileObject,
    #[doc = "The [purpose](https://docs.stripe.com/file-upload#uploading-a-file) of the uploaded file."]
    #[json(default)]
    pub purpose: stripe_types_03::files::FilePurpose,
    #[doc = "The size of the file object in bytes."]
    #[json(default)]
    pub size: i64,
    #[doc = "A suitable title for the document."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The returned file type (for example, `csv`, `pdf`, `jpg`, or `png`)."]
    #[json(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[doc = "Use your live secret API key to download the file from this URL."]
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
impl crate::PagedItem for File {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "files")]
#[derive(Debug, Clone, Default)]
pub struct ListFilesParams {
    pub created: Option<stripe_types_00::files::ListFilesParamsCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub purpose: Option<stripe_types_02::files::ListFilesParamsPurpose>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "files")]
impl crate::params::FormEncode for ListFilesParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("created", self.created.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("purpose", self.purpose.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}
