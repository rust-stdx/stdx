#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParamsCustomFieldsItemDropdownOptionsItem {
    #[doc = "The label for the option, displayed to the customer."]
    pub label: String,
    #[doc = "The value for this option, not displayed to the customer, used by your integration to reconcile the option selected b..."]
    pub value: String,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsCustomFieldsItemDropdownOptionsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("label", &self.label);
        form.field("value", &self.value);
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentLinkParamsCustomFieldsItemLabelType {
    Custom,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_links")]
impl CreatePaymentLinkParamsCustomFieldsItemLabelType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentLinkParamsCustomFieldsItemLabelType::Custom => "custom",
            CreatePaymentLinkParamsCustomFieldsItemLabelType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_links")]
impl<'json> json::Decode<'json> for CreatePaymentLinkParamsCustomFieldsItemLabelType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentLinkParamsCustomFieldsItemLabelType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "custom" => CreatePaymentLinkParamsCustomFieldsItemLabelType::Custom,
            other => CreatePaymentLinkParamsCustomFieldsItemLabelType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsCustomFieldsItemLabelType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParamsCustomTextAfterSubmit {
    #[doc = "Text can be up to 1200 characters in length."]
    pub message: String,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsCustomTextAfterSubmit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("message", &self.message);
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentLinkParamsInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay {
    Unnamed,
    ExcludeTax,
    IncludeInclusiveTax,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_links")]
impl CreatePaymentLinkParamsInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentLinkParamsInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::Unnamed => "",
            CreatePaymentLinkParamsInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::ExcludeTax => {
                "exclude_tax"
            }
            CreatePaymentLinkParamsInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::IncludeInclusiveTax => {
                "include_inclusive_tax"
            }
            CreatePaymentLinkParamsInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_links")]
impl<'json> json::Decode<'json> for CreatePaymentLinkParamsInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreatePaymentLinkParamsInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreatePaymentLinkParamsInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::Unnamed,
            "exclude_tax" => {
                CreatePaymentLinkParamsInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::ExcludeTax
            }
            "include_inclusive_tax" => {
                CreatePaymentLinkParamsInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::IncludeInclusiveTax
            }
            other => CreatePaymentLinkParamsInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParamsLineItemsItemAdjustableQuantity {
    #[doc = "Set to true if the quantity can be adjusted to any non-negative Integer."]
    pub enabled: bool,
    #[doc = "The maximum quantity the customer can purchase."]
    pub maximum: Option<i64>,
    #[doc = "The minimum quantity the customer can purchase."]
    pub minimum: Option<i64>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsLineItemsItemAdjustableQuantity {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("maximum", self.maximum.as_ref());
        form.field_opt("minimum", self.minimum.as_ref());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentLinkParamsLineItemsItemPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_links")]
impl CreatePaymentLinkParamsLineItemsItemPriceDataRecurringInterval {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentLinkParamsLineItemsItemPriceDataRecurringInterval::Day => "day",
            CreatePaymentLinkParamsLineItemsItemPriceDataRecurringInterval::Month => "month",
            CreatePaymentLinkParamsLineItemsItemPriceDataRecurringInterval::Week => "week",
            CreatePaymentLinkParamsLineItemsItemPriceDataRecurringInterval::Year => "year",
            CreatePaymentLinkParamsLineItemsItemPriceDataRecurringInterval::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_links")]
impl<'json> json::Decode<'json> for CreatePaymentLinkParamsLineItemsItemPriceDataRecurringInterval {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentLinkParamsLineItemsItemPriceDataRecurringInterval::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "day" => CreatePaymentLinkParamsLineItemsItemPriceDataRecurringInterval::Day,
            "month" => CreatePaymentLinkParamsLineItemsItemPriceDataRecurringInterval::Month,
            "week" => CreatePaymentLinkParamsLineItemsItemPriceDataRecurringInterval::Week,
            "year" => CreatePaymentLinkParamsLineItemsItemPriceDataRecurringInterval::Year,
            other => CreatePaymentLinkParamsLineItemsItemPriceDataRecurringInterval::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsLineItemsItemPriceDataRecurringInterval {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParamsManagedPayments {
    #[doc = "Set to `true` to enable [Managed Payments](https://docs.stripe.com/payments/managed-payments), Stripe's merchant of r..."]
    pub enabled: Option<bool>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsManagedPayments {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("enabled", self.enabled.as_ref());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem {
    AC,
    AD,
    AE,
    AF,
    AG,
    AI,
    AL,
    AM,
    AO,
    AQ,
    AR,
    AT,
    AU,
    AW,
    AX,
    AZ,
    BA,
    BB,
    BD,
    BE,
    BF,
    BG,
    BH,
    BI,
    BJ,
    BL,
    BM,
    BN,
    BO,
    BQ,
    BR,
    BS,
    BT,
    BV,
    BW,
    BY,
    BZ,
    CA,
    CD,
    CF,
    CG,
    CH,
    CI,
    CK,
    CL,
    CM,
    CN,
    CO,
    CR,
    CV,
    CW,
    CY,
    CZ,
    DE,
    DJ,
    DK,
    DM,
    DO,
    DZ,
    EC,
    EE,
    EG,
    EH,
    ER,
    ES,
    ET,
    FI,
    FJ,
    FK,
    FO,
    FR,
    GA,
    GB,
    GD,
    GE,
    GF,
    GG,
    GH,
    GI,
    GL,
    GM,
    GN,
    GP,
    GQ,
    GR,
    GS,
    GT,
    GU,
    GW,
    GY,
    HK,
    HN,
    HR,
    HT,
    HU,
    ID,
    IE,
    IL,
    IM,
    IN,
    IO,
    IQ,
    IS,
    IT,
    JE,
    JM,
    JO,
    JP,
    KE,
    KG,
    KH,
    KI,
    KM,
    KN,
    KR,
    KW,
    KY,
    KZ,
    LA,
    LB,
    LC,
    LI,
    LK,
    LR,
    LS,
    LT,
    LU,
    LV,
    LY,
    MA,
    MC,
    MD,
    ME,
    MF,
    MG,
    MK,
    ML,
    MM,
    MN,
    MO,
    MQ,
    MR,
    MS,
    MT,
    MU,
    MV,
    MW,
    MX,
    MY,
    MZ,
    NA,
    NC,
    NE,
    NG,
    NI,
    NL,
    NO,
    NP,
    NR,
    NU,
    NZ,
    OM,
    PA,
    PE,
    PF,
    PG,
    PH,
    PK,
    PL,
    PM,
    PN,
    PR,
    PS,
    PT,
    PY,
    QA,
    RE,
    RO,
    RS,
    RU,
    RW,
    SA,
    SB,
    SC,
    SD,
    SE,
    SG,
    SH,
    SI,
    SJ,
    SK,
    SL,
    SM,
    SN,
    SO,
    SR,
    SS,
    ST,
    SV,
    SX,
    SZ,
    TA,
    TC,
    TD,
    TF,
    TG,
    TH,
    TJ,
    TK,
    TL,
    TM,
    TN,
    TO,
    TR,
    TT,
    TV,
    TW,
    TZ,
    UA,
    UG,
    US,
    UY,
    UZ,
    VA,
    VC,
    VE,
    VG,
    VN,
    VU,
    WF,
    WS,
    XK,
    YE,
    YT,
    ZA,
    ZM,
    ZW,
    ZZ,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_links")]
impl CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::AC => "AC",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::AD => "AD",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::AE => "AE",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::AF => "AF",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::AG => "AG",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::AI => "AI",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::AL => "AL",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::AM => "AM",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::AO => "AO",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::AQ => "AQ",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::AR => "AR",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::AT => "AT",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::AU => "AU",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::AW => "AW",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::AX => "AX",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::AZ => "AZ",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BA => "BA",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BB => "BB",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BD => "BD",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BE => "BE",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BF => "BF",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BG => "BG",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BH => "BH",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BI => "BI",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BJ => "BJ",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BL => "BL",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BM => "BM",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BN => "BN",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BO => "BO",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BQ => "BQ",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BR => "BR",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BS => "BS",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BT => "BT",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BV => "BV",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BW => "BW",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BY => "BY",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BZ => "BZ",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::CA => "CA",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::CD => "CD",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::CF => "CF",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::CG => "CG",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::CH => "CH",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::CI => "CI",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::CK => "CK",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::CL => "CL",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::CM => "CM",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::CN => "CN",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::CO => "CO",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::CR => "CR",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::CV => "CV",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::CW => "CW",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::CY => "CY",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::CZ => "CZ",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::DE => "DE",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::DJ => "DJ",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::DK => "DK",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::DM => "DM",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::DO => "DO",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::DZ => "DZ",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::EC => "EC",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::EE => "EE",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::EG => "EG",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::EH => "EH",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::ER => "ER",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::ES => "ES",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::ET => "ET",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::FI => "FI",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::FJ => "FJ",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::FK => "FK",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::FO => "FO",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::FR => "FR",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GA => "GA",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GB => "GB",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GD => "GD",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GE => "GE",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GF => "GF",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GG => "GG",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GH => "GH",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GI => "GI",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GL => "GL",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GM => "GM",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GN => "GN",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GP => "GP",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GQ => "GQ",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GR => "GR",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GS => "GS",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GT => "GT",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GU => "GU",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GW => "GW",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GY => "GY",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::HK => "HK",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::HN => "HN",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::HR => "HR",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::HT => "HT",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::HU => "HU",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::ID => "ID",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::IE => "IE",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::IL => "IL",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::IM => "IM",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::IN => "IN",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::IO => "IO",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::IQ => "IQ",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::IS => "IS",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::IT => "IT",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::JE => "JE",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::JM => "JM",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::JO => "JO",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::JP => "JP",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::KE => "KE",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::KG => "KG",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::KH => "KH",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::KI => "KI",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::KM => "KM",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::KN => "KN",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::KR => "KR",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::KW => "KW",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::KY => "KY",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::KZ => "KZ",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::LA => "LA",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::LB => "LB",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::LC => "LC",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::LI => "LI",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::LK => "LK",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::LR => "LR",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::LS => "LS",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::LT => "LT",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::LU => "LU",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::LV => "LV",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::LY => "LY",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MA => "MA",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MC => "MC",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MD => "MD",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::ME => "ME",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MF => "MF",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MG => "MG",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MK => "MK",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::ML => "ML",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MM => "MM",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MN => "MN",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MO => "MO",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MQ => "MQ",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MR => "MR",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MS => "MS",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MT => "MT",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MU => "MU",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MV => "MV",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MW => "MW",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MX => "MX",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MY => "MY",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MZ => "MZ",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::NA => "NA",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::NC => "NC",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::NE => "NE",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::NG => "NG",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::NI => "NI",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::NL => "NL",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::NO => "NO",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::NP => "NP",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::NR => "NR",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::NU => "NU",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::NZ => "NZ",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::OM => "OM",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::PA => "PA",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::PE => "PE",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::PF => "PF",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::PG => "PG",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::PH => "PH",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::PK => "PK",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::PL => "PL",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::PM => "PM",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::PN => "PN",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::PR => "PR",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::PS => "PS",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::PT => "PT",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::PY => "PY",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::QA => "QA",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::RE => "RE",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::RO => "RO",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::RS => "RS",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::RU => "RU",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::RW => "RW",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SA => "SA",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SB => "SB",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SC => "SC",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SD => "SD",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SE => "SE",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SG => "SG",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SH => "SH",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SI => "SI",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SJ => "SJ",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SK => "SK",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SL => "SL",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SM => "SM",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SN => "SN",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SO => "SO",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SR => "SR",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SS => "SS",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::ST => "ST",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SV => "SV",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SX => "SX",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SZ => "SZ",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TA => "TA",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TC => "TC",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TD => "TD",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TF => "TF",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TG => "TG",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TH => "TH",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TJ => "TJ",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TK => "TK",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TL => "TL",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TM => "TM",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TN => "TN",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TO => "TO",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TR => "TR",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TT => "TT",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TV => "TV",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TW => "TW",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TZ => "TZ",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::UA => "UA",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::UG => "UG",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::US => "US",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::UY => "UY",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::UZ => "UZ",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::VA => "VA",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::VC => "VC",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::VE => "VE",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::VG => "VG",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::VN => "VN",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::VU => "VU",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::WF => "WF",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::WS => "WS",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::XK => "XK",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::YE => "YE",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::YT => "YT",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::ZA => "ZA",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::ZM => "ZM",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::ZW => "ZW",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::ZZ => "ZZ",
            CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_links")]
impl<'json> json::Decode<'json> for CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "AC" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::AC,
            "AD" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::AD,
            "AE" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::AE,
            "AF" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::AF,
            "AG" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::AG,
            "AI" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::AI,
            "AL" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::AL,
            "AM" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::AM,
            "AO" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::AO,
            "AQ" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::AQ,
            "AR" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::AR,
            "AT" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::AT,
            "AU" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::AU,
            "AW" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::AW,
            "AX" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::AX,
            "AZ" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::AZ,
            "BA" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BA,
            "BB" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BB,
            "BD" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BD,
            "BE" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BE,
            "BF" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BF,
            "BG" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BG,
            "BH" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BH,
            "BI" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BI,
            "BJ" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BJ,
            "BL" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BL,
            "BM" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BM,
            "BN" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BN,
            "BO" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BO,
            "BQ" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BQ,
            "BR" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BR,
            "BS" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BS,
            "BT" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BT,
            "BV" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BV,
            "BW" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BW,
            "BY" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BY,
            "BZ" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::BZ,
            "CA" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::CA,
            "CD" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::CD,
            "CF" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::CF,
            "CG" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::CG,
            "CH" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::CH,
            "CI" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::CI,
            "CK" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::CK,
            "CL" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::CL,
            "CM" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::CM,
            "CN" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::CN,
            "CO" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::CO,
            "CR" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::CR,
            "CV" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::CV,
            "CW" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::CW,
            "CY" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::CY,
            "CZ" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::CZ,
            "DE" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::DE,
            "DJ" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::DJ,
            "DK" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::DK,
            "DM" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::DM,
            "DO" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::DO,
            "DZ" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::DZ,
            "EC" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::EC,
            "EE" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::EE,
            "EG" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::EG,
            "EH" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::EH,
            "ER" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::ER,
            "ES" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::ES,
            "ET" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::ET,
            "FI" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::FI,
            "FJ" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::FJ,
            "FK" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::FK,
            "FO" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::FO,
            "FR" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::FR,
            "GA" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GA,
            "GB" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GB,
            "GD" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GD,
            "GE" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GE,
            "GF" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GF,
            "GG" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GG,
            "GH" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GH,
            "GI" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GI,
            "GL" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GL,
            "GM" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GM,
            "GN" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GN,
            "GP" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GP,
            "GQ" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GQ,
            "GR" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GR,
            "GS" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GS,
            "GT" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GT,
            "GU" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GU,
            "GW" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GW,
            "GY" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::GY,
            "HK" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::HK,
            "HN" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::HN,
            "HR" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::HR,
            "HT" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::HT,
            "HU" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::HU,
            "ID" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::ID,
            "IE" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::IE,
            "IL" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::IL,
            "IM" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::IM,
            "IN" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::IN,
            "IO" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::IO,
            "IQ" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::IQ,
            "IS" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::IS,
            "IT" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::IT,
            "JE" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::JE,
            "JM" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::JM,
            "JO" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::JO,
            "JP" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::JP,
            "KE" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::KE,
            "KG" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::KG,
            "KH" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::KH,
            "KI" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::KI,
            "KM" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::KM,
            "KN" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::KN,
            "KR" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::KR,
            "KW" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::KW,
            "KY" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::KY,
            "KZ" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::KZ,
            "LA" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::LA,
            "LB" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::LB,
            "LC" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::LC,
            "LI" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::LI,
            "LK" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::LK,
            "LR" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::LR,
            "LS" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::LS,
            "LT" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::LT,
            "LU" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::LU,
            "LV" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::LV,
            "LY" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::LY,
            "MA" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MA,
            "MC" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MC,
            "MD" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MD,
            "ME" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::ME,
            "MF" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MF,
            "MG" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MG,
            "MK" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MK,
            "ML" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::ML,
            "MM" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MM,
            "MN" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MN,
            "MO" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MO,
            "MQ" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MQ,
            "MR" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MR,
            "MS" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MS,
            "MT" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MT,
            "MU" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MU,
            "MV" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MV,
            "MW" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MW,
            "MX" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MX,
            "MY" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MY,
            "MZ" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::MZ,
            "NA" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::NA,
            "NC" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::NC,
            "NE" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::NE,
            "NG" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::NG,
            "NI" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::NI,
            "NL" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::NL,
            "NO" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::NO,
            "NP" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::NP,
            "NR" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::NR,
            "NU" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::NU,
            "NZ" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::NZ,
            "OM" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::OM,
            "PA" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::PA,
            "PE" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::PE,
            "PF" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::PF,
            "PG" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::PG,
            "PH" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::PH,
            "PK" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::PK,
            "PL" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::PL,
            "PM" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::PM,
            "PN" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::PN,
            "PR" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::PR,
            "PS" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::PS,
            "PT" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::PT,
            "PY" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::PY,
            "QA" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::QA,
            "RE" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::RE,
            "RO" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::RO,
            "RS" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::RS,
            "RU" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::RU,
            "RW" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::RW,
            "SA" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SA,
            "SB" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SB,
            "SC" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SC,
            "SD" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SD,
            "SE" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SE,
            "SG" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SG,
            "SH" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SH,
            "SI" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SI,
            "SJ" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SJ,
            "SK" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SK,
            "SL" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SL,
            "SM" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SM,
            "SN" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SN,
            "SO" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SO,
            "SR" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SR,
            "SS" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SS,
            "ST" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::ST,
            "SV" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SV,
            "SX" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SX,
            "SZ" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::SZ,
            "TA" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TA,
            "TC" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TC,
            "TD" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TD,
            "TF" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TF,
            "TG" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TG,
            "TH" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TH,
            "TJ" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TJ,
            "TK" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TK,
            "TL" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TL,
            "TM" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TM,
            "TN" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TN,
            "TO" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TO,
            "TR" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TR,
            "TT" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TT,
            "TV" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TV,
            "TW" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TW,
            "TZ" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::TZ,
            "UA" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::UA,
            "UG" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::UG,
            "US" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::US,
            "UY" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::UY,
            "UZ" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::UZ,
            "VA" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::VA,
            "VC" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::VC,
            "VE" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::VE,
            "VG" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::VG,
            "VN" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::VN,
            "VU" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::VU,
            "WF" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::WF,
            "WS" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::WS,
            "XK" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::XK,
            "YE" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::YE,
            "YT" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::YT,
            "ZA" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::ZA,
            "ZM" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::ZM,
            "ZW" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::ZW,
            "ZZ" => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::ZZ,
            other => CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsShippingAddressCollectionAllowedCountriesItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParamsShippingOptionsItem {
    #[doc = "The ID of the Shipping Rate to use for this shipping option."]
    pub shipping_rate: Option<String>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsShippingOptionsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("shipping_rate", self.shipping_rate.as_ref());
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentLinksResourceBusinessName {
    #[doc = "Indicates whether business name collection is enabled for the payment link."]
    #[json(default)]
    pub enabled: bool,
    #[doc = "Whether the customer is required to complete the field before checking out."]
    #[json(default)]
    pub optional: bool,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentLinksResourceCustomFieldsDropdownOption {
    #[doc = "The label for the option, displayed to the customer."]
    #[json(default)]
    pub label: String,
    #[doc = "The value for this option, not displayed to the customer, used by your integration to reconcile the option selected b..."]
    #[json(default)]
    pub value: String,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentLinksResourceCustomTextPosition {
    #[doc = "Text can be up to 1200 characters in length."]
    #[json(default)]
    pub message: String,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentLinksResourceIndividualName {
    #[doc = "Indicates whether individual name collection is enabled for the payment link."]
    #[json(default)]
    pub enabled: bool,
    #[doc = "Whether the customer is required to complete the field before checking out."]
    #[json(default)]
    pub optional: bool,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    OffSession,
    OnSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
impl PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentLinksResourcePaymentIntentDataSetupFutureUsage::OffSession => "off_session",
            PaymentLinksResourcePaymentIntentDataSetupFutureUsage::OnSession => "on_session",
            PaymentLinksResourcePaymentIntentDataSetupFutureUsage::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
impl<'json> json::Decode<'json> for PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentLinksResourcePaymentIntentDataSetupFutureUsage::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "off_session" => PaymentLinksResourcePaymentIntentDataSetupFutureUsage::OffSession,
            "on_session" => PaymentLinksResourcePaymentIntentDataSetupFutureUsage::OnSession,
            other => PaymentLinksResourcePaymentIntentDataSetupFutureUsage::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentLinksResourceTaxIdCollectionRequired {
    IfSupported,
    Never,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
impl PaymentLinksResourceTaxIdCollectionRequired {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentLinksResourceTaxIdCollectionRequired::IfSupported => "if_supported",
            PaymentLinksResourceTaxIdCollectionRequired::Never => "never",
            PaymentLinksResourceTaxIdCollectionRequired::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
impl Default for PaymentLinksResourceTaxIdCollectionRequired {
    fn default() -> Self {
        PaymentLinksResourceTaxIdCollectionRequired::Unknown(String::new())
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
impl<'json> json::Decode<'json> for PaymentLinksResourceTaxIdCollectionRequired {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentLinksResourceTaxIdCollectionRequired::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "if_supported" => PaymentLinksResourceTaxIdCollectionRequired::IfSupported,
            "never" => PaymentLinksResourceTaxIdCollectionRequired::Never,
            other => PaymentLinksResourceTaxIdCollectionRequired::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentLinkParamsPaymentMethodOptionsCardRestrictions {
    #[doc = "The card brands to block."]
    pub brands_blocked: Option<json::OwnedValue>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for UpdatePaymentLinkParamsPaymentMethodOptionsCardRestrictions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        let _ = form;
    }
}
