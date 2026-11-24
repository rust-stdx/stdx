#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsFlowDirectionsItem {
    Inbound,
    Outbound,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsFlowDirectionsItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsFlowDirectionsItem::Inbound => "inbound",
            CreateSetupIntentParamsFlowDirectionsItem::Outbound => "outbound",
            CreateSetupIntentParamsFlowDirectionsItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsFlowDirectionsItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSetupIntentParamsFlowDirectionsItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "inbound" => CreateSetupIntentParamsFlowDirectionsItem::Inbound,
            "outbound" => CreateSetupIntentParamsFlowDirectionsItem::Outbound,
            other => CreateSetupIntentParamsFlowDirectionsItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsFlowDirectionsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodDataAllowRedisplay {
    Always,
    Limited,
    Unspecified,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodDataAllowRedisplay {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodDataAllowRedisplay::Always => "always",
            CreateSetupIntentParamsPaymentMethodDataAllowRedisplay::Limited => "limited",
            CreateSetupIntentParamsPaymentMethodDataAllowRedisplay::Unspecified => "unspecified",
            CreateSetupIntentParamsPaymentMethodDataAllowRedisplay::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsPaymentMethodDataAllowRedisplay {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSetupIntentParamsPaymentMethodDataAllowRedisplay::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "always" => CreateSetupIntentParamsPaymentMethodDataAllowRedisplay::Always,
            "limited" => CreateSetupIntentParamsPaymentMethodDataAllowRedisplay::Limited,
            "unspecified" => CreateSetupIntentParamsPaymentMethodDataAllowRedisplay::Unspecified,
            other => CreateSetupIntentParamsPaymentMethodDataAllowRedisplay::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodDataAllowRedisplay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodDataBacsDebit {
    #[doc = "Account number of the bank account that the funds will be debited from."]
    pub account_number: Option<String>,
    #[doc = "Sort code of the bank account."]
    pub sort_code: Option<String>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodDataBacsDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account_number", self.account_number.as_ref());
        form.field_opt("sort_code", self.sort_code.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodDataFpxAccountHolderType {
    Company,
    Individual,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodDataFpxAccountHolderType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodDataFpxAccountHolderType::Company => "company",
            CreateSetupIntentParamsPaymentMethodDataFpxAccountHolderType::Individual => "individual",
            CreateSetupIntentParamsPaymentMethodDataFpxAccountHolderType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsPaymentMethodDataFpxAccountHolderType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSetupIntentParamsPaymentMethodDataFpxAccountHolderType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "company" => CreateSetupIntentParamsPaymentMethodDataFpxAccountHolderType::Company,
            "individual" => CreateSetupIntentParamsPaymentMethodDataFpxAccountHolderType::Individual,
            other => CreateSetupIntentParamsPaymentMethodDataFpxAccountHolderType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodDataFpxAccountHolderType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodDataNaverPayFunding {
    Card,
    Points,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodDataNaverPayFunding {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodDataNaverPayFunding::Card => "card",
            CreateSetupIntentParamsPaymentMethodDataNaverPayFunding::Points => "points",
            CreateSetupIntentParamsPaymentMethodDataNaverPayFunding::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsPaymentMethodDataNaverPayFunding {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSetupIntentParamsPaymentMethodDataNaverPayFunding::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "card" => CreateSetupIntentParamsPaymentMethodDataNaverPayFunding::Card,
            "points" => CreateSetupIntentParamsPaymentMethodDataNaverPayFunding::Points,
            other => CreateSetupIntentParamsPaymentMethodDataNaverPayFunding::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodDataNaverPayFunding {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodDataP24Bank {
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

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodDataP24Bank {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodDataP24Bank::AliorBank => "alior_bank",
            CreateSetupIntentParamsPaymentMethodDataP24Bank::BankMillennium => "bank_millennium",
            CreateSetupIntentParamsPaymentMethodDataP24Bank::BankNowyBfgSa => "bank_nowy_bfg_sa",
            CreateSetupIntentParamsPaymentMethodDataP24Bank::BankPekaoSa => "bank_pekao_sa",
            CreateSetupIntentParamsPaymentMethodDataP24Bank::BankiSpbdzielcze => "banki_spbdzielcze",
            CreateSetupIntentParamsPaymentMethodDataP24Bank::Blik => "blik",
            CreateSetupIntentParamsPaymentMethodDataP24Bank::BnpParibas => "bnp_paribas",
            CreateSetupIntentParamsPaymentMethodDataP24Bank::Boz => "boz",
            CreateSetupIntentParamsPaymentMethodDataP24Bank::CitiHandlowy => "citi_handlowy",
            CreateSetupIntentParamsPaymentMethodDataP24Bank::CreditAgricole => "credit_agricole",
            CreateSetupIntentParamsPaymentMethodDataP24Bank::Envelobank => "envelobank",
            CreateSetupIntentParamsPaymentMethodDataP24Bank::EtransferPocztowy24 => "etransfer_pocztowy24",
            CreateSetupIntentParamsPaymentMethodDataP24Bank::GetinBank => "getin_bank",
            CreateSetupIntentParamsPaymentMethodDataP24Bank::Ideabank => "ideabank",
            CreateSetupIntentParamsPaymentMethodDataP24Bank::Ing => "ing",
            CreateSetupIntentParamsPaymentMethodDataP24Bank::Inteligo => "inteligo",
            CreateSetupIntentParamsPaymentMethodDataP24Bank::MbankMtransfer => "mbank_mtransfer",
            CreateSetupIntentParamsPaymentMethodDataP24Bank::NestPrzelew => "nest_przelew",
            CreateSetupIntentParamsPaymentMethodDataP24Bank::NoblePay => "noble_pay",
            CreateSetupIntentParamsPaymentMethodDataP24Bank::PbacZIpko => "pbac_z_ipko",
            CreateSetupIntentParamsPaymentMethodDataP24Bank::PlusBank => "plus_bank",
            CreateSetupIntentParamsPaymentMethodDataP24Bank::SantanderPrzelew24 => "santander_przelew24",
            CreateSetupIntentParamsPaymentMethodDataP24Bank::TmobileUsbugiBankowe => "tmobile_usbugi_bankowe",
            CreateSetupIntentParamsPaymentMethodDataP24Bank::ToyotaBank => "toyota_bank",
            CreateSetupIntentParamsPaymentMethodDataP24Bank::Velobank => "velobank",
            CreateSetupIntentParamsPaymentMethodDataP24Bank::VolkswagenBank => "volkswagen_bank",
            CreateSetupIntentParamsPaymentMethodDataP24Bank::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsPaymentMethodDataP24Bank {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSetupIntentParamsPaymentMethodDataP24Bank::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "alior_bank" => CreateSetupIntentParamsPaymentMethodDataP24Bank::AliorBank,
            "bank_millennium" => CreateSetupIntentParamsPaymentMethodDataP24Bank::BankMillennium,
            "bank_nowy_bfg_sa" => CreateSetupIntentParamsPaymentMethodDataP24Bank::BankNowyBfgSa,
            "bank_pekao_sa" => CreateSetupIntentParamsPaymentMethodDataP24Bank::BankPekaoSa,
            "banki_spbdzielcze" => CreateSetupIntentParamsPaymentMethodDataP24Bank::BankiSpbdzielcze,
            "blik" => CreateSetupIntentParamsPaymentMethodDataP24Bank::Blik,
            "bnp_paribas" => CreateSetupIntentParamsPaymentMethodDataP24Bank::BnpParibas,
            "boz" => CreateSetupIntentParamsPaymentMethodDataP24Bank::Boz,
            "citi_handlowy" => CreateSetupIntentParamsPaymentMethodDataP24Bank::CitiHandlowy,
            "credit_agricole" => CreateSetupIntentParamsPaymentMethodDataP24Bank::CreditAgricole,
            "envelobank" => CreateSetupIntentParamsPaymentMethodDataP24Bank::Envelobank,
            "etransfer_pocztowy24" => CreateSetupIntentParamsPaymentMethodDataP24Bank::EtransferPocztowy24,
            "getin_bank" => CreateSetupIntentParamsPaymentMethodDataP24Bank::GetinBank,
            "ideabank" => CreateSetupIntentParamsPaymentMethodDataP24Bank::Ideabank,
            "ing" => CreateSetupIntentParamsPaymentMethodDataP24Bank::Ing,
            "inteligo" => CreateSetupIntentParamsPaymentMethodDataP24Bank::Inteligo,
            "mbank_mtransfer" => CreateSetupIntentParamsPaymentMethodDataP24Bank::MbankMtransfer,
            "nest_przelew" => CreateSetupIntentParamsPaymentMethodDataP24Bank::NestPrzelew,
            "noble_pay" => CreateSetupIntentParamsPaymentMethodDataP24Bank::NoblePay,
            "pbac_z_ipko" => CreateSetupIntentParamsPaymentMethodDataP24Bank::PbacZIpko,
            "plus_bank" => CreateSetupIntentParamsPaymentMethodDataP24Bank::PlusBank,
            "santander_przelew24" => CreateSetupIntentParamsPaymentMethodDataP24Bank::SantanderPrzelew24,
            "tmobile_usbugi_bankowe" => CreateSetupIntentParamsPaymentMethodDataP24Bank::TmobileUsbugiBankowe,
            "toyota_bank" => CreateSetupIntentParamsPaymentMethodDataP24Bank::ToyotaBank,
            "velobank" => CreateSetupIntentParamsPaymentMethodDataP24Bank::Velobank,
            "volkswagen_bank" => CreateSetupIntentParamsPaymentMethodDataP24Bank::VolkswagenBank,
            other => CreateSetupIntentParamsPaymentMethodDataP24Bank::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodDataP24Bank {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodDataSofortCountry {
    AT,
    BE,
    DE,
    ES,
    IT,
    NL,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodDataSofortCountry {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodDataSofortCountry::AT => "AT",
            CreateSetupIntentParamsPaymentMethodDataSofortCountry::BE => "BE",
            CreateSetupIntentParamsPaymentMethodDataSofortCountry::DE => "DE",
            CreateSetupIntentParamsPaymentMethodDataSofortCountry::ES => "ES",
            CreateSetupIntentParamsPaymentMethodDataSofortCountry::IT => "IT",
            CreateSetupIntentParamsPaymentMethodDataSofortCountry::NL => "NL",
            CreateSetupIntentParamsPaymentMethodDataSofortCountry::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsPaymentMethodDataSofortCountry {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSetupIntentParamsPaymentMethodDataSofortCountry::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "AT" => CreateSetupIntentParamsPaymentMethodDataSofortCountry::AT,
            "BE" => CreateSetupIntentParamsPaymentMethodDataSofortCountry::BE,
            "DE" => CreateSetupIntentParamsPaymentMethodDataSofortCountry::DE,
            "ES" => CreateSetupIntentParamsPaymentMethodDataSofortCountry::ES,
            "IT" => CreateSetupIntentParamsPaymentMethodDataSofortCountry::IT,
            "NL" => CreateSetupIntentParamsPaymentMethodDataSofortCountry::NL,
            other => CreateSetupIntentParamsPaymentMethodDataSofortCountry::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodDataSofortCountry {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Combined => "combined",
            CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Interval => "interval",
            CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Sporadic => "sporadic",
            CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "combined" => CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Combined,
            "interval" => CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Interval,
            "sporadic" => CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Sporadic,
            other => CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodOptionsBlikMandateOptions {
    #[doc = "Expiry date of the mandate."]
    pub expires_at: Option<i64>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsBlikMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("expires_at", self.expires_at.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo {
    N0,
    N1,
    N2,
    N3,
    N4,
    A,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::N0 => "0",
            CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::N1 => "1",
            CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::N2 => "2",
            CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::N3 => "3",
            CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::N4 => "4",
            CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::A => "A",
            CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json>
    for CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "0" => CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::N0,
            "1" => CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::N1,
            "2" => CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::N2,
            "3" => CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::N3,
            "4" => CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::N4,
            "A" => CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::A,
            other => CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode
    for CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale {
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

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::CsCZ => "cs-CZ",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::DaDK => "da-DK",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::DeAT => "de-AT",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::DeCH => "de-CH",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::DeDE => "de-DE",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::ElGR => "el-GR",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnAT => "en-AT",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnAU => "en-AU",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnBE => "en-BE",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnCA => "en-CA",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnCH => "en-CH",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnCZ => "en-CZ",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnDE => "en-DE",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnDK => "en-DK",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnES => "en-ES",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnFI => "en-FI",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnFR => "en-FR",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnGB => "en-GB",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnGR => "en-GR",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnIE => "en-IE",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnIT => "en-IT",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnNL => "en-NL",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnNO => "en-NO",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnNZ => "en-NZ",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnPL => "en-PL",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnPT => "en-PT",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnRO => "en-RO",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnSE => "en-SE",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnUS => "en-US",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EsES => "es-ES",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EsUS => "es-US",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::FiFI => "fi-FI",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::FrBE => "fr-BE",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::FrCA => "fr-CA",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::FrCH => "fr-CH",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::FrFR => "fr-FR",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::ItCH => "it-CH",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::ItIT => "it-IT",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::NbNO => "nb-NO",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::NlBE => "nl-BE",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::NlNL => "nl-NL",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::PlPL => "pl-PL",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::PtPT => "pt-PT",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::RoRO => "ro-RO",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::SvFI => "sv-FI",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::SvSE => "sv-SE",
            CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "cs-CZ" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::CsCZ,
            "da-DK" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::DaDK,
            "de-AT" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::DeAT,
            "de-CH" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::DeCH,
            "de-DE" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::DeDE,
            "el-GR" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::ElGR,
            "en-AT" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnAT,
            "en-AU" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnAU,
            "en-BE" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnBE,
            "en-CA" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnCA,
            "en-CH" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnCH,
            "en-CZ" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnCZ,
            "en-DE" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnDE,
            "en-DK" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnDK,
            "en-ES" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnES,
            "en-FI" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnFI,
            "en-FR" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnFR,
            "en-GB" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnGB,
            "en-GR" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnGR,
            "en-IE" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnIE,
            "en-IT" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnIT,
            "en-NL" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnNL,
            "en-NO" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnNO,
            "en-NZ" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnNZ,
            "en-PL" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnPL,
            "en-PT" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnPT,
            "en-RO" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnRO,
            "en-SE" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnSE,
            "en-US" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EnUS,
            "es-ES" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EsES,
            "es-US" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::EsUS,
            "fi-FI" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::FiFI,
            "fr-BE" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::FrBE,
            "fr-CA" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::FrCA,
            "fr-CH" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::FrCH,
            "fr-FR" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::FrFR,
            "it-CH" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::ItCH,
            "it-IT" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::ItIT,
            "nb-NO" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::NbNO,
            "nl-BE" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::NlBE,
            "nl-NL" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::NlNL,
            "pl-PL" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::PlPL,
            "pt-PT" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::PtPT,
            "ro-RO" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::RoRO,
            "sv-FI" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::SvFI,
            "sv-SE" => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::SvSE,
            other => CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct GetSetupIntentParams {
    pub client_secret: Option<String>,
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for GetSetupIntentParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("client_secret", self.client_secret.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
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
#[derive(Debug, Clone, PartialEq)]
pub enum SetupIntentAllowedPaymentMethodTypesItem {
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    Alma,
    AmazonPay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Billie,
    Bizum,
    Blik,
    BokuPromptpay,
    Boleto,
    CapchasePay,
    Card,
    CardPresent,
    Cashapp,
    CheckScan,
    ClickToPay,
    Crypto,
    CustomerBalance,
    DemoPay,
    Duitnow,
    DummyAuthPush,
    DummyPassthroughCard,
    Edenred,
    Eps,
    Fpx,
    Gcash,
    Getbalance,
    GiftCard,
    Giropay,
    Gopay,
    Grabpay,
    IdBankTransfer,
    Ideal,
    InteracPresent,
    KakaoPay,
    Klarna,
    Knet,
    Konbini,
    KrCard,
    KrMarket,
    Kriya,
    Link,
    MbWay,
    Mobilepay,
    Momo,
    Mondu,
    Multibanco,
    NaverPay,
    Netbanking,
    NgBank,
    NgBankTransfer,
    NgCard,
    NgMarket,
    NgUssd,
    NgWallet,
    NzBankAccount,
    Octopus,
    Oxxo,
    P24,
    PaperCheck,
    PayByBank,
    Payco,
    Paynow,
    Paypal,
    Paypay,
    Payto,
    Pix,
    Promptpay,
    Qris,
    Rechnung,
    RevolutPay,
    SamsungPay,
    Satispay,
    Scalapay,
    SepaDebit,
    Sequra,
    ShopPay,
    Shopeepay,
    Sofort,
    SouthKoreaMarket,
    StripeBalance,
    Sunbit,
    Swish,
    Tamara,
    TestPay,
    TouchNGo,
    Truemoney,
    Twint,
    Upi,
    UsBankAccount,
    UsCashVoucher,
    Vipps,
    WechatPay,
    Wero,
    Zip,
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
impl SetupIntentAllowedPaymentMethodTypesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SetupIntentAllowedPaymentMethodTypesItem::AcssDebit => "acss_debit",
            SetupIntentAllowedPaymentMethodTypesItem::Affirm => "affirm",
            SetupIntentAllowedPaymentMethodTypesItem::AfterpayClearpay => "afterpay_clearpay",
            SetupIntentAllowedPaymentMethodTypesItem::Alipay => "alipay",
            SetupIntentAllowedPaymentMethodTypesItem::Alma => "alma",
            SetupIntentAllowedPaymentMethodTypesItem::AmazonPay => "amazon_pay",
            SetupIntentAllowedPaymentMethodTypesItem::AuBecsDebit => "au_becs_debit",
            SetupIntentAllowedPaymentMethodTypesItem::BacsDebit => "bacs_debit",
            SetupIntentAllowedPaymentMethodTypesItem::Bancontact => "bancontact",
            SetupIntentAllowedPaymentMethodTypesItem::Billie => "billie",
            SetupIntentAllowedPaymentMethodTypesItem::Bizum => "bizum",
            SetupIntentAllowedPaymentMethodTypesItem::Blik => "blik",
            SetupIntentAllowedPaymentMethodTypesItem::BokuPromptpay => "boku_promptpay",
            SetupIntentAllowedPaymentMethodTypesItem::Boleto => "boleto",
            SetupIntentAllowedPaymentMethodTypesItem::CapchasePay => "capchase_pay",
            SetupIntentAllowedPaymentMethodTypesItem::Card => "card",
            SetupIntentAllowedPaymentMethodTypesItem::CardPresent => "card_present",
            SetupIntentAllowedPaymentMethodTypesItem::Cashapp => "cashapp",
            SetupIntentAllowedPaymentMethodTypesItem::CheckScan => "check_scan",
            SetupIntentAllowedPaymentMethodTypesItem::ClickToPay => "click_to_pay",
            SetupIntentAllowedPaymentMethodTypesItem::Crypto => "crypto",
            SetupIntentAllowedPaymentMethodTypesItem::CustomerBalance => "customer_balance",
            SetupIntentAllowedPaymentMethodTypesItem::DemoPay => "demo_pay",
            SetupIntentAllowedPaymentMethodTypesItem::Duitnow => "duitnow",
            SetupIntentAllowedPaymentMethodTypesItem::DummyAuthPush => "dummy_auth_push",
            SetupIntentAllowedPaymentMethodTypesItem::DummyPassthroughCard => "dummy_passthrough_card",
            SetupIntentAllowedPaymentMethodTypesItem::Edenred => "edenred",
            SetupIntentAllowedPaymentMethodTypesItem::Eps => "eps",
            SetupIntentAllowedPaymentMethodTypesItem::Fpx => "fpx",
            SetupIntentAllowedPaymentMethodTypesItem::Gcash => "gcash",
            SetupIntentAllowedPaymentMethodTypesItem::Getbalance => "getbalance",
            SetupIntentAllowedPaymentMethodTypesItem::GiftCard => "gift_card",
            SetupIntentAllowedPaymentMethodTypesItem::Giropay => "giropay",
            SetupIntentAllowedPaymentMethodTypesItem::Gopay => "gopay",
            SetupIntentAllowedPaymentMethodTypesItem::Grabpay => "grabpay",
            SetupIntentAllowedPaymentMethodTypesItem::IdBankTransfer => "id_bank_transfer",
            SetupIntentAllowedPaymentMethodTypesItem::Ideal => "ideal",
            SetupIntentAllowedPaymentMethodTypesItem::InteracPresent => "interac_present",
            SetupIntentAllowedPaymentMethodTypesItem::KakaoPay => "kakao_pay",
            SetupIntentAllowedPaymentMethodTypesItem::Klarna => "klarna",
            SetupIntentAllowedPaymentMethodTypesItem::Knet => "knet",
            SetupIntentAllowedPaymentMethodTypesItem::Konbini => "konbini",
            SetupIntentAllowedPaymentMethodTypesItem::KrCard => "kr_card",
            SetupIntentAllowedPaymentMethodTypesItem::KrMarket => "kr_market",
            SetupIntentAllowedPaymentMethodTypesItem::Kriya => "kriya",
            SetupIntentAllowedPaymentMethodTypesItem::Link => "link",
            SetupIntentAllowedPaymentMethodTypesItem::MbWay => "mb_way",
            SetupIntentAllowedPaymentMethodTypesItem::Mobilepay => "mobilepay",
            SetupIntentAllowedPaymentMethodTypesItem::Momo => "momo",
            SetupIntentAllowedPaymentMethodTypesItem::Mondu => "mondu",
            SetupIntentAllowedPaymentMethodTypesItem::Multibanco => "multibanco",
            SetupIntentAllowedPaymentMethodTypesItem::NaverPay => "naver_pay",
            SetupIntentAllowedPaymentMethodTypesItem::Netbanking => "netbanking",
            SetupIntentAllowedPaymentMethodTypesItem::NgBank => "ng_bank",
            SetupIntentAllowedPaymentMethodTypesItem::NgBankTransfer => "ng_bank_transfer",
            SetupIntentAllowedPaymentMethodTypesItem::NgCard => "ng_card",
            SetupIntentAllowedPaymentMethodTypesItem::NgMarket => "ng_market",
            SetupIntentAllowedPaymentMethodTypesItem::NgUssd => "ng_ussd",
            SetupIntentAllowedPaymentMethodTypesItem::NgWallet => "ng_wallet",
            SetupIntentAllowedPaymentMethodTypesItem::NzBankAccount => "nz_bank_account",
            SetupIntentAllowedPaymentMethodTypesItem::Octopus => "octopus",
            SetupIntentAllowedPaymentMethodTypesItem::Oxxo => "oxxo",
            SetupIntentAllowedPaymentMethodTypesItem::P24 => "p24",
            SetupIntentAllowedPaymentMethodTypesItem::PaperCheck => "paper_check",
            SetupIntentAllowedPaymentMethodTypesItem::PayByBank => "pay_by_bank",
            SetupIntentAllowedPaymentMethodTypesItem::Payco => "payco",
            SetupIntentAllowedPaymentMethodTypesItem::Paynow => "paynow",
            SetupIntentAllowedPaymentMethodTypesItem::Paypal => "paypal",
            SetupIntentAllowedPaymentMethodTypesItem::Paypay => "paypay",
            SetupIntentAllowedPaymentMethodTypesItem::Payto => "payto",
            SetupIntentAllowedPaymentMethodTypesItem::Pix => "pix",
            SetupIntentAllowedPaymentMethodTypesItem::Promptpay => "promptpay",
            SetupIntentAllowedPaymentMethodTypesItem::Qris => "qris",
            SetupIntentAllowedPaymentMethodTypesItem::Rechnung => "rechnung",
            SetupIntentAllowedPaymentMethodTypesItem::RevolutPay => "revolut_pay",
            SetupIntentAllowedPaymentMethodTypesItem::SamsungPay => "samsung_pay",
            SetupIntentAllowedPaymentMethodTypesItem::Satispay => "satispay",
            SetupIntentAllowedPaymentMethodTypesItem::Scalapay => "scalapay",
            SetupIntentAllowedPaymentMethodTypesItem::SepaDebit => "sepa_debit",
            SetupIntentAllowedPaymentMethodTypesItem::Sequra => "sequra",
            SetupIntentAllowedPaymentMethodTypesItem::ShopPay => "shop_pay",
            SetupIntentAllowedPaymentMethodTypesItem::Shopeepay => "shopeepay",
            SetupIntentAllowedPaymentMethodTypesItem::Sofort => "sofort",
            SetupIntentAllowedPaymentMethodTypesItem::SouthKoreaMarket => "south_korea_market",
            SetupIntentAllowedPaymentMethodTypesItem::StripeBalance => "stripe_balance",
            SetupIntentAllowedPaymentMethodTypesItem::Sunbit => "sunbit",
            SetupIntentAllowedPaymentMethodTypesItem::Swish => "swish",
            SetupIntentAllowedPaymentMethodTypesItem::Tamara => "tamara",
            SetupIntentAllowedPaymentMethodTypesItem::TestPay => "test_pay",
            SetupIntentAllowedPaymentMethodTypesItem::TouchNGo => "touch_n_go",
            SetupIntentAllowedPaymentMethodTypesItem::Truemoney => "truemoney",
            SetupIntentAllowedPaymentMethodTypesItem::Twint => "twint",
            SetupIntentAllowedPaymentMethodTypesItem::Upi => "upi",
            SetupIntentAllowedPaymentMethodTypesItem::UsBankAccount => "us_bank_account",
            SetupIntentAllowedPaymentMethodTypesItem::UsCashVoucher => "us_cash_voucher",
            SetupIntentAllowedPaymentMethodTypesItem::Vipps => "vipps",
            SetupIntentAllowedPaymentMethodTypesItem::WechatPay => "wechat_pay",
            SetupIntentAllowedPaymentMethodTypesItem::Wero => "wero",
            SetupIntentAllowedPaymentMethodTypesItem::Zip => "zip",
            SetupIntentAllowedPaymentMethodTypesItem::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for SetupIntentAllowedPaymentMethodTypesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SetupIntentAllowedPaymentMethodTypesItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "acss_debit" => SetupIntentAllowedPaymentMethodTypesItem::AcssDebit,
            "affirm" => SetupIntentAllowedPaymentMethodTypesItem::Affirm,
            "afterpay_clearpay" => SetupIntentAllowedPaymentMethodTypesItem::AfterpayClearpay,
            "alipay" => SetupIntentAllowedPaymentMethodTypesItem::Alipay,
            "alma" => SetupIntentAllowedPaymentMethodTypesItem::Alma,
            "amazon_pay" => SetupIntentAllowedPaymentMethodTypesItem::AmazonPay,
            "au_becs_debit" => SetupIntentAllowedPaymentMethodTypesItem::AuBecsDebit,
            "bacs_debit" => SetupIntentAllowedPaymentMethodTypesItem::BacsDebit,
            "bancontact" => SetupIntentAllowedPaymentMethodTypesItem::Bancontact,
            "billie" => SetupIntentAllowedPaymentMethodTypesItem::Billie,
            "bizum" => SetupIntentAllowedPaymentMethodTypesItem::Bizum,
            "blik" => SetupIntentAllowedPaymentMethodTypesItem::Blik,
            "boku_promptpay" => SetupIntentAllowedPaymentMethodTypesItem::BokuPromptpay,
            "boleto" => SetupIntentAllowedPaymentMethodTypesItem::Boleto,
            "capchase_pay" => SetupIntentAllowedPaymentMethodTypesItem::CapchasePay,
            "card" => SetupIntentAllowedPaymentMethodTypesItem::Card,
            "card_present" => SetupIntentAllowedPaymentMethodTypesItem::CardPresent,
            "cashapp" => SetupIntentAllowedPaymentMethodTypesItem::Cashapp,
            "check_scan" => SetupIntentAllowedPaymentMethodTypesItem::CheckScan,
            "click_to_pay" => SetupIntentAllowedPaymentMethodTypesItem::ClickToPay,
            "crypto" => SetupIntentAllowedPaymentMethodTypesItem::Crypto,
            "customer_balance" => SetupIntentAllowedPaymentMethodTypesItem::CustomerBalance,
            "demo_pay" => SetupIntentAllowedPaymentMethodTypesItem::DemoPay,
            "duitnow" => SetupIntentAllowedPaymentMethodTypesItem::Duitnow,
            "dummy_auth_push" => SetupIntentAllowedPaymentMethodTypesItem::DummyAuthPush,
            "dummy_passthrough_card" => SetupIntentAllowedPaymentMethodTypesItem::DummyPassthroughCard,
            "edenred" => SetupIntentAllowedPaymentMethodTypesItem::Edenred,
            "eps" => SetupIntentAllowedPaymentMethodTypesItem::Eps,
            "fpx" => SetupIntentAllowedPaymentMethodTypesItem::Fpx,
            "gcash" => SetupIntentAllowedPaymentMethodTypesItem::Gcash,
            "getbalance" => SetupIntentAllowedPaymentMethodTypesItem::Getbalance,
            "gift_card" => SetupIntentAllowedPaymentMethodTypesItem::GiftCard,
            "giropay" => SetupIntentAllowedPaymentMethodTypesItem::Giropay,
            "gopay" => SetupIntentAllowedPaymentMethodTypesItem::Gopay,
            "grabpay" => SetupIntentAllowedPaymentMethodTypesItem::Grabpay,
            "id_bank_transfer" => SetupIntentAllowedPaymentMethodTypesItem::IdBankTransfer,
            "ideal" => SetupIntentAllowedPaymentMethodTypesItem::Ideal,
            "interac_present" => SetupIntentAllowedPaymentMethodTypesItem::InteracPresent,
            "kakao_pay" => SetupIntentAllowedPaymentMethodTypesItem::KakaoPay,
            "klarna" => SetupIntentAllowedPaymentMethodTypesItem::Klarna,
            "knet" => SetupIntentAllowedPaymentMethodTypesItem::Knet,
            "konbini" => SetupIntentAllowedPaymentMethodTypesItem::Konbini,
            "kr_card" => SetupIntentAllowedPaymentMethodTypesItem::KrCard,
            "kr_market" => SetupIntentAllowedPaymentMethodTypesItem::KrMarket,
            "kriya" => SetupIntentAllowedPaymentMethodTypesItem::Kriya,
            "link" => SetupIntentAllowedPaymentMethodTypesItem::Link,
            "mb_way" => SetupIntentAllowedPaymentMethodTypesItem::MbWay,
            "mobilepay" => SetupIntentAllowedPaymentMethodTypesItem::Mobilepay,
            "momo" => SetupIntentAllowedPaymentMethodTypesItem::Momo,
            "mondu" => SetupIntentAllowedPaymentMethodTypesItem::Mondu,
            "multibanco" => SetupIntentAllowedPaymentMethodTypesItem::Multibanco,
            "naver_pay" => SetupIntentAllowedPaymentMethodTypesItem::NaverPay,
            "netbanking" => SetupIntentAllowedPaymentMethodTypesItem::Netbanking,
            "ng_bank" => SetupIntentAllowedPaymentMethodTypesItem::NgBank,
            "ng_bank_transfer" => SetupIntentAllowedPaymentMethodTypesItem::NgBankTransfer,
            "ng_card" => SetupIntentAllowedPaymentMethodTypesItem::NgCard,
            "ng_market" => SetupIntentAllowedPaymentMethodTypesItem::NgMarket,
            "ng_ussd" => SetupIntentAllowedPaymentMethodTypesItem::NgUssd,
            "ng_wallet" => SetupIntentAllowedPaymentMethodTypesItem::NgWallet,
            "nz_bank_account" => SetupIntentAllowedPaymentMethodTypesItem::NzBankAccount,
            "octopus" => SetupIntentAllowedPaymentMethodTypesItem::Octopus,
            "oxxo" => SetupIntentAllowedPaymentMethodTypesItem::Oxxo,
            "p24" => SetupIntentAllowedPaymentMethodTypesItem::P24,
            "paper_check" => SetupIntentAllowedPaymentMethodTypesItem::PaperCheck,
            "pay_by_bank" => SetupIntentAllowedPaymentMethodTypesItem::PayByBank,
            "payco" => SetupIntentAllowedPaymentMethodTypesItem::Payco,
            "paynow" => SetupIntentAllowedPaymentMethodTypesItem::Paynow,
            "paypal" => SetupIntentAllowedPaymentMethodTypesItem::Paypal,
            "paypay" => SetupIntentAllowedPaymentMethodTypesItem::Paypay,
            "payto" => SetupIntentAllowedPaymentMethodTypesItem::Payto,
            "pix" => SetupIntentAllowedPaymentMethodTypesItem::Pix,
            "promptpay" => SetupIntentAllowedPaymentMethodTypesItem::Promptpay,
            "qris" => SetupIntentAllowedPaymentMethodTypesItem::Qris,
            "rechnung" => SetupIntentAllowedPaymentMethodTypesItem::Rechnung,
            "revolut_pay" => SetupIntentAllowedPaymentMethodTypesItem::RevolutPay,
            "samsung_pay" => SetupIntentAllowedPaymentMethodTypesItem::SamsungPay,
            "satispay" => SetupIntentAllowedPaymentMethodTypesItem::Satispay,
            "scalapay" => SetupIntentAllowedPaymentMethodTypesItem::Scalapay,
            "sepa_debit" => SetupIntentAllowedPaymentMethodTypesItem::SepaDebit,
            "sequra" => SetupIntentAllowedPaymentMethodTypesItem::Sequra,
            "shop_pay" => SetupIntentAllowedPaymentMethodTypesItem::ShopPay,
            "shopeepay" => SetupIntentAllowedPaymentMethodTypesItem::Shopeepay,
            "sofort" => SetupIntentAllowedPaymentMethodTypesItem::Sofort,
            "south_korea_market" => SetupIntentAllowedPaymentMethodTypesItem::SouthKoreaMarket,
            "stripe_balance" => SetupIntentAllowedPaymentMethodTypesItem::StripeBalance,
            "sunbit" => SetupIntentAllowedPaymentMethodTypesItem::Sunbit,
            "swish" => SetupIntentAllowedPaymentMethodTypesItem::Swish,
            "tamara" => SetupIntentAllowedPaymentMethodTypesItem::Tamara,
            "test_pay" => SetupIntentAllowedPaymentMethodTypesItem::TestPay,
            "touch_n_go" => SetupIntentAllowedPaymentMethodTypesItem::TouchNGo,
            "truemoney" => SetupIntentAllowedPaymentMethodTypesItem::Truemoney,
            "twint" => SetupIntentAllowedPaymentMethodTypesItem::Twint,
            "upi" => SetupIntentAllowedPaymentMethodTypesItem::Upi,
            "us_bank_account" => SetupIntentAllowedPaymentMethodTypesItem::UsBankAccount,
            "us_cash_voucher" => SetupIntentAllowedPaymentMethodTypesItem::UsCashVoucher,
            "vipps" => SetupIntentAllowedPaymentMethodTypesItem::Vipps,
            "wechat_pay" => SetupIntentAllowedPaymentMethodTypesItem::WechatPay,
            "wero" => SetupIntentAllowedPaymentMethodTypesItem::Wero,
            "zip" => SetupIntentAllowedPaymentMethodTypesItem::Zip,
            other => SetupIntentAllowedPaymentMethodTypesItem::Unknown(other.to_string()),
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
pub enum SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType {
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
impl SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType::Amounts => "amounts",
            SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType::DescriptorCode => "descriptor_code",
            SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "amounts" => SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType::Amounts,
            "descriptor_code" => SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType::DescriptorCode,
            other => SetupIntentNextActionVerifyWithMicrodepositsMicrodepositType::Unknown(other.to_string()),
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
pub struct SetupIntentPaymentMethodOptionsAmazonPay {}

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
pub enum SetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
    Challenge,
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
impl SetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SetupIntentPaymentMethodOptionsCardRequestThreeDSecure::Any => "any",
            SetupIntentPaymentMethodOptionsCardRequestThreeDSecure::Automatic => "automatic",
            SetupIntentPaymentMethodOptionsCardRequestThreeDSecure::Challenge => "challenge",
            SetupIntentPaymentMethodOptionsCardRequestThreeDSecure::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for SetupIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SetupIntentPaymentMethodOptionsCardRequestThreeDSecure::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "any" => SetupIntentPaymentMethodOptionsCardRequestThreeDSecure::Any,
            "automatic" => SetupIntentPaymentMethodOptionsCardRequestThreeDSecure::Automatic,
            "challenge" => SetupIntentPaymentMethodOptionsCardRequestThreeDSecure::Challenge,
            other => SetupIntentPaymentMethodOptionsCardRequestThreeDSecure::Unknown(other.to_string()),
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
pub struct SetupIntentPaymentMethodOptionsLink {
    #[doc = "Deprecated This is a legacy parameter that no longer has any function."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub persistent_token: Option<String>,
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
pub struct SetupIntentPaymentMethodOptionsMandateOptionsSepaDebit {
    #[doc = "Prefix used to generate the Mandate reference."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reference_prefix: Option<String>,
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct VerifyMicrodepositsParams {
    #[doc = "Two positive integers, in *cents*, equal to the values of the microdeposits sent to the bank account."]
    pub amounts: Option<Vec<i64>>,
    #[doc = "A six-character code starting with SM present in the microdeposit sent to the bank account."]
    pub descriptor_code: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "setup_intents")]
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
