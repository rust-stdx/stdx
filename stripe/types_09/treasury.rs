#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct CreateTreasuryFinancialAccountFeatureParamsOutboundTransfers {
    #[doc = "Enables ACH transfers via the OutboundTransfers API."]
    pub ach: Option<stripe_types_05::treasury::CreateTreasuryFinancialAccountFeatureParamsOutboundTransfersAch>,
    #[doc = "Enables US domestic wire transfers via the OutboundTransfers API."]
    pub us_domestic_wire:
        Option<stripe_types_05::treasury::CreateTreasuryFinancialAccountFeatureParamsOutboundTransfersUsDomesticWire>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for CreateTreasuryFinancialAccountFeatureParamsOutboundTransfers {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ach", self.ach.as_ref());
        form.field_opt("us_domestic_wire", self.us_domestic_wire.as_ref());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct CreateTreasuryFinancialAccountParamsFeaturesOutboundPayments {
    #[doc = "Enables ACH transfers via the OutboundPayments API."]
    pub ach: Option<stripe_types_05::treasury::CreateTreasuryFinancialAccountParamsFeaturesOutboundPaymentsAch>,
    #[doc = "Enables US domestic wire transfers via the OutboundPayments API."]
    pub us_domestic_wire:
        Option<stripe_types_06::treasury::CreateTreasuryFinancialAccountParamsFeaturesOutboundPaymentsUsDomesticWire>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for CreateTreasuryFinancialAccountParamsFeaturesOutboundPayments {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ach", self.ach.as_ref());
        form.field_opt("us_domestic_wire", self.us_domestic_wire.as_ref());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct CreateTreasuryFinancialAccountParamsFeaturesOutboundTransfers {
    #[doc = "Enables ACH transfers via the OutboundTransfers API."]
    pub ach: Option<stripe_types_05::treasury::CreateTreasuryFinancialAccountParamsFeaturesOutboundTransfersAch>,
    #[doc = "Enables US domestic wire transfers via the OutboundTransfers API."]
    pub us_domestic_wire:
        Option<stripe_types_06::treasury::CreateTreasuryFinancialAccountParamsFeaturesOutboundTransfersUsDomesticWire>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for CreateTreasuryFinancialAccountParamsFeaturesOutboundTransfers {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ach", self.ach.as_ref());
        form.field_opt("us_domestic_wire", self.us_domestic_wire.as_ref());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct CreateTreasuryFinancialAccountParamsPlatformRestrictions {
    #[doc = "Restricts all inbound money movement."]
    pub inbound_flows:
        Option<stripe_types_00::treasury::CreateTreasuryFinancialAccountParamsPlatformRestrictionsInboundFlows>,
    #[doc = "Restricts all outbound money movement."]
    pub outbound_flows:
        Option<stripe_types_05::treasury::CreateTreasuryFinancialAccountParamsPlatformRestrictionsOutboundFlows>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for CreateTreasuryFinancialAccountParamsPlatformRestrictions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("inbound_flows", self.inbound_flows.as_ref());
        form.field_opt("outbound_flows", self.outbound_flows.as_ref());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodOptions {
    #[doc = "Optional fields for `us_bank_account`."]
    pub us_bank_account: Option<
        stripe_types_05::treasury::CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodOptionsUsBankAccount,
    >,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("us_bank_account", self.us_bank_account.as_ref());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct ListTreasuryOutboundPaymentsParams {
    pub created: Option<stripe_types_05::treasury::ListTreasuryOutboundPaymentsParamsCreated>,
    pub customer: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub financial_account: String,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<stripe_types_02::treasury::ListTreasuryOutboundPaymentsParamsStatus>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for ListTreasuryOutboundPaymentsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("created", self.created.as_ref());
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("financial_account", &self.financial_account);
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("status", self.status.as_ref());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct ListTreasuryOutboundTransfersParams {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub financial_account: String,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<stripe_types_06::treasury::ListTreasuryOutboundTransfersParamsStatus>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for ListTreasuryOutboundTransfersParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("financial_account", &self.financial_account);
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("status", self.status.as_ref());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct ListTreasuryReceivedDebitsParams {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub financial_account: String,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<stripe_types_05::treasury::ListTreasuryReceivedDebitsParamsStatus>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for ListTreasuryReceivedDebitsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("financial_account", &self.financial_account);
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("status", self.status.as_ref());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct ListTreasuryTransactionsParamsStatusTransitions {
    #[doc = "Returns Transactions with `posted_at` within the specified range."]
    pub posted_at: Option<stripe_types_05::treasury::ListTreasuryTransactionsParamsStatusTransitionsPostedAt>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for ListTreasuryTransactionsParamsStatusTransitions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("posted_at", self.posted_at.as_ref());
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryFinancialAccountsResourceAbaToggleSettings {
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
pub struct TreasuryFinancialAccountsResourceOutboundAchToggleSettings {
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
pub struct TreasuryFinancialAccountsResourceToggleSettings {
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
