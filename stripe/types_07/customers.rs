#[cfg(feature = "customers")]
#[derive(Debug, Clone)]
pub struct CreateCustomerParamsShipping {
    #[doc = "Customer shipping address."]
    pub address: stripe_types_05::customers::CreateCustomerParamsShippingAddress,
    #[doc = "Customer name."]
    pub name: String,
    #[doc = "Customer phone (including extension)."]
    pub phone: Option<String>,
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for CreateCustomerParamsShipping {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("address", &self.address);
        form.field("name", &self.name);
        form.field_opt("phone", self.phone.as_ref());
    }
}

#[cfg(feature = "customers")]
#[derive(Debug, Clone)]
pub struct CreateCustomerTaxIdParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Type of the tax ID, one of `ad_nrt`, `ae_trn`, `al_tin`, `am_tin`, `ao_tin`, `ar_cuit`, `au_abn`, `au_arn`, `aw_tin`,..."]
    pub r#type: stripe_types_06::customers::CreateCustomerTaxIdParamsType,
    #[doc = "Value of the tax ID."]
    pub value: String,
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for CreateCustomerTaxIdParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("type", &self.r#type);
        form.field("value", &self.value);
    }
}

#[cfg(feature = "customers")]
#[derive(Debug, Clone, Default)]
pub struct ListCustomerBalanceTransactionsParams {
    pub created: Option<stripe_types_06::customers::ListCustomerBalanceTransactionsParamsCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub invoice: Option<String>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for ListCustomerBalanceTransactionsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("created", self.created.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("invoice", self.invoice.as_ref());
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}

#[cfg(feature = "customers")]
#[derive(Debug, Clone, Default)]
pub struct UpdateCustomerCashBalanceParamsSettings {
    #[doc = "Controls how funds transferred by the customer are applied to payment intents and invoices."]
    pub reconciliation_mode:
        Option<stripe_types_05::customers::UpdateCustomerCashBalanceParamsSettingsReconciliationMode>,
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for UpdateCustomerCashBalanceParamsSettings {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("reconciliation_mode", self.reconciliation_mode.as_ref());
    }
}

#[cfg(feature = "customers")]
#[derive(Debug, Clone, Default)]
pub struct UpdateCustomerParamsInvoiceSettingsRenderingOptions {
    #[doc = "How line-item prices and amounts will be displayed with respect to tax on invoice PDFs."]
    pub amount_tax_display:
        Option<stripe_types_05::customers::UpdateCustomerParamsInvoiceSettingsRenderingOptionsAmountTaxDisplay>,
    #[doc = "ID of the invoice rendering template to use for future invoices."]
    pub template: Option<String>,
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for UpdateCustomerParamsInvoiceSettingsRenderingOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount_tax_display", self.amount_tax_display.as_ref());
        form.field_opt("template", self.template.as_ref());
    }
}
