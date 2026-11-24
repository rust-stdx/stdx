#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateReversalParamsLineItemsItem {
    #[doc = "The amount to reverse, in the [smallest currency unit](https://docs.stripe.com/currencies#minor-units) in negative."]
    pub amount: i64,
    #[doc = "The amount of tax to reverse, in the [smallest currency unit](https://docs.stripe.com/currencies#minor-units) in nega..."]
    pub amount_tax: i64,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The `id` of the line item to reverse in the original transaction."]
    pub original_line_item: String,
    #[doc = "The quantity reversed."]
    pub quantity: Option<i64>,
    #[doc = "A custom identifier for this line item in the reversal transaction, such as 'L1-refund'."]
    pub reference: String,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateReversalParamsLineItemsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        form.field("amount_tax", &self.amount_tax);
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field("original_line_item", &self.original_line_item);
        form.field_opt("quantity", self.quantity.as_ref());
        form.field("reference", &self.reference);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateReversalParamsMode {
    Full,
    Partial,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl CreateReversalParamsMode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateReversalParamsMode::Full => "full",
            CreateReversalParamsMode::Partial => "partial",
            CreateReversalParamsMode::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for CreateReversalParamsMode {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateReversalParamsMode::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "full" => CreateReversalParamsMode::Full,
            "partial" => CreateReversalParamsMode::Partial,
            other => CreateReversalParamsMode::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateReversalParamsMode {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateReversalParamsShippingCost {
    #[doc = "The amount to reverse, in the [smallest currency unit](https://docs.stripe.com/currencies#minor-units) in negative."]
    pub amount: i64,
    #[doc = "The amount of tax to reverse, in the [smallest currency unit](https://docs.stripe.com/currencies#minor-units) in nega..."]
    pub amount_tax: i64,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateReversalParamsShippingCost {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        form.field("amount_tax", &self.amount_tax);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType {
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

#[cfg(feature = "tax")]
impl CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::AdNrt => "ad_nrt",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::AeTrn => "ae_trn",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::AlTin => "al_tin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::AmTin => "am_tin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::AoTin => "ao_tin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::ArCuit => "ar_cuit",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::AuAbn => "au_abn",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::AuArn => "au_arn",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::AwTin => "aw_tin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::AzTin => "az_tin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::BaTin => "ba_tin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::BbTin => "bb_tin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::BdBin => "bd_bin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::BfIfu => "bf_ifu",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::BgUic => "bg_uic",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::BhVat => "bh_vat",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::BjIfu => "bj_ifu",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::BoTin => "bo_tin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::BrCnpj => "br_cnpj",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::BrCpf => "br_cpf",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::BsTin => "bs_tin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::ByTin => "by_tin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::CaBn => "ca_bn",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::CaGstHst => "ca_gst_hst",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::CaPstBc => "ca_pst_bc",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::CaPstMb => "ca_pst_mb",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::CaPstSk => "ca_pst_sk",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::CaQst => "ca_qst",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::CdNif => "cd_nif",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::ChUid => "ch_uid",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::ChVat => "ch_vat",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::ClTin => "cl_tin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::CmNiu => "cm_niu",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::CnTin => "cn_tin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::CoNit => "co_nit",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::CrTin => "cr_tin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::CvNif => "cv_nif",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::DeStn => "de_stn",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::DoRcn => "do_rcn",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::EcRuc => "ec_ruc",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::EgTin => "eg_tin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::EsCif => "es_cif",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::EtTin => "et_tin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::EuOssVat => "eu_oss_vat",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::EuVat => "eu_vat",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::FoVat => "fo_vat",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::GbVat => "gb_vat",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::GeVat => "ge_vat",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::GiTin => "gi_tin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::GnNif => "gn_nif",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::HkBr => "hk_br",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::HrOib => "hr_oib",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::HuTin => "hu_tin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::IcNif => "ic_nif",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::IdNpwp => "id_npwp",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::IlVat => "il_vat",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::InGst => "in_gst",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::IsVat => "is_vat",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::ItCf => "it_cf",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::JpCn => "jp_cn",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::JpRn => "jp_rn",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::JpTrn => "jp_trn",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::KePin => "ke_pin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::KgTin => "kg_tin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::KhTin => "kh_tin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::KrBrn => "kr_brn",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::KzBin => "kz_bin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::LaTin => "la_tin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::LiUid => "li_uid",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::LiVat => "li_vat",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::LkVat => "lk_vat",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::MaVat => "ma_vat",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::MdVat => "md_vat",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::MePib => "me_pib",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::MkVat => "mk_vat",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::MrNif => "mr_nif",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::MxRfc => "mx_rfc",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::MyFrp => "my_frp",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::MyItn => "my_itn",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::MySst => "my_sst",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::NgTin => "ng_tin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::NoVat => "no_vat",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::NoVoec => "no_voec",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::NpPan => "np_pan",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::NzGst => "nz_gst",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::OmVat => "om_vat",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::PeRuc => "pe_ruc",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::PhTin => "ph_tin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::PlNip => "pl_nip",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::PyRuc => "py_ruc",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::RoTin => "ro_tin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::RsPib => "rs_pib",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::RuInn => "ru_inn",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::RuKpp => "ru_kpp",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::SaVat => "sa_vat",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::SgGst => "sg_gst",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::SgUen => "sg_uen",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::SiTin => "si_tin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::SnNinea => "sn_ninea",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::SrFin => "sr_fin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::SvNit => "sv_nit",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::ThVat => "th_vat",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::TjTin => "tj_tin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::TrTin => "tr_tin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::TwVat => "tw_vat",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::TzVat => "tz_vat",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::UaVat => "ua_vat",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::UgTin => "ug_tin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::UsEin => "us_ein",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::UyRuc => "uy_ruc",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::UzTin => "uz_tin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::UzVat => "uz_vat",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::VeRif => "ve_rif",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::VnTin => "vn_tin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::ZaVat => "za_vat",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::ZmTin => "zm_tin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::ZwTin => "zw_tin",
            CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ad_nrt" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::AdNrt,
            "ae_trn" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::AeTrn,
            "al_tin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::AlTin,
            "am_tin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::AmTin,
            "ao_tin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::AoTin,
            "ar_cuit" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::ArCuit,
            "au_abn" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::AuAbn,
            "au_arn" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::AuArn,
            "aw_tin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::AwTin,
            "az_tin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::AzTin,
            "ba_tin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::BaTin,
            "bb_tin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::BbTin,
            "bd_bin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::BdBin,
            "bf_ifu" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::BfIfu,
            "bg_uic" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::BgUic,
            "bh_vat" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::BhVat,
            "bj_ifu" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::BjIfu,
            "bo_tin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::BoTin,
            "br_cnpj" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::BrCnpj,
            "br_cpf" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::BrCpf,
            "bs_tin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::BsTin,
            "by_tin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::ByTin,
            "ca_bn" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::CaBn,
            "ca_gst_hst" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::CaGstHst,
            "ca_pst_bc" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::CaPstBc,
            "ca_pst_mb" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::CaPstMb,
            "ca_pst_sk" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::CaPstSk,
            "ca_qst" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::CaQst,
            "cd_nif" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::CdNif,
            "ch_uid" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::ChUid,
            "ch_vat" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::ChVat,
            "cl_tin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::ClTin,
            "cm_niu" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::CmNiu,
            "cn_tin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::CnTin,
            "co_nit" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::CoNit,
            "cr_tin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::CrTin,
            "cv_nif" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::CvNif,
            "de_stn" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::DeStn,
            "do_rcn" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::DoRcn,
            "ec_ruc" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::EcRuc,
            "eg_tin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::EgTin,
            "es_cif" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::EsCif,
            "et_tin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::EtTin,
            "eu_oss_vat" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::EuOssVat,
            "eu_vat" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::EuVat,
            "fo_vat" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::FoVat,
            "gb_vat" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::GbVat,
            "ge_vat" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::GeVat,
            "gi_tin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::GiTin,
            "gn_nif" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::GnNif,
            "hk_br" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::HkBr,
            "hr_oib" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::HrOib,
            "hu_tin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::HuTin,
            "ic_nif" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::IcNif,
            "id_npwp" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::IdNpwp,
            "il_vat" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::IlVat,
            "in_gst" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::InGst,
            "is_vat" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::IsVat,
            "it_cf" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::ItCf,
            "jp_cn" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::JpCn,
            "jp_rn" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::JpRn,
            "jp_trn" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::JpTrn,
            "ke_pin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::KePin,
            "kg_tin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::KgTin,
            "kh_tin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::KhTin,
            "kr_brn" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::KrBrn,
            "kz_bin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::KzBin,
            "la_tin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::LaTin,
            "li_uid" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::LiUid,
            "li_vat" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::LiVat,
            "lk_vat" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::LkVat,
            "ma_vat" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::MaVat,
            "md_vat" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::MdVat,
            "me_pib" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::MePib,
            "mk_vat" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::MkVat,
            "mr_nif" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::MrNif,
            "mx_rfc" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::MxRfc,
            "my_frp" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::MyFrp,
            "my_itn" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::MyItn,
            "my_sst" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::MySst,
            "ng_tin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::NgTin,
            "no_vat" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::NoVat,
            "no_voec" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::NoVoec,
            "np_pan" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::NpPan,
            "nz_gst" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::NzGst,
            "om_vat" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::OmVat,
            "pe_ruc" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::PeRuc,
            "ph_tin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::PhTin,
            "pl_nip" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::PlNip,
            "py_ruc" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::PyRuc,
            "ro_tin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::RoTin,
            "rs_pib" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::RsPib,
            "ru_inn" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::RuInn,
            "ru_kpp" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::RuKpp,
            "sa_vat" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::SaVat,
            "sg_gst" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::SgGst,
            "sg_uen" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::SgUen,
            "si_tin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::SiTin,
            "sn_ninea" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::SnNinea,
            "sr_fin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::SrFin,
            "sv_nit" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::SvNit,
            "th_vat" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::ThVat,
            "tj_tin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::TjTin,
            "tr_tin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::TrTin,
            "tw_vat" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::TwVat,
            "tz_vat" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::TzVat,
            "ua_vat" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::UaVat,
            "ug_tin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::UgTin,
            "us_ein" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::UsEin,
            "uy_ruc" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::UyRuc,
            "uz_tin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::UzTin,
            "uz_vat" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::UzVat,
            "ve_rif" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::VeRif,
            "vn_tin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::VnTin,
            "za_vat" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::ZaVat,
            "zm_tin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::ZmTin,
            "zw_tin" => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::ZwTin,
            other => CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxCalculationParamsCustomerDetailsTaxIdsItemType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTaxCalculationParamsCustomerDetailsTaxabilityOverride {
    CustomerExempt,
    None,
    ReverseCharge,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl CreateTaxCalculationParamsCustomerDetailsTaxabilityOverride {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTaxCalculationParamsCustomerDetailsTaxabilityOverride::CustomerExempt => "customer_exempt",
            CreateTaxCalculationParamsCustomerDetailsTaxabilityOverride::None => "none",
            CreateTaxCalculationParamsCustomerDetailsTaxabilityOverride::ReverseCharge => "reverse_charge",
            CreateTaxCalculationParamsCustomerDetailsTaxabilityOverride::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for CreateTaxCalculationParamsCustomerDetailsTaxabilityOverride {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTaxCalculationParamsCustomerDetailsTaxabilityOverride::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "customer_exempt" => CreateTaxCalculationParamsCustomerDetailsTaxabilityOverride::CustomerExempt,
            "none" => CreateTaxCalculationParamsCustomerDetailsTaxabilityOverride::None,
            "reverse_charge" => CreateTaxCalculationParamsCustomerDetailsTaxabilityOverride::ReverseCharge,
            other => CreateTaxCalculationParamsCustomerDetailsTaxabilityOverride::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxCalculationParamsCustomerDetailsTaxabilityOverride {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTaxRegistrationParamsCountryOptionsAeType {
    Standard,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl CreateTaxRegistrationParamsCountryOptionsAeType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTaxRegistrationParamsCountryOptionsAeType::Standard => "standard",
            CreateTaxRegistrationParamsCountryOptionsAeType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for CreateTaxRegistrationParamsCountryOptionsAeType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTaxRegistrationParamsCountryOptionsAeType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "standard" => CreateTaxRegistrationParamsCountryOptionsAeType::Standard,
            other => CreateTaxRegistrationParamsCountryOptionsAeType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsAeType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTaxRegistrationParamsCountryOptionsAmType {
    Simplified,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl CreateTaxRegistrationParamsCountryOptionsAmType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTaxRegistrationParamsCountryOptionsAmType::Simplified => "simplified",
            CreateTaxRegistrationParamsCountryOptionsAmType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for CreateTaxRegistrationParamsCountryOptionsAmType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTaxRegistrationParamsCountryOptionsAmType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "simplified" => CreateTaxRegistrationParamsCountryOptionsAmType::Simplified,
            other => CreateTaxRegistrationParamsCountryOptionsAmType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsAmType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxLocationType {
    Performance,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl TaxLocationType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxLocationType::Performance => "performance",
            TaxLocationType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl Default for TaxLocationType {
    fn default() -> Self {
        TaxLocationType::Unknown(String::new())
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for TaxLocationType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TaxLocationType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "performance" => TaxLocationType::Performance,
            other => TaxLocationType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxProductRegistrationsResourceCountryOptionsDefaultType {
    Standard,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl TaxProductRegistrationsResourceCountryOptionsDefaultType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxProductRegistrationsResourceCountryOptionsDefaultType::Standard => "standard",
            TaxProductRegistrationsResourceCountryOptionsDefaultType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl Default for TaxProductRegistrationsResourceCountryOptionsDefaultType {
    fn default() -> Self {
        TaxProductRegistrationsResourceCountryOptionsDefaultType::Unknown(String::new())
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for TaxProductRegistrationsResourceCountryOptionsDefaultType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TaxProductRegistrationsResourceCountryOptionsDefaultType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "standard" => TaxProductRegistrationsResourceCountryOptionsDefaultType::Standard,
            other => TaxProductRegistrationsResourceCountryOptionsDefaultType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductRegistrationsResourceCountryOptionsUsParkingTax {
    #[doc = "A [jurisdiction code](https://docs.stripe.com/tax/registering?type=parking_tax#registration-types) representing the l..."]
    #[json(default)]
    pub jurisdiction: String,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason {
    AnotherPaymentAssociatedWithCalculation,
    CalculationExpired,
    CurrencyMismatch,
    OriginalTransactionVoided,
    UniqueReferenceViolation,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason::AnotherPaymentAssociatedWithCalculation => "another_payment_associated_with_calculation",
            TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason::CalculationExpired => "calculation_expired",
            TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason::CurrencyMismatch => "currency_mismatch",
            TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason::OriginalTransactionVoided => "original_transaction_voided",
            TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason::UniqueReferenceViolation => "unique_reference_violation",
            TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl Default for TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason {
    fn default() -> Self {
        TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason::Unknown(String::new())
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "another_payment_associated_with_calculation" => TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason::AnotherPaymentAssociatedWithCalculation,
            "calculation_expired" => TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason::CalculationExpired,
            "currency_mismatch" => TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason::CurrencyMismatch,
            "original_transaction_voided" => TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason::OriginalTransactionVoided,
            "unique_reference_violation" => TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason::UniqueReferenceViolation,
            other => TaxProductResourceTaxAssociationTransactionAttemptsResourceErroredReason::Unknown(other.to_string()),
        })
    }
}
