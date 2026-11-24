#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CapturePaymentIntentParamsTransferData {
    #[doc = "The amount that will be transferred automatically when a charge succeeds."]
    pub amount: Option<i64>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CapturePaymentIntentParamsTransferData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodDataNzBankAccount {
    #[doc = "The name on the bank account."]
    pub account_holder_name: Option<String>,
    #[doc = "The account number for the bank account."]
    pub account_number: String,
    #[doc = "The numeric code for the bank account's bank."]
    pub bank_code: String,
    #[doc = "The numeric code for the bank account's bank branch."]
    pub branch_code: String,
    pub reference: Option<String>,
    #[doc = "The suffix of the bank account number."]
    pub suffix: String,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodDataNzBankAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account_holder_name", self.account_holder_name.as_ref());
        form.field("account_number", &self.account_number);
        form.field("bank_code", &self.bank_code);
        form.field("branch_code", &self.branch_code);
        form.field_opt("reference", self.reference.as_ref());
        form.field("suffix", &self.suffix);
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodDataP24Bank {
    AliorBank,
    BankMillennium,
    BankNowyBfgSa,
    BankPekaoSa,
    BankiSpbdzielcze,
    Blik,
    BnpParibas,
    Boz,
    CitiHandlowy,
    CreditAgricole,
    Envelobank,
    EtransferPocztowy24,
    GetinBank,
    Ideabank,
    Ing,
    Inteligo,
    MbankMtransfer,
    NestPrzelew,
    NoblePay,
    PbacZIpko,
    PlusBank,
    SantanderPrzelew24,
    TmobileUsbugiBankowe,
    ToyotaBank,
    Velobank,
    VolkswagenBank,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodDataP24Bank {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodDataP24Bank::AliorBank => "alior_bank",
            CreatePaymentIntentParamsPaymentMethodDataP24Bank::BankMillennium => "bank_millennium",
            CreatePaymentIntentParamsPaymentMethodDataP24Bank::BankNowyBfgSa => "bank_nowy_bfg_sa",
            CreatePaymentIntentParamsPaymentMethodDataP24Bank::BankPekaoSa => "bank_pekao_sa",
            CreatePaymentIntentParamsPaymentMethodDataP24Bank::BankiSpbdzielcze => "banki_spbdzielcze",
            CreatePaymentIntentParamsPaymentMethodDataP24Bank::Blik => "blik",
            CreatePaymentIntentParamsPaymentMethodDataP24Bank::BnpParibas => "bnp_paribas",
            CreatePaymentIntentParamsPaymentMethodDataP24Bank::Boz => "boz",
            CreatePaymentIntentParamsPaymentMethodDataP24Bank::CitiHandlowy => "citi_handlowy",
            CreatePaymentIntentParamsPaymentMethodDataP24Bank::CreditAgricole => "credit_agricole",
            CreatePaymentIntentParamsPaymentMethodDataP24Bank::Envelobank => "envelobank",
            CreatePaymentIntentParamsPaymentMethodDataP24Bank::EtransferPocztowy24 => "etransfer_pocztowy24",
            CreatePaymentIntentParamsPaymentMethodDataP24Bank::GetinBank => "getin_bank",
            CreatePaymentIntentParamsPaymentMethodDataP24Bank::Ideabank => "ideabank",
            CreatePaymentIntentParamsPaymentMethodDataP24Bank::Ing => "ing",
            CreatePaymentIntentParamsPaymentMethodDataP24Bank::Inteligo => "inteligo",
            CreatePaymentIntentParamsPaymentMethodDataP24Bank::MbankMtransfer => "mbank_mtransfer",
            CreatePaymentIntentParamsPaymentMethodDataP24Bank::NestPrzelew => "nest_przelew",
            CreatePaymentIntentParamsPaymentMethodDataP24Bank::NoblePay => "noble_pay",
            CreatePaymentIntentParamsPaymentMethodDataP24Bank::PbacZIpko => "pbac_z_ipko",
            CreatePaymentIntentParamsPaymentMethodDataP24Bank::PlusBank => "plus_bank",
            CreatePaymentIntentParamsPaymentMethodDataP24Bank::SantanderPrzelew24 => "santander_przelew24",
            CreatePaymentIntentParamsPaymentMethodDataP24Bank::TmobileUsbugiBankowe => "tmobile_usbugi_bankowe",
            CreatePaymentIntentParamsPaymentMethodDataP24Bank::ToyotaBank => "toyota_bank",
            CreatePaymentIntentParamsPaymentMethodDataP24Bank::Velobank => "velobank",
            CreatePaymentIntentParamsPaymentMethodDataP24Bank::VolkswagenBank => "volkswagen_bank",
            CreatePaymentIntentParamsPaymentMethodDataP24Bank::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodDataP24Bank {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsPaymentMethodDataP24Bank::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "alior_bank" => CreatePaymentIntentParamsPaymentMethodDataP24Bank::AliorBank,
            "bank_millennium" => CreatePaymentIntentParamsPaymentMethodDataP24Bank::BankMillennium,
            "bank_nowy_bfg_sa" => CreatePaymentIntentParamsPaymentMethodDataP24Bank::BankNowyBfgSa,
            "bank_pekao_sa" => CreatePaymentIntentParamsPaymentMethodDataP24Bank::BankPekaoSa,
            "banki_spbdzielcze" => CreatePaymentIntentParamsPaymentMethodDataP24Bank::BankiSpbdzielcze,
            "blik" => CreatePaymentIntentParamsPaymentMethodDataP24Bank::Blik,
            "bnp_paribas" => CreatePaymentIntentParamsPaymentMethodDataP24Bank::BnpParibas,
            "boz" => CreatePaymentIntentParamsPaymentMethodDataP24Bank::Boz,
            "citi_handlowy" => CreatePaymentIntentParamsPaymentMethodDataP24Bank::CitiHandlowy,
            "credit_agricole" => CreatePaymentIntentParamsPaymentMethodDataP24Bank::CreditAgricole,
            "envelobank" => CreatePaymentIntentParamsPaymentMethodDataP24Bank::Envelobank,
            "etransfer_pocztowy24" => CreatePaymentIntentParamsPaymentMethodDataP24Bank::EtransferPocztowy24,
            "getin_bank" => CreatePaymentIntentParamsPaymentMethodDataP24Bank::GetinBank,
            "ideabank" => CreatePaymentIntentParamsPaymentMethodDataP24Bank::Ideabank,
            "ing" => CreatePaymentIntentParamsPaymentMethodDataP24Bank::Ing,
            "inteligo" => CreatePaymentIntentParamsPaymentMethodDataP24Bank::Inteligo,
            "mbank_mtransfer" => CreatePaymentIntentParamsPaymentMethodDataP24Bank::MbankMtransfer,
            "nest_przelew" => CreatePaymentIntentParamsPaymentMethodDataP24Bank::NestPrzelew,
            "noble_pay" => CreatePaymentIntentParamsPaymentMethodDataP24Bank::NoblePay,
            "pbac_z_ipko" => CreatePaymentIntentParamsPaymentMethodDataP24Bank::PbacZIpko,
            "plus_bank" => CreatePaymentIntentParamsPaymentMethodDataP24Bank::PlusBank,
            "santander_przelew24" => CreatePaymentIntentParamsPaymentMethodDataP24Bank::SantanderPrzelew24,
            "tmobile_usbugi_bankowe" => CreatePaymentIntentParamsPaymentMethodDataP24Bank::TmobileUsbugiBankowe,
            "toyota_bank" => CreatePaymentIntentParamsPaymentMethodDataP24Bank::ToyotaBank,
            "velobank" => CreatePaymentIntentParamsPaymentMethodDataP24Bank::Velobank,
            "volkswagen_bank" => CreatePaymentIntentParamsPaymentMethodDataP24Bank::VolkswagenBank,
            other => CreatePaymentIntentParamsPaymentMethodDataP24Bank::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodDataP24Bank {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodDataUpiMandateOptionsAmountType {
    Fixed,
    Maximum,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodDataUpiMandateOptionsAmountType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodDataUpiMandateOptionsAmountType::Fixed => "fixed",
            CreatePaymentIntentParamsPaymentMethodDataUpiMandateOptionsAmountType::Maximum => "maximum",
            CreatePaymentIntentParamsPaymentMethodDataUpiMandateOptionsAmountType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodDataUpiMandateOptionsAmountType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsPaymentMethodDataUpiMandateOptionsAmountType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "fixed" => CreatePaymentIntentParamsPaymentMethodDataUpiMandateOptionsAmountType::Fixed,
            "maximum" => CreatePaymentIntentParamsPaymentMethodDataUpiMandateOptionsAmountType::Maximum,
            other => CreatePaymentIntentParamsPaymentMethodDataUpiMandateOptionsAmountType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodDataUpiMandateOptionsAmountType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsBancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsBancontactPreferredLanguage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsBancontactPreferredLanguage::De => "de",
            CreatePaymentIntentParamsPaymentMethodOptionsBancontactPreferredLanguage::En => "en",
            CreatePaymentIntentParamsPaymentMethodOptionsBancontactPreferredLanguage::Fr => "fr",
            CreatePaymentIntentParamsPaymentMethodOptionsBancontactPreferredLanguage::Nl => "nl",
            CreatePaymentIntentParamsPaymentMethodOptionsBancontactPreferredLanguage::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsBancontactPreferredLanguage {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreatePaymentIntentParamsPaymentMethodOptionsBancontactPreferredLanguage::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "de" => CreatePaymentIntentParamsPaymentMethodOptionsBancontactPreferredLanguage::De,
            "en" => CreatePaymentIntentParamsPaymentMethodOptionsBancontactPreferredLanguage::En,
            "fr" => CreatePaymentIntentParamsPaymentMethodOptionsBancontactPreferredLanguage::Fr,
            "nl" => CreatePaymentIntentParamsPaymentMethodOptionsBancontactPreferredLanguage::Nl,
            other => {
                CreatePaymentIntentParamsPaymentMethodOptionsBancontactPreferredLanguage::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsBancontactPreferredLanguage {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlanType {
    Bonus,
    FixedCount,
    Revolving,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlanType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlanType::Bonus => "bonus",
            CreatePaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlanType::FixedCount => "fixed_count",
            CreatePaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlanType::Revolving => "revolving",
            CreatePaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlanType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlanType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlanType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "bonus" => CreatePaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlanType::Bonus,
            "fixed_count" => CreatePaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlanType::FixedCount,
            "revolving" => CreatePaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlanType::Revolving,
            other => CreatePaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlanType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlanType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsCardPresentRoutingRequestedPriority {
    Domestic,
    International,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsCardPresentRoutingRequestedPriority {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsCardPresentRoutingRequestedPriority::Domestic => "domestic",
            CreatePaymentIntentParamsPaymentMethodOptionsCardPresentRoutingRequestedPriority::International => {
                "international"
            }
            CreatePaymentIntentParamsPaymentMethodOptionsCardPresentRoutingRequestedPriority::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsCardPresentRoutingRequestedPriority {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreatePaymentIntentParamsPaymentMethodOptionsCardPresentRoutingRequestedPriority::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "domestic" => CreatePaymentIntentParamsPaymentMethodOptionsCardPresentRoutingRequestedPriority::Domestic,
            "international" => {
                CreatePaymentIntentParamsPaymentMethodOptionsCardPresentRoutingRequestedPriority::International
            }
            other => CreatePaymentIntentParamsPaymentMethodOptionsCardPresentRoutingRequestedPriority::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsCardPresentRoutingRequestedPriority {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureExemptionIndicator {
    LowRisk,
    None,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureExemptionIndicator {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureExemptionIndicator::LowRisk => "low_risk",
            CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureExemptionIndicator::None => "none",
            CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureExemptionIndicator::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureExemptionIndicator {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureExemptionIndicator::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "low_risk" => CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureExemptionIndicator::LowRisk,
            "none" => CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureExemptionIndicator::None,
            other => CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureExemptionIndicator::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureExemptionIndicator {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale {
    CsCZ,
    DaDK,
    DeAT,
    DeCH,
    DeDE,
    ElGR,
    EnAT,
    EnAU,
    EnBE,
    EnCA,
    EnCH,
    EnCZ,
    EnDE,
    EnDK,
    EnES,
    EnFI,
    EnFR,
    EnGB,
    EnGR,
    EnIE,
    EnIT,
    EnNL,
    EnNO,
    EnNZ,
    EnPL,
    EnPT,
    EnRO,
    EnSE,
    EnUS,
    EsES,
    EsUS,
    FiFI,
    FrBE,
    FrCA,
    FrCH,
    FrFR,
    ItCH,
    ItIT,
    NbNO,
    NlBE,
    NlNL,
    PlPL,
    PtPT,
    RoRO,
    SvFI,
    SvSE,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::CsCZ => "cs-CZ",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::DaDK => "da-DK",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::DeAT => "de-AT",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::DeCH => "de-CH",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::DeDE => "de-DE",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::ElGR => "el-GR",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnAT => "en-AT",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnAU => "en-AU",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnBE => "en-BE",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnCA => "en-CA",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnCH => "en-CH",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnCZ => "en-CZ",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnDE => "en-DE",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnDK => "en-DK",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnES => "en-ES",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnFI => "en-FI",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnFR => "en-FR",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnGB => "en-GB",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnGR => "en-GR",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnIE => "en-IE",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnIT => "en-IT",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnNL => "en-NL",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnNO => "en-NO",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnNZ => "en-NZ",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnPL => "en-PL",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnPT => "en-PT",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnRO => "en-RO",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnSE => "en-SE",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnUS => "en-US",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EsES => "es-ES",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EsUS => "es-US",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::FiFI => "fi-FI",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::FrBE => "fr-BE",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::FrCA => "fr-CA",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::FrCH => "fr-CH",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::FrFR => "fr-FR",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::ItCH => "it-CH",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::ItIT => "it-IT",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::NbNO => "nb-NO",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::NlBE => "nl-BE",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::NlNL => "nl-NL",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::PlPL => "pl-PL",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::PtPT => "pt-PT",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::RoRO => "ro-RO",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::SvFI => "sv-FI",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::SvSE => "sv-SE",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "cs-CZ" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::CsCZ,
            "da-DK" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::DaDK,
            "de-AT" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::DeAT,
            "de-CH" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::DeCH,
            "de-DE" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::DeDE,
            "el-GR" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::ElGR,
            "en-AT" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnAT,
            "en-AU" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnAU,
            "en-BE" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnBE,
            "en-CA" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnCA,
            "en-CH" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnCH,
            "en-CZ" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnCZ,
            "en-DE" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnDE,
            "en-DK" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnDK,
            "en-ES" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnES,
            "en-FI" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnFI,
            "en-FR" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnFR,
            "en-GB" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnGB,
            "en-GR" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnGR,
            "en-IE" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnIE,
            "en-IT" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnIT,
            "en-NL" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnNL,
            "en-NO" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnNO,
            "en-NZ" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnNZ,
            "en-PL" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnPL,
            "en-PT" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnPT,
            "en-RO" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnRO,
            "en-SE" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnSE,
            "en-US" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnUS,
            "es-ES" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EsES,
            "es-US" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EsUS,
            "fi-FI" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::FiFI,
            "fr-BE" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::FrBE,
            "fr-CA" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::FrCA,
            "fr-CH" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::FrCH,
            "fr-FR" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::FrFR,
            "it-CH" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::ItCH,
            "it-IT" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::ItIT,
            "nb-NO" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::NbNO,
            "nl-BE" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::NlBE,
            "nl-NL" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::NlNL,
            "pl-PL" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::PlPL,
            "pt-PT" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::PtPT,
            "ro-RO" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::RoRO,
            "sv-FI" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::SvFI,
            "sv-SE" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::SvSE,
            other => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsKlarnaPreferredLocale {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsPixSetupFutureUsage {
    None,
    OffSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsPixSetupFutureUsage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsPixSetupFutureUsage::None => "none",
            CreatePaymentIntentParamsPaymentMethodOptionsPixSetupFutureUsage::OffSession => "off_session",
            CreatePaymentIntentParamsPaymentMethodOptionsPixSetupFutureUsage::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsPixSetupFutureUsage {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsPaymentMethodOptionsPixSetupFutureUsage::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "none" => CreatePaymentIntentParamsPaymentMethodOptionsPixSetupFutureUsage::None,
            "off_session" => CreatePaymentIntentParamsPaymentMethodOptionsPixSetupFutureUsage::OffSession,
            other => CreatePaymentIntentParamsPaymentMethodOptionsPixSetupFutureUsage::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsPixSetupFutureUsage {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountTransactionPurpose {
    Unnamed,
    Goods,
    Other,
    Services,
    Unspecified,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountTransactionPurpose {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountTransactionPurpose::Unnamed => "",
            CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountTransactionPurpose::Goods => "goods",
            CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountTransactionPurpose::Other => "other",
            CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountTransactionPurpose::Services => "services",
            CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountTransactionPurpose::Unspecified => "unspecified",
            CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountTransactionPurpose::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountTransactionPurpose {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountTransactionPurpose::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountTransactionPurpose::Unnamed,
            "goods" => CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountTransactionPurpose::Goods,
            "other" => CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountTransactionPurpose::Other,
            "services" => CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountTransactionPurpose::Services,
            "unspecified" => CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountTransactionPurpose::Unspecified,
            other => {
                CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountTransactionPurpose::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountTransactionPurpose {
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
pub enum FundingInstructionsBankTransferFinancialAddressSupportedNetworksItem {
    Ach,
    Bacs,
    Chaps,
    DomesticWireUs,
    Fps,
    Sepa,
    Spei,
    Swift,
    Zengin,
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
impl FundingInstructionsBankTransferFinancialAddressSupportedNetworksItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            FundingInstructionsBankTransferFinancialAddressSupportedNetworksItem::Ach => "ach",
            FundingInstructionsBankTransferFinancialAddressSupportedNetworksItem::Bacs => "bacs",
            FundingInstructionsBankTransferFinancialAddressSupportedNetworksItem::Chaps => "chaps",
            FundingInstructionsBankTransferFinancialAddressSupportedNetworksItem::DomesticWireUs => "domestic_wire_us",
            FundingInstructionsBankTransferFinancialAddressSupportedNetworksItem::Fps => "fps",
            FundingInstructionsBankTransferFinancialAddressSupportedNetworksItem::Sepa => "sepa",
            FundingInstructionsBankTransferFinancialAddressSupportedNetworksItem::Spei => "spei",
            FundingInstructionsBankTransferFinancialAddressSupportedNetworksItem::Swift => "swift",
            FundingInstructionsBankTransferFinancialAddressSupportedNetworksItem::Zengin => "zengin",
            FundingInstructionsBankTransferFinancialAddressSupportedNetworksItem::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for FundingInstructionsBankTransferFinancialAddressSupportedNetworksItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(FundingInstructionsBankTransferFinancialAddressSupportedNetworksItem::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ach" => FundingInstructionsBankTransferFinancialAddressSupportedNetworksItem::Ach,
            "bacs" => FundingInstructionsBankTransferFinancialAddressSupportedNetworksItem::Bacs,
            "chaps" => FundingInstructionsBankTransferFinancialAddressSupportedNetworksItem::Chaps,
            "domestic_wire_us" => FundingInstructionsBankTransferFinancialAddressSupportedNetworksItem::DomesticWireUs,
            "fps" => FundingInstructionsBankTransferFinancialAddressSupportedNetworksItem::Fps,
            "sepa" => FundingInstructionsBankTransferFinancialAddressSupportedNetworksItem::Sepa,
            "spei" => FundingInstructionsBankTransferFinancialAddressSupportedNetworksItem::Spei,
            "swift" => FundingInstructionsBankTransferFinancialAddressSupportedNetworksItem::Swift,
            "zengin" => FundingInstructionsBankTransferFinancialAddressSupportedNetworksItem::Zengin,
            other => FundingInstructionsBankTransferFinancialAddressSupportedNetworksItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ListPaymentIntentAmountDetailsLineItemsParams {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ListPaymentIntentAmountDetailsLineItemsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
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
pub struct PaymentFlowsAmountDetailsResourceTax {
    #[doc = "The total amount of tax on the transaction represented in the smallest currency unit..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub total_tax_amount: Option<i64>,
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
pub struct PaymentFlowsPaymentDetails {
    #[doc = "A unique value to identify the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_reference: Option<String>,
    #[doc = "A unique value assigned by the business to identify the transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub order_reference: Option<String>,
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
pub struct PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsResourceCompanyDetailsRegisteredAddress
{
    #[doc = "City, district, suburb, town, or village."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "Two-letter country code."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "Address line 1 (e.g., street, PO Box, or company name)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    #[doc = "Address line 2 (e.g., apartment, suite, unit, or building)."]
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
pub enum PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorizationStatus
{
    Disabled,
    Enabled,
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
impl PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorizationStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorizationStatus::Disabled => "disabled",
            PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorizationStatus::Enabled => "enabled",
            PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorizationStatus::Unknown(value) => value,
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
impl Default for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorizationStatus {
    fn default() -> Self {
        PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorizationStatus::Unknown(String::new())
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
impl<'json> json::Decode<'json> for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorizationStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorizationStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "disabled" => PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorizationStatus::Disabled,
            "enabled" => PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorizationStatus::Enabled,
            other => PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorizationStatus::Unknown(other.to_string()),
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
pub struct PaymentFlowsPrivatePaymentMethodsKlarnaPaymentIntentAmountDetailsLineItemPaymentMethodOptions {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub product_url: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub subscription_reference: Option<String>,
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
pub enum PaymentFlowsPrivatePaymentMethodsKonbiniDetailsResourceStoreChain {
    Familymart,
    Lawson,
    Ministop,
    Seicomart,
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
impl PaymentFlowsPrivatePaymentMethodsKonbiniDetailsResourceStoreChain {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentFlowsPrivatePaymentMethodsKonbiniDetailsResourceStoreChain::Familymart => "familymart",
            PaymentFlowsPrivatePaymentMethodsKonbiniDetailsResourceStoreChain::Lawson => "lawson",
            PaymentFlowsPrivatePaymentMethodsKonbiniDetailsResourceStoreChain::Ministop => "ministop",
            PaymentFlowsPrivatePaymentMethodsKonbiniDetailsResourceStoreChain::Seicomart => "seicomart",
            PaymentFlowsPrivatePaymentMethodsKonbiniDetailsResourceStoreChain::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PaymentFlowsPrivatePaymentMethodsKonbiniDetailsResourceStoreChain {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentFlowsPrivatePaymentMethodsKonbiniDetailsResourceStoreChain::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "familymart" => PaymentFlowsPrivatePaymentMethodsKonbiniDetailsResourceStoreChain::Familymart,
            "lawson" => PaymentFlowsPrivatePaymentMethodsKonbiniDetailsResourceStoreChain::Lawson,
            "ministop" => PaymentFlowsPrivatePaymentMethodsKonbiniDetailsResourceStoreChain::Ministop,
            "seicomart" => PaymentFlowsPrivatePaymentMethodsKonbiniDetailsResourceStoreChain::Seicomart,
            other => PaymentFlowsPrivatePaymentMethodsKonbiniDetailsResourceStoreChain::Unknown(other.to_string()),
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
pub enum PaymentFlowsPrivatePaymentMethodsPaycoPaymentMethodOptionsSetupFutureUsage {
    None,
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
impl PaymentFlowsPrivatePaymentMethodsPaycoPaymentMethodOptionsSetupFutureUsage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentFlowsPrivatePaymentMethodsPaycoPaymentMethodOptionsSetupFutureUsage::None => "none",
            PaymentFlowsPrivatePaymentMethodsPaycoPaymentMethodOptionsSetupFutureUsage::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PaymentFlowsPrivatePaymentMethodsPaycoPaymentMethodOptionsSetupFutureUsage {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                PaymentFlowsPrivatePaymentMethodsPaycoPaymentMethodOptionsSetupFutureUsage::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "none" => PaymentFlowsPrivatePaymentMethodsPaycoPaymentMethodOptionsSetupFutureUsage::None,
            other => {
                PaymentFlowsPrivatePaymentMethodsPaycoPaymentMethodOptionsSetupFutureUsage::Unknown(other.to_string())
            }
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
pub enum PaymentIntentAmountDetailsLineItemObject {
    PaymentIntentAmountDetailsLineItem,
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
impl PaymentIntentAmountDetailsLineItemObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentIntentAmountDetailsLineItemObject::PaymentIntentAmountDetailsLineItem => {
                "payment_intent_amount_details_line_item"
            }
            PaymentIntentAmountDetailsLineItemObject::Unknown(value) => value,
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
impl Default for PaymentIntentAmountDetailsLineItemObject {
    fn default() -> Self {
        PaymentIntentAmountDetailsLineItemObject::Unknown(String::new())
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
impl<'json> json::Decode<'json> for PaymentIntentAmountDetailsLineItemObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentIntentAmountDetailsLineItemObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "payment_intent_amount_details_line_item" => {
                PaymentIntentAmountDetailsLineItemObject::PaymentIntentAmountDetailsLineItem
            }
            other => PaymentIntentAmountDetailsLineItemObject::Unknown(other.to_string()),
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
pub enum PaymentIntentConfirmationMethod {
    Automatic,
    Manual,
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
impl PaymentIntentConfirmationMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentIntentConfirmationMethod::Automatic => "automatic",
            PaymentIntentConfirmationMethod::Manual => "manual",
            PaymentIntentConfirmationMethod::Unknown(value) => value,
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
impl Default for PaymentIntentConfirmationMethod {
    fn default() -> Self {
        PaymentIntentConfirmationMethod::Unknown(String::new())
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
impl<'json> json::Decode<'json> for PaymentIntentConfirmationMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentIntentConfirmationMethod::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "automatic" => PaymentIntentConfirmationMethod::Automatic,
            "manual" => PaymentIntentConfirmationMethod::Manual,
            other => PaymentIntentConfirmationMethod::Unknown(other.to_string()),
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
pub struct PaymentIntentNextActionBoleto {
    #[doc = "The timestamp after which the boleto expires."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[doc = "The URL to the hosted boleto voucher page, which allows customers to view the boleto voucher."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub hosted_voucher_url: Option<String>,
    #[doc = "The boleto number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[doc = "The URL to the downloadable boleto voucher PDF."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pdf: Option<String>,
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
pub struct PaymentIntentNextActionDisplayOxxoDetails {
    #[doc = "The timestamp after which the OXXO voucher expires."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<i64>,
    #[doc = "The URL for the hosted OXXO voucher page, which allows customers to view and print an OXXO voucher."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub hosted_voucher_url: Option<String>,
    #[doc = "OXXO reference number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
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
pub struct PaymentIntentNextActionKlarnaDisplayQrCode {
    #[doc = "The data being used to generate QR code"]
    #[json(default)]
    pub data: String,
    #[doc = "The timestamp at which the QR code expires."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[doc = "The image_url_png string used to render QR code"]
    #[json(default)]
    pub image_url_png: String,
    #[doc = "The image_url_svg string used to render QR code"]
    #[json(default)]
    pub image_url_svg: String,
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
pub struct PaymentIntentNextActionKonbiniLawson {
    #[doc = "The confirmation number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub confirmation_number: Option<String>,
    #[doc = "The payment code."]
    #[json(default)]
    pub payment_code: String,
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
pub struct PaymentIntentNextActionKonbiniMinistop {
    #[doc = "The confirmation number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub confirmation_number: Option<String>,
    #[doc = "The payment code."]
    #[json(default)]
    pub payment_code: String,
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
pub struct PaymentIntentNextActionUpiqrCode {
    #[doc = "The date (unix timestamp) when the QR code expires."]
    #[json(default)]
    pub expires_at: i64,
    #[doc = "The image_url_png string used to render QR code"]
    #[json(default)]
    pub image_url_png: String,
    #[doc = "The image_url_svg string used to render QR code"]
    #[json(default)]
    pub image_url_svg: String,
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
pub enum PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    Amounts,
    DescriptorCode,
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
impl PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType::Amounts => "amounts",
            PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType::DescriptorCode => "descriptor_code",
            PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "amounts" => PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType::Amounts,
            "descriptor_code" => PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType::DescriptorCode,
            other => PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType::Unknown(other.to_string()),
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
pub enum PaymentIntentObject {
    PaymentIntent,
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
impl PaymentIntentObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentIntentObject::PaymentIntent => "payment_intent",
            PaymentIntentObject::Unknown(value) => value,
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
impl Default for PaymentIntentObject {
    fn default() -> Self {
        PaymentIntentObject::Unknown(String::new())
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
impl<'json> json::Decode<'json> for PaymentIntentObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentIntentObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "payment_intent" => PaymentIntentObject::PaymentIntent,
            other => PaymentIntentObject::Unknown(other.to_string()),
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
pub enum PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPurpose {
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
impl PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPurpose {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::DependantSupport => "dependant_support",
            PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Government => "government",
            PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Loan => "loan",
            PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Mortgage => "mortgage",
            PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Other => "other",
            PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Pension => "pension",
            PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Personal => "personal",
            PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Retail => "retail",
            PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Salary => "salary",
            PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Tax => "tax",
            PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Utility => "utility",
            PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPurpose {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "dependant_support" => PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::DependantSupport,
            "government" => PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Government,
            "loan" => PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Loan,
            "mortgage" => PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Mortgage,
            "other" => PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Other,
            "pension" => PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Pension,
            "personal" => PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Personal,
            "retail" => PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Retail,
            "salary" => PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Salary,
            "tax" => PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Tax,
            "utility" => PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Utility,
            other => PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPurpose::Unknown(other.to_string()),
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
pub enum PaymentIntentStatus {
    Canceled,
    Processing,
    RequiresAction,
    RequiresCapture,
    RequiresConfirmation,
    RequiresPaymentMethod,
    Succeeded,
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
impl PaymentIntentStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentIntentStatus::Canceled => "canceled",
            PaymentIntentStatus::Processing => "processing",
            PaymentIntentStatus::RequiresAction => "requires_action",
            PaymentIntentStatus::RequiresCapture => "requires_capture",
            PaymentIntentStatus::RequiresConfirmation => "requires_confirmation",
            PaymentIntentStatus::RequiresPaymentMethod => "requires_payment_method",
            PaymentIntentStatus::Succeeded => "succeeded",
            PaymentIntentStatus::Unknown(value) => value,
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
impl Default for PaymentIntentStatus {
    fn default() -> Self {
        PaymentIntentStatus::Unknown(String::new())
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
impl<'json> json::Decode<'json> for PaymentIntentStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentIntentStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "canceled" => PaymentIntentStatus::Canceled,
            "processing" => PaymentIntentStatus::Processing,
            "requires_action" => PaymentIntentStatus::RequiresAction,
            "requires_capture" => PaymentIntentStatus::RequiresCapture,
            "requires_confirmation" => PaymentIntentStatus::RequiresConfirmation,
            "requires_payment_method" => PaymentIntentStatus::RequiresPaymentMethod,
            "succeeded" => PaymentIntentStatus::Succeeded,
            other => PaymentIntentStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct VerifyMicrodepositsParams {
    #[doc = "Two positive integers, in *cents*, equal to the values of the microdeposits sent to the bank account."]
    pub amounts: Option<Vec<i64>>,
    #[doc = "A six-character code starting with SM present in the microdeposit sent to the bank account."]
    pub descriptor_code: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for VerifyMicrodepositsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.amounts.as_ref() {
            form.field_array("amounts", values.as_slice());
        }
        form.field_opt("descriptor_code", self.descriptor_code.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}
