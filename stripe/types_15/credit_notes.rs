#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CreditNote {
    #[doc = "The integer amount in cents (or local equivalent) representing the total amount of the credit note, including tax."]
    #[json(default)]
    pub amount: i64,
    #[doc = "This is the sum of all the shipping amounts."]
    #[json(default)]
    pub amount_shipping: i64,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "ID of the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<crate::Expandable<crate::credit_notes::CreditNoteCustomerExpanded>>,
    #[doc = "ID of the account representing the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_account: Option<String>,
    #[doc = "Customer balance transaction related to this credit note."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_balance_transaction: Option<crate::Expandable<crate::customers::CustomerBalanceTransaction>>,
    #[doc = "The integer amount in cents (or local equivalent) representing the total amount of discount that was credited."]
    #[json(default)]
    pub discount_amount: i64,
    #[doc = "The aggregate amounts calculated per discount for all line items."]
    #[json(default)]
    pub discount_amounts: Vec<stripe_types_14::common::DiscountsResourceDiscountAmount>,
    #[doc = "The date when this credit note is in effect."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub effective_at: Option<i64>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "ID of the invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub invoice: Option<crate::Expandable<stripe_types_14::invoices::Invoice>>,
    #[doc = "Line items that make up the credit note"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub lines: Option<crate::List<crate::credit_notes::CreditNoteLineItem>>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Customer-facing text that appears on the credit note PDF."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "A unique number that identifies this particular credit note and appears on the PDF of the credit note and its associa..."]
    #[json(default)]
    pub number: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::credit_notes::CreditNoteObject,
    #[doc = "Amount that was credited outside of Stripe."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub out_of_band_amount: Option<i64>,
    #[doc = "The link to download the PDF of the credit note."]
    #[json(default)]
    pub pdf: String,
    #[doc = "The amount of the credit note that was refunded to the customer, credited to the customer's balance, credited outside..."]
    #[json(default)]
    pub post_payment_amount: i64,
    #[doc = "The amount of the credit note by which the invoice's `amount_remaining` and `amount_due` were reduced."]
    #[json(default)]
    pub pre_payment_amount: i64,
    #[doc = "The pretax credit amounts (ex: discount, credit grants, etc) for all line items."]
    #[json(default)]
    pub pretax_credit_amounts: Vec<crate::credit_notes::CreditNotesPretaxCreditAmount>,
    #[doc = "Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<stripe_types_03::credit_notes::CreditNoteReason>,
    #[doc = "Refunds related to this credit note."]
    #[json(default)]
    pub refunds: Vec<crate::credit_notes::CreditNoteRefund>,
    #[doc = "The details of the cost of shipping, including the ShippingRate applied to the invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<stripe_types_12::invoices::InvoicesResourceShippingCost>,
    #[doc = "Status of this credit note, one of `issued` or `void`."]
    #[json(default)]
    pub status: stripe_types_03::credit_notes::CreditNoteStatus,
    #[doc = "The integer amount in cents (or local equivalent) representing the amount of the credit note, excluding exclusive tax..."]
    #[json(default)]
    pub subtotal: i64,
    #[doc = "The integer amount in cents (or local equivalent) representing the amount of the credit note, excluding all tax and i..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub subtotal_excluding_tax: Option<i64>,
    #[doc = "The integer amount in cents (or local equivalent) representing the total amount of the credit note, including tax and..."]
    #[json(default)]
    pub total: i64,
    #[doc = "The integer amount in cents (or local equivalent) representing the total amount of the credit note, excluding tax, bu..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub total_excluding_tax: Option<i64>,
    #[doc = "The aggregate tax information for all line items."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub total_taxes: Option<Vec<stripe_types_10::billing::BillingBillResourceInvoicingTaxesTax>>,
    #[doc = "Type of this credit note, one of `pre_payment` or `post_payment`."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::credit_notes::CreditNoteType,
    #[doc = "The time that the credit note was voided."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub voided_at: Option<i64>,
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
impl crate::PagedItem for CreditNote {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
#[derive(Debug, Clone)]
pub enum CreditNoteCustomerExpanded {
    /// Expanded `customer`.
    Customer(Box<stripe_types_14::customers::Customer>),
    /// An object type not known to this version of the crate.
    Other(json::OwnedValue),
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
impl<'json> json::Decode<'json> for CreditNoteCustomerExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "customer" => {
                CreditNoteCustomerExpanded::Customer(Box::new(json::from_slice_owned(&json::to_vec(&value))?))
            }
            _ => CreditNoteCustomerExpanded::Other(value),
        })
    }
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CreditNoteLineItem {
    #[doc = "The integer amount in cents (or local equivalent) representing the gross amount being credited for this line item, ex..."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Description of the item being credited."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The integer amount in cents (or local equivalent) representing the discount being credited for this line item."]
    #[json(default)]
    pub discount_amount: i64,
    #[doc = "The amount of discount calculated per discount for this line item"]
    #[json(default)]
    pub discount_amounts: Vec<stripe_types_14::common::DiscountsResourceDiscountAmount>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "ID of the invoice line item being credited"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub invoice_line_item: Option<String>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::credit_notes::CreditNoteLineItemObject,
    #[doc = "The pretax credit amounts (ex: discount, credit grants, etc) for this line item."]
    #[json(default)]
    pub pretax_credit_amounts: Vec<crate::credit_notes::CreditNotesPretaxCreditAmount>,
    #[doc = "The number of units of product being credited."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[doc = "The tax rates which apply to the line item."]
    #[json(default)]
    pub tax_rates: Vec<stripe_types_06::tax_rates::TaxRate>,
    #[doc = "The tax information of the line item."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub taxes: Option<Vec<stripe_types_10::billing::BillingBillResourceInvoicingTaxesTax>>,
    #[doc = "The type of the credit note line item, one of `invoice_line_item` or `custom_line_item`."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::credit_notes::CreditNoteLineItemType,
    #[doc = "The cost of each unit of product being credited."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    #[doc = "Same as `unit_amount`, but contains a decimal value with at most 12 decimal places."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
impl crate::PagedItem for CreditNoteLineItem {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CreditNoteRefund {
    #[doc = "Amount of the refund that applies to this credit note, in cents (or local equivalent)."]
    #[json(default)]
    pub amount_refunded: i64,
    #[doc = "The PaymentRecord refund details associated with this credit note refund."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_record_refund: Option<stripe_types_00::credit_notes::CreditNotesPaymentRecordRefund>,
    #[doc = "ID of the refund."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refund: Option<crate::Expandable<stripe_types_14::refunds::Refund>>,
    #[doc = "Type of the refund, one of `refund` or `payment_record_refund`."]
    #[json(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<stripe_types_01::credit_notes::CreditNoteRefundType>,
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CreditNotesPretaxCreditAmount {
    #[doc = "The amount, in cents (or local equivalent), of the pretax credit amount."]
    #[json(default)]
    pub amount: i64,
    #[doc = "The credit balance transaction that was applied to get this pretax credit amount."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub credit_balance_transaction:
        Option<crate::Expandable<stripe_types_14::billing::BillingCreditBalanceTransaction>>,
    #[doc = "The discount that was applied to get this pretax credit amount."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub discount: Option<crate::Expandable<crate::credit_notes::CreditNotesPretaxCreditAmountDiscountExpanded>>,
    #[doc = "Type of the pretax credit amount referenced."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::credit_notes::CreditNotesPretaxCreditAmountType,
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
#[derive(Debug, Clone)]
pub enum CreditNotesPretaxCreditAmountDiscountExpanded {
    /// Expanded `discount`.
    Discount(Box<stripe_types_14::coupons::Discount>),
    /// An object type not known to this version of the crate.
    Other(json::OwnedValue),
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
impl<'json> json::Decode<'json> for CreditNotesPretaxCreditAmountDiscountExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "discount" => CreditNotesPretaxCreditAmountDiscountExpanded::Discount(Box::new(json::from_slice_owned(
                &json::to_vec(&value),
            )?)),
            _ => CreditNotesPretaxCreditAmountDiscountExpanded::Other(value),
        })
    }
}
