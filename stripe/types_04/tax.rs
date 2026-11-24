#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxCalculationParamsCustomerDetailsAddress {
    #[doc = "City, district, suburb, town, or village."]
    pub city: Option<json::OwnedValue>,
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub country: String,
    #[doc = "Address line 1, such as the street, PO Box, or company name."]
    pub line1: Option<json::OwnedValue>,
    #[doc = "Address line 2, such as the apartment, suite, unit, or building."]
    pub line2: Option<json::OwnedValue>,
    #[doc = "ZIP or postal code."]
    pub postal_code: Option<json::OwnedValue>,
    #[doc = "State, county, province, or region."]
    pub state: Option<json::OwnedValue>,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxCalculationParamsCustomerDetailsAddress {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("country", &self.country);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRegistrationParamsCountryOptionsCaProvinceStandard {
    #[doc = "Two-letter CA province code ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2))."]
    pub province: String,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsCaProvinceStandard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("province", &self.province);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTaxRegistrationParamsCountryOptionsCaType {
    ProvinceStandard,
    Simplified,
    Standard,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl CreateTaxRegistrationParamsCountryOptionsCaType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTaxRegistrationParamsCountryOptionsCaType::ProvinceStandard => "province_standard",
            CreateTaxRegistrationParamsCountryOptionsCaType::Simplified => "simplified",
            CreateTaxRegistrationParamsCountryOptionsCaType::Standard => "standard",
            CreateTaxRegistrationParamsCountryOptionsCaType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for CreateTaxRegistrationParamsCountryOptionsCaType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTaxRegistrationParamsCountryOptionsCaType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "province_standard" => CreateTaxRegistrationParamsCountryOptionsCaType::ProvinceStandard,
            "simplified" => CreateTaxRegistrationParamsCountryOptionsCaType::Simplified,
            "standard" => CreateTaxRegistrationParamsCountryOptionsCaType::Standard,
            other => CreateTaxRegistrationParamsCountryOptionsCaType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsCaType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, Default)]
pub struct CreateTaxRegistrationParamsCountryOptionsUsAdmissionsTax {
    #[doc = "A jurisdiction code representing the [local jurisdiction](/tax/registering?type=admissions_tax#registration-types)."]
    pub jurisdiction: String,
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for CreateTaxRegistrationParamsCountryOptionsUsAdmissionsTax {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("jurisdiction", &self.jurisdiction);
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListTaxLocationsParamsType {
    Performance,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl ListTaxLocationsParamsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListTaxLocationsParamsType::Performance => "performance",
            ListTaxLocationsParamsType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for ListTaxLocationsParamsType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListTaxLocationsParamsType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "performance" => ListTaxLocationsParamsType::Performance,
            other => ListTaxLocationsParamsType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for ListTaxLocationsParamsType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListTaxRegistrationsParamsStatus {
    Active,
    All,
    Expired,
    Scheduled,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl ListTaxRegistrationsParamsStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListTaxRegistrationsParamsStatus::Active => "active",
            ListTaxRegistrationsParamsStatus::All => "all",
            ListTaxRegistrationsParamsStatus::Expired => "expired",
            ListTaxRegistrationsParamsStatus::Scheduled => "scheduled",
            ListTaxRegistrationsParamsStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for ListTaxRegistrationsParamsStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListTaxRegistrationsParamsStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "active" => ListTaxRegistrationsParamsStatus::Active,
            "all" => ListTaxRegistrationsParamsStatus::All,
            "expired" => ListTaxRegistrationsParamsStatus::Expired,
            "scheduled" => ListTaxRegistrationsParamsStatus::Scheduled,
            other => ListTaxRegistrationsParamsStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
impl crate::params::FormEncode for ListTaxRegistrationsParamsStatus {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxCalculationLineItemObject {
    TaxCalculationLineItem,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl TaxCalculationLineItemObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxCalculationLineItemObject::TaxCalculationLineItem => "tax.calculation_line_item",
            TaxCalculationLineItemObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl Default for TaxCalculationLineItemObject {
    fn default() -> Self {
        TaxCalculationLineItemObject::Unknown(String::new())
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for TaxCalculationLineItemObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TaxCalculationLineItemObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "tax.calculation_line_item" => TaxCalculationLineItemObject::TaxCalculationLineItem,
            other => TaxCalculationLineItemObject::Unknown(other.to_string()),
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
pub enum TaxDeductedAtSourceObject {
    TaxDeductedAtSource,
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
impl TaxDeductedAtSourceObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxDeductedAtSourceObject::TaxDeductedAtSource => "tax_deducted_at_source",
            TaxDeductedAtSourceObject::Unknown(value) => value,
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
impl Default for TaxDeductedAtSourceObject {
    fn default() -> Self {
        TaxDeductedAtSourceObject::Unknown(String::new())
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
impl<'json> json::Decode<'json> for TaxDeductedAtSourceObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TaxDeductedAtSourceObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "tax_deducted_at_source" => TaxDeductedAtSourceObject::TaxDeductedAtSource,
            other => TaxDeductedAtSourceObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxLocationObject {
    TaxLocation,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl TaxLocationObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxLocationObject::TaxLocation => "tax.location",
            TaxLocationObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl Default for TaxLocationObject {
    fn default() -> Self {
        TaxLocationObject::Unknown(String::new())
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for TaxLocationObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TaxLocationObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "tax.location" => TaxLocationObject::TaxLocation,
            other => TaxLocationObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxProductRegistrationsResourceCountryOptionsDefaultStandardPlaceOfSupplyScheme {
    InboundGoods,
    Standard,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl TaxProductRegistrationsResourceCountryOptionsDefaultStandardPlaceOfSupplyScheme {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxProductRegistrationsResourceCountryOptionsDefaultStandardPlaceOfSupplyScheme::InboundGoods => {
                "inbound_goods"
            }
            TaxProductRegistrationsResourceCountryOptionsDefaultStandardPlaceOfSupplyScheme::Standard => "standard",
            TaxProductRegistrationsResourceCountryOptionsDefaultStandardPlaceOfSupplyScheme::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl Default for TaxProductRegistrationsResourceCountryOptionsDefaultStandardPlaceOfSupplyScheme {
    fn default() -> Self {
        TaxProductRegistrationsResourceCountryOptionsDefaultStandardPlaceOfSupplyScheme::Unknown(String::new())
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for TaxProductRegistrationsResourceCountryOptionsDefaultStandardPlaceOfSupplyScheme {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                TaxProductRegistrationsResourceCountryOptionsDefaultStandardPlaceOfSupplyScheme::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "inbound_goods" => {
                TaxProductRegistrationsResourceCountryOptionsDefaultStandardPlaceOfSupplyScheme::InboundGoods
            }
            "standard" => TaxProductRegistrationsResourceCountryOptionsDefaultStandardPlaceOfSupplyScheme::Standard,
            other => TaxProductRegistrationsResourceCountryOptionsDefaultStandardPlaceOfSupplyScheme::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductRegistrationsResourceCountryOptionsUsEntertainmentTax {
    #[doc = "A [jurisdiction code](https://docs.stripe.com/tax/registering?type=entertainment_tax#registration-types) representing..."]
    #[json(default)]
    pub jurisdiction: String,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductRegistrationsResourceCountryOptionsUsHospitalityTax {
    #[doc = "A [jurisdiction code](https://docs.stripe.com/tax/registering?type=hospitality_tax#registration-types) representing t..."]
    #[json(default)]
    pub jurisdiction: String,
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType {
    LocalUseTax,
    SimplifiedSellersUseTax,
    SingleLocalUseTax,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "tax")]
impl TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType::LocalUseTax => "local_use_tax",
            TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType::SimplifiedSellersUseTax => {
                "simplified_sellers_use_tax"
            }
            TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType::SingleLocalUseTax => {
                "single_local_use_tax"
            }
            TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl Default for TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType {
    fn default() -> Self {
        TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType::Unknown(String::new())
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "local_use_tax" => TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType::LocalUseTax,
            "simplified_sellers_use_tax" => {
                TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType::SimplifiedSellersUseTax
            }
            "single_local_use_tax" => {
                TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType::SingleLocalUseTax
            }
            other => {
                TaxProductRegistrationsResourceCountryOptionsUsStateSalesTaxElectionType::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxProductResourceCustomerDetailsResourceTaxIdType {
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
    UnknownValue,
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
impl TaxProductResourceCustomerDetailsResourceTaxIdType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxProductResourceCustomerDetailsResourceTaxIdType::AdNrt => "ad_nrt",
            TaxProductResourceCustomerDetailsResourceTaxIdType::AeTrn => "ae_trn",
            TaxProductResourceCustomerDetailsResourceTaxIdType::AlTin => "al_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::AmTin => "am_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::AoTin => "ao_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::ArCuit => "ar_cuit",
            TaxProductResourceCustomerDetailsResourceTaxIdType::AuAbn => "au_abn",
            TaxProductResourceCustomerDetailsResourceTaxIdType::AuArn => "au_arn",
            TaxProductResourceCustomerDetailsResourceTaxIdType::AwTin => "aw_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::AzTin => "az_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::BaTin => "ba_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::BbTin => "bb_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::BdBin => "bd_bin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::BfIfu => "bf_ifu",
            TaxProductResourceCustomerDetailsResourceTaxIdType::BgUic => "bg_uic",
            TaxProductResourceCustomerDetailsResourceTaxIdType::BhVat => "bh_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::BjIfu => "bj_ifu",
            TaxProductResourceCustomerDetailsResourceTaxIdType::BoTin => "bo_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::BrCnpj => "br_cnpj",
            TaxProductResourceCustomerDetailsResourceTaxIdType::BrCpf => "br_cpf",
            TaxProductResourceCustomerDetailsResourceTaxIdType::BsTin => "bs_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::ByTin => "by_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::CaBn => "ca_bn",
            TaxProductResourceCustomerDetailsResourceTaxIdType::CaGstHst => "ca_gst_hst",
            TaxProductResourceCustomerDetailsResourceTaxIdType::CaPstBc => "ca_pst_bc",
            TaxProductResourceCustomerDetailsResourceTaxIdType::CaPstMb => "ca_pst_mb",
            TaxProductResourceCustomerDetailsResourceTaxIdType::CaPstSk => "ca_pst_sk",
            TaxProductResourceCustomerDetailsResourceTaxIdType::CaQst => "ca_qst",
            TaxProductResourceCustomerDetailsResourceTaxIdType::CdNif => "cd_nif",
            TaxProductResourceCustomerDetailsResourceTaxIdType::ChUid => "ch_uid",
            TaxProductResourceCustomerDetailsResourceTaxIdType::ChVat => "ch_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::ClTin => "cl_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::CmNiu => "cm_niu",
            TaxProductResourceCustomerDetailsResourceTaxIdType::CnTin => "cn_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::CoNit => "co_nit",
            TaxProductResourceCustomerDetailsResourceTaxIdType::CrTin => "cr_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::CvNif => "cv_nif",
            TaxProductResourceCustomerDetailsResourceTaxIdType::DeStn => "de_stn",
            TaxProductResourceCustomerDetailsResourceTaxIdType::DoRcn => "do_rcn",
            TaxProductResourceCustomerDetailsResourceTaxIdType::EcRuc => "ec_ruc",
            TaxProductResourceCustomerDetailsResourceTaxIdType::EgTin => "eg_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::EsCif => "es_cif",
            TaxProductResourceCustomerDetailsResourceTaxIdType::EtTin => "et_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::EuOssVat => "eu_oss_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::EuVat => "eu_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::FoVat => "fo_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::GbVat => "gb_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::GeVat => "ge_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::GiTin => "gi_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::GnNif => "gn_nif",
            TaxProductResourceCustomerDetailsResourceTaxIdType::HkBr => "hk_br",
            TaxProductResourceCustomerDetailsResourceTaxIdType::HrOib => "hr_oib",
            TaxProductResourceCustomerDetailsResourceTaxIdType::HuTin => "hu_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::IcNif => "ic_nif",
            TaxProductResourceCustomerDetailsResourceTaxIdType::IdNpwp => "id_npwp",
            TaxProductResourceCustomerDetailsResourceTaxIdType::IlVat => "il_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::InGst => "in_gst",
            TaxProductResourceCustomerDetailsResourceTaxIdType::IsVat => "is_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::ItCf => "it_cf",
            TaxProductResourceCustomerDetailsResourceTaxIdType::JpCn => "jp_cn",
            TaxProductResourceCustomerDetailsResourceTaxIdType::JpRn => "jp_rn",
            TaxProductResourceCustomerDetailsResourceTaxIdType::JpTrn => "jp_trn",
            TaxProductResourceCustomerDetailsResourceTaxIdType::KePin => "ke_pin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::KgTin => "kg_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::KhTin => "kh_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::KrBrn => "kr_brn",
            TaxProductResourceCustomerDetailsResourceTaxIdType::KzBin => "kz_bin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::LaTin => "la_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::LiUid => "li_uid",
            TaxProductResourceCustomerDetailsResourceTaxIdType::LiVat => "li_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::LkVat => "lk_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::MaVat => "ma_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::MdVat => "md_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::MePib => "me_pib",
            TaxProductResourceCustomerDetailsResourceTaxIdType::MkVat => "mk_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::MrNif => "mr_nif",
            TaxProductResourceCustomerDetailsResourceTaxIdType::MxRfc => "mx_rfc",
            TaxProductResourceCustomerDetailsResourceTaxIdType::MyFrp => "my_frp",
            TaxProductResourceCustomerDetailsResourceTaxIdType::MyItn => "my_itn",
            TaxProductResourceCustomerDetailsResourceTaxIdType::MySst => "my_sst",
            TaxProductResourceCustomerDetailsResourceTaxIdType::NgTin => "ng_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::NoVat => "no_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::NoVoec => "no_voec",
            TaxProductResourceCustomerDetailsResourceTaxIdType::NpPan => "np_pan",
            TaxProductResourceCustomerDetailsResourceTaxIdType::NzGst => "nz_gst",
            TaxProductResourceCustomerDetailsResourceTaxIdType::OmVat => "om_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::PeRuc => "pe_ruc",
            TaxProductResourceCustomerDetailsResourceTaxIdType::PhTin => "ph_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::PlNip => "pl_nip",
            TaxProductResourceCustomerDetailsResourceTaxIdType::PyRuc => "py_ruc",
            TaxProductResourceCustomerDetailsResourceTaxIdType::RoTin => "ro_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::RsPib => "rs_pib",
            TaxProductResourceCustomerDetailsResourceTaxIdType::RuInn => "ru_inn",
            TaxProductResourceCustomerDetailsResourceTaxIdType::RuKpp => "ru_kpp",
            TaxProductResourceCustomerDetailsResourceTaxIdType::SaVat => "sa_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::SgGst => "sg_gst",
            TaxProductResourceCustomerDetailsResourceTaxIdType::SgUen => "sg_uen",
            TaxProductResourceCustomerDetailsResourceTaxIdType::SiTin => "si_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::SnNinea => "sn_ninea",
            TaxProductResourceCustomerDetailsResourceTaxIdType::SrFin => "sr_fin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::SvNit => "sv_nit",
            TaxProductResourceCustomerDetailsResourceTaxIdType::ThVat => "th_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::TjTin => "tj_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::TrTin => "tr_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::TwVat => "tw_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::TzVat => "tz_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::UaVat => "ua_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::UgTin => "ug_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::UnknownValue => "unknown",
            TaxProductResourceCustomerDetailsResourceTaxIdType::UsEin => "us_ein",
            TaxProductResourceCustomerDetailsResourceTaxIdType::UyRuc => "uy_ruc",
            TaxProductResourceCustomerDetailsResourceTaxIdType::UzTin => "uz_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::UzVat => "uz_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::VeRif => "ve_rif",
            TaxProductResourceCustomerDetailsResourceTaxIdType::VnTin => "vn_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::ZaVat => "za_vat",
            TaxProductResourceCustomerDetailsResourceTaxIdType::ZmTin => "zm_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::ZwTin => "zw_tin",
            TaxProductResourceCustomerDetailsResourceTaxIdType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "tax")]
impl Default for TaxProductResourceCustomerDetailsResourceTaxIdType {
    fn default() -> Self {
        TaxProductResourceCustomerDetailsResourceTaxIdType::Unknown(String::new())
    }
}

#[cfg(feature = "tax")]
impl<'json> json::Decode<'json> for TaxProductResourceCustomerDetailsResourceTaxIdType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TaxProductResourceCustomerDetailsResourceTaxIdType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ad_nrt" => TaxProductResourceCustomerDetailsResourceTaxIdType::AdNrt,
            "ae_trn" => TaxProductResourceCustomerDetailsResourceTaxIdType::AeTrn,
            "al_tin" => TaxProductResourceCustomerDetailsResourceTaxIdType::AlTin,
            "am_tin" => TaxProductResourceCustomerDetailsResourceTaxIdType::AmTin,
            "ao_tin" => TaxProductResourceCustomerDetailsResourceTaxIdType::AoTin,
            "ar_cuit" => TaxProductResourceCustomerDetailsResourceTaxIdType::ArCuit,
            "au_abn" => TaxProductResourceCustomerDetailsResourceTaxIdType::AuAbn,
            "au_arn" => TaxProductResourceCustomerDetailsResourceTaxIdType::AuArn,
            "aw_tin" => TaxProductResourceCustomerDetailsResourceTaxIdType::AwTin,
            "az_tin" => TaxProductResourceCustomerDetailsResourceTaxIdType::AzTin,
            "ba_tin" => TaxProductResourceCustomerDetailsResourceTaxIdType::BaTin,
            "bb_tin" => TaxProductResourceCustomerDetailsResourceTaxIdType::BbTin,
            "bd_bin" => TaxProductResourceCustomerDetailsResourceTaxIdType::BdBin,
            "bf_ifu" => TaxProductResourceCustomerDetailsResourceTaxIdType::BfIfu,
            "bg_uic" => TaxProductResourceCustomerDetailsResourceTaxIdType::BgUic,
            "bh_vat" => TaxProductResourceCustomerDetailsResourceTaxIdType::BhVat,
            "bj_ifu" => TaxProductResourceCustomerDetailsResourceTaxIdType::BjIfu,
            "bo_tin" => TaxProductResourceCustomerDetailsResourceTaxIdType::BoTin,
            "br_cnpj" => TaxProductResourceCustomerDetailsResourceTaxIdType::BrCnpj,
            "br_cpf" => TaxProductResourceCustomerDetailsResourceTaxIdType::BrCpf,
            "bs_tin" => TaxProductResourceCustomerDetailsResourceTaxIdType::BsTin,
            "by_tin" => TaxProductResourceCustomerDetailsResourceTaxIdType::ByTin,
            "ca_bn" => TaxProductResourceCustomerDetailsResourceTaxIdType::CaBn,
            "ca_gst_hst" => TaxProductResourceCustomerDetailsResourceTaxIdType::CaGstHst,
            "ca_pst_bc" => TaxProductResourceCustomerDetailsResourceTaxIdType::CaPstBc,
            "ca_pst_mb" => TaxProductResourceCustomerDetailsResourceTaxIdType::CaPstMb,
            "ca_pst_sk" => TaxProductResourceCustomerDetailsResourceTaxIdType::CaPstSk,
            "ca_qst" => TaxProductResourceCustomerDetailsResourceTaxIdType::CaQst,
            "cd_nif" => TaxProductResourceCustomerDetailsResourceTaxIdType::CdNif,
            "ch_uid" => TaxProductResourceCustomerDetailsResourceTaxIdType::ChUid,
            "ch_vat" => TaxProductResourceCustomerDetailsResourceTaxIdType::ChVat,
            "cl_tin" => TaxProductResourceCustomerDetailsResourceTaxIdType::ClTin,
            "cm_niu" => TaxProductResourceCustomerDetailsResourceTaxIdType::CmNiu,
            "cn_tin" => TaxProductResourceCustomerDetailsResourceTaxIdType::CnTin,
            "co_nit" => TaxProductResourceCustomerDetailsResourceTaxIdType::CoNit,
            "cr_tin" => TaxProductResourceCustomerDetailsResourceTaxIdType::CrTin,
            "cv_nif" => TaxProductResourceCustomerDetailsResourceTaxIdType::CvNif,
            "de_stn" => TaxProductResourceCustomerDetailsResourceTaxIdType::DeStn,
            "do_rcn" => TaxProductResourceCustomerDetailsResourceTaxIdType::DoRcn,
            "ec_ruc" => TaxProductResourceCustomerDetailsResourceTaxIdType::EcRuc,
            "eg_tin" => TaxProductResourceCustomerDetailsResourceTaxIdType::EgTin,
            "es_cif" => TaxProductResourceCustomerDetailsResourceTaxIdType::EsCif,
            "et_tin" => TaxProductResourceCustomerDetailsResourceTaxIdType::EtTin,
            "eu_oss_vat" => TaxProductResourceCustomerDetailsResourceTaxIdType::EuOssVat,
            "eu_vat" => TaxProductResourceCustomerDetailsResourceTaxIdType::EuVat,
            "fo_vat" => TaxProductResourceCustomerDetailsResourceTaxIdType::FoVat,
            "gb_vat" => TaxProductResourceCustomerDetailsResourceTaxIdType::GbVat,
            "ge_vat" => TaxProductResourceCustomerDetailsResourceTaxIdType::GeVat,
            "gi_tin" => TaxProductResourceCustomerDetailsResourceTaxIdType::GiTin,
            "gn_nif" => TaxProductResourceCustomerDetailsResourceTaxIdType::GnNif,
            "hk_br" => TaxProductResourceCustomerDetailsResourceTaxIdType::HkBr,
            "hr_oib" => TaxProductResourceCustomerDetailsResourceTaxIdType::HrOib,
            "hu_tin" => TaxProductResourceCustomerDetailsResourceTaxIdType::HuTin,
            "ic_nif" => TaxProductResourceCustomerDetailsResourceTaxIdType::IcNif,
            "id_npwp" => TaxProductResourceCustomerDetailsResourceTaxIdType::IdNpwp,
            "il_vat" => TaxProductResourceCustomerDetailsResourceTaxIdType::IlVat,
            "in_gst" => TaxProductResourceCustomerDetailsResourceTaxIdType::InGst,
            "is_vat" => TaxProductResourceCustomerDetailsResourceTaxIdType::IsVat,
            "it_cf" => TaxProductResourceCustomerDetailsResourceTaxIdType::ItCf,
            "jp_cn" => TaxProductResourceCustomerDetailsResourceTaxIdType::JpCn,
            "jp_rn" => TaxProductResourceCustomerDetailsResourceTaxIdType::JpRn,
            "jp_trn" => TaxProductResourceCustomerDetailsResourceTaxIdType::JpTrn,
            "ke_pin" => TaxProductResourceCustomerDetailsResourceTaxIdType::KePin,
            "kg_tin" => TaxProductResourceCustomerDetailsResourceTaxIdType::KgTin,
            "kh_tin" => TaxProductResourceCustomerDetailsResourceTaxIdType::KhTin,
            "kr_brn" => TaxProductResourceCustomerDetailsResourceTaxIdType::KrBrn,
            "kz_bin" => TaxProductResourceCustomerDetailsResourceTaxIdType::KzBin,
            "la_tin" => TaxProductResourceCustomerDetailsResourceTaxIdType::LaTin,
            "li_uid" => TaxProductResourceCustomerDetailsResourceTaxIdType::LiUid,
            "li_vat" => TaxProductResourceCustomerDetailsResourceTaxIdType::LiVat,
            "lk_vat" => TaxProductResourceCustomerDetailsResourceTaxIdType::LkVat,
            "ma_vat" => TaxProductResourceCustomerDetailsResourceTaxIdType::MaVat,
            "md_vat" => TaxProductResourceCustomerDetailsResourceTaxIdType::MdVat,
            "me_pib" => TaxProductResourceCustomerDetailsResourceTaxIdType::MePib,
            "mk_vat" => TaxProductResourceCustomerDetailsResourceTaxIdType::MkVat,
            "mr_nif" => TaxProductResourceCustomerDetailsResourceTaxIdType::MrNif,
            "mx_rfc" => TaxProductResourceCustomerDetailsResourceTaxIdType::MxRfc,
            "my_frp" => TaxProductResourceCustomerDetailsResourceTaxIdType::MyFrp,
            "my_itn" => TaxProductResourceCustomerDetailsResourceTaxIdType::MyItn,
            "my_sst" => TaxProductResourceCustomerDetailsResourceTaxIdType::MySst,
            "ng_tin" => TaxProductResourceCustomerDetailsResourceTaxIdType::NgTin,
            "no_vat" => TaxProductResourceCustomerDetailsResourceTaxIdType::NoVat,
            "no_voec" => TaxProductResourceCustomerDetailsResourceTaxIdType::NoVoec,
            "np_pan" => TaxProductResourceCustomerDetailsResourceTaxIdType::NpPan,
            "nz_gst" => TaxProductResourceCustomerDetailsResourceTaxIdType::NzGst,
            "om_vat" => TaxProductResourceCustomerDetailsResourceTaxIdType::OmVat,
            "pe_ruc" => TaxProductResourceCustomerDetailsResourceTaxIdType::PeRuc,
            "ph_tin" => TaxProductResourceCustomerDetailsResourceTaxIdType::PhTin,
            "pl_nip" => TaxProductResourceCustomerDetailsResourceTaxIdType::PlNip,
            "py_ruc" => TaxProductResourceCustomerDetailsResourceTaxIdType::PyRuc,
            "ro_tin" => TaxProductResourceCustomerDetailsResourceTaxIdType::RoTin,
            "rs_pib" => TaxProductResourceCustomerDetailsResourceTaxIdType::RsPib,
            "ru_inn" => TaxProductResourceCustomerDetailsResourceTaxIdType::RuInn,
            "ru_kpp" => TaxProductResourceCustomerDetailsResourceTaxIdType::RuKpp,
            "sa_vat" => TaxProductResourceCustomerDetailsResourceTaxIdType::SaVat,
            "sg_gst" => TaxProductResourceCustomerDetailsResourceTaxIdType::SgGst,
            "sg_uen" => TaxProductResourceCustomerDetailsResourceTaxIdType::SgUen,
            "si_tin" => TaxProductResourceCustomerDetailsResourceTaxIdType::SiTin,
            "sn_ninea" => TaxProductResourceCustomerDetailsResourceTaxIdType::SnNinea,
            "sr_fin" => TaxProductResourceCustomerDetailsResourceTaxIdType::SrFin,
            "sv_nit" => TaxProductResourceCustomerDetailsResourceTaxIdType::SvNit,
            "th_vat" => TaxProductResourceCustomerDetailsResourceTaxIdType::ThVat,
            "tj_tin" => TaxProductResourceCustomerDetailsResourceTaxIdType::TjTin,
            "tr_tin" => TaxProductResourceCustomerDetailsResourceTaxIdType::TrTin,
            "tw_vat" => TaxProductResourceCustomerDetailsResourceTaxIdType::TwVat,
            "tz_vat" => TaxProductResourceCustomerDetailsResourceTaxIdType::TzVat,
            "ua_vat" => TaxProductResourceCustomerDetailsResourceTaxIdType::UaVat,
            "ug_tin" => TaxProductResourceCustomerDetailsResourceTaxIdType::UgTin,
            "unknown" => TaxProductResourceCustomerDetailsResourceTaxIdType::UnknownValue,
            "us_ein" => TaxProductResourceCustomerDetailsResourceTaxIdType::UsEin,
            "uy_ruc" => TaxProductResourceCustomerDetailsResourceTaxIdType::UyRuc,
            "uz_tin" => TaxProductResourceCustomerDetailsResourceTaxIdType::UzTin,
            "uz_vat" => TaxProductResourceCustomerDetailsResourceTaxIdType::UzVat,
            "ve_rif" => TaxProductResourceCustomerDetailsResourceTaxIdType::VeRif,
            "vn_tin" => TaxProductResourceCustomerDetailsResourceTaxIdType::VnTin,
            "za_vat" => TaxProductResourceCustomerDetailsResourceTaxIdType::ZaVat,
            "zm_tin" => TaxProductResourceCustomerDetailsResourceTaxIdType::ZmTin,
            "zw_tin" => TaxProductResourceCustomerDetailsResourceTaxIdType::ZwTin,
            other => TaxProductResourceCustomerDetailsResourceTaxIdType::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "tax"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxProductResourceTaxSettingsDefaultsProvider {
    Anrok,
    Avalara,
    Sphere,
    Stripe,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "tax"))]
impl TaxProductResourceTaxSettingsDefaultsProvider {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxProductResourceTaxSettingsDefaultsProvider::Anrok => "anrok",
            TaxProductResourceTaxSettingsDefaultsProvider::Avalara => "avalara",
            TaxProductResourceTaxSettingsDefaultsProvider::Sphere => "sphere",
            TaxProductResourceTaxSettingsDefaultsProvider::Stripe => "stripe",
            TaxProductResourceTaxSettingsDefaultsProvider::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "tax"))]
impl Default for TaxProductResourceTaxSettingsDefaultsProvider {
    fn default() -> Self {
        TaxProductResourceTaxSettingsDefaultsProvider::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "tax"))]
impl<'json> json::Decode<'json> for TaxProductResourceTaxSettingsDefaultsProvider {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TaxProductResourceTaxSettingsDefaultsProvider::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "anrok" => TaxProductResourceTaxSettingsDefaultsProvider::Anrok,
            "avalara" => TaxProductResourceTaxSettingsDefaultsProvider::Avalara,
            "sphere" => TaxProductResourceTaxSettingsDefaultsProvider::Sphere,
            "stripe" => TaxProductResourceTaxSettingsDefaultsProvider::Stripe,
            other => TaxProductResourceTaxSettingsDefaultsProvider::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "tax")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TaxProductResourceTaxTransactionResourceReversal {
    #[doc = "The `id` of the reversed `Transaction` object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub original_transaction: Option<String>,
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
    feature = "plans",
    feature = "prices",
    feature = "product_catalog",
    feature = "products",
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
pub struct TaxProductTaxDetailsResourceTaxDetails {
    #[doc = "The ID of a tax location with type `performance`, representing where the performance takes place."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub performance_location: Option<String>,
    #[doc = "A [tax code](https://docs.stripe.com/tax/tax-categories) ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
}

#[cfg(any(feature = "events", feature = "tax"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxSettingsObject {
    TaxSettings,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "tax"))]
impl TaxSettingsObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxSettingsObject::TaxSettings => "tax.settings",
            TaxSettingsObject::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "tax"))]
impl Default for TaxSettingsObject {
    fn default() -> Self {
        TaxSettingsObject::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "tax"))]
impl<'json> json::Decode<'json> for TaxSettingsObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TaxSettingsObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "tax.settings" => TaxSettingsObject::TaxSettings,
            other => TaxSettingsObject::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "tax"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TaxSettingsStatus {
    Active,
    Pending,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "tax"))]
impl TaxSettingsStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TaxSettingsStatus::Active => "active",
            TaxSettingsStatus::Pending => "pending",
            TaxSettingsStatus::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "tax"))]
impl Default for TaxSettingsStatus {
    fn default() -> Self {
        TaxSettingsStatus::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "tax"))]
impl<'json> json::Decode<'json> for TaxSettingsStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TaxSettingsStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "active" => TaxSettingsStatus::Active,
            "pending" => TaxSettingsStatus::Pending,
            other => TaxSettingsStatus::Unknown(other.to_string()),
        })
    }
}
