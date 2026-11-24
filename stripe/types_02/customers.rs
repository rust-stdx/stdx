#[cfg(feature = "customers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCustomerFundingInstructionParamsBankTransferRequestedAddressTypesItem {
    Iban,
    SortCode,
    Spei,
    Zengin,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "customers")]
impl CreateCustomerFundingInstructionParamsBankTransferRequestedAddressTypesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCustomerFundingInstructionParamsBankTransferRequestedAddressTypesItem::Iban => "iban",
            CreateCustomerFundingInstructionParamsBankTransferRequestedAddressTypesItem::SortCode => "sort_code",
            CreateCustomerFundingInstructionParamsBankTransferRequestedAddressTypesItem::Spei => "spei",
            CreateCustomerFundingInstructionParamsBankTransferRequestedAddressTypesItem::Zengin => "zengin",
            CreateCustomerFundingInstructionParamsBankTransferRequestedAddressTypesItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "customers")]
impl<'json> json::Decode<'json> for CreateCustomerFundingInstructionParamsBankTransferRequestedAddressTypesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCustomerFundingInstructionParamsBankTransferRequestedAddressTypesItem::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "iban" => CreateCustomerFundingInstructionParamsBankTransferRequestedAddressTypesItem::Iban,
            "sort_code" => CreateCustomerFundingInstructionParamsBankTransferRequestedAddressTypesItem::SortCode,
            "spei" => CreateCustomerFundingInstructionParamsBankTransferRequestedAddressTypesItem::Spei,
            "zengin" => CreateCustomerFundingInstructionParamsBankTransferRequestedAddressTypesItem::Zengin,
            other => {
                CreateCustomerFundingInstructionParamsBankTransferRequestedAddressTypesItem::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for CreateCustomerFundingInstructionParamsBankTransferRequestedAddressTypesItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "customers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCustomerFundingInstructionParamsBankTransferType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
    UsBankTransfer,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "customers")]
impl CreateCustomerFundingInstructionParamsBankTransferType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCustomerFundingInstructionParamsBankTransferType::EuBankTransfer => "eu_bank_transfer",
            CreateCustomerFundingInstructionParamsBankTransferType::GbBankTransfer => "gb_bank_transfer",
            CreateCustomerFundingInstructionParamsBankTransferType::JpBankTransfer => "jp_bank_transfer",
            CreateCustomerFundingInstructionParamsBankTransferType::MxBankTransfer => "mx_bank_transfer",
            CreateCustomerFundingInstructionParamsBankTransferType::UsBankTransfer => "us_bank_transfer",
            CreateCustomerFundingInstructionParamsBankTransferType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "customers")]
impl<'json> json::Decode<'json> for CreateCustomerFundingInstructionParamsBankTransferType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCustomerFundingInstructionParamsBankTransferType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "eu_bank_transfer" => CreateCustomerFundingInstructionParamsBankTransferType::EuBankTransfer,
            "gb_bank_transfer" => CreateCustomerFundingInstructionParamsBankTransferType::GbBankTransfer,
            "jp_bank_transfer" => CreateCustomerFundingInstructionParamsBankTransferType::JpBankTransfer,
            "mx_bank_transfer" => CreateCustomerFundingInstructionParamsBankTransferType::MxBankTransfer,
            "us_bank_transfer" => CreateCustomerFundingInstructionParamsBankTransferType::UsBankTransfer,
            other => CreateCustomerFundingInstructionParamsBankTransferType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for CreateCustomerFundingInstructionParamsBankTransferType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "customers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCustomerParamsTaxIdDataItemType {
    AdNrt,
    AeTrn,
    AlTin,
    AmTin,
    AoTin,
    ArCuit,
    AuAbn,
    AuArn,
    AwTin,
    AzTin,
    BaTin,
    BbTin,
    BdBin,
    BfIfu,
    BgUic,
    BhVat,
    BjIfu,
    BoTin,
    BrCnpj,
    BrCpf,
    BsTin,
    ByTin,
    CaBn,
    CaGstHst,
    CaPstBc,
    CaPstMb,
    CaPstSk,
    CaQst,
    CdNif,
    ChUid,
    ChVat,
    ClTin,
    CmNiu,
    CnTin,
    CoNit,
    CrTin,
    CvNif,
    DeStn,
    DoRcn,
    EcRuc,
    EgTin,
    EsCif,
    EtTin,
    EuOssVat,
    EuVat,
    FoVat,
    GbVat,
    GeVat,
    GiTin,
    GnNif,
    HkBr,
    HrOib,
    HuTin,
    IcNif,
    IdNpwp,
    IlVat,
    InGst,
    IsVat,
    ItCf,
    JpCn,
    JpRn,
    JpTrn,
    KePin,
    KgTin,
    KhTin,
    KrBrn,
    KzBin,
    LaTin,
    LiUid,
    LiVat,
    LkVat,
    MaVat,
    MdVat,
    MePib,
    MkVat,
    MrNif,
    MxRfc,
    MyFrp,
    MyItn,
    MySst,
    NgTin,
    NoVat,
    NoVoec,
    NpPan,
    NzGst,
    OmVat,
    PeRuc,
    PhTin,
    PlNip,
    PyRuc,
    RoTin,
    RsPib,
    RuInn,
    RuKpp,
    SaVat,
    SgGst,
    SgUen,
    SiTin,
    SnNinea,
    SrFin,
    SvNit,
    ThVat,
    TjTin,
    TrTin,
    TwVat,
    TzVat,
    UaVat,
    UgTin,
    UsEin,
    UyRuc,
    UzTin,
    UzVat,
    VeRif,
    VnTin,
    ZaVat,
    ZmTin,
    ZwTin,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "customers")]
impl CreateCustomerParamsTaxIdDataItemType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCustomerParamsTaxIdDataItemType::AdNrt => "ad_nrt",
            CreateCustomerParamsTaxIdDataItemType::AeTrn => "ae_trn",
            CreateCustomerParamsTaxIdDataItemType::AlTin => "al_tin",
            CreateCustomerParamsTaxIdDataItemType::AmTin => "am_tin",
            CreateCustomerParamsTaxIdDataItemType::AoTin => "ao_tin",
            CreateCustomerParamsTaxIdDataItemType::ArCuit => "ar_cuit",
            CreateCustomerParamsTaxIdDataItemType::AuAbn => "au_abn",
            CreateCustomerParamsTaxIdDataItemType::AuArn => "au_arn",
            CreateCustomerParamsTaxIdDataItemType::AwTin => "aw_tin",
            CreateCustomerParamsTaxIdDataItemType::AzTin => "az_tin",
            CreateCustomerParamsTaxIdDataItemType::BaTin => "ba_tin",
            CreateCustomerParamsTaxIdDataItemType::BbTin => "bb_tin",
            CreateCustomerParamsTaxIdDataItemType::BdBin => "bd_bin",
            CreateCustomerParamsTaxIdDataItemType::BfIfu => "bf_ifu",
            CreateCustomerParamsTaxIdDataItemType::BgUic => "bg_uic",
            CreateCustomerParamsTaxIdDataItemType::BhVat => "bh_vat",
            CreateCustomerParamsTaxIdDataItemType::BjIfu => "bj_ifu",
            CreateCustomerParamsTaxIdDataItemType::BoTin => "bo_tin",
            CreateCustomerParamsTaxIdDataItemType::BrCnpj => "br_cnpj",
            CreateCustomerParamsTaxIdDataItemType::BrCpf => "br_cpf",
            CreateCustomerParamsTaxIdDataItemType::BsTin => "bs_tin",
            CreateCustomerParamsTaxIdDataItemType::ByTin => "by_tin",
            CreateCustomerParamsTaxIdDataItemType::CaBn => "ca_bn",
            CreateCustomerParamsTaxIdDataItemType::CaGstHst => "ca_gst_hst",
            CreateCustomerParamsTaxIdDataItemType::CaPstBc => "ca_pst_bc",
            CreateCustomerParamsTaxIdDataItemType::CaPstMb => "ca_pst_mb",
            CreateCustomerParamsTaxIdDataItemType::CaPstSk => "ca_pst_sk",
            CreateCustomerParamsTaxIdDataItemType::CaQst => "ca_qst",
            CreateCustomerParamsTaxIdDataItemType::CdNif => "cd_nif",
            CreateCustomerParamsTaxIdDataItemType::ChUid => "ch_uid",
            CreateCustomerParamsTaxIdDataItemType::ChVat => "ch_vat",
            CreateCustomerParamsTaxIdDataItemType::ClTin => "cl_tin",
            CreateCustomerParamsTaxIdDataItemType::CmNiu => "cm_niu",
            CreateCustomerParamsTaxIdDataItemType::CnTin => "cn_tin",
            CreateCustomerParamsTaxIdDataItemType::CoNit => "co_nit",
            CreateCustomerParamsTaxIdDataItemType::CrTin => "cr_tin",
            CreateCustomerParamsTaxIdDataItemType::CvNif => "cv_nif",
            CreateCustomerParamsTaxIdDataItemType::DeStn => "de_stn",
            CreateCustomerParamsTaxIdDataItemType::DoRcn => "do_rcn",
            CreateCustomerParamsTaxIdDataItemType::EcRuc => "ec_ruc",
            CreateCustomerParamsTaxIdDataItemType::EgTin => "eg_tin",
            CreateCustomerParamsTaxIdDataItemType::EsCif => "es_cif",
            CreateCustomerParamsTaxIdDataItemType::EtTin => "et_tin",
            CreateCustomerParamsTaxIdDataItemType::EuOssVat => "eu_oss_vat",
            CreateCustomerParamsTaxIdDataItemType::EuVat => "eu_vat",
            CreateCustomerParamsTaxIdDataItemType::FoVat => "fo_vat",
            CreateCustomerParamsTaxIdDataItemType::GbVat => "gb_vat",
            CreateCustomerParamsTaxIdDataItemType::GeVat => "ge_vat",
            CreateCustomerParamsTaxIdDataItemType::GiTin => "gi_tin",
            CreateCustomerParamsTaxIdDataItemType::GnNif => "gn_nif",
            CreateCustomerParamsTaxIdDataItemType::HkBr => "hk_br",
            CreateCustomerParamsTaxIdDataItemType::HrOib => "hr_oib",
            CreateCustomerParamsTaxIdDataItemType::HuTin => "hu_tin",
            CreateCustomerParamsTaxIdDataItemType::IcNif => "ic_nif",
            CreateCustomerParamsTaxIdDataItemType::IdNpwp => "id_npwp",
            CreateCustomerParamsTaxIdDataItemType::IlVat => "il_vat",
            CreateCustomerParamsTaxIdDataItemType::InGst => "in_gst",
            CreateCustomerParamsTaxIdDataItemType::IsVat => "is_vat",
            CreateCustomerParamsTaxIdDataItemType::ItCf => "it_cf",
            CreateCustomerParamsTaxIdDataItemType::JpCn => "jp_cn",
            CreateCustomerParamsTaxIdDataItemType::JpRn => "jp_rn",
            CreateCustomerParamsTaxIdDataItemType::JpTrn => "jp_trn",
            CreateCustomerParamsTaxIdDataItemType::KePin => "ke_pin",
            CreateCustomerParamsTaxIdDataItemType::KgTin => "kg_tin",
            CreateCustomerParamsTaxIdDataItemType::KhTin => "kh_tin",
            CreateCustomerParamsTaxIdDataItemType::KrBrn => "kr_brn",
            CreateCustomerParamsTaxIdDataItemType::KzBin => "kz_bin",
            CreateCustomerParamsTaxIdDataItemType::LaTin => "la_tin",
            CreateCustomerParamsTaxIdDataItemType::LiUid => "li_uid",
            CreateCustomerParamsTaxIdDataItemType::LiVat => "li_vat",
            CreateCustomerParamsTaxIdDataItemType::LkVat => "lk_vat",
            CreateCustomerParamsTaxIdDataItemType::MaVat => "ma_vat",
            CreateCustomerParamsTaxIdDataItemType::MdVat => "md_vat",
            CreateCustomerParamsTaxIdDataItemType::MePib => "me_pib",
            CreateCustomerParamsTaxIdDataItemType::MkVat => "mk_vat",
            CreateCustomerParamsTaxIdDataItemType::MrNif => "mr_nif",
            CreateCustomerParamsTaxIdDataItemType::MxRfc => "mx_rfc",
            CreateCustomerParamsTaxIdDataItemType::MyFrp => "my_frp",
            CreateCustomerParamsTaxIdDataItemType::MyItn => "my_itn",
            CreateCustomerParamsTaxIdDataItemType::MySst => "my_sst",
            CreateCustomerParamsTaxIdDataItemType::NgTin => "ng_tin",
            CreateCustomerParamsTaxIdDataItemType::NoVat => "no_vat",
            CreateCustomerParamsTaxIdDataItemType::NoVoec => "no_voec",
            CreateCustomerParamsTaxIdDataItemType::NpPan => "np_pan",
            CreateCustomerParamsTaxIdDataItemType::NzGst => "nz_gst",
            CreateCustomerParamsTaxIdDataItemType::OmVat => "om_vat",
            CreateCustomerParamsTaxIdDataItemType::PeRuc => "pe_ruc",
            CreateCustomerParamsTaxIdDataItemType::PhTin => "ph_tin",
            CreateCustomerParamsTaxIdDataItemType::PlNip => "pl_nip",
            CreateCustomerParamsTaxIdDataItemType::PyRuc => "py_ruc",
            CreateCustomerParamsTaxIdDataItemType::RoTin => "ro_tin",
            CreateCustomerParamsTaxIdDataItemType::RsPib => "rs_pib",
            CreateCustomerParamsTaxIdDataItemType::RuInn => "ru_inn",
            CreateCustomerParamsTaxIdDataItemType::RuKpp => "ru_kpp",
            CreateCustomerParamsTaxIdDataItemType::SaVat => "sa_vat",
            CreateCustomerParamsTaxIdDataItemType::SgGst => "sg_gst",
            CreateCustomerParamsTaxIdDataItemType::SgUen => "sg_uen",
            CreateCustomerParamsTaxIdDataItemType::SiTin => "si_tin",
            CreateCustomerParamsTaxIdDataItemType::SnNinea => "sn_ninea",
            CreateCustomerParamsTaxIdDataItemType::SrFin => "sr_fin",
            CreateCustomerParamsTaxIdDataItemType::SvNit => "sv_nit",
            CreateCustomerParamsTaxIdDataItemType::ThVat => "th_vat",
            CreateCustomerParamsTaxIdDataItemType::TjTin => "tj_tin",
            CreateCustomerParamsTaxIdDataItemType::TrTin => "tr_tin",
            CreateCustomerParamsTaxIdDataItemType::TwVat => "tw_vat",
            CreateCustomerParamsTaxIdDataItemType::TzVat => "tz_vat",
            CreateCustomerParamsTaxIdDataItemType::UaVat => "ua_vat",
            CreateCustomerParamsTaxIdDataItemType::UgTin => "ug_tin",
            CreateCustomerParamsTaxIdDataItemType::UsEin => "us_ein",
            CreateCustomerParamsTaxIdDataItemType::UyRuc => "uy_ruc",
            CreateCustomerParamsTaxIdDataItemType::UzTin => "uz_tin",
            CreateCustomerParamsTaxIdDataItemType::UzVat => "uz_vat",
            CreateCustomerParamsTaxIdDataItemType::VeRif => "ve_rif",
            CreateCustomerParamsTaxIdDataItemType::VnTin => "vn_tin",
            CreateCustomerParamsTaxIdDataItemType::ZaVat => "za_vat",
            CreateCustomerParamsTaxIdDataItemType::ZmTin => "zm_tin",
            CreateCustomerParamsTaxIdDataItemType::ZwTin => "zw_tin",
            CreateCustomerParamsTaxIdDataItemType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "customers")]
impl<'json> json::Decode<'json> for CreateCustomerParamsTaxIdDataItemType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCustomerParamsTaxIdDataItemType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ad_nrt" => CreateCustomerParamsTaxIdDataItemType::AdNrt,
            "ae_trn" => CreateCustomerParamsTaxIdDataItemType::AeTrn,
            "al_tin" => CreateCustomerParamsTaxIdDataItemType::AlTin,
            "am_tin" => CreateCustomerParamsTaxIdDataItemType::AmTin,
            "ao_tin" => CreateCustomerParamsTaxIdDataItemType::AoTin,
            "ar_cuit" => CreateCustomerParamsTaxIdDataItemType::ArCuit,
            "au_abn" => CreateCustomerParamsTaxIdDataItemType::AuAbn,
            "au_arn" => CreateCustomerParamsTaxIdDataItemType::AuArn,
            "aw_tin" => CreateCustomerParamsTaxIdDataItemType::AwTin,
            "az_tin" => CreateCustomerParamsTaxIdDataItemType::AzTin,
            "ba_tin" => CreateCustomerParamsTaxIdDataItemType::BaTin,
            "bb_tin" => CreateCustomerParamsTaxIdDataItemType::BbTin,
            "bd_bin" => CreateCustomerParamsTaxIdDataItemType::BdBin,
            "bf_ifu" => CreateCustomerParamsTaxIdDataItemType::BfIfu,
            "bg_uic" => CreateCustomerParamsTaxIdDataItemType::BgUic,
            "bh_vat" => CreateCustomerParamsTaxIdDataItemType::BhVat,
            "bj_ifu" => CreateCustomerParamsTaxIdDataItemType::BjIfu,
            "bo_tin" => CreateCustomerParamsTaxIdDataItemType::BoTin,
            "br_cnpj" => CreateCustomerParamsTaxIdDataItemType::BrCnpj,
            "br_cpf" => CreateCustomerParamsTaxIdDataItemType::BrCpf,
            "bs_tin" => CreateCustomerParamsTaxIdDataItemType::BsTin,
            "by_tin" => CreateCustomerParamsTaxIdDataItemType::ByTin,
            "ca_bn" => CreateCustomerParamsTaxIdDataItemType::CaBn,
            "ca_gst_hst" => CreateCustomerParamsTaxIdDataItemType::CaGstHst,
            "ca_pst_bc" => CreateCustomerParamsTaxIdDataItemType::CaPstBc,
            "ca_pst_mb" => CreateCustomerParamsTaxIdDataItemType::CaPstMb,
            "ca_pst_sk" => CreateCustomerParamsTaxIdDataItemType::CaPstSk,
            "ca_qst" => CreateCustomerParamsTaxIdDataItemType::CaQst,
            "cd_nif" => CreateCustomerParamsTaxIdDataItemType::CdNif,
            "ch_uid" => CreateCustomerParamsTaxIdDataItemType::ChUid,
            "ch_vat" => CreateCustomerParamsTaxIdDataItemType::ChVat,
            "cl_tin" => CreateCustomerParamsTaxIdDataItemType::ClTin,
            "cm_niu" => CreateCustomerParamsTaxIdDataItemType::CmNiu,
            "cn_tin" => CreateCustomerParamsTaxIdDataItemType::CnTin,
            "co_nit" => CreateCustomerParamsTaxIdDataItemType::CoNit,
            "cr_tin" => CreateCustomerParamsTaxIdDataItemType::CrTin,
            "cv_nif" => CreateCustomerParamsTaxIdDataItemType::CvNif,
            "de_stn" => CreateCustomerParamsTaxIdDataItemType::DeStn,
            "do_rcn" => CreateCustomerParamsTaxIdDataItemType::DoRcn,
            "ec_ruc" => CreateCustomerParamsTaxIdDataItemType::EcRuc,
            "eg_tin" => CreateCustomerParamsTaxIdDataItemType::EgTin,
            "es_cif" => CreateCustomerParamsTaxIdDataItemType::EsCif,
            "et_tin" => CreateCustomerParamsTaxIdDataItemType::EtTin,
            "eu_oss_vat" => CreateCustomerParamsTaxIdDataItemType::EuOssVat,
            "eu_vat" => CreateCustomerParamsTaxIdDataItemType::EuVat,
            "fo_vat" => CreateCustomerParamsTaxIdDataItemType::FoVat,
            "gb_vat" => CreateCustomerParamsTaxIdDataItemType::GbVat,
            "ge_vat" => CreateCustomerParamsTaxIdDataItemType::GeVat,
            "gi_tin" => CreateCustomerParamsTaxIdDataItemType::GiTin,
            "gn_nif" => CreateCustomerParamsTaxIdDataItemType::GnNif,
            "hk_br" => CreateCustomerParamsTaxIdDataItemType::HkBr,
            "hr_oib" => CreateCustomerParamsTaxIdDataItemType::HrOib,
            "hu_tin" => CreateCustomerParamsTaxIdDataItemType::HuTin,
            "ic_nif" => CreateCustomerParamsTaxIdDataItemType::IcNif,
            "id_npwp" => CreateCustomerParamsTaxIdDataItemType::IdNpwp,
            "il_vat" => CreateCustomerParamsTaxIdDataItemType::IlVat,
            "in_gst" => CreateCustomerParamsTaxIdDataItemType::InGst,
            "is_vat" => CreateCustomerParamsTaxIdDataItemType::IsVat,
            "it_cf" => CreateCustomerParamsTaxIdDataItemType::ItCf,
            "jp_cn" => CreateCustomerParamsTaxIdDataItemType::JpCn,
            "jp_rn" => CreateCustomerParamsTaxIdDataItemType::JpRn,
            "jp_trn" => CreateCustomerParamsTaxIdDataItemType::JpTrn,
            "ke_pin" => CreateCustomerParamsTaxIdDataItemType::KePin,
            "kg_tin" => CreateCustomerParamsTaxIdDataItemType::KgTin,
            "kh_tin" => CreateCustomerParamsTaxIdDataItemType::KhTin,
            "kr_brn" => CreateCustomerParamsTaxIdDataItemType::KrBrn,
            "kz_bin" => CreateCustomerParamsTaxIdDataItemType::KzBin,
            "la_tin" => CreateCustomerParamsTaxIdDataItemType::LaTin,
            "li_uid" => CreateCustomerParamsTaxIdDataItemType::LiUid,
            "li_vat" => CreateCustomerParamsTaxIdDataItemType::LiVat,
            "lk_vat" => CreateCustomerParamsTaxIdDataItemType::LkVat,
            "ma_vat" => CreateCustomerParamsTaxIdDataItemType::MaVat,
            "md_vat" => CreateCustomerParamsTaxIdDataItemType::MdVat,
            "me_pib" => CreateCustomerParamsTaxIdDataItemType::MePib,
            "mk_vat" => CreateCustomerParamsTaxIdDataItemType::MkVat,
            "mr_nif" => CreateCustomerParamsTaxIdDataItemType::MrNif,
            "mx_rfc" => CreateCustomerParamsTaxIdDataItemType::MxRfc,
            "my_frp" => CreateCustomerParamsTaxIdDataItemType::MyFrp,
            "my_itn" => CreateCustomerParamsTaxIdDataItemType::MyItn,
            "my_sst" => CreateCustomerParamsTaxIdDataItemType::MySst,
            "ng_tin" => CreateCustomerParamsTaxIdDataItemType::NgTin,
            "no_vat" => CreateCustomerParamsTaxIdDataItemType::NoVat,
            "no_voec" => CreateCustomerParamsTaxIdDataItemType::NoVoec,
            "np_pan" => CreateCustomerParamsTaxIdDataItemType::NpPan,
            "nz_gst" => CreateCustomerParamsTaxIdDataItemType::NzGst,
            "om_vat" => CreateCustomerParamsTaxIdDataItemType::OmVat,
            "pe_ruc" => CreateCustomerParamsTaxIdDataItemType::PeRuc,
            "ph_tin" => CreateCustomerParamsTaxIdDataItemType::PhTin,
            "pl_nip" => CreateCustomerParamsTaxIdDataItemType::PlNip,
            "py_ruc" => CreateCustomerParamsTaxIdDataItemType::PyRuc,
            "ro_tin" => CreateCustomerParamsTaxIdDataItemType::RoTin,
            "rs_pib" => CreateCustomerParamsTaxIdDataItemType::RsPib,
            "ru_inn" => CreateCustomerParamsTaxIdDataItemType::RuInn,
            "ru_kpp" => CreateCustomerParamsTaxIdDataItemType::RuKpp,
            "sa_vat" => CreateCustomerParamsTaxIdDataItemType::SaVat,
            "sg_gst" => CreateCustomerParamsTaxIdDataItemType::SgGst,
            "sg_uen" => CreateCustomerParamsTaxIdDataItemType::SgUen,
            "si_tin" => CreateCustomerParamsTaxIdDataItemType::SiTin,
            "sn_ninea" => CreateCustomerParamsTaxIdDataItemType::SnNinea,
            "sr_fin" => CreateCustomerParamsTaxIdDataItemType::SrFin,
            "sv_nit" => CreateCustomerParamsTaxIdDataItemType::SvNit,
            "th_vat" => CreateCustomerParamsTaxIdDataItemType::ThVat,
            "tj_tin" => CreateCustomerParamsTaxIdDataItemType::TjTin,
            "tr_tin" => CreateCustomerParamsTaxIdDataItemType::TrTin,
            "tw_vat" => CreateCustomerParamsTaxIdDataItemType::TwVat,
            "tz_vat" => CreateCustomerParamsTaxIdDataItemType::TzVat,
            "ua_vat" => CreateCustomerParamsTaxIdDataItemType::UaVat,
            "ug_tin" => CreateCustomerParamsTaxIdDataItemType::UgTin,
            "us_ein" => CreateCustomerParamsTaxIdDataItemType::UsEin,
            "uy_ruc" => CreateCustomerParamsTaxIdDataItemType::UyRuc,
            "uz_tin" => CreateCustomerParamsTaxIdDataItemType::UzTin,
            "uz_vat" => CreateCustomerParamsTaxIdDataItemType::UzVat,
            "ve_rif" => CreateCustomerParamsTaxIdDataItemType::VeRif,
            "vn_tin" => CreateCustomerParamsTaxIdDataItemType::VnTin,
            "za_vat" => CreateCustomerParamsTaxIdDataItemType::ZaVat,
            "zm_tin" => CreateCustomerParamsTaxIdDataItemType::ZmTin,
            "zw_tin" => CreateCustomerParamsTaxIdDataItemType::ZwTin,
            other => CreateCustomerParamsTaxIdDataItemType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for CreateCustomerParamsTaxIdDataItemType {
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
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceJpBankTransfer
{
    #[doc = "The name of the bank of the sender of the funding."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sender_bank: Option<String>,
    #[doc = "The name of the bank branch of the sender of the funding."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sender_branch: Option<String>,
    #[doc = "The full name of the sender, as supplied by the sending bank."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
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
pub enum CustomerObject {
    Customer,
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
impl CustomerObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CustomerObject::Customer => "customer",
            CustomerObject::Unknown(value) => value,
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
impl Default for CustomerObject {
    fn default() -> Self {
        CustomerObject::Unknown(String::new())
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
impl<'json> json::Decode<'json> for CustomerObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CustomerObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "customer" => CustomerObject::Customer,
            other => CustomerObject::Unknown(other.to_string()),
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
pub enum CustomerTaxExempt {
    Exempt,
    None,
    Reverse,
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
impl CustomerTaxExempt {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CustomerTaxExempt::Exempt => "exempt",
            CustomerTaxExempt::None => "none",
            CustomerTaxExempt::Reverse => "reverse",
            CustomerTaxExempt::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for CustomerTaxExempt {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CustomerTaxExempt::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "exempt" => CustomerTaxExempt::Exempt,
            "none" => CustomerTaxExempt::None,
            "reverse" => CustomerTaxExempt::Reverse,
            other => CustomerTaxExempt::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "customers")]
#[derive(Debug, Clone, Default)]
pub struct VerifyCustomerSourceParams {
    #[doc = "Two positive integers, in *cents*, equal to the values of the microdeposits sent to the bank account."]
    pub amounts: Option<Vec<i64>>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "customers")]
impl crate::params::FormEncode for VerifyCustomerSourceParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.amounts.as_ref() {
            form.field_array("amounts", values.as_slice());
        }
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}
