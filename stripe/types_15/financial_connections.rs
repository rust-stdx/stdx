#[cfg(any(feature = "events", feature = "financial_connections"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct FinancialConnectionsAccount {
    #[doc = "The account holder that this account belongs to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_holder: Option<crate::payment_methods::BankConnectionsResourceAccountholder>,
    #[doc = "Details about the account numbers."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_numbers: Option<Vec<stripe_types_05::payment_methods::BankConnectionsResourceAccountNumberDetails>>,
    #[doc = "The most recent information about the account's balance."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub balance: Option<stripe_types_06::payment_methods::BankConnectionsResourceBalance>,
    #[doc = "The state of the most recent attempt to refresh the account balance."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub balance_refresh: Option<stripe_types_05::payment_methods::BankConnectionsResourceBalanceRefresh>,
    #[doc = "The type of the account."]
    #[json(default)]
    pub category: stripe_types_01::financial_connections::FinancialConnectionsAccountCategory,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "A human-readable name that has been assigned to this account, either by the account holder or by the institution."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "The name of the institution that holds this account."]
    #[json(default)]
    pub institution_name: String,
    #[doc = "The last 4 digits of the account number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_01::financial_connections::FinancialConnectionsAccountObject,
    #[doc = "The most recent information about the account's owners."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ownership:
        Option<crate::Expandable<stripe_types_07::financial_connections::FinancialConnectionsAccountOwnership>>,
    #[doc = "The state of the most recent attempt to refresh the account owners."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ownership_refresh: Option<stripe_types_05::payment_methods::BankConnectionsResourceOwnershipRefresh>,
    #[doc = "The list of permissions granted by this account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<stripe_types_02::financial_connections::FinancialConnectionsAccountPermissionsItem>>,
    #[doc = "The status of the link to the account."]
    #[json(default)]
    pub status: stripe_types_01::financial_connections::FinancialConnectionsAccountStatus,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub status_details: Option<stripe_types_08::payment_methods::BankConnectionsResourceAccountStatusDetails>,
    #[doc = "If `category` is `cash`, one of:   - `checking`  - `savings`  - `other`  If `category` is `credit`, one of:   - `mort..."]
    #[json(default)]
    pub subcategory: stripe_types_04::financial_connections::FinancialConnectionsAccountSubcategory,
    #[doc = "The list of data refresh subscriptions requested on this account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub subscriptions:
        Option<Vec<stripe_types_00::financial_connections::FinancialConnectionsAccountSubscriptionsItem>>,
    #[doc = "The [PaymentMethod type](https://docs.stripe.com/api/payment_methods/object#payment_method_object-type)(s) that can b..."]
    #[json(default)]
    pub supported_payment_method_types:
        Vec<stripe_types_02::financial_connections::FinancialConnectionsAccountSupportedPaymentMethodTypesItem>,
    #[doc = "The state of the most recent attempt to refresh the account transactions."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transaction_refresh: Option<stripe_types_05::payment_methods::BankConnectionsResourceTransactionRefresh>,
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl crate::PagedItem for FinancialConnectionsAccount {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "financial_connections")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct FinancialConnectionsSession {
    #[doc = "The account holder for whom accounts are collected in this session."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_holder: Option<crate::payment_methods::BankConnectionsResourceAccountholder>,
    #[doc = "The accounts that were collected as part of this Session."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub accounts: Option<crate::List<crate::financial_connections::FinancialConnectionsAccount>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_account_token: Option<crate::tokens::Token>,
    #[doc = "A value that will be passed to the client to launch the authentication flow."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<stripe_types_05::payment_methods::BankConnectionsResourceLinkAccountSessionFilters>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<stripe_types_00::payment_methods::BankConnectionsResourceLinkAccountSessionLimits>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub manual_entry: Option<stripe_types_06::payment_methods::BankConnectionsResourceLinkAccountSessionManualEntry>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_02::financial_connections::FinancialConnectionsSessionObject,
    #[doc = "Permissions requested for accounts collected during this session."]
    #[json(default)]
    pub permissions: Vec<stripe_types_02::financial_connections::FinancialConnectionsAccountPermissionsItem>,
    #[doc = "Data features requested to be retrieved upon account creation."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub prefetch: Option<Vec<stripe_types_01::financial_connections::FinancialConnectionsSessionPrefetchItem>>,
    #[doc = "For webview integrations only."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}

#[cfg(feature = "financial_connections")]
impl crate::PagedItem for FinancialConnectionsSession {
    fn id(&self) -> &str {
        &self.id
    }
}
