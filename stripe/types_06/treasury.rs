#[cfg(feature = "treasury")]
pub type CancelTreasuryInboundTransferParams = stripe_types_01::treasury::GetTreasuryCreditReversalParams;

#[cfg(feature = "treasury")]
pub type CancelTreasuryOutboundTransferParams = stripe_types_01::treasury::GetTreasuryCreditReversalParams;

#[cfg(feature = "treasury")]
pub type CreateTreasuryFinancialAccountFeatureParamsCardIssuing =
    stripe_types_00::treasury::CreateTreasuryFinancialAccountParamsFeaturesCardIssuing;

#[cfg(feature = "treasury")]
pub type CreateTreasuryFinancialAccountFeatureParamsFinancialAddressesAba =
    stripe_types_00::treasury::CreateTreasuryFinancialAccountParamsFeaturesCardIssuing;

#[cfg(feature = "treasury")]
pub type CreateTreasuryFinancialAccountFeatureParamsInboundTransfersAch =
    stripe_types_00::treasury::CreateTreasuryFinancialAccountParamsFeaturesCardIssuing;

#[cfg(feature = "treasury")]
pub type CreateTreasuryFinancialAccountFeatureParamsIntraStripeFlows =
    stripe_types_00::treasury::CreateTreasuryFinancialAccountParamsFeaturesCardIssuing;

#[cfg(feature = "treasury")]
pub type CreateTreasuryFinancialAccountFeatureParamsOutboundPaymentsUsDomesticWire =
    stripe_types_00::treasury::CreateTreasuryFinancialAccountParamsFeaturesCardIssuing;

#[cfg(feature = "treasury")]
pub type CreateTreasuryFinancialAccountParamsFeaturesDepositInsurance =
    stripe_types_00::treasury::CreateTreasuryFinancialAccountParamsFeaturesCardIssuing;

#[cfg(feature = "treasury")]
pub type CreateTreasuryFinancialAccountParamsFeaturesFinancialAddressesAba =
    stripe_types_00::treasury::CreateTreasuryFinancialAccountParamsFeaturesCardIssuing;

#[cfg(feature = "treasury")]
pub type CreateTreasuryFinancialAccountParamsFeaturesInboundTransfersAch =
    stripe_types_00::treasury::CreateTreasuryFinancialAccountParamsFeaturesCardIssuing;

#[cfg(feature = "treasury")]
pub type CreateTreasuryFinancialAccountParamsFeaturesOutboundPaymentsUsDomesticWire =
    stripe_types_00::treasury::CreateTreasuryFinancialAccountParamsFeaturesCardIssuing;

#[cfg(feature = "treasury")]
pub type CreateTreasuryFinancialAccountParamsFeaturesOutboundTransfersUsDomesticWire =
    stripe_types_00::treasury::CreateTreasuryFinancialAccountParamsFeaturesCardIssuing;

#[cfg(feature = "treasury")]
pub type GetTreasuryFinancialAccountFeatureParams = stripe_types_01::treasury::GetTreasuryCreditReversalParams;

#[cfg(feature = "treasury")]
pub type GetTreasuryFinancialAccountParams = stripe_types_01::treasury::GetTreasuryCreditReversalParams;

#[cfg(feature = "treasury")]
pub type GetTreasuryInboundTransferParams = stripe_types_01::treasury::GetTreasuryCreditReversalParams;

#[cfg(feature = "treasury")]
pub type GetTreasuryReceivedDebitParams = stripe_types_01::treasury::GetTreasuryCreditReversalParams;

#[cfg(feature = "treasury")]
pub type GetTreasuryTransactionEntryParams = stripe_types_01::treasury::GetTreasuryCreditReversalParams;

#[cfg(feature = "treasury")]
pub type GetTreasuryTransactionParams = stripe_types_01::treasury::GetTreasuryCreditReversalParams;

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct ListTreasuryCreditReversalsParams {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub financial_account: String,
    pub limit: Option<i64>,
    pub received_credit: Option<String>,
    pub starting_after: Option<String>,
    pub status: Option<stripe_types_00::treasury::ListTreasuryCreditReversalsParamsStatus>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for ListTreasuryCreditReversalsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("financial_account", &self.financial_account);
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("received_credit", self.received_credit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("status", self.status.as_ref());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct ListTreasuryDebitReversalsParams {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub financial_account: String,
    pub limit: Option<i64>,
    pub received_debit: Option<String>,
    pub resolution: Option<stripe_types_02::treasury::ListTreasuryDebitReversalsParamsResolution>,
    pub starting_after: Option<String>,
    pub status: Option<stripe_types_02::treasury::ListTreasuryDebitReversalsParamsStatus>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for ListTreasuryDebitReversalsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("financial_account", &self.financial_account);
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("received_debit", self.received_debit.as_ref());
        form.field_opt("resolution", self.resolution.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("status", self.status.as_ref());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct ListTreasuryFinancialAccountsParams {
    pub created: Option<stripe_types_03::treasury::ListTreasuryFinancialAccountsParamsCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<stripe_types_03::treasury::ListTreasuryFinancialAccountsParamsStatus>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for ListTreasuryFinancialAccountsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("created", self.created.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("status", self.status.as_ref());
    }
}

#[cfg(feature = "treasury")]
pub type ListTreasuryOutboundTransfersParamsStatus =
    stripe_types_02::treasury::ListTreasuryOutboundPaymentsParamsStatus;

#[cfg(feature = "treasury")]
pub type ListTreasuryTransactionEntriesParamsEffectiveAt =
    stripe_types_03::treasury::ListTreasuryFinancialAccountsParamsCreated;

#[cfg(feature = "treasury")]
pub type ListTreasuryTransactionsParamsCreated = stripe_types_03::treasury::ListTreasuryFinancialAccountsParamsCreated;

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ReceivedPaymentMethodDetailsFinancialAccount {
    #[doc = "The FinancialAccount ID."]
    #[json(default)]
    pub id: String,
    #[doc = "The rails the ReceivedCredit was sent over."]
    #[json(default)]
    pub network: stripe_types_03::treasury::OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl crate::PagedItem for ReceivedPaymentMethodDetailsFinancialAccount {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ReservePlan {
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Indicates which party created this ReservePlan."]
    #[json(default)]
    pub created_by: stripe_types_01::treasury::ReserveHoldCreatedBy,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "The balance destination to which the reserved funds are sent."]
    #[json(default)]
    pub destination: stripe_types_04::treasury::ReserveHoldDestination,
    #[doc = "Time at which the ReservePlan was disabled."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub disabled_at: Option<i64>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fixed_release: Option<stripe_types_01::treasury::ReservesReservePlansResourcesFixedRelease>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub manual_release: Option<stripe_types_04::treasury::ReservesReservePlansResourcesManualRelease>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::treasury::ReservePlanObject,
    #[doc = "The percent of each Charge to reserve."]
    #[json(default)]
    pub percent: i64,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub rolling_release: Option<stripe_types_04::treasury::ReservesReservePlansResourcesRollingRelease>,
    #[doc = "The current status of the ReservePlan."]
    #[json(default)]
    pub status: stripe_types_03::treasury::ReservePlanStatus,
    #[doc = "The type of the ReservePlan."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::treasury::ReservePlanType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for ReservePlan {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryFinancialAccountsResourceFinancialAddress {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub aba: Option<stripe_types_02::treasury::TreasuryFinancialAccountsResourceAbaRecord>,
    #[doc = "The list of networks that the address supports"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub supported_networks:
        Option<Vec<stripe_types_00::treasury::TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworksItem>>,
    #[doc = "The type of financial address"]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::treasury::TreasuryFinancialAccountsResourceFinancialAddressType,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetails {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ach: Option<stripe_types_01::treasury::TreasuryOutboundPaymentsResourceAchTrackingDetails>,
    #[doc = "The US bank account network used to send funds."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::treasury::OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire:
        Option<stripe_types_03::treasury::TreasuryOutboundPaymentsResourceUsDomesticWireTrackingDetails>,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetails {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ach: Option<stripe_types_02::treasury::TreasuryOutboundTransfersResourceAchTrackingDetails>,
    #[doc = "The US bank account network used to send funds."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::treasury::OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire:
        Option<stripe_types_04::treasury::TreasuryOutboundTransfersResourceUsDomesticWireTrackingDetails>,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryReceivedDebitsResourceReversalDetails {
    #[doc = "Time before which a ReceivedDebit can be reversed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub deadline: Option<i64>,
    #[doc = "Set if a ReceivedDebit can't be reversed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub restricted_reason:
        Option<stripe_types_00::treasury::TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason>,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasurySharedResourceBillingDetails {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_01::common::Address>,
    #[doc = "Email address."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "Full name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[cfg(feature = "treasury")]
pub type UpdateTreasuryFinancialAccountParamsFeaturesDepositInsurance =
    stripe_types_00::treasury::CreateTreasuryFinancialAccountParamsFeaturesCardIssuing;

#[cfg(feature = "treasury")]
pub type UpdateTreasuryFinancialAccountParamsFeaturesInboundTransfersAch =
    stripe_types_00::treasury::CreateTreasuryFinancialAccountParamsFeaturesCardIssuing;

#[cfg(feature = "treasury")]
pub type UpdateTreasuryFinancialAccountParamsFeaturesOutboundTransfersAch =
    stripe_types_00::treasury::CreateTreasuryFinancialAccountParamsFeaturesCardIssuing;

#[cfg(feature = "treasury")]
pub type UpdateTreasuryFinancialAccountParamsPlatformRestrictionsInboundFlows =
    stripe_types_00::treasury::CreateTreasuryFinancialAccountParamsPlatformRestrictionsInboundFlows;

#[cfg(feature = "treasury")]
pub type UpdateTreasuryFinancialAccountParamsPlatformRestrictionsOutboundFlows =
    stripe_types_00::treasury::CreateTreasuryFinancialAccountParamsPlatformRestrictionsInboundFlows;
