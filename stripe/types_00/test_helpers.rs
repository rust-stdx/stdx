#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsLodging {
    #[doc = "The time of checking into the lodging."]
    pub check_in_at: Option<i64>,
    #[doc = "The number of nights stayed at the lodging."]
    pub nights: Option<i64>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsLodging {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("check_in_at", self.check_in_at.as_ref());
        form.field_opt("nights", self.nights.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm {}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        let _ = form;
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank {
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

#[cfg(feature = "test_helpers")]
impl CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::ArzteUndApothekerBank => {
                "arzte_und_apotheker_bank"
            }
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::AustrianAnadiBankAg => {
                "austrian_anadi_bank_ag"
            }
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::BankAustria => "bank_austria",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::BankhausCarlSpangler => {
                "bankhaus_carl_spangler"
            }
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::BankhausSchelhammerUndSchatteraAg => {
                "bankhaus_schelhammer_und_schattera_ag"
            }
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::BawagPskAg => "bawag_psk_ag",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::BksBankAg => "bks_bank_ag",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::BrullKallmusBankAg => {
                "brull_kallmus_bank_ag"
            }
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::BtvVierLanderBank => {
                "btv_vier_lander_bank"
            }
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::CapitalBankGraweGruppeAg => {
                "capital_bank_grawe_gruppe_ag"
            }
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::DeutscheBankAg => "deutsche_bank_ag",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::Dolomitenbank => "dolomitenbank",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::EasybankAg => "easybank_ag",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::ErsteBankUndSparkassen => {
                "erste_bank_und_sparkassen"
            }
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::HypoAlpeadriabankInternationalAg => {
                "hypo_alpeadriabank_international_ag"
            }
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::HypoBankBurgenlandAktiengesellschaft => {
                "hypo_bank_burgenland_aktiengesellschaft"
            }
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::HypoNoeLbFurNiederosterreichUWien => {
                "hypo_noe_lb_fur_niederosterreich_u_wien"
            }
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::HypoOberosterreichSalzburgSteiermark => {
                "hypo_oberosterreich_salzburg_steiermark"
            }
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::HypoTirolBankAg => "hypo_tirol_bank_ag",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::HypoVorarlbergBankAg => {
                "hypo_vorarlberg_bank_ag"
            }
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::MarchfelderBank => "marchfelder_bank",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::OberbankAg => "oberbank_ag",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::RaiffeisenBankengruppeOsterreich => {
                "raiffeisen_bankengruppe_osterreich"
            }
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::SchoellerbankAg => "schoellerbank_ag",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::SpardaBankWien => "sparda_bank_wien",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::VolksbankGruppe => "volksbank_gruppe",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::VolkskreditbankAg => "volkskreditbank_ag",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::VrBankBraunau => "vr_bank_braunau",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json> for CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "arzte_und_apotheker_bank" => {
                CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::ArzteUndApothekerBank
            }
            "austrian_anadi_bank_ag" => {
                CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::AustrianAnadiBankAg
            }
            "bank_austria" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::BankAustria,
            "bankhaus_carl_spangler" => {
                CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::BankhausCarlSpangler
            }
            "bankhaus_schelhammer_und_schattera_ag" => {
                CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::BankhausSchelhammerUndSchatteraAg
            }
            "bawag_psk_ag" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::BawagPskAg,
            "bks_bank_ag" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::BksBankAg,
            "brull_kallmus_bank_ag" => {
                CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::BrullKallmusBankAg
            }
            "btv_vier_lander_bank" => {
                CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::BtvVierLanderBank
            }
            "capital_bank_grawe_gruppe_ag" => {
                CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::CapitalBankGraweGruppeAg
            }
            "deutsche_bank_ag" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::DeutscheBankAg,
            "dolomitenbank" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::Dolomitenbank,
            "easybank_ag" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::EasybankAg,
            "erste_bank_und_sparkassen" => {
                CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::ErsteBankUndSparkassen
            }
            "hypo_alpeadriabank_international_ag" => {
                CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::HypoAlpeadriabankInternationalAg
            }
            "hypo_bank_burgenland_aktiengesellschaft" => {
                CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::HypoBankBurgenlandAktiengesellschaft
            }
            "hypo_noe_lb_fur_niederosterreich_u_wien" => {
                CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::HypoNoeLbFurNiederosterreichUWien
            }
            "hypo_oberosterreich_salzburg_steiermark" => {
                CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::HypoOberosterreichSalzburgSteiermark
            }
            "hypo_tirol_bank_ag" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::HypoTirolBankAg,
            "hypo_vorarlberg_bank_ag" => {
                CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::HypoVorarlbergBankAg
            }
            "marchfelder_bank" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::MarchfelderBank,
            "oberbank_ag" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::OberbankAg,
            "raiffeisen_bankengruppe_osterreich" => {
                CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::RaiffeisenBankengruppeOsterreich
            }
            "schoellerbank_ag" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::SchoellerbankAg,
            "sparda_bank_wien" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::SpardaBankWien,
            "volksbank_gruppe" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::VolksbankGruppe,
            "volkskreditbank_ag" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::VolkskreditbankAg,
            "vr_bank_braunau" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::VrBankBraunau,
            other => CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodDataEpsBank {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperConfirmationTokenParamsPaymentMethodDataPayto {
    #[doc = "The account number for the bank account."]
    pub account_number: Option<String>,
    #[doc = "Bank-State-Branch number of the bank account."]
    pub bsb_number: Option<String>,
    #[doc = "The PayID alias for the bank account."]
    pub pay_id: Option<String>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodDataPayto {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account_number", self.account_number.as_ref());
        form.field_opt("bsb_number", self.bsb_number.as_ref());
        form.field_opt("pay_id", self.pay_id.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTestHelperConfirmationTokenParamsPaymentMethodDataType {
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
    Link,
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

#[cfg(feature = "test_helpers")]
impl CreateTestHelperConfirmationTokenParamsPaymentMethodDataType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::AcssDebit => "acss_debit",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Affirm => "affirm",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::AfterpayClearpay => "afterpay_clearpay",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Alipay => "alipay",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Alma => "alma",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::AmazonPay => "amazon_pay",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::AuBecsDebit => "au_becs_debit",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::BacsDebit => "bacs_debit",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Bancontact => "bancontact",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Billie => "billie",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Bizum => "bizum",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Blik => "blik",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Boleto => "boleto",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Cashapp => "cashapp",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Crypto => "crypto",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::CustomerBalance => "customer_balance",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Eps => "eps",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Fpx => "fpx",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Giropay => "giropay",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Grabpay => "grabpay",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Ideal => "ideal",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::KakaoPay => "kakao_pay",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Klarna => "klarna",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Konbini => "konbini",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::KrCard => "kr_card",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Link => "link",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::MbWay => "mb_way",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Mobilepay => "mobilepay",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Multibanco => "multibanco",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::NaverPay => "naver_pay",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::NzBankAccount => "nz_bank_account",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Oxxo => "oxxo",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::P24 => "p24",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::PayByBank => "pay_by_bank",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Payco => "payco",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Paynow => "paynow",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Paypal => "paypal",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Paypay => "paypay",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Payto => "payto",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Pix => "pix",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Promptpay => "promptpay",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::RevolutPay => "revolut_pay",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::SamsungPay => "samsung_pay",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Satispay => "satispay",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Scalapay => "scalapay",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::SepaDebit => "sepa_debit",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Sequra => "sequra",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Sofort => "sofort",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Sunbit => "sunbit",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Swish => "swish",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Twint => "twint",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Upi => "upi",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::UsBankAccount => "us_bank_account",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::WechatPay => "wechat_pay",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Zip => "zip",
            CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json> for CreateTestHelperConfirmationTokenParamsPaymentMethodDataType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "acss_debit" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::AcssDebit,
            "affirm" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Affirm,
            "afterpay_clearpay" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::AfterpayClearpay,
            "alipay" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Alipay,
            "alma" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Alma,
            "amazon_pay" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::AmazonPay,
            "au_becs_debit" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::AuBecsDebit,
            "bacs_debit" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::BacsDebit,
            "bancontact" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Bancontact,
            "billie" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Billie,
            "bizum" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Bizum,
            "blik" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Blik,
            "boleto" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Boleto,
            "cashapp" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Cashapp,
            "crypto" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Crypto,
            "customer_balance" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::CustomerBalance,
            "eps" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Eps,
            "fpx" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Fpx,
            "giropay" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Giropay,
            "grabpay" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Grabpay,
            "ideal" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Ideal,
            "kakao_pay" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::KakaoPay,
            "klarna" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Klarna,
            "konbini" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Konbini,
            "kr_card" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::KrCard,
            "link" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Link,
            "mb_way" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::MbWay,
            "mobilepay" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Mobilepay,
            "multibanco" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Multibanco,
            "naver_pay" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::NaverPay,
            "nz_bank_account" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::NzBankAccount,
            "oxxo" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Oxxo,
            "p24" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::P24,
            "pay_by_bank" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::PayByBank,
            "payco" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Payco,
            "paynow" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Paynow,
            "paypal" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Paypal,
            "paypay" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Paypay,
            "payto" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Payto,
            "pix" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Pix,
            "promptpay" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Promptpay,
            "revolut_pay" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::RevolutPay,
            "samsung_pay" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::SamsungPay,
            "satispay" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Satispay,
            "scalapay" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Scalapay,
            "sepa_debit" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::SepaDebit,
            "sequra" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Sequra,
            "sofort" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Sofort,
            "sunbit" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Sunbit,
            "swish" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Swish,
            "twint" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Twint,
            "upi" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Upi,
            "us_bank_account" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::UsBankAccount,
            "wechat_pay" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::WechatPay,
            "zip" => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Zip,
            other => CreateTestHelperConfirmationTokenParamsPaymentMethodDataType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodDataType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTestHelperConfirmationTokenParamsPaymentMethodOptionsCardInstallmentsPlanType {
    Bonus,
    FixedCount,
    Revolving,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl CreateTestHelperConfirmationTokenParamsPaymentMethodOptionsCardInstallmentsPlanType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTestHelperConfirmationTokenParamsPaymentMethodOptionsCardInstallmentsPlanType::Bonus => "bonus",
            CreateTestHelperConfirmationTokenParamsPaymentMethodOptionsCardInstallmentsPlanType::FixedCount => {
                "fixed_count"
            }
            CreateTestHelperConfirmationTokenParamsPaymentMethodOptionsCardInstallmentsPlanType::Revolving => {
                "revolving"
            }
            CreateTestHelperConfirmationTokenParamsPaymentMethodOptionsCardInstallmentsPlanType::Unknown(value) => {
                value
            }
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json>
    for CreateTestHelperConfirmationTokenParamsPaymentMethodOptionsCardInstallmentsPlanType
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateTestHelperConfirmationTokenParamsPaymentMethodOptionsCardInstallmentsPlanType::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "bonus" => CreateTestHelperConfirmationTokenParamsPaymentMethodOptionsCardInstallmentsPlanType::Bonus,
            "fixed_count" => {
                CreateTestHelperConfirmationTokenParamsPaymentMethodOptionsCardInstallmentsPlanType::FixedCount
            }
            "revolving" => {
                CreateTestHelperConfirmationTokenParamsPaymentMethodOptionsCardInstallmentsPlanType::Revolving
            }
            other => CreateTestHelperConfirmationTokenParamsPaymentMethodOptionsCardInstallmentsPlanType::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodOptionsCardInstallmentsPlanType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperIssuingAuthorizationParamsFleetReportedBreakdownTax {
    #[doc = "Amount of state or provincial Sales Tax included in the transaction amount."]
    pub local_amount_decimal: Option<String>,
    #[doc = "Amount of national Sales Tax or VAT included in the transaction amount."]
    pub national_amount_decimal: Option<String>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperIssuingAuthorizationParamsFleetReportedBreakdownTax {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("local_amount_decimal", self.local_amount_decimal.as_ref());
        form.field_opt("national_amount_decimal", self.national_amount_decimal.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTestHelperIssuingAuthorizationParamsFleetServiceType {
    FullService,
    NonFuelTransaction,
    SelfService,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl CreateTestHelperIssuingAuthorizationParamsFleetServiceType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTestHelperIssuingAuthorizationParamsFleetServiceType::FullService => "full_service",
            CreateTestHelperIssuingAuthorizationParamsFleetServiceType::NonFuelTransaction => "non_fuel_transaction",
            CreateTestHelperIssuingAuthorizationParamsFleetServiceType::SelfService => "self_service",
            CreateTestHelperIssuingAuthorizationParamsFleetServiceType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json> for CreateTestHelperIssuingAuthorizationParamsFleetServiceType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTestHelperIssuingAuthorizationParamsFleetServiceType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "full_service" => CreateTestHelperIssuingAuthorizationParamsFleetServiceType::FullService,
            "non_fuel_transaction" => CreateTestHelperIssuingAuthorizationParamsFleetServiceType::NonFuelTransaction,
            "self_service" => CreateTestHelperIssuingAuthorizationParamsFleetServiceType::SelfService,
            other => CreateTestHelperIssuingAuthorizationParamsFleetServiceType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperIssuingAuthorizationParamsFleetServiceType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTestHelperIssuingAuthorizationParamsFraudDisputabilityLikelihood {
    Neutral,
    UnknownValue,
    VeryLikely,
    VeryUnlikely,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl CreateTestHelperIssuingAuthorizationParamsFraudDisputabilityLikelihood {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTestHelperIssuingAuthorizationParamsFraudDisputabilityLikelihood::Neutral => "neutral",
            CreateTestHelperIssuingAuthorizationParamsFraudDisputabilityLikelihood::UnknownValue => "unknown",
            CreateTestHelperIssuingAuthorizationParamsFraudDisputabilityLikelihood::VeryLikely => "very_likely",
            CreateTestHelperIssuingAuthorizationParamsFraudDisputabilityLikelihood::VeryUnlikely => "very_unlikely",
            CreateTestHelperIssuingAuthorizationParamsFraudDisputabilityLikelihood::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json> for CreateTestHelperIssuingAuthorizationParamsFraudDisputabilityLikelihood {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTestHelperIssuingAuthorizationParamsFraudDisputabilityLikelihood::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "neutral" => CreateTestHelperIssuingAuthorizationParamsFraudDisputabilityLikelihood::Neutral,
            "unknown" => CreateTestHelperIssuingAuthorizationParamsFraudDisputabilityLikelihood::UnknownValue,
            "very_likely" => CreateTestHelperIssuingAuthorizationParamsFraudDisputabilityLikelihood::VeryLikely,
            "very_unlikely" => CreateTestHelperIssuingAuthorizationParamsFraudDisputabilityLikelihood::VeryUnlikely,
            other => CreateTestHelperIssuingAuthorizationParamsFraudDisputabilityLikelihood::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperIssuingAuthorizationParamsFraudDisputabilityLikelihood {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTestHelperTreasuryReceivedCreditParamsInitiatingPaymentMethodDetailsType {
    UsBankAccount,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl CreateTestHelperTreasuryReceivedCreditParamsInitiatingPaymentMethodDetailsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTestHelperTreasuryReceivedCreditParamsInitiatingPaymentMethodDetailsType::UsBankAccount => {
                "us_bank_account"
            }
            CreateTestHelperTreasuryReceivedCreditParamsInitiatingPaymentMethodDetailsType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json> for CreateTestHelperTreasuryReceivedCreditParamsInitiatingPaymentMethodDetailsType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateTestHelperTreasuryReceivedCreditParamsInitiatingPaymentMethodDetailsType::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "us_bank_account" => {
                CreateTestHelperTreasuryReceivedCreditParamsInitiatingPaymentMethodDetailsType::UsBankAccount
            }
            other => CreateTestHelperTreasuryReceivedCreditParamsInitiatingPaymentMethodDetailsType::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperTreasuryReceivedCreditParamsInitiatingPaymentMethodDetailsType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateTestHelperTreasuryReceivedCreditParamsNetwork {
    Ach,
    Rtp,
    UsDomesticWire,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl CreateTestHelperTreasuryReceivedCreditParamsNetwork {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateTestHelperTreasuryReceivedCreditParamsNetwork::Ach => "ach",
            CreateTestHelperTreasuryReceivedCreditParamsNetwork::Rtp => "rtp",
            CreateTestHelperTreasuryReceivedCreditParamsNetwork::UsDomesticWire => "us_domestic_wire",
            CreateTestHelperTreasuryReceivedCreditParamsNetwork::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json> for CreateTestHelperTreasuryReceivedCreditParamsNetwork {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateTestHelperTreasuryReceivedCreditParamsNetwork::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ach" => CreateTestHelperTreasuryReceivedCreditParamsNetwork::Ach,
            "rtp" => CreateTestHelperTreasuryReceivedCreditParamsNetwork::Rtp,
            "us_domestic_wire" => CreateTestHelperTreasuryReceivedCreditParamsNetwork::UsDomesticWire,
            other => CreateTestHelperTreasuryReceivedCreditParamsNetwork::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperTreasuryReceivedCreditParamsNetwork {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct FundCashBalanceParams {
    #[doc = "Amount to be used for this test cash balance transaction."]
    pub amount: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "A description of the test funding."]
    pub reference: Option<String>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for FundCashBalanceParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        form.field("currency", &self.currency);
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("reference", self.reference.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct PresentPaymentMethodParamsCard {
    #[doc = "Card security code."]
    pub cvc: Option<String>,
    #[doc = "Two-digit number representing the card's expiration month."]
    pub exp_month: i64,
    #[doc = "Two- or four-digit number representing the card's expiration year."]
    pub exp_year: i64,
    #[doc = "The card number, as a string without any separators."]
    pub number: String,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for PresentPaymentMethodParamsCard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("cvc", self.cvc.as_ref());
        form.field("exp_month", &self.exp_month);
        form.field("exp_year", &self.exp_year);
        form.field("number", &self.number);
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct ReverseTestHelperIssuingAuthorizationParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The amount to reverse from the authorization."]
    pub reverse_amount: Option<i64>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for ReverseTestHelperIssuingAuthorizationParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("reverse_amount", self.reverse_amount.as_ref());
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
pub enum TestHelpersTestClockStatus {
    Advancing,
    InternalFailure,
    Ready,
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
impl TestHelpersTestClockStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            TestHelpersTestClockStatus::Advancing => "advancing",
            TestHelpersTestClockStatus::InternalFailure => "internal_failure",
            TestHelpersTestClockStatus::Ready => "ready",
            TestHelpersTestClockStatus::Unknown(value) => value,
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
impl Default for TestHelpersTestClockStatus {
    fn default() -> Self {
        TestHelpersTestClockStatus::Unknown(String::new())
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
impl<'json> json::Decode<'json> for TestHelpersTestClockStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(TestHelpersTestClockStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "advancing" => TestHelpersTestClockStatus::Advancing,
            "internal_failure" => TestHelpersTestClockStatus::InternalFailure,
            "ready" => TestHelpersTestClockStatus::Ready,
            other => TestHelpersTestClockStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct UpdateTestHelperTreasuryOutboundPaymentParamsTrackingDetailsAch {
    #[doc = "ACH trace ID for funds sent over the `ach` network."]
    pub trace_id: String,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for UpdateTestHelperTreasuryOutboundPaymentParamsTrackingDetailsAch {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("trace_id", &self.trace_id);
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, PartialEq)]
pub enum UpdateTestHelperTreasuryOutboundPaymentParamsTrackingDetailsType {
    Ach,
    UsDomesticWire,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "test_helpers")]
impl UpdateTestHelperTreasuryOutboundPaymentParamsTrackingDetailsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            UpdateTestHelperTreasuryOutboundPaymentParamsTrackingDetailsType::Ach => "ach",
            UpdateTestHelperTreasuryOutboundPaymentParamsTrackingDetailsType::UsDomesticWire => "us_domestic_wire",
            UpdateTestHelperTreasuryOutboundPaymentParamsTrackingDetailsType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "test_helpers")]
impl<'json> json::Decode<'json> for UpdateTestHelperTreasuryOutboundPaymentParamsTrackingDetailsType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(UpdateTestHelperTreasuryOutboundPaymentParamsTrackingDetailsType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ach" => UpdateTestHelperTreasuryOutboundPaymentParamsTrackingDetailsType::Ach,
            "us_domestic_wire" => UpdateTestHelperTreasuryOutboundPaymentParamsTrackingDetailsType::UsDomesticWire,
            other => UpdateTestHelperTreasuryOutboundPaymentParamsTrackingDetailsType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for UpdateTestHelperTreasuryOutboundPaymentParamsTrackingDetailsType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
