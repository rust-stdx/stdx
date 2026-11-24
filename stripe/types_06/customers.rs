#[cfg(feature = "customers")]
#[derive(Debug, Clone)]
pub struct CreateCustomerFundingInstructionParamsBankTransfer {
    #[doc = "Configuration for eu_bank_transfer funding type."]
    pub eu_bank_transfer:
        Option<stripe_types_01::customers::CreateCustomerFundingInstructionParamsBankTransferEuBankTransfer>,
    #[doc = "List of address types that should be returned in the financial_addresses response."]
    pub requested_address_types: Option<
        Vec<stripe_types_02::customers::CreateCustomerFundingInstructionParamsBankTransferRequestedAddressTypesItem>,
    >,
    #[doc = "The type of the `bank_transfer`"]
    pub r#type: stripe_types_02::customers::CreateCustomerFundingInstructionParamsBankTransferType,
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for CreateCustomerFundingInstructionParamsBankTransfer {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("eu_bank_transfer", self.eu_bank_transfer.as_ref());
        if let Some(values) = self.requested_address_types.as_ref() {
            form.field_array("requested_address_types", values.as_slice());
        }
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "customers")]
pub type CreateCustomerTaxIdParamsType = stripe_types_02::customers::CreateCustomerParamsTaxIdDataItemType;

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
pub struct CustomerAcceptance {
    #[doc = "The time that the customer accepts the mandate."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub accepted_at: Option<i64>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub offline: Option<stripe_types_02::terminal::OfflineAcceptance>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub online: Option<stripe_types_01::terminal::OnlineAcceptance>,
    #[doc = "The mandate includes the type of customer acceptance information, such as: `online` or `offline`."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::customers::CustomerAcceptanceType,
}

#[cfg(feature = "customers")]
pub type GetCustomerPaymentMethodParams = stripe_types_04::customers::GetCustomerParams;

#[cfg(feature = "customers")]
pub type GetCustomerSourceParams = stripe_types_04::customers::GetCustomerParams;

#[cfg(feature = "customers")]
pub type ListCustomerBalanceTransactionsParamsCreated = stripe_types_03::customers::ListCustomersParamsCreated;

#[cfg(feature = "customers")]
#[derive(Debug, Clone, Default)]
pub struct ListCustomerPaymentMethodsParams {
    pub allow_redisplay: Option<stripe_types_00::customers::ListCustomerPaymentMethodsParamsAllowRedisplay>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub r#type: Option<stripe_types_03::customers::ListCustomerPaymentMethodsParamsType>,
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for ListCustomerPaymentMethodsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("allow_redisplay", self.allow_redisplay.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("type", self.r#type.as_ref());
    }
}

#[cfg(feature = "customers")]
pub type UpdateCustomerParamsAddress = stripe_types_00::customers::CreateCustomerParamsAddress;

#[cfg(feature = "customers")]
pub type UpdateCustomerParamsCashBalanceSettingsReconciliationMode =
    stripe_types_04::customers::CreateCustomerParamsCashBalanceSettingsReconciliationMode;

#[cfg(feature = "customers")]
pub type UpdateCustomerParamsShippingAddress = stripe_types_00::customers::CreateCustomerParamsAddress;

#[cfg(feature = "customers")]
#[derive(Debug, Clone, Default)]
pub struct UpdateCustomerParamsTax {
    #[doc = "A recent IP address of the customer used for tax reporting and tax location inference."]
    pub ip_address: Option<json::OwnedValue>,
    #[doc = "A flag that indicates when Stripe should validate the customer tax location."]
    pub validate_location: Option<stripe_types_00::customers::UpdateCustomerParamsTaxValidateLocation>,
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for UpdateCustomerParamsTax {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("validate_location", self.validate_location.as_ref());
    }
}
