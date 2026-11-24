#[cfg(feature = "charges")]
#[derive(Debug, Clone, Default)]
pub struct CaptureChargeParams {
    #[doc = "The amount to capture, which must be less than or equal to the original amount."]
    pub amount: Option<i64>,
    #[doc = "An application fee to add on to this charge."]
    pub application_fee: Option<i64>,
    #[doc = "An application fee amount to add on to this charge, which must be less than or equal to the original amount."]
    pub application_fee_amount: Option<i64>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The email address to send this charge's receipt to."]
    pub receipt_email: Option<String>,
    #[doc = "For a non-card charge, text that appears on the customer's statement as the statement descriptor."]
    pub statement_descriptor: Option<String>,
    #[doc = "Provides information about a card charge."]
    pub statement_descriptor_suffix: Option<String>,
    #[doc = "An optional dictionary including the account to automatically transfer to as part of a destination charge."]
    pub transfer_data: Option<stripe_types_00::charges::CaptureChargeParamsTransferData>,
    #[doc = "A string that identifies this transaction as part of a group."]
    pub transfer_group: Option<String>,
}

#[cfg(feature = "charges")]
impl crate::params::FormEncode for CaptureChargeParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("application_fee", self.application_fee.as_ref());
        form.field_opt("application_fee_amount", self.application_fee_amount.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("receipt_email", self.receipt_email.as_ref());
        form.field_opt("statement_descriptor", self.statement_descriptor.as_ref());
        form.field_opt("statement_descriptor_suffix", self.statement_descriptor_suffix.as_ref());
        form.field_opt("transfer_data", self.transfer_data.as_ref());
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
pub struct ChargeOutcome {
    #[doc = "An enumerated value providing a more detailed explanation on how to proceed with an error..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub advice_code: Option<stripe_types_00::charges::ChargeOutcomeAdviceCode>,
    #[doc = "For charges declined by the network, a 2 digit code which indicates the advice returned by the network on how to proc..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub network_advice_code: Option<String>,
    #[doc = "For charges declined by the network, an alphanumeric code which indicates the reason the charge failed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub network_decline_code: Option<String>,
    #[doc = "Possible values are `approved_by_network`, `declined_by_network`, `not_sent_to_network`, and `reversed_after_approval`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub network_status: Option<String>,
    #[doc = "An enumerated value providing a more detailed explanation of the outcome's `type`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "Stripe Radar's evaluation of the riskiness of the payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub risk_level: Option<String>,
    #[doc = "Stripe Radar's evaluation of the riskiness of the payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub risk_score: Option<i64>,
    #[doc = "The ID of the Radar rule that matched the payment, if applicable."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub rule: Option<crate::Expandable<stripe_types_00::radar::Rule>>,
    #[doc = "A human-readable description of the outcome type and reason, designed for you (the recipient of the payment), not you..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub seller_message: Option<String>,
    #[doc = "Possible values are `authorized`, `manual_review`, `issuer_declined`, `blocked`, and `invalid`."]
    #[json(rename = "type", default)]
    pub r#type: String,
}

#[cfg(feature = "charges")]
pub type GetChargeRefundParams = stripe_types_01::charges::GetChargeParams;

#[cfg(feature = "charges")]
#[derive(Debug, Clone, Default)]
pub struct ListChargesParams {
    pub created: Option<stripe_types_03::charges::ListChargesParamsCreated>,
    pub customer: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub payment_intent: Option<String>,
    pub starting_after: Option<String>,
    pub transfer_group: Option<String>,
}

#[cfg(feature = "charges")]
impl crate::params::FormEncode for ListChargesParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("created", self.created.as_ref());
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("payment_intent", self.payment_intent.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("transfer_group", self.transfer_group.as_ref());
    }
}

#[cfg(feature = "charges")]
#[derive(Debug, Clone)]
pub struct UpdateChargeParamsFraudDetails {
    #[doc = "Either `safe` or `fraudulent`."]
    pub user_report: stripe_types_01::charges::UpdateChargeParamsFraudDetailsUserReport,
}

#[cfg(feature = "charges")]
impl crate::params::FormEncode for UpdateChargeParamsFraudDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("user_report", &self.user_report);
    }
}
