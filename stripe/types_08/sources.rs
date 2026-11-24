#[cfg(feature = "sources")]
#[derive(Debug, Clone, Default)]
pub struct CreateSourceParamsMandate {
    #[doc = "The parameters required to notify Stripe of a mandate acceptance or refusal by the customer."]
    pub acceptance: Option<stripe_types_05::sources::CreateSourceParamsMandateAcceptance>,
    #[doc = "The amount specified by the mandate."]
    pub amount: Option<json::OwnedValue>,
    #[doc = "The currency specified by the mandate."]
    pub currency: Option<String>,
    #[doc = "The interval of debits permitted by the mandate."]
    pub interval: Option<stripe_types_01::sources::CreateSourceParamsMandateInterval>,
    #[doc = "The method Stripe should use to notify the customer of upcoming debit instructions and/or mandate confirmation as req..."]
    pub notification_method: Option<stripe_types_04::sources::CreateSourceParamsMandateNotificationMethod>,
}

#[cfg(feature = "sources")]
impl crate::params::FormEncode for CreateSourceParamsMandate {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("acceptance", self.acceptance.as_ref());
        form.field_opt("currency", self.currency.as_ref());
        form.field_opt("interval", self.interval.as_ref());
        form.field_opt("notification_method", self.notification_method.as_ref());
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
pub struct SourceOrder {
    #[doc = "A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zer..."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "The email address of the customer placing the order."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "List of items constituting the order."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<stripe_types_03::sources::SourceOrderItem>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping: Option<stripe_types_05::common::Shipping>,
}

#[cfg(feature = "sources")]
#[derive(Debug, Clone)]
pub struct UpdateSourceParamsMandateAcceptance {
    #[doc = "The Unix timestamp (in seconds) when the mandate was accepted or refused by the customer."]
    pub date: Option<i64>,
    #[doc = "The IP address from which the mandate was accepted or refused by the customer."]
    pub ip: Option<String>,
    #[doc = "The parameters required to store a mandate accepted offline."]
    pub offline: Option<stripe_types_05::sources::UpdateSourceParamsMandateAcceptanceOffline>,
    #[doc = "The parameters required to store a mandate accepted online."]
    pub online: Option<stripe_types_06::sources::UpdateSourceParamsMandateAcceptanceOnline>,
    #[doc = "The status of the mandate acceptance."]
    pub status: stripe_types_05::sources::UpdateSourceParamsMandateAcceptanceStatus,
    #[doc = "The type of acceptance information included with the mandate."]
    pub r#type: Option<stripe_types_05::sources::UpdateSourceParamsMandateAcceptanceType>,
    #[doc = "The user agent of the browser from which the mandate was accepted or refused by the customer."]
    pub user_agent: Option<String>,
}

#[cfg(feature = "sources")]
impl crate::params::FormEncode for UpdateSourceParamsMandateAcceptance {
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

#[cfg(feature = "sources")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSourceParamsOwner {
    #[doc = "Owner's address."]
    pub address: Option<stripe_types_06::sources::UpdateSourceParamsOwnerAddress>,
    #[doc = "Owner's email address."]
    pub email: Option<String>,
    #[doc = "Owner's full name."]
    pub name: Option<String>,
    #[doc = "Owner's phone number."]
    pub phone: Option<String>,
}

#[cfg(feature = "sources")]
impl crate::params::FormEncode for UpdateSourceParamsOwner {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("address", self.address.as_ref());
        form.field_opt("email", self.email.as_ref());
        form.field_opt("name", self.name.as_ref());
        form.field_opt("phone", self.phone.as_ref());
    }
}

#[cfg(feature = "sources")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSourceParamsSourceOrderItemsItem {
    pub amount: Option<i64>,
    pub currency: Option<String>,
    pub description: Option<String>,
    #[doc = "The ID of the SKU being ordered."]
    pub parent: Option<String>,
    #[doc = "The quantity of this order item."]
    pub quantity: Option<i64>,
    pub r#type: Option<stripe_types_06::sources::UpdateSourceParamsSourceOrderItemsItemType>,
}

#[cfg(feature = "sources")]
impl crate::params::FormEncode for UpdateSourceParamsSourceOrderItemsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("currency", self.currency.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field_opt("parent", self.parent.as_ref());
        form.field_opt("quantity", self.quantity.as_ref());
        form.field_opt("type", self.r#type.as_ref());
    }
}
