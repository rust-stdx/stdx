#[cfg(any(feature = "accounts", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum CapabilityStatus {
    Active,
    Inactive,
    Pending,
    Unrequested,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "accounts", feature = "events"))]
impl CapabilityStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CapabilityStatus::Active => "active",
            CapabilityStatus::Inactive => "inactive",
            CapabilityStatus::Pending => "pending",
            CapabilityStatus::Unrequested => "unrequested",
            CapabilityStatus::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "accounts", feature = "events"))]
impl Default for CapabilityStatus {
    fn default() -> Self {
        CapabilityStatus::Unknown(String::new())
    }
}

#[cfg(any(feature = "accounts", feature = "events"))]
impl<'json> json::Decode<'json> for CapabilityStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CapabilityStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "active" => CapabilityStatus::Active,
            "inactive" => CapabilityStatus::Inactive,
            "pending" => CapabilityStatus::Pending,
            "unrequested" => CapabilityStatus::Unrequested,
            other => CapabilityStatus::Unknown(other.to_string()),
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
pub enum ConnectAccountReferenceType {
    Account,
    SelfValue,
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
impl ConnectAccountReferenceType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ConnectAccountReferenceType::Account => "account",
            ConnectAccountReferenceType::SelfValue => "self",
            ConnectAccountReferenceType::Unknown(value) => value,
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
impl Default for ConnectAccountReferenceType {
    fn default() -> Self {
        ConnectAccountReferenceType::Unknown(String::new())
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
impl<'json> json::Decode<'json> for ConnectAccountReferenceType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ConnectAccountReferenceType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account" => ConnectAccountReferenceType::Account,
            "self" => ConnectAccountReferenceType::SelfValue,
            other => ConnectAccountReferenceType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConnectEmbeddedBaseFeatures {}

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConnectEmbeddedDisputesListFeatures {
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

#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConnectEmbeddedIssuingCardFeatures {
    #[doc = "Whether to allow card management features."]
    #[json(default)]
    pub card_management: bool,
    #[doc = "Whether to allow card spend dispute management features."]
    #[json(default)]
    pub card_spend_dispute_management: bool,
    #[doc = "Whether to allow cardholder management features."]
    #[json(default)]
    pub cardholder_management: bool,
    #[doc = "Whether to allow spend control management features."]
    #[json(default)]
    pub spend_control_management: bool,
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsCompanyVerificationDocument {
    #[doc = "The back of a document returned by a [file upload](https://docs.stripe.com/api#create_file) with a `purpose` value of..."]
    pub back: Option<String>,
    #[doc = "The front of a document returned by a [file upload](https://docs.stripe.com/api#create_file) with a `purpose` value o..."]
    pub front: Option<String>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsCompanyVerificationDocument {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("back", self.back.as_ref());
        form.field_opt("front", self.front.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsIndividualRelationship {
    #[doc = "Whether the person is a director of the account's legal entity."]
    pub director: Option<bool>,
    #[doc = "Whether the person has significant responsibility to control, manage, or direct the organization."]
    pub executive: Option<bool>,
    #[doc = "Whether the person is an owner of the account’s legal entity."]
    pub owner: Option<bool>,
    #[doc = "The percent owned by the person of the account's legal entity."]
    pub percent_ownership: Option<json::OwnedValue>,
    #[doc = "The person's title (e.g., CEO, Support Engineer)."]
    pub title: Option<String>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsIndividualRelationship {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("director", self.director.as_ref());
        form.field_opt("executive", self.executive.as_ref());
        form.field_opt("owner", self.owner.as_ref());
        form.field_opt("title", self.title.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsSettingsCardPaymentsDeclineOn {
    #[doc = "Whether Stripe automatically declines charges with an incorrect ZIP or postal code."]
    pub avs_failure: Option<bool>,
    #[doc = "Whether Stripe automatically declines charges with an incorrect CVC."]
    pub cvc_failure: Option<bool>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsSettingsCardPaymentsDeclineOn {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("avs_failure", self.avs_failure.as_ref());
        form.field_opt("cvc_failure", self.cvc_failure.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateAccountParamsSettingsPayoutsScheduleWeeklyAnchor {
    Friday,
    Monday,
    Saturday,
    Sunday,
    Thursday,
    Tuesday,
    Wednesday,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "accounts")]
impl CreateAccountParamsSettingsPayoutsScheduleWeeklyAnchor {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateAccountParamsSettingsPayoutsScheduleWeeklyAnchor::Friday => "friday",
            CreateAccountParamsSettingsPayoutsScheduleWeeklyAnchor::Monday => "monday",
            CreateAccountParamsSettingsPayoutsScheduleWeeklyAnchor::Saturday => "saturday",
            CreateAccountParamsSettingsPayoutsScheduleWeeklyAnchor::Sunday => "sunday",
            CreateAccountParamsSettingsPayoutsScheduleWeeklyAnchor::Thursday => "thursday",
            CreateAccountParamsSettingsPayoutsScheduleWeeklyAnchor::Tuesday => "tuesday",
            CreateAccountParamsSettingsPayoutsScheduleWeeklyAnchor::Wednesday => "wednesday",
            CreateAccountParamsSettingsPayoutsScheduleWeeklyAnchor::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "accounts")]
impl<'json> json::Decode<'json> for CreateAccountParamsSettingsPayoutsScheduleWeeklyAnchor {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateAccountParamsSettingsPayoutsScheduleWeeklyAnchor::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "friday" => CreateAccountParamsSettingsPayoutsScheduleWeeklyAnchor::Friday,
            "monday" => CreateAccountParamsSettingsPayoutsScheduleWeeklyAnchor::Monday,
            "saturday" => CreateAccountParamsSettingsPayoutsScheduleWeeklyAnchor::Saturday,
            "sunday" => CreateAccountParamsSettingsPayoutsScheduleWeeklyAnchor::Sunday,
            "thursday" => CreateAccountParamsSettingsPayoutsScheduleWeeklyAnchor::Thursday,
            "tuesday" => CreateAccountParamsSettingsPayoutsScheduleWeeklyAnchor::Tuesday,
            "wednesday" => CreateAccountParamsSettingsPayoutsScheduleWeeklyAnchor::Wednesday,
            other => CreateAccountParamsSettingsPayoutsScheduleWeeklyAnchor::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsSettingsPayoutsScheduleWeeklyAnchor {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateAccountParamsSettingsPaypayPaymentsGoodsType {
    DigitalContent,
    Other,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "accounts")]
impl CreateAccountParamsSettingsPaypayPaymentsGoodsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateAccountParamsSettingsPaypayPaymentsGoodsType::DigitalContent => "digital_content",
            CreateAccountParamsSettingsPaypayPaymentsGoodsType::Other => "other",
            CreateAccountParamsSettingsPaypayPaymentsGoodsType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "accounts")]
impl<'json> json::Decode<'json> for CreateAccountParamsSettingsPaypayPaymentsGoodsType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateAccountParamsSettingsPaypayPaymentsGoodsType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "digital_content" => CreateAccountParamsSettingsPaypayPaymentsGoodsType::DigitalContent,
            "other" => CreateAccountParamsSettingsPaypayPaymentsGoodsType::Other,
            other => CreateAccountParamsSettingsPaypayPaymentsGoodsType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsSettingsPaypayPaymentsGoodsType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsSettingsSepaDebitPayments {
    #[doc = "The business creditor id for european payments."]
    pub creditor_id: Option<String>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsSettingsSepaDebitPayments {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("creditor_id", self.creditor_id.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct ListAccountPersonsParamsRelationship {
    #[doc = "A filter on the list of people returned based on whether these people are authorizers of the account's representative."]
    pub authorizer: Option<bool>,
    #[doc = "A filter on the list of people returned based on whether these people are directors of the account's company."]
    pub director: Option<bool>,
    #[doc = "A filter on the list of people returned based on whether these people are executives of the account's company."]
    pub executive: Option<bool>,
    #[doc = "A filter on the list of people returned based on whether these people are legal guardians of the account's representa..."]
    pub legal_guardian: Option<bool>,
    #[doc = "A filter on the list of people returned based on whether these people are owners of the account's company."]
    pub owner: Option<bool>,
    #[doc = "A filter on the list of people returned based on whether these people are the representative of the account's company."]
    pub representative: Option<bool>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for ListAccountPersonsParamsRelationship {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("authorizer", self.authorizer.as_ref());
        form.field_opt("director", self.director.as_ref());
        form.field_opt("executive", self.executive.as_ref());
        form.field_opt("legal_guardian", self.legal_guardian.as_ref());
        form.field_opt("owner", self.owner.as_ref());
        form.field_opt("representative", self.representative.as_ref());
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
pub enum PersonEthnicityDetailsEthnicityItem {
    Cuban,
    HispanicOrLatino,
    Mexican,
    NotHispanicOrLatino,
    OtherHispanicOrLatino,
    PreferNotToAnswer,
    PuertoRican,
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
impl PersonEthnicityDetailsEthnicityItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PersonEthnicityDetailsEthnicityItem::Cuban => "cuban",
            PersonEthnicityDetailsEthnicityItem::HispanicOrLatino => "hispanic_or_latino",
            PersonEthnicityDetailsEthnicityItem::Mexican => "mexican",
            PersonEthnicityDetailsEthnicityItem::NotHispanicOrLatino => "not_hispanic_or_latino",
            PersonEthnicityDetailsEthnicityItem::OtherHispanicOrLatino => "other_hispanic_or_latino",
            PersonEthnicityDetailsEthnicityItem::PreferNotToAnswer => "prefer_not_to_answer",
            PersonEthnicityDetailsEthnicityItem::PuertoRican => "puerto_rican",
            PersonEthnicityDetailsEthnicityItem::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PersonEthnicityDetailsEthnicityItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PersonEthnicityDetailsEthnicityItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "cuban" => PersonEthnicityDetailsEthnicityItem::Cuban,
            "hispanic_or_latino" => PersonEthnicityDetailsEthnicityItem::HispanicOrLatino,
            "mexican" => PersonEthnicityDetailsEthnicityItem::Mexican,
            "not_hispanic_or_latino" => PersonEthnicityDetailsEthnicityItem::NotHispanicOrLatino,
            "other_hispanic_or_latino" => PersonEthnicityDetailsEthnicityItem::OtherHispanicOrLatino,
            "prefer_not_to_answer" => PersonEthnicityDetailsEthnicityItem::PreferNotToAnswer,
            "puerto_rican" => PersonEthnicityDetailsEthnicityItem::PuertoRican,
            other => PersonEthnicityDetailsEthnicityItem::Unknown(other.to_string()),
        })
    }
}
