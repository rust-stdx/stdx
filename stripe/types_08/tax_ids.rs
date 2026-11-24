#[cfg(feature = "tax_ids")]
#[derive(Debug, Clone)]
pub struct CreateTaxIdParamsOwner {
    #[doc = "Connected Account the tax ID belongs to."]
    pub account: Option<String>,
    #[doc = "Customer the tax ID belongs to."]
    pub customer: Option<String>,
    #[doc = "ID of the Account representing the customer that the tax ID belongs to."]
    pub customer_account: Option<String>,
    #[doc = "Type of owner referenced."]
    pub r#type: stripe_types_05::tax_ids::CreateTaxIdParamsOwnerType,
}

#[cfg(feature = "tax_ids")]
impl crate::params::FormEncode for CreateTaxIdParamsOwner {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("customer_account", self.customer_account.as_ref());
        form.field("type", &self.r#type);
    }
}
