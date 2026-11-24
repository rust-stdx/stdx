#[cfg(feature = "customers")]
#[derive(Debug, Clone, Default)]
pub struct UpdateCustomerParams {
    #[doc = "The customer's address."]
    pub address: Option<stripe_types_06::customers::UpdateCustomerParamsAddress>,
    #[doc = "An integer amount in cents (or local equivalent) that represents the customer's current balance, which affect the cus..."]
    pub balance: Option<i64>,
    #[doc = "The customer's business name."]
    pub business_name: Option<json::OwnedValue>,
    #[doc = "Balance information and default balance settings for this customer."]
    pub cash_balance: Option<stripe_types_10::customers::UpdateCustomerParamsCashBalance>,
    #[doc = "If you are using payment methods created via the PaymentMethods API, see the invoice_settings.default_payment_method..."]
    pub default_source: Option<String>,
    #[doc = "An arbitrary string that you can attach to a customer object."]
    pub description: Option<String>,
    #[doc = "Customer's email address."]
    pub email: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The customer's full name."]
    pub individual_name: Option<json::OwnedValue>,
    #[doc = "The prefix for the customer used to generate unique invoice numbers."]
    pub invoice_prefix: Option<String>,
    #[doc = "Default invoice settings for this customer."]
    pub invoice_settings: Option<stripe_types_10::customers::UpdateCustomerParamsInvoiceSettings>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The customer's full name or business name."]
    pub name: Option<String>,
    #[doc = "The sequence to be used on the customer's next invoice."]
    pub next_invoice_sequence: Option<i64>,
    #[doc = "The customer's phone number."]
    pub phone: Option<String>,
    #[doc = "Customer's preferred languages, ordered by preference."]
    pub preferred_locales: Option<Vec<String>>,
    #[doc = "The customer's shipping information."]
    pub shipping: Option<stripe_types_08::customers::UpdateCustomerParamsShipping>,
    pub source: Option<String>,
    #[doc = "Tax details about the customer."]
    pub tax: Option<stripe_types_06::customers::UpdateCustomerParamsTax>,
    #[doc = "The customer's tax exemption."]
    pub tax_exempt: Option<stripe_types_05::customers::UpdateCustomerParamsTaxExempt>,
    pub validate: Option<bool>,
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for UpdateCustomerParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("address", self.address.as_ref());
        form.field_opt("balance", self.balance.as_ref());
        form.field_opt("cash_balance", self.cash_balance.as_ref());
        form.field_opt("default_source", self.default_source.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field_opt("email", self.email.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("invoice_prefix", self.invoice_prefix.as_ref());
        form.field_opt("invoice_settings", self.invoice_settings.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("name", self.name.as_ref());
        form.field_opt("next_invoice_sequence", self.next_invoice_sequence.as_ref());
        form.field_opt("phone", self.phone.as_ref());
        if let Some(values) = self.preferred_locales.as_ref() {
            form.field_array("preferred_locales", values.as_slice());
        }
        form.field_opt("shipping", self.shipping.as_ref());
        form.field_opt("source", self.source.as_ref());
        form.field_opt("tax", self.tax.as_ref());
        form.field_opt("tax_exempt", self.tax_exempt.as_ref());
        form.field_opt("validate", self.validate.as_ref());
    }
}
