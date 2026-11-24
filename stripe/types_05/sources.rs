#[cfg(feature = "sources")]
#[derive(Debug, Clone)]
pub struct CreateSourceParamsMandateAcceptance {
    #[doc = "The Unix timestamp (in seconds) when the mandate was accepted or refused by the customer."]
    pub date: Option<i64>,
    #[doc = "The IP address from which the mandate was accepted or refused by the customer."]
    pub ip: Option<String>,
    #[doc = "The parameters required to store a mandate accepted offline."]
    pub offline: Option<stripe_types_00::sources::CreateSourceParamsMandateAcceptanceOffline>,
    #[doc = "The parameters required to store a mandate accepted online."]
    pub online: Option<stripe_types_01::sources::CreateSourceParamsMandateAcceptanceOnline>,
    #[doc = "The status of the mandate acceptance."]
    pub status: stripe_types_02::sources::CreateSourceParamsMandateAcceptanceStatus,
    #[doc = "The type of acceptance information included with the mandate."]
    pub r#type: Option<stripe_types_00::sources::CreateSourceParamsMandateAcceptanceType>,
    #[doc = "The user agent of the browser from which the mandate was accepted or refused by the customer."]
    pub user_agent: Option<String>,
}

#[cfg(feature = "sources")]
impl crate::params::FormEncode for CreateSourceParamsMandateAcceptance {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("date", self.date.as_ref());
        form.field_opt("ip", self.ip.as_ref());
        form.field_opt("offline", self.offline.as_ref());
        form.field_opt("online", self.online.as_ref());
        form.field("status", &self.status);
        form.field_opt("type", self.r#type.as_ref());
        form.field_opt("user_agent", self.user_agent.as_ref());
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
    feature = "sources",
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
pub struct SourceOwner {
    #[doc = "Owner's address."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_01::common::Address>,
    #[doc = "Owner's email address."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "Owner's full name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Owner's phone number (including extension)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "Verified owner's address."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verified_address: Option<stripe_types_01::common::Address>,
    #[doc = "Verified owner's email address."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verified_email: Option<String>,
    #[doc = "Verified owner's full name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verified_name: Option<String>,
    #[doc = "Verified owner's phone number (including extension)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verified_phone: Option<String>,
}

#[cfg(feature = "sources")]
pub type UpdateSourceParamsMandateAcceptanceOffline =
    stripe_types_00::sources::CreateSourceParamsMandateAcceptanceOffline;

#[cfg(feature = "sources")]
pub type UpdateSourceParamsMandateAcceptanceStatus =
    stripe_types_02::sources::CreateSourceParamsMandateAcceptanceStatus;

#[cfg(feature = "sources")]
pub type UpdateSourceParamsMandateAcceptanceType = stripe_types_00::sources::CreateSourceParamsMandateAcceptanceType;

#[cfg(feature = "sources")]
pub type UpdateSourceParamsSourceOrderShippingAddress =
    stripe_types_01::sources::CreateSourceParamsSourceOrderShippingAddress;
