#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage {
    None,
    OffSession,
    OnSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage::None => "none",
            CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage::OffSession => "off_session",
            CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage::OnSession => "on_session",
            CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "none" => CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage::None,
            "off_session" => CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage::OffSession,
            "on_session" => CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage::OnSession,
            other => CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutCardInstallmentsOptions {
    #[doc = "Indicates if installments are enabled"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypesItem {
    Aba,
    Iban,
    Sepa,
    SortCode,
    Spei,
    Swift,
    Zengin,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypesItem::Aba => "aba",
            CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypesItem::Iban => "iban",
            CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypesItem::Sepa => "sepa",
            CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypesItem::SortCode => "sort_code",
            CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypesItem::Spei => "spei",
            CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypesItem::Swift => "swift",
            CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypesItem::Zengin => "zengin",
            CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypesItem::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypesItem::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "aba" => CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypesItem::Aba,
            "iban" => CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypesItem::Iban,
            "sepa" => CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypesItem::Sepa,
            "sort_code" => CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypesItem::SortCode,
            "spei" => CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypesItem::Spei,
            "swift" => CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypesItem::Swift,
            "zengin" => CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypesItem::Zengin,
            other => CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypesItem::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum CheckoutPixPaymentMethodOptionsAmountIncludesIof {
    Always,
    Never,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl CheckoutPixPaymentMethodOptionsAmountIncludesIof {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CheckoutPixPaymentMethodOptionsAmountIncludesIof::Always => "always",
            CheckoutPixPaymentMethodOptionsAmountIncludesIof::Never => "never",
            CheckoutPixPaymentMethodOptionsAmountIncludesIof::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for CheckoutPixPaymentMethodOptionsAmountIncludesIof {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CheckoutPixPaymentMethodOptionsAmountIncludesIof::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "always" => CheckoutPixPaymentMethodOptionsAmountIncludesIof::Always,
            "never" => CheckoutPixPaymentMethodOptionsAmountIncludesIof::Never,
            other => CheckoutPixPaymentMethodOptionsAmountIncludesIof::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum CheckoutSessionBillingAddressCollection {
    Auto,
    Required,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl CheckoutSessionBillingAddressCollection {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CheckoutSessionBillingAddressCollection::Auto => "auto",
            CheckoutSessionBillingAddressCollection::Required => "required",
            CheckoutSessionBillingAddressCollection::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for CheckoutSessionBillingAddressCollection {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CheckoutSessionBillingAddressCollection::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "auto" => CheckoutSessionBillingAddressCollection::Auto,
            "required" => CheckoutSessionBillingAddressCollection::Required,
            other => CheckoutSessionBillingAddressCollection::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum CheckoutSessionLocale {
    Auto,
    Bg,
    Cs,
    Da,
    De,
    El,
    En,
    EnGB,
    Es,
    Es419,
    Et,
    Fi,
    Fil,
    Fr,
    FrCA,
    Hr,
    Hu,
    Id,
    It,
    Ja,
    Ko,
    Lt,
    Lv,
    Ms,
    Mt,
    Nb,
    Nl,
    Pl,
    Pt,
    PtBR,
    Ro,
    Ru,
    Sk,
    Sl,
    Sv,
    Th,
    Tr,
    Vi,
    Zh,
    ZhHK,
    ZhTW,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl CheckoutSessionLocale {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CheckoutSessionLocale::Auto => "auto",
            CheckoutSessionLocale::Bg => "bg",
            CheckoutSessionLocale::Cs => "cs",
            CheckoutSessionLocale::Da => "da",
            CheckoutSessionLocale::De => "de",
            CheckoutSessionLocale::El => "el",
            CheckoutSessionLocale::En => "en",
            CheckoutSessionLocale::EnGB => "en-GB",
            CheckoutSessionLocale::Es => "es",
            CheckoutSessionLocale::Es419 => "es-419",
            CheckoutSessionLocale::Et => "et",
            CheckoutSessionLocale::Fi => "fi",
            CheckoutSessionLocale::Fil => "fil",
            CheckoutSessionLocale::Fr => "fr",
            CheckoutSessionLocale::FrCA => "fr-CA",
            CheckoutSessionLocale::Hr => "hr",
            CheckoutSessionLocale::Hu => "hu",
            CheckoutSessionLocale::Id => "id",
            CheckoutSessionLocale::It => "it",
            CheckoutSessionLocale::Ja => "ja",
            CheckoutSessionLocale::Ko => "ko",
            CheckoutSessionLocale::Lt => "lt",
            CheckoutSessionLocale::Lv => "lv",
            CheckoutSessionLocale::Ms => "ms",
            CheckoutSessionLocale::Mt => "mt",
            CheckoutSessionLocale::Nb => "nb",
            CheckoutSessionLocale::Nl => "nl",
            CheckoutSessionLocale::Pl => "pl",
            CheckoutSessionLocale::Pt => "pt",
            CheckoutSessionLocale::PtBR => "pt-BR",
            CheckoutSessionLocale::Ro => "ro",
            CheckoutSessionLocale::Ru => "ru",
            CheckoutSessionLocale::Sk => "sk",
            CheckoutSessionLocale::Sl => "sl",
            CheckoutSessionLocale::Sv => "sv",
            CheckoutSessionLocale::Th => "th",
            CheckoutSessionLocale::Tr => "tr",
            CheckoutSessionLocale::Vi => "vi",
            CheckoutSessionLocale::Zh => "zh",
            CheckoutSessionLocale::ZhHK => "zh-HK",
            CheckoutSessionLocale::ZhTW => "zh-TW",
            CheckoutSessionLocale::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for CheckoutSessionLocale {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CheckoutSessionLocale::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "auto" => CheckoutSessionLocale::Auto,
            "bg" => CheckoutSessionLocale::Bg,
            "cs" => CheckoutSessionLocale::Cs,
            "da" => CheckoutSessionLocale::Da,
            "de" => CheckoutSessionLocale::De,
            "el" => CheckoutSessionLocale::El,
            "en" => CheckoutSessionLocale::En,
            "en-GB" => CheckoutSessionLocale::EnGB,
            "es" => CheckoutSessionLocale::Es,
            "es-419" => CheckoutSessionLocale::Es419,
            "et" => CheckoutSessionLocale::Et,
            "fi" => CheckoutSessionLocale::Fi,
            "fil" => CheckoutSessionLocale::Fil,
            "fr" => CheckoutSessionLocale::Fr,
            "fr-CA" => CheckoutSessionLocale::FrCA,
            "hr" => CheckoutSessionLocale::Hr,
            "hu" => CheckoutSessionLocale::Hu,
            "id" => CheckoutSessionLocale::Id,
            "it" => CheckoutSessionLocale::It,
            "ja" => CheckoutSessionLocale::Ja,
            "ko" => CheckoutSessionLocale::Ko,
            "lt" => CheckoutSessionLocale::Lt,
            "lv" => CheckoutSessionLocale::Lv,
            "ms" => CheckoutSessionLocale::Ms,
            "mt" => CheckoutSessionLocale::Mt,
            "nb" => CheckoutSessionLocale::Nb,
            "nl" => CheckoutSessionLocale::Nl,
            "pl" => CheckoutSessionLocale::Pl,
            "pt" => CheckoutSessionLocale::Pt,
            "pt-BR" => CheckoutSessionLocale::PtBR,
            "ro" => CheckoutSessionLocale::Ro,
            "ru" => CheckoutSessionLocale::Ru,
            "sk" => CheckoutSessionLocale::Sk,
            "sl" => CheckoutSessionLocale::Sl,
            "sv" => CheckoutSessionLocale::Sv,
            "th" => CheckoutSessionLocale::Th,
            "tr" => CheckoutSessionLocale::Tr,
            "vi" => CheckoutSessionLocale::Vi,
            "zh" => CheckoutSessionLocale::Zh,
            "zh-HK" => CheckoutSessionLocale::ZhHK,
            "zh-TW" => CheckoutSessionLocale::ZhTW,
            other => CheckoutSessionLocale::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum CheckoutSessionMode {
    Payment,
    Setup,
    Subscription,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl CheckoutSessionMode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CheckoutSessionMode::Payment => "payment",
            CheckoutSessionMode::Setup => "setup",
            CheckoutSessionMode::Subscription => "subscription",
            CheckoutSessionMode::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl Default for CheckoutSessionMode {
    fn default() -> Self {
        CheckoutSessionMode::Unknown(String::new())
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for CheckoutSessionMode {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CheckoutSessionMode::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "payment" => CheckoutSessionMode::Payment,
            "setup" => CheckoutSessionMode::Setup,
            "subscription" => CheckoutSessionMode::Subscription,
            other => CheckoutSessionMode::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsBrandingSettingsBorderStyle {
    Unnamed,
    Pill,
    Rectangular,
    Rounded,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsBrandingSettingsBorderStyle {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsBrandingSettingsBorderStyle::Unnamed => "",
            CreateCheckoutSessionParamsBrandingSettingsBorderStyle::Pill => "pill",
            CreateCheckoutSessionParamsBrandingSettingsBorderStyle::Rectangular => "rectangular",
            CreateCheckoutSessionParamsBrandingSettingsBorderStyle::Rounded => "rounded",
            CreateCheckoutSessionParamsBrandingSettingsBorderStyle::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsBrandingSettingsBorderStyle {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsBrandingSettingsBorderStyle::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreateCheckoutSessionParamsBrandingSettingsBorderStyle::Unnamed,
            "pill" => CreateCheckoutSessionParamsBrandingSettingsBorderStyle::Pill,
            "rectangular" => CreateCheckoutSessionParamsBrandingSettingsBorderStyle::Rectangular,
            "rounded" => CreateCheckoutSessionParamsBrandingSettingsBorderStyle::Rounded,
            other => CreateCheckoutSessionParamsBrandingSettingsBorderStyle::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsBrandingSettingsBorderStyle {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsBrandingSettingsFontFamily {
    Unnamed,
    BeVietnamPro,
    Bitter,
    ChakraPetch,
    Default,
    Hahmlet,
    Inconsolata,
    Inter,
    Lato,
    Lora,
    MPlus1Code,
    Montserrat,
    NotoSans,
    NotoSansJp,
    NotoSerif,
    Nunito,
    OpenSans,
    Pridi,
    PtSans,
    PtSerif,
    Raleway,
    Roboto,
    RobotoSlab,
    SourceSansPro,
    TitilliumWeb,
    UbuntuMono,
    ZenMaruGothic,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsBrandingSettingsFontFamily {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsBrandingSettingsFontFamily::Unnamed => "",
            CreateCheckoutSessionParamsBrandingSettingsFontFamily::BeVietnamPro => "be_vietnam_pro",
            CreateCheckoutSessionParamsBrandingSettingsFontFamily::Bitter => "bitter",
            CreateCheckoutSessionParamsBrandingSettingsFontFamily::ChakraPetch => "chakra_petch",
            CreateCheckoutSessionParamsBrandingSettingsFontFamily::Default => "default",
            CreateCheckoutSessionParamsBrandingSettingsFontFamily::Hahmlet => "hahmlet",
            CreateCheckoutSessionParamsBrandingSettingsFontFamily::Inconsolata => "inconsolata",
            CreateCheckoutSessionParamsBrandingSettingsFontFamily::Inter => "inter",
            CreateCheckoutSessionParamsBrandingSettingsFontFamily::Lato => "lato",
            CreateCheckoutSessionParamsBrandingSettingsFontFamily::Lora => "lora",
            CreateCheckoutSessionParamsBrandingSettingsFontFamily::MPlus1Code => "m_plus_1_code",
            CreateCheckoutSessionParamsBrandingSettingsFontFamily::Montserrat => "montserrat",
            CreateCheckoutSessionParamsBrandingSettingsFontFamily::NotoSans => "noto_sans",
            CreateCheckoutSessionParamsBrandingSettingsFontFamily::NotoSansJp => "noto_sans_jp",
            CreateCheckoutSessionParamsBrandingSettingsFontFamily::NotoSerif => "noto_serif",
            CreateCheckoutSessionParamsBrandingSettingsFontFamily::Nunito => "nunito",
            CreateCheckoutSessionParamsBrandingSettingsFontFamily::OpenSans => "open_sans",
            CreateCheckoutSessionParamsBrandingSettingsFontFamily::Pridi => "pridi",
            CreateCheckoutSessionParamsBrandingSettingsFontFamily::PtSans => "pt_sans",
            CreateCheckoutSessionParamsBrandingSettingsFontFamily::PtSerif => "pt_serif",
            CreateCheckoutSessionParamsBrandingSettingsFontFamily::Raleway => "raleway",
            CreateCheckoutSessionParamsBrandingSettingsFontFamily::Roboto => "roboto",
            CreateCheckoutSessionParamsBrandingSettingsFontFamily::RobotoSlab => "roboto_slab",
            CreateCheckoutSessionParamsBrandingSettingsFontFamily::SourceSansPro => "source_sans_pro",
            CreateCheckoutSessionParamsBrandingSettingsFontFamily::TitilliumWeb => "titillium_web",
            CreateCheckoutSessionParamsBrandingSettingsFontFamily::UbuntuMono => "ubuntu_mono",
            CreateCheckoutSessionParamsBrandingSettingsFontFamily::ZenMaruGothic => "zen_maru_gothic",
            CreateCheckoutSessionParamsBrandingSettingsFontFamily::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsBrandingSettingsFontFamily {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsBrandingSettingsFontFamily::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreateCheckoutSessionParamsBrandingSettingsFontFamily::Unnamed,
            "be_vietnam_pro" => CreateCheckoutSessionParamsBrandingSettingsFontFamily::BeVietnamPro,
            "bitter" => CreateCheckoutSessionParamsBrandingSettingsFontFamily::Bitter,
            "chakra_petch" => CreateCheckoutSessionParamsBrandingSettingsFontFamily::ChakraPetch,
            "default" => CreateCheckoutSessionParamsBrandingSettingsFontFamily::Default,
            "hahmlet" => CreateCheckoutSessionParamsBrandingSettingsFontFamily::Hahmlet,
            "inconsolata" => CreateCheckoutSessionParamsBrandingSettingsFontFamily::Inconsolata,
            "inter" => CreateCheckoutSessionParamsBrandingSettingsFontFamily::Inter,
            "lato" => CreateCheckoutSessionParamsBrandingSettingsFontFamily::Lato,
            "lora" => CreateCheckoutSessionParamsBrandingSettingsFontFamily::Lora,
            "m_plus_1_code" => CreateCheckoutSessionParamsBrandingSettingsFontFamily::MPlus1Code,
            "montserrat" => CreateCheckoutSessionParamsBrandingSettingsFontFamily::Montserrat,
            "noto_sans" => CreateCheckoutSessionParamsBrandingSettingsFontFamily::NotoSans,
            "noto_sans_jp" => CreateCheckoutSessionParamsBrandingSettingsFontFamily::NotoSansJp,
            "noto_serif" => CreateCheckoutSessionParamsBrandingSettingsFontFamily::NotoSerif,
            "nunito" => CreateCheckoutSessionParamsBrandingSettingsFontFamily::Nunito,
            "open_sans" => CreateCheckoutSessionParamsBrandingSettingsFontFamily::OpenSans,
            "pridi" => CreateCheckoutSessionParamsBrandingSettingsFontFamily::Pridi,
            "pt_sans" => CreateCheckoutSessionParamsBrandingSettingsFontFamily::PtSans,
            "pt_serif" => CreateCheckoutSessionParamsBrandingSettingsFontFamily::PtSerif,
            "raleway" => CreateCheckoutSessionParamsBrandingSettingsFontFamily::Raleway,
            "roboto" => CreateCheckoutSessionParamsBrandingSettingsFontFamily::Roboto,
            "roboto_slab" => CreateCheckoutSessionParamsBrandingSettingsFontFamily::RobotoSlab,
            "source_sans_pro" => CreateCheckoutSessionParamsBrandingSettingsFontFamily::SourceSansPro,
            "titillium_web" => CreateCheckoutSessionParamsBrandingSettingsFontFamily::TitilliumWeb,
            "ubuntu_mono" => CreateCheckoutSessionParamsBrandingSettingsFontFamily::UbuntuMono,
            "zen_maru_gothic" => CreateCheckoutSessionParamsBrandingSettingsFontFamily::ZenMaruGothic,
            other => CreateCheckoutSessionParamsBrandingSettingsFontFamily::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsBrandingSettingsFontFamily {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsCustomTextAfterSubmit {
    #[doc = "Text can be up to 1200 characters in length."]
    pub message: String,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsCustomTextAfterSubmit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("message", &self.message);
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem {
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
    Boleto,
    Card,
    Cashapp,
    Crypto,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    KakaoPay,
    Klarna,
    Konbini,
    KrCard,
    MbWay,
    Mobilepay,
    Multibanco,
    NaverPay,
    NzBankAccount,
    Oxxo,
    P24,
    PayByBank,
    Payco,
    Paynow,
    Paypal,
    Paypay,
    Payto,
    Pix,
    Promptpay,
    RevolutPay,
    SamsungPay,
    Satispay,
    Scalapay,
    SepaDebit,
    Sequra,
    Sofort,
    Sunbit,
    Swish,
    Twint,
    Upi,
    UsBankAccount,
    WechatPay,
    Zip,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::AcssDebit => "acss_debit",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Affirm => "affirm",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::AfterpayClearpay => "afterpay_clearpay",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Alipay => "alipay",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Alma => "alma",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::AmazonPay => "amazon_pay",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::AuBecsDebit => "au_becs_debit",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::BacsDebit => "bacs_debit",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Bancontact => "bancontact",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Billie => "billie",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Bizum => "bizum",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Blik => "blik",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Boleto => "boleto",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Card => "card",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Cashapp => "cashapp",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Crypto => "crypto",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::CustomerBalance => "customer_balance",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Eps => "eps",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Fpx => "fpx",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Giropay => "giropay",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Grabpay => "grabpay",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Ideal => "ideal",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::KakaoPay => "kakao_pay",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Klarna => "klarna",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Konbini => "konbini",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::KrCard => "kr_card",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::MbWay => "mb_way",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Mobilepay => "mobilepay",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Multibanco => "multibanco",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::NaverPay => "naver_pay",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::NzBankAccount => "nz_bank_account",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Oxxo => "oxxo",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::P24 => "p24",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::PayByBank => "pay_by_bank",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Payco => "payco",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Paynow => "paynow",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Paypal => "paypal",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Paypay => "paypay",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Payto => "payto",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Pix => "pix",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Promptpay => "promptpay",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::RevolutPay => "revolut_pay",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::SamsungPay => "samsung_pay",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Satispay => "satispay",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Scalapay => "scalapay",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::SepaDebit => "sepa_debit",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Sequra => "sequra",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Sofort => "sofort",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Sunbit => "sunbit",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Swish => "swish",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Twint => "twint",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Upi => "upi",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::UsBankAccount => "us_bank_account",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::WechatPay => "wechat_pay",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Zip => "zip",
            CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "acss_debit" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::AcssDebit,
            "affirm" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Affirm,
            "afterpay_clearpay" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::AfterpayClearpay,
            "alipay" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Alipay,
            "alma" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Alma,
            "amazon_pay" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::AmazonPay,
            "au_becs_debit" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::AuBecsDebit,
            "bacs_debit" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::BacsDebit,
            "bancontact" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Bancontact,
            "billie" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Billie,
            "bizum" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Bizum,
            "blik" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Blik,
            "boleto" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Boleto,
            "card" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Card,
            "cashapp" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Cashapp,
            "crypto" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Crypto,
            "customer_balance" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::CustomerBalance,
            "eps" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Eps,
            "fpx" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Fpx,
            "giropay" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Giropay,
            "grabpay" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Grabpay,
            "ideal" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Ideal,
            "kakao_pay" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::KakaoPay,
            "klarna" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Klarna,
            "konbini" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Konbini,
            "kr_card" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::KrCard,
            "mb_way" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::MbWay,
            "mobilepay" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Mobilepay,
            "multibanco" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Multibanco,
            "naver_pay" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::NaverPay,
            "nz_bank_account" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::NzBankAccount,
            "oxxo" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Oxxo,
            "p24" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::P24,
            "pay_by_bank" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::PayByBank,
            "payco" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Payco,
            "paynow" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Paynow,
            "paypal" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Paypal,
            "paypay" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Paypay,
            "payto" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Payto,
            "pix" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Pix,
            "promptpay" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Promptpay,
            "revolut_pay" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::RevolutPay,
            "samsung_pay" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::SamsungPay,
            "satispay" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Satispay,
            "scalapay" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Scalapay,
            "sepa_debit" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::SepaDebit,
            "sequra" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Sequra,
            "sofort" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Sofort,
            "sunbit" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Sunbit,
            "swish" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Swish,
            "twint" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Twint,
            "upi" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Upi,
            "us_bank_account" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::UsBankAccount,
            "wechat_pay" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::WechatPay,
            "zip" => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Zip,
            other => CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsExcludedPaymentMethodTypesItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsLineItemsItemAdjustableQuantity {
    #[doc = "Set to true if the quantity can be adjusted to any non-negative integer."]
    pub enabled: bool,
    #[doc = "The maximum quantity the customer can purchase for the Checkout Session."]
    pub maximum: Option<i64>,
    #[doc = "The minimum quantity the customer must purchase for the Checkout Session."]
    pub minimum: Option<i64>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsLineItemsItemAdjustableQuantity {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("maximum", self.maximum.as_ref());
        form.field_opt("minimum", self.minimum.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsNameCollectionBusiness {
    #[doc = "Enable business name collection on the Checkout Session."]
    pub enabled: bool,
    #[doc = "Whether the customer is required to provide a business name before completing the Checkout Session."]
    pub optional: Option<bool>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsNameCollectionBusiness {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
        form.field_opt("optional", self.optional.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Business => {
                "business"
            }
            CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Personal => {
                "personal"
            }
            CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Unknown(value) => {
                value
            }
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json>
    for CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "business" => {
                CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Business
            }
            "personal" => {
                CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Personal
            }
            other => CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode
    for CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitSetupFutureUsage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitSetupFutureUsage::None => "none",
            CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitSetupFutureUsage::OffSession => "off_session",
            CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitSetupFutureUsage::OnSession => "on_session",
            CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitSetupFutureUsage::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitSetupFutureUsage {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitSetupFutureUsage::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "none" => CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitSetupFutureUsage::None,
            "off_session" => CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitSetupFutureUsage::OffSession,
            "on_session" => CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitSetupFutureUsage::OnSession,
            other => {
                CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitSetupFutureUsage::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitSetupFutureUsage {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPaymentMethodOptionsAmazonPaySetupFutureUsage {
    None,
    OffSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPaymentMethodOptionsAmazonPaySetupFutureUsage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPaymentMethodOptionsAmazonPaySetupFutureUsage::None => "none",
            CreateCheckoutSessionParamsPaymentMethodOptionsAmazonPaySetupFutureUsage::OffSession => "off_session",
            CreateCheckoutSessionParamsPaymentMethodOptionsAmazonPaySetupFutureUsage::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsPaymentMethodOptionsAmazonPaySetupFutureUsage {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCheckoutSessionParamsPaymentMethodOptionsAmazonPaySetupFutureUsage::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "none" => CreateCheckoutSessionParamsPaymentMethodOptionsAmazonPaySetupFutureUsage::None,
            "off_session" => CreateCheckoutSessionParamsPaymentMethodOptionsAmazonPaySetupFutureUsage::OffSession,
            other => {
                CreateCheckoutSessionParamsPaymentMethodOptionsAmazonPaySetupFutureUsage::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsAmazonPaySetupFutureUsage {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsPayByBank {}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsPayByBank {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        let _ = form;
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsAmountType {
    Unnamed,
    Fixed,
    Maximum,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsAmountType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsAmountType::Unnamed => "",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsAmountType::Fixed => "fixed",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsAmountType::Maximum => "maximum",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsAmountType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsAmountType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsAmountType::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsAmountType::Unnamed,
            "fixed" => CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsAmountType::Fixed,
            "maximum" => CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsAmountType::Maximum,
            other => {
                CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsAmountType::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsAmountType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem {
    Balances,
    Ownership,
    Transactions,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Balances => "balances",
            CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Ownership => "ownership",
            CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Transactions => "transactions",
            CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json>
    for CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "balances" => CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Balances,
            "ownership" => CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Ownership,
            "transactions" => CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Transactions,
            other => CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode
    for CreateCheckoutSessionParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPermissionsUpdateShippingDetails {
    ClientOnly,
    ServerOnly,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPermissionsUpdateShippingDetails {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPermissionsUpdateShippingDetails::ClientOnly => "client_only",
            CreateCheckoutSessionParamsPermissionsUpdateShippingDetails::ServerOnly => "server_only",
            CreateCheckoutSessionParamsPermissionsUpdateShippingDetails::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsPermissionsUpdateShippingDetails {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsPermissionsUpdateShippingDetails::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "client_only" => CreateCheckoutSessionParamsPermissionsUpdateShippingDetails::ClientOnly,
            "server_only" => CreateCheckoutSessionParamsPermissionsUpdateShippingDetails::ServerOnly,
            other => CreateCheckoutSessionParamsPermissionsUpdateShippingDetails::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPermissionsUpdateShippingDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsRedirectOnCompletion {
    Always,
    IfRequired,
    Never,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsRedirectOnCompletion {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsRedirectOnCompletion::Always => "always",
            CreateCheckoutSessionParamsRedirectOnCompletion::IfRequired => "if_required",
            CreateCheckoutSessionParamsRedirectOnCompletion::Never => "never",
            CreateCheckoutSessionParamsRedirectOnCompletion::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsRedirectOnCompletion {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsRedirectOnCompletion::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "always" => CreateCheckoutSessionParamsRedirectOnCompletion::Always,
            "if_required" => CreateCheckoutSessionParamsRedirectOnCompletion::IfRequired,
            "never" => CreateCheckoutSessionParamsRedirectOnCompletion::Never,
            other => CreateCheckoutSessionParamsRedirectOnCompletion::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsRedirectOnCompletion {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsSubscriptionDataProrationBehavior {
    CreateProrations,
    None,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsSubscriptionDataProrationBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsSubscriptionDataProrationBehavior::CreateProrations => "create_prorations",
            CreateCheckoutSessionParamsSubscriptionDataProrationBehavior::None => "none",
            CreateCheckoutSessionParamsSubscriptionDataProrationBehavior::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsSubscriptionDataProrationBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsSubscriptionDataProrationBehavior::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "create_prorations" => CreateCheckoutSessionParamsSubscriptionDataProrationBehavior::CreateProrations,
            "none" => CreateCheckoutSessionParamsSubscriptionDataProrationBehavior::None,
            other => CreateCheckoutSessionParamsSubscriptionDataProrationBehavior::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsSubscriptionDataProrationBehavior {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsSubscriptionDataTransferData {
    #[doc = "A non-negative decimal between 0 and 100, with at most two decimal places."]
    pub amount_percent: Option<f64>,
    #[doc = "ID of an existing, connected Stripe account."]
    pub destination: String,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsSubscriptionDataTransferData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount_percent", self.amount_percent.as_ref());
        form.field("destination", &self.destination);
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsUiMode {
    Elements,
    EmbeddedPage,
    Form,
    HostedPage,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsUiMode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsUiMode::Elements => "elements",
            CreateCheckoutSessionParamsUiMode::EmbeddedPage => "embedded_page",
            CreateCheckoutSessionParamsUiMode::Form => "form",
            CreateCheckoutSessionParamsUiMode::HostedPage => "hosted_page",
            CreateCheckoutSessionParamsUiMode::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsUiMode {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsUiMode::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "elements" => CreateCheckoutSessionParamsUiMode::Elements,
            "embedded_page" => CreateCheckoutSessionParamsUiMode::EmbeddedPage,
            "form" => CreateCheckoutSessionParamsUiMode::Form,
            "hosted_page" => CreateCheckoutSessionParamsUiMode::HostedPage,
            other => CreateCheckoutSessionParamsUiMode::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsUiMode {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct GetCheckoutSessionParams {
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for GetCheckoutSessionParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentPagesCheckoutSessionAutomaticTaxStatus {
    Complete,
    Failed,
    RequiresLocationInputs,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl PaymentPagesCheckoutSessionAutomaticTaxStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentPagesCheckoutSessionAutomaticTaxStatus::Complete => "complete",
            PaymentPagesCheckoutSessionAutomaticTaxStatus::Failed => "failed",
            PaymentPagesCheckoutSessionAutomaticTaxStatus::RequiresLocationInputs => "requires_location_inputs",
            PaymentPagesCheckoutSessionAutomaticTaxStatus::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for PaymentPagesCheckoutSessionAutomaticTaxStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentPagesCheckoutSessionAutomaticTaxStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "complete" => PaymentPagesCheckoutSessionAutomaticTaxStatus::Complete,
            "failed" => PaymentPagesCheckoutSessionAutomaticTaxStatus::Failed,
            "requires_location_inputs" => PaymentPagesCheckoutSessionAutomaticTaxStatus::RequiresLocationInputs,
            other => PaymentPagesCheckoutSessionAutomaticTaxStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentPagesCheckoutSessionCustomFieldsText {
    #[doc = "The value that pre-fills the field on the payment page."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[doc = "The maximum character length constraint for the customer's input."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub maximum_length: Option<i64>,
    #[doc = "The minimum character length requirement for the customer's input."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<i64>,
    #[doc = "The value entered by the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition {
    Auto,
    Hidden,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition::Auto => "auto",
            PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition::Hidden => "hidden",
            PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl Default for PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition {
    fn default() -> Self {
        PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition::Unknown(String::new())
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "auto" => PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition::Auto,
            "hidden" => PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition::Hidden,
            other => PaymentPagesCheckoutSessionPaymentMethodReuseAgreementPosition::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentPagesCheckoutSessionPermissionsUpdateShippingDetails {
    ClientOnly,
    ServerOnly,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl PaymentPagesCheckoutSessionPermissionsUpdateShippingDetails {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentPagesCheckoutSessionPermissionsUpdateShippingDetails::ClientOnly => "client_only",
            PaymentPagesCheckoutSessionPermissionsUpdateShippingDetails::ServerOnly => "server_only",
            PaymentPagesCheckoutSessionPermissionsUpdateShippingDetails::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for PaymentPagesCheckoutSessionPermissionsUpdateShippingDetails {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentPagesCheckoutSessionPermissionsUpdateShippingDetails::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "client_only" => PaymentPagesCheckoutSessionPermissionsUpdateShippingDetails::ClientOnly,
            "server_only" => PaymentPagesCheckoutSessionPermissionsUpdateShippingDetails::ServerOnly,
            other => PaymentPagesCheckoutSessionPermissionsUpdateShippingDetails::Unknown(other.to_string()),
        })
    }
}
