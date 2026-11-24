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
pub struct PaymentIntent {
    #[doc = "The list of payment method types allowed for use with this payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub allowed_payment_method_types:
        Option<Vec<stripe_types_03::payment_intents::PaymentIntentAllowedPaymentMethodTypesItem>>,
    #[doc = "Amount intended to be collected by this PaymentIntent."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Amount that can be captured from this PaymentIntent."]
    #[json(default)]
    pub amount_capturable: i64,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_details: Option<stripe_types_12::payment_intents::PaymentFlowsAmountDetails>,
    #[doc = "Amount that this PaymentIntent collects."]
    #[json(default)]
    pub amount_received: i64,
    #[doc = "ID of the Connect application that created the PaymentIntent."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<crate::Expandable<stripe_types_05::common::Application>>,
    #[doc = "The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    #[doc = "Settings to configure compatible payment methods from the Stripe Dashboard..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub automatic_payment_methods:
        Option<stripe_types_05::payment_intents::PaymentFlowsAutomaticPaymentMethodsPaymentIntent>,
    #[doc = "Populated when `status` is `canceled`, this is the time at which the PaymentIntent was canceled."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<i64>,
    #[doc = "Reason for cancellation of this PaymentIntent, either user-provided (`duplicate`, `fraudulent`, `requested_by_custome..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<stripe_types_04::payment_intents::PaymentIntentCancellationReason>,
    #[doc = "Controls when the funds will be captured from the customer's account."]
    #[json(default)]
    pub capture_method: stripe_types_04::payment_intents::PaymentIntentCaptureMethod,
    #[doc = "The client secret of this PaymentIntent."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[doc = "Describes whether we can confirm this PaymentIntent automatically, or if it requires customer action to confirm the p..."]
    #[json(default)]
    pub confirmation_method: stripe_types_01::payment_intents::PaymentIntentConfirmationMethod,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "ID of the Customer this PaymentIntent belongs to, if one exists."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<crate::Expandable<crate::payment_intents::PaymentIntentCustomerExpanded>>,
    #[doc = "ID of the Account representing the customer that this PaymentIntent belongs to, if one exists."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_account: Option<String>,
    #[doc = "An arbitrary string attached to the object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The list of payment method types to exclude from use with this payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub excluded_payment_method_types:
        Option<Vec<stripe_types_03::payment_intents::PaymentIntentExcludedPaymentMethodTypesItem>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub hooks: Option<stripe_types_09::payment_intents::PaymentFlowsPaymentIntentAsyncWorkflows>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "The payment error encountered in the previous PaymentIntent confirmation."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last_payment_error: Option<Box<crate::common::ApiErrors>>,
    #[doc = "ID of the latest [Charge object](https://docs.stripe.com/api/charges) created by this PaymentIntent."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub latest_charge: Option<crate::Expandable<crate::charges::Charge>>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Settings for Managed Payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub managed_payments: Option<stripe_types_01::common::SmorResourceManagedPayments>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "If present, this property tells you what actions you need to take in order for your customer to fulfill a payment usi..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub next_action: Option<stripe_types_12::payment_intents::PaymentIntentNextAction>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_01::payment_intents::PaymentIntentObject,
    #[doc = "You can specify the settlement merchant as the connected account using the `on_behalf_of` attribute on the charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<crate::Expandable<crate::account::Account>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_details: Option<stripe_types_01::payment_intents::PaymentFlowsPaymentDetails>,
    #[doc = "ID of the payment method used in this PaymentIntent."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<crate::Expandable<crate::payment_methods::PaymentMethod>>,
    #[doc = "Information about the [payment method configuration](https://docs.stripe.com/api/payment_method_configurations) used ..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_configuration_details:
        Option<stripe_types_03::payment_methods::PaymentMethodConfigBizPaymentMethodConfigurationDetails>,
    #[doc = "Payment-method-specific configuration for this PaymentIntent."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<stripe_types_12::payment_intents::PaymentIntentPaymentMethodOptions>,
    #[doc = "The list of payment method types (e.g."]
    #[json(default)]
    pub payment_method_types: Vec<String>,
    #[doc = "ID of the [Payment Record object](https://docs.stripe.com/api/payment-record) created by this PaymentIntent."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_record: Option<crate::Expandable<crate::payment_records::PaymentRecord>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub presentment_details: Option<stripe_types_04::payment_intents::PaymentFlowsPaymentIntentPresentmentDetails>,
    #[doc = "If present, this property tells you about the processing state of the payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub processing: Option<stripe_types_07::payment_intents::PaymentIntentProcessing>,
    #[doc = "Email address that the receipt for the resulting payment will be sent to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<String>,
    #[doc = "ID of the review associated with this PaymentIntent, if any."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub review: Option<crate::Expandable<crate::reviews::Review>>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<stripe_types_04::payment_intents::PaymentIntentSetupFutureUsage>,
    #[doc = "Shipping information for this PaymentIntent."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping: Option<stripe_types_05::common::Shipping>,
    #[doc = "This is a legacy field that will be removed in the future."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<crate::Expandable<crate::payment_intents::PaymentIntentSourceExpanded>>,
    #[doc = "Text that appears on the customer's statement as the statement descriptor for a non-card charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    #[doc = "Provides information about a card charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<String>,
    #[doc = "Status of this PaymentIntent, one of `requires_payment_method`, `requires_confirmation`, `requires_action`, `processi..."]
    #[json(default)]
    pub status: stripe_types_01::payment_intents::PaymentIntentStatus,
    #[doc = "The data that automatically creates a Transfer after the payment finalizes."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<crate::transfers::TransferData>,
    #[doc = "A string that identifies the resulting payment as part of a group."]
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
impl crate::PagedItem for PaymentIntent {
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
pub enum PaymentIntentCustomerExpanded {
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
impl<'json> json::Decode<'json> for PaymentIntentCustomerExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "customer" => {
                PaymentIntentCustomerExpanded::Customer(Box::new(json::from_slice_owned(&json::to_vec(&value))?))
            }
            _ => PaymentIntentCustomerExpanded::Other(value),
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
#[derive(Debug, Clone)]
pub enum PaymentIntentSourceExpanded {
    /// Expanded `payment_source`.
    PaymentSource(Box<crate::common::PaymentSource>),
    /// Expanded `deleted_payment_source`.
    DeletedPaymentSource(Box<stripe_types_08::common::DeletedPaymentSource>),
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
impl<'json> json::Decode<'json> for PaymentIntentSourceExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "payment_source" => {
                PaymentIntentSourceExpanded::PaymentSource(Box::new(json::from_slice_owned(&json::to_vec(&value))?))
            }
            "deleted_payment_source" => PaymentIntentSourceExpanded::DeletedPaymentSource(Box::new(
                json::from_slice_owned(&json::to_vec(&value))?,
            )),
            _ => PaymentIntentSourceExpanded::Other(value),
        })
    }
}
