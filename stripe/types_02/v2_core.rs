#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsConfigurationMerchantBacsDebitPayments {
    #[doc = "Display name for Bacs Direct Debit payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCoreAccountParamsDashboard {
    Express,
    Full,
    None,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl CreateCoreAccountParamsDashboard {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCoreAccountParamsDashboard::Express => "express",
            CreateCoreAccountParamsDashboard::Full => "full",
            CreateCoreAccountParamsDashboard::None => "none",
            CreateCoreAccountParamsDashboard::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl json::Encode for CreateCoreAccountParamsDashboard {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for CreateCoreAccountParamsDashboard {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCoreAccountParamsDashboard::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "express" => CreateCoreAccountParamsDashboard::Express,
            "full" => CreateCoreAccountParamsDashboard::Full,
            "none" => CreateCoreAccountParamsDashboard::None,
            other => CreateCoreAccountParamsDashboard::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCoreAccountTokenParamsIdentityAttestationsPersonsProvidedOwnershipExemptionReason {
    QualifiedEntityExceedsOwnershipThreshold,
    QualifiesAsFinancialInstitution,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl CreateCoreAccountTokenParamsIdentityAttestationsPersonsProvidedOwnershipExemptionReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCoreAccountTokenParamsIdentityAttestationsPersonsProvidedOwnershipExemptionReason::QualifiedEntityExceedsOwnershipThreshold => "qualified_entity_exceeds_ownership_threshold",
            CreateCoreAccountTokenParamsIdentityAttestationsPersonsProvidedOwnershipExemptionReason::QualifiesAsFinancialInstitution => "qualifies_as_financial_institution",
            CreateCoreAccountTokenParamsIdentityAttestationsPersonsProvidedOwnershipExemptionReason::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl json::Encode for CreateCoreAccountTokenParamsIdentityAttestationsPersonsProvidedOwnershipExemptionReason {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json>
    for CreateCoreAccountTokenParamsIdentityAttestationsPersonsProvidedOwnershipExemptionReason
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCoreAccountTokenParamsIdentityAttestationsPersonsProvidedOwnershipExemptionReason::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "qualified_entity_exceeds_ownership_threshold" => CreateCoreAccountTokenParamsIdentityAttestationsPersonsProvidedOwnershipExemptionReason::QualifiedEntityExceedsOwnershipThreshold,
            "qualifies_as_financial_institution" => CreateCoreAccountTokenParamsIdentityAttestationsPersonsProvidedOwnershipExemptionReason::QualifiesAsFinancialInstitution,
            other => CreateCoreAccountTokenParamsIdentityAttestationsPersonsProvidedOwnershipExemptionReason::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCoreAccountTokenParamsIdentityBusinessDetailsAdditionalAddressesItemPurpose {
    Administrative,
    PrincipalPlaceOfBusiness,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl CreateCoreAccountTokenParamsIdentityBusinessDetailsAdditionalAddressesItemPurpose {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCoreAccountTokenParamsIdentityBusinessDetailsAdditionalAddressesItemPurpose::Administrative => "administrative",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsAdditionalAddressesItemPurpose::PrincipalPlaceOfBusiness => "principal_place_of_business",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsAdditionalAddressesItemPurpose::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl json::Encode for CreateCoreAccountTokenParamsIdentityBusinessDetailsAdditionalAddressesItemPurpose {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for CreateCoreAccountTokenParamsIdentityBusinessDetailsAdditionalAddressesItemPurpose {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCoreAccountTokenParamsIdentityBusinessDetailsAdditionalAddressesItemPurpose::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "administrative" => CreateCoreAccountTokenParamsIdentityBusinessDetailsAdditionalAddressesItemPurpose::Administrative,
            "principal_place_of_business" => CreateCoreAccountTokenParamsIdentityBusinessDetailsAdditionalAddressesItemPurpose::PrincipalPlaceOfBusiness,
            other => CreateCoreAccountTokenParamsIdentityBusinessDetailsAdditionalAddressesItemPurpose::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType {
    Files,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType::Files => "files",
            CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl json::Encode for CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json>
    for CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "files" => CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType::Files,
            other => CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsBankAccountOwnershipVerificationType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsPrimaryVerificationFrontBack {
    #[doc = "A [file upload](https://docs.stripe.com/api/persons/update#create_file) token representing the back of the verificati..."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub back: Option<String>,
    #[doc = "A [file upload](https://docs.stripe.com/api/persons/update#create_file) token representing the front of the verificat..."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub front: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsConfigurationCustomerCapabilitiesAutomaticIndirectTax {
    #[doc = "To request a new Capability for an account, pass true."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountDefaultsResponsibilitiesLossesCollector {
    Application,
    Stripe,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountDefaultsResponsibilitiesLossesCollector {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountDefaultsResponsibilitiesLossesCollector::Application => "application",
            V2CoreAccountDefaultsResponsibilitiesLossesCollector::Stripe => "stripe",
            V2CoreAccountDefaultsResponsibilitiesLossesCollector::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl Default for V2CoreAccountDefaultsResponsibilitiesLossesCollector {
    fn default() -> Self {
        V2CoreAccountDefaultsResponsibilitiesLossesCollector::Unknown(String::new())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreAccountDefaultsResponsibilitiesLossesCollector {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountDefaultsResponsibilitiesLossesCollector::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "application" => V2CoreAccountDefaultsResponsibilitiesLossesCollector::Application,
            "stripe" => V2CoreAccountDefaultsResponsibilitiesLossesCollector::Stripe,
            other => V2CoreAccountDefaultsResponsibilitiesLossesCollector::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountIdentityAttestationsDirectorshipDeclaration {
    #[doc = "The time marking when the director attestation was made."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[doc = "The IP address from which the director attestation was made."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[doc = "The user agent of the browser from which the director attestation was made."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountIdentityBusinessDetailsDocumentsPrimaryVerificationFrontBack {
    #[doc = "A [file upload](https://docs.stripe.com/api/persons/update#create_file) token representing the back of the verificati..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub back: Option<String>,
    #[doc = "A [file upload](https://docs.stripe.com/api/persons/update#create_file) token representing the front of the verificat..."]
    #[json(default)]
    pub front: String,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountIdentityBusinessDetailsIdNumbersItemType {
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
impl V2CoreAccountIdentityBusinessDetailsIdNumbersItemType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::AeCrn => "ae_crn",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::AeVat => "ae_vat",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::AoNif => "ao_nif",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::ArCuit => "ar_cuit",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::AtFn => "at_fn",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::AtStn => "at_stn",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::AtVat => "at_vat",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::AuAbn => "au_abn",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::AuAcn => "au_acn",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::AuIn => "au_in",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::AzTin => "az_tin",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::BdEtin => "bd_etin",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::BeCbe => "be_cbe",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::BeVat => "be_vat",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::BgUic => "bg_uic",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::BgVat => "bg_vat",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::BrCnpj => "br_cnpj",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::CaCn => "ca_cn",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::CaCrarr => "ca_crarr",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::CaGstHst => "ca_gst_hst",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::CaNeq => "ca_neq",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::CaRid => "ca_rid",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::ChChid => "ch_chid",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::ChUid => "ch_uid",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::CrCpj => "cr_cpj",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::CrNite => "cr_nite",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::CyHe => "cy_he",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::CyTic => "cy_tic",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::CyVat => "cy_vat",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::CzIco => "cz_ico",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::CzVat => "cz_vat",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::DeHrn => "de_hrn",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::DeStn => "de_stn",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::DeVat => "de_vat",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::DkCvr => "dk_cvr",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::DkVat => "dk_vat",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::DoRcn => "do_rcn",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::EeRk => "ee_rk",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::EeVat => "ee_vat",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::EsCif => "es_cif",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::EsVat => "es_vat",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::FiVat => "fi_vat",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::FiYt => "fi_yt",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::FrRna => "fr_rna",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::FrSiren => "fr_siren",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::FrVat => "fr_vat",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::GbCrn => "gb_crn",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::GbVat => "gb_vat",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::GiCrn => "gi_crn",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::GrAfm => "gr_afm",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::GrGemi => "gr_gemi",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::GrVat => "gr_vat",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::GtNit => "gt_nit",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::HkBr => "hk_br",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::HkCr => "hk_cr",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::HrMbs => "hr_mbs",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::HrOib => "hr_oib",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::HrVat => "hr_vat",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::HuCjs => "hu_cjs",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::HuTin => "hu_tin",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::HuVat => "hu_vat",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::IeCrn => "ie_crn",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::IeTrn => "ie_trn",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::IeVat => "ie_vat",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::ItRea => "it_rea",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::ItVat => "it_vat",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::JpCn => "jp_cn",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::KzBin => "kz_bin",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::LiUid => "li_uid",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::LtCcrn => "lt_ccrn",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::LtVat => "lt_vat",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::LuNif => "lu_nif",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::LuRcs => "lu_rcs",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::LuVat => "lu_vat",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::LvUrn => "lv_urn",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::LvVat => "lv_vat",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::MtCrn => "mt_crn",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::MtTin => "mt_tin",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::MtVat => "mt_vat",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::MxRfc => "mx_rfc",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::MyBrn => "my_brn",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::MyCoid => "my_coid",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::MyItn => "my_itn",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::MySst => "my_sst",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::MzNuit => "mz_nuit",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::NlKvk => "nl_kvk",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::NlRsin => "nl_rsin",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::NlVat => "nl_vat",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::NoOrgnr => "no_orgnr",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::NzBn => "nz_bn",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::NzIrd => "nz_ird",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::PeRuc => "pe_ruc",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::PkNtn => "pk_ntn",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::PlNip => "pl_nip",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::PlRegon => "pl_regon",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::PlVat => "pl_vat",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::PtVat => "pt_vat",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::RoCui => "ro_cui",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::RoOrc => "ro_orc",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::RoVat => "ro_vat",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::SaCrn => "sa_crn",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::SaTin => "sa_tin",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::SeOrgnr => "se_orgnr",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::SeVat => "se_vat",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::SgUen => "sg_uen",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::SiMsp => "si_msp",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::SiTin => "si_tin",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::SiVat => "si_vat",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::SkDic => "sk_dic",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::SkIco => "sk_ico",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::SkVat => "sk_vat",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::ThCrn => "th_crn",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::ThPrn => "th_prn",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::ThTin => "th_tin",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::UsEin => "us_ein",
            V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl Default for V2CoreAccountIdentityBusinessDetailsIdNumbersItemType {
    fn default() -> Self {
        V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::Unknown(String::new())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreAccountIdentityBusinessDetailsIdNumbersItemType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ae_crn" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::AeCrn,
            "ae_vat" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::AeVat,
            "ao_nif" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::AoNif,
            "ar_cuit" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::ArCuit,
            "at_fn" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::AtFn,
            "at_stn" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::AtStn,
            "at_vat" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::AtVat,
            "au_abn" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::AuAbn,
            "au_acn" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::AuAcn,
            "au_in" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::AuIn,
            "az_tin" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::AzTin,
            "bd_etin" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::BdEtin,
            "be_cbe" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::BeCbe,
            "be_vat" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::BeVat,
            "bg_uic" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::BgUic,
            "bg_vat" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::BgVat,
            "br_cnpj" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::BrCnpj,
            "ca_cn" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::CaCn,
            "ca_crarr" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::CaCrarr,
            "ca_gst_hst" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::CaGstHst,
            "ca_neq" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::CaNeq,
            "ca_rid" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::CaRid,
            "ch_chid" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::ChChid,
            "ch_uid" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::ChUid,
            "cr_cpj" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::CrCpj,
            "cr_nite" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::CrNite,
            "cy_he" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::CyHe,
            "cy_tic" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::CyTic,
            "cy_vat" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::CyVat,
            "cz_ico" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::CzIco,
            "cz_vat" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::CzVat,
            "de_hrn" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::DeHrn,
            "de_stn" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::DeStn,
            "de_vat" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::DeVat,
            "dk_cvr" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::DkCvr,
            "dk_vat" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::DkVat,
            "do_rcn" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::DoRcn,
            "ee_rk" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::EeRk,
            "ee_vat" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::EeVat,
            "es_cif" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::EsCif,
            "es_vat" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::EsVat,
            "fi_vat" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::FiVat,
            "fi_yt" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::FiYt,
            "fr_rna" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::FrRna,
            "fr_siren" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::FrSiren,
            "fr_vat" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::FrVat,
            "gb_crn" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::GbCrn,
            "gb_vat" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::GbVat,
            "gi_crn" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::GiCrn,
            "gr_afm" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::GrAfm,
            "gr_gemi" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::GrGemi,
            "gr_vat" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::GrVat,
            "gt_nit" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::GtNit,
            "hk_br" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::HkBr,
            "hk_cr" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::HkCr,
            "hr_mbs" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::HrMbs,
            "hr_oib" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::HrOib,
            "hr_vat" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::HrVat,
            "hu_cjs" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::HuCjs,
            "hu_tin" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::HuTin,
            "hu_vat" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::HuVat,
            "ie_crn" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::IeCrn,
            "ie_trn" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::IeTrn,
            "ie_vat" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::IeVat,
            "it_rea" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::ItRea,
            "it_vat" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::ItVat,
            "jp_cn" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::JpCn,
            "kz_bin" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::KzBin,
            "li_uid" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::LiUid,
            "lt_ccrn" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::LtCcrn,
            "lt_vat" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::LtVat,
            "lu_nif" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::LuNif,
            "lu_rcs" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::LuRcs,
            "lu_vat" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::LuVat,
            "lv_urn" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::LvUrn,
            "lv_vat" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::LvVat,
            "mt_crn" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::MtCrn,
            "mt_tin" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::MtTin,
            "mt_vat" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::MtVat,
            "mx_rfc" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::MxRfc,
            "my_brn" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::MyBrn,
            "my_coid" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::MyCoid,
            "my_itn" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::MyItn,
            "my_sst" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::MySst,
            "mz_nuit" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::MzNuit,
            "nl_kvk" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::NlKvk,
            "nl_rsin" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::NlRsin,
            "nl_vat" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::NlVat,
            "no_orgnr" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::NoOrgnr,
            "nz_bn" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::NzBn,
            "nz_ird" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::NzIrd,
            "pe_ruc" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::PeRuc,
            "pk_ntn" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::PkNtn,
            "pl_nip" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::PlNip,
            "pl_regon" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::PlRegon,
            "pl_vat" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::PlVat,
            "pt_vat" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::PtVat,
            "ro_cui" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::RoCui,
            "ro_orc" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::RoOrc,
            "ro_vat" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::RoVat,
            "sa_crn" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::SaCrn,
            "sa_tin" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::SaTin,
            "se_orgnr" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::SeOrgnr,
            "se_vat" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::SeVat,
            "sg_uen" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::SgUen,
            "si_msp" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::SiMsp,
            "si_tin" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::SiTin,
            "si_vat" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::SiVat,
            "sk_dic" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::SkDic,
            "sk_ico" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::SkIco,
            "sk_vat" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::SkVat,
            "th_crn" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::ThCrn,
            "th_prn" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::ThPrn,
            "th_tin" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::ThTin,
            "us_ein" => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::UsEin,
            other => V2CoreAccountIdentityBusinessDetailsIdNumbersItemType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountIdentityBusinessDetailsStructure {
    Cooperative,
    FreeZoneEstablishment,
    FreeZoneLlc,
    GovernmentalUnit,
    GovernmentInstrumentality,
    IncorporatedAssociation,
    IncorporatedNonProfit,
    IncorporatedPartnership,
    LimitedLiabilityPartnership,
    Llc,
    MultiMemberLlc,
    PrivateCompany,
    PrivateCorporation,
    PrivatePartnership,
    PublicCompany,
    PublicCorporation,
    PublicListedCorporation,
    PublicPartnership,
    RegisteredCharity,
    SingleMemberLlc,
    SoleEstablishment,
    SoleProprietorship,
    TaxExemptGovernmentInstrumentality,
    Trust,
    UnincorporatedAssociation,
    UnincorporatedNonProfit,
    UnincorporatedPartnership,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountIdentityBusinessDetailsStructure {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountIdentityBusinessDetailsStructure::Cooperative => "cooperative",
            V2CoreAccountIdentityBusinessDetailsStructure::FreeZoneEstablishment => "free_zone_establishment",
            V2CoreAccountIdentityBusinessDetailsStructure::FreeZoneLlc => "free_zone_llc",
            V2CoreAccountIdentityBusinessDetailsStructure::GovernmentalUnit => "governmental_unit",
            V2CoreAccountIdentityBusinessDetailsStructure::GovernmentInstrumentality => "government_instrumentality",
            V2CoreAccountIdentityBusinessDetailsStructure::IncorporatedAssociation => "incorporated_association",
            V2CoreAccountIdentityBusinessDetailsStructure::IncorporatedNonProfit => "incorporated_non_profit",
            V2CoreAccountIdentityBusinessDetailsStructure::IncorporatedPartnership => "incorporated_partnership",
            V2CoreAccountIdentityBusinessDetailsStructure::LimitedLiabilityPartnership => {
                "limited_liability_partnership"
            }
            V2CoreAccountIdentityBusinessDetailsStructure::Llc => "llc",
            V2CoreAccountIdentityBusinessDetailsStructure::MultiMemberLlc => "multi_member_llc",
            V2CoreAccountIdentityBusinessDetailsStructure::PrivateCompany => "private_company",
            V2CoreAccountIdentityBusinessDetailsStructure::PrivateCorporation => "private_corporation",
            V2CoreAccountIdentityBusinessDetailsStructure::PrivatePartnership => "private_partnership",
            V2CoreAccountIdentityBusinessDetailsStructure::PublicCompany => "public_company",
            V2CoreAccountIdentityBusinessDetailsStructure::PublicCorporation => "public_corporation",
            V2CoreAccountIdentityBusinessDetailsStructure::PublicListedCorporation => "public_listed_corporation",
            V2CoreAccountIdentityBusinessDetailsStructure::PublicPartnership => "public_partnership",
            V2CoreAccountIdentityBusinessDetailsStructure::RegisteredCharity => "registered_charity",
            V2CoreAccountIdentityBusinessDetailsStructure::SingleMemberLlc => "single_member_llc",
            V2CoreAccountIdentityBusinessDetailsStructure::SoleEstablishment => "sole_establishment",
            V2CoreAccountIdentityBusinessDetailsStructure::SoleProprietorship => "sole_proprietorship",
            V2CoreAccountIdentityBusinessDetailsStructure::TaxExemptGovernmentInstrumentality => {
                "tax_exempt_government_instrumentality"
            }
            V2CoreAccountIdentityBusinessDetailsStructure::Trust => "trust",
            V2CoreAccountIdentityBusinessDetailsStructure::UnincorporatedAssociation => "unincorporated_association",
            V2CoreAccountIdentityBusinessDetailsStructure::UnincorporatedNonProfit => "unincorporated_non_profit",
            V2CoreAccountIdentityBusinessDetailsStructure::UnincorporatedPartnership => "unincorporated_partnership",
            V2CoreAccountIdentityBusinessDetailsStructure::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreAccountIdentityBusinessDetailsStructure {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountIdentityBusinessDetailsStructure::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "cooperative" => V2CoreAccountIdentityBusinessDetailsStructure::Cooperative,
            "free_zone_establishment" => V2CoreAccountIdentityBusinessDetailsStructure::FreeZoneEstablishment,
            "free_zone_llc" => V2CoreAccountIdentityBusinessDetailsStructure::FreeZoneLlc,
            "governmental_unit" => V2CoreAccountIdentityBusinessDetailsStructure::GovernmentalUnit,
            "government_instrumentality" => V2CoreAccountIdentityBusinessDetailsStructure::GovernmentInstrumentality,
            "incorporated_association" => V2CoreAccountIdentityBusinessDetailsStructure::IncorporatedAssociation,
            "incorporated_non_profit" => V2CoreAccountIdentityBusinessDetailsStructure::IncorporatedNonProfit,
            "incorporated_partnership" => V2CoreAccountIdentityBusinessDetailsStructure::IncorporatedPartnership,
            "limited_liability_partnership" => {
                V2CoreAccountIdentityBusinessDetailsStructure::LimitedLiabilityPartnership
            }
            "llc" => V2CoreAccountIdentityBusinessDetailsStructure::Llc,
            "multi_member_llc" => V2CoreAccountIdentityBusinessDetailsStructure::MultiMemberLlc,
            "private_company" => V2CoreAccountIdentityBusinessDetailsStructure::PrivateCompany,
            "private_corporation" => V2CoreAccountIdentityBusinessDetailsStructure::PrivateCorporation,
            "private_partnership" => V2CoreAccountIdentityBusinessDetailsStructure::PrivatePartnership,
            "public_company" => V2CoreAccountIdentityBusinessDetailsStructure::PublicCompany,
            "public_corporation" => V2CoreAccountIdentityBusinessDetailsStructure::PublicCorporation,
            "public_listed_corporation" => V2CoreAccountIdentityBusinessDetailsStructure::PublicListedCorporation,
            "public_partnership" => V2CoreAccountIdentityBusinessDetailsStructure::PublicPartnership,
            "registered_charity" => V2CoreAccountIdentityBusinessDetailsStructure::RegisteredCharity,
            "single_member_llc" => V2CoreAccountIdentityBusinessDetailsStructure::SingleMemberLlc,
            "sole_establishment" => V2CoreAccountIdentityBusinessDetailsStructure::SoleEstablishment,
            "sole_proprietorship" => V2CoreAccountIdentityBusinessDetailsStructure::SoleProprietorship,
            "tax_exempt_government_instrumentality" => {
                V2CoreAccountIdentityBusinessDetailsStructure::TaxExemptGovernmentInstrumentality
            }
            "trust" => V2CoreAccountIdentityBusinessDetailsStructure::Trust,
            "unincorporated_association" => V2CoreAccountIdentityBusinessDetailsStructure::UnincorporatedAssociation,
            "unincorporated_non_profit" => V2CoreAccountIdentityBusinessDetailsStructure::UnincorporatedNonProfit,
            "unincorporated_partnership" => V2CoreAccountIdentityBusinessDetailsStructure::UnincorporatedPartnership,
            other => V2CoreAccountIdentityBusinessDetailsStructure::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountIdentityEntityType {
    Company,
    GovernmentEntity,
    Individual,
    NonProfit,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountIdentityEntityType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountIdentityEntityType::Company => "company",
            V2CoreAccountIdentityEntityType::GovernmentEntity => "government_entity",
            V2CoreAccountIdentityEntityType::Individual => "individual",
            V2CoreAccountIdentityEntityType::NonProfit => "non_profit",
            V2CoreAccountIdentityEntityType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreAccountIdentityEntityType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountIdentityEntityType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "company" => V2CoreAccountIdentityEntityType::Company,
            "government_entity" => V2CoreAccountIdentityEntityType::GovernmentEntity,
            "individual" => V2CoreAccountIdentityEntityType::Individual,
            "non_profit" => V2CoreAccountIdentityEntityType::NonProfit,
            other => V2CoreAccountIdentityEntityType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountIdentityIndividualAdditionalAddressesItemPurpose {
    Registered,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountIdentityIndividualAdditionalAddressesItemPurpose {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountIdentityIndividualAdditionalAddressesItemPurpose::Registered => "registered",
            V2CoreAccountIdentityIndividualAdditionalAddressesItemPurpose::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl Default for V2CoreAccountIdentityIndividualAdditionalAddressesItemPurpose {
    fn default() -> Self {
        V2CoreAccountIdentityIndividualAdditionalAddressesItemPurpose::Unknown(String::new())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreAccountIdentityIndividualAdditionalAddressesItemPurpose {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountIdentityIndividualAdditionalAddressesItemPurpose::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "registered" => V2CoreAccountIdentityIndividualAdditionalAddressesItemPurpose::Registered,
            other => V2CoreAccountIdentityIndividualAdditionalAddressesItemPurpose::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountIdentityIndividualAdditionalNamesItemPurpose {
    Alias,
    Maiden,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountIdentityIndividualAdditionalNamesItemPurpose {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountIdentityIndividualAdditionalNamesItemPurpose::Alias => "alias",
            V2CoreAccountIdentityIndividualAdditionalNamesItemPurpose::Maiden => "maiden",
            V2CoreAccountIdentityIndividualAdditionalNamesItemPurpose::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl Default for V2CoreAccountIdentityIndividualAdditionalNamesItemPurpose {
    fn default() -> Self {
        V2CoreAccountIdentityIndividualAdditionalNamesItemPurpose::Unknown(String::new())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreAccountIdentityIndividualAdditionalNamesItemPurpose {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountIdentityIndividualAdditionalNamesItemPurpose::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "alias" => V2CoreAccountIdentityIndividualAdditionalNamesItemPurpose::Alias,
            "maiden" => V2CoreAccountIdentityIndividualAdditionalNamesItemPurpose::Maiden,
            other => V2CoreAccountIdentityIndividualAdditionalNamesItemPurpose::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountIdentityIndividualIdNumbersItemType {
    AeEid,
    AoNif,
    ArCuil,
    ArDni,
    AtStn,
    AzTin,
    BdBrc,
    BdEtin,
    BdNid,
    BeNrn,
    BgUcn,
    BnNric,
    BrCpf,
    CaSin,
    ChOasi,
    ClRut,
    CnPp,
    CoNuip,
    CrCi,
    CrCpf,
    CrDimex,
    CrNite,
    CyTic,
    CzRc,
    DeStn,
    DkCpr,
    DoCie,
    DoRcn,
    EcCi,
    EeIk,
    EsNif,
    FiHetu,
    FrNir,
    GbNino,
    GrAfm,
    GtNit,
    HkId,
    HrOib,
    HuAd,
    IdNik,
    IePpsn,
    IsKt,
    ItCf,
    JpInc,
    KePin,
    KzIin,
    LiPeid,
    LtAk,
    LuNif,
    LvPk,
    MxRfc,
    MyNric,
    MzNuit,
    NgNin,
    NlBsn,
    NoNin,
    NzIrd,
    PeDni,
    PkCnic,
    PkSnic,
    PlPesel,
    PtNif,
    RoCnp,
    SaTin,
    SePin,
    SgFin,
    SgNric,
    SkDic,
    ThLc,
    ThPin,
    TrTin,
    UsItin,
    UsItinLast4,
    UsSsn,
    UsSsnLast4,
    UyDni,
    ZaId,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountIdentityIndividualIdNumbersItemType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountIdentityIndividualIdNumbersItemType::AeEid => "ae_eid",
            V2CoreAccountIdentityIndividualIdNumbersItemType::AoNif => "ao_nif",
            V2CoreAccountIdentityIndividualIdNumbersItemType::ArCuil => "ar_cuil",
            V2CoreAccountIdentityIndividualIdNumbersItemType::ArDni => "ar_dni",
            V2CoreAccountIdentityIndividualIdNumbersItemType::AtStn => "at_stn",
            V2CoreAccountIdentityIndividualIdNumbersItemType::AzTin => "az_tin",
            V2CoreAccountIdentityIndividualIdNumbersItemType::BdBrc => "bd_brc",
            V2CoreAccountIdentityIndividualIdNumbersItemType::BdEtin => "bd_etin",
            V2CoreAccountIdentityIndividualIdNumbersItemType::BdNid => "bd_nid",
            V2CoreAccountIdentityIndividualIdNumbersItemType::BeNrn => "be_nrn",
            V2CoreAccountIdentityIndividualIdNumbersItemType::BgUcn => "bg_ucn",
            V2CoreAccountIdentityIndividualIdNumbersItemType::BnNric => "bn_nric",
            V2CoreAccountIdentityIndividualIdNumbersItemType::BrCpf => "br_cpf",
            V2CoreAccountIdentityIndividualIdNumbersItemType::CaSin => "ca_sin",
            V2CoreAccountIdentityIndividualIdNumbersItemType::ChOasi => "ch_oasi",
            V2CoreAccountIdentityIndividualIdNumbersItemType::ClRut => "cl_rut",
            V2CoreAccountIdentityIndividualIdNumbersItemType::CnPp => "cn_pp",
            V2CoreAccountIdentityIndividualIdNumbersItemType::CoNuip => "co_nuip",
            V2CoreAccountIdentityIndividualIdNumbersItemType::CrCi => "cr_ci",
            V2CoreAccountIdentityIndividualIdNumbersItemType::CrCpf => "cr_cpf",
            V2CoreAccountIdentityIndividualIdNumbersItemType::CrDimex => "cr_dimex",
            V2CoreAccountIdentityIndividualIdNumbersItemType::CrNite => "cr_nite",
            V2CoreAccountIdentityIndividualIdNumbersItemType::CyTic => "cy_tic",
            V2CoreAccountIdentityIndividualIdNumbersItemType::CzRc => "cz_rc",
            V2CoreAccountIdentityIndividualIdNumbersItemType::DeStn => "de_stn",
            V2CoreAccountIdentityIndividualIdNumbersItemType::DkCpr => "dk_cpr",
            V2CoreAccountIdentityIndividualIdNumbersItemType::DoCie => "do_cie",
            V2CoreAccountIdentityIndividualIdNumbersItemType::DoRcn => "do_rcn",
            V2CoreAccountIdentityIndividualIdNumbersItemType::EcCi => "ec_ci",
            V2CoreAccountIdentityIndividualIdNumbersItemType::EeIk => "ee_ik",
            V2CoreAccountIdentityIndividualIdNumbersItemType::EsNif => "es_nif",
            V2CoreAccountIdentityIndividualIdNumbersItemType::FiHetu => "fi_hetu",
            V2CoreAccountIdentityIndividualIdNumbersItemType::FrNir => "fr_nir",
            V2CoreAccountIdentityIndividualIdNumbersItemType::GbNino => "gb_nino",
            V2CoreAccountIdentityIndividualIdNumbersItemType::GrAfm => "gr_afm",
            V2CoreAccountIdentityIndividualIdNumbersItemType::GtNit => "gt_nit",
            V2CoreAccountIdentityIndividualIdNumbersItemType::HkId => "hk_id",
            V2CoreAccountIdentityIndividualIdNumbersItemType::HrOib => "hr_oib",
            V2CoreAccountIdentityIndividualIdNumbersItemType::HuAd => "hu_ad",
            V2CoreAccountIdentityIndividualIdNumbersItemType::IdNik => "id_nik",
            V2CoreAccountIdentityIndividualIdNumbersItemType::IePpsn => "ie_ppsn",
            V2CoreAccountIdentityIndividualIdNumbersItemType::IsKt => "is_kt",
            V2CoreAccountIdentityIndividualIdNumbersItemType::ItCf => "it_cf",
            V2CoreAccountIdentityIndividualIdNumbersItemType::JpInc => "jp_inc",
            V2CoreAccountIdentityIndividualIdNumbersItemType::KePin => "ke_pin",
            V2CoreAccountIdentityIndividualIdNumbersItemType::KzIin => "kz_iin",
            V2CoreAccountIdentityIndividualIdNumbersItemType::LiPeid => "li_peid",
            V2CoreAccountIdentityIndividualIdNumbersItemType::LtAk => "lt_ak",
            V2CoreAccountIdentityIndividualIdNumbersItemType::LuNif => "lu_nif",
            V2CoreAccountIdentityIndividualIdNumbersItemType::LvPk => "lv_pk",
            V2CoreAccountIdentityIndividualIdNumbersItemType::MxRfc => "mx_rfc",
            V2CoreAccountIdentityIndividualIdNumbersItemType::MyNric => "my_nric",
            V2CoreAccountIdentityIndividualIdNumbersItemType::MzNuit => "mz_nuit",
            V2CoreAccountIdentityIndividualIdNumbersItemType::NgNin => "ng_nin",
            V2CoreAccountIdentityIndividualIdNumbersItemType::NlBsn => "nl_bsn",
            V2CoreAccountIdentityIndividualIdNumbersItemType::NoNin => "no_nin",
            V2CoreAccountIdentityIndividualIdNumbersItemType::NzIrd => "nz_ird",
            V2CoreAccountIdentityIndividualIdNumbersItemType::PeDni => "pe_dni",
            V2CoreAccountIdentityIndividualIdNumbersItemType::PkCnic => "pk_cnic",
            V2CoreAccountIdentityIndividualIdNumbersItemType::PkSnic => "pk_snic",
            V2CoreAccountIdentityIndividualIdNumbersItemType::PlPesel => "pl_pesel",
            V2CoreAccountIdentityIndividualIdNumbersItemType::PtNif => "pt_nif",
            V2CoreAccountIdentityIndividualIdNumbersItemType::RoCnp => "ro_cnp",
            V2CoreAccountIdentityIndividualIdNumbersItemType::SaTin => "sa_tin",
            V2CoreAccountIdentityIndividualIdNumbersItemType::SePin => "se_pin",
            V2CoreAccountIdentityIndividualIdNumbersItemType::SgFin => "sg_fin",
            V2CoreAccountIdentityIndividualIdNumbersItemType::SgNric => "sg_nric",
            V2CoreAccountIdentityIndividualIdNumbersItemType::SkDic => "sk_dic",
            V2CoreAccountIdentityIndividualIdNumbersItemType::ThLc => "th_lc",
            V2CoreAccountIdentityIndividualIdNumbersItemType::ThPin => "th_pin",
            V2CoreAccountIdentityIndividualIdNumbersItemType::TrTin => "tr_tin",
            V2CoreAccountIdentityIndividualIdNumbersItemType::UsItin => "us_itin",
            V2CoreAccountIdentityIndividualIdNumbersItemType::UsItinLast4 => "us_itin_last_4",
            V2CoreAccountIdentityIndividualIdNumbersItemType::UsSsn => "us_ssn",
            V2CoreAccountIdentityIndividualIdNumbersItemType::UsSsnLast4 => "us_ssn_last_4",
            V2CoreAccountIdentityIndividualIdNumbersItemType::UyDni => "uy_dni",
            V2CoreAccountIdentityIndividualIdNumbersItemType::ZaId => "za_id",
            V2CoreAccountIdentityIndividualIdNumbersItemType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl Default for V2CoreAccountIdentityIndividualIdNumbersItemType {
    fn default() -> Self {
        V2CoreAccountIdentityIndividualIdNumbersItemType::Unknown(String::new())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreAccountIdentityIndividualIdNumbersItemType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountIdentityIndividualIdNumbersItemType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ae_eid" => V2CoreAccountIdentityIndividualIdNumbersItemType::AeEid,
            "ao_nif" => V2CoreAccountIdentityIndividualIdNumbersItemType::AoNif,
            "ar_cuil" => V2CoreAccountIdentityIndividualIdNumbersItemType::ArCuil,
            "ar_dni" => V2CoreAccountIdentityIndividualIdNumbersItemType::ArDni,
            "at_stn" => V2CoreAccountIdentityIndividualIdNumbersItemType::AtStn,
            "az_tin" => V2CoreAccountIdentityIndividualIdNumbersItemType::AzTin,
            "bd_brc" => V2CoreAccountIdentityIndividualIdNumbersItemType::BdBrc,
            "bd_etin" => V2CoreAccountIdentityIndividualIdNumbersItemType::BdEtin,
            "bd_nid" => V2CoreAccountIdentityIndividualIdNumbersItemType::BdNid,
            "be_nrn" => V2CoreAccountIdentityIndividualIdNumbersItemType::BeNrn,
            "bg_ucn" => V2CoreAccountIdentityIndividualIdNumbersItemType::BgUcn,
            "bn_nric" => V2CoreAccountIdentityIndividualIdNumbersItemType::BnNric,
            "br_cpf" => V2CoreAccountIdentityIndividualIdNumbersItemType::BrCpf,
            "ca_sin" => V2CoreAccountIdentityIndividualIdNumbersItemType::CaSin,
            "ch_oasi" => V2CoreAccountIdentityIndividualIdNumbersItemType::ChOasi,
            "cl_rut" => V2CoreAccountIdentityIndividualIdNumbersItemType::ClRut,
            "cn_pp" => V2CoreAccountIdentityIndividualIdNumbersItemType::CnPp,
            "co_nuip" => V2CoreAccountIdentityIndividualIdNumbersItemType::CoNuip,
            "cr_ci" => V2CoreAccountIdentityIndividualIdNumbersItemType::CrCi,
            "cr_cpf" => V2CoreAccountIdentityIndividualIdNumbersItemType::CrCpf,
            "cr_dimex" => V2CoreAccountIdentityIndividualIdNumbersItemType::CrDimex,
            "cr_nite" => V2CoreAccountIdentityIndividualIdNumbersItemType::CrNite,
            "cy_tic" => V2CoreAccountIdentityIndividualIdNumbersItemType::CyTic,
            "cz_rc" => V2CoreAccountIdentityIndividualIdNumbersItemType::CzRc,
            "de_stn" => V2CoreAccountIdentityIndividualIdNumbersItemType::DeStn,
            "dk_cpr" => V2CoreAccountIdentityIndividualIdNumbersItemType::DkCpr,
            "do_cie" => V2CoreAccountIdentityIndividualIdNumbersItemType::DoCie,
            "do_rcn" => V2CoreAccountIdentityIndividualIdNumbersItemType::DoRcn,
            "ec_ci" => V2CoreAccountIdentityIndividualIdNumbersItemType::EcCi,
            "ee_ik" => V2CoreAccountIdentityIndividualIdNumbersItemType::EeIk,
            "es_nif" => V2CoreAccountIdentityIndividualIdNumbersItemType::EsNif,
            "fi_hetu" => V2CoreAccountIdentityIndividualIdNumbersItemType::FiHetu,
            "fr_nir" => V2CoreAccountIdentityIndividualIdNumbersItemType::FrNir,
            "gb_nino" => V2CoreAccountIdentityIndividualIdNumbersItemType::GbNino,
            "gr_afm" => V2CoreAccountIdentityIndividualIdNumbersItemType::GrAfm,
            "gt_nit" => V2CoreAccountIdentityIndividualIdNumbersItemType::GtNit,
            "hk_id" => V2CoreAccountIdentityIndividualIdNumbersItemType::HkId,
            "hr_oib" => V2CoreAccountIdentityIndividualIdNumbersItemType::HrOib,
            "hu_ad" => V2CoreAccountIdentityIndividualIdNumbersItemType::HuAd,
            "id_nik" => V2CoreAccountIdentityIndividualIdNumbersItemType::IdNik,
            "ie_ppsn" => V2CoreAccountIdentityIndividualIdNumbersItemType::IePpsn,
            "is_kt" => V2CoreAccountIdentityIndividualIdNumbersItemType::IsKt,
            "it_cf" => V2CoreAccountIdentityIndividualIdNumbersItemType::ItCf,
            "jp_inc" => V2CoreAccountIdentityIndividualIdNumbersItemType::JpInc,
            "ke_pin" => V2CoreAccountIdentityIndividualIdNumbersItemType::KePin,
            "kz_iin" => V2CoreAccountIdentityIndividualIdNumbersItemType::KzIin,
            "li_peid" => V2CoreAccountIdentityIndividualIdNumbersItemType::LiPeid,
            "lt_ak" => V2CoreAccountIdentityIndividualIdNumbersItemType::LtAk,
            "lu_nif" => V2CoreAccountIdentityIndividualIdNumbersItemType::LuNif,
            "lv_pk" => V2CoreAccountIdentityIndividualIdNumbersItemType::LvPk,
            "mx_rfc" => V2CoreAccountIdentityIndividualIdNumbersItemType::MxRfc,
            "my_nric" => V2CoreAccountIdentityIndividualIdNumbersItemType::MyNric,
            "mz_nuit" => V2CoreAccountIdentityIndividualIdNumbersItemType::MzNuit,
            "ng_nin" => V2CoreAccountIdentityIndividualIdNumbersItemType::NgNin,
            "nl_bsn" => V2CoreAccountIdentityIndividualIdNumbersItemType::NlBsn,
            "no_nin" => V2CoreAccountIdentityIndividualIdNumbersItemType::NoNin,
            "nz_ird" => V2CoreAccountIdentityIndividualIdNumbersItemType::NzIrd,
            "pe_dni" => V2CoreAccountIdentityIndividualIdNumbersItemType::PeDni,
            "pk_cnic" => V2CoreAccountIdentityIndividualIdNumbersItemType::PkCnic,
            "pk_snic" => V2CoreAccountIdentityIndividualIdNumbersItemType::PkSnic,
            "pl_pesel" => V2CoreAccountIdentityIndividualIdNumbersItemType::PlPesel,
            "pt_nif" => V2CoreAccountIdentityIndividualIdNumbersItemType::PtNif,
            "ro_cnp" => V2CoreAccountIdentityIndividualIdNumbersItemType::RoCnp,
            "sa_tin" => V2CoreAccountIdentityIndividualIdNumbersItemType::SaTin,
            "se_pin" => V2CoreAccountIdentityIndividualIdNumbersItemType::SePin,
            "sg_fin" => V2CoreAccountIdentityIndividualIdNumbersItemType::SgFin,
            "sg_nric" => V2CoreAccountIdentityIndividualIdNumbersItemType::SgNric,
            "sk_dic" => V2CoreAccountIdentityIndividualIdNumbersItemType::SkDic,
            "th_lc" => V2CoreAccountIdentityIndividualIdNumbersItemType::ThLc,
            "th_pin" => V2CoreAccountIdentityIndividualIdNumbersItemType::ThPin,
            "tr_tin" => V2CoreAccountIdentityIndividualIdNumbersItemType::TrTin,
            "us_itin" => V2CoreAccountIdentityIndividualIdNumbersItemType::UsItin,
            "us_itin_last_4" => V2CoreAccountIdentityIndividualIdNumbersItemType::UsItinLast4,
            "us_ssn" => V2CoreAccountIdentityIndividualIdNumbersItemType::UsSsn,
            "us_ssn_last_4" => V2CoreAccountIdentityIndividualIdNumbersItemType::UsSsnLast4,
            "uy_dni" => V2CoreAccountIdentityIndividualIdNumbersItemType::UyDni,
            "za_id" => V2CoreAccountIdentityIndividualIdNumbersItemType::ZaId,
            other => V2CoreAccountIdentityIndividualIdNumbersItemType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountIdentityIndividualRelationship {
    #[doc = "Whether the individual is an authorizer of the Account's identity."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub authorizer: Option<bool>,
    #[doc = "Whether the individual is a director of the Account's identity."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub director: Option<bool>,
    #[doc = "Whether the individual has significant responsibility to control, manage, or direct the organization."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub executive: Option<bool>,
    #[doc = "Whether the individual is the legal guardian of the Account's representative."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub legal_guardian: Option<bool>,
    #[doc = "Whether the individual is an owner of the Account's identity."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<bool>,
    #[doc = "The percentage of the Account's identity that the individual owns."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub percent_ownership: Option<String>,
    #[doc = "Whether the individual is authorized as the primary representative of the Account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub representative: Option<bool>,
    #[doc = "The individual's title (e.g., CEO, Support Engineer)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountIdentityIndividualScriptNamesKana {
    #[doc = "The person's first or given name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[doc = "The person's last or family name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountLinkObject {
    V2CoreAccountLink,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountLinkObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountLinkObject::V2CoreAccountLink => "v2.core.account_link",
            V2CoreAccountLinkObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl Default for V2CoreAccountLinkObject {
    fn default() -> Self {
        V2CoreAccountLinkObject::Unknown(String::new())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreAccountLinkObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountLinkObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.core.account_link" => V2CoreAccountLinkObject::V2CoreAccountLink,
            other => V2CoreAccountLinkObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountTokenObject {
    V2CoreAccountToken,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountTokenObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountTokenObject::V2CoreAccountToken => "v2.core.account_token",
            V2CoreAccountTokenObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl Default for V2CoreAccountTokenObject {
    fn default() -> Self {
        V2CoreAccountTokenObject::Unknown(String::new())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreAccountTokenObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountTokenObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.core.account_token" => V2CoreAccountTokenObject::V2CoreAccountToken,
            other => V2CoreAccountTokenObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreEventDestinationAmazonEventbridgeAwsEventSourceStatus {
    Active,
    Deleted,
    Pending,
    UnknownValue,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreEventDestinationAmazonEventbridgeAwsEventSourceStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreEventDestinationAmazonEventbridgeAwsEventSourceStatus::Active => "active",
            V2CoreEventDestinationAmazonEventbridgeAwsEventSourceStatus::Deleted => "deleted",
            V2CoreEventDestinationAmazonEventbridgeAwsEventSourceStatus::Pending => "pending",
            V2CoreEventDestinationAmazonEventbridgeAwsEventSourceStatus::UnknownValue => "unknown",
            V2CoreEventDestinationAmazonEventbridgeAwsEventSourceStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl Default for V2CoreEventDestinationAmazonEventbridgeAwsEventSourceStatus {
    fn default() -> Self {
        V2CoreEventDestinationAmazonEventbridgeAwsEventSourceStatus::Unknown(String::new())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreEventDestinationAmazonEventbridgeAwsEventSourceStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreEventDestinationAmazonEventbridgeAwsEventSourceStatus::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "active" => V2CoreEventDestinationAmazonEventbridgeAwsEventSourceStatus::Active,
            "deleted" => V2CoreEventDestinationAmazonEventbridgeAwsEventSourceStatus::Deleted,
            "pending" => V2CoreEventDestinationAmazonEventbridgeAwsEventSourceStatus::Pending,
            "unknown" => V2CoreEventDestinationAmazonEventbridgeAwsEventSourceStatus::UnknownValue,
            other => V2CoreEventDestinationAmazonEventbridgeAwsEventSourceStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreEventDestinationAzureEventGridAzurePartnerTopicStatus {
    Activated,
    Deleted,
    NeverActivated,
    UnknownValue,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreEventDestinationAzureEventGridAzurePartnerTopicStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreEventDestinationAzureEventGridAzurePartnerTopicStatus::Activated => "activated",
            V2CoreEventDestinationAzureEventGridAzurePartnerTopicStatus::Deleted => "deleted",
            V2CoreEventDestinationAzureEventGridAzurePartnerTopicStatus::NeverActivated => "never_activated",
            V2CoreEventDestinationAzureEventGridAzurePartnerTopicStatus::UnknownValue => "unknown",
            V2CoreEventDestinationAzureEventGridAzurePartnerTopicStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl Default for V2CoreEventDestinationAzureEventGridAzurePartnerTopicStatus {
    fn default() -> Self {
        V2CoreEventDestinationAzureEventGridAzurePartnerTopicStatus::Unknown(String::new())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreEventDestinationAzureEventGridAzurePartnerTopicStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreEventDestinationAzureEventGridAzurePartnerTopicStatus::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "activated" => V2CoreEventDestinationAzureEventGridAzurePartnerTopicStatus::Activated,
            "deleted" => V2CoreEventDestinationAzureEventGridAzurePartnerTopicStatus::Deleted,
            "never_activated" => V2CoreEventDestinationAzureEventGridAzurePartnerTopicStatus::NeverActivated,
            "unknown" => V2CoreEventDestinationAzureEventGridAzurePartnerTopicStatus::UnknownValue,
            other => V2CoreEventDestinationAzureEventGridAzurePartnerTopicStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreEventDestinationObject {
    V2CoreEventDestination,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreEventDestinationObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreEventDestinationObject::V2CoreEventDestination => "v2.core.event_destination",
            V2CoreEventDestinationObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl Default for V2CoreEventDestinationObject {
    fn default() -> Self {
        V2CoreEventDestinationObject::Unknown(String::new())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreEventDestinationObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreEventDestinationObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.core.event_destination" => V2CoreEventDestinationObject::V2CoreEventDestination,
            other => V2CoreEventDestinationObject::Unknown(other.to_string()),
        })
    }
}
