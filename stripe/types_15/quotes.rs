#[cfg(any(feature = "events", feature = "quotes"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct Quote {
    #[doc = "Total before any discounts or taxes are applied."]
    #[json(default)]
    pub amount_subtotal: i64,
    #[doc = "Total after discounts and taxes are applied."]
    #[json(default)]
    pub amount_total: i64,
    #[doc = "ID of the Connect Application that created the quote."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<crate::Expandable<stripe_types_08::quotes::QuoteApplicationExpanded>>,
    #[doc = "The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    #[doc = "A non-negative decimal between 0 and 100, with at most two decimal places."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<crate::quotes::QuotesResourceAutomaticTax>,
    #[doc = "Either `charge_automatically`, or `send_invoice`."]
    #[json(default)]
    pub collection_method: stripe_types_02::quotes::QuoteCollectionMethod,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub computed: Option<crate::quotes::QuotesResourceComputed>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "The customer who received this quote."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<crate::Expandable<crate::quotes::QuoteCustomerExpanded>>,
    #[doc = "The account representing the customer who received this quote."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_account: Option<String>,
    #[doc = "The tax rates applied to this quote."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<crate::Expandable<stripe_types_06::tax_rates::TaxRate>>>,
    #[doc = "A description that will be displayed on the quote PDF."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The discounts applied to this quote."]
    #[json(default)]
    pub discounts: Vec<crate::Expandable<stripe_types_14::coupons::Discount>>,
    #[doc = "The date on which the quote will be canceled if in `open` or `draft` status."]
    #[json(default)]
    pub expires_at: i64,
    #[doc = "A footer that will be displayed on the quote PDF."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    #[doc = "Details of the quote that was cloned."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub from_quote: Option<crate::quotes::QuotesResourceFromQuote>,
    #[doc = "A header that will be displayed on the quote PDF."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "The invoice that was created from this quote."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub invoice: Option<crate::Expandable<crate::quotes::QuoteInvoiceExpanded>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<crate::invoices::InvoiceSettingQuoteSetting>,
    #[doc = "A list of items the customer is being quoted for."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub line_items: Option<crate::List<crate::common::Item>>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "A unique number that identifies this particular quote."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::quotes::QuoteObject,
    #[doc = "The account on behalf of which to charge."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<crate::Expandable<stripe_types_14::account::Account>>,
    #[doc = "The status of the quote."]
    #[json(default)]
    pub status: stripe_types_00::quotes::QuoteStatus,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub status_transitions: Option<stripe_types_01::quotes::QuotesResourceStatusTransitions>,
    #[doc = "The subscription that was created or updated from this quote."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub subscription: Option<crate::Expandable<stripe_types_14::subscriptions::Subscription>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub subscription_data: Option<stripe_types_10::quotes::QuotesResourceSubscriptionDataSubscriptionData>,
    #[doc = "The subscription schedule that was created or updated from this quote."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub subscription_schedule: Option<crate::Expandable<stripe_types_14::subscription_schedules::SubscriptionSchedule>>,
    #[doc = "ID of the test clock this quote belongs to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<crate::Expandable<stripe_types_08::test_helpers::TestHelpersTestClock>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub total_details: Option<crate::quotes::QuotesResourceTotalDetails>,
    #[doc = "The account (if any) the payments will be attributed to for tax reporting, and where funds from each payment will be ..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<crate::quotes::QuotesResourceTransferData>,
}

#[cfg(any(feature = "events", feature = "quotes"))]
impl crate::PagedItem for Quote {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(any(feature = "events", feature = "quotes"))]
#[derive(Debug, Clone)]
pub enum QuoteCustomerExpanded {
    /// Expanded `customer`.
    Customer(Box<stripe_types_14::customers::Customer>),
    /// An object type not known to this version of the crate.
    Other(json::OwnedValue),
}

#[cfg(any(feature = "events", feature = "quotes"))]
impl<'json> json::Decode<'json> for QuoteCustomerExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "customer" => QuoteCustomerExpanded::Customer(Box::new(json::from_slice_owned(&json::to_vec(&value))?)),
            _ => QuoteCustomerExpanded::Other(value),
        })
    }
}

#[cfg(any(feature = "events", feature = "quotes"))]
#[derive(Debug, Clone)]
pub enum QuoteInvoiceExpanded {
    /// Expanded `invoice`.
    Invoice(Box<stripe_types_14::invoices::Invoice>),
    /// An object type not known to this version of the crate.
    Other(json::OwnedValue),
}

#[cfg(any(feature = "events", feature = "quotes"))]
impl<'json> json::Decode<'json> for QuoteInvoiceExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "invoice" => QuoteInvoiceExpanded::Invoice(Box::new(json::from_slice_owned(&json::to_vec(&value))?)),
            _ => QuoteInvoiceExpanded::Other(value),
        })
    }
}

#[cfg(any(feature = "events", feature = "quotes"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct QuotesResourceAutomaticTax {
    #[doc = "Automatically calculate taxes"]
    #[json(default)]
    pub enabled: bool,
    #[doc = "The account that's liable for tax."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub liability: Option<stripe_types_14::accounts::ConnectAccountReference>,
    #[doc = "The tax provider powering automatic tax."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "The status of the most recent automated tax calculation for this quote."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<stripe_types_02::quotes::QuotesResourceAutomaticTaxStatus>,
}

#[cfg(any(feature = "events", feature = "quotes"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct QuotesResourceComputed {
    #[doc = "The definitive totals and line items the customer will be charged on a recurring basis."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub recurring: Option<crate::quotes::QuotesResourceRecurring>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub upfront: Option<crate::quotes::QuotesResourceUpfront>,
}

#[cfg(any(feature = "events", feature = "quotes"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct QuotesResourceFromQuote {
    #[doc = "Whether this quote is a revision of a different quote."]
    #[json(default)]
    pub is_revision: bool,
    #[doc = "The quote that was cloned."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub quote: Option<crate::Expandable<crate::quotes::Quote>>,
}

#[cfg(any(feature = "events", feature = "quotes"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct QuotesResourceRecurring {
    #[doc = "Total before any discounts or taxes are applied."]
    #[json(default)]
    pub amount_subtotal: i64,
    #[doc = "Total after discounts and taxes are applied."]
    #[json(default)]
    pub amount_total: i64,
    #[doc = "The frequency at which a subscription is billed."]
    #[json(default)]
    pub interval: stripe_types_00::quotes::QuotesResourceRecurringInterval,
    #[doc = "The number of intervals (specified in the `interval` attribute) between subscription billings."]
    #[json(default)]
    pub interval_count: i64,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub total_details: Option<crate::quotes::QuotesResourceTotalDetails>,
}

#[cfg(any(feature = "events", feature = "quotes"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct QuotesResourceTotalDetails {
    #[doc = "This is the sum of all the discounts."]
    #[json(default)]
    pub amount_discount: i64,
    #[doc = "This is the sum of all the shipping amounts."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_shipping: Option<i64>,
    #[doc = "This is the sum of all the tax amounts."]
    #[json(default)]
    pub amount_tax: i64,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub breakdown: Option<crate::quotes::QuotesResourceTotalDetailsResourceBreakdown>,
}

#[cfg(any(feature = "events", feature = "quotes"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct QuotesResourceTotalDetailsResourceBreakdown {
    #[doc = "The aggregated discounts."]
    #[json(default)]
    pub discounts: Vec<crate::common::LineItemsDiscountAmount>,
    #[doc = "The aggregated tax amounts by rate."]
    #[json(default)]
    pub taxes: Vec<stripe_types_07::common::LineItemsTaxAmount>,
}

#[cfg(any(feature = "events", feature = "quotes"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct QuotesResourceTransferData {
    #[doc = "The amount in cents (or local equivalent) that will be transferred to the destination account when the invoice is paid."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[doc = "A non-negative decimal between 0 and 100, with at most two decimal places."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,
    #[doc = "The account where funds from the payment will be transferred to upon payment success."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<crate::Expandable<stripe_types_14::account::Account>>,
}

#[cfg(any(feature = "events", feature = "quotes"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct QuotesResourceUpfront {
    #[doc = "Total before any discounts or taxes are applied."]
    #[json(default)]
    pub amount_subtotal: i64,
    #[doc = "Total after discounts and taxes are applied."]
    #[json(default)]
    pub amount_total: i64,
    #[doc = "The line items that will appear on the next invoice after this quote is accepted."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub line_items: Option<crate::List<crate::common::Item>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub total_details: Option<crate::quotes::QuotesResourceTotalDetails>,
}
