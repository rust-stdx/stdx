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
pub struct Invoice {
    #[doc = "The country of the business associated with this invoice, most often the business creating the invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_country: Option<String>,
    #[doc = "The public name of the business associated with this invoice, most often the business creating the invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[doc = "The account tax IDs associated with the invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<Vec<crate::Expandable<crate::invoices::InvoiceAccountTaxIdsItemExpanded>>>,
    #[doc = "Final amount due at this time for this invoice."]
    #[json(default)]
    pub amount_due: i64,
    #[doc = "Amount that was overpaid on the invoice."]
    #[json(default)]
    pub amount_overpaid: i64,
    #[doc = "The amount, in cents (or local equivalent), that was paid."]
    #[json(default)]
    pub amount_paid: i64,
    #[doc = "Amount, in cents (or local equivalent), that was paid on the invoice outside of Stripe."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_paid_off_stripe: Option<i64>,
    #[doc = "The difference between amount_due and amount_paid, in cents (or local equivalent)."]
    #[json(default)]
    pub amount_remaining: i64,
    #[doc = "This is the sum of all the shipping amounts."]
    #[json(default)]
    pub amount_shipping: i64,
    #[doc = "ID of the Connect Application that created the invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<crate::Expandable<stripe_types_07::invoices::InvoiceApplicationExpanded>>,
    #[doc = "Number of payment attempts made for this invoice, from the perspective of the payment retry schedule."]
    #[json(default)]
    pub attempt_count: i64,
    #[doc = "Whether an attempt has been made to pay the invoice."]
    #[json(default)]
    pub attempted: bool,
    #[doc = "Controls whether Stripe performs automatic collection..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub auto_advance: Option<bool>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<crate::common::AutomaticTax>,
    #[doc = "The time when this invoice is currently scheduled to be automatically finalized."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub automatically_finalizes_at: Option<i64>,
    #[doc = "Indicates the reason why the invoice was created."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billing_reason: Option<stripe_types_02::invoices::InvoiceBillingReason>,
    #[doc = "Either `charge_automatically`, or `send_invoice`."]
    #[json(default)]
    pub collection_method: stripe_types_00::invoices::InvoiceCollectionMethod,
    #[doc = "The confirmation secret associated with this invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub confirmation_secret: Option<stripe_types_00::invoices::InvoicesResourceConfirmationSecret>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "Custom fields displayed on the invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<stripe_types_00::invoices::InvoiceSettingCustomField>>,
    #[doc = "The ID of the customer to bill."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<crate::Expandable<crate::invoices::InvoiceCustomerExpanded>>,
    #[doc = "The ID of the account representing the customer to bill."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_account: Option<String>,
    #[doc = "The customer's address."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_address: Option<stripe_types_01::common::Address>,
    #[doc = "The customer's email."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_email: Option<String>,
    #[doc = "The customer's name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
    #[doc = "The customer's phone number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_phone: Option<String>,
    #[doc = "The customer's shipping information."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_shipping: Option<stripe_types_05::common::Shipping>,
    #[doc = "The customer's tax exempt status."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_tax_exempt: Option<stripe_types_00::invoices::InvoiceCustomerTaxExempt>,
    #[doc = "The customer's tax IDs."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_tax_ids: Option<Vec<stripe_types_06::invoices::InvoicesResourceInvoiceTaxId>>,
    #[doc = "ID of the default payment method for the invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<crate::Expandable<crate::payment_methods::PaymentMethod>>,
    #[doc = "ID of the default payment source for the invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub default_source: Option<crate::Expandable<crate::common::PaymentSource>>,
    #[doc = "The tax rates applied to this invoice, if any."]
    #[json(default)]
    pub default_tax_rates: Vec<stripe_types_06::tax_rates::TaxRate>,
    #[doc = "An arbitrary string attached to the object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The discounts applied to the invoice."]
    #[json(default)]
    pub discounts: Vec<crate::Expandable<crate::invoices::InvoiceDiscountsItemExpanded>>,
    #[doc = "The date on which payment for this invoice is due."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub due_date: Option<i64>,
    #[doc = "The date when this invoice is in effect."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub effective_at: Option<i64>,
    #[doc = "Ending customer balance after the invoice is finalized."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ending_balance: Option<i64>,
    #[doc = "Footer displayed on the invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    #[doc = "Details of the invoice that was cloned."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub from_invoice: Option<crate::invoices::InvoicesResourceFromInvoice>,
    #[doc = "The URL for the hosted invoice page, which allows customers to view and pay an invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub hosted_invoice_url: Option<String>,
    #[doc = "Unique identifier for the object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The link to download the PDF for the invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub invoice_pdf: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<crate::accounts::ConnectAccountReference>,
    #[doc = "The error encountered during the previous attempt to finalize the invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last_finalization_error: Option<crate::common::ApiErrors>,
    #[doc = "The ID of the most recent non-draft revision of this invoice"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub latest_revision: Option<crate::Expandable<crate::invoices::Invoice>>,
    #[doc = "The individual line items that make up the invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub lines: Option<crate::List<crate::invoices::LineItem>>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The time at which payment will next be attempted."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub next_payment_attempt: Option<i64>,
    #[doc = "A unique, identifying string that appears on emails sent to the customer for this invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::invoices::InvoiceObject,
    #[doc = "The account (if any) for which the funds of the invoice payment are intended."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<crate::Expandable<crate::account::Account>>,
    #[doc = "The parent that generated this invoice"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<crate::billing::BillingBillResourceInvoicingParentsInvoiceParent>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_settings: Option<stripe_types_13::invoices::InvoicesPaymentSettings>,
    #[doc = "Payments for this invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payments: Option<crate::List<crate::invoice_payments::InvoicePayment>>,
    #[doc = "The latest timestamp at which invoice items can be associated with this invoice."]
    #[json(default)]
    pub period_end: i64,
    #[doc = "The earliest timestamp at which invoice items can be associated with this invoice."]
    #[json(default)]
    pub period_start: i64,
    #[doc = "Total amount of all post-payment credit notes issued for this invoice."]
    #[json(default)]
    pub post_payment_credit_notes_amount: i64,
    #[doc = "Total amount of all pre-payment credit notes issued for this invoice."]
    #[json(default)]
    pub pre_payment_credit_notes_amount: i64,
    #[doc = "This is the transaction number that appears on email receipts sent for this invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub receipt_number: Option<String>,
    #[doc = "The rendering-related settings that control how invoices render in customer-facing interfaces such as the PDF or host..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub rendering: Option<stripe_types_09::invoices::InvoicesResourceInvoiceRendering>,
    #[doc = "The details of the cost of shipping, including the ShippingRate applied on the invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<stripe_types_12::invoices::InvoicesResourceShippingCost>,
    #[doc = "Shipping details for the invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping_details: Option<stripe_types_05::common::Shipping>,
    #[doc = "Starting customer balance before the invoice is finalized."]
    #[json(default)]
    pub starting_balance: i64,
    #[doc = "Extra information about an invoice for the customer's credit card statement."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    #[doc = "The status of the invoice, one of `draft`, `open`, `paid`, `uncollectible`, or `void`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<stripe_types_04::invoices::InvoiceStatus>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<stripe_types_09::invoices::InvoicesResourceStatusDetails>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub status_transitions: Option<stripe_types_02::invoices::InvoicesResourceStatusTransitions>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub subscription: Option<crate::Expandable<crate::subscriptions::Subscription>>,
    #[doc = "Total of all subscriptions, invoice items, and prorations on the invoice before any invoice level discount or exclusi..."]
    #[json(default)]
    pub subtotal: i64,
    #[doc = "The integer amount in cents (or local equivalent) representing the subtotal of the invoice before any invoice level d..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub subtotal_excluding_tax: Option<i64>,
    #[doc = "ID of the test clock this invoice belongs to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<crate::Expandable<stripe_types_08::test_helpers::TestHelpersTestClock>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub threshold_reason: Option<stripe_types_05::invoices::InvoiceThresholdReason>,
    #[doc = "Total after discounts and taxes."]
    #[json(default)]
    pub total: i64,
    #[doc = "The aggregate amounts calculated per discount across all line items."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub total_discount_amounts: Option<Vec<crate::common::DiscountsResourceDiscountAmount>>,
    #[doc = "The integer amount in cents (or local equivalent) representing the total amount of the invoice including all discount..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub total_excluding_tax: Option<i64>,
    #[doc = "Contains pretax credit amounts (ex: discount, credit grants, etc) that apply to this invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub total_pretax_credit_amounts: Option<Vec<crate::invoices::InvoicesResourcePretaxCreditAmount>>,
    #[doc = "The aggregate tax information of all line items."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub total_taxes: Option<Vec<stripe_types_10::billing::BillingBillResourceInvoicingTaxesTax>>,
    #[doc = "Invoices are automatically paid or sent 1 hour after webhooks are delivered, or until all webhook delivery attempts h..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub webhooks_delivered_at: Option<i64>,
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
pub enum InvoiceAccountTaxIdsItemExpanded {
    /// Expanded `tax_id`.
    TaxId(Box<crate::tax_ids::TaxId>),
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
impl<'json> json::Decode<'json> for InvoiceAccountTaxIdsItemExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "tax_id" => {
                InvoiceAccountTaxIdsItemExpanded::TaxId(Box::new(json::from_slice_owned(&json::to_vec(&value))?))
            }
            _ => InvoiceAccountTaxIdsItemExpanded::Other(value),
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
pub enum InvoiceCustomerExpanded {
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
impl<'json> json::Decode<'json> for InvoiceCustomerExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "customer" => InvoiceCustomerExpanded::Customer(Box::new(json::from_slice_owned(&json::to_vec(&value))?)),
            _ => InvoiceCustomerExpanded::Other(value),
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
pub enum InvoiceDiscountsItemExpanded {
    /// Expanded `discount`.
    Discount(Box<crate::coupons::Discount>),
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
impl<'json> json::Decode<'json> for InvoiceDiscountsItemExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "discount" => {
                InvoiceDiscountsItemExpanded::Discount(Box::new(json::from_slice_owned(&json::to_vec(&value))?))
            }
            _ => InvoiceDiscountsItemExpanded::Other(value),
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
pub struct InvoiceSettingCustomerSetting {
    #[doc = "Default custom fields to be displayed on invoices for this customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<stripe_types_00::invoices::InvoiceSettingCustomField>>,
    #[doc = "ID of a payment method that's attached to the customer, to be used as the customer's default payment method for subsc..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<crate::Expandable<crate::payment_methods::PaymentMethod>>,
    #[doc = "Default footer to be displayed on invoices for this customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    #[doc = "Default options for invoice PDF rendering for this customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub rendering_options: Option<stripe_types_01::invoices::InvoiceSettingCustomerRenderingOptions>,
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
pub struct InvoiceSettingSubscriptionSchedulePhaseSetting {
    #[doc = "The account tax IDs associated with this phase of the subscription schedule."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<Vec<crate::Expandable<crate::invoices::InvoiceAccountTaxIdsItemExpanded>>>,
    #[doc = "A list of up to 4 custom fields to be displayed on the invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<stripe_types_00::invoices::InvoiceSettingCustomField>>,
    #[doc = "Number of days within which a customer must pay invoices generated by this subscription schedule."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<i64>,
    #[doc = "An arbitrary string attached to the object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Footer to be displayed on the invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    #[doc = "The connected account that issues the invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<crate::accounts::ConnectAccountReference>,
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
pub struct InvoiceSettingSubscriptionScheduleSetting {
    #[doc = "The account tax IDs associated with the subscription schedule."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<Vec<crate::Expandable<crate::invoices::InvoiceAccountTaxIdsItemExpanded>>>,
    #[doc = "A list of up to 4 custom fields to be displayed on the invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<stripe_types_00::invoices::InvoiceSettingCustomField>>,
    #[doc = "Number of days within which a customer must pay invoices generated by this subscription schedule."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<i64>,
    #[doc = "An arbitrary string attached to the object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Footer to be displayed on the invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<crate::accounts::ConnectAccountReference>,
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
pub struct InvoicesPaymentsInvoicePaymentAssociatedPayment {
    #[doc = "ID of the successful charge for this payment when `type` is `charge`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub charge: Option<crate::Expandable<crate::charges::Charge>>,
    #[doc = "ID of the PaymentIntent associated with this payment when `type` is `payment_intent`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<crate::Expandable<crate::payment_intents::PaymentIntent>>,
    #[doc = "ID of the PaymentRecord associated with this payment when `type` is `payment_record`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_record: Option<crate::Expandable<crate::payment_records::PaymentRecord>>,
    #[doc = "Type of payment object associated with this invoice payment."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::invoices::InvoicesPaymentsInvoicePaymentAssociatedPaymentType,
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
pub struct InvoicesResourceFromInvoice {
    #[doc = "The relation between this invoice and the cloned invoice"]
    #[json(default)]
    pub action: String,
    #[doc = "The invoice that was cloned."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub invoice: Option<crate::Expandable<crate::invoices::Invoice>>,
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
pub struct InvoicesResourcePretaxCreditAmount {
    #[doc = "The amount, in cents (or local equivalent), of the pretax credit amount."]
    #[json(default)]
    pub amount: i64,
    #[doc = "The credit balance transaction that was applied to get this pretax credit amount."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub credit_balance_transaction: Option<crate::Expandable<crate::billing::BillingCreditBalanceTransaction>>,
    #[doc = "The discount that was applied to get this pretax credit amount."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub discount: Option<crate::Expandable<crate::invoices::InvoiceDiscountsItemExpanded>>,
    #[doc = "Type of the pretax credit amount referenced."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::invoices::InvoicesResourcePretaxCreditAmountType,
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
pub struct LineItem {
    #[doc = "The amount, in cents (or local equivalent)."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "An arbitrary string attached to the object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The amount of discount calculated per discount for this line item."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub discount_amounts: Option<Vec<crate::common::DiscountsResourceDiscountAmount>>,
    #[doc = "If true, discounts will apply to this line item."]
    #[json(default)]
    pub discountable: bool,
    #[doc = "The discounts applied to the invoice line item."]
    #[json(default)]
    pub discounts: Vec<crate::Expandable<crate::coupons::Discount>>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "The ID of the invoice that contains this line item."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub invoice: Option<String>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::invoices::LineItemObject,
    #[doc = "The parent that generated this line item."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<stripe_types_10::billing::BillingBillResourceInvoicingLinesParentsInvoiceLineItemParent>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<stripe_types_00::invoices::InvoiceLineItemPeriod>,
    #[doc = "Contains pretax credit amounts (ex: discount, credit grants, etc) that apply to this line item."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pretax_credit_amounts: Option<Vec<crate::invoices::InvoicesResourcePretaxCreditAmount>>,
    #[doc = "The pricing information of the line item."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pricing: Option<stripe_types_13::billing::BillingBillResourceInvoicingPricingPricing>,
    #[doc = "Quantity of units for the invoice line item in integer format, with any decimal precision truncated."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[doc = "Non-negative decimal with at most 12 decimal places."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub quantity_decimal: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub subscription: Option<crate::Expandable<crate::subscriptions::Subscription>>,
    #[doc = "The subtotal of the line item, in cents (or local equivalent), before any discounts or taxes."]
    #[json(default)]
    pub subtotal: i64,
    #[doc = "The tax information of the line item."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub taxes: Option<Vec<stripe_types_10::billing::BillingBillResourceInvoicingTaxesTax>>,
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
impl crate::PagedItem for LineItem {
    fn id(&self) -> &str {
        &self.id
    }
}
