#[cfg(feature = "quotes")]
#[derive(Debug, Clone, Default)]
pub struct UpdateQuoteParams {
    #[doc = "The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the..."]
    pub application_fee_amount: Option<json::OwnedValue>,
    #[doc = "A non-negative decimal between 0 and 100, with at most two decimal places."]
    pub application_fee_percent: Option<json::OwnedValue>,
    #[doc = "Settings for automatic tax lookup for this quote and resulting invoices and subscriptions."]
    pub automatic_tax: Option<stripe_types_11::quotes::UpdateQuoteParamsAutomaticTax>,
    #[doc = "Either `charge_automatically`, or `send_invoice`."]
    pub collection_method: Option<stripe_types_06::quotes::UpdateQuoteParamsCollectionMethod>,
    #[doc = "The customer for which this quote belongs to."]
    pub customer: Option<String>,
    #[doc = "The account for which this quote belongs to."]
    pub customer_account: Option<String>,
    #[doc = "The tax rates that will apply to any line item that does not have `tax_rates` set."]
    pub default_tax_rates: Option<json::OwnedValue>,
    #[doc = "A description that will be displayed on the quote PDF."]
    pub description: Option<json::OwnedValue>,
    #[doc = "The discounts applied to the quote."]
    pub discounts: Option<json::OwnedValue>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "A future timestamp on which the quote will be canceled if in `open` or `draft` status."]
    pub expires_at: Option<i64>,
    #[doc = "A footer that will be displayed on the quote PDF."]
    pub footer: Option<json::OwnedValue>,
    #[doc = "A header that will be displayed on the quote PDF."]
    pub header: Option<json::OwnedValue>,
    #[doc = "All invoices will be billed using the specified settings."]
    pub invoice_settings: Option<stripe_types_10::quotes::UpdateQuoteParamsInvoiceSettings>,
    #[doc = "A list of line items the customer is being quoted for."]
    pub line_items: Option<Vec<stripe_types_12::quotes::UpdateQuoteParamsLineItemsItem>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The account on behalf of which to charge."]
    pub on_behalf_of: Option<json::OwnedValue>,
    #[doc = "When creating a subscription or subscription schedule, the specified configuration data will be used."]
    pub subscription_data: Option<stripe_types_04::quotes::UpdateQuoteParamsSubscriptionData>,
    #[doc = "The data with which to automatically create a Transfer for each of the invoices."]
    pub transfer_data: Option<stripe_types_05::quotes::UpdateQuoteParamsTransferData>,
}

#[cfg(feature = "quotes")]
impl crate::params::FormEncode for UpdateQuoteParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("automatic_tax", self.automatic_tax.as_ref());
        form.field_opt("collection_method", self.collection_method.as_ref());
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("customer_account", self.customer_account.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("expires_at", self.expires_at.as_ref());
        form.field_opt("invoice_settings", self.invoice_settings.as_ref());
        if let Some(values) = self.line_items.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("line_items", index, item);
            }
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("subscription_data", self.subscription_data.as_ref());
        form.field_opt("transfer_data", self.transfer_data.as_ref());
    }
}
