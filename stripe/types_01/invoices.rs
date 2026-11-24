#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct AttachPaymentParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The ID of the PaymentIntent to attach to the invoice."]
    pub payment_intent: Option<String>,
    #[doc = "The ID of the PaymentRecord to attach to the invoice."]
    pub payment_record: Option<String>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for AttachPaymentParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("payment_intent", self.payment_intent.as_ref());
        form.field_opt("payment_record", self.payment_record.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateInvoiceParamsAutomaticTaxLiabilityType {
    Account,
    SelfValue,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreateInvoiceParamsAutomaticTaxLiabilityType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateInvoiceParamsAutomaticTaxLiabilityType::Account => "account",
            CreateInvoiceParamsAutomaticTaxLiabilityType::SelfValue => "self",
            CreateInvoiceParamsAutomaticTaxLiabilityType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for CreateInvoiceParamsAutomaticTaxLiabilityType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateInvoiceParamsAutomaticTaxLiabilityType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account" => CreateInvoiceParamsAutomaticTaxLiabilityType::Account,
            "self" => CreateInvoiceParamsAutomaticTaxLiabilityType::SelfValue,
            other => CreateInvoiceParamsAutomaticTaxLiabilityType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsAutomaticTaxLiabilityType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegisteredAddress {
    #[doc = "City, district, suburb, town, or village."]
    pub city: Option<String>,
    #[doc = "Two-letter country code."]
    pub country: Option<String>,
    #[doc = "Address line 1 (for example, street, PO Box, or company name)."]
    pub line1: Option<String>,
    #[doc = "Address line 2 (for example, apartment, suite, unit, or building)."]
    pub line2: Option<String>,
    #[doc = "ZIP or postal code."]
    pub postal_code: Option<String>,
    #[doc = "State, county, province, or region."]
    pub state: Option<String>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode
    for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBillieCompanyDetailsRegisteredAddress
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("city", self.city.as_ref());
        form.field_opt("country", self.country.as_ref());
        form.field_opt("line1", self.line1.as_ref());
        form.field_opt("line2", self.line2.as_ref());
        form.field_opt("postal_code", self.postal_code.as_ref());
        form.field_opt("state", self.state.as_ref());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBlik {}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsBlik {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        let _ = form;
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose {
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

#[cfg(feature = "invoices")]
impl CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::DependantSupport => {
                "dependant_support"
            }
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Government => {
                "government"
            }
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Loan => "loan",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Mortgage => "mortgage",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Other => "other",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Pension => "pension",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Personal => "personal",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Retail => "retail",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Salary => "salary",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Tax => "tax",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Utility => "utility",
            CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "dependant_support" => {
                CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::DependantSupport
            }
            "government" => {
                CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Government
            }
            "loan" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Loan,
            "mortgage" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Mortgage,
            "other" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Other,
            "pension" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Pension,
            "personal" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Personal,
            "retail" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Retail,
            "salary" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Salary,
            "tax" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Tax,
            "utility" => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Utility,
            other => CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsPaymentSettingsPaymentMethodOptionsPaytoMandateOptionsPurpose {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateInvoiceParamsRenderingPdfPageSize {
    A4,
    Auto,
    Letter,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreateInvoiceParamsRenderingPdfPageSize {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateInvoiceParamsRenderingPdfPageSize::A4 => "a4",
            CreateInvoiceParamsRenderingPdfPageSize::Auto => "auto",
            CreateInvoiceParamsRenderingPdfPageSize::Letter => "letter",
            CreateInvoiceParamsRenderingPdfPageSize::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for CreateInvoiceParamsRenderingPdfPageSize {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateInvoiceParamsRenderingPdfPageSize::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "a4" => CreateInvoiceParamsRenderingPdfPageSize::A4,
            "auto" => CreateInvoiceParamsRenderingPdfPageSize::Auto,
            "letter" => CreateInvoiceParamsRenderingPdfPageSize::Letter,
            other => CreateInvoiceParamsRenderingPdfPageSize::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreateInvoiceParamsRenderingPdfPageSize {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePreviewParamsCustomerDetailsTaxIdsItemType {
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

#[cfg(feature = "invoices")]
impl CreatePreviewParamsCustomerDetailsTaxIdsItemType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::AdNrt => "ad_nrt",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::AeTrn => "ae_trn",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::AlTin => "al_tin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::AmTin => "am_tin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::AoTin => "ao_tin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::ArCuit => "ar_cuit",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::AuAbn => "au_abn",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::AuArn => "au_arn",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::AwTin => "aw_tin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::AzTin => "az_tin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::BaTin => "ba_tin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::BbTin => "bb_tin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::BdBin => "bd_bin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::BfIfu => "bf_ifu",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::BgUic => "bg_uic",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::BhVat => "bh_vat",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::BjIfu => "bj_ifu",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::BoTin => "bo_tin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::BrCnpj => "br_cnpj",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::BrCpf => "br_cpf",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::BsTin => "bs_tin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::ByTin => "by_tin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::CaBn => "ca_bn",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::CaGstHst => "ca_gst_hst",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::CaPstBc => "ca_pst_bc",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::CaPstMb => "ca_pst_mb",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::CaPstSk => "ca_pst_sk",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::CaQst => "ca_qst",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::CdNif => "cd_nif",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::ChUid => "ch_uid",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::ChVat => "ch_vat",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::ClTin => "cl_tin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::CmNiu => "cm_niu",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::CnTin => "cn_tin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::CoNit => "co_nit",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::CrTin => "cr_tin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::CvNif => "cv_nif",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::DeStn => "de_stn",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::DoRcn => "do_rcn",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::EcRuc => "ec_ruc",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::EgTin => "eg_tin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::EsCif => "es_cif",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::EtTin => "et_tin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::EuOssVat => "eu_oss_vat",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::EuVat => "eu_vat",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::FoVat => "fo_vat",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::GbVat => "gb_vat",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::GeVat => "ge_vat",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::GiTin => "gi_tin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::GnNif => "gn_nif",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::HkBr => "hk_br",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::HrOib => "hr_oib",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::HuTin => "hu_tin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::IcNif => "ic_nif",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::IdNpwp => "id_npwp",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::IlVat => "il_vat",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::InGst => "in_gst",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::IsVat => "is_vat",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::ItCf => "it_cf",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::JpCn => "jp_cn",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::JpRn => "jp_rn",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::JpTrn => "jp_trn",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::KePin => "ke_pin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::KgTin => "kg_tin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::KhTin => "kh_tin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::KrBrn => "kr_brn",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::KzBin => "kz_bin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::LaTin => "la_tin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::LiUid => "li_uid",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::LiVat => "li_vat",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::LkVat => "lk_vat",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::MaVat => "ma_vat",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::MdVat => "md_vat",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::MePib => "me_pib",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::MkVat => "mk_vat",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::MrNif => "mr_nif",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::MxRfc => "mx_rfc",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::MyFrp => "my_frp",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::MyItn => "my_itn",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::MySst => "my_sst",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::NgTin => "ng_tin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::NoVat => "no_vat",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::NoVoec => "no_voec",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::NpPan => "np_pan",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::NzGst => "nz_gst",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::OmVat => "om_vat",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::PeRuc => "pe_ruc",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::PhTin => "ph_tin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::PlNip => "pl_nip",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::PyRuc => "py_ruc",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::RoTin => "ro_tin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::RsPib => "rs_pib",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::RuInn => "ru_inn",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::RuKpp => "ru_kpp",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::SaVat => "sa_vat",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::SgGst => "sg_gst",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::SgUen => "sg_uen",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::SiTin => "si_tin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::SnNinea => "sn_ninea",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::SrFin => "sr_fin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::SvNit => "sv_nit",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::ThVat => "th_vat",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::TjTin => "tj_tin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::TrTin => "tr_tin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::TwVat => "tw_vat",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::TzVat => "tz_vat",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::UaVat => "ua_vat",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::UgTin => "ug_tin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::UsEin => "us_ein",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::UyRuc => "uy_ruc",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::UzTin => "uz_tin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::UzVat => "uz_vat",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::VeRif => "ve_rif",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::VnTin => "vn_tin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::ZaVat => "za_vat",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::ZmTin => "zm_tin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::ZwTin => "zw_tin",
            CreatePreviewParamsCustomerDetailsTaxIdsItemType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for CreatePreviewParamsCustomerDetailsTaxIdsItemType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePreviewParamsCustomerDetailsTaxIdsItemType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ad_nrt" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::AdNrt,
            "ae_trn" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::AeTrn,
            "al_tin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::AlTin,
            "am_tin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::AmTin,
            "ao_tin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::AoTin,
            "ar_cuit" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::ArCuit,
            "au_abn" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::AuAbn,
            "au_arn" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::AuArn,
            "aw_tin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::AwTin,
            "az_tin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::AzTin,
            "ba_tin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::BaTin,
            "bb_tin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::BbTin,
            "bd_bin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::BdBin,
            "bf_ifu" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::BfIfu,
            "bg_uic" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::BgUic,
            "bh_vat" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::BhVat,
            "bj_ifu" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::BjIfu,
            "bo_tin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::BoTin,
            "br_cnpj" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::BrCnpj,
            "br_cpf" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::BrCpf,
            "bs_tin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::BsTin,
            "by_tin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::ByTin,
            "ca_bn" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::CaBn,
            "ca_gst_hst" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::CaGstHst,
            "ca_pst_bc" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::CaPstBc,
            "ca_pst_mb" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::CaPstMb,
            "ca_pst_sk" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::CaPstSk,
            "ca_qst" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::CaQst,
            "cd_nif" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::CdNif,
            "ch_uid" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::ChUid,
            "ch_vat" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::ChVat,
            "cl_tin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::ClTin,
            "cm_niu" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::CmNiu,
            "cn_tin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::CnTin,
            "co_nit" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::CoNit,
            "cr_tin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::CrTin,
            "cv_nif" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::CvNif,
            "de_stn" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::DeStn,
            "do_rcn" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::DoRcn,
            "ec_ruc" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::EcRuc,
            "eg_tin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::EgTin,
            "es_cif" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::EsCif,
            "et_tin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::EtTin,
            "eu_oss_vat" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::EuOssVat,
            "eu_vat" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::EuVat,
            "fo_vat" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::FoVat,
            "gb_vat" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::GbVat,
            "ge_vat" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::GeVat,
            "gi_tin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::GiTin,
            "gn_nif" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::GnNif,
            "hk_br" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::HkBr,
            "hr_oib" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::HrOib,
            "hu_tin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::HuTin,
            "ic_nif" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::IcNif,
            "id_npwp" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::IdNpwp,
            "il_vat" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::IlVat,
            "in_gst" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::InGst,
            "is_vat" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::IsVat,
            "it_cf" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::ItCf,
            "jp_cn" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::JpCn,
            "jp_rn" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::JpRn,
            "jp_trn" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::JpTrn,
            "ke_pin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::KePin,
            "kg_tin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::KgTin,
            "kh_tin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::KhTin,
            "kr_brn" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::KrBrn,
            "kz_bin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::KzBin,
            "la_tin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::LaTin,
            "li_uid" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::LiUid,
            "li_vat" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::LiVat,
            "lk_vat" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::LkVat,
            "ma_vat" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::MaVat,
            "md_vat" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::MdVat,
            "me_pib" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::MePib,
            "mk_vat" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::MkVat,
            "mr_nif" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::MrNif,
            "mx_rfc" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::MxRfc,
            "my_frp" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::MyFrp,
            "my_itn" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::MyItn,
            "my_sst" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::MySst,
            "ng_tin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::NgTin,
            "no_vat" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::NoVat,
            "no_voec" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::NoVoec,
            "np_pan" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::NpPan,
            "nz_gst" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::NzGst,
            "om_vat" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::OmVat,
            "pe_ruc" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::PeRuc,
            "ph_tin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::PhTin,
            "pl_nip" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::PlNip,
            "py_ruc" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::PyRuc,
            "ro_tin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::RoTin,
            "rs_pib" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::RsPib,
            "ru_inn" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::RuInn,
            "ru_kpp" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::RuKpp,
            "sa_vat" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::SaVat,
            "sg_gst" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::SgGst,
            "sg_uen" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::SgUen,
            "si_tin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::SiTin,
            "sn_ninea" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::SnNinea,
            "sr_fin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::SrFin,
            "sv_nit" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::SvNit,
            "th_vat" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::ThVat,
            "tj_tin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::TjTin,
            "tr_tin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::TrTin,
            "tw_vat" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::TwVat,
            "tz_vat" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::TzVat,
            "ua_vat" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::UaVat,
            "ug_tin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::UgTin,
            "us_ein" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::UsEin,
            "uy_ruc" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::UyRuc,
            "uz_tin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::UzTin,
            "uz_vat" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::UzVat,
            "ve_rif" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::VeRif,
            "vn_tin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::VnTin,
            "za_vat" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::ZaVat,
            "zm_tin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::ZmTin,
            "zw_tin" => CreatePreviewParamsCustomerDetailsTaxIdsItemType::ZwTin,
            other => CreatePreviewParamsCustomerDetailsTaxIdsItemType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsCustomerDetailsTaxIdsItemType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePreviewParamsScheduleDetailsBillingModeType {
    Classic,
    Flexible,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreatePreviewParamsScheduleDetailsBillingModeType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePreviewParamsScheduleDetailsBillingModeType::Classic => "classic",
            CreatePreviewParamsScheduleDetailsBillingModeType::Flexible => "flexible",
            CreatePreviewParamsScheduleDetailsBillingModeType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for CreatePreviewParamsScheduleDetailsBillingModeType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePreviewParamsScheduleDetailsBillingModeType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "classic" => CreatePreviewParamsScheduleDetailsBillingModeType::Classic,
            "flexible" => CreatePreviewParamsScheduleDetailsBillingModeType::Flexible,
            other => CreatePreviewParamsScheduleDetailsBillingModeType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsScheduleDetailsBillingModeType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePreviewParamsSubscriptionDetailsBillingCycleAnchorType {
    Now,
    Timestamp,
    Unchanged,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreatePreviewParamsSubscriptionDetailsBillingCycleAnchorType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePreviewParamsSubscriptionDetailsBillingCycleAnchorType::Now => "now",
            CreatePreviewParamsSubscriptionDetailsBillingCycleAnchorType::Timestamp => "timestamp",
            CreatePreviewParamsSubscriptionDetailsBillingCycleAnchorType::Unchanged => "unchanged",
            CreatePreviewParamsSubscriptionDetailsBillingCycleAnchorType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for CreatePreviewParamsSubscriptionDetailsBillingCycleAnchorType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePreviewParamsSubscriptionDetailsBillingCycleAnchorType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "now" => CreatePreviewParamsSubscriptionDetailsBillingCycleAnchorType::Now,
            "timestamp" => CreatePreviewParamsSubscriptionDetailsBillingCycleAnchorType::Timestamp,
            "unchanged" => CreatePreviewParamsSubscriptionDetailsBillingCycleAnchorType::Unchanged,
            other => CreatePreviewParamsSubscriptionDetailsBillingCycleAnchorType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsSubscriptionDetailsBillingCycleAnchorType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePreviewParamsSubscriptionDetailsPauseBillForOutstandingUsageThroughType {
    None,
    Now,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreatePreviewParamsSubscriptionDetailsPauseBillForOutstandingUsageThroughType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePreviewParamsSubscriptionDetailsPauseBillForOutstandingUsageThroughType::None => "none",
            CreatePreviewParamsSubscriptionDetailsPauseBillForOutstandingUsageThroughType::Now => "now",
            CreatePreviewParamsSubscriptionDetailsPauseBillForOutstandingUsageThroughType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for CreatePreviewParamsSubscriptionDetailsPauseBillForOutstandingUsageThroughType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreatePreviewParamsSubscriptionDetailsPauseBillForOutstandingUsageThroughType::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "none" => CreatePreviewParamsSubscriptionDetailsPauseBillForOutstandingUsageThroughType::None,
            "now" => CreatePreviewParamsSubscriptionDetailsPauseBillForOutstandingUsageThroughType::Now,
            other => CreatePreviewParamsSubscriptionDetailsPauseBillForOutstandingUsageThroughType::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsSubscriptionDetailsPauseBillForOutstandingUsageThroughType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePreviewParamsSubscriptionDetailsPauseBillForUnusedTimeFromType {
    ItemCurrentPeriodStart,
    None,
    Now,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl CreatePreviewParamsSubscriptionDetailsPauseBillForUnusedTimeFromType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePreviewParamsSubscriptionDetailsPauseBillForUnusedTimeFromType::ItemCurrentPeriodStart => {
                "item_current_period_start"
            }
            CreatePreviewParamsSubscriptionDetailsPauseBillForUnusedTimeFromType::None => "none",
            CreatePreviewParamsSubscriptionDetailsPauseBillForUnusedTimeFromType::Now => "now",
            CreatePreviewParamsSubscriptionDetailsPauseBillForUnusedTimeFromType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for CreatePreviewParamsSubscriptionDetailsPauseBillForUnusedTimeFromType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePreviewParamsSubscriptionDetailsPauseBillForUnusedTimeFromType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "item_current_period_start" => {
                CreatePreviewParamsSubscriptionDetailsPauseBillForUnusedTimeFromType::ItemCurrentPeriodStart
            }
            "none" => CreatePreviewParamsSubscriptionDetailsPauseBillForUnusedTimeFromType::None,
            "now" => CreatePreviewParamsSubscriptionDetailsPauseBillForUnusedTimeFromType::Now,
            other => CreatePreviewParamsSubscriptionDetailsPauseBillForUnusedTimeFromType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for CreatePreviewParamsSubscriptionDetailsPauseBillForUnusedTimeFromType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, Default)]
pub struct FinalizeInvoiceParams {
    #[doc = "Controls whether Stripe performs automatic collection..."]
    pub auto_advance: Option<bool>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for FinalizeInvoiceParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("auto_advance", self.auto_advance.as_ref());
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
pub enum InvoiceMandateOptionsPaytoPurpose {
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
impl InvoiceMandateOptionsPaytoPurpose {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            InvoiceMandateOptionsPaytoPurpose::DependantSupport => "dependant_support",
            InvoiceMandateOptionsPaytoPurpose::Government => "government",
            InvoiceMandateOptionsPaytoPurpose::Loan => "loan",
            InvoiceMandateOptionsPaytoPurpose::Mortgage => "mortgage",
            InvoiceMandateOptionsPaytoPurpose::Other => "other",
            InvoiceMandateOptionsPaytoPurpose::Pension => "pension",
            InvoiceMandateOptionsPaytoPurpose::Personal => "personal",
            InvoiceMandateOptionsPaytoPurpose::Retail => "retail",
            InvoiceMandateOptionsPaytoPurpose::Salary => "salary",
            InvoiceMandateOptionsPaytoPurpose::Tax => "tax",
            InvoiceMandateOptionsPaytoPurpose::Utility => "utility",
            InvoiceMandateOptionsPaytoPurpose::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for InvoiceMandateOptionsPaytoPurpose {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(InvoiceMandateOptionsPaytoPurpose::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "dependant_support" => InvoiceMandateOptionsPaytoPurpose::DependantSupport,
            "government" => InvoiceMandateOptionsPaytoPurpose::Government,
            "loan" => InvoiceMandateOptionsPaytoPurpose::Loan,
            "mortgage" => InvoiceMandateOptionsPaytoPurpose::Mortgage,
            "other" => InvoiceMandateOptionsPaytoPurpose::Other,
            "pension" => InvoiceMandateOptionsPaytoPurpose::Pension,
            "personal" => InvoiceMandateOptionsPaytoPurpose::Personal,
            "retail" => InvoiceMandateOptionsPaytoPurpose::Retail,
            "salary" => InvoiceMandateOptionsPaytoPurpose::Salary,
            "tax" => InvoiceMandateOptionsPaytoPurpose::Tax,
            "utility" => InvoiceMandateOptionsPaytoPurpose::Utility,
            other => InvoiceMandateOptionsPaytoPurpose::Unknown(other.to_string()),
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
pub struct InvoiceSettingCustomerRenderingOptions {
    #[doc = "How line-item prices and amounts will be displayed with respect to tax on invoice PDFs."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_tax_display: Option<String>,
    #[doc = "ID of the invoice rendering template to be used for this customer's invoices."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
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
pub struct InvoicesPaymentsInvoicePaymentStatusTransitions {
    #[doc = "The time that the payment was canceled."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<i64>,
    #[doc = "The time that the payment succeeded."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<i64>,
}

#[cfg(feature = "invoices")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListInvoicesParamsStatus {
    Draft,
    Open,
    Paid,
    Uncollectible,
    Void,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "invoices")]
impl ListInvoicesParamsStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListInvoicesParamsStatus::Draft => "draft",
            ListInvoicesParamsStatus::Open => "open",
            ListInvoicesParamsStatus::Paid => "paid",
            ListInvoicesParamsStatus::Uncollectible => "uncollectible",
            ListInvoicesParamsStatus::Void => "void",
            ListInvoicesParamsStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "invoices")]
impl<'json> json::Decode<'json> for ListInvoicesParamsStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListInvoicesParamsStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "draft" => ListInvoicesParamsStatus::Draft,
            "open" => ListInvoicesParamsStatus::Open,
            "paid" => ListInvoicesParamsStatus::Paid,
            "uncollectible" => ListInvoicesParamsStatus::Uncollectible,
            "void" => ListInvoicesParamsStatus::Void,
            other => ListInvoicesParamsStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "invoices")]
impl crate::params::FormEncode for ListInvoicesParamsStatus {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
