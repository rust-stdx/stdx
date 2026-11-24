#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionParamsBillingSchedulesItemAppliesToItemType {
    Price,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl CreateSubscriptionParamsBillingSchedulesItemAppliesToItemType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionParamsBillingSchedulesItemAppliesToItemType::Price => "price",
            CreateSubscriptionParamsBillingSchedulesItemAppliesToItemType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json> for CreateSubscriptionParamsBillingSchedulesItemAppliesToItemType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSubscriptionParamsBillingSchedulesItemAppliesToItemType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "price" => CreateSubscriptionParamsBillingSchedulesItemAppliesToItemType::Price,
            other => CreateSubscriptionParamsBillingSchedulesItemAppliesToItemType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsBillingSchedulesItemAppliesToItemType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBlikMandateOptions {
    #[doc = "Date when the mandate expires and no further payments will be charged."]
    pub expires_at: Option<i64>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBlikMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("expires_at", self.expires_at.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
    Challenge,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::Any => "any",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::Automatic => {
                "automatic"
            }
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::Challenge => {
                "challenge"
            }
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json> for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "any" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::Any,
            "automatic" => {
                CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::Automatic
            }
            "challenge" => {
                CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::Challenge
            }
            other => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose {
    DependantSupport,
    Government,
    Loan,
    Mortgage,
    Other,
    Pension,
    Personal,
    Retail,
    Salary,
    Tax,
    Utility,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::DependantSupport => {
                "dependant_support"
            }
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Government => {
                "government"
            }
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Loan => "loan",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Mortgage => {
                "mortgage"
            }
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Other => "other",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Pension => "pension",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Personal => {
                "personal"
            }
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Retail => "retail",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Salary => "salary",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Tax => "tax",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Utility => "utility",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Unknown(value) => {
                value
            }
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json>
    for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "dependant_support" => {
                CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::DependantSupport
            }
            "government" => {
                CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Government
            }
            "loan" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Loan,
            "mortgage" => {
                CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Mortgage
            }
            "other" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Other,
            "pension" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Pension,
            "personal" => {
                CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Personal
            }
            "retail" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Retail,
            "salary" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Salary,
            "tax" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Tax,
            "utility" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Utility,
            other => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode
    for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem
{
    Checking,
    Savings,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem::Checking => "checking",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem::Savings => "savings",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json> for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "checking" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem::Checking,
            "savings" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem::Savings,
            other => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Balances => "balances",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Ownership => "ownership",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::PaymentMethod => "payment_method",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Transactions => "transactions",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json>
    for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "balances" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Balances,
            "ownership" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Ownership,
            "payment_method" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::PaymentMethod,
            "transactions" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Transactions,
            other => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode
    for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionParamsProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl CreateSubscriptionParamsProrationBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionParamsProrationBehavior::AlwaysInvoice => "always_invoice",
            CreateSubscriptionParamsProrationBehavior::CreateProrations => "create_prorations",
            CreateSubscriptionParamsProrationBehavior::None => "none",
            CreateSubscriptionParamsProrationBehavior::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json> for CreateSubscriptionParamsProrationBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSubscriptionParamsProrationBehavior::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "always_invoice" => CreateSubscriptionParamsProrationBehavior::AlwaysInvoice,
            "create_prorations" => CreateSubscriptionParamsProrationBehavior::CreateProrations,
            "none" => CreateSubscriptionParamsProrationBehavior::None,
            other => CreateSubscriptionParamsProrationBehavior::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsProrationBehavior {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionParamsTransferData {
    #[doc = "A non-negative decimal between 0 and 100, with at most two decimal places."]
    pub amount_percent: Option<f64>,
    #[doc = "ID of an existing, connected Stripe account."]
    pub destination: String,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsTransferData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount_percent", self.amount_percent.as_ref());
        form.field("destination", &self.destination);
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum ResumeSubscriptionParamsPaymentBehavior {
    ResumeOnPaymentAttempt,
    ResumeOnPaymentSuccess,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl ResumeSubscriptionParamsPaymentBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ResumeSubscriptionParamsPaymentBehavior::ResumeOnPaymentAttempt => "resume_on_payment_attempt",
            ResumeSubscriptionParamsPaymentBehavior::ResumeOnPaymentSuccess => "resume_on_payment_success",
            ResumeSubscriptionParamsPaymentBehavior::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json> for ResumeSubscriptionParamsPaymentBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ResumeSubscriptionParamsPaymentBehavior::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "resume_on_payment_attempt" => ResumeSubscriptionParamsPaymentBehavior::ResumeOnPaymentAttempt,
            "resume_on_payment_success" => ResumeSubscriptionParamsPaymentBehavior::ResumeOnPaymentSuccess,
            other => ResumeSubscriptionParamsPaymentBehavior::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for ResumeSubscriptionParamsPaymentBehavior {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
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
#[derive(Debug, Clone, PartialEq)]
pub enum SubscriptionObject {
    Subscription,
    /// A value not known to this version of the crate.
    Unknown(String),
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
impl SubscriptionObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionObject::Subscription => "subscription",
            SubscriptionObject::Unknown(value) => value,
        }
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
impl Default for SubscriptionObject {
    fn default() -> Self {
        SubscriptionObject::Unknown(String::new())
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
impl<'json> json::Decode<'json> for SubscriptionObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SubscriptionObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "subscription" => SubscriptionObject::Subscription,
            other => SubscriptionObject::Unknown(other.to_string()),
        })
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
#[derive(Debug, Clone, PartialEq)]
pub enum SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem {
    AchCreditTransfer,
    AchDebit,
    AcssDebit,
    Affirm,
    Alipay,
    AmazonPay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Billie,
    Blik,
    Boleto,
    Card,
    Cashapp,
    Crypto,
    Custom,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    JpCreditTransfer,
    KakaoPay,
    Klarna,
    Konbini,
    KrCard,
    Link,
    MbWay,
    Multibanco,
    NaverPay,
    NzBankAccount,
    P24,
    PayByBank,
    Payco,
    Paynow,
    Paypal,
    Payto,
    Pix,
    Promptpay,
    RevolutPay,
    Satispay,
    SepaCreditTransfer,
    SepaDebit,
    Sofort,
    Swish,
    Twint,
    Upi,
    UsBankAccount,
    WechatPay,
    /// A value not known to this version of the crate.
    Unknown(String),
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
impl SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::AchCreditTransfer => "ach_credit_transfer",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::AchDebit => "ach_debit",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::AcssDebit => "acss_debit",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Affirm => "affirm",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Alipay => "alipay",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::AmazonPay => "amazon_pay",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::AuBecsDebit => "au_becs_debit",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::BacsDebit => "bacs_debit",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Bancontact => "bancontact",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Billie => "billie",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Blik => "blik",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Boleto => "boleto",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Card => "card",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Cashapp => "cashapp",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Crypto => "crypto",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Custom => "custom",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::CustomerBalance => "customer_balance",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Eps => "eps",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Fpx => "fpx",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Giropay => "giropay",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Grabpay => "grabpay",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Ideal => "ideal",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::JpCreditTransfer => "jp_credit_transfer",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::KakaoPay => "kakao_pay",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Klarna => "klarna",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Konbini => "konbini",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::KrCard => "kr_card",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Link => "link",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::MbWay => "mb_way",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Multibanco => "multibanco",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::NaverPay => "naver_pay",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::NzBankAccount => "nz_bank_account",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::P24 => "p24",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::PayByBank => "pay_by_bank",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Payco => "payco",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Paynow => "paynow",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Paypal => "paypal",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Payto => "payto",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Pix => "pix",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Promptpay => "promptpay",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::RevolutPay => "revolut_pay",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Satispay => "satispay",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::SepaCreditTransfer => "sepa_credit_transfer",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::SepaDebit => "sepa_debit",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Sofort => "sofort",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Swish => "swish",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Twint => "twint",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Upi => "upi",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::UsBankAccount => "us_bank_account",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::WechatPay => "wechat_pay",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Unknown(value) => value,
        }
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
impl<'json> json::Decode<'json> for SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ach_credit_transfer" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::AchCreditTransfer,
            "ach_debit" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::AchDebit,
            "acss_debit" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::AcssDebit,
            "affirm" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Affirm,
            "alipay" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Alipay,
            "amazon_pay" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::AmazonPay,
            "au_becs_debit" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::AuBecsDebit,
            "bacs_debit" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::BacsDebit,
            "bancontact" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Bancontact,
            "billie" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Billie,
            "blik" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Blik,
            "boleto" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Boleto,
            "card" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Card,
            "cashapp" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Cashapp,
            "crypto" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Crypto,
            "custom" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Custom,
            "customer_balance" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::CustomerBalance,
            "eps" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Eps,
            "fpx" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Fpx,
            "giropay" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Giropay,
            "grabpay" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Grabpay,
            "ideal" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Ideal,
            "jp_credit_transfer" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::JpCreditTransfer,
            "kakao_pay" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::KakaoPay,
            "klarna" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Klarna,
            "konbini" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Konbini,
            "kr_card" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::KrCard,
            "link" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Link,
            "mb_way" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::MbWay,
            "multibanco" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Multibanco,
            "naver_pay" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::NaverPay,
            "nz_bank_account" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::NzBankAccount,
            "p24" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::P24,
            "pay_by_bank" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::PayByBank,
            "payco" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Payco,
            "paynow" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Paynow,
            "paypal" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Paypal,
            "payto" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Payto,
            "pix" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Pix,
            "promptpay" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Promptpay,
            "revolut_pay" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::RevolutPay,
            "satispay" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Satispay,
            "sepa_credit_transfer" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::SepaCreditTransfer,
            "sepa_debit" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::SepaDebit,
            "sofort" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Sofort,
            "swish" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Swish,
            "twint" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Twint,
            "upi" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Upi,
            "us_bank_account" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::UsBankAccount,
            "wechat_pay" => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::WechatPay,
            other => SubscriptionsResourcePaymentSettingsPaymentMethodTypesItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum UpdateSubscriptionParamsPauseCollectionBehavior {
    KeepAsDraft,
    MarkUncollectible,
    Void,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl UpdateSubscriptionParamsPauseCollectionBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            UpdateSubscriptionParamsPauseCollectionBehavior::KeepAsDraft => "keep_as_draft",
            UpdateSubscriptionParamsPauseCollectionBehavior::MarkUncollectible => "mark_uncollectible",
            UpdateSubscriptionParamsPauseCollectionBehavior::Void => "void",
            UpdateSubscriptionParamsPauseCollectionBehavior::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json> for UpdateSubscriptionParamsPauseCollectionBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(UpdateSubscriptionParamsPauseCollectionBehavior::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "keep_as_draft" => UpdateSubscriptionParamsPauseCollectionBehavior::KeepAsDraft,
            "mark_uncollectible" => UpdateSubscriptionParamsPauseCollectionBehavior::MarkUncollectible,
            "void" => UpdateSubscriptionParamsPauseCollectionBehavior::Void,
            other => UpdateSubscriptionParamsPauseCollectionBehavior::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for UpdateSubscriptionParamsPauseCollectionBehavior {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
