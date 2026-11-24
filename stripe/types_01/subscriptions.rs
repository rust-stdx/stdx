#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionParamsAddInvoiceItemsItemPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl CreateSubscriptionParamsAddInvoiceItemsItemPriceDataTaxBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionParamsAddInvoiceItemsItemPriceDataTaxBehavior::Exclusive => "exclusive",
            CreateSubscriptionParamsAddInvoiceItemsItemPriceDataTaxBehavior::Inclusive => "inclusive",
            CreateSubscriptionParamsAddInvoiceItemsItemPriceDataTaxBehavior::Unspecified => "unspecified",
            CreateSubscriptionParamsAddInvoiceItemsItemPriceDataTaxBehavior::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json> for CreateSubscriptionParamsAddInvoiceItemsItemPriceDataTaxBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSubscriptionParamsAddInvoiceItemsItemPriceDataTaxBehavior::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "exclusive" => CreateSubscriptionParamsAddInvoiceItemsItemPriceDataTaxBehavior::Exclusive,
            "inclusive" => CreateSubscriptionParamsAddInvoiceItemsItemPriceDataTaxBehavior::Inclusive,
            "unspecified" => CreateSubscriptionParamsAddInvoiceItemsItemPriceDataTaxBehavior::Unspecified,
            other => CreateSubscriptionParamsAddInvoiceItemsItemPriceDataTaxBehavior::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsAddInvoiceItemsItemPriceDataTaxBehavior {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionParamsBillingModeType {
    Classic,
    Flexible,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl CreateSubscriptionParamsBillingModeType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionParamsBillingModeType::Classic => "classic",
            CreateSubscriptionParamsBillingModeType::Flexible => "flexible",
            CreateSubscriptionParamsBillingModeType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json> for CreateSubscriptionParamsBillingModeType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSubscriptionParamsBillingModeType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "classic" => CreateSubscriptionParamsBillingModeType::Classic,
            "flexible" => CreateSubscriptionParamsBillingModeType::Flexible,
            other => CreateSubscriptionParamsBillingModeType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsBillingModeType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionParamsBillingSchedulesItemBillUntilType {
    Duration,
    Timestamp,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl CreateSubscriptionParamsBillingSchedulesItemBillUntilType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionParamsBillingSchedulesItemBillUntilType::Duration => "duration",
            CreateSubscriptionParamsBillingSchedulesItemBillUntilType::Timestamp => "timestamp",
            CreateSubscriptionParamsBillingSchedulesItemBillUntilType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json> for CreateSubscriptionParamsBillingSchedulesItemBillUntilType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSubscriptionParamsBillingSchedulesItemBillUntilType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "duration" => CreateSubscriptionParamsBillingSchedulesItemBillUntilType::Duration,
            "timestamp" => CreateSubscriptionParamsBillingSchedulesItemBillUntilType::Timestamp,
            other => CreateSubscriptionParamsBillingSchedulesItemBillUntilType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsBillingSchedulesItemBillUntilType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionParamsItemsItemBillingThresholds {
    #[doc = "Number of units that meets the billing threshold to advance the subscription to a new billing period (e.g., it takes ..."]
    pub usage_gte: i64,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsItemsItemBillingThresholds {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("usage_gte", &self.usage_gte);
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionParamsPaymentBehavior {
    AllowIncomplete,
    DefaultIncomplete,
    ErrorIfIncomplete,
    PendingIfIncomplete,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl CreateSubscriptionParamsPaymentBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionParamsPaymentBehavior::AllowIncomplete => "allow_incomplete",
            CreateSubscriptionParamsPaymentBehavior::DefaultIncomplete => "default_incomplete",
            CreateSubscriptionParamsPaymentBehavior::ErrorIfIncomplete => "error_if_incomplete",
            CreateSubscriptionParamsPaymentBehavior::PendingIfIncomplete => "pending_if_incomplete",
            CreateSubscriptionParamsPaymentBehavior::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json> for CreateSubscriptionParamsPaymentBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSubscriptionParamsPaymentBehavior::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "allow_incomplete" => CreateSubscriptionParamsPaymentBehavior::AllowIncomplete,
            "default_incomplete" => CreateSubscriptionParamsPaymentBehavior::DefaultIncomplete,
            "error_if_incomplete" => CreateSubscriptionParamsPaymentBehavior::ErrorIfIncomplete,
            "pending_if_incomplete" => CreateSubscriptionParamsPaymentBehavior::PendingIfIncomplete,
            other => CreateSubscriptionParamsPaymentBehavior::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsPaymentBehavior {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Automatic => {
                "automatic"
            }
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Instant => {
                "instant"
            }
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Microdeposits => {
                "microdeposits"
            }
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Unknown(value) => {
                value
            }
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json>
    for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "automatic" => {
                CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Automatic
            }
            "instant" => {
                CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Instant
            }
            "microdeposits" => {
                CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Microdeposits
            }
            other => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode
    for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::De => "de",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::En => "en",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::Fr => "fr",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::Nl => "nl",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::Unknown(value) => {
                value
            }
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json>
    for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "de" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::De,
            "en" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::En,
            "fr" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::Fr,
            "nl" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::Nl,
            other => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode
    for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType {
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

#[cfg(feature = "subscriptions")]
impl CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::Unnamed => "",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::ChEin => "ch_ein",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::DeHrb => "de_hrb",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::DkCvr => "dk_cvr",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::EsCif => "es_cif",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::FiTunnus => "fi_tunnus",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::FrSiren => "fr_siren",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::FrSiret => "fr_siret",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::ItRea => "it_rea",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::NlKvk => "nl_kvk",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::NoOrgNumber => "no_org_number",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::NoPno => "no_pno",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::SeOrgNumber => "se_org_number",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::SePno => "se_pno",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::UkCrn => "uk_crn",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json>
    for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::Unnamed,
            "ch_ein" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::ChEin,
            "de_hrb" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::DeHrb,
            "dk_cvr" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::DkCvr,
            "es_cif" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::EsCif,
            "fi_tunnus" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::FiTunnus,
            "fr_siren" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::FrSiren,
            "fr_siret" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::FrSiret,
            "it_rea" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::ItRea,
            "nl_kvk" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::NlKvk,
            "no_org_number" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::NoOrgNumber,
            "no_pno" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::NoPno,
            "se_org_number" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::SeOrgNumber,
            "se_pno" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::SePno,
            "uk_crn" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::UkCrn,
            other => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode
    for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegistrationType
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType {
    Fixed,
    Maximum,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType::Fixed => "fixed",
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType::Maximum => {
                "maximum"
            }
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType::Unknown(value) => {
                value
            }
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json>
    for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "fixed" => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType::Fixed,
            "maximum" => {
                CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType::Maximum
            }
            other => CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode
    for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsKonbini {}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsKonbini {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        let _ = form;
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsAmountIncludesIof {
    Always,
    Never,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsAmountIncludesIof {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsAmountIncludesIof::Always => {
                "always"
            }
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsAmountIncludesIof::Never => {
                "never"
            }
            CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsAmountIncludesIof::Unknown(
                value,
            ) => value,
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json>
    for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsAmountIncludesIof
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsAmountIncludesIof::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "always" => {
                CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsAmountIncludesIof::Always
            }
            "never" => {
                CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsAmountIncludesIof::Never
            }
            other => {
                CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsAmountIncludesIof::Unknown(
                    other.to_string(),
                )
            }
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode
    for CreateSubscriptionParamsPaymentSettingsPaymentMethodOptionsPixMandateOptionsAmountIncludesIof
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSubscriptionParamsTrialSettingsEndBehaviorMissingPaymentMethod {
    Cancel,
    CreateInvoice,
    Pause,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl CreateSubscriptionParamsTrialSettingsEndBehaviorMissingPaymentMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSubscriptionParamsTrialSettingsEndBehaviorMissingPaymentMethod::Cancel => "cancel",
            CreateSubscriptionParamsTrialSettingsEndBehaviorMissingPaymentMethod::CreateInvoice => "create_invoice",
            CreateSubscriptionParamsTrialSettingsEndBehaviorMissingPaymentMethod::Pause => "pause",
            CreateSubscriptionParamsTrialSettingsEndBehaviorMissingPaymentMethod::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json> for CreateSubscriptionParamsTrialSettingsEndBehaviorMissingPaymentMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSubscriptionParamsTrialSettingsEndBehaviorMissingPaymentMethod::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "cancel" => CreateSubscriptionParamsTrialSettingsEndBehaviorMissingPaymentMethod::Cancel,
            "create_invoice" => CreateSubscriptionParamsTrialSettingsEndBehaviorMissingPaymentMethod::CreateInvoice,
            "pause" => CreateSubscriptionParamsTrialSettingsEndBehaviorMissingPaymentMethod::Pause,
            other => CreateSubscriptionParamsTrialSettingsEndBehaviorMissingPaymentMethod::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for CreateSubscriptionParamsTrialSettingsEndBehaviorMissingPaymentMethod {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct ListSubscriptionsParamsAutomaticTax {
    #[doc = "Enabled automatic tax calculation which will automatically compute tax rates on all invoices generated by the subscri..."]
    pub enabled: bool,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for ListSubscriptionsParamsAutomaticTax {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, Default)]
pub struct ListSubscriptionsParamsCreated {
    #[doc = "Minimum value to filter by (exclusive)"]
    pub gt: Option<i64>,
    #[doc = "Minimum value to filter by (inclusive)"]
    pub gte: Option<i64>,
    #[doc = "Maximum value to filter by (exclusive)"]
    pub lt: Option<i64>,
    #[doc = "Maximum value to filter by (inclusive)"]
    pub lte: Option<i64>,
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for ListSubscriptionsParamsCreated {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("gt", self.gt.as_ref());
        form.field_opt("gte", self.gte.as_ref());
        form.field_opt("lt", self.lt.as_ref());
        form.field_opt("lte", self.lte.as_ref());
    }
}

#[cfg(feature = "subscriptions")]
#[derive(Debug, Clone, PartialEq)]
pub enum PauseSubscriptionParamsInvoicingBehavior {
    Invoice,
    PendingInvoiceItem,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "subscriptions")]
impl PauseSubscriptionParamsInvoicingBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PauseSubscriptionParamsInvoicingBehavior::Invoice => "invoice",
            PauseSubscriptionParamsInvoicingBehavior::PendingInvoiceItem => "pending_invoice_item",
            PauseSubscriptionParamsInvoicingBehavior::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "subscriptions")]
impl<'json> json::Decode<'json> for PauseSubscriptionParamsInvoicingBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PauseSubscriptionParamsInvoicingBehavior::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "invoice" => PauseSubscriptionParamsInvoicingBehavior::Invoice,
            "pending_invoice_item" => PauseSubscriptionParamsInvoicingBehavior::PendingInvoiceItem,
            other => PauseSubscriptionParamsInvoicingBehavior::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "subscriptions")]
impl crate::params::FormEncode for PauseSubscriptionParamsInvoicingBehavior {
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
pub struct SubscriptionBillingThresholds {
    #[doc = "Monetary threshold that triggers the subscription to create an invoice"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_gte: Option<i64>,
    #[doc = "Indicates if the `billing_cycle_anchor` should be reset when a threshold is reached."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reset_billing_cycle_anchor: Option<bool>,
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
pub struct SubscriptionPaymentMethodOptionsBillieCompanyDetailsRegisteredAddress {
    #[doc = "City, district, suburb, town, or village."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "Two-letter country code."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "Address line 1 (for example, street, PO Box, or company name)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    #[doc = "Address line 2 (for example, apartment, suite, unit, or building)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    #[doc = "ZIP or postal code."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[doc = "State, county, province, or region."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
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
pub enum SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule {
    Halfyearly,
    Monthly,
    Quarterly,
    Weekly,
    Yearly,
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
impl SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule::Halfyearly => "halfyearly",
            SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule::Monthly => "monthly",
            SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule::Quarterly => "quarterly",
            SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule::Weekly => "weekly",
            SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule::Yearly => "yearly",
            SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "halfyearly" => SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule::Halfyearly,
            "monthly" => SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule::Monthly,
            "quarterly" => SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule::Quarterly,
            "weekly" => SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule::Weekly,
            "yearly" => SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule::Yearly,
            other => SubscriptionPaymentMethodOptionsMandateOptionsPixPaymentSchedule::Unknown(other.to_string()),
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
pub enum SubscriptionPendingInvoiceItemIntervalInterval {
    Day,
    Month,
    Week,
    Year,
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
impl SubscriptionPendingInvoiceItemIntervalInterval {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionPendingInvoiceItemIntervalInterval::Day => "day",
            SubscriptionPendingInvoiceItemIntervalInterval::Month => "month",
            SubscriptionPendingInvoiceItemIntervalInterval::Week => "week",
            SubscriptionPendingInvoiceItemIntervalInterval::Year => "year",
            SubscriptionPendingInvoiceItemIntervalInterval::Unknown(value) => value,
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
impl Default for SubscriptionPendingInvoiceItemIntervalInterval {
    fn default() -> Self {
        SubscriptionPendingInvoiceItemIntervalInterval::Unknown(String::new())
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
impl<'json> json::Decode<'json> for SubscriptionPendingInvoiceItemIntervalInterval {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SubscriptionPendingInvoiceItemIntervalInterval::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "day" => SubscriptionPendingInvoiceItemIntervalInterval::Day,
            "month" => SubscriptionPendingInvoiceItemIntervalInterval::Month,
            "week" => SubscriptionPendingInvoiceItemIntervalInterval::Week,
            "year" => SubscriptionPendingInvoiceItemIntervalInterval::Year,
            other => SubscriptionPendingInvoiceItemIntervalInterval::Unknown(other.to_string()),
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
pub enum SubscriptionsResourceBillingSchedulesBillUntilType {
    Duration,
    Timestamp,
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
impl SubscriptionsResourceBillingSchedulesBillUntilType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionsResourceBillingSchedulesBillUntilType::Duration => "duration",
            SubscriptionsResourceBillingSchedulesBillUntilType::Timestamp => "timestamp",
            SubscriptionsResourceBillingSchedulesBillUntilType::Unknown(value) => value,
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
impl Default for SubscriptionsResourceBillingSchedulesBillUntilType {
    fn default() -> Self {
        SubscriptionsResourceBillingSchedulesBillUntilType::Unknown(String::new())
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
impl<'json> json::Decode<'json> for SubscriptionsResourceBillingSchedulesBillUntilType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SubscriptionsResourceBillingSchedulesBillUntilType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "duration" => SubscriptionsResourceBillingSchedulesBillUntilType::Duration,
            "timestamp" => SubscriptionsResourceBillingSchedulesBillUntilType::Timestamp,
            other => SubscriptionsResourceBillingSchedulesBillUntilType::Unknown(other.to_string()),
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
pub enum SubscriptionsResourceTrialSettingsEndBehaviorBillingCycleAnchor {
    Now,
    Unchanged,
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
impl SubscriptionsResourceTrialSettingsEndBehaviorBillingCycleAnchor {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionsResourceTrialSettingsEndBehaviorBillingCycleAnchor::Now => "now",
            SubscriptionsResourceTrialSettingsEndBehaviorBillingCycleAnchor::Unchanged => "unchanged",
            SubscriptionsResourceTrialSettingsEndBehaviorBillingCycleAnchor::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for SubscriptionsResourceTrialSettingsEndBehaviorBillingCycleAnchor {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SubscriptionsResourceTrialSettingsEndBehaviorBillingCycleAnchor::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "now" => SubscriptionsResourceTrialSettingsEndBehaviorBillingCycleAnchor::Now,
            "unchanged" => SubscriptionsResourceTrialSettingsEndBehaviorBillingCycleAnchor::Unchanged,
            other => SubscriptionsResourceTrialSettingsEndBehaviorBillingCycleAnchor::Unknown(other.to_string()),
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
pub struct SubscriptionsTrialsResourceCurrentTrial {
    #[json(default)]
    pub end_date: i64,
    #[json(default)]
    pub start_date: i64,
    #[json(default)]
    pub trial_offer: String,
}
