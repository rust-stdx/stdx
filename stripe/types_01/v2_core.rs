#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsConfigurationMerchantSepaDebitPayments {
    #[doc = "Creditor ID for SEPA Direct Debit payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub creditor_id: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCoreAccountParamsDefaultsLocalesItem {
    ArSA,
    Bg,
    BgBG,
    Cs,
    CsCZ,
    Da,
    DaDK,
    De,
    DeDE,
    El,
    ElGR,
    En,
    EnAU,
    EnCA,
    EnGB,
    EnIE,
    EnIN,
    EnNZ,
    EnSG,
    EnUS,
    Es,
    Es419,
    EsES,
    Et,
    EtEE,
    Fi,
    Fil,
    FilPH,
    FiFI,
    Fr,
    FrCA,
    FrFR,
    HeIL,
    Hr,
    HrHR,
    Hu,
    HuHU,
    Id,
    IdID,
    It,
    ItIT,
    Ja,
    JaJP,
    Ko,
    KoKR,
    Lt,
    LtLT,
    Lv,
    LvLV,
    Ms,
    MsMY,
    Mt,
    MtMT,
    Nb,
    NbNO,
    Nl,
    NlNL,
    Pl,
    PlPL,
    Pt,
    PtBR,
    PtPT,
    Ro,
    RoRO,
    Ru,
    RuRU,
    Sk,
    SkSK,
    Sl,
    SlSI,
    Sv,
    SvSE,
    Th,
    ThTH,
    Tr,
    TrTR,
    Vi,
    ViVN,
    Zh,
    ZhHans,
    ZhHantHK,
    ZhHantTW,
    ZhHK,
    ZhTW,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl CreateCoreAccountParamsDefaultsLocalesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCoreAccountParamsDefaultsLocalesItem::ArSA => "ar-SA",
            CreateCoreAccountParamsDefaultsLocalesItem::Bg => "bg",
            CreateCoreAccountParamsDefaultsLocalesItem::BgBG => "bg-BG",
            CreateCoreAccountParamsDefaultsLocalesItem::Cs => "cs",
            CreateCoreAccountParamsDefaultsLocalesItem::CsCZ => "cs-CZ",
            CreateCoreAccountParamsDefaultsLocalesItem::Da => "da",
            CreateCoreAccountParamsDefaultsLocalesItem::DaDK => "da-DK",
            CreateCoreAccountParamsDefaultsLocalesItem::De => "de",
            CreateCoreAccountParamsDefaultsLocalesItem::DeDE => "de-DE",
            CreateCoreAccountParamsDefaultsLocalesItem::El => "el",
            CreateCoreAccountParamsDefaultsLocalesItem::ElGR => "el-GR",
            CreateCoreAccountParamsDefaultsLocalesItem::En => "en",
            CreateCoreAccountParamsDefaultsLocalesItem::EnAU => "en-AU",
            CreateCoreAccountParamsDefaultsLocalesItem::EnCA => "en-CA",
            CreateCoreAccountParamsDefaultsLocalesItem::EnGB => "en-GB",
            CreateCoreAccountParamsDefaultsLocalesItem::EnIE => "en-IE",
            CreateCoreAccountParamsDefaultsLocalesItem::EnIN => "en-IN",
            CreateCoreAccountParamsDefaultsLocalesItem::EnNZ => "en-NZ",
            CreateCoreAccountParamsDefaultsLocalesItem::EnSG => "en-SG",
            CreateCoreAccountParamsDefaultsLocalesItem::EnUS => "en-US",
            CreateCoreAccountParamsDefaultsLocalesItem::Es => "es",
            CreateCoreAccountParamsDefaultsLocalesItem::Es419 => "es-419",
            CreateCoreAccountParamsDefaultsLocalesItem::EsES => "es-ES",
            CreateCoreAccountParamsDefaultsLocalesItem::Et => "et",
            CreateCoreAccountParamsDefaultsLocalesItem::EtEE => "et-EE",
            CreateCoreAccountParamsDefaultsLocalesItem::Fi => "fi",
            CreateCoreAccountParamsDefaultsLocalesItem::Fil => "fil",
            CreateCoreAccountParamsDefaultsLocalesItem::FilPH => "fil-PH",
            CreateCoreAccountParamsDefaultsLocalesItem::FiFI => "fi-FI",
            CreateCoreAccountParamsDefaultsLocalesItem::Fr => "fr",
            CreateCoreAccountParamsDefaultsLocalesItem::FrCA => "fr-CA",
            CreateCoreAccountParamsDefaultsLocalesItem::FrFR => "fr-FR",
            CreateCoreAccountParamsDefaultsLocalesItem::HeIL => "he-IL",
            CreateCoreAccountParamsDefaultsLocalesItem::Hr => "hr",
            CreateCoreAccountParamsDefaultsLocalesItem::HrHR => "hr-HR",
            CreateCoreAccountParamsDefaultsLocalesItem::Hu => "hu",
            CreateCoreAccountParamsDefaultsLocalesItem::HuHU => "hu-HU",
            CreateCoreAccountParamsDefaultsLocalesItem::Id => "id",
            CreateCoreAccountParamsDefaultsLocalesItem::IdID => "id-ID",
            CreateCoreAccountParamsDefaultsLocalesItem::It => "it",
            CreateCoreAccountParamsDefaultsLocalesItem::ItIT => "it-IT",
            CreateCoreAccountParamsDefaultsLocalesItem::Ja => "ja",
            CreateCoreAccountParamsDefaultsLocalesItem::JaJP => "ja-JP",
            CreateCoreAccountParamsDefaultsLocalesItem::Ko => "ko",
            CreateCoreAccountParamsDefaultsLocalesItem::KoKR => "ko-KR",
            CreateCoreAccountParamsDefaultsLocalesItem::Lt => "lt",
            CreateCoreAccountParamsDefaultsLocalesItem::LtLT => "lt-LT",
            CreateCoreAccountParamsDefaultsLocalesItem::Lv => "lv",
            CreateCoreAccountParamsDefaultsLocalesItem::LvLV => "lv-LV",
            CreateCoreAccountParamsDefaultsLocalesItem::Ms => "ms",
            CreateCoreAccountParamsDefaultsLocalesItem::MsMY => "ms-MY",
            CreateCoreAccountParamsDefaultsLocalesItem::Mt => "mt",
            CreateCoreAccountParamsDefaultsLocalesItem::MtMT => "mt-MT",
            CreateCoreAccountParamsDefaultsLocalesItem::Nb => "nb",
            CreateCoreAccountParamsDefaultsLocalesItem::NbNO => "nb-NO",
            CreateCoreAccountParamsDefaultsLocalesItem::Nl => "nl",
            CreateCoreAccountParamsDefaultsLocalesItem::NlNL => "nl-NL",
            CreateCoreAccountParamsDefaultsLocalesItem::Pl => "pl",
            CreateCoreAccountParamsDefaultsLocalesItem::PlPL => "pl-PL",
            CreateCoreAccountParamsDefaultsLocalesItem::Pt => "pt",
            CreateCoreAccountParamsDefaultsLocalesItem::PtBR => "pt-BR",
            CreateCoreAccountParamsDefaultsLocalesItem::PtPT => "pt-PT",
            CreateCoreAccountParamsDefaultsLocalesItem::Ro => "ro",
            CreateCoreAccountParamsDefaultsLocalesItem::RoRO => "ro-RO",
            CreateCoreAccountParamsDefaultsLocalesItem::Ru => "ru",
            CreateCoreAccountParamsDefaultsLocalesItem::RuRU => "ru-RU",
            CreateCoreAccountParamsDefaultsLocalesItem::Sk => "sk",
            CreateCoreAccountParamsDefaultsLocalesItem::SkSK => "sk-SK",
            CreateCoreAccountParamsDefaultsLocalesItem::Sl => "sl",
            CreateCoreAccountParamsDefaultsLocalesItem::SlSI => "sl-SI",
            CreateCoreAccountParamsDefaultsLocalesItem::Sv => "sv",
            CreateCoreAccountParamsDefaultsLocalesItem::SvSE => "sv-SE",
            CreateCoreAccountParamsDefaultsLocalesItem::Th => "th",
            CreateCoreAccountParamsDefaultsLocalesItem::ThTH => "th-TH",
            CreateCoreAccountParamsDefaultsLocalesItem::Tr => "tr",
            CreateCoreAccountParamsDefaultsLocalesItem::TrTR => "tr-TR",
            CreateCoreAccountParamsDefaultsLocalesItem::Vi => "vi",
            CreateCoreAccountParamsDefaultsLocalesItem::ViVN => "vi-VN",
            CreateCoreAccountParamsDefaultsLocalesItem::Zh => "zh",
            CreateCoreAccountParamsDefaultsLocalesItem::ZhHans => "zh-Hans",
            CreateCoreAccountParamsDefaultsLocalesItem::ZhHantHK => "zh-Hant-HK",
            CreateCoreAccountParamsDefaultsLocalesItem::ZhHantTW => "zh-Hant-TW",
            CreateCoreAccountParamsDefaultsLocalesItem::ZhHK => "zh-HK",
            CreateCoreAccountParamsDefaultsLocalesItem::ZhTW => "zh-TW",
            CreateCoreAccountParamsDefaultsLocalesItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl json::Encode for CreateCoreAccountParamsDefaultsLocalesItem {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for CreateCoreAccountParamsDefaultsLocalesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCoreAccountParamsDefaultsLocalesItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ar-SA" => CreateCoreAccountParamsDefaultsLocalesItem::ArSA,
            "bg" => CreateCoreAccountParamsDefaultsLocalesItem::Bg,
            "bg-BG" => CreateCoreAccountParamsDefaultsLocalesItem::BgBG,
            "cs" => CreateCoreAccountParamsDefaultsLocalesItem::Cs,
            "cs-CZ" => CreateCoreAccountParamsDefaultsLocalesItem::CsCZ,
            "da" => CreateCoreAccountParamsDefaultsLocalesItem::Da,
            "da-DK" => CreateCoreAccountParamsDefaultsLocalesItem::DaDK,
            "de" => CreateCoreAccountParamsDefaultsLocalesItem::De,
            "de-DE" => CreateCoreAccountParamsDefaultsLocalesItem::DeDE,
            "el" => CreateCoreAccountParamsDefaultsLocalesItem::El,
            "el-GR" => CreateCoreAccountParamsDefaultsLocalesItem::ElGR,
            "en" => CreateCoreAccountParamsDefaultsLocalesItem::En,
            "en-AU" => CreateCoreAccountParamsDefaultsLocalesItem::EnAU,
            "en-CA" => CreateCoreAccountParamsDefaultsLocalesItem::EnCA,
            "en-GB" => CreateCoreAccountParamsDefaultsLocalesItem::EnGB,
            "en-IE" => CreateCoreAccountParamsDefaultsLocalesItem::EnIE,
            "en-IN" => CreateCoreAccountParamsDefaultsLocalesItem::EnIN,
            "en-NZ" => CreateCoreAccountParamsDefaultsLocalesItem::EnNZ,
            "en-SG" => CreateCoreAccountParamsDefaultsLocalesItem::EnSG,
            "en-US" => CreateCoreAccountParamsDefaultsLocalesItem::EnUS,
            "es" => CreateCoreAccountParamsDefaultsLocalesItem::Es,
            "es-419" => CreateCoreAccountParamsDefaultsLocalesItem::Es419,
            "es-ES" => CreateCoreAccountParamsDefaultsLocalesItem::EsES,
            "et" => CreateCoreAccountParamsDefaultsLocalesItem::Et,
            "et-EE" => CreateCoreAccountParamsDefaultsLocalesItem::EtEE,
            "fi" => CreateCoreAccountParamsDefaultsLocalesItem::Fi,
            "fil" => CreateCoreAccountParamsDefaultsLocalesItem::Fil,
            "fil-PH" => CreateCoreAccountParamsDefaultsLocalesItem::FilPH,
            "fi-FI" => CreateCoreAccountParamsDefaultsLocalesItem::FiFI,
            "fr" => CreateCoreAccountParamsDefaultsLocalesItem::Fr,
            "fr-CA" => CreateCoreAccountParamsDefaultsLocalesItem::FrCA,
            "fr-FR" => CreateCoreAccountParamsDefaultsLocalesItem::FrFR,
            "he-IL" => CreateCoreAccountParamsDefaultsLocalesItem::HeIL,
            "hr" => CreateCoreAccountParamsDefaultsLocalesItem::Hr,
            "hr-HR" => CreateCoreAccountParamsDefaultsLocalesItem::HrHR,
            "hu" => CreateCoreAccountParamsDefaultsLocalesItem::Hu,
            "hu-HU" => CreateCoreAccountParamsDefaultsLocalesItem::HuHU,
            "id" => CreateCoreAccountParamsDefaultsLocalesItem::Id,
            "id-ID" => CreateCoreAccountParamsDefaultsLocalesItem::IdID,
            "it" => CreateCoreAccountParamsDefaultsLocalesItem::It,
            "it-IT" => CreateCoreAccountParamsDefaultsLocalesItem::ItIT,
            "ja" => CreateCoreAccountParamsDefaultsLocalesItem::Ja,
            "ja-JP" => CreateCoreAccountParamsDefaultsLocalesItem::JaJP,
            "ko" => CreateCoreAccountParamsDefaultsLocalesItem::Ko,
            "ko-KR" => CreateCoreAccountParamsDefaultsLocalesItem::KoKR,
            "lt" => CreateCoreAccountParamsDefaultsLocalesItem::Lt,
            "lt-LT" => CreateCoreAccountParamsDefaultsLocalesItem::LtLT,
            "lv" => CreateCoreAccountParamsDefaultsLocalesItem::Lv,
            "lv-LV" => CreateCoreAccountParamsDefaultsLocalesItem::LvLV,
            "ms" => CreateCoreAccountParamsDefaultsLocalesItem::Ms,
            "ms-MY" => CreateCoreAccountParamsDefaultsLocalesItem::MsMY,
            "mt" => CreateCoreAccountParamsDefaultsLocalesItem::Mt,
            "mt-MT" => CreateCoreAccountParamsDefaultsLocalesItem::MtMT,
            "nb" => CreateCoreAccountParamsDefaultsLocalesItem::Nb,
            "nb-NO" => CreateCoreAccountParamsDefaultsLocalesItem::NbNO,
            "nl" => CreateCoreAccountParamsDefaultsLocalesItem::Nl,
            "nl-NL" => CreateCoreAccountParamsDefaultsLocalesItem::NlNL,
            "pl" => CreateCoreAccountParamsDefaultsLocalesItem::Pl,
            "pl-PL" => CreateCoreAccountParamsDefaultsLocalesItem::PlPL,
            "pt" => CreateCoreAccountParamsDefaultsLocalesItem::Pt,
            "pt-BR" => CreateCoreAccountParamsDefaultsLocalesItem::PtBR,
            "pt-PT" => CreateCoreAccountParamsDefaultsLocalesItem::PtPT,
            "ro" => CreateCoreAccountParamsDefaultsLocalesItem::Ro,
            "ro-RO" => CreateCoreAccountParamsDefaultsLocalesItem::RoRO,
            "ru" => CreateCoreAccountParamsDefaultsLocalesItem::Ru,
            "ru-RU" => CreateCoreAccountParamsDefaultsLocalesItem::RuRU,
            "sk" => CreateCoreAccountParamsDefaultsLocalesItem::Sk,
            "sk-SK" => CreateCoreAccountParamsDefaultsLocalesItem::SkSK,
            "sl" => CreateCoreAccountParamsDefaultsLocalesItem::Sl,
            "sl-SI" => CreateCoreAccountParamsDefaultsLocalesItem::SlSI,
            "sv" => CreateCoreAccountParamsDefaultsLocalesItem::Sv,
            "sv-SE" => CreateCoreAccountParamsDefaultsLocalesItem::SvSE,
            "th" => CreateCoreAccountParamsDefaultsLocalesItem::Th,
            "th-TH" => CreateCoreAccountParamsDefaultsLocalesItem::ThTH,
            "tr" => CreateCoreAccountParamsDefaultsLocalesItem::Tr,
            "tr-TR" => CreateCoreAccountParamsDefaultsLocalesItem::TrTR,
            "vi" => CreateCoreAccountParamsDefaultsLocalesItem::Vi,
            "vi-VN" => CreateCoreAccountParamsDefaultsLocalesItem::ViVN,
            "zh" => CreateCoreAccountParamsDefaultsLocalesItem::Zh,
            "zh-Hans" => CreateCoreAccountParamsDefaultsLocalesItem::ZhHans,
            "zh-Hant-HK" => CreateCoreAccountParamsDefaultsLocalesItem::ZhHantHK,
            "zh-Hant-TW" => CreateCoreAccountParamsDefaultsLocalesItem::ZhHantTW,
            "zh-HK" => CreateCoreAccountParamsDefaultsLocalesItem::ZhHK,
            "zh-TW" => CreateCoreAccountParamsDefaultsLocalesItem::ZhTW,
            other => CreateCoreAccountParamsDefaultsLocalesItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsDefaultsProfile {
    #[doc = "The business's publicly-available website."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub business_url: Option<String>,
    #[doc = "The name which is used by the business."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub doing_business_as: Option<String>,
    #[doc = "Internal-only description of the product sold or service provided by the business."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityAttestationsTermsOfServiceAccount {
    #[doc = "The time when the Account's representative accepted the terms of service."]
    pub date: String,
    #[doc = "The IP address from which the Account's representative accepted the terms of service."]
    pub ip: String,
    #[doc = "The user agent of the browser from which the Account's representative accepted the terms of service."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityBusinessDetailsDocumentsPrimaryVerificationFrontBack {
    #[doc = "A [file upload](https://docs.stripe.com/api/persons/update#create_file) token representing the back of the verificati..."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub back: Option<String>,
    #[doc = "A [file upload](https://docs.stripe.com/api/persons/update#create_file) token representing the front of the verificat..."]
    pub front: String,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCoreAccountParamsIncludeItem {
    ConfigurationCustomer,
    ConfigurationMerchant,
    ConfigurationRecipient,
    Defaults,
    FutureRequirements,
    Identity,
    Requirements,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl CreateCoreAccountParamsIncludeItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCoreAccountParamsIncludeItem::ConfigurationCustomer => "configuration.customer",
            CreateCoreAccountParamsIncludeItem::ConfigurationMerchant => "configuration.merchant",
            CreateCoreAccountParamsIncludeItem::ConfigurationRecipient => "configuration.recipient",
            CreateCoreAccountParamsIncludeItem::Defaults => "defaults",
            CreateCoreAccountParamsIncludeItem::FutureRequirements => "future_requirements",
            CreateCoreAccountParamsIncludeItem::Identity => "identity",
            CreateCoreAccountParamsIncludeItem::Requirements => "requirements",
            CreateCoreAccountParamsIncludeItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl json::Encode for CreateCoreAccountParamsIncludeItem {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for CreateCoreAccountParamsIncludeItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCoreAccountParamsIncludeItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "configuration.customer" => CreateCoreAccountParamsIncludeItem::ConfigurationCustomer,
            "configuration.merchant" => CreateCoreAccountParamsIncludeItem::ConfigurationMerchant,
            "configuration.recipient" => CreateCoreAccountParamsIncludeItem::ConfigurationRecipient,
            "defaults" => CreateCoreAccountParamsIncludeItem::Defaults,
            "future_requirements" => CreateCoreAccountParamsIncludeItem::FutureRequirements,
            "identity" => CreateCoreAccountParamsIncludeItem::Identity,
            "requirements" => CreateCoreAccountParamsIncludeItem::Requirements,
            other => CreateCoreAccountParamsIncludeItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityAttestationsTermsOfServiceAccount {
    #[doc = "The boolean value indicating if the terms of service have been accepted."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub shown_and_accepted: Option<bool>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType {
    AeCrn,
    AeVat,
    AoNif,
    ArCuit,
    AtFn,
    AtStn,
    AtVat,
    AuAbn,
    AuAcn,
    AuIn,
    AzTin,
    BdEtin,
    BeCbe,
    BeVat,
    BgUic,
    BgVat,
    BrCnpj,
    CaCn,
    CaCrarr,
    CaGstHst,
    CaNeq,
    CaRid,
    ChChid,
    ChUid,
    CrCpj,
    CrNite,
    CyHe,
    CyTic,
    CyVat,
    CzIco,
    CzVat,
    DeHrn,
    DeStn,
    DeVat,
    DkCvr,
    DkVat,
    DoRcn,
    EeRk,
    EeVat,
    EsCif,
    EsVat,
    FiVat,
    FiYt,
    FrRna,
    FrSiren,
    FrVat,
    GbCrn,
    GbVat,
    GiCrn,
    GrAfm,
    GrGemi,
    GrVat,
    GtNit,
    HkBr,
    HkCr,
    HrMbs,
    HrOib,
    HrVat,
    HuCjs,
    HuTin,
    HuVat,
    IeCrn,
    IeTrn,
    IeVat,
    ItRea,
    ItVat,
    JpCn,
    KzBin,
    LiUid,
    LtCcrn,
    LtVat,
    LuNif,
    LuRcs,
    LuVat,
    LvUrn,
    LvVat,
    MtCrn,
    MtTin,
    MtVat,
    MxRfc,
    MyBrn,
    MyCoid,
    MyItn,
    MySst,
    MzNuit,
    NlKvk,
    NlRsin,
    NlVat,
    NoOrgnr,
    NzBn,
    NzIrd,
    PeRuc,
    PkNtn,
    PlNip,
    PlRegon,
    PlVat,
    PtVat,
    RoCui,
    RoOrc,
    RoVat,
    SaCrn,
    SaTin,
    SeOrgnr,
    SeVat,
    SgUen,
    SiMsp,
    SiTin,
    SiVat,
    SkDic,
    SkIco,
    SkVat,
    ThCrn,
    ThPrn,
    ThTin,
    UsEin,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::AeCrn => "ae_crn",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::AeVat => "ae_vat",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::AoNif => "ao_nif",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::ArCuit => "ar_cuit",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::AtFn => "at_fn",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::AtStn => "at_stn",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::AtVat => "at_vat",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::AuAbn => "au_abn",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::AuAcn => "au_acn",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::AuIn => "au_in",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::AzTin => "az_tin",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::BdEtin => "bd_etin",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::BeCbe => "be_cbe",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::BeVat => "be_vat",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::BgUic => "bg_uic",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::BgVat => "bg_vat",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::BrCnpj => "br_cnpj",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::CaCn => "ca_cn",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::CaCrarr => "ca_crarr",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::CaGstHst => "ca_gst_hst",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::CaNeq => "ca_neq",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::CaRid => "ca_rid",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::ChChid => "ch_chid",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::ChUid => "ch_uid",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::CrCpj => "cr_cpj",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::CrNite => "cr_nite",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::CyHe => "cy_he",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::CyTic => "cy_tic",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::CyVat => "cy_vat",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::CzIco => "cz_ico",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::CzVat => "cz_vat",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::DeHrn => "de_hrn",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::DeStn => "de_stn",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::DeVat => "de_vat",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::DkCvr => "dk_cvr",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::DkVat => "dk_vat",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::DoRcn => "do_rcn",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::EeRk => "ee_rk",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::EeVat => "ee_vat",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::EsCif => "es_cif",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::EsVat => "es_vat",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::FiVat => "fi_vat",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::FiYt => "fi_yt",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::FrRna => "fr_rna",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::FrSiren => "fr_siren",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::FrVat => "fr_vat",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::GbCrn => "gb_crn",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::GbVat => "gb_vat",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::GiCrn => "gi_crn",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::GrAfm => "gr_afm",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::GrGemi => "gr_gemi",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::GrVat => "gr_vat",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::GtNit => "gt_nit",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::HkBr => "hk_br",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::HkCr => "hk_cr",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::HrMbs => "hr_mbs",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::HrOib => "hr_oib",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::HrVat => "hr_vat",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::HuCjs => "hu_cjs",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::HuTin => "hu_tin",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::HuVat => "hu_vat",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::IeCrn => "ie_crn",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::IeTrn => "ie_trn",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::IeVat => "ie_vat",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::ItRea => "it_rea",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::ItVat => "it_vat",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::JpCn => "jp_cn",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::KzBin => "kz_bin",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::LiUid => "li_uid",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::LtCcrn => "lt_ccrn",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::LtVat => "lt_vat",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::LuNif => "lu_nif",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::LuRcs => "lu_rcs",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::LuVat => "lu_vat",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::LvUrn => "lv_urn",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::LvVat => "lv_vat",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::MtCrn => "mt_crn",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::MtTin => "mt_tin",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::MtVat => "mt_vat",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::MxRfc => "mx_rfc",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::MyBrn => "my_brn",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::MyCoid => "my_coid",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::MyItn => "my_itn",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::MySst => "my_sst",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::MzNuit => "mz_nuit",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::NlKvk => "nl_kvk",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::NlRsin => "nl_rsin",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::NlVat => "nl_vat",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::NoOrgnr => "no_orgnr",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::NzBn => "nz_bn",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::NzIrd => "nz_ird",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::PeRuc => "pe_ruc",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::PkNtn => "pk_ntn",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::PlNip => "pl_nip",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::PlRegon => "pl_regon",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::PlVat => "pl_vat",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::PtVat => "pt_vat",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::RoCui => "ro_cui",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::RoOrc => "ro_orc",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::RoVat => "ro_vat",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::SaCrn => "sa_crn",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::SaTin => "sa_tin",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::SeOrgnr => "se_orgnr",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::SeVat => "se_vat",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::SgUen => "sg_uen",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::SiMsp => "si_msp",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::SiTin => "si_tin",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::SiVat => "si_vat",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::SkDic => "sk_dic",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::SkIco => "sk_ico",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::SkVat => "sk_vat",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::ThCrn => "th_crn",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::ThPrn => "th_prn",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::ThTin => "th_tin",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::UsEin => "us_ein",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl json::Encode for CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ae_crn" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::AeCrn,
            "ae_vat" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::AeVat,
            "ao_nif" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::AoNif,
            "ar_cuit" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::ArCuit,
            "at_fn" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::AtFn,
            "at_stn" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::AtStn,
            "at_vat" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::AtVat,
            "au_abn" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::AuAbn,
            "au_acn" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::AuAcn,
            "au_in" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::AuIn,
            "az_tin" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::AzTin,
            "bd_etin" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::BdEtin,
            "be_cbe" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::BeCbe,
            "be_vat" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::BeVat,
            "bg_uic" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::BgUic,
            "bg_vat" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::BgVat,
            "br_cnpj" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::BrCnpj,
            "ca_cn" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::CaCn,
            "ca_crarr" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::CaCrarr,
            "ca_gst_hst" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::CaGstHst,
            "ca_neq" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::CaNeq,
            "ca_rid" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::CaRid,
            "ch_chid" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::ChChid,
            "ch_uid" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::ChUid,
            "cr_cpj" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::CrCpj,
            "cr_nite" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::CrNite,
            "cy_he" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::CyHe,
            "cy_tic" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::CyTic,
            "cy_vat" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::CyVat,
            "cz_ico" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::CzIco,
            "cz_vat" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::CzVat,
            "de_hrn" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::DeHrn,
            "de_stn" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::DeStn,
            "de_vat" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::DeVat,
            "dk_cvr" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::DkCvr,
            "dk_vat" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::DkVat,
            "do_rcn" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::DoRcn,
            "ee_rk" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::EeRk,
            "ee_vat" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::EeVat,
            "es_cif" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::EsCif,
            "es_vat" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::EsVat,
            "fi_vat" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::FiVat,
            "fi_yt" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::FiYt,
            "fr_rna" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::FrRna,
            "fr_siren" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::FrSiren,
            "fr_vat" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::FrVat,
            "gb_crn" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::GbCrn,
            "gb_vat" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::GbVat,
            "gi_crn" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::GiCrn,
            "gr_afm" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::GrAfm,
            "gr_gemi" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::GrGemi,
            "gr_vat" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::GrVat,
            "gt_nit" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::GtNit,
            "hk_br" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::HkBr,
            "hk_cr" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::HkCr,
            "hr_mbs" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::HrMbs,
            "hr_oib" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::HrOib,
            "hr_vat" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::HrVat,
            "hu_cjs" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::HuCjs,
            "hu_tin" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::HuTin,
            "hu_vat" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::HuVat,
            "ie_crn" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::IeCrn,
            "ie_trn" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::IeTrn,
            "ie_vat" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::IeVat,
            "it_rea" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::ItRea,
            "it_vat" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::ItVat,
            "jp_cn" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::JpCn,
            "kz_bin" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::KzBin,
            "li_uid" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::LiUid,
            "lt_ccrn" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::LtCcrn,
            "lt_vat" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::LtVat,
            "lu_nif" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::LuNif,
            "lu_rcs" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::LuRcs,
            "lu_vat" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::LuVat,
            "lv_urn" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::LvUrn,
            "lv_vat" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::LvVat,
            "mt_crn" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::MtCrn,
            "mt_tin" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::MtTin,
            "mt_vat" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::MtVat,
            "mx_rfc" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::MxRfc,
            "my_brn" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::MyBrn,
            "my_coid" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::MyCoid,
            "my_itn" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::MyItn,
            "my_sst" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::MySst,
            "mz_nuit" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::MzNuit,
            "nl_kvk" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::NlKvk,
            "nl_rsin" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::NlRsin,
            "nl_vat" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::NlVat,
            "no_orgnr" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::NoOrgnr,
            "nz_bn" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::NzBn,
            "nz_ird" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::NzIrd,
            "pe_ruc" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::PeRuc,
            "pk_ntn" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::PkNtn,
            "pl_nip" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::PlNip,
            "pl_regon" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::PlRegon,
            "pl_vat" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::PlVat,
            "pt_vat" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::PtVat,
            "ro_cui" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::RoCui,
            "ro_orc" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::RoOrc,
            "ro_vat" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::RoVat,
            "sa_crn" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::SaCrn,
            "sa_tin" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::SaTin,
            "se_orgnr" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::SeOrgnr,
            "se_vat" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::SeVat,
            "sg_uen" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::SgUen,
            "si_msp" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::SiMsp,
            "si_tin" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::SiTin,
            "si_vat" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::SiVat,
            "sk_dic" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::SkDic,
            "sk_ico" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::SkIco,
            "sk_vat" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::SkVat,
            "th_crn" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::ThCrn,
            "th_prn" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::ThPrn,
            "th_tin" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::ThTin,
            "us_ein" => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::UsEin,
            other => CreateCoreAccountTokenParamsIdentityBusinessDetailsIdNumbersItemType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityBusinessDetailsRegistrationDate {
    #[doc = "The day of registration, between 1 and 31."]
    pub day: i64,
    #[doc = "The month of registration, between 1 and 12."]
    pub month: i64,
    #[doc = "The four-digit year of registration."]
    pub year: i64,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCoreAccountTokenParamsIdentityEntityType {
    Company,
    GovernmentEntity,
    Individual,
    NonProfit,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl CreateCoreAccountTokenParamsIdentityEntityType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCoreAccountTokenParamsIdentityEntityType::Company => "company",
            CreateCoreAccountTokenParamsIdentityEntityType::GovernmentEntity => "government_entity",
            CreateCoreAccountTokenParamsIdentityEntityType::Individual => "individual",
            CreateCoreAccountTokenParamsIdentityEntityType::NonProfit => "non_profit",
            CreateCoreAccountTokenParamsIdentityEntityType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl json::Encode for CreateCoreAccountTokenParamsIdentityEntityType {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for CreateCoreAccountTokenParamsIdentityEntityType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCoreAccountTokenParamsIdentityEntityType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "company" => CreateCoreAccountTokenParamsIdentityEntityType::Company,
            "government_entity" => CreateCoreAccountTokenParamsIdentityEntityType::GovernmentEntity,
            "individual" => CreateCoreAccountTokenParamsIdentityEntityType::Individual,
            "non_profit" => CreateCoreAccountTokenParamsIdentityEntityType::NonProfit,
            other => CreateCoreAccountTokenParamsIdentityEntityType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCoreEventDestinationParamsEventPayload {
    Snapshot,
    Thin,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl CreateCoreEventDestinationParamsEventPayload {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCoreEventDestinationParamsEventPayload::Snapshot => "snapshot",
            CreateCoreEventDestinationParamsEventPayload::Thin => "thin",
            CreateCoreEventDestinationParamsEventPayload::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl json::Encode for CreateCoreEventDestinationParamsEventPayload {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for CreateCoreEventDestinationParamsEventPayload {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCoreEventDestinationParamsEventPayload::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "snapshot" => CreateCoreEventDestinationParamsEventPayload::Snapshot,
            "thin" => CreateCoreEventDestinationParamsEventPayload::Thin,
            other => CreateCoreEventDestinationParamsEventPayload::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCoreEventDestinationParamsType {
    AmazonEventbridge,
    AzureEventGrid,
    WebhookEndpoint,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl CreateCoreEventDestinationParamsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCoreEventDestinationParamsType::AmazonEventbridge => "amazon_eventbridge",
            CreateCoreEventDestinationParamsType::AzureEventGrid => "azure_event_grid",
            CreateCoreEventDestinationParamsType::WebhookEndpoint => "webhook_endpoint",
            CreateCoreEventDestinationParamsType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl json::Encode for CreateCoreEventDestinationParamsType {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for CreateCoreEventDestinationParamsType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCoreEventDestinationParamsType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "amazon_eventbridge" => CreateCoreEventDestinationParamsType::AmazonEventbridge,
            "azure_event_grid" => CreateCoreEventDestinationParamsType::AzureEventGrid,
            "webhook_endpoint" => CreateCoreEventDestinationParamsType::WebhookEndpoint,
            other => CreateCoreEventDestinationParamsType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, Default)]
pub struct ListCoreEventsParamsCreated {
    #[doc = "Filter for events created after the specified timestamp."]
    pub gt: Option<String>,
    #[doc = "Filter for events created at or after the specified timestamp."]
    pub gte: Option<String>,
    #[doc = "Filter for events created before the specified timestamp."]
    pub lt: Option<String>,
    #[doc = "Filter for events created at or before the specified timestamp."]
    pub lte: Option<String>,
}

#[cfg(feature = "v2_core")]
impl crate::params::FormEncode for ListCoreEventsParamsCreated {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("gt", self.gt.as_ref());
        form.field_opt("gte", self.gte.as_ref());
        form.field_opt("lt", self.lt.as_ref());
        form.field_opt("lte", self.lte.as_ref());
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum UpdateCoreAccountParamsConfigurationCustomerAutomaticIndirectTaxValidateLocation {
    Auto,
    Deferred,
    Immediately,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl UpdateCoreAccountParamsConfigurationCustomerAutomaticIndirectTaxValidateLocation {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            UpdateCoreAccountParamsConfigurationCustomerAutomaticIndirectTaxValidateLocation::Auto => "auto",
            UpdateCoreAccountParamsConfigurationCustomerAutomaticIndirectTaxValidateLocation::Deferred => "deferred",
            UpdateCoreAccountParamsConfigurationCustomerAutomaticIndirectTaxValidateLocation::Immediately => {
                "immediately"
            }
            UpdateCoreAccountParamsConfigurationCustomerAutomaticIndirectTaxValidateLocation::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl json::Encode for UpdateCoreAccountParamsConfigurationCustomerAutomaticIndirectTaxValidateLocation {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for UpdateCoreAccountParamsConfigurationCustomerAutomaticIndirectTaxValidateLocation {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                UpdateCoreAccountParamsConfigurationCustomerAutomaticIndirectTaxValidateLocation::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "auto" => UpdateCoreAccountParamsConfigurationCustomerAutomaticIndirectTaxValidateLocation::Auto,
            "deferred" => UpdateCoreAccountParamsConfigurationCustomerAutomaticIndirectTaxValidateLocation::Deferred,
            "immediately" => {
                UpdateCoreAccountParamsConfigurationCustomerAutomaticIndirectTaxValidateLocation::Immediately
            }
            other => UpdateCoreAccountParamsConfigurationCustomerAutomaticIndirectTaxValidateLocation::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountConfigurationCustomerAutomaticIndirectTaxExempt {
    Exempt,
    None,
    Reverse,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountConfigurationCustomerAutomaticIndirectTaxExempt {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountConfigurationCustomerAutomaticIndirectTaxExempt::Exempt => "exempt",
            V2CoreAccountConfigurationCustomerAutomaticIndirectTaxExempt::None => "none",
            V2CoreAccountConfigurationCustomerAutomaticIndirectTaxExempt::Reverse => "reverse",
            V2CoreAccountConfigurationCustomerAutomaticIndirectTaxExempt::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreAccountConfigurationCustomerAutomaticIndirectTaxExempt {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountConfigurationCustomerAutomaticIndirectTaxExempt::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "exempt" => V2CoreAccountConfigurationCustomerAutomaticIndirectTaxExempt::Exempt,
            "none" => V2CoreAccountConfigurationCustomerAutomaticIndirectTaxExempt::None,
            "reverse" => V2CoreAccountConfigurationCustomerAutomaticIndirectTaxExempt::Reverse,
            other => V2CoreAccountConfigurationCustomerAutomaticIndirectTaxExempt::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationCustomerAutomaticIndirectTaxLocation {
    #[doc = "The identified tax country of the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "The identified tax state, county, province, or region of the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountConfigurationCustomerBillingInvoiceRenderingAmountTaxDisplay {
    ExcludeTax,
    IncludeInclusiveTax,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountConfigurationCustomerBillingInvoiceRenderingAmountTaxDisplay {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountConfigurationCustomerBillingInvoiceRenderingAmountTaxDisplay::ExcludeTax => "exclude_tax",
            V2CoreAccountConfigurationCustomerBillingInvoiceRenderingAmountTaxDisplay::IncludeInclusiveTax => {
                "include_inclusive_tax"
            }
            V2CoreAccountConfigurationCustomerBillingInvoiceRenderingAmountTaxDisplay::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreAccountConfigurationCustomerBillingInvoiceRenderingAmountTaxDisplay {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                V2CoreAccountConfigurationCustomerBillingInvoiceRenderingAmountTaxDisplay::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "exclude_tax" => V2CoreAccountConfigurationCustomerBillingInvoiceRenderingAmountTaxDisplay::ExcludeTax,
            "include_inclusive_tax" => {
                V2CoreAccountConfigurationCustomerBillingInvoiceRenderingAmountTaxDisplay::IncludeInclusiveTax
            }
            other => {
                V2CoreAccountConfigurationCustomerBillingInvoiceRenderingAmountTaxDisplay::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationMerchantCardPaymentsDeclineOn {
    #[doc = "Whether Stripe automatically declines charges with an incorrect ZIP or postal code."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub avs_failure: Option<bool>,
    #[doc = "Whether Stripe automatically declines charges with an incorrect CVC."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cvc_failure: Option<bool>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationMerchantSupportAddress {
    #[doc = "City, district, suburb, town, or village."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
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
    #[doc = "Town or district."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountDefaultsLocalesItem {
    ArSA,
    Bg,
    BgBG,
    Cs,
    CsCZ,
    Da,
    DaDK,
    De,
    DeDE,
    El,
    ElGR,
    En,
    EnAU,
    EnCA,
    EnGB,
    EnIE,
    EnIN,
    EnNZ,
    EnSG,
    EnUS,
    Es,
    Es419,
    EsES,
    Et,
    EtEE,
    Fi,
    Fil,
    FilPH,
    FiFI,
    Fr,
    FrCA,
    FrFR,
    HeIL,
    Hr,
    HrHR,
    Hu,
    HuHU,
    Id,
    IdID,
    It,
    ItIT,
    Ja,
    JaJP,
    Ko,
    KoKR,
    Lt,
    LtLT,
    Lv,
    LvLV,
    Ms,
    MsMY,
    Mt,
    MtMT,
    Nb,
    NbNO,
    Nl,
    NlNL,
    Pl,
    PlPL,
    Pt,
    PtBR,
    PtPT,
    Ro,
    RoRO,
    Ru,
    RuRU,
    Sk,
    SkSK,
    Sl,
    SlSI,
    Sv,
    SvSE,
    Th,
    ThTH,
    Tr,
    TrTR,
    Vi,
    ViVN,
    Zh,
    ZhHans,
    ZhHantHK,
    ZhHantTW,
    ZhHK,
    ZhTW,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountDefaultsLocalesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountDefaultsLocalesItem::ArSA => "ar-SA",
            V2CoreAccountDefaultsLocalesItem::Bg => "bg",
            V2CoreAccountDefaultsLocalesItem::BgBG => "bg-BG",
            V2CoreAccountDefaultsLocalesItem::Cs => "cs",
            V2CoreAccountDefaultsLocalesItem::CsCZ => "cs-CZ",
            V2CoreAccountDefaultsLocalesItem::Da => "da",
            V2CoreAccountDefaultsLocalesItem::DaDK => "da-DK",
            V2CoreAccountDefaultsLocalesItem::De => "de",
            V2CoreAccountDefaultsLocalesItem::DeDE => "de-DE",
            V2CoreAccountDefaultsLocalesItem::El => "el",
            V2CoreAccountDefaultsLocalesItem::ElGR => "el-GR",
            V2CoreAccountDefaultsLocalesItem::En => "en",
            V2CoreAccountDefaultsLocalesItem::EnAU => "en-AU",
            V2CoreAccountDefaultsLocalesItem::EnCA => "en-CA",
            V2CoreAccountDefaultsLocalesItem::EnGB => "en-GB",
            V2CoreAccountDefaultsLocalesItem::EnIE => "en-IE",
            V2CoreAccountDefaultsLocalesItem::EnIN => "en-IN",
            V2CoreAccountDefaultsLocalesItem::EnNZ => "en-NZ",
            V2CoreAccountDefaultsLocalesItem::EnSG => "en-SG",
            V2CoreAccountDefaultsLocalesItem::EnUS => "en-US",
            V2CoreAccountDefaultsLocalesItem::Es => "es",
            V2CoreAccountDefaultsLocalesItem::Es419 => "es-419",
            V2CoreAccountDefaultsLocalesItem::EsES => "es-ES",
            V2CoreAccountDefaultsLocalesItem::Et => "et",
            V2CoreAccountDefaultsLocalesItem::EtEE => "et-EE",
            V2CoreAccountDefaultsLocalesItem::Fi => "fi",
            V2CoreAccountDefaultsLocalesItem::Fil => "fil",
            V2CoreAccountDefaultsLocalesItem::FilPH => "fil-PH",
            V2CoreAccountDefaultsLocalesItem::FiFI => "fi-FI",
            V2CoreAccountDefaultsLocalesItem::Fr => "fr",
            V2CoreAccountDefaultsLocalesItem::FrCA => "fr-CA",
            V2CoreAccountDefaultsLocalesItem::FrFR => "fr-FR",
            V2CoreAccountDefaultsLocalesItem::HeIL => "he-IL",
            V2CoreAccountDefaultsLocalesItem::Hr => "hr",
            V2CoreAccountDefaultsLocalesItem::HrHR => "hr-HR",
            V2CoreAccountDefaultsLocalesItem::Hu => "hu",
            V2CoreAccountDefaultsLocalesItem::HuHU => "hu-HU",
            V2CoreAccountDefaultsLocalesItem::Id => "id",
            V2CoreAccountDefaultsLocalesItem::IdID => "id-ID",
            V2CoreAccountDefaultsLocalesItem::It => "it",
            V2CoreAccountDefaultsLocalesItem::ItIT => "it-IT",
            V2CoreAccountDefaultsLocalesItem::Ja => "ja",
            V2CoreAccountDefaultsLocalesItem::JaJP => "ja-JP",
            V2CoreAccountDefaultsLocalesItem::Ko => "ko",
            V2CoreAccountDefaultsLocalesItem::KoKR => "ko-KR",
            V2CoreAccountDefaultsLocalesItem::Lt => "lt",
            V2CoreAccountDefaultsLocalesItem::LtLT => "lt-LT",
            V2CoreAccountDefaultsLocalesItem::Lv => "lv",
            V2CoreAccountDefaultsLocalesItem::LvLV => "lv-LV",
            V2CoreAccountDefaultsLocalesItem::Ms => "ms",
            V2CoreAccountDefaultsLocalesItem::MsMY => "ms-MY",
            V2CoreAccountDefaultsLocalesItem::Mt => "mt",
            V2CoreAccountDefaultsLocalesItem::MtMT => "mt-MT",
            V2CoreAccountDefaultsLocalesItem::Nb => "nb",
            V2CoreAccountDefaultsLocalesItem::NbNO => "nb-NO",
            V2CoreAccountDefaultsLocalesItem::Nl => "nl",
            V2CoreAccountDefaultsLocalesItem::NlNL => "nl-NL",
            V2CoreAccountDefaultsLocalesItem::Pl => "pl",
            V2CoreAccountDefaultsLocalesItem::PlPL => "pl-PL",
            V2CoreAccountDefaultsLocalesItem::Pt => "pt",
            V2CoreAccountDefaultsLocalesItem::PtBR => "pt-BR",
            V2CoreAccountDefaultsLocalesItem::PtPT => "pt-PT",
            V2CoreAccountDefaultsLocalesItem::Ro => "ro",
            V2CoreAccountDefaultsLocalesItem::RoRO => "ro-RO",
            V2CoreAccountDefaultsLocalesItem::Ru => "ru",
            V2CoreAccountDefaultsLocalesItem::RuRU => "ru-RU",
            V2CoreAccountDefaultsLocalesItem::Sk => "sk",
            V2CoreAccountDefaultsLocalesItem::SkSK => "sk-SK",
            V2CoreAccountDefaultsLocalesItem::Sl => "sl",
            V2CoreAccountDefaultsLocalesItem::SlSI => "sl-SI",
            V2CoreAccountDefaultsLocalesItem::Sv => "sv",
            V2CoreAccountDefaultsLocalesItem::SvSE => "sv-SE",
            V2CoreAccountDefaultsLocalesItem::Th => "th",
            V2CoreAccountDefaultsLocalesItem::ThTH => "th-TH",
            V2CoreAccountDefaultsLocalesItem::Tr => "tr",
            V2CoreAccountDefaultsLocalesItem::TrTR => "tr-TR",
            V2CoreAccountDefaultsLocalesItem::Vi => "vi",
            V2CoreAccountDefaultsLocalesItem::ViVN => "vi-VN",
            V2CoreAccountDefaultsLocalesItem::Zh => "zh",
            V2CoreAccountDefaultsLocalesItem::ZhHans => "zh-Hans",
            V2CoreAccountDefaultsLocalesItem::ZhHantHK => "zh-Hant-HK",
            V2CoreAccountDefaultsLocalesItem::ZhHantTW => "zh-Hant-TW",
            V2CoreAccountDefaultsLocalesItem::ZhHK => "zh-HK",
            V2CoreAccountDefaultsLocalesItem::ZhTW => "zh-TW",
            V2CoreAccountDefaultsLocalesItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreAccountDefaultsLocalesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountDefaultsLocalesItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ar-SA" => V2CoreAccountDefaultsLocalesItem::ArSA,
            "bg" => V2CoreAccountDefaultsLocalesItem::Bg,
            "bg-BG" => V2CoreAccountDefaultsLocalesItem::BgBG,
            "cs" => V2CoreAccountDefaultsLocalesItem::Cs,
            "cs-CZ" => V2CoreAccountDefaultsLocalesItem::CsCZ,
            "da" => V2CoreAccountDefaultsLocalesItem::Da,
            "da-DK" => V2CoreAccountDefaultsLocalesItem::DaDK,
            "de" => V2CoreAccountDefaultsLocalesItem::De,
            "de-DE" => V2CoreAccountDefaultsLocalesItem::DeDE,
            "el" => V2CoreAccountDefaultsLocalesItem::El,
            "el-GR" => V2CoreAccountDefaultsLocalesItem::ElGR,
            "en" => V2CoreAccountDefaultsLocalesItem::En,
            "en-AU" => V2CoreAccountDefaultsLocalesItem::EnAU,
            "en-CA" => V2CoreAccountDefaultsLocalesItem::EnCA,
            "en-GB" => V2CoreAccountDefaultsLocalesItem::EnGB,
            "en-IE" => V2CoreAccountDefaultsLocalesItem::EnIE,
            "en-IN" => V2CoreAccountDefaultsLocalesItem::EnIN,
            "en-NZ" => V2CoreAccountDefaultsLocalesItem::EnNZ,
            "en-SG" => V2CoreAccountDefaultsLocalesItem::EnSG,
            "en-US" => V2CoreAccountDefaultsLocalesItem::EnUS,
            "es" => V2CoreAccountDefaultsLocalesItem::Es,
            "es-419" => V2CoreAccountDefaultsLocalesItem::Es419,
            "es-ES" => V2CoreAccountDefaultsLocalesItem::EsES,
            "et" => V2CoreAccountDefaultsLocalesItem::Et,
            "et-EE" => V2CoreAccountDefaultsLocalesItem::EtEE,
            "fi" => V2CoreAccountDefaultsLocalesItem::Fi,
            "fil" => V2CoreAccountDefaultsLocalesItem::Fil,
            "fil-PH" => V2CoreAccountDefaultsLocalesItem::FilPH,
            "fi-FI" => V2CoreAccountDefaultsLocalesItem::FiFI,
            "fr" => V2CoreAccountDefaultsLocalesItem::Fr,
            "fr-CA" => V2CoreAccountDefaultsLocalesItem::FrCA,
            "fr-FR" => V2CoreAccountDefaultsLocalesItem::FrFR,
            "he-IL" => V2CoreAccountDefaultsLocalesItem::HeIL,
            "hr" => V2CoreAccountDefaultsLocalesItem::Hr,
            "hr-HR" => V2CoreAccountDefaultsLocalesItem::HrHR,
            "hu" => V2CoreAccountDefaultsLocalesItem::Hu,
            "hu-HU" => V2CoreAccountDefaultsLocalesItem::HuHU,
            "id" => V2CoreAccountDefaultsLocalesItem::Id,
            "id-ID" => V2CoreAccountDefaultsLocalesItem::IdID,
            "it" => V2CoreAccountDefaultsLocalesItem::It,
            "it-IT" => V2CoreAccountDefaultsLocalesItem::ItIT,
            "ja" => V2CoreAccountDefaultsLocalesItem::Ja,
            "ja-JP" => V2CoreAccountDefaultsLocalesItem::JaJP,
            "ko" => V2CoreAccountDefaultsLocalesItem::Ko,
            "ko-KR" => V2CoreAccountDefaultsLocalesItem::KoKR,
            "lt" => V2CoreAccountDefaultsLocalesItem::Lt,
            "lt-LT" => V2CoreAccountDefaultsLocalesItem::LtLT,
            "lv" => V2CoreAccountDefaultsLocalesItem::Lv,
            "lv-LV" => V2CoreAccountDefaultsLocalesItem::LvLV,
            "ms" => V2CoreAccountDefaultsLocalesItem::Ms,
            "ms-MY" => V2CoreAccountDefaultsLocalesItem::MsMY,
            "mt" => V2CoreAccountDefaultsLocalesItem::Mt,
            "mt-MT" => V2CoreAccountDefaultsLocalesItem::MtMT,
            "nb" => V2CoreAccountDefaultsLocalesItem::Nb,
            "nb-NO" => V2CoreAccountDefaultsLocalesItem::NbNO,
            "nl" => V2CoreAccountDefaultsLocalesItem::Nl,
            "nl-NL" => V2CoreAccountDefaultsLocalesItem::NlNL,
            "pl" => V2CoreAccountDefaultsLocalesItem::Pl,
            "pl-PL" => V2CoreAccountDefaultsLocalesItem::PlPL,
            "pt" => V2CoreAccountDefaultsLocalesItem::Pt,
            "pt-BR" => V2CoreAccountDefaultsLocalesItem::PtBR,
            "pt-PT" => V2CoreAccountDefaultsLocalesItem::PtPT,
            "ro" => V2CoreAccountDefaultsLocalesItem::Ro,
            "ro-RO" => V2CoreAccountDefaultsLocalesItem::RoRO,
            "ru" => V2CoreAccountDefaultsLocalesItem::Ru,
            "ru-RU" => V2CoreAccountDefaultsLocalesItem::RuRU,
            "sk" => V2CoreAccountDefaultsLocalesItem::Sk,
            "sk-SK" => V2CoreAccountDefaultsLocalesItem::SkSK,
            "sl" => V2CoreAccountDefaultsLocalesItem::Sl,
            "sl-SI" => V2CoreAccountDefaultsLocalesItem::SlSI,
            "sv" => V2CoreAccountDefaultsLocalesItem::Sv,
            "sv-SE" => V2CoreAccountDefaultsLocalesItem::SvSE,
            "th" => V2CoreAccountDefaultsLocalesItem::Th,
            "th-TH" => V2CoreAccountDefaultsLocalesItem::ThTH,
            "tr" => V2CoreAccountDefaultsLocalesItem::Tr,
            "tr-TR" => V2CoreAccountDefaultsLocalesItem::TrTR,
            "vi" => V2CoreAccountDefaultsLocalesItem::Vi,
            "vi-VN" => V2CoreAccountDefaultsLocalesItem::ViVN,
            "zh" => V2CoreAccountDefaultsLocalesItem::Zh,
            "zh-Hans" => V2CoreAccountDefaultsLocalesItem::ZhHans,
            "zh-Hant-HK" => V2CoreAccountDefaultsLocalesItem::ZhHantHK,
            "zh-Hant-TW" => V2CoreAccountDefaultsLocalesItem::ZhHantTW,
            "zh-HK" => V2CoreAccountDefaultsLocalesItem::ZhHK,
            "zh-TW" => V2CoreAccountDefaultsLocalesItem::ZhTW,
            other => V2CoreAccountDefaultsLocalesItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountFutureRequirementsEntriesItemAwaitingActionFrom {
    Stripe,
    User,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountFutureRequirementsEntriesItemAwaitingActionFrom {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountFutureRequirementsEntriesItemAwaitingActionFrom::Stripe => "stripe",
            V2CoreAccountFutureRequirementsEntriesItemAwaitingActionFrom::User => "user",
            V2CoreAccountFutureRequirementsEntriesItemAwaitingActionFrom::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl Default for V2CoreAccountFutureRequirementsEntriesItemAwaitingActionFrom {
    fn default() -> Self {
        V2CoreAccountFutureRequirementsEntriesItemAwaitingActionFrom::Unknown(String::new())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreAccountFutureRequirementsEntriesItemAwaitingActionFrom {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountFutureRequirementsEntriesItemAwaitingActionFrom::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "stripe" => V2CoreAccountFutureRequirementsEntriesItemAwaitingActionFrom::Stripe,
            "user" => V2CoreAccountFutureRequirementsEntriesItemAwaitingActionFrom::User,
            other => V2CoreAccountFutureRequirementsEntriesItemAwaitingActionFrom::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountIdentityBusinessDetailsDocumentsPrimaryVerificationType {
    FrontBack,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountIdentityBusinessDetailsDocumentsPrimaryVerificationType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountIdentityBusinessDetailsDocumentsPrimaryVerificationType::FrontBack => "front_back",
            V2CoreAccountIdentityBusinessDetailsDocumentsPrimaryVerificationType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl Default for V2CoreAccountIdentityBusinessDetailsDocumentsPrimaryVerificationType {
    fn default() -> Self {
        V2CoreAccountIdentityBusinessDetailsDocumentsPrimaryVerificationType::Unknown(String::new())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreAccountIdentityBusinessDetailsDocumentsPrimaryVerificationType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountIdentityBusinessDetailsDocumentsPrimaryVerificationType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "front_back" => V2CoreAccountIdentityBusinessDetailsDocumentsPrimaryVerificationType::FrontBack,
            other => V2CoreAccountIdentityBusinessDetailsDocumentsPrimaryVerificationType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountIdentityIndividualLegalGender {
    Female,
    Male,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountIdentityIndividualLegalGender {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountIdentityIndividualLegalGender::Female => "female",
            V2CoreAccountIdentityIndividualLegalGender::Male => "male",
            V2CoreAccountIdentityIndividualLegalGender::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreAccountIdentityIndividualLegalGender {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountIdentityIndividualLegalGender::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "female" => V2CoreAccountIdentityIndividualLegalGender::Female,
            "male" => V2CoreAccountIdentityIndividualLegalGender::Male,
            other => V2CoreAccountIdentityIndividualLegalGender::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountLinkUseCaseAccountOnboardingCollectionOptionsFutureRequirements {
    Include,
    Omit,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountLinkUseCaseAccountOnboardingCollectionOptionsFutureRequirements {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountLinkUseCaseAccountOnboardingCollectionOptionsFutureRequirements::Include => "include",
            V2CoreAccountLinkUseCaseAccountOnboardingCollectionOptionsFutureRequirements::Omit => "omit",
            V2CoreAccountLinkUseCaseAccountOnboardingCollectionOptionsFutureRequirements::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreAccountLinkUseCaseAccountOnboardingCollectionOptionsFutureRequirements {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                V2CoreAccountLinkUseCaseAccountOnboardingCollectionOptionsFutureRequirements::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "include" => V2CoreAccountLinkUseCaseAccountOnboardingCollectionOptionsFutureRequirements::Include,
            "omit" => V2CoreAccountLinkUseCaseAccountOnboardingCollectionOptionsFutureRequirements::Omit,
            other => {
                V2CoreAccountLinkUseCaseAccountOnboardingCollectionOptionsFutureRequirements::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountPersonObject {
    V2CoreAccountPerson,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountPersonObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountPersonObject::V2CoreAccountPerson => "v2.core.account_person",
            V2CoreAccountPersonObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl Default for V2CoreAccountPersonObject {
    fn default() -> Self {
        V2CoreAccountPersonObject::Unknown(String::new())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreAccountPersonObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountPersonObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.core.account_person" => V2CoreAccountPersonObject::V2CoreAccountPerson,
            other => V2CoreAccountPersonObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreEventDestinationStatus {
    Disabled,
    Enabled,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreEventDestinationStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreEventDestinationStatus::Disabled => "disabled",
            V2CoreEventDestinationStatus::Enabled => "enabled",
            V2CoreEventDestinationStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl Default for V2CoreEventDestinationStatus {
    fn default() -> Self {
        V2CoreEventDestinationStatus::Unknown(String::new())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreEventDestinationStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreEventDestinationStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "disabled" => V2CoreEventDestinationStatus::Disabled,
            "enabled" => V2CoreEventDestinationStatus::Enabled,
            other => V2CoreEventDestinationStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreEventReasonType {
    Request,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreEventReasonType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreEventReasonType::Request => "request",
            V2CoreEventReasonType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl Default for V2CoreEventReasonType {
    fn default() -> Self {
        V2CoreEventReasonType::Unknown(String::new())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreEventReasonType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreEventReasonType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "request" => V2CoreEventReasonType::Request,
            other => V2CoreEventReasonType::Unknown(other.to_string()),
        })
    }
}
