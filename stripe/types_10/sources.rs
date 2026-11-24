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
pub struct Source {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ach_credit_transfer: Option<stripe_types_00::sources::SourceTypeAchCreditTransfer>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ach_debit: Option<stripe_types_03::sources::SourceTypeAchDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<stripe_types_02::sources::SourceTypeAcssDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub alipay: Option<stripe_types_04::sources::SourceTypeAlipay>,
    #[doc = "This field indicates whether this payment method can be shown again to its customer in a checkout flow."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub allow_redisplay: Option<stripe_types_04::sources::SourceAllowRedisplay>,
    #[doc = "A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zer..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<stripe_types_02::sources::SourceTypeAuBecsDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<stripe_types_01::sources::SourceTypeBancontact>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types_04::sources::SourceTypeCard>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card_present: Option<stripe_types_02::sources::SourceTypeCardPresent>,
    #[doc = "The client secret of the source."]
    #[json(default)]
    pub client_secret: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub code_verification: Option<stripe_types_00::sources::SourceCodeVerificationFlow>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) associated with the source."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "The ID of the customer to which this source is attached."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub eps: Option<stripe_types_04::sources::SourceTypeEps>,
    #[doc = "The authentication `flow` of the source."]
    #[json(default)]
    pub flow: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub giropay: Option<stripe_types_02::sources::SourceTypeGiropay>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ideal: Option<stripe_types_03::sources::SourceTypeIdeal>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub klarna: Option<stripe_types_02::sources::SourceTypeKlarna>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub multibanco: Option<stripe_types_00::sources::SourceTypeMultibanco>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_02::sources::SourceObject,
    #[doc = "Information about the owner of the payment instrument that may be used or required by particular source types."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<stripe_types_05::sources::SourceOwner>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub p24: Option<stripe_types_01::sources::SourceTypeP24>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub receiver: Option<stripe_types_02::sources::SourceReceiverFlow>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub redirect: Option<stripe_types_04::sources::SourceRedirectFlow>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sepa_credit_transfer: Option<stripe_types_01::sources::SourceTypeSepaCreditTransfer>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<stripe_types_03::sources::SourceTypeSepaDebit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sofort: Option<stripe_types_04::sources::SourceTypeSofort>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub source_order: Option<stripe_types_08::sources::SourceOrder>,
    #[doc = "Extra information about a source."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    #[doc = "The status of the source, one of `canceled`, `chargeable`, `consumed`, `failed`, or `pending`."]
    #[json(default)]
    pub status: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<stripe_types_00::sources::SourceTypeThreeDSecure>,
    #[doc = "The `type` of the source."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::sources::SourceType,
    #[doc = "Either `reusable` or `single_use`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub wechat: Option<stripe_types_02::sources::SourceTypeWechat>,
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
impl crate::PagedItem for Source {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "sources")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSourceParamsMandate {
    #[doc = "The parameters required to notify Stripe of a mandate acceptance or refusal by the customer."]
    pub acceptance: Option<stripe_types_08::sources::UpdateSourceParamsMandateAcceptance>,
    #[doc = "The amount specified by the mandate."]
    pub amount: Option<json::OwnedValue>,
    #[doc = "The currency specified by the mandate."]
    pub currency: Option<String>,
    #[doc = "The interval of debits permitted by the mandate."]
    pub interval: Option<stripe_types_06::sources::UpdateSourceParamsMandateInterval>,
    #[doc = "The method Stripe should use to notify the customer of upcoming debit instructions and/or mandate confirmation as req..."]
    pub notification_method: Option<stripe_types_06::sources::UpdateSourceParamsMandateNotificationMethod>,
}

#[cfg(feature = "sources")]
impl crate::params::FormEncode for UpdateSourceParamsMandate {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("acceptance", self.acceptance.as_ref());
        form.field_opt("currency", self.currency.as_ref());
        form.field_opt("interval", self.interval.as_ref());
        form.field_opt("notification_method", self.notification_method.as_ref());
    }
}

#[cfg(feature = "sources")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSourceParamsSourceOrder {
    #[doc = "List of items constituting the order."]
    pub items: Option<Vec<stripe_types_08::sources::UpdateSourceParamsSourceOrderItemsItem>>,
    #[doc = "Shipping address for the order."]
    pub shipping: Option<stripe_types_07::sources::UpdateSourceParamsSourceOrderShipping>,
}

#[cfg(feature = "sources")]
impl crate::params::FormEncode for UpdateSourceParamsSourceOrder {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.items.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("items", index, item);
            }
        }
        form.field_opt("shipping", self.shipping.as_ref());
    }
}
