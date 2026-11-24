#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct CreateTreasuryFinancialAccountFeatureParamsInboundTransfers {
    #[doc = "Enables ACH Debits via the InboundTransfers API."]
    pub ach: Option<stripe_types_06::treasury::CreateTreasuryFinancialAccountFeatureParamsInboundTransfersAch>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for CreateTreasuryFinancialAccountFeatureParamsInboundTransfers {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ach", self.ach.as_ref());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct CreateTreasuryFinancialAccountParamsFeaturesFinancialAddresses {
    #[doc = "Adds an ABA FinancialAddress to the FinancialAccount."]
    pub aba: Option<stripe_types_06::treasury::CreateTreasuryFinancialAccountParamsFeaturesFinancialAddressesAba>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for CreateTreasuryFinancialAccountParamsFeaturesFinancialAddresses {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("aba", self.aba.as_ref());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct ListTreasuryReceivedCreditsParams {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub financial_account: String,
    pub limit: Option<i64>,
    pub linked_flows: Option<stripe_types_05::treasury::ListTreasuryReceivedCreditsParamsLinkedFlows>,
    pub starting_after: Option<String>,
    pub status: Option<stripe_types_04::treasury::ListTreasuryReceivedCreditsParamsStatus>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for ListTreasuryReceivedCreditsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("financial_account", &self.financial_account);
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("linked_flows", self.linked_flows.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("status", self.status.as_ref());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct ListTreasuryTransactionEntriesParams {
    pub created: Option<stripe_types_05::treasury::ListTreasuryTransactionEntriesParamsCreated>,
    pub effective_at: Option<stripe_types_06::treasury::ListTreasuryTransactionEntriesParamsEffectiveAt>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub financial_account: String,
    pub limit: Option<i64>,
    pub order_by: Option<stripe_types_01::treasury::ListTreasuryTransactionEntriesParamsOrderBy>,
    pub starting_after: Option<String>,
    pub transaction: Option<String>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for ListTreasuryTransactionEntriesParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("created", self.created.as_ref());
        form.field_opt("effective_at", self.effective_at.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("financial_account", &self.financial_account);
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("order_by", self.order_by.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("transaction", self.transaction.as_ref());
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryFinancialAccountsResourceInboundAchToggleSettings {
    #[doc = "Whether the FinancialAccount should have the Feature."]
    #[json(default)]
    pub requested: bool,
    #[doc = "Whether the Feature is operational."]
    #[json(default)]
    pub status: stripe_types_00::treasury::TreasuryFinancialAccountsResourceAbaToggleSettingsStatus,
    #[doc = "Additional details; includes at least one entry when the status is not `active`."]
    #[json(default)]
    pub status_details: Vec<stripe_types_05::treasury::TreasuryFinancialAccountsResourceTogglesSettingStatusDetails>,
}

#[cfg(any(feature = "events", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryFinancialAccountsResourceStatusDetails {
    #[doc = "Details related to the closure of this FinancialAccount"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<stripe_types_05::treasury::TreasuryFinancialAccountsResourceClosedStatusDetails>,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails {
    #[doc = "Set when `type` is `balance`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub balance: Option<stripe_types_03::treasury::TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<stripe_types_06::treasury::TreasurySharedResourceBillingDetails>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<stripe_types_06::treasury::ReceivedPaymentMethodDetailsFinancialAccount>,
    #[doc = "Set when `type` is `issuing_card`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub issuing_card: Option<String>,
    #[doc = "Polymorphic type matching the originating money movement's source."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::treasury::TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<stripe_types_01::treasury::TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount>,
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct UpdateTreasuryFinancialAccountParamsFeaturesInboundTransfers {
    #[doc = "Enables ACH Debits via the InboundTransfers API."]
    pub ach: Option<stripe_types_06::treasury::UpdateTreasuryFinancialAccountParamsFeaturesInboundTransfersAch>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for UpdateTreasuryFinancialAccountParamsFeaturesInboundTransfers {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ach", self.ach.as_ref());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct UpdateTreasuryFinancialAccountParamsFeaturesOutboundPayments {
    #[doc = "Enables ACH transfers via the OutboundPayments API."]
    pub ach: Option<stripe_types_05::treasury::UpdateTreasuryFinancialAccountParamsFeaturesOutboundPaymentsAch>,
    #[doc = "Enables US domestic wire transfers via the OutboundPayments API."]
    pub us_domestic_wire:
        Option<stripe_types_05::treasury::UpdateTreasuryFinancialAccountParamsFeaturesOutboundPaymentsUsDomesticWire>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for UpdateTreasuryFinancialAccountParamsFeaturesOutboundPayments {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ach", self.ach.as_ref());
        form.field_opt("us_domestic_wire", self.us_domestic_wire.as_ref());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct UpdateTreasuryFinancialAccountParamsFeaturesOutboundTransfers {
    #[doc = "Enables ACH transfers via the OutboundTransfers API."]
    pub ach: Option<stripe_types_06::treasury::UpdateTreasuryFinancialAccountParamsFeaturesOutboundTransfersAch>,
    #[doc = "Enables US domestic wire transfers via the OutboundTransfers API."]
    pub us_domestic_wire:
        Option<stripe_types_05::treasury::UpdateTreasuryFinancialAccountParamsFeaturesOutboundTransfersUsDomesticWire>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for UpdateTreasuryFinancialAccountParamsFeaturesOutboundTransfers {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ach", self.ach.as_ref());
        form.field_opt("us_domestic_wire", self.us_domestic_wire.as_ref());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct UpdateTreasuryFinancialAccountParamsPlatformRestrictions {
    #[doc = "Restricts all inbound money movement."]
    pub inbound_flows:
        Option<stripe_types_06::treasury::UpdateTreasuryFinancialAccountParamsPlatformRestrictionsInboundFlows>,
    #[doc = "Restricts all outbound money movement."]
    pub outbound_flows:
        Option<stripe_types_06::treasury::UpdateTreasuryFinancialAccountParamsPlatformRestrictionsOutboundFlows>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for UpdateTreasuryFinancialAccountParamsPlatformRestrictions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("inbound_flows", self.inbound_flows.as_ref());
        form.field_opt("outbound_flows", self.outbound_flows.as_ref());
    }
}
