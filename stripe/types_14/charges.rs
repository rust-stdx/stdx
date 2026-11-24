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
pub struct Charge {
    #[doc = "Amount intended to be collected by this payment."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Amount in cents (or local equivalent) captured (can be less than the amount attribute on the charge if a partial capt..."]
    #[json(default)]
    pub amount_captured: i64,
    #[doc = "Amount in cents (or local equivalent) refunded (can be less than the amount attribute on the charge if a partial refu..."]
    #[json(default)]
    pub amount_refunded: i64,
    #[doc = "ID of the Connect application that created the charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<crate::Expandable<stripe_types_05::common::Application>>,
    #[doc = "The application fee (if any) for the charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub application_fee: Option<crate::Expandable<crate::application_fees::ApplicationFee>>,
    #[doc = "The amount of the application fee (if any) requested for the charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    #[doc = "Authorization code on the charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<String>,
    #[doc = "ID of the balance transaction that describes the impact of this charge on your account balance (not including refunds..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<crate::Expandable<crate::balance_transactions::BalanceTransaction>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<stripe_types_05::billing::BillingDetails>,
    #[doc = "The full statement descriptor that is passed to card networks, and that is displayed on your customers' credit card a..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub calculated_statement_descriptor: Option<String>,
    #[doc = "If the charge was created without capturing, this Boolean represents whether it is still uncaptured or has since been..."]
    #[json(default)]
    pub captured: bool,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "ID of the customer this charge is for if one exists."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<crate::Expandable<crate::charges::ChargeCustomerExpanded>>,
    #[doc = "An arbitrary string attached to the object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Whether the charge has been disputed."]
    #[json(default)]
    pub disputed: bool,
    #[doc = "ID of the balance transaction that describes the reversal of the balance on your account due to payment failure."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub failure_balance_transaction: Option<crate::Expandable<crate::balance_transactions::BalanceTransaction>>,
    #[doc = "Error code explaining reason for charge failure if available (see the errors section..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[doc = "Message to user further explaining reason for charge failure if available."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[doc = "Information on fraud assessments for the charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fraud_details: Option<stripe_types_00::charges::ChargeFraudDetails>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub level3: Option<stripe_types_06::charges::Level3>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_01::charges::ChargeObject,
    #[doc = "The account (if any) the charge was made on behalf of without triggering an automatic transfer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<crate::Expandable<crate::account::Account>>,
    #[doc = "Details about whether the payment was accepted, and why."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<stripe_types_05::charges::ChargeOutcome>,
    #[doc = "`true` if the charge succeeded, or was successfully authorized for later capture."]
    #[json(default)]
    pub paid: bool,
    #[doc = "ID of the PaymentIntent associated with this charge, if one exists."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<crate::Expandable<crate::payment_intents::PaymentIntent>>,
    #[doc = "ID of the payment method used in this charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    #[doc = "Details about the payment method at the time of the transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_details: Option<crate::payment_methods::PaymentMethodDetails>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub presentment_details: Option<stripe_types_04::payment_intents::PaymentFlowsPaymentIntentPresentmentDetails>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<stripe_types_00::radar::RadarRadarOptions>,
    #[doc = "This is the email address that the receipt for this charge was sent to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<String>,
    #[doc = "This is the transaction number that appears on email receipts sent for this charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub receipt_number: Option<String>,
    #[doc = "This is the URL to view the receipt for this charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub receipt_url: Option<String>,
    #[doc = "Whether the charge has been fully refunded."]
    #[json(default)]
    pub refunded: bool,
    #[doc = "A list of refunds that have been applied to the charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refunds: Option<crate::List<crate::refunds::Refund>>,
    #[doc = "ID of the review associated with this charge if one exists."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub review: Option<crate::Expandable<crate::reviews::Review>>,
    #[doc = "Shipping information for the charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping: Option<stripe_types_05::common::Shipping>,
    #[doc = "This is a legacy field that will be removed in the future."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<crate::common::PaymentSource>,
    #[doc = "The transfer ID which created this charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub source_transfer: Option<crate::Expandable<crate::transfers::Transfer>>,
    #[doc = "For a non-card charge, text that appears on the customer's statement as the statement descriptor."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    #[doc = "Provides information about a card charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<String>,
    #[doc = "The status of the payment is either `succeeded`, `pending`, or `failed`."]
    #[json(default)]
    pub status: stripe_types_02::charges::ChargeStatus,
    #[doc = "ID of the transfer to the `destination` account (only applicable if the charge was created using the `destination` pa..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transfer: Option<crate::Expandable<crate::transfers::Transfer>>,
    #[doc = "An optional dictionary including the account to automatically transfer to as part of a destination charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<crate::charges::ChargeTransferData>,
    #[doc = "A string that identifies this transaction as part of a group."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
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
impl crate::PagedItem for Charge {
    fn id(&self) -> &str {
        &self.id
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
#[derive(Debug, Clone)]
pub enum ChargeCustomerExpanded {
    /// Expanded `customer`.
    Customer(Box<crate::customers::Customer>),
    /// An object type not known to this version of the crate.
    Other(json::OwnedValue),
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
impl<'json> json::Decode<'json> for ChargeCustomerExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "customer" => ChargeCustomerExpanded::Customer(Box::new(json::from_slice_owned(&json::to_vec(&value))?)),
            _ => ChargeCustomerExpanded::Other(value),
        })
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
pub struct ChargeTransferData {
    #[doc = "The amount transferred to the destination account, if specified."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[doc = "ID of an existing, connected Stripe account to transfer funds to if `transfer_data` was specified in the charge request."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<crate::Expandable<crate::account::Account>>,
}
