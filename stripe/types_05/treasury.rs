#[cfg(feature = "treasury")]
pub type CancelTreasuryOutboundPaymentParams = stripe_types_01::treasury::GetTreasuryCreditReversalParams;

#[cfg(feature = "treasury")]
pub type CloseTreasuryFinancialAccountParamsForwardingSettingsType =
    stripe_types_03::treasury::UpdateTreasuryFinancialAccountParamsForwardingSettingsType;

#[cfg(feature = "treasury")]
pub type CreateTreasuryFinancialAccountFeatureParamsDepositInsurance =
    stripe_types_00::treasury::CreateTreasuryFinancialAccountParamsFeaturesCardIssuing;

#[cfg(feature = "treasury")]
pub type CreateTreasuryFinancialAccountFeatureParamsOutboundPaymentsAch =
    stripe_types_00::treasury::CreateTreasuryFinancialAccountParamsFeaturesCardIssuing;

#[cfg(feature = "treasury")]
pub type CreateTreasuryFinancialAccountFeatureParamsOutboundTransfersAch =
    stripe_types_00::treasury::CreateTreasuryFinancialAccountParamsFeaturesCardIssuing;

#[cfg(feature = "treasury")]
pub type CreateTreasuryFinancialAccountFeatureParamsOutboundTransfersUsDomesticWire =
    stripe_types_00::treasury::CreateTreasuryFinancialAccountParamsFeaturesCardIssuing;

#[cfg(feature = "treasury")]
pub type CreateTreasuryFinancialAccountParamsFeaturesIntraStripeFlows =
    stripe_types_00::treasury::CreateTreasuryFinancialAccountParamsFeaturesCardIssuing;

#[cfg(feature = "treasury")]
pub type CreateTreasuryFinancialAccountParamsFeaturesOutboundPaymentsAch =
    stripe_types_00::treasury::CreateTreasuryFinancialAccountParamsFeaturesCardIssuing;

#[cfg(feature = "treasury")]
pub type CreateTreasuryFinancialAccountParamsFeaturesOutboundTransfersAch =
    stripe_types_00::treasury::CreateTreasuryFinancialAccountParamsFeaturesCardIssuing;

#[cfg(feature = "treasury")]
pub type CreateTreasuryFinancialAccountParamsPlatformRestrictionsOutboundFlows =
    stripe_types_00::treasury::CreateTreasuryFinancialAccountParamsPlatformRestrictionsInboundFlows;

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataBillingDetails {
    #[doc = "Billing address."]
    pub address: Option<
        stripe_types_03::treasury::CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataBillingDetailsAddress,
    >,
    #[doc = "Email address."]
    pub email: Option<json::OwnedValue>,
    #[doc = "Full name."]
    pub name: Option<json::OwnedValue>,
    #[doc = "Billing phone number (including extension)."]
    pub phone: Option<json::OwnedValue>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataBillingDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("address", self.address.as_ref());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataUsBankAccount {
    #[doc = "Account holder type: individual or company."]
    pub account_holder_type: Option<stripe_types_00::treasury::CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataUsBankAccountAccountHolderType>,
    #[doc = "Account number of the bank account."]
    pub account_number: Option<String>,
    #[doc = "Account type: checkings or savings."]
    pub account_type: Option<stripe_types_04::treasury::CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataUsBankAccountAccountType>,
    #[doc = "The ID of a Financial Connections Account to use as a payment method."]
    pub financial_connections_account: Option<String>,
    #[doc = "Routing number of the bank account."]
    pub routing_number: Option<String>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodDataUsBankAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account_holder_type", self.account_holder_type.as_ref());
        form.field_opt("account_number", self.account_number.as_ref());
        form.field_opt("account_type", self.account_type.as_ref());
        form.field_opt("financial_connections_account", self.financial_connections_account.as_ref());
        form.field_opt("routing_number", self.routing_number.as_ref());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodOptionsUsBankAccount {
    #[doc = "Specifies the network rails to be used."]
    pub network: Option<stripe_types_02::treasury::CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodOptionsUsBankAccountNetwork>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodOptionsUsBankAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("network", self.network.as_ref());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone)]
pub struct CreateTreasuryOutboundTransferParamsDestinationPaymentMethodData {
    #[doc = "Required if type is set to `financial_account`."]
    pub financial_account: Option<String>,
    #[doc = "The type of the destination."]
    pub r#type: stripe_types_04::treasury::CreateTreasuryOutboundTransferParamsDestinationPaymentMethodDataType,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for CreateTreasuryOutboundTransferParamsDestinationPaymentMethodData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("financial_account", self.financial_account.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "treasury")]
pub type CreateTreasuryOutboundTransferParamsDestinationPaymentMethodOptionsUsBankAccountNetwork =
    stripe_types_02::treasury::CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodOptionsUsBankAccountNetwork;

#[cfg(feature = "treasury")]
pub type GetTreasuryDebitReversalParams = stripe_types_01::treasury::GetTreasuryCreditReversalParams;

#[cfg(feature = "treasury")]
pub type GetTreasuryOutboundPaymentParams = stripe_types_01::treasury::GetTreasuryCreditReversalParams;

#[cfg(feature = "treasury")]
pub type GetTreasuryOutboundTransferParams = stripe_types_01::treasury::GetTreasuryCreditReversalParams;

#[cfg(feature = "treasury")]
pub type GetTreasuryReceivedCreditParams = stripe_types_01::treasury::GetTreasuryCreditReversalParams;

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct ListTreasuryInboundTransfersParams {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub financial_account: String,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<stripe_types_01::treasury::ListTreasuryInboundTransfersParamsStatus>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for ListTreasuryInboundTransfersParams {
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
pub type ListTreasuryOutboundPaymentsParamsCreated =
    stripe_types_03::treasury::ListTreasuryFinancialAccountsParamsCreated;

#[cfg(feature = "treasury")]
#[derive(Debug, Clone)]
pub struct ListTreasuryReceivedCreditsParamsLinkedFlows {
    #[doc = "The source flow type."]
    pub source_flow_type: stripe_types_02::treasury::ListTreasuryReceivedCreditsParamsLinkedFlowsSourceFlowType,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for ListTreasuryReceivedCreditsParamsLinkedFlows {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("source_flow_type", &self.source_flow_type);
    }
}

#[cfg(feature = "treasury")]
pub type ListTreasuryReceivedDebitsParamsStatus = stripe_types_04::treasury::ListTreasuryReceivedCreditsParamsStatus;

#[cfg(feature = "treasury")]
pub type ListTreasuryTransactionEntriesParamsCreated =
    stripe_types_03::treasury::ListTreasuryFinancialAccountsParamsCreated;

#[cfg(feature = "treasury")]
pub type ListTreasuryTransactionsParamsStatusTransitionsPostedAt =
    stripe_types_03::treasury::ListTreasuryFinancialAccountsParamsCreated;

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct OutboundPaymentsPaymentMethodDetailsFinancialAccount {
    #[doc = "Token of the FinancialAccount."]
    #[json(default)]
    pub id: String,
    #[doc = "The rails used to send funds."]
    #[json(default)]
    pub network: stripe_types_03::treasury::OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl crate::PagedItem for OutboundPaymentsPaymentMethodDetailsFinancialAccount {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct OutboundTransfersPaymentMethodDetailsFinancialAccount {
    #[doc = "Token of the FinancialAccount."]
    #[json(default)]
    pub id: String,
    #[doc = "The rails used to send funds."]
    #[json(default)]
    pub network: stripe_types_03::treasury::OutboundPaymentsPaymentMethodDetailsFinancialAccountNetwork,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl crate::PagedItem for OutboundTransfersPaymentMethodDetailsFinancialAccount {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ReserveTransaction {
    #[json(default)]
    pub amount: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "An arbitrary string attached to the object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_02::treasury::ReserveTransactionObject,
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
impl crate::PagedItem for ReserveTransaction {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryFinancialAccountsResourceClosedStatusDetails {
    #[doc = "The array that contains reasons for a FinancialAccount closure."]
    #[json(default)]
    pub reasons: Vec<stripe_types_01::treasury::TreasuryFinancialAccountsResourceClosedStatusDetailsReasonsItem>,
}

#[cfg(any(feature = "events", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryFinancialAccountsResourcePlatformRestrictions {
    #[doc = "Restricts all inbound money movement."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub inbound_flows:
        Option<stripe_types_02::treasury::TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows>,
    #[doc = "Restricts all outbound money movement."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub outbound_flows:
        Option<stripe_types_02::treasury::TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows>,
}

#[cfg(any(feature = "events", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryFinancialAccountsResourceTogglesSettingStatusDetails {
    #[doc = "Represents the reason why the status is `pending` or `restricted`."]
    #[json(default)]
    pub code: stripe_types_03::treasury::TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode,
    #[doc = "Represents what the user should do, if anything, to activate the Feature."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub resolution:
        Option<stripe_types_00::treasury::TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution>,
    #[doc = "The `platform_restrictions` that are restricting this Feature."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub restriction:
        Option<stripe_types_01::treasury::TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction>,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryInboundTransfersResourceFailureDetails {
    #[doc = "Reason for the failure."]
    #[json(default)]
    pub code: stripe_types_03::treasury::TreasuryInboundTransfersResourceFailureDetailsCode,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryReceivedCreditsResourceReversalDetails {
    #[doc = "Time before which a ReceivedCredit can be reversed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub deadline: Option<i64>,
    #[doc = "Set if a ReceivedCredit cannot be reversed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub restricted_reason:
        Option<stripe_types_00::treasury::TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason>,
}

#[cfg(feature = "treasury")]
pub type UpdateTreasuryFinancialAccountParamsFeaturesCardIssuing =
    stripe_types_00::treasury::CreateTreasuryFinancialAccountParamsFeaturesCardIssuing;

#[cfg(feature = "treasury")]
pub type UpdateTreasuryFinancialAccountParamsFeaturesFinancialAddressesAba =
    stripe_types_00::treasury::CreateTreasuryFinancialAccountParamsFeaturesCardIssuing;

#[cfg(feature = "treasury")]
pub type UpdateTreasuryFinancialAccountParamsFeaturesIntraStripeFlows =
    stripe_types_00::treasury::CreateTreasuryFinancialAccountParamsFeaturesCardIssuing;

#[cfg(feature = "treasury")]
pub type UpdateTreasuryFinancialAccountParamsFeaturesOutboundPaymentsAch =
    stripe_types_00::treasury::CreateTreasuryFinancialAccountParamsFeaturesCardIssuing;

#[cfg(feature = "treasury")]
pub type UpdateTreasuryFinancialAccountParamsFeaturesOutboundPaymentsUsDomesticWire =
    stripe_types_00::treasury::CreateTreasuryFinancialAccountParamsFeaturesCardIssuing;

#[cfg(feature = "treasury")]
pub type UpdateTreasuryFinancialAccountParamsFeaturesOutboundTransfersUsDomesticWire =
    stripe_types_00::treasury::CreateTreasuryFinancialAccountParamsFeaturesCardIssuing;

#[cfg(feature = "treasury")]
#[derive(Debug, Clone)]
pub struct UpdateTreasuryFinancialAccountParamsForwardingSettings {
    #[doc = "The financial_account id"]
    pub financial_account: Option<String>,
    #[doc = "The payment_method or bank account id."]
    pub payment_method: Option<String>,
    #[doc = "The type of the bank account provided."]
    pub r#type: stripe_types_03::treasury::UpdateTreasuryFinancialAccountParamsForwardingSettingsType,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for UpdateTreasuryFinancialAccountParamsForwardingSettings {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("financial_account", self.financial_account.as_ref());
        form.field_opt("payment_method", self.payment_method.as_ref());
        form.field("type", &self.r#type);
    }
}
