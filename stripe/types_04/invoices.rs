#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateInvoiceParamsFromInvoiceAction {
    Revision,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreateInvoiceParamsFromInvoiceAction {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateInvoiceParamsFromInvoiceAction::Revision => "revision",
            CreateInvoiceParamsFromInvoiceAction::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for CreateInvoiceParamsFromInvoiceAction {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateInvoiceParamsFromInvoiceAction::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "revision" => CreateInvoiceParamsFromInvoiceAction::Revision,
            other => CreateInvoiceParamsFromInvoiceAction::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsFromInvoiceAction {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::De => "de",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::En => "en",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::Fr => "fr",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::Nl => "nl",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "de" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::De,
            "en" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::En,
            "fr" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::Fr,
            "nl" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::Nl,
            other => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType {
    Unnamed,
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

#[cfg(feature = "invoices")]
impl CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::Unnamed => "",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::ChEin => {
                "ch_ein"
            }
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::DeHrb => {
                "de_hrb"
            }
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::DkCvr => {
                "dk_cvr"
            }
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::EsCif => {
                "es_cif"
            }
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::FiTunnus => {
                "fi_tunnus"
            }
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::FrSiren => {
                "fr_siren"
            }
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::FrSiret => {
                "fr_siret"
            }
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::ItRea => {
                "it_rea"
            }
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::NlKvk => {
                "nl_kvk"
            }
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::NoOrgNumber => {
                "no_org_number"
            }
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::NoPno => {
                "no_pno"
            }
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::SeOrgNumber => {
                "se_org_number"
            }
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::SePno => {
                "se_pno"
            }
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::UkCrn => {
                "uk_crn"
            }
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::Unknown(
                value,
            ) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json>
    for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::Unnamed,
            "ch_ein" => {
                CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::ChEin
            }
            "de_hrb" => {
                CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::DeHrb
            }
            "dk_cvr" => {
                CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::DkCvr
            }
            "es_cif" => {
                CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::EsCif
            }
            "fi_tunnus" => {
                CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::FiTunnus
            }
            "fr_siren" => {
                CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::FrSiren
            }
            "fr_siret" => {
                CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::FrSiret
            }
            "it_rea" => {
                CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::ItRea
            }
            "nl_kvk" => {
                CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::NlKvk
            }
            "no_org_number" => {
                CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::NoOrgNumber
            }
            "no_pno" => {
                CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::NoPno
            }
            "se_org_number" => {
                CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::SeOrgNumber
            }
            "se_pno" => {
                CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::SePno
            }
            "uk_crn" => {
                CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::UkCrn
            }
            other => {
                CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::Unknown(
                    other.to_string(),
                )
            }
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode
    for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateInvoiceParamsRenderingAmountTaxDisplay {
    Unnamed,
    ExcludeTax,
    IncludeInclusiveTax,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreateInvoiceParamsRenderingAmountTaxDisplay {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateInvoiceParamsRenderingAmountTaxDisplay::Unnamed => "",
            CreateInvoiceParamsRenderingAmountTaxDisplay::ExcludeTax => "exclude_tax",
            CreateInvoiceParamsRenderingAmountTaxDisplay::IncludeInclusiveTax => "include_inclusive_tax",
            CreateInvoiceParamsRenderingAmountTaxDisplay::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for CreateInvoiceParamsRenderingAmountTaxDisplay {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateInvoiceParamsRenderingAmountTaxDisplay::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreateInvoiceParamsRenderingAmountTaxDisplay::Unnamed,
            "exclude_tax" => CreateInvoiceParamsRenderingAmountTaxDisplay::ExcludeTax,
            "include_inclusive_tax" => CreateInvoiceParamsRenderingAmountTaxDisplay::IncludeInclusiveTax,
            other => CreateInvoiceParamsRenderingAmountTaxDisplay::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsRenderingAmountTaxDisplay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePreviewParamsCustomerDetailsTaxExempt {
    Unnamed,
    Exempt,
    None,
    Reverse,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreatePreviewParamsCustomerDetailsTaxExempt {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePreviewParamsCustomerDetailsTaxExempt::Unnamed => "",
            CreatePreviewParamsCustomerDetailsTaxExempt::Exempt => "exempt",
            CreatePreviewParamsCustomerDetailsTaxExempt::None => "none",
            CreatePreviewParamsCustomerDetailsTaxExempt::Reverse => "reverse",
            CreatePreviewParamsCustomerDetailsTaxExempt::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for CreatePreviewParamsCustomerDetailsTaxExempt {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePreviewParamsCustomerDetailsTaxExempt::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreatePreviewParamsCustomerDetailsTaxExempt::Unnamed,
            "exempt" => CreatePreviewParamsCustomerDetailsTaxExempt::Exempt,
            "none" => CreatePreviewParamsCustomerDetailsTaxExempt::None,
            "reverse" => CreatePreviewParamsCustomerDetailsTaxExempt::Reverse,
            other => CreatePreviewParamsCustomerDetailsTaxExempt::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsCustomerDetailsTaxExempt {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemDiscountsItem {
    #[doc = "ID of the coupon to create a new discount for."]
    pub coupon: Option<String>,
    #[doc = "ID of an existing discount on the object (or one of its ancestors) to reuse."]
    pub discount: Option<String>,
    #[doc = "ID of the promotion code to create a new discount for."]
    pub promotion_code: Option<String>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsScheduleDetailsPhasesItemAddInvoiceItemsItemDiscountsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("coupon", self.coupon.as_ref());
        form.field_opt("discount", self.discount.as_ref());
        form.field_opt("promotion_code", self.promotion_code.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePreviewParamsScheduleDetailsPhasesItemProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreatePreviewParamsScheduleDetailsPhasesItemProrationBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePreviewParamsScheduleDetailsPhasesItemProrationBehavior::AlwaysInvoice => "always_invoice",
            CreatePreviewParamsScheduleDetailsPhasesItemProrationBehavior::CreateProrations => "create_prorations",
            CreatePreviewParamsScheduleDetailsPhasesItemProrationBehavior::None => "none",
            CreatePreviewParamsScheduleDetailsPhasesItemProrationBehavior::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for CreatePreviewParamsScheduleDetailsPhasesItemProrationBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePreviewParamsScheduleDetailsPhasesItemProrationBehavior::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "always_invoice" => CreatePreviewParamsScheduleDetailsPhasesItemProrationBehavior::AlwaysInvoice,
            "create_prorations" => CreatePreviewParamsScheduleDetailsPhasesItemProrationBehavior::CreateProrations,
            "none" => CreatePreviewParamsScheduleDetailsPhasesItemProrationBehavior::None,
            other => CreatePreviewParamsScheduleDetailsPhasesItemProrationBehavior::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsScheduleDetailsPhasesItemProrationBehavior {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePreviewParamsSubscriptionDetailsPauseType {
    Subscription,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreatePreviewParamsSubscriptionDetailsPauseType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePreviewParamsSubscriptionDetailsPauseType::Subscription => "subscription",
            CreatePreviewParamsSubscriptionDetailsPauseType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for CreatePreviewParamsSubscriptionDetailsPauseType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePreviewParamsSubscriptionDetailsPauseType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "subscription" => CreatePreviewParamsSubscriptionDetailsPauseType::Subscription,
            other => CreatePreviewParamsSubscriptionDetailsPauseType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsSubscriptionDetailsPauseType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(any(feature = "events", feature = "invoiceitems"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CreditedItemsInvoiceLineItems {
    #[doc = "The invoice id for the debited line item(s)."]
    #[json(default)]
    pub invoice: String,
    #[doc = "IDs of the debited invoice line item(s) on the invoice that correspond to the credit proration."]
    #[json(default)]
    pub invoice_line_items: Vec<String>,
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
pub struct InvoiceInstallmentsCard {
    #[doc = "Whether Installments are enabled for this Invoice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
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
pub enum InvoiceObject {
    Invoice,
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
impl InvoiceObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InvoiceObject::Invoice => "invoice",
            InvoiceObject::Unknown(value) => value,
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
impl Default for InvoiceObject {
    fn default() -> Self {
        InvoiceObject::Unknown(String::new())
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
impl<'json> json::Decode<'json> for InvoiceObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InvoiceObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "invoice" => InvoiceObject::Invoice,
            other => InvoiceObject::Unknown(other.to_string()),
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
pub enum InvoiceRenderingPdfPageSize {
    A4,
    Auto,
    Letter,
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
impl InvoiceRenderingPdfPageSize {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InvoiceRenderingPdfPageSize::A4 => "a4",
            InvoiceRenderingPdfPageSize::Auto => "auto",
            InvoiceRenderingPdfPageSize::Letter => "letter",
            InvoiceRenderingPdfPageSize::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for InvoiceRenderingPdfPageSize {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InvoiceRenderingPdfPageSize::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "a4" => InvoiceRenderingPdfPageSize::A4,
            "auto" => InvoiceRenderingPdfPageSize::Auto,
            "letter" => InvoiceRenderingPdfPageSize::Letter,
            other => InvoiceRenderingPdfPageSize::Unknown(other.to_string()),
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
pub enum InvoiceStatus {
    Draft,
    Open,
    Paid,
    Uncollectible,
    Void,
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
impl InvoiceStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InvoiceStatus::Draft => "draft",
            InvoiceStatus::Open => "open",
            InvoiceStatus::Paid => "paid",
            InvoiceStatus::Uncollectible => "uncollectible",
            InvoiceStatus::Void => "void",
            InvoiceStatus::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for InvoiceStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InvoiceStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "draft" => InvoiceStatus::Draft,
            "open" => InvoiceStatus::Open,
            "paid" => InvoiceStatus::Paid,
            "uncollectible" => InvoiceStatus::Uncollectible,
            "void" => InvoiceStatus::Void,
            other => InvoiceStatus::Unknown(other.to_string()),
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
pub enum LineItemObject {
    LineItem,
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
impl LineItemObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            LineItemObject::LineItem => "line_item",
            LineItemObject::Unknown(value) => value,
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
impl Default for LineItemObject {
    fn default() -> Self {
        LineItemObject::Unknown(String::new())
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
impl<'json> json::Decode<'json> for LineItemObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(LineItemObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "line_item" => LineItemObject::LineItem,
            other => LineItemObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct ListInvoiceLinesParams {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for ListInvoiceLinesParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListInvoicesParamsCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl ListInvoicesParamsCollectionMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListInvoicesParamsCollectionMethod::ChargeAutomatically => "charge_automatically",
            ListInvoicesParamsCollectionMethod::SendInvoice => "send_invoice",
            ListInvoicesParamsCollectionMethod::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for ListInvoicesParamsCollectionMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListInvoicesParamsCollectionMethod::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "charge_automatically" => ListInvoicesParamsCollectionMethod::ChargeAutomatically,
            "send_invoice" => ListInvoicesParamsCollectionMethod::SendInvoice,
            other => ListInvoicesParamsCollectionMethod::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for ListInvoicesParamsCollectionMethod {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct SearchInvoicesParams {
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub page: Option<String>,
    pub query: String,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for SearchInvoicesParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("page", self.page.as_ref());
        form.field("query", &self.query);
    }
}
