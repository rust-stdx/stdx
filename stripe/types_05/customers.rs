#[cfg(feature = "customers")]
#[derive(Debug, Clone, Default)]
pub struct CreateCustomerParamsCashBalanceSettings {
    #[doc = "Controls how funds transferred by the customer are applied to payment intents and invoices."]
    pub reconciliation_mode:
        Option<stripe_types_04::customers::CreateCustomerParamsCashBalanceSettingsReconciliationMode>,
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for CreateCustomerParamsCashBalanceSettings {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("reconciliation_mode", self.reconciliation_mode.as_ref());
    }
}

#[cfg(feature = "customers")]
#[derive(Debug, Clone, Default)]
pub struct CreateCustomerParamsInvoiceSettingsRenderingOptions {
    #[doc = "How line-item prices and amounts will be displayed with respect to tax on invoice PDFs."]
    pub amount_tax_display:
        Option<stripe_types_00::customers::CreateCustomerParamsInvoiceSettingsRenderingOptionsAmountTaxDisplay>,
    #[doc = "ID of the invoice rendering template to use for future invoices."]
    pub template: Option<String>,
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for CreateCustomerParamsInvoiceSettingsRenderingOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount_tax_display", self.amount_tax_display.as_ref());
        form.field_opt("template", self.template.as_ref());
    }
}

#[cfg(feature = "customers")]
pub type CreateCustomerParamsShippingAddress = stripe_types_00::customers::CreateCustomerParamsAddress;

#[cfg(feature = "customers")]
#[derive(Debug, Clone, Default)]
pub struct CreateCustomerParamsTax {
    #[doc = "A recent IP address of the customer used for tax reporting and tax location inference."]
    pub ip_address: Option<json::OwnedValue>,
    #[doc = "A flag that indicates when Stripe should validate the customer tax location."]
    pub validate_location: Option<stripe_types_01::customers::CreateCustomerParamsTaxValidateLocation>,
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for CreateCustomerParamsTax {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("validate_location", self.validate_location.as_ref());
    }
}

#[cfg(feature = "customers")]
#[derive(Debug, Clone)]
pub struct CreateCustomerParamsTaxIdDataItem {
    #[doc = "Type of the tax ID, one of `ad_nrt`, `ae_trn`, `al_tin`, `am_tin`, `ao_tin`, `ar_cuit`, `au_abn`, `au_arn`, `aw_tin`,..."]
    pub r#type: stripe_types_02::customers::CreateCustomerParamsTaxIdDataItemType,
    #[doc = "Value of the tax ID."]
    pub value: String,
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for CreateCustomerParamsTaxIdDataItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("type", &self.r#type);
        form.field("value", &self.value);
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
pub struct CustomerBalanceCustomerBalanceSettings {
    #[doc = "The configuration for how funds that land in the customer cash balance are reconciled."]
    #[json(default)]
    pub reconciliation_mode: stripe_types_03::customers::CustomerBalanceCustomerBalanceSettingsReconciliationMode,
    #[doc = "A flag to indicate if reconciliation mode returned is the user's default or is specific to this customer cash balance"]
    #[json(default)]
    pub using_merchant_default: bool,
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
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceUsBankTransfer {
    #[doc = "The banking network used for this funding."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<stripe_types_00::customers::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceUsBankTransferNetwork>,
    #[doc = "The full name of the sender, as supplied by the sending bank."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
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
pub struct CustomerTaxLocation {
    #[doc = "The identified tax country of the customer."]
    #[json(default)]
    pub country: String,
    #[doc = "The data source used to infer the customer's location."]
    #[json(default)]
    pub source: stripe_types_03::customers::CustomerTaxLocationSource,
    #[doc = "The identified tax state, county, province, or region of the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[cfg(feature = "customers")]
pub type DeleteCustomerSourceParams = stripe_types_04::customers::GetCustomerParams;

#[cfg(feature = "customers")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct DeletedCustomer {
    #[doc = "Always true for a deleted object"]
    #[json(default)]
    pub deleted: bool,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_02::customers::CustomerObject,
}

#[cfg(feature = "customers")]
impl crate::PagedItem for DeletedCustomer {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "customers")]
pub type GetCustomerBalanceTransactionParams = stripe_types_04::customers::GetCustomerParams;

#[cfg(feature = "customers")]
pub type GetCustomerCashBalanceParams = stripe_types_04::customers::GetCustomerParams;

#[cfg(feature = "customers")]
pub type GetCustomerCashBalanceTransactionParams = stripe_types_04::customers::GetCustomerParams;

#[cfg(feature = "customers")]
pub type GetCustomerTaxIdParams = stripe_types_04::customers::GetCustomerParams;

#[cfg(feature = "customers")]
pub type ListCustomerTaxIdsParams = stripe_types_00::customers::ListCustomerCashBalanceTransactionsParams;

#[cfg(feature = "customers")]
#[derive(Debug, Clone, Default)]
pub struct ListCustomersParams {
    pub created: Option<stripe_types_03::customers::ListCustomersParamsCreated>,
    pub email: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub test_clock: Option<String>,
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for ListCustomersParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("created", self.created.as_ref());
        form.field_opt("email", self.email.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("test_clock", self.test_clock.as_ref());
    }
}

#[cfg(feature = "customers")]
pub type UpdateCustomerCashBalanceParamsSettingsReconciliationMode =
    stripe_types_04::customers::CreateCustomerParamsCashBalanceSettingsReconciliationMode;

#[cfg(feature = "customers")]
pub type UpdateCustomerParamsInvoiceSettingsRenderingOptionsAmountTaxDisplay =
    stripe_types_00::customers::CreateCustomerParamsInvoiceSettingsRenderingOptionsAmountTaxDisplay;

#[cfg(feature = "customers")]
pub type UpdateCustomerParamsTaxExempt = stripe_types_01::customers::CreateCustomerParamsTaxExempt;

#[cfg(feature = "customers")]
pub type UpdateCustomerSourceParamsOwnerAddress = stripe_types_00::customers::CreateCustomerParamsAddress;
