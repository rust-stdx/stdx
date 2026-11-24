#[cfg(any(feature = "events", feature = "invoiceitems"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct Invoiceitem {
    #[doc = "Amount (in the `currency` specified) of the invoice item."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "The ID of the customer to bill for this invoice item."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<crate::Expandable<crate::invoiceitems::InvoiceitemCustomerExpanded>>,
    #[doc = "The ID of the account to bill for this invoice item."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_account: Option<String>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub date: i64,
    #[doc = "An arbitrary string attached to the object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "If true, discounts will apply to this invoice item."]
    #[json(default)]
    pub discountable: bool,
    #[doc = "The discounts which apply to the invoice item."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<crate::Expandable<stripe_types_14::coupons::Discount>>>,
    #[doc = "Array of field names that can't be modified."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub frozen_fields: Option<Vec<stripe_types_02::invoiceitems::InvoiceitemFrozenFieldsItem>>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "The ID of the invoice this invoice item belongs to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub invoice: Option<crate::Expandable<stripe_types_14::invoices::Invoice>>,
    #[doc = "The rules that control when this invoice item is eligible for invoicing."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub invoicing_rules: Option<Vec<stripe_types_05::invoices::InvoicingRule>>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The amount after discounts, but before credits and taxes."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<i64>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::invoiceitems::InvoiceitemObject,
    #[doc = "The parent that generated this invoice item."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<stripe_types_05::billing::BillingBillResourceInvoiceItemParentsInvoiceItemParent>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<stripe_types_00::invoices::InvoiceLineItemPeriod>,
    #[doc = "The pricing information of the invoice item."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pricing: Option<stripe_types_13::billing::BillingBillResourceInvoicingPricingPricing>,
    #[doc = "Whether the invoice item was created automatically as a proration adjustment when the customer switched plans."]
    #[json(default)]
    pub proration: bool,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub proration_details: Option<crate::invoices::ProrationDetails>,
    #[doc = "Quantity of units for the invoice item in integer format, with any decimal precision truncated."]
    #[json(default)]
    pub quantity: i64,
    #[doc = "Non-negative decimal with at most 12 decimal places."]
    #[json(default)]
    pub quantity_decimal: String,
    #[doc = "The tax rates which apply to the invoice item."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<stripe_types_06::tax_rates::TaxRate>>,
    #[doc = "ID of the test clock this invoice item belongs to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<crate::Expandable<stripe_types_08::test_helpers::TestHelpersTestClock>>,
}

#[cfg(any(feature = "events", feature = "invoiceitems"))]
impl crate::PagedItem for Invoiceitem {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(any(feature = "events", feature = "invoiceitems"))]
#[derive(Debug, Clone)]
pub enum InvoiceitemCustomerExpanded {
    /// Expanded `customer`.
    Customer(Box<stripe_types_14::customers::Customer>),
    /// An object type not known to this version of the crate.
    Other(json::OwnedValue),
}

#[cfg(any(feature = "events", feature = "invoiceitems"))]
impl<'json> json::Decode<'json> for InvoiceitemCustomerExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "customer" => {
                InvoiceitemCustomerExpanded::Customer(Box::new(json::from_slice_owned(&json::to_vec(&value))?))
            }
            _ => InvoiceitemCustomerExpanded::Other(value),
        })
    }
}
