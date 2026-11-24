#[cfg(feature = "transfers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTransferParams {
    #[doc = "A positive integer in cents (or local equivalent) representing how much to transfer."]
    pub amount: Option<i64>,
    #[doc = "Three-letter [ISO code for currency](https://www.iso.org/iso-4217-currency-codes.html) in lowercase."]
    pub currency: String,
    #[doc = "An arbitrary string attached to the object."]
    pub description: Option<String>,
    #[doc = "The ID of a connected Stripe account."]
    pub destination: String,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "You can use this parameter to transfer funds from a charge before they are added to your available balance."]
    pub source_transaction: Option<String>,
    #[doc = "The source balance to use for this transfer."]
    pub source_type: Option<stripe_types_04::transfers::CreateTransferParamsSourceType>,
    #[doc = "A string that identifies this transaction as part of a group."]
    pub transfer_group: Option<String>,
}

#[cfg(feature = "transfers")]
impl crate::params::FormEncode for CreateTransferParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field("currency", &self.currency);
        form.field_opt("description", self.description.as_ref());
        form.field("destination", &self.destination);
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("source_transaction", self.source_transaction.as_ref());
        form.field_opt("source_type", self.source_type.as_ref());
        form.field_opt("transfer_group", self.transfer_group.as_ref());
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
pub struct TransferSchedule {
    #[doc = "The number of days charges for the account will be held before being paid out."]
    #[json(default)]
    pub delay_days: i64,
    #[doc = "How frequently funds will be paid out."]
    #[json(default)]
    pub interval: String,
    #[doc = "The day of the month funds will be paid out."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub monthly_anchor: Option<i64>,
    #[doc = "The days of the month funds will be paid out."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub monthly_payout_days: Option<Vec<i64>>,
    #[doc = "The day of the week funds will be paid out, of the style 'monday', 'tuesday', etc."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub weekly_anchor: Option<String>,
    #[doc = "The days of the week when available funds are paid out, specified as an array, for example, `monday`, `tuesday`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub weekly_payout_days: Option<Vec<stripe_types_03::transfers::TransferScheduleWeeklyPayoutDaysItem>>,
}
