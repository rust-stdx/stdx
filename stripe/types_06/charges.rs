#[cfg(feature = "charges")]
#[derive(Debug, Clone)]
pub struct CreateChargeParamsShipping {
    #[doc = "Shipping address."]
    pub address: stripe_types_04::charges::CreateChargeParamsShippingAddress,
    #[doc = "The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc."]
    pub carrier: Option<String>,
    #[doc = "Recipient name."]
    pub name: String,
    #[doc = "Recipient phone (including extension)."]
    pub phone: Option<String>,
    #[doc = "The tracking number for a physical product, obtained from the delivery service."]
    pub tracking_number: Option<String>,
}

#[cfg(feature = "charges")]
impl crate::params::FormEncode for CreateChargeParamsShipping {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("address", &self.address);
        form.field_opt("carrier", self.carrier.as_ref());
        form.field("name", &self.name);
        form.field_opt("phone", self.phone.as_ref());
        form.field_opt("tracking_number", self.tracking_number.as_ref());
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
pub struct Level3 {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_reference: Option<String>,
    #[json(default)]
    pub line_items: Vec<stripe_types_00::charges::Level3LineItems>,
    #[json(default)]
    pub merchant_reference: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping_address_zip: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping_amount: Option<i64>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping_from_zip: Option<String>,
}

#[cfg(feature = "charges")]
pub type UpdateChargeParamsShippingAddress = stripe_types_04::charges::CreateChargeParamsShippingAddress;
