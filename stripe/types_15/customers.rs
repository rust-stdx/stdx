#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerBalanceTransaction {
    #[doc = "The amount of the transaction."]
    #[json(default)]
    pub amount: i64,
    #[doc = "The ID of the checkout session (if any) that created the transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub checkout_session: Option<crate::Expandable<crate::checkout::CheckoutSession>>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "The ID of the credit note (if any) related to the transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub credit_note: Option<crate::Expandable<crate::credit_notes::CreditNote>>,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "The ID of the customer the transaction belongs to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<crate::Expandable<stripe_types_14::customers::Customer>>,
    #[doc = "The ID of an Account representing a customer that the transaction belongs to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_account: Option<String>,
    #[doc = "An arbitrary string attached to the object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The customer's `balance` after the transaction was applied."]
    #[json(default)]
    pub ending_balance: i64,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "The ID of the invoice (if any) related to the transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub invoice: Option<crate::Expandable<stripe_types_14::invoices::Invoice>>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_01::customers::CustomerBalanceTransactionObject,
    #[doc = "Transaction type: `adjustment`, `applied_to_invoice`, `credit_note`, `initial`, `invoice_overpaid`, `invoice_too_larg..."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::customers::CustomerBalanceTransactionType,
}

#[cfg(any(feature = "credit_notes", feature = "customers", feature = "events"))]
impl crate::PagedItem for CustomerBalanceTransaction {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "customers")]
#[derive(Debug, Clone)]
pub enum DeleteCustomerSourceResponseExpanded {
    /// Expanded `payment_source`.
    PaymentSource(Box<stripe_types_14::common::PaymentSource>),
    /// Expanded `deleted_payment_source`.
    DeletedPaymentSource(Box<stripe_types_08::common::DeletedPaymentSource>),
    /// An object type not known to this version of the crate.
    Other(json::OwnedValue),
}

#[cfg(feature = "customers")]
impl<'json> json::Decode<'json> for DeleteCustomerSourceResponseExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "payment_source" => DeleteCustomerSourceResponseExpanded::PaymentSource(Box::new(json::from_slice_owned(
                &json::to_vec(&value),
            )?)),
            "deleted_payment_source" => DeleteCustomerSourceResponseExpanded::DeletedPaymentSource(Box::new(
                json::from_slice_owned(&json::to_vec(&value))?,
            )),
            _ => DeleteCustomerSourceResponseExpanded::Other(value),
        })
    }
}

#[cfg(feature = "customers")]
#[derive(Debug, Clone)]
pub enum GetCustomerResponseExpanded {
    /// Expanded `customer`.
    Customer(Box<stripe_types_14::customers::Customer>),
    /// An object type not known to this version of the crate.
    Other(json::OwnedValue),
}

#[cfg(feature = "customers")]
impl<'json> json::Decode<'json> for GetCustomerResponseExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "customer" => {
                GetCustomerResponseExpanded::Customer(Box::new(json::from_slice_owned(&json::to_vec(&value))?))
            }
            _ => GetCustomerResponseExpanded::Other(value),
        })
    }
}

#[cfg(feature = "customers")]
#[derive(Debug, Clone)]
pub enum UpdateCustomerSourceResponseExpanded {
    /// Expanded `card`.
    Card(Box<stripe_types_14::payment_methods::Card>),
    /// Expanded `bank_account`.
    BankAccount(Box<stripe_types_14::payment_methods::BankAccount>),
    /// Expanded `source`.
    Source(Box<stripe_types_10::sources::Source>),
    /// An object type not known to this version of the crate.
    Other(json::OwnedValue),
}

#[cfg(feature = "customers")]
impl<'json> json::Decode<'json> for UpdateCustomerSourceResponseExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "card" => {
                UpdateCustomerSourceResponseExpanded::Card(Box::new(json::from_slice_owned(&json::to_vec(&value))?))
            }
            "bank_account" => UpdateCustomerSourceResponseExpanded::BankAccount(Box::new(json::from_slice_owned(
                &json::to_vec(&value),
            )?)),
            "source" => {
                UpdateCustomerSourceResponseExpanded::Source(Box::new(json::from_slice_owned(&json::to_vec(&value))?))
            }
            _ => UpdateCustomerSourceResponseExpanded::Other(value),
        })
    }
}
