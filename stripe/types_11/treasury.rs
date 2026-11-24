#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct CloseTreasuryFinancialAccountParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "A different bank account where funds can be deposited/debited in order to get the closing FA's balance to $0"]
    pub forwarding_settings: Option<stripe_types_08::treasury::CloseTreasuryFinancialAccountParamsForwardingSettings>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for CloseTreasuryFinancialAccountParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("forwarding_settings", self.forwarding_settings.as_ref());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct CreateTreasuryFinancialAccountFeatureParams {
    #[doc = "Encodes the FinancialAccount's ability to be used with the Issuing product, including attaching cards to and drawing ..."]
    pub card_issuing: Option<stripe_types_06::treasury::CreateTreasuryFinancialAccountFeatureParamsCardIssuing>,
    #[doc = "Represents whether this FinancialAccount is eligible for deposit insurance."]
    pub deposit_insurance:
        Option<stripe_types_05::treasury::CreateTreasuryFinancialAccountFeatureParamsDepositInsurance>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Contains Features that add FinancialAddresses to the FinancialAccount."]
    pub financial_addresses:
        Option<stripe_types_08::treasury::CreateTreasuryFinancialAccountFeatureParamsFinancialAddresses>,
    #[doc = "Contains settings related to adding funds to a FinancialAccount from another Account with the same owner."]
    pub inbound_transfers:
        Option<stripe_types_07::treasury::CreateTreasuryFinancialAccountFeatureParamsInboundTransfers>,
    #[doc = "Represents the ability for the FinancialAccount to send money to, or receive money from other FinancialAccounts (for ..."]
    pub intra_stripe_flows:
        Option<stripe_types_06::treasury::CreateTreasuryFinancialAccountFeatureParamsIntraStripeFlows>,
    #[doc = "Includes Features related to initiating money movement out of the FinancialAccount to someone else's bucket of money."]
    pub outbound_payments:
        Option<stripe_types_08::treasury::CreateTreasuryFinancialAccountFeatureParamsOutboundPayments>,
    #[doc = "Contains a Feature and settings related to moving money out of the FinancialAccount into another Account with the sam..."]
    pub outbound_transfers:
        Option<stripe_types_09::treasury::CreateTreasuryFinancialAccountFeatureParamsOutboundTransfers>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for CreateTreasuryFinancialAccountFeatureParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("card_issuing", self.card_issuing.as_ref());
        form.field_opt("deposit_insurance", self.deposit_insurance.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("financial_addresses", self.financial_addresses.as_ref());
        form.field_opt("inbound_transfers", self.inbound_transfers.as_ref());
        form.field_opt("intra_stripe_flows", self.intra_stripe_flows.as_ref());
        form.field_opt("outbound_payments", self.outbound_payments.as_ref());
        form.field_opt("outbound_transfers", self.outbound_transfers.as_ref());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct CreateTreasuryFinancialAccountParamsFeatures {
    #[doc = "Encodes the FinancialAccount's ability to be used with the Issuing product, including attaching cards to and drawing ..."]
    pub card_issuing: Option<stripe_types_00::treasury::CreateTreasuryFinancialAccountParamsFeaturesCardIssuing>,
    #[doc = "Represents whether this FinancialAccount is eligible for deposit insurance."]
    pub deposit_insurance:
        Option<stripe_types_06::treasury::CreateTreasuryFinancialAccountParamsFeaturesDepositInsurance>,
    #[doc = "Contains Features that add FinancialAddresses to the FinancialAccount."]
    pub financial_addresses:
        Option<stripe_types_07::treasury::CreateTreasuryFinancialAccountParamsFeaturesFinancialAddresses>,
    #[doc = "Contains settings related to adding funds to a FinancialAccount from another Account with the same owner."]
    pub inbound_transfers:
        Option<stripe_types_08::treasury::CreateTreasuryFinancialAccountParamsFeaturesInboundTransfers>,
    #[doc = "Represents the ability for the FinancialAccount to send money to, or receive money from other FinancialAccounts (for ..."]
    pub intra_stripe_flows:
        Option<stripe_types_05::treasury::CreateTreasuryFinancialAccountParamsFeaturesIntraStripeFlows>,
    #[doc = "Includes Features related to initiating money movement out of the FinancialAccount to someone else's bucket of money."]
    pub outbound_payments:
        Option<stripe_types_09::treasury::CreateTreasuryFinancialAccountParamsFeaturesOutboundPayments>,
    #[doc = "Contains a Feature and settings related to moving money out of the FinancialAccount into another Account with the sam..."]
    pub outbound_transfers:
        Option<stripe_types_09::treasury::CreateTreasuryFinancialAccountParamsFeaturesOutboundTransfers>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for CreateTreasuryFinancialAccountParamsFeatures {
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

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct CreateTreasuryOutboundPaymentParams {
    #[doc = "Amount (in cents) to be transferred."]
    pub amount: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "ID of the customer to whom the OutboundPayment is sent."]
    pub customer: Option<String>,
    #[doc = "An arbitrary string attached to the object."]
    pub description: Option<String>,
    #[doc = "The PaymentMethod to use as the payment instrument for the OutboundPayment."]
    pub destination_payment_method: Option<String>,
    #[doc = "Hash used to generate the PaymentMethod to be used for this OutboundPayment."]
    pub destination_payment_method_data:
        Option<stripe_types_08::treasury::CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodData>,
    #[doc = "Payment method-specific configuration for this OutboundPayment."]
    pub destination_payment_method_options:
        Option<stripe_types_09::treasury::CreateTreasuryOutboundPaymentParamsDestinationPaymentMethodOptions>,
    #[doc = "End user details."]
    pub end_user_details: Option<stripe_types_00::treasury::CreateTreasuryOutboundPaymentParamsEndUserDetails>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The FinancialAccount to pull funds from."]
    pub financial_account: String,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The description that appears on the receiving end for this OutboundPayment (for example, bank statement for external ..."]
    pub statement_descriptor: Option<String>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for CreateTreasuryOutboundPaymentParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        form.field("currency", &self.currency);
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field_opt("destination_payment_method", self.destination_payment_method.as_ref());
        form.field_opt("destination_payment_method_data", self.destination_payment_method_data.as_ref());
        form.field_opt(
            "destination_payment_method_options",
            self.destination_payment_method_options.as_ref(),
        );
        form.field_opt("end_user_details", self.end_user_details.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("financial_account", &self.financial_account);
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("statement_descriptor", self.statement_descriptor.as_ref());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct CreateTreasuryOutboundTransferParamsDestinationPaymentMethodOptions {
    #[doc = "Optional fields for `us_bank_account`."]
    pub us_bank_account: Option<
        stripe_types_08::treasury::CreateTreasuryOutboundTransferParamsDestinationPaymentMethodOptionsUsBankAccount,
    >,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for CreateTreasuryOutboundTransferParamsDestinationPaymentMethodOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("us_bank_account", self.us_bank_account.as_ref());
    }
}

#[cfg(any(feature = "events", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryFinancialAccountsResourceInboundTransfers {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ach: Option<stripe_types_07::treasury::TreasuryFinancialAccountsResourceInboundAchToggleSettings>,
}

#[cfg(any(feature = "events", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryFinancialAccountsResourceOutboundPayments {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ach: Option<stripe_types_09::treasury::TreasuryFinancialAccountsResourceOutboundAchToggleSettings>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire: Option<stripe_types_09::treasury::TreasuryFinancialAccountsResourceToggleSettings>,
}
