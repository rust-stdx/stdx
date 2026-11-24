#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionParamsPaymentSettingsSaveDefaultPaymentMethod {
    Off,
    OnSubscription,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl CreateSubscriptionParamsPaymentSettingsSaveDefaultPaymentMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionParamsPaymentSettingsSaveDefaultPaymentMethod::Off => "off",
            CreateSubscriptionParamsPaymentSettingsSaveDefaultPaymentMethod::OnSubscription => "on_subscription",
            CreateSubscriptionParamsPaymentSettingsSaveDefaultPaymentMethod::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json> for CreateSubscriptionParamsPaymentSettingsSaveDefaultPaymentMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSubscriptionParamsPaymentSettingsSaveDefaultPaymentMethod::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "off" => CreateSubscriptionParamsPaymentSettingsSaveDefaultPaymentMethod::Off,
            "on_subscription" => CreateSubscriptionParamsPaymentSettingsSaveDefaultPaymentMethod::OnSubscription,
            other => CreateSubscriptionParamsPaymentSettingsSaveDefaultPaymentMethod::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsPaymentSettingsSaveDefaultPaymentMethod {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionParamsTrialSettingsEndBehaviorBillingCycleAnchor {
    Now,
    Unchanged,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl CreateSubscriptionParamsTrialSettingsEndBehaviorBillingCycleAnchor {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionParamsTrialSettingsEndBehaviorBillingCycleAnchor::Now => "now",
            CreateSubscriptionParamsTrialSettingsEndBehaviorBillingCycleAnchor::Unchanged => "unchanged",
            CreateSubscriptionParamsTrialSettingsEndBehaviorBillingCycleAnchor::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json> for CreateSubscriptionParamsTrialSettingsEndBehaviorBillingCycleAnchor {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSubscriptionParamsTrialSettingsEndBehaviorBillingCycleAnchor::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "now" => CreateSubscriptionParamsTrialSettingsEndBehaviorBillingCycleAnchor::Now,
            "unchanged" => CreateSubscriptionParamsTrialSettingsEndBehaviorBillingCycleAnchor::Unchanged,
            other => CreateSubscriptionParamsTrialSettingsEndBehaviorBillingCycleAnchor::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsTrialSettingsEndBehaviorBillingCycleAnchor {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListSubscriptionsParamsCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl ListSubscriptionsParamsCollectionMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListSubscriptionsParamsCollectionMethod::ChargeAutomatically => "charge_automatically",
            ListSubscriptionsParamsCollectionMethod::SendInvoice => "send_invoice",
            ListSubscriptionsParamsCollectionMethod::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json> for ListSubscriptionsParamsCollectionMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListSubscriptionsParamsCollectionMethod::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "charge_automatically" => ListSubscriptionsParamsCollectionMethod::ChargeAutomatically,
            "send_invoice" => ListSubscriptionsParamsCollectionMethod::SendInvoice,
            other => ListSubscriptionsParamsCollectionMethod::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for ListSubscriptionsParamsCollectionMethod {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum PauseSubscriptionParamsBillForOutstandingUsageThroughType {
    None,
    Now,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl PauseSubscriptionParamsBillForOutstandingUsageThroughType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PauseSubscriptionParamsBillForOutstandingUsageThroughType::None => "none",
            PauseSubscriptionParamsBillForOutstandingUsageThroughType::Now => "now",
            PauseSubscriptionParamsBillForOutstandingUsageThroughType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json> for PauseSubscriptionParamsBillForOutstandingUsageThroughType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PauseSubscriptionParamsBillForOutstandingUsageThroughType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "none" => PauseSubscriptionParamsBillForOutstandingUsageThroughType::None,
            "now" => PauseSubscriptionParamsBillForOutstandingUsageThroughType::Now,
            other => PauseSubscriptionParamsBillForOutstandingUsageThroughType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for PauseSubscriptionParamsBillForOutstandingUsageThroughType {
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
pub enum SubscriptionAutomaticTaxDisabledReason {
    RequiresLocationInputs,
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
impl SubscriptionAutomaticTaxDisabledReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionAutomaticTaxDisabledReason::RequiresLocationInputs => "requires_location_inputs",
            SubscriptionAutomaticTaxDisabledReason::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for SubscriptionAutomaticTaxDisabledReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SubscriptionAutomaticTaxDisabledReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "requires_location_inputs" => SubscriptionAutomaticTaxDisabledReason::RequiresLocationInputs,
            other => SubscriptionAutomaticTaxDisabledReason::Unknown(other.to_string()),
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
pub enum SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType {
    ChEin,
    DeHrb,
    DkCvr,
    EsCif,
    FiTunnus,
    FrSiren,
    FrSiret,
    ItRea,
    NlKvk,
    NoOrgNumber,
    NoPno,
    SeOrgNumber,
    SePno,
    UkCrn,
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
impl SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::ChEin => "ch_ein",
            SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::DeHrb => "de_hrb",
            SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::DkCvr => "dk_cvr",
            SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::EsCif => "es_cif",
            SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::FiTunnus => "fi_tunnus",
            SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::FrSiren => "fr_siren",
            SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::FrSiret => "fr_siret",
            SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::ItRea => "it_rea",
            SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::NlKvk => "nl_kvk",
            SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::NoOrgNumber => "no_org_number",
            SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::NoPno => "no_pno",
            SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::SeOrgNumber => "se_org_number",
            SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::SePno => "se_pno",
            SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::UkCrn => "uk_crn",
            SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ch_ein" => SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::ChEin,
            "de_hrb" => SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::DeHrb,
            "dk_cvr" => SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::DkCvr,
            "es_cif" => SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::EsCif,
            "fi_tunnus" => SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::FiTunnus,
            "fr_siren" => SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::FrSiren,
            "fr_siret" => SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::FrSiret,
            "it_rea" => SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::ItRea,
            "nl_kvk" => SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::NlKvk,
            "no_org_number" => SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::NoOrgNumber,
            "no_pno" => SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::NoPno,
            "se_org_number" => SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::SeOrgNumber,
            "se_pno" => SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::SePno,
            "uk_crn" => SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::UkCrn,
            other => SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegistrationType::Unknown(other.to_string()),
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
pub enum SubscriptionPaymentMethodOptionsMandateOptionsPixAmountIncludesIof {
    Always,
    Never,
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
impl SubscriptionPaymentMethodOptionsMandateOptionsPixAmountIncludesIof {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionPaymentMethodOptionsMandateOptionsPixAmountIncludesIof::Always => "always",
            SubscriptionPaymentMethodOptionsMandateOptionsPixAmountIncludesIof::Never => "never",
            SubscriptionPaymentMethodOptionsMandateOptionsPixAmountIncludesIof::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for SubscriptionPaymentMethodOptionsMandateOptionsPixAmountIncludesIof {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SubscriptionPaymentMethodOptionsMandateOptionsPixAmountIncludesIof::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "always" => SubscriptionPaymentMethodOptionsMandateOptionsPixAmountIncludesIof::Always,
            "never" => SubscriptionPaymentMethodOptionsMandateOptionsPixAmountIncludesIof::Never,
            other => SubscriptionPaymentMethodOptionsMandateOptionsPixAmountIncludesIof::Unknown(other.to_string()),
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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SubscriptionsResourceBillingCycleAnchorConfig {
    #[doc = "The day of the month of the billing_cycle_anchor."]
    #[json(default)]
    pub day_of_month: i64,
    #[doc = "The hour of the day of the billing_cycle_anchor."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub hour: Option<i64>,
    #[doc = "The minute of the hour of the billing_cycle_anchor."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub minute: Option<i64>,
    #[doc = "The month to start full cycle billing periods."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub month: Option<i64>,
    #[doc = "The second of the minute of the billing_cycle_anchor."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub second: Option<i64>,
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
pub enum SubscriptionsResourceBillingModeType {
    Classic,
    Flexible,
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
impl SubscriptionsResourceBillingModeType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionsResourceBillingModeType::Classic => "classic",
            SubscriptionsResourceBillingModeType::Flexible => "flexible",
            SubscriptionsResourceBillingModeType::Unknown(value) => value,
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
impl Default for SubscriptionsResourceBillingModeType {
    fn default() -> Self {
        SubscriptionsResourceBillingModeType::Unknown(String::new())
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
impl<'json> json::Decode<'json> for SubscriptionsResourceBillingModeType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SubscriptionsResourceBillingModeType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "classic" => SubscriptionsResourceBillingModeType::Classic,
            "flexible" => SubscriptionsResourceBillingModeType::Flexible,
            other => SubscriptionsResourceBillingModeType::Unknown(other.to_string()),
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
pub enum SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod {
    Cancel,
    CreateInvoice,
    Pause,
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
impl SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod::Cancel => "cancel",
            SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod::CreateInvoice => "create_invoice",
            SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod::Pause => "pause",
            SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod::Unknown(value) => value,
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
impl Default for SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod {
    fn default() -> Self {
        SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod::Unknown(String::new())
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
impl<'json> json::Decode<'json> for SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "cancel" => SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod::Cancel,
            "create_invoice" => SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod::CreateInvoice,
            "pause" => SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod::Pause,
            other => SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum UpdateSubscriptionParamsCancellationDetailsFeedback {
    Unnamed,
    CustomerService,
    LowQuality,
    MissingFeatures,
    Other,
    SwitchedService,
    TooComplex,
    TooExpensive,
    Unused,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl UpdateSubscriptionParamsCancellationDetailsFeedback {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            UpdateSubscriptionParamsCancellationDetailsFeedback::Unnamed => "",
            UpdateSubscriptionParamsCancellationDetailsFeedback::CustomerService => "customer_service",
            UpdateSubscriptionParamsCancellationDetailsFeedback::LowQuality => "low_quality",
            UpdateSubscriptionParamsCancellationDetailsFeedback::MissingFeatures => "missing_features",
            UpdateSubscriptionParamsCancellationDetailsFeedback::Other => "other",
            UpdateSubscriptionParamsCancellationDetailsFeedback::SwitchedService => "switched_service",
            UpdateSubscriptionParamsCancellationDetailsFeedback::TooComplex => "too_complex",
            UpdateSubscriptionParamsCancellationDetailsFeedback::TooExpensive => "too_expensive",
            UpdateSubscriptionParamsCancellationDetailsFeedback::Unused => "unused",
            UpdateSubscriptionParamsCancellationDetailsFeedback::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json> for UpdateSubscriptionParamsCancellationDetailsFeedback {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(UpdateSubscriptionParamsCancellationDetailsFeedback::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => UpdateSubscriptionParamsCancellationDetailsFeedback::Unnamed,
            "customer_service" => UpdateSubscriptionParamsCancellationDetailsFeedback::CustomerService,
            "low_quality" => UpdateSubscriptionParamsCancellationDetailsFeedback::LowQuality,
            "missing_features" => UpdateSubscriptionParamsCancellationDetailsFeedback::MissingFeatures,
            "other" => UpdateSubscriptionParamsCancellationDetailsFeedback::Other,
            "switched_service" => UpdateSubscriptionParamsCancellationDetailsFeedback::SwitchedService,
            "too_complex" => UpdateSubscriptionParamsCancellationDetailsFeedback::TooComplex,
            "too_expensive" => UpdateSubscriptionParamsCancellationDetailsFeedback::TooExpensive,
            "unused" => UpdateSubscriptionParamsCancellationDetailsFeedback::Unused,
            other => UpdateSubscriptionParamsCancellationDetailsFeedback::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for UpdateSubscriptionParamsCancellationDetailsFeedback {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
