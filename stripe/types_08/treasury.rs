#[cfg(feature = "treasury")]
#[derive(Debug, Clone)]
pub struct CloseTreasuryFinancialAccountParamsForwardingSettings {
    #[doc = "The financial_account id"]
    pub financial_account: Option<String>,
    #[doc = "The payment_method or bank account id."]
    pub payment_method: Option<String>,
    #[doc = "The type of the bank account provided."]
    pub r#type: stripe_types_05::treasury::CloseTreasuryFinancialAccountParamsForwardingSettingsType,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for CloseTreasuryFinancialAccountParamsForwardingSettings {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("financial_account", self.financial_account.as_ref());
        form.field_opt("payment_method", self.payment_method.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct CreateTreasuryFinancialAccountFeatureParamsFinancialAddresses {
    #[doc = "Adds an ABA FinancialAddress to the FinancialAccount."]
    pub aba: Option<stripe_types_06::treasury::CreateTreasuryFinancialAccountFeatureParamsFinancialAddressesAba>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for CreateTreasuryFinancialAccountFeatureParamsFinancialAddresses {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("aba", self.aba.as_ref());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct CreateTreasuryFinancialAccountFeatureParamsOutboundPayments {
    #[doc = "Enables ACH transfers via the OutboundPayments API."]
    pub ach: Option<stripe_types_05::treasury::CreateTreasuryFinancialAccountFeatureParamsOutboundPaymentsAch>,
    #[doc = "Enables US domestic wire transfers via the OutboundPayments API."]
    pub us_domestic_wire:
        Option<stripe_types_06::treasury::CreateTreasuryFinancialAccountFeatureParamsOutboundPaymentsUsDomesticWire>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for CreateTreasuryFinancialAccountFeatureParamsOutboundPayments {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ach", self.ach.as_ref());
        form.field_opt("us_domestic_wire", self.us_domestic_wire.as_ref());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct CreateTreasuryFinancialAccountParamsFeaturesInboundTransfers {
    #[doc = "Enables ACH Debits via the InboundTransfers API."]
    pub ach: Option<stripe_types_06::treasury::CreateTreasuryFinancialAccountParamsFeaturesInboundTransfersAch>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for CreateTreasuryFinancialAccountParamsFeaturesInboundTransfers {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ach", self.ach.as_ref());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone)]
pub struct CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodData {
    #[doc = "Billing information associated with the PaymentMethod that may be used or required by particular types of payment met..."]
    pub billing_details: Option<
        stripe_types_05::treasury::CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataBillingDetails,
    >,
    #[doc = "Required if type is set to `financial_account`."]
    pub financial_account: Option<String>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The type of the PaymentMethod."]
    pub r#type: stripe_types_02::treasury::CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataType,
    #[doc = "Required hash if type is set to `us_bank_account`."]
    pub us_bank_account:
        Option<stripe_types_05::treasury::CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataUsBankAccount>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("billing_details", self.billing_details.as_ref());
        form.field_opt("financial_account", self.financial_account.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field("type", &self.r#type);
        form.field_opt("us_bank_account", self.us_bank_account.as_ref());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct CreateTreasuryOutboundTransferParamsDestinationPaymentMethodOptionsUsBankAccount {
    #[doc = "Specifies the network rails to be used."]
    pub network: Option<stripe_types_05::treasury::CreateTreasuryOutboundTransferParamsDestinationPaymentMethodOptionsUsBankAccountNetwork>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for CreateTreasuryOutboundTransferParamsDestinationPaymentMethodOptionsUsBankAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("network", self.network.as_ref());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct UpdateTreasuryFinancialAccountParamsFeaturesFinancialAddresses {
    #[doc = "Adds an ABA FinancialAddress to the FinancialAccount."]
    pub aba: Option<stripe_types_05::treasury::UpdateTreasuryFinancialAccountParamsFeaturesFinancialAddressesAba>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for UpdateTreasuryFinancialAccountParamsFeaturesFinancialAddresses {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("aba", self.aba.as_ref());
    }
}
