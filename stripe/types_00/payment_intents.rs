#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsAutomaticPaymentMethodsAllowRedirects {
    Always,
    Never,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsAutomaticPaymentMethodsAllowRedirects {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsAutomaticPaymentMethodsAllowRedirects::Always => "always",
            CreatePaymentIntentParamsAutomaticPaymentMethodsAllowRedirects::Never => "never",
            CreatePaymentIntentParamsAutomaticPaymentMethodsAllowRedirects::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsAutomaticPaymentMethodsAllowRedirects {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsAutomaticPaymentMethodsAllowRedirects::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "always" => CreatePaymentIntentParamsAutomaticPaymentMethodsAllowRedirects::Always,
            "never" => CreatePaymentIntentParamsAutomaticPaymentMethodsAllowRedirects::Never,
            other => CreatePaymentIntentParamsAutomaticPaymentMethodsAllowRedirects::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsAutomaticPaymentMethodsAllowRedirects {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsExcludedPaymentMethodTypesItem {
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

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsExcludedPaymentMethodTypesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::AcssDebit => "acss_debit",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Affirm => "affirm",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::AfterpayClearpay => "afterpay_clearpay",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Alipay => "alipay",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Alma => "alma",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::AmazonPay => "amazon_pay",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::AuBecsDebit => "au_becs_debit",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::BacsDebit => "bacs_debit",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Bancontact => "bancontact",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Billie => "billie",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Bizum => "bizum",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Blik => "blik",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Boleto => "boleto",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Card => "card",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Cashapp => "cashapp",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Crypto => "crypto",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::CustomerBalance => "customer_balance",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Eps => "eps",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Fpx => "fpx",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Giropay => "giropay",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Grabpay => "grabpay",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Ideal => "ideal",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::KakaoPay => "kakao_pay",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Klarna => "klarna",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Konbini => "konbini",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::KrCard => "kr_card",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::MbWay => "mb_way",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Mobilepay => "mobilepay",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Multibanco => "multibanco",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::NaverPay => "naver_pay",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::NzBankAccount => "nz_bank_account",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Oxxo => "oxxo",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::P24 => "p24",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::PayByBank => "pay_by_bank",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Payco => "payco",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Paynow => "paynow",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Paypal => "paypal",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Paypay => "paypay",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Payto => "payto",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Pix => "pix",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Promptpay => "promptpay",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::RevolutPay => "revolut_pay",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::SamsungPay => "samsung_pay",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Satispay => "satispay",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Scalapay => "scalapay",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::SepaDebit => "sepa_debit",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Sequra => "sequra",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Sofort => "sofort",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Sunbit => "sunbit",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Swish => "swish",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Twint => "twint",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Upi => "upi",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::UsBankAccount => "us_bank_account",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::WechatPay => "wechat_pay",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Zip => "zip",
            CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsExcludedPaymentMethodTypesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "acss_debit" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::AcssDebit,
            "affirm" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Affirm,
            "afterpay_clearpay" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::AfterpayClearpay,
            "alipay" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Alipay,
            "alma" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Alma,
            "amazon_pay" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::AmazonPay,
            "au_becs_debit" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::AuBecsDebit,
            "bacs_debit" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::BacsDebit,
            "bancontact" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Bancontact,
            "billie" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Billie,
            "bizum" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Bizum,
            "blik" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Blik,
            "boleto" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Boleto,
            "card" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Card,
            "cashapp" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Cashapp,
            "crypto" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Crypto,
            "customer_balance" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::CustomerBalance,
            "eps" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Eps,
            "fpx" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Fpx,
            "giropay" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Giropay,
            "grabpay" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Grabpay,
            "ideal" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Ideal,
            "kakao_pay" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::KakaoPay,
            "klarna" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Klarna,
            "konbini" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Konbini,
            "kr_card" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::KrCard,
            "mb_way" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::MbWay,
            "mobilepay" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Mobilepay,
            "multibanco" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Multibanco,
            "naver_pay" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::NaverPay,
            "nz_bank_account" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::NzBankAccount,
            "oxxo" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Oxxo,
            "p24" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::P24,
            "pay_by_bank" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::PayByBank,
            "payco" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Payco,
            "paynow" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Paynow,
            "paypal" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Paypal,
            "paypay" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Paypay,
            "payto" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Payto,
            "pix" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Pix,
            "promptpay" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Promptpay,
            "revolut_pay" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::RevolutPay,
            "samsung_pay" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::SamsungPay,
            "satispay" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Satispay,
            "scalapay" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Scalapay,
            "sepa_debit" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::SepaDebit,
            "sequra" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Sequra,
            "sofort" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Sofort,
            "sunbit" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Sunbit,
            "swish" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Swish,
            "twint" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Twint,
            "upi" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Upi,
            "us_bank_account" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::UsBankAccount,
            "wechat_pay" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::WechatPay,
            "zip" => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Zip,
            other => CreatePaymentIntentParamsExcludedPaymentMethodTypesItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsExcludedPaymentMethodTypesItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodDataAuBecsDebit {
    #[doc = "The account number for the bank account."]
    pub account_number: String,
    #[doc = "Bank-State-Branch number of the bank account."]
    pub bsb_number: String,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodDataAuBecsDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("account_number", &self.account_number);
        form.field("bsb_number", &self.bsb_number);
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodDataEpsBank {
    ArzteUndApothekerBank,
    AustrianAnadiBankAg,
    BankAustria,
    BankhausCarlSpangler,
    BankhausSchelhammerUndSchatteraAg,
    BawagPskAg,
    BksBankAg,
    BrullKallmusBankAg,
    BtvVierLanderBank,
    CapitalBankGraweGruppeAg,
    DeutscheBankAg,
    Dolomitenbank,
    EasybankAg,
    ErsteBankUndSparkassen,
    HypoAlpeadriabankInternationalAg,
    HypoBankBurgenlandAktiengesellschaft,
    HypoNoeLbFurNiederosterreichUWien,
    HypoOberosterreichSalzburgSteiermark,
    HypoTirolBankAg,
    HypoVorarlbergBankAg,
    MarchfelderBank,
    OberbankAg,
    RaiffeisenBankengruppeOsterreich,
    SchoellerbankAg,
    SpardaBankWien,
    VolksbankGruppe,
    VolkskreditbankAg,
    VrBankBraunau,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodDataEpsBank {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodDataEpsBank::ArzteUndApothekerBank => "arzte_und_apotheker_bank",
            CreatePaymentIntentParamsPaymentMethodDataEpsBank::AustrianAnadiBankAg => "austrian_anadi_bank_ag",
            CreatePaymentIntentParamsPaymentMethodDataEpsBank::BankAustria => "bank_austria",
            CreatePaymentIntentParamsPaymentMethodDataEpsBank::BankhausCarlSpangler => "bankhaus_carl_spangler",
            CreatePaymentIntentParamsPaymentMethodDataEpsBank::BankhausSchelhammerUndSchatteraAg => {
                "bankhaus_schelhammer_und_schattera_ag"
            }
            CreatePaymentIntentParamsPaymentMethodDataEpsBank::BawagPskAg => "bawag_psk_ag",
            CreatePaymentIntentParamsPaymentMethodDataEpsBank::BksBankAg => "bks_bank_ag",
            CreatePaymentIntentParamsPaymentMethodDataEpsBank::BrullKallmusBankAg => "brull_kallmus_bank_ag",
            CreatePaymentIntentParamsPaymentMethodDataEpsBank::BtvVierLanderBank => "btv_vier_lander_bank",
            CreatePaymentIntentParamsPaymentMethodDataEpsBank::CapitalBankGraweGruppeAg => {
                "capital_bank_grawe_gruppe_ag"
            }
            CreatePaymentIntentParamsPaymentMethodDataEpsBank::DeutscheBankAg => "deutsche_bank_ag",
            CreatePaymentIntentParamsPaymentMethodDataEpsBank::Dolomitenbank => "dolomitenbank",
            CreatePaymentIntentParamsPaymentMethodDataEpsBank::EasybankAg => "easybank_ag",
            CreatePaymentIntentParamsPaymentMethodDataEpsBank::ErsteBankUndSparkassen => "erste_bank_und_sparkassen",
            CreatePaymentIntentParamsPaymentMethodDataEpsBank::HypoAlpeadriabankInternationalAg => {
                "hypo_alpeadriabank_international_ag"
            }
            CreatePaymentIntentParamsPaymentMethodDataEpsBank::HypoBankBurgenlandAktiengesellschaft => {
                "hypo_bank_burgenland_aktiengesellschaft"
            }
            CreatePaymentIntentParamsPaymentMethodDataEpsBank::HypoNoeLbFurNiederosterreichUWien => {
                "hypo_noe_lb_fur_niederosterreich_u_wien"
            }
            CreatePaymentIntentParamsPaymentMethodDataEpsBank::HypoOberosterreichSalzburgSteiermark => {
                "hypo_oberosterreich_salzburg_steiermark"
            }
            CreatePaymentIntentParamsPaymentMethodDataEpsBank::HypoTirolBankAg => "hypo_tirol_bank_ag",
            CreatePaymentIntentParamsPaymentMethodDataEpsBank::HypoVorarlbergBankAg => "hypo_vorarlberg_bank_ag",
            CreatePaymentIntentParamsPaymentMethodDataEpsBank::MarchfelderBank => "marchfelder_bank",
            CreatePaymentIntentParamsPaymentMethodDataEpsBank::OberbankAg => "oberbank_ag",
            CreatePaymentIntentParamsPaymentMethodDataEpsBank::RaiffeisenBankengruppeOsterreich => {
                "raiffeisen_bankengruppe_osterreich"
            }
            CreatePaymentIntentParamsPaymentMethodDataEpsBank::SchoellerbankAg => "schoellerbank_ag",
            CreatePaymentIntentParamsPaymentMethodDataEpsBank::SpardaBankWien => "sparda_bank_wien",
            CreatePaymentIntentParamsPaymentMethodDataEpsBank::VolksbankGruppe => "volksbank_gruppe",
            CreatePaymentIntentParamsPaymentMethodDataEpsBank::VolkskreditbankAg => "volkskreditbank_ag",
            CreatePaymentIntentParamsPaymentMethodDataEpsBank::VrBankBraunau => "vr_bank_braunau",
            CreatePaymentIntentParamsPaymentMethodDataEpsBank::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodDataEpsBank {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsPaymentMethodDataEpsBank::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "arzte_und_apotheker_bank" => CreatePaymentIntentParamsPaymentMethodDataEpsBank::ArzteUndApothekerBank,
            "austrian_anadi_bank_ag" => CreatePaymentIntentParamsPaymentMethodDataEpsBank::AustrianAnadiBankAg,
            "bank_austria" => CreatePaymentIntentParamsPaymentMethodDataEpsBank::BankAustria,
            "bankhaus_carl_spangler" => CreatePaymentIntentParamsPaymentMethodDataEpsBank::BankhausCarlSpangler,
            "bankhaus_schelhammer_und_schattera_ag" => {
                CreatePaymentIntentParamsPaymentMethodDataEpsBank::BankhausSchelhammerUndSchatteraAg
            }
            "bawag_psk_ag" => CreatePaymentIntentParamsPaymentMethodDataEpsBank::BawagPskAg,
            "bks_bank_ag" => CreatePaymentIntentParamsPaymentMethodDataEpsBank::BksBankAg,
            "brull_kallmus_bank_ag" => CreatePaymentIntentParamsPaymentMethodDataEpsBank::BrullKallmusBankAg,
            "btv_vier_lander_bank" => CreatePaymentIntentParamsPaymentMethodDataEpsBank::BtvVierLanderBank,
            "capital_bank_grawe_gruppe_ag" => {
                CreatePaymentIntentParamsPaymentMethodDataEpsBank::CapitalBankGraweGruppeAg
            }
            "deutsche_bank_ag" => CreatePaymentIntentParamsPaymentMethodDataEpsBank::DeutscheBankAg,
            "dolomitenbank" => CreatePaymentIntentParamsPaymentMethodDataEpsBank::Dolomitenbank,
            "easybank_ag" => CreatePaymentIntentParamsPaymentMethodDataEpsBank::EasybankAg,
            "erste_bank_und_sparkassen" => CreatePaymentIntentParamsPaymentMethodDataEpsBank::ErsteBankUndSparkassen,
            "hypo_alpeadriabank_international_ag" => {
                CreatePaymentIntentParamsPaymentMethodDataEpsBank::HypoAlpeadriabankInternationalAg
            }
            "hypo_bank_burgenland_aktiengesellschaft" => {
                CreatePaymentIntentParamsPaymentMethodDataEpsBank::HypoBankBurgenlandAktiengesellschaft
            }
            "hypo_noe_lb_fur_niederosterreich_u_wien" => {
                CreatePaymentIntentParamsPaymentMethodDataEpsBank::HypoNoeLbFurNiederosterreichUWien
            }
            "hypo_oberosterreich_salzburg_steiermark" => {
                CreatePaymentIntentParamsPaymentMethodDataEpsBank::HypoOberosterreichSalzburgSteiermark
            }
            "hypo_tirol_bank_ag" => CreatePaymentIntentParamsPaymentMethodDataEpsBank::HypoTirolBankAg,
            "hypo_vorarlberg_bank_ag" => CreatePaymentIntentParamsPaymentMethodDataEpsBank::HypoVorarlbergBankAg,
            "marchfelder_bank" => CreatePaymentIntentParamsPaymentMethodDataEpsBank::MarchfelderBank,
            "oberbank_ag" => CreatePaymentIntentParamsPaymentMethodDataEpsBank::OberbankAg,
            "raiffeisen_bankengruppe_osterreich" => {
                CreatePaymentIntentParamsPaymentMethodDataEpsBank::RaiffeisenBankengruppeOsterreich
            }
            "schoellerbank_ag" => CreatePaymentIntentParamsPaymentMethodDataEpsBank::SchoellerbankAg,
            "sparda_bank_wien" => CreatePaymentIntentParamsPaymentMethodDataEpsBank::SpardaBankWien,
            "volksbank_gruppe" => CreatePaymentIntentParamsPaymentMethodDataEpsBank::VolksbankGruppe,
            "volkskreditbank_ag" => CreatePaymentIntentParamsPaymentMethodDataEpsBank::VolkskreditbankAg,
            "vr_bank_braunau" => CreatePaymentIntentParamsPaymentMethodDataEpsBank::VrBankBraunau,
            other => CreatePaymentIntentParamsPaymentMethodDataEpsBank::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodDataEpsBank {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodDataKlarnaDob {
    #[doc = "The day of birth, between 1 and 31."]
    pub day: i64,
    #[doc = "The month of birth, between 1 and 12."]
    pub month: i64,
    #[doc = "The four-digit year of birth."]
    pub year: i64,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodDataKlarnaDob {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("day", &self.day);
        form.field("month", &self.month);
        form.field("year", &self.year);
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage {
    Unnamed,
    None,
    OffSession,
    OnSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage::Unnamed => "",
            CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage::None => "none",
            CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage::OffSession => "off_session",
            CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage::OnSession => "on_session",
            CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage::Unnamed,
            "none" => CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage::None,
            "off_session" => CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage::OffSession,
            "on_session" => CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage::OnSession,
            other => CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod {
    Unnamed,
    Manual,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod::Unnamed => "",
            CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod::Manual => "manual",
            CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod::Unnamed,
            "manual" => CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod::Manual,
            other => CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType {
    Unnamed,
    ChEin,
    DeHrb,
    DkCvr,
    EsCif,
    FiTunnus,
    FrSiren,
    FrSiret,
    ItRea,
    NlKvk,
    NoOrgNumber,
    NoPno,
    SeOrgNumber,
    SePno,
    UkCrn,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::Unnamed => "",
            CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::ChEin => "ch_ein",
            CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::DeHrb => "de_hrb",
            CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::DkCvr => "dk_cvr",
            CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::EsCif => "es_cif",
            CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::FiTunnus => "fi_tunnus",
            CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::FrSiren => "fr_siren",
            CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::FrSiret => "fr_siret",
            CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::ItRea => "it_rea",
            CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::NlKvk => "nl_kvk",
            CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::NoOrgNumber => {
                "no_org_number"
            }
            CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::NoPno => "no_pno",
            CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::SeOrgNumber => {
                "se_org_number"
            }
            CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::SePno => "se_pno",
            CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::UkCrn => "uk_crn",
            CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::Unnamed,
            "ch_ein" => CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::ChEin,
            "de_hrb" => CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::DeHrb,
            "dk_cvr" => CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::DkCvr,
            "es_cif" => CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::EsCif,
            "fi_tunnus" => CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::FiTunnus,
            "fr_siren" => CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::FrSiren,
            "fr_siret" => CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::FrSiret,
            "it_rea" => CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::ItRea,
            "nl_kvk" => CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::NlKvk,
            "no_org_number" => {
                CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::NoOrgNumber
            }
            "no_pno" => CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::NoPno,
            "se_org_number" => {
                CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::SeOrgNumber
            }
            "se_pno" => CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::SePno,
            "uk_crn" => CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::UkCrn,
            other => CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork {
    Amex,
    CartesBancaires,
    Diners,
    Discover,
    EftposAu,
    Girocard,
    Interac,
    Jcb,
    Link,
    Mastercard,
    Unionpay,
    UnknownValue,
    Visa,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork::Amex => "amex",
            CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork::CartesBancaires => "cartes_bancaires",
            CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork::Diners => "diners",
            CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork::Discover => "discover",
            CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork::EftposAu => "eftpos_au",
            CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork::Girocard => "girocard",
            CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork::Interac => "interac",
            CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork::Jcb => "jcb",
            CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork::Link => "link",
            CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork::Mastercard => "mastercard",
            CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork::Unionpay => "unionpay",
            CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork::UnknownValue => "unknown",
            CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork::Visa => "visa",
            CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "amex" => CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork::Amex,
            "cartes_bancaires" => CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork::CartesBancaires,
            "diners" => CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork::Diners,
            "discover" => CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork::Discover,
            "eftpos_au" => CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork::EftposAu,
            "girocard" => CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork::Girocard,
            "interac" => CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork::Interac,
            "jcb" => CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork::Jcb,
            "link" => CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork::Link,
            "mastercard" => CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork::Mastercard,
            "unionpay" => CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork::Unionpay,
            "unknown" => CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork::UnknownValue,
            "visa" => CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork::Visa,
            other => CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsCardRequestExtendedAuthorization {
    IfAvailable,
    Never,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsCardRequestExtendedAuthorization {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsCardRequestExtendedAuthorization::IfAvailable => {
                "if_available"
            }
            CreatePaymentIntentParamsPaymentMethodOptionsCardRequestExtendedAuthorization::Never => "never",
            CreatePaymentIntentParamsPaymentMethodOptionsCardRequestExtendedAuthorization::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsCardRequestExtendedAuthorization {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreatePaymentIntentParamsPaymentMethodOptionsCardRequestExtendedAuthorization::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "if_available" => {
                CreatePaymentIntentParamsPaymentMethodOptionsCardRequestExtendedAuthorization::IfAvailable
            }
            "never" => CreatePaymentIntentParamsPaymentMethodOptionsCardRequestExtendedAuthorization::Never,
            other => CreatePaymentIntentParamsPaymentMethodOptionsCardRequestExtendedAuthorization::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsCardRequestExtendedAuthorization {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus {
    A,
    C,
    I,
    N,
    R,
    U,
    Y,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::A => "A",
            CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::C => "C",
            CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::I => "I",
            CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::N => "N",
            CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::R => "R",
            CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::U => "U",
            CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::Y => "Y",
            CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "A" => CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::A,
            "C" => CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::C,
            "I" => CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::I,
            "N" => CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::N,
            "R" => CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::R,
            "U" => CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::U,
            "Y" => CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::Y,
            other => {
                CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo {
    N0,
    N1,
    N2,
    N3,
    N4,
    A,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::N0 => "0",
            CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::N1 => "1",
            CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::N2 => "2",
            CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::N3 => "3",
            CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::N4 => "4",
            CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::A => "A",
            CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json>
    for CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "0" => CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::N0,
            "1" => CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::N1,
            "2" => CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::N2,
            "3" => CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::N3,
            "4" => CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::N4,
            "A" => CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::A,
            other => CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode
    for CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsKlarnaSetupFutureUsage {
    None,
    OffSession,
    OnSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsKlarnaSetupFutureUsage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaSetupFutureUsage::None => "none",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaSetupFutureUsage::OffSession => "off_session",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaSetupFutureUsage::OnSession => "on_session",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaSetupFutureUsage::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsKlarnaSetupFutureUsage {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsPaymentMethodOptionsKlarnaSetupFutureUsage::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "none" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaSetupFutureUsage::None,
            "off_session" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaSetupFutureUsage::OffSession,
            "on_session" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaSetupFutureUsage::OnSession,
            other => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaSetupFutureUsage::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsKlarnaSetupFutureUsage {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule {
    Unnamed,
    Adhoc,
    Annual,
    Daily,
    Fortnightly,
    Monthly,
    Quarterly,
    SemiAnnual,
    Weekly,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Unnamed => "",
            CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Adhoc => "adhoc",
            CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Annual => "annual",
            CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Daily => "daily",
            CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Fortnightly => {
                "fortnightly"
            }
            CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Monthly => "monthly",
            CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Quarterly => "quarterly",
            CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::SemiAnnual => {
                "semi_annual"
            }
            CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Weekly => "weekly",
            CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Unnamed,
            "adhoc" => CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Adhoc,
            "annual" => CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Annual,
            "daily" => CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Daily,
            "fortnightly" => {
                CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Fortnightly
            }
            "monthly" => CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Monthly,
            "quarterly" => CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Quarterly,
            "semi_annual" => {
                CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::SemiAnnual
            }
            "weekly" => CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Weekly,
            other => CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Balances => "balances",
            CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Ownership => "ownership",
            CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::PaymentMethod => "payment_method",
            CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Transactions => "transactions",
            CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json>
    for CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "balances" => CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Balances,
            "ownership" => CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Ownership,
            "payment_method" => CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::PaymentMethod,
            "transactions" => CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Transactions,
            other => CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode
    for CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem {
    Balances,
    Ownership,
    Transactions,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Balances => "balances",
            CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Ownership => "ownership",
            CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Transactions => "transactions",
            CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json>
    for CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "balances" => {
                CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Balances
            }
            "ownership" => {
                CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Ownership
            }
            "transactions" => {
                CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Transactions
            }
            other => {
                CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem::Unknown(
                    other.to_string(),
                )
            }
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode
    for CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod {
    Unnamed,
    Paper,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod::Unnamed => "",
            CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod::Paper => "paper",
            CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod::Unknown(
                value,
            ) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json>
    for CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod::Unnamed,
            "paper" => CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod::Paper,
            other => CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode
    for CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsSetupFutureUsage {
    OffSession,
    OnSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsSetupFutureUsage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsSetupFutureUsage::OffSession => "off_session",
            CreatePaymentIntentParamsSetupFutureUsage::OnSession => "on_session",
            CreatePaymentIntentParamsSetupFutureUsage::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsSetupFutureUsage {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsSetupFutureUsage::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "off_session" => CreatePaymentIntentParamsSetupFutureUsage::OffSession,
            "on_session" => CreatePaymentIntentParamsSetupFutureUsage::OnSession,
            other => CreatePaymentIntentParamsSetupFutureUsage::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsSetupFutureUsage {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsTransferDataPaymentData {
    #[doc = "An arbitrary string attached to the destination payment."]
    pub description: Option<String>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsTransferDataPaymentData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("description", self.description.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
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
pub enum PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects {
    Always,
    Never,
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
impl PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects::Always => "always",
            PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects::Never => "never",
            PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "always" => PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects::Always,
            "never" => PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects::Never,
            other => PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects::Unknown(other.to_string()),
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
pub struct PaymentIntentNextActionAlipayHandleRedirect {
    #[doc = "The native data to be used with Alipay SDK you must redirect your customer to in order to authenticate the payment in..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub native_data: Option<String>,
    #[doc = "The native URL you must redirect your customer to in order to authenticate the payment in an iOS App."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub native_url: Option<String>,
    #[doc = "If the customer does not exit their browser while authenticating, they will be redirected to this specified URL after..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[doc = "The URL you must redirect your customer to in order to authenticate the payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
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
pub enum PaymentIntentNextActionDisplayBankTransferInstructionsType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
    UsBankTransfer,
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
impl PaymentIntentNextActionDisplayBankTransferInstructionsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentIntentNextActionDisplayBankTransferInstructionsType::EuBankTransfer => "eu_bank_transfer",
            PaymentIntentNextActionDisplayBankTransferInstructionsType::GbBankTransfer => "gb_bank_transfer",
            PaymentIntentNextActionDisplayBankTransferInstructionsType::JpBankTransfer => "jp_bank_transfer",
            PaymentIntentNextActionDisplayBankTransferInstructionsType::MxBankTransfer => "mx_bank_transfer",
            PaymentIntentNextActionDisplayBankTransferInstructionsType::UsBankTransfer => "us_bank_transfer",
            PaymentIntentNextActionDisplayBankTransferInstructionsType::Unknown(value) => value,
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
impl Default for PaymentIntentNextActionDisplayBankTransferInstructionsType {
    fn default() -> Self {
        PaymentIntentNextActionDisplayBankTransferInstructionsType::Unknown(String::new())
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
impl<'json> json::Decode<'json> for PaymentIntentNextActionDisplayBankTransferInstructionsType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentIntentNextActionDisplayBankTransferInstructionsType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "eu_bank_transfer" => PaymentIntentNextActionDisplayBankTransferInstructionsType::EuBankTransfer,
            "gb_bank_transfer" => PaymentIntentNextActionDisplayBankTransferInstructionsType::GbBankTransfer,
            "jp_bank_transfer" => PaymentIntentNextActionDisplayBankTransferInstructionsType::JpBankTransfer,
            "mx_bank_transfer" => PaymentIntentNextActionDisplayBankTransferInstructionsType::MxBankTransfer,
            "us_bank_transfer" => PaymentIntentNextActionDisplayBankTransferInstructionsType::UsBankTransfer,
            other => PaymentIntentNextActionDisplayBankTransferInstructionsType::Unknown(other.to_string()),
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
pub struct PaymentIntentNextActionKonbiniFamilymart {
    #[doc = "The confirmation number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub confirmation_number: Option<String>,
    #[doc = "The payment code."]
    #[json(default)]
    pub payment_code: String,
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
pub enum PaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
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
impl PaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod::Automatic => "automatic",
            PaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod::Instant => "instant",
            PaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod::Microdeposits => "microdeposits",
            PaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "automatic" => PaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod::Automatic,
            "instant" => PaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod::Instant,
            "microdeposits" => PaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod::Microdeposits,
            other => PaymentIntentPaymentMethodOptionsAcssDebitVerificationMethod::Unknown(other.to_string()),
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
pub enum PaymentIntentPaymentMethodOptionsCardNetwork {
    Amex,
    CartesBancaires,
    Diners,
    Discover,
    EftposAu,
    Girocard,
    Interac,
    Jcb,
    Link,
    Mastercard,
    Unionpay,
    UnknownValue,
    Visa,
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
impl PaymentIntentPaymentMethodOptionsCardNetwork {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentIntentPaymentMethodOptionsCardNetwork::Amex => "amex",
            PaymentIntentPaymentMethodOptionsCardNetwork::CartesBancaires => "cartes_bancaires",
            PaymentIntentPaymentMethodOptionsCardNetwork::Diners => "diners",
            PaymentIntentPaymentMethodOptionsCardNetwork::Discover => "discover",
            PaymentIntentPaymentMethodOptionsCardNetwork::EftposAu => "eftpos_au",
            PaymentIntentPaymentMethodOptionsCardNetwork::Girocard => "girocard",
            PaymentIntentPaymentMethodOptionsCardNetwork::Interac => "interac",
            PaymentIntentPaymentMethodOptionsCardNetwork::Jcb => "jcb",
            PaymentIntentPaymentMethodOptionsCardNetwork::Link => "link",
            PaymentIntentPaymentMethodOptionsCardNetwork::Mastercard => "mastercard",
            PaymentIntentPaymentMethodOptionsCardNetwork::Unionpay => "unionpay",
            PaymentIntentPaymentMethodOptionsCardNetwork::UnknownValue => "unknown",
            PaymentIntentPaymentMethodOptionsCardNetwork::Visa => "visa",
            PaymentIntentPaymentMethodOptionsCardNetwork::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PaymentIntentPaymentMethodOptionsCardNetwork {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentIntentPaymentMethodOptionsCardNetwork::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "amex" => PaymentIntentPaymentMethodOptionsCardNetwork::Amex,
            "cartes_bancaires" => PaymentIntentPaymentMethodOptionsCardNetwork::CartesBancaires,
            "diners" => PaymentIntentPaymentMethodOptionsCardNetwork::Diners,
            "discover" => PaymentIntentPaymentMethodOptionsCardNetwork::Discover,
            "eftpos_au" => PaymentIntentPaymentMethodOptionsCardNetwork::EftposAu,
            "girocard" => PaymentIntentPaymentMethodOptionsCardNetwork::Girocard,
            "interac" => PaymentIntentPaymentMethodOptionsCardNetwork::Interac,
            "jcb" => PaymentIntentPaymentMethodOptionsCardNetwork::Jcb,
            "link" => PaymentIntentPaymentMethodOptionsCardNetwork::Link,
            "mastercard" => PaymentIntentPaymentMethodOptionsCardNetwork::Mastercard,
            "unionpay" => PaymentIntentPaymentMethodOptionsCardNetwork::Unionpay,
            "unknown" => PaymentIntentPaymentMethodOptionsCardNetwork::UnknownValue,
            "visa" => PaymentIntentPaymentMethodOptionsCardNetwork::Visa,
            other => PaymentIntentPaymentMethodOptionsCardNetwork::Unknown(other.to_string()),
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
pub enum PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
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
impl PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::Combined => "combined",
            PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::Interval => "interval",
            PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::Sporadic => "sporadic",
            PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "combined" => PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::Combined,
            "interval" => PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::Interval,
            "sporadic" => PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::Sporadic,
            other => {
                PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::Unknown(other.to_string())
            }
        })
    }
}
