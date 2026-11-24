#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct CreateTreasuryFinancialAccountParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Encodes whether a FinancialAccount has access to a particular feature."]
    pub features: Option<stripe_types_11::treasury::CreateTreasuryFinancialAccountParamsFeatures>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The nickname for the FinancialAccount."]
    pub nickname: Option<json::OwnedValue>,
    #[doc = "The set of functionalities that the platform can restrict on the FinancialAccount."]
    pub platform_restrictions:
        Option<stripe_types_09::treasury::CreateTreasuryFinancialAccountParamsPlatformRestrictions>,
    #[doc = "The currencies the FinancialAccount can hold a balance in."]
    pub supported_currencies: Vec<String>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for CreateTreasuryFinancialAccountParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("features", self.features.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("platform_restrictions", self.platform_restrictions.as_ref());
        form.field_array("supported_currencies", self.supported_currencies.as_slice());
    }
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct CreateTreasuryOutboundTransferParams {
    #[doc = "Amount (in cents) to be transferred."]
    pub amount: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "An arbitrary string attached to the object."]
    pub description: Option<String>,
    #[doc = "The PaymentMethod to use as the payment instrument for the OutboundTransfer."]
    pub destination_payment_method: Option<String>,
    #[doc = "Hash used to generate the PaymentMethod to be used for this OutboundTransfer."]
    pub destination_payment_method_data:
        Option<stripe_types_05::treasury::CreateTreasuryOutboundTransferParamsDestinationPaymentMethodData>,
    #[doc = "Hash describing payment method configuration details."]
    pub destination_payment_method_options:
        Option<stripe_types_11::treasury::CreateTreasuryOutboundTransferParamsDestinationPaymentMethodOptions>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The FinancialAccount to pull funds from."]
    pub financial_account: String,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Statement descriptor to be shown on the receiving end of an OutboundTransfer."]
    pub statement_descriptor: Option<String>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for CreateTreasuryOutboundTransferParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        form.field("currency", &self.currency);
        form.field_opt("description", self.description.as_ref());
        form.field_opt("destination_payment_method", self.destination_payment_method.as_ref());
        form.field_opt("destination_payment_method_data", self.destination_payment_method_data.as_ref());
        form.field_opt(
            "destination_payment_method_options",
            self.destination_payment_method_options.as_ref(),
        );
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

#[cfg(any(feature = "events", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryFinancialAccountFeatures {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<stripe_types_09::treasury::TreasuryFinancialAccountsResourceToggleSettings>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub deposit_insurance: Option<stripe_types_09::treasury::TreasuryFinancialAccountsResourceToggleSettings>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub financial_addresses:
        Option<stripe_types_10::treasury::TreasuryFinancialAccountsResourceFinancialAddressesFeatures>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub inbound_transfers: Option<stripe_types_11::treasury::TreasuryFinancialAccountsResourceInboundTransfers>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub intra_stripe_flows: Option<stripe_types_09::treasury::TreasuryFinancialAccountsResourceToggleSettings>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::treasury::TreasuryFinancialAccountFeaturesObject,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub outbound_payments: Option<stripe_types_11::treasury::TreasuryFinancialAccountsResourceOutboundPayments>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub outbound_transfers: Option<stripe_types_10::treasury::TreasuryFinancialAccountsResourceOutboundTransfers>,
}

#[cfg(feature = "treasury")]
#[derive(Debug, Clone, Default)]
pub struct UpdateTreasuryFinancialAccountParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Encodes whether a FinancialAccount has access to a particular feature, with a status enum and associated `status_deta..."]
    pub features: Option<stripe_types_10::treasury::UpdateTreasuryFinancialAccountParamsFeatures>,
    #[doc = "A different bank account where funds can be deposited/debited in order to get the closing FA's balance to $0"]
    pub forwarding_settings: Option<stripe_types_05::treasury::UpdateTreasuryFinancialAccountParamsForwardingSettings>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The nickname for the FinancialAccount."]
    pub nickname: Option<json::OwnedValue>,
    #[doc = "The set of functionalities that the platform can restrict on the FinancialAccount."]
    pub platform_restrictions:
        Option<stripe_types_07::treasury::UpdateTreasuryFinancialAccountParamsPlatformRestrictions>,
}

#[cfg(feature = "treasury")]
impl crate::params::FormEncode for UpdateTreasuryFinancialAccountParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("features", self.features.as_ref());
        form.field_opt("forwarding_settings", self.forwarding_settings.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("platform_restrictions", self.platform_restrictions.as_ref());
    }
}
