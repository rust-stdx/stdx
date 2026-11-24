#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct ListTreasuryTransactionsParams {
    pub created: Option<stripe_types_06::treasury::ListTreasuryTransactionsParamsCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub financial_account: String,
    pub limit: Option<i64>,
    pub order_by: Option<stripe_types_00::treasury::ListTreasuryTransactionsParamsOrderBy>,
    pub starting_after: Option<String>,
    pub status: Option<stripe_types_04::treasury::ListTreasuryTransactionsParamsStatus>,
    pub status_transitions: Option<stripe_types_09::treasury::ListTreasuryTransactionsParamsStatusTransitions>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for ListTreasuryTransactionsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("created", self.created.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("financial_account", &self.financial_account);
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("order_by", self.order_by.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("status", self.status.as_ref());
        form.field_opt("status_transitions", self.status_transitions.as_ref());
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryFinancialAccountsResourceFinancialAddressesFeatures {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub aba: Option<stripe_types_09::treasury::TreasuryFinancialAccountsResourceAbaToggleSettings>,
}

#[cfg(any(feature = "events", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryFinancialAccountsResourceOutboundTransfers {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ach: Option<stripe_types_09::treasury::TreasuryFinancialAccountsResourceOutboundAchToggleSettings>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire: Option<stripe_types_09::treasury::TreasuryFinancialAccountsResourceToggleSettings>,
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct UpdateTreasuryFinancialAccountParamsFeatures {
    #[doc = "Encodes the FinancialAccount's ability to be used with the Issuing product, including attaching cards to and drawing ..."]
    pub card_issuing: Option<stripe_types_05::treasury::UpdateTreasuryFinancialAccountParamsFeaturesCardIssuing>,
    #[doc = "Represents whether this FinancialAccount is eligible for deposit insurance."]
    pub deposit_insurance:
        Option<stripe_types_06::treasury::UpdateTreasuryFinancialAccountParamsFeaturesDepositInsurance>,
    #[doc = "Contains Features that add FinancialAddresses to the FinancialAccount."]
    pub financial_addresses:
        Option<stripe_types_08::treasury::UpdateTreasuryFinancialAccountParamsFeaturesFinancialAddresses>,
    #[doc = "Contains settings related to adding funds to a FinancialAccount from another Account with the same owner."]
    pub inbound_transfers:
        Option<stripe_types_07::treasury::UpdateTreasuryFinancialAccountParamsFeaturesInboundTransfers>,
    #[doc = "Represents the ability for the FinancialAccount to send money to, or receive money from other FinancialAccounts (for ..."]
    pub intra_stripe_flows:
        Option<stripe_types_05::treasury::UpdateTreasuryFinancialAccountParamsFeaturesIntraStripeFlows>,
    #[doc = "Includes Features related to initiating money movement out of the FinancialAccount to someone else's bucket of money."]
    pub outbound_payments:
        Option<stripe_types_07::treasury::UpdateTreasuryFinancialAccountParamsFeaturesOutboundPayments>,
    #[doc = "Contains a Feature and settings related to moving money out of the FinancialAccount into another Account with the sam..."]
    pub outbound_transfers:
        Option<stripe_types_07::treasury::UpdateTreasuryFinancialAccountParamsFeaturesOutboundTransfers>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for UpdateTreasuryFinancialAccountParamsFeatures {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("card_issuing", self.card_issuing.as_ref());
        form.field_opt("deposit_insurance", self.deposit_insurance.as_ref());
        form.field_opt("financial_addresses", self.financial_addresses.as_ref());
        form.field_opt("inbound_transfers", self.inbound_transfers.as_ref());
        form.field_opt("intra_stripe_flows", self.intra_stripe_flows.as_ref());
        form.field_opt("outbound_payments", self.outbound_payments.as_ref());
        form.field_opt("outbound_transfers", self.outbound_transfers.as_ref());
    }
}
