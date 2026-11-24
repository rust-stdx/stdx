#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConnectEmbeddedFinancialAccountFeatures {
    #[doc = "Whether Stripe user authentication is disabled."]
    #[json(default)]
    pub disable_stripe_user_authentication: bool,
    #[doc = "Whether external account collection is enabled."]
    #[json(default)]
    pub external_account_collection: bool,
    #[doc = "Whether to allow sending money."]
    #[json(default)]
    pub send_money: bool,
    #[doc = "Whether to allow transferring balance."]
    #[json(default)]
    pub transfer_balance: bool,
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConnectEmbeddedPaymentsFeatures {
    #[doc = "Whether to allow capturing and cancelling payment intents."]
    #[json(default)]
    pub capture_payments: bool,
    #[doc = "Whether connected accounts can manage destination charges that are created on behalf of them."]
    #[json(default)]
    pub destination_on_behalf_of_charge_management: bool,
    #[doc = "Whether responding to disputes is enabled, including submitting evidence and accepting disputes."]
    #[json(default)]
    pub dispute_management: bool,
    #[doc = "Whether sending refunds is enabled."]
    #[json(default)]
    pub refund_management: bool,
    #[doc = "Whether to allow connected accounts to submit disputes using Smart Disputes."]
    #[json(default)]
    pub smart_disputes_management: bool,
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountExternalAccountParams {
    #[doc = "When set to true, or if this is the first external account added in this currency, this account becomes the default e..."]
    pub default_for_currency: Option<bool>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "A token, like the ones returned by [Stripe.js](https://docs.stripe.com/js) or a dictionary containing a user's extern..."]
    pub external_account: String,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountExternalAccountParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("default_for_currency", self.default_for_currency.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("external_account", &self.external_account);
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateAccountParamsBusinessProfileMinorityOwnedBusinessDesignationItem {
    LgbtqiOwnedBusiness,
    MinorityOwnedBusiness,
    NoneOfTheseApply,
    PreferNotToAnswer,
    WomenOwnedBusiness,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "accounts")]
impl CreateAccountParamsBusinessProfileMinorityOwnedBusinessDesignationItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateAccountParamsBusinessProfileMinorityOwnedBusinessDesignationItem::LgbtqiOwnedBusiness => {
                "lgbtqi_owned_business"
            }
            CreateAccountParamsBusinessProfileMinorityOwnedBusinessDesignationItem::MinorityOwnedBusiness => {
                "minority_owned_business"
            }
            CreateAccountParamsBusinessProfileMinorityOwnedBusinessDesignationItem::NoneOfTheseApply => {
                "none_of_these_apply"
            }
            CreateAccountParamsBusinessProfileMinorityOwnedBusinessDesignationItem::PreferNotToAnswer => {
                "prefer_not_to_answer"
            }
            CreateAccountParamsBusinessProfileMinorityOwnedBusinessDesignationItem::WomenOwnedBusiness => {
                "women_owned_business"
            }
            CreateAccountParamsBusinessProfileMinorityOwnedBusinessDesignationItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "accounts")]
impl<'json> json::Decode<'json> for CreateAccountParamsBusinessProfileMinorityOwnedBusinessDesignationItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateAccountParamsBusinessProfileMinorityOwnedBusinessDesignationItem::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "lgbtqi_owned_business" => {
                CreateAccountParamsBusinessProfileMinorityOwnedBusinessDesignationItem::LgbtqiOwnedBusiness
            }
            "minority_owned_business" => {
                CreateAccountParamsBusinessProfileMinorityOwnedBusinessDesignationItem::MinorityOwnedBusiness
            }
            "none_of_these_apply" => {
                CreateAccountParamsBusinessProfileMinorityOwnedBusinessDesignationItem::NoneOfTheseApply
            }
            "prefer_not_to_answer" => {
                CreateAccountParamsBusinessProfileMinorityOwnedBusinessDesignationItem::PreferNotToAnswer
            }
            "women_owned_business" => {
                CreateAccountParamsBusinessProfileMinorityOwnedBusinessDesignationItem::WomenOwnedBusiness
            }
            other => CreateAccountParamsBusinessProfileMinorityOwnedBusinessDesignationItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsBusinessProfileMinorityOwnedBusinessDesignationItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsBusinessProfileMonthlyEstimatedRevenue {
    #[doc = "A non-negative integer representing how much to charge in the [smallest currency unit](/currencies#zero-decimal)."]
    pub amount: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsBusinessProfileMonthlyEstimatedRevenue {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        form.field("currency", &self.currency);
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsDocumentsBankAccountOwnershipVerification {
    #[doc = "One or more document ids returned by a [file upload](https://docs.stripe.com/api#create_file) with a `purpose` value ..."]
    pub files: Option<Vec<String>>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsDocumentsBankAccountOwnershipVerification {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.files.as_ref() {
            form.field_array("files", values.as_slice());
        }
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateAccountParamsType {
    Custom,
    Express,
    Standard,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "accounts")]
impl CreateAccountParamsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateAccountParamsType::Custom => "custom",
            CreateAccountParamsType::Express => "express",
            CreateAccountParamsType::Standard => "standard",
            CreateAccountParamsType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "accounts")]
impl<'json> json::Decode<'json> for CreateAccountParamsType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateAccountParamsType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "custom" => CreateAccountParamsType::Custom,
            "express" => CreateAccountParamsType::Express,
            "standard" => CreateAccountParamsType::Standard,
            other => CreateAccountParamsType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsType {
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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PersonAdditionalTosAcceptance {
    #[doc = "The Unix timestamp marking when the legal guardian accepted the service agreement."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<i64>,
    #[doc = "The IP address from which the legal guardian accepted the service agreement."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[doc = "The user agent of the browser from which the legal guardian accepted the service agreement."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
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
pub enum PersonObject {
    Person,
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
impl PersonObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PersonObject::Person => "person",
            PersonObject::Unknown(value) => value,
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
impl Default for PersonObject {
    fn default() -> Self {
        PersonObject::Unknown(String::new())
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
impl<'json> json::Decode<'json> for PersonObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PersonObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "person" => PersonObject::Person,
            other => PersonObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, PartialEq)]
pub enum UpdateAccountExternalAccountParamsAccountType {
    Checking,
    Futsu,
    Savings,
    Toza,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "accounts")]
impl UpdateAccountExternalAccountParamsAccountType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            UpdateAccountExternalAccountParamsAccountType::Checking => "checking",
            UpdateAccountExternalAccountParamsAccountType::Futsu => "futsu",
            UpdateAccountExternalAccountParamsAccountType::Savings => "savings",
            UpdateAccountExternalAccountParamsAccountType::Toza => "toza",
            UpdateAccountExternalAccountParamsAccountType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "accounts")]
impl<'json> json::Decode<'json> for UpdateAccountExternalAccountParamsAccountType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(UpdateAccountExternalAccountParamsAccountType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "checking" => UpdateAccountExternalAccountParamsAccountType::Checking,
            "futsu" => UpdateAccountExternalAccountParamsAccountType::Futsu,
            "savings" => UpdateAccountExternalAccountParamsAccountType::Savings,
            "toza" => UpdateAccountExternalAccountParamsAccountType::Toza,
            other => UpdateAccountExternalAccountParamsAccountType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for UpdateAccountExternalAccountParamsAccountType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
