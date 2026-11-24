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
pub struct CashBalance {
    #[doc = "A hash of all cash balances available to this customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub available: Option<std::collections::HashMap<String, i64>>,
    #[doc = "The ID of the customer whose cash balance this object represents."]
    #[json(default)]
    pub customer: String,
    #[doc = "The ID of an Account representing a customer whose cash balance this object represents."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_account: Option<String>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::customers::CashBalanceObject,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<stripe_types_05::customers::CustomerBalanceCustomerBalanceSettings>,
}

#[cfg(feature = "customers")]
#[derive(Debug, Clone, Default)]
pub struct CreateCustomerParamsCashBalance {
    #[doc = "Settings controlling the behavior of the customer's cash balance, such as reconciliation of funds received."]
    pub settings: Option<stripe_types_05::customers::CreateCustomerParamsCashBalanceSettings>,
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for CreateCustomerParamsCashBalance {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("settings", self.settings.as_ref());
    }
}

#[cfg(feature = "customers")]
#[derive(Debug, Clone, Default)]
pub struct CreateCustomerParamsInvoiceSettings {
    #[doc = "The list of up to 4 default custom fields to be displayed on invoices for this customer."]
    pub custom_fields: Option<json::OwnedValue>,
    #[doc = "ID of a payment method that's attached to the customer, to be used as the customer's default payment method for subsc..."]
    pub default_payment_method: Option<String>,
    #[doc = "Default footer to be displayed on invoices for this customer."]
    pub footer: Option<String>,
    #[doc = "Default options for invoice PDF rendering for this customer."]
    pub rendering_options: Option<stripe_types_05::customers::CreateCustomerParamsInvoiceSettingsRenderingOptions>,
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for CreateCustomerParamsInvoiceSettings {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("default_payment_method", self.default_payment_method.as_ref());
        form.field_opt("footer", self.footer.as_ref());
        form.field_opt("rendering_options", self.rendering_options.as_ref());
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
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransfer {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub eu_bank_transfer: Option<stripe_types_00::customers::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceEuBankTransfer>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub gb_bank_transfer: Option<stripe_types_04::customers::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransfer>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub jp_bank_transfer: Option<stripe_types_02::customers::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceJpBankTransfer>,
    #[doc = "The user-supplied reference field on the bank transfer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[doc = "The funding method type used to fund the customer balance."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::customers::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferType,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub us_bank_transfer: Option<stripe_types_05::customers::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceUsBankTransfer>,
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
pub struct CustomerTax {
    #[doc = "Surfaces if automatic tax computation is possible given the current customer location information."]
    #[json(default)]
    pub automatic_tax: stripe_types_01::customers::CustomerTaxAutomaticTax,
    #[doc = "A recent IP address of the customer used for tax reporting and tax location inference."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "The identified tax location of the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<stripe_types_05::customers::CustomerTaxLocation>,
    #[doc = "The tax calculation provider used for location resolution."]
    #[json(default)]
    pub provider: stripe_types_04::customers::CustomerTaxProvider,
}

#[cfg(feature = "customers")]
#[derive(Debug, Clone)]
pub struct UpdateCustomerParamsShipping {
    #[doc = "Customer shipping address."]
    pub address: stripe_types_06::customers::UpdateCustomerParamsShippingAddress,
    #[doc = "Customer name."]
    pub name: String,
    #[doc = "Customer phone (including extension)."]
    pub phone: Option<String>,
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for UpdateCustomerParamsShipping {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("address", &self.address);
        form.field("name", &self.name);
        form.field_opt("phone", self.phone.as_ref());
    }
}
