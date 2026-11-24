#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCoreAccountLinkParamsUseCaseAccountOnboardingCollectionOptionsFields {
    CurrentlyDue,
    EventuallyDue,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl CreateCoreAccountLinkParamsUseCaseAccountOnboardingCollectionOptionsFields {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCoreAccountLinkParamsUseCaseAccountOnboardingCollectionOptionsFields::CurrentlyDue => "currently_due",
            CreateCoreAccountLinkParamsUseCaseAccountOnboardingCollectionOptionsFields::EventuallyDue => {
                "eventually_due"
            }
            CreateCoreAccountLinkParamsUseCaseAccountOnboardingCollectionOptionsFields::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl json::Encode for CreateCoreAccountLinkParamsUseCaseAccountOnboardingCollectionOptionsFields {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for CreateCoreAccountLinkParamsUseCaseAccountOnboardingCollectionOptionsFields {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCoreAccountLinkParamsUseCaseAccountOnboardingCollectionOptionsFields::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "currently_due" => CreateCoreAccountLinkParamsUseCaseAccountOnboardingCollectionOptionsFields::CurrentlyDue,
            "eventually_due" => {
                CreateCoreAccountLinkParamsUseCaseAccountOnboardingCollectionOptionsFields::EventuallyDue
            }
            other => {
                CreateCoreAccountLinkParamsUseCaseAccountOnboardingCollectionOptionsFields::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCoreAccountParamsConfigurationCustomerAutomaticIndirectTaxExempt {
    Exempt,
    None,
    Reverse,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl CreateCoreAccountParamsConfigurationCustomerAutomaticIndirectTaxExempt {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCoreAccountParamsConfigurationCustomerAutomaticIndirectTaxExempt::Exempt => "exempt",
            CreateCoreAccountParamsConfigurationCustomerAutomaticIndirectTaxExempt::None => "none",
            CreateCoreAccountParamsConfigurationCustomerAutomaticIndirectTaxExempt::Reverse => "reverse",
            CreateCoreAccountParamsConfigurationCustomerAutomaticIndirectTaxExempt::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl json::Encode for CreateCoreAccountParamsConfigurationCustomerAutomaticIndirectTaxExempt {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for CreateCoreAccountParamsConfigurationCustomerAutomaticIndirectTaxExempt {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCoreAccountParamsConfigurationCustomerAutomaticIndirectTaxExempt::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "exempt" => CreateCoreAccountParamsConfigurationCustomerAutomaticIndirectTaxExempt::Exempt,
            "none" => CreateCoreAccountParamsConfigurationCustomerAutomaticIndirectTaxExempt::None,
            "reverse" => CreateCoreAccountParamsConfigurationCustomerAutomaticIndirectTaxExempt::Reverse,
            other => CreateCoreAccountParamsConfigurationCustomerAutomaticIndirectTaxExempt::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsConfigurationCustomerBillingInvoiceCustomFieldsItem {
    #[doc = "The name of the custom field."]
    pub name: String,
    #[doc = "The value of the custom field."]
    pub value: String,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCoreAccountParamsConfigurationCustomerBillingInvoiceRenderingAmountTaxDisplay {
    ExcludeTax,
    IncludeInclusiveTax,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl CreateCoreAccountParamsConfigurationCustomerBillingInvoiceRenderingAmountTaxDisplay {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCoreAccountParamsConfigurationCustomerBillingInvoiceRenderingAmountTaxDisplay::ExcludeTax => "exclude_tax",
            CreateCoreAccountParamsConfigurationCustomerBillingInvoiceRenderingAmountTaxDisplay::IncludeInclusiveTax => "include_inclusive_tax",
            CreateCoreAccountParamsConfigurationCustomerBillingInvoiceRenderingAmountTaxDisplay::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl json::Encode for CreateCoreAccountParamsConfigurationCustomerBillingInvoiceRenderingAmountTaxDisplay {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json>
    for CreateCoreAccountParamsConfigurationCustomerBillingInvoiceRenderingAmountTaxDisplay
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCoreAccountParamsConfigurationCustomerBillingInvoiceRenderingAmountTaxDisplay::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "exclude_tax" => {
                CreateCoreAccountParamsConfigurationCustomerBillingInvoiceRenderingAmountTaxDisplay::ExcludeTax
            }
            "include_inclusive_tax" => {
                CreateCoreAccountParamsConfigurationCustomerBillingInvoiceRenderingAmountTaxDisplay::IncludeInclusiveTax
            }
            other => CreateCoreAccountParamsConfigurationCustomerBillingInvoiceRenderingAmountTaxDisplay::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsConfigurationMerchantBranding {
    #[doc = "ID of a [file upload](https://docs.stripe.com/api/persons/update#create_file): An icon for the merchant."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[doc = "ID of a [file upload](https://docs.stripe.com/api/persons/update#create_file): A logo for the merchant that will be u..."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[doc = "A CSS hex color value representing the primary branding color for the merchant."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub primary_color: Option<String>,
    #[doc = "A CSS hex color value representing the secondary branding color for the merchant."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub secondary_color: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsConfigurationMerchantCardPaymentsDeclineOn {
    #[doc = "Whether Stripe automatically declines charges with an incorrect ZIP or postal code."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub avs_failure: Option<bool>,
    #[doc = "Whether Stripe automatically declines charges with an incorrect CVC."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub cvc_failure: Option<bool>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsConfigurationMerchantScriptStatementDescriptorKana {
    #[doc = "The default text that appears on statements for non-card charges outside of Japan."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<String>,
    #[doc = "Default text that appears on statements for card charges outside of Japan, prefixing any dynamic statement_descriptor..."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCoreAccountParamsDefaultsResponsibilitiesLossesCollector {
    Application,
    Stripe,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl CreateCoreAccountParamsDefaultsResponsibilitiesLossesCollector {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCoreAccountParamsDefaultsResponsibilitiesLossesCollector::Application => "application",
            CreateCoreAccountParamsDefaultsResponsibilitiesLossesCollector::Stripe => "stripe",
            CreateCoreAccountParamsDefaultsResponsibilitiesLossesCollector::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl json::Encode for CreateCoreAccountParamsDefaultsResponsibilitiesLossesCollector {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for CreateCoreAccountParamsDefaultsResponsibilitiesLossesCollector {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCoreAccountParamsDefaultsResponsibilitiesLossesCollector::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "application" => CreateCoreAccountParamsDefaultsResponsibilitiesLossesCollector::Application,
            "stripe" => CreateCoreAccountParamsDefaultsResponsibilitiesLossesCollector::Stripe,
            other => CreateCoreAccountParamsDefaultsResponsibilitiesLossesCollector::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCoreAccountTokenParamsIdentityIndividualAdditionalNamesItemPurpose {
    Alias,
    Maiden,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl CreateCoreAccountTokenParamsIdentityIndividualAdditionalNamesItemPurpose {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCoreAccountTokenParamsIdentityIndividualAdditionalNamesItemPurpose::Alias => "alias",
            CreateCoreAccountTokenParamsIdentityIndividualAdditionalNamesItemPurpose::Maiden => "maiden",
            CreateCoreAccountTokenParamsIdentityIndividualAdditionalNamesItemPurpose::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl json::Encode for CreateCoreAccountTokenParamsIdentityIndividualAdditionalNamesItemPurpose {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for CreateCoreAccountTokenParamsIdentityIndividualAdditionalNamesItemPurpose {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCoreAccountTokenParamsIdentityIndividualAdditionalNamesItemPurpose::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "alias" => CreateCoreAccountTokenParamsIdentityIndividualAdditionalNamesItemPurpose::Alias,
            "maiden" => CreateCoreAccountTokenParamsIdentityIndividualAdditionalNamesItemPurpose::Maiden,
            other => {
                CreateCoreAccountTokenParamsIdentityIndividualAdditionalNamesItemPurpose::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType {
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
impl CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::AeEid => "ae_eid",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::AoNif => "ao_nif",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::ArCuil => "ar_cuil",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::ArDni => "ar_dni",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::AtStn => "at_stn",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::AzTin => "az_tin",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::BdBrc => "bd_brc",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::BdEtin => "bd_etin",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::BdNid => "bd_nid",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::BeNrn => "be_nrn",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::BgUcn => "bg_ucn",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::BnNric => "bn_nric",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::BrCpf => "br_cpf",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::CaSin => "ca_sin",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::ChOasi => "ch_oasi",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::ClRut => "cl_rut",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::CnPp => "cn_pp",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::CoNuip => "co_nuip",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::CrCi => "cr_ci",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::CrCpf => "cr_cpf",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::CrDimex => "cr_dimex",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::CrNite => "cr_nite",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::CyTic => "cy_tic",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::CzRc => "cz_rc",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::DeStn => "de_stn",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::DkCpr => "dk_cpr",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::DoCie => "do_cie",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::DoRcn => "do_rcn",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::EcCi => "ec_ci",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::EeIk => "ee_ik",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::EsNif => "es_nif",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::FiHetu => "fi_hetu",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::FrNir => "fr_nir",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::GbNino => "gb_nino",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::GrAfm => "gr_afm",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::GtNit => "gt_nit",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::HkId => "hk_id",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::HrOib => "hr_oib",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::HuAd => "hu_ad",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::IdNik => "id_nik",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::IePpsn => "ie_ppsn",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::IsKt => "is_kt",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::ItCf => "it_cf",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::JpInc => "jp_inc",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::KePin => "ke_pin",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::KzIin => "kz_iin",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::LiPeid => "li_peid",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::LtAk => "lt_ak",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::LuNif => "lu_nif",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::LvPk => "lv_pk",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::MxRfc => "mx_rfc",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::MyNric => "my_nric",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::MzNuit => "mz_nuit",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::NgNin => "ng_nin",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::NlBsn => "nl_bsn",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::NoNin => "no_nin",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::NzIrd => "nz_ird",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::PeDni => "pe_dni",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::PkCnic => "pk_cnic",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::PkSnic => "pk_snic",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::PlPesel => "pl_pesel",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::PtNif => "pt_nif",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::RoCnp => "ro_cnp",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::SaTin => "sa_tin",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::SePin => "se_pin",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::SgFin => "sg_fin",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::SgNric => "sg_nric",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::SkDic => "sk_dic",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::ThLc => "th_lc",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::ThPin => "th_pin",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::TrTin => "tr_tin",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::UsItin => "us_itin",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::UsItinLast4 => "us_itin_last_4",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::UsSsn => "us_ssn",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::UsSsnLast4 => "us_ssn_last_4",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::UyDni => "uy_dni",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::ZaId => "za_id",
            CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl json::Encode for CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ae_eid" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::AeEid,
            "ao_nif" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::AoNif,
            "ar_cuil" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::ArCuil,
            "ar_dni" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::ArDni,
            "at_stn" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::AtStn,
            "az_tin" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::AzTin,
            "bd_brc" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::BdBrc,
            "bd_etin" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::BdEtin,
            "bd_nid" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::BdNid,
            "be_nrn" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::BeNrn,
            "bg_ucn" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::BgUcn,
            "bn_nric" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::BnNric,
            "br_cpf" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::BrCpf,
            "ca_sin" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::CaSin,
            "ch_oasi" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::ChOasi,
            "cl_rut" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::ClRut,
            "cn_pp" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::CnPp,
            "co_nuip" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::CoNuip,
            "cr_ci" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::CrCi,
            "cr_cpf" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::CrCpf,
            "cr_dimex" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::CrDimex,
            "cr_nite" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::CrNite,
            "cy_tic" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::CyTic,
            "cz_rc" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::CzRc,
            "de_stn" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::DeStn,
            "dk_cpr" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::DkCpr,
            "do_cie" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::DoCie,
            "do_rcn" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::DoRcn,
            "ec_ci" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::EcCi,
            "ee_ik" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::EeIk,
            "es_nif" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::EsNif,
            "fi_hetu" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::FiHetu,
            "fr_nir" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::FrNir,
            "gb_nino" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::GbNino,
            "gr_afm" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::GrAfm,
            "gt_nit" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::GtNit,
            "hk_id" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::HkId,
            "hr_oib" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::HrOib,
            "hu_ad" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::HuAd,
            "id_nik" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::IdNik,
            "ie_ppsn" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::IePpsn,
            "is_kt" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::IsKt,
            "it_cf" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::ItCf,
            "jp_inc" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::JpInc,
            "ke_pin" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::KePin,
            "kz_iin" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::KzIin,
            "li_peid" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::LiPeid,
            "lt_ak" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::LtAk,
            "lu_nif" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::LuNif,
            "lv_pk" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::LvPk,
            "mx_rfc" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::MxRfc,
            "my_nric" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::MyNric,
            "mz_nuit" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::MzNuit,
            "ng_nin" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::NgNin,
            "nl_bsn" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::NlBsn,
            "no_nin" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::NoNin,
            "nz_ird" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::NzIrd,
            "pe_dni" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::PeDni,
            "pk_cnic" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::PkCnic,
            "pk_snic" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::PkSnic,
            "pl_pesel" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::PlPesel,
            "pt_nif" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::PtNif,
            "ro_cnp" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::RoCnp,
            "sa_tin" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::SaTin,
            "se_pin" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::SePin,
            "sg_fin" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::SgFin,
            "sg_nric" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::SgNric,
            "sk_dic" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::SkDic,
            "th_lc" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::ThLc,
            "th_pin" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::ThPin,
            "tr_tin" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::TrTin,
            "us_itin" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::UsItin,
            "us_itin_last_4" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::UsItinLast4,
            "us_ssn" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::UsSsn,
            "us_ssn_last_4" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::UsSsnLast4,
            "uy_dni" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::UyDni,
            "za_id" => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::ZaId,
            other => CreateCoreAccountTokenParamsIdentityIndividualIdNumbersItemType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreEventDestinationParamsAzureEventGrid {
    #[doc = "The Azure region where Stripe creates the partner topic."]
    pub azure_region: String,
    #[doc = "The Azure resource group where Stripe creates the partner topic."]
    pub azure_resource_group_name: String,
    #[doc = "The Azure subscription where Stripe creates the partner topic."]
    pub azure_subscription_id: String,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCoreEventDestinationParamsIncludeItem {
    WebhookEndpointSigningSecret,
    WebhookEndpointUrl,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl CreateCoreEventDestinationParamsIncludeItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCoreEventDestinationParamsIncludeItem::WebhookEndpointSigningSecret => {
                "webhook_endpoint.signing_secret"
            }
            CreateCoreEventDestinationParamsIncludeItem::WebhookEndpointUrl => "webhook_endpoint.url",
            CreateCoreEventDestinationParamsIncludeItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl json::Encode for CreateCoreEventDestinationParamsIncludeItem {
    fn encode<W: json::Write>(&self, encoder: &mut json::Encoder<W>) -> Result<(), W::Error> {
        encoder.string(self.as_str())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for CreateCoreEventDestinationParamsIncludeItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCoreEventDestinationParamsIncludeItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "webhook_endpoint.signing_secret" => {
                CreateCoreEventDestinationParamsIncludeItem::WebhookEndpointSigningSecret
            }
            "webhook_endpoint.url" => CreateCoreEventDestinationParamsIncludeItem::WebhookEndpointUrl,
            other => CreateCoreEventDestinationParamsIncludeItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreEventDestinationParamsWebhookEndpoint {
    #[doc = "The URL where Stripe sends matching events."]
    pub url: String,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum GetCoreAccountParamsIncludeItem {
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
impl GetCoreAccountParamsIncludeItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            GetCoreAccountParamsIncludeItem::ConfigurationCustomer => "configuration.customer",
            GetCoreAccountParamsIncludeItem::ConfigurationMerchant => "configuration.merchant",
            GetCoreAccountParamsIncludeItem::ConfigurationRecipient => "configuration.recipient",
            GetCoreAccountParamsIncludeItem::Defaults => "defaults",
            GetCoreAccountParamsIncludeItem::FutureRequirements => "future_requirements",
            GetCoreAccountParamsIncludeItem::Identity => "identity",
            GetCoreAccountParamsIncludeItem::Requirements => "requirements",
            GetCoreAccountParamsIncludeItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for GetCoreAccountParamsIncludeItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(GetCoreAccountParamsIncludeItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "configuration.customer" => GetCoreAccountParamsIncludeItem::ConfigurationCustomer,
            "configuration.merchant" => GetCoreAccountParamsIncludeItem::ConfigurationMerchant,
            "configuration.recipient" => GetCoreAccountParamsIncludeItem::ConfigurationRecipient,
            "defaults" => GetCoreAccountParamsIncludeItem::Defaults,
            "future_requirements" => GetCoreAccountParamsIncludeItem::FutureRequirements,
            "identity" => GetCoreAccountParamsIncludeItem::Identity,
            "requirements" => GetCoreAccountParamsIncludeItem::Requirements,
            other => GetCoreAccountParamsIncludeItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
impl crate::params::FormEncode for GetCoreAccountParamsIncludeItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountAppliedConfigurationsItem {
    Customer,
    Merchant,
    Recipient,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountAppliedConfigurationsItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountAppliedConfigurationsItem::Customer => "customer",
            V2CoreAccountAppliedConfigurationsItem::Merchant => "merchant",
            V2CoreAccountAppliedConfigurationsItem::Recipient => "recipient",
            V2CoreAccountAppliedConfigurationsItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl Default for V2CoreAccountAppliedConfigurationsItem {
    fn default() -> Self {
        V2CoreAccountAppliedConfigurationsItem::Unknown(String::new())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreAccountAppliedConfigurationsItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountAppliedConfigurationsItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "customer" => V2CoreAccountAppliedConfigurationsItem::Customer,
            "merchant" => V2CoreAccountAppliedConfigurationsItem::Merchant,
            "recipient" => V2CoreAccountAppliedConfigurationsItem::Recipient,
            other => V2CoreAccountAppliedConfigurationsItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationCustomerShippingAddress {
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
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountDashboard {
    Express,
    Full,
    None,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountDashboard {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountDashboard::Express => "express",
            V2CoreAccountDashboard::Full => "full",
            V2CoreAccountDashboard::None => "none",
            V2CoreAccountDashboard::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreAccountDashboard {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountDashboard::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "express" => V2CoreAccountDashboard::Express,
            "full" => V2CoreAccountDashboard::Full,
            "none" => V2CoreAccountDashboard::None,
            other => V2CoreAccountDashboard::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountDefaultsProfile {
    #[doc = "The business's publicly-available website."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub business_url: Option<String>,
    #[doc = "The customer-facing business name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub doing_business_as: Option<String>,
    #[doc = "Internal-only description of the product sold or service provided by the business."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountDefaultsResponsibilitiesFeesCollector {
    Application,
    ApplicationCustom,
    ApplicationExpress,
    Stripe,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountDefaultsResponsibilitiesFeesCollector {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountDefaultsResponsibilitiesFeesCollector::Application => "application",
            V2CoreAccountDefaultsResponsibilitiesFeesCollector::ApplicationCustom => "application_custom",
            V2CoreAccountDefaultsResponsibilitiesFeesCollector::ApplicationExpress => "application_express",
            V2CoreAccountDefaultsResponsibilitiesFeesCollector::Stripe => "stripe",
            V2CoreAccountDefaultsResponsibilitiesFeesCollector::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreAccountDefaultsResponsibilitiesFeesCollector {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountDefaultsResponsibilitiesFeesCollector::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "application" => V2CoreAccountDefaultsResponsibilitiesFeesCollector::Application,
            "application_custom" => V2CoreAccountDefaultsResponsibilitiesFeesCollector::ApplicationCustom,
            "application_express" => V2CoreAccountDefaultsResponsibilitiesFeesCollector::ApplicationExpress,
            "stripe" => V2CoreAccountDefaultsResponsibilitiesFeesCollector::Stripe,
            other => V2CoreAccountDefaultsResponsibilitiesFeesCollector::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountFutureRequirementsEntriesItemReferenceType {
    Inquiry,
    PaymentMethod,
    Person,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountFutureRequirementsEntriesItemReferenceType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountFutureRequirementsEntriesItemReferenceType::Inquiry => "inquiry",
            V2CoreAccountFutureRequirementsEntriesItemReferenceType::PaymentMethod => "payment_method",
            V2CoreAccountFutureRequirementsEntriesItemReferenceType::Person => "person",
            V2CoreAccountFutureRequirementsEntriesItemReferenceType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl Default for V2CoreAccountFutureRequirementsEntriesItemReferenceType {
    fn default() -> Self {
        V2CoreAccountFutureRequirementsEntriesItemReferenceType::Unknown(String::new())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreAccountFutureRequirementsEntriesItemReferenceType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountFutureRequirementsEntriesItemReferenceType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "inquiry" => V2CoreAccountFutureRequirementsEntriesItemReferenceType::Inquiry,
            "payment_method" => V2CoreAccountFutureRequirementsEntriesItemReferenceType::PaymentMethod,
            "person" => V2CoreAccountFutureRequirementsEntriesItemReferenceType::Person,
            other => V2CoreAccountFutureRequirementsEntriesItemReferenceType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountFutureRequirementsEntriesItemRequestedReasonsItemCode {
    RoutineOnboarding,
    RoutineVerification,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountFutureRequirementsEntriesItemRequestedReasonsItemCode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountFutureRequirementsEntriesItemRequestedReasonsItemCode::RoutineOnboarding => {
                "routine_onboarding"
            }
            V2CoreAccountFutureRequirementsEntriesItemRequestedReasonsItemCode::RoutineVerification => {
                "routine_verification"
            }
            V2CoreAccountFutureRequirementsEntriesItemRequestedReasonsItemCode::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl Default for V2CoreAccountFutureRequirementsEntriesItemRequestedReasonsItemCode {
    fn default() -> Self {
        V2CoreAccountFutureRequirementsEntriesItemRequestedReasonsItemCode::Unknown(String::new())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreAccountFutureRequirementsEntriesItemRequestedReasonsItemCode {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountFutureRequirementsEntriesItemRequestedReasonsItemCode::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "routine_onboarding" => {
                V2CoreAccountFutureRequirementsEntriesItemRequestedReasonsItemCode::RoutineOnboarding
            }
            "routine_verification" => {
                V2CoreAccountFutureRequirementsEntriesItemRequestedReasonsItemCode::RoutineVerification
            }
            other => V2CoreAccountFutureRequirementsEntriesItemRequestedReasonsItemCode::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountIdentityBusinessDetailsAnnualRevenueAmount {
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "A non-negative integer representing how much to charge in the smallest currency unit..."]
    #[json(default)]
    pub value: i64,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountIdentityBusinessDetailsDocumentsProofOfRegistrationSigner {
    #[doc = "Person signing the document."]
    #[json(default)]
    pub person: String,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountIdentityBusinessDetailsRegistrationDate {
    #[doc = "The day of registration, between 1 and 31."]
    #[json(default)]
    pub day: i64,
    #[doc = "The month of registration, between 1 and 12."]
    #[json(default)]
    pub month: i64,
    #[doc = "The four-digit year of registration."]
    #[json(default)]
    pub year: i64,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountObject {
    V2CoreAccount,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountObject::V2CoreAccount => "v2.core.account",
            V2CoreAccountObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl Default for V2CoreAccountObject {
    fn default() -> Self {
        V2CoreAccountObject::Unknown(String::new())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreAccountObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.core.account" => V2CoreAccountObject::V2CoreAccount,
            other => V2CoreAccountObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountPersonTokenObject {
    V2CoreAccountPersonToken,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreAccountPersonTokenObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountPersonTokenObject::V2CoreAccountPersonToken => "v2.core.account_person_token",
            V2CoreAccountPersonTokenObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl Default for V2CoreAccountPersonTokenObject {
    fn default() -> Self {
        V2CoreAccountPersonTokenObject::Unknown(String::new())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreAccountPersonTokenObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountPersonTokenObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.core.account_person_token" => V2CoreAccountPersonTokenObject::V2CoreAccountPersonToken,
            other => V2CoreAccountPersonTokenObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreEventDestinationStatusDetailsDisabledReason {
    NoAwsEventSourceExists,
    NoAzurePartnerTopicExists,
    User,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "v2_core")]
impl V2CoreEventDestinationStatusDetailsDisabledReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreEventDestinationStatusDetailsDisabledReason::NoAwsEventSourceExists => "no_aws_event_source_exists",
            V2CoreEventDestinationStatusDetailsDisabledReason::NoAzurePartnerTopicExists => {
                "no_azure_partner_topic_exists"
            }
            V2CoreEventDestinationStatusDetailsDisabledReason::User => "user",
            V2CoreEventDestinationStatusDetailsDisabledReason::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "v2_core")]
impl Default for V2CoreEventDestinationStatusDetailsDisabledReason {
    fn default() -> Self {
        V2CoreEventDestinationStatusDetailsDisabledReason::Unknown(String::new())
    }
}

#[cfg(feature = "v2_core")]
impl<'json> json::Decode<'json> for V2CoreEventDestinationStatusDetailsDisabledReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreEventDestinationStatusDetailsDisabledReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "no_aws_event_source_exists" => V2CoreEventDestinationStatusDetailsDisabledReason::NoAwsEventSourceExists,
            "no_azure_partner_topic_exists" => {
                V2CoreEventDestinationStatusDetailsDisabledReason::NoAzurePartnerTopicExists
            }
            "user" => V2CoreEventDestinationStatusDetailsDisabledReason::User,
            other => V2CoreEventDestinationStatusDetailsDisabledReason::Unknown(other.to_string()),
        })
    }
}
