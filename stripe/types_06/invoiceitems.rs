#[cfg(feature = "invoiceitems")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct DeletedInvoiceitem {
    #[doc = "Always true for a deleted object"]
    #[json(default)]
    pub deleted: bool,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::invoiceitems::InvoiceitemObject,
}

#[cfg(feature = "invoiceitems")]
impl crate::PagedItem for DeletedInvoiceitem {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "invoiceitems")]
#[derive(Debug, Clone, Default)]
pub struct ListInvoiceitemsParams {
    pub created: Option<stripe_types_00::invoiceitems::ListInvoiceitemsParamsCreated>,
    pub customer: Option<String>,
    pub customer_account: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub invoice: Option<String>,
    pub limit: Option<i64>,
    pub pending: Option<bool>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "invoiceitems")]
impl crate::params::FormEncode for ListInvoiceitemsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("created", self.created.as_ref());
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("customer_account", self.customer_account.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("invoice", self.invoice.as_ref());
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("pending", self.pending.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}

#[cfg(feature = "invoiceitems")]
pub type UpdateInvoiceitemParamsPriceDataTaxBehavior =
    stripe_types_03::invoiceitems::CreateInvoiceitemParamsPriceDataTaxBehavior;
