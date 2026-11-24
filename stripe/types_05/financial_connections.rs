#[cfg(feature = "financial_connections")]
#[derive(Debug, Clone)]
pub struct CreateFinancialConnectionSessionParamsAccountHolder {
    #[doc = "The ID of the Stripe account whose accounts you will retrieve."]
    pub account: Option<String>,
    #[doc = "The ID of the Stripe customer whose accounts you will retrieve."]
    pub customer: Option<String>,
    #[doc = "The ID of Account representing a customer whose accounts you will retrieve."]
    pub customer_account: Option<String>,
    #[doc = "Type of account holder to collect accounts for."]
    pub r#type: stripe_types_00::financial_connections::CreateFinancialConnectionSessionParamsAccountHolderType,
}

#[cfg(feature = "financial_connections")]
impl crate::params::FormEncode for CreateFinancialConnectionSessionParamsAccountHolder {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("customer_account", self.customer_account.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "financial_connections")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct FinancialConnectionsTransaction {
    #[doc = "The ID of the Financial Connections Account this transaction belongs to."]
    #[json(default)]
    pub account: String,
    #[doc = "The amount of this transaction, in cents (or local equivalent)."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "The description of this transaction."]
    #[json(default)]
    pub description: String,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_01::financial_connections::FinancialConnectionsTransactionObject,
    #[doc = "The status of the transaction."]
    #[json(default)]
    pub status: stripe_types_00::financial_connections::FinancialConnectionsTransactionStatus,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub status_transitions:
        Option<stripe_types_03::payment_methods::BankConnectionsResourceTransactionResourceStatusTransitions>,
    #[doc = "Time at which the transaction was transacted."]
    #[json(default)]
    pub transacted_at: i64,
    #[doc = "The token of the transaction refresh that last updated or created this transaction."]
    #[json(default)]
    pub transaction_refresh: String,
    #[doc = "Time at which the object was last updated."]
    #[json(default)]
    pub updated: i64,
}

#[cfg(feature = "financial_connections")]
impl crate::PagedItem for FinancialConnectionsTransaction {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "financial_connections")]
pub type GetFinancialConnectionTransactionParams =
    stripe_types_03::financial_connections::GetFinancialConnectionAccountParams;

#[cfg(feature = "financial_connections")]
#[derive(Debug, Clone, Default)]
pub struct ListFinancialConnectionTransactionsParams {
    pub account: String,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub transacted_at:
        Option<stripe_types_01::financial_connections::ListFinancialConnectionTransactionsParamsTransactedAt>,
    pub transaction_refresh:
        Option<stripe_types_04::financial_connections::ListFinancialConnectionTransactionsParamsTransactionRefresh>,
}

#[cfg(feature = "financial_connections")]
impl crate::params::FormEncode for ListFinancialConnectionTransactionsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("account", &self.account);
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("transacted_at", self.transacted_at.as_ref());
        form.field_opt("transaction_refresh", self.transaction_refresh.as_ref());
    }
}

#[cfg(feature = "financial_connections")]
pub type UnsubscribeFinancialConnectionAccountParamsFeaturesItem =
    stripe_types_02::financial_connections::SubscribeFinancialConnectionAccountParamsFeaturesItem;
