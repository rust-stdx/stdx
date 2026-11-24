#[cfg(feature = "customers")]
#[derive(Debug, Clone)]
pub struct CreateCustomerFundingInstructionParams {
    #[doc = "Additional parameters for `bank_transfer` funding types"]
    pub bank_transfer: stripe_types_06::customers::CreateCustomerFundingInstructionParamsBankTransfer,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The `funding_type` to get the instructions for."]
    pub funding_type: stripe_types_04::customers::CreateCustomerFundingInstructionParamsFundingType,
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for CreateCustomerFundingInstructionParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("bank_transfer", &self.bank_transfer);
        form.field("currency", &self.currency);
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("funding_type", &self.funding_type);
    }
}

#[cfg(feature = "customers")]
#[derive(Debug, Clone, Default)]
pub struct UpdateCustomerParamsCashBalanceSettings {
    #[doc = "Controls how funds transferred by the customer are applied to payment intents and invoices."]
    pub reconciliation_mode:
        Option<stripe_types_06::customers::UpdateCustomerParamsCashBalanceSettingsReconciliationMode>,
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for UpdateCustomerParamsCashBalanceSettings {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("reconciliation_mode", self.reconciliation_mode.as_ref());
    }
}

#[cfg(feature = "customers")]
#[derive(Debug, Clone, Default)]
pub struct UpdateCustomerSourceParamsOwner {
    #[doc = "Owner's address."]
    pub address: Option<stripe_types_05::customers::UpdateCustomerSourceParamsOwnerAddress>,
    #[doc = "Owner's email address."]
    pub email: Option<String>,
    #[doc = "Owner's full name."]
    pub name: Option<String>,
    #[doc = "Owner's phone number."]
    pub phone: Option<String>,
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for UpdateCustomerSourceParamsOwner {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("address", self.address.as_ref());
        form.field_opt("email", self.email.as_ref());
        form.field_opt("name", self.name.as_ref());
        form.field_opt("phone", self.phone.as_ref());
    }
}
