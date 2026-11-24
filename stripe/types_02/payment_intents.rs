#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CancelPaymentIntentParamsCancellationReason {
    Abandoned,
    Duplicate,
    Fraudulent,
    RequestedByCustomer,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CancelPaymentIntentParamsCancellationReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CancelPaymentIntentParamsCancellationReason::Abandoned => "abandoned",
            CancelPaymentIntentParamsCancellationReason::Duplicate => "duplicate",
            CancelPaymentIntentParamsCancellationReason::Fraudulent => "fraudulent",
            CancelPaymentIntentParamsCancellationReason::RequestedByCustomer => "requested_by_customer",
            CancelPaymentIntentParamsCancellationReason::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CancelPaymentIntentParamsCancellationReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CancelPaymentIntentParamsCancellationReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "abandoned" => CancelPaymentIntentParamsCancellationReason::Abandoned,
            "duplicate" => CancelPaymentIntentParamsCancellationReason::Duplicate,
            "fraudulent" => CancelPaymentIntentParamsCancellationReason::Fraudulent,
            "requested_by_customer" => CancelPaymentIntentParamsCancellationReason::RequestedByCustomer,
            other => CancelPaymentIntentParamsCancellationReason::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CancelPaymentIntentParamsCancellationReason {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsAmountDetailsShipping {
    #[doc = "If a physical good is being shipped, the cost of shipping represented in the smallest currency unit..."]
    pub amount: Option<json::OwnedValue>,
    #[doc = "If a physical good is being shipped, the postal code of where it is being shipped from."]
    pub from_postal_code: Option<json::OwnedValue>,
    #[doc = "If a physical good is being shipped, the postal code of where it is being shipped to."]
    pub to_postal_code: Option<json::OwnedValue>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsAmountDetailsShipping {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        let _ = form;
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsConfirmationMethod {
    Automatic,
    Manual,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsConfirmationMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsConfirmationMethod::Automatic => "automatic",
            CreatePaymentIntentParamsConfirmationMethod::Manual => "manual",
            CreatePaymentIntentParamsConfirmationMethod::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsConfirmationMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsConfirmationMethod::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "automatic" => CreatePaymentIntentParamsConfirmationMethod::Automatic,
            "manual" => CreatePaymentIntentParamsConfirmationMethod::Manual,
            other => CreatePaymentIntentParamsConfirmationMethod::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsConfirmationMethod {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsMandateDataCustomerAcceptanceOnline {
    #[doc = "The IP address from which the Mandate was accepted by the customer."]
    pub ip_address: String,
    #[doc = "The user agent of the browser from which the Mandate was accepted by the customer."]
    pub user_agent: String,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsMandateDataCustomerAcceptanceOnline {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("ip_address", &self.ip_address);
        form.field("user_agent", &self.user_agent);
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentDetails {
    #[doc = "A unique value to identify the customer."]
    pub customer_reference: Option<json::OwnedValue>,
    #[doc = "A unique value assigned by the business to identify the transaction."]
    pub order_reference: Option<json::OwnedValue>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        let _ = form;
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodDataBacsDebit {
    #[doc = "Account number of the bank account that the funds will be debited from."]
    pub account_number: Option<String>,
    #[doc = "Sort code of the bank account."]
    pub sort_code: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodDataBacsDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account_number", self.account_number.as_ref());
        form.field_opt("sort_code", self.sort_code.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodDataFpxAccountHolderType {
    Company,
    Individual,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodDataFpxAccountHolderType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodDataFpxAccountHolderType::Company => "company",
            CreatePaymentIntentParamsPaymentMethodDataFpxAccountHolderType::Individual => "individual",
            CreatePaymentIntentParamsPaymentMethodDataFpxAccountHolderType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodDataFpxAccountHolderType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsPaymentMethodDataFpxAccountHolderType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "company" => CreatePaymentIntentParamsPaymentMethodDataFpxAccountHolderType::Company,
            "individual" => CreatePaymentIntentParamsPaymentMethodDataFpxAccountHolderType::Individual,
            other => CreatePaymentIntentParamsPaymentMethodDataFpxAccountHolderType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodDataFpxAccountHolderType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodDataFpxBank {
    AffinBank,
    Agrobank,
    AllianceBank,
    Ambank,
    BankIslam,
    BankMuamalat,
    BankOfChina,
    BankRakyat,
    BnpParibas,
    Bsn,
    Cimb,
    Citibank,
    DeutscheBank,
    HongLeongBank,
    Hsbc,
    Kfh,
    Maybank2e,
    Maybank2u,
    MbsbBank,
    Ocbc,
    PbEnterprise,
    PublicBank,
    Rhb,
    StandardChartered,
    Uob,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodDataFpxBank {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodDataFpxBank::AffinBank => "affin_bank",
            CreatePaymentIntentParamsPaymentMethodDataFpxBank::Agrobank => "agrobank",
            CreatePaymentIntentParamsPaymentMethodDataFpxBank::AllianceBank => "alliance_bank",
            CreatePaymentIntentParamsPaymentMethodDataFpxBank::Ambank => "ambank",
            CreatePaymentIntentParamsPaymentMethodDataFpxBank::BankIslam => "bank_islam",
            CreatePaymentIntentParamsPaymentMethodDataFpxBank::BankMuamalat => "bank_muamalat",
            CreatePaymentIntentParamsPaymentMethodDataFpxBank::BankOfChina => "bank_of_china",
            CreatePaymentIntentParamsPaymentMethodDataFpxBank::BankRakyat => "bank_rakyat",
            CreatePaymentIntentParamsPaymentMethodDataFpxBank::BnpParibas => "bnp_paribas",
            CreatePaymentIntentParamsPaymentMethodDataFpxBank::Bsn => "bsn",
            CreatePaymentIntentParamsPaymentMethodDataFpxBank::Cimb => "cimb",
            CreatePaymentIntentParamsPaymentMethodDataFpxBank::Citibank => "citibank",
            CreatePaymentIntentParamsPaymentMethodDataFpxBank::DeutscheBank => "deutsche_bank",
            CreatePaymentIntentParamsPaymentMethodDataFpxBank::HongLeongBank => "hong_leong_bank",
            CreatePaymentIntentParamsPaymentMethodDataFpxBank::Hsbc => "hsbc",
            CreatePaymentIntentParamsPaymentMethodDataFpxBank::Kfh => "kfh",
            CreatePaymentIntentParamsPaymentMethodDataFpxBank::Maybank2e => "maybank2e",
            CreatePaymentIntentParamsPaymentMethodDataFpxBank::Maybank2u => "maybank2u",
            CreatePaymentIntentParamsPaymentMethodDataFpxBank::MbsbBank => "mbsb_bank",
            CreatePaymentIntentParamsPaymentMethodDataFpxBank::Ocbc => "ocbc",
            CreatePaymentIntentParamsPaymentMethodDataFpxBank::PbEnterprise => "pb_enterprise",
            CreatePaymentIntentParamsPaymentMethodDataFpxBank::PublicBank => "public_bank",
            CreatePaymentIntentParamsPaymentMethodDataFpxBank::Rhb => "rhb",
            CreatePaymentIntentParamsPaymentMethodDataFpxBank::StandardChartered => "standard_chartered",
            CreatePaymentIntentParamsPaymentMethodDataFpxBank::Uob => "uob",
            CreatePaymentIntentParamsPaymentMethodDataFpxBank::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodDataFpxBank {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsPaymentMethodDataFpxBank::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "affin_bank" => CreatePaymentIntentParamsPaymentMethodDataFpxBank::AffinBank,
            "agrobank" => CreatePaymentIntentParamsPaymentMethodDataFpxBank::Agrobank,
            "alliance_bank" => CreatePaymentIntentParamsPaymentMethodDataFpxBank::AllianceBank,
            "ambank" => CreatePaymentIntentParamsPaymentMethodDataFpxBank::Ambank,
            "bank_islam" => CreatePaymentIntentParamsPaymentMethodDataFpxBank::BankIslam,
            "bank_muamalat" => CreatePaymentIntentParamsPaymentMethodDataFpxBank::BankMuamalat,
            "bank_of_china" => CreatePaymentIntentParamsPaymentMethodDataFpxBank::BankOfChina,
            "bank_rakyat" => CreatePaymentIntentParamsPaymentMethodDataFpxBank::BankRakyat,
            "bnp_paribas" => CreatePaymentIntentParamsPaymentMethodDataFpxBank::BnpParibas,
            "bsn" => CreatePaymentIntentParamsPaymentMethodDataFpxBank::Bsn,
            "cimb" => CreatePaymentIntentParamsPaymentMethodDataFpxBank::Cimb,
            "citibank" => CreatePaymentIntentParamsPaymentMethodDataFpxBank::Citibank,
            "deutsche_bank" => CreatePaymentIntentParamsPaymentMethodDataFpxBank::DeutscheBank,
            "hong_leong_bank" => CreatePaymentIntentParamsPaymentMethodDataFpxBank::HongLeongBank,
            "hsbc" => CreatePaymentIntentParamsPaymentMethodDataFpxBank::Hsbc,
            "kfh" => CreatePaymentIntentParamsPaymentMethodDataFpxBank::Kfh,
            "maybank2e" => CreatePaymentIntentParamsPaymentMethodDataFpxBank::Maybank2e,
            "maybank2u" => CreatePaymentIntentParamsPaymentMethodDataFpxBank::Maybank2u,
            "mbsb_bank" => CreatePaymentIntentParamsPaymentMethodDataFpxBank::MbsbBank,
            "ocbc" => CreatePaymentIntentParamsPaymentMethodDataFpxBank::Ocbc,
            "pb_enterprise" => CreatePaymentIntentParamsPaymentMethodDataFpxBank::PbEnterprise,
            "public_bank" => CreatePaymentIntentParamsPaymentMethodDataFpxBank::PublicBank,
            "rhb" => CreatePaymentIntentParamsPaymentMethodDataFpxBank::Rhb,
            "standard_chartered" => CreatePaymentIntentParamsPaymentMethodDataFpxBank::StandardChartered,
            "uob" => CreatePaymentIntentParamsPaymentMethodDataFpxBank::Uob,
            other => CreatePaymentIntentParamsPaymentMethodDataFpxBank::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodDataFpxBank {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodDataNaverPayFunding {
    Card,
    Points,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodDataNaverPayFunding {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodDataNaverPayFunding::Card => "card",
            CreatePaymentIntentParamsPaymentMethodDataNaverPayFunding::Points => "points",
            CreatePaymentIntentParamsPaymentMethodDataNaverPayFunding::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodDataNaverPayFunding {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsPaymentMethodDataNaverPayFunding::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "card" => CreatePaymentIntentParamsPaymentMethodDataNaverPayFunding::Card,
            "points" => CreatePaymentIntentParamsPaymentMethodDataNaverPayFunding::Points,
            other => CreatePaymentIntentParamsPaymentMethodDataNaverPayFunding::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodDataNaverPayFunding {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodDataType {
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

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodDataType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodDataType::AcssDebit => "acss_debit",
            CreatePaymentIntentParamsPaymentMethodDataType::Affirm => "affirm",
            CreatePaymentIntentParamsPaymentMethodDataType::AfterpayClearpay => "afterpay_clearpay",
            CreatePaymentIntentParamsPaymentMethodDataType::Alipay => "alipay",
            CreatePaymentIntentParamsPaymentMethodDataType::Alma => "alma",
            CreatePaymentIntentParamsPaymentMethodDataType::AmazonPay => "amazon_pay",
            CreatePaymentIntentParamsPaymentMethodDataType::AuBecsDebit => "au_becs_debit",
            CreatePaymentIntentParamsPaymentMethodDataType::BacsDebit => "bacs_debit",
            CreatePaymentIntentParamsPaymentMethodDataType::Bancontact => "bancontact",
            CreatePaymentIntentParamsPaymentMethodDataType::Billie => "billie",
            CreatePaymentIntentParamsPaymentMethodDataType::Bizum => "bizum",
            CreatePaymentIntentParamsPaymentMethodDataType::Blik => "blik",
            CreatePaymentIntentParamsPaymentMethodDataType::Boleto => "boleto",
            CreatePaymentIntentParamsPaymentMethodDataType::Cashapp => "cashapp",
            CreatePaymentIntentParamsPaymentMethodDataType::Crypto => "crypto",
            CreatePaymentIntentParamsPaymentMethodDataType::CustomerBalance => "customer_balance",
            CreatePaymentIntentParamsPaymentMethodDataType::Eps => "eps",
            CreatePaymentIntentParamsPaymentMethodDataType::Fpx => "fpx",
            CreatePaymentIntentParamsPaymentMethodDataType::Giropay => "giropay",
            CreatePaymentIntentParamsPaymentMethodDataType::Grabpay => "grabpay",
            CreatePaymentIntentParamsPaymentMethodDataType::Ideal => "ideal",
            CreatePaymentIntentParamsPaymentMethodDataType::KakaoPay => "kakao_pay",
            CreatePaymentIntentParamsPaymentMethodDataType::Klarna => "klarna",
            CreatePaymentIntentParamsPaymentMethodDataType::Konbini => "konbini",
            CreatePaymentIntentParamsPaymentMethodDataType::KrCard => "kr_card",
            CreatePaymentIntentParamsPaymentMethodDataType::Link => "link",
            CreatePaymentIntentParamsPaymentMethodDataType::MbWay => "mb_way",
            CreatePaymentIntentParamsPaymentMethodDataType::Mobilepay => "mobilepay",
            CreatePaymentIntentParamsPaymentMethodDataType::Multibanco => "multibanco",
            CreatePaymentIntentParamsPaymentMethodDataType::NaverPay => "naver_pay",
            CreatePaymentIntentParamsPaymentMethodDataType::NzBankAccount => "nz_bank_account",
            CreatePaymentIntentParamsPaymentMethodDataType::Oxxo => "oxxo",
            CreatePaymentIntentParamsPaymentMethodDataType::P24 => "p24",
            CreatePaymentIntentParamsPaymentMethodDataType::PayByBank => "pay_by_bank",
            CreatePaymentIntentParamsPaymentMethodDataType::Payco => "payco",
            CreatePaymentIntentParamsPaymentMethodDataType::Paynow => "paynow",
            CreatePaymentIntentParamsPaymentMethodDataType::Paypal => "paypal",
            CreatePaymentIntentParamsPaymentMethodDataType::Paypay => "paypay",
            CreatePaymentIntentParamsPaymentMethodDataType::Payto => "payto",
            CreatePaymentIntentParamsPaymentMethodDataType::Pix => "pix",
            CreatePaymentIntentParamsPaymentMethodDataType::Promptpay => "promptpay",
            CreatePaymentIntentParamsPaymentMethodDataType::RevolutPay => "revolut_pay",
            CreatePaymentIntentParamsPaymentMethodDataType::SamsungPay => "samsung_pay",
            CreatePaymentIntentParamsPaymentMethodDataType::Satispay => "satispay",
            CreatePaymentIntentParamsPaymentMethodDataType::Scalapay => "scalapay",
            CreatePaymentIntentParamsPaymentMethodDataType::SepaDebit => "sepa_debit",
            CreatePaymentIntentParamsPaymentMethodDataType::Sequra => "sequra",
            CreatePaymentIntentParamsPaymentMethodDataType::Sofort => "sofort",
            CreatePaymentIntentParamsPaymentMethodDataType::Sunbit => "sunbit",
            CreatePaymentIntentParamsPaymentMethodDataType::Swish => "swish",
            CreatePaymentIntentParamsPaymentMethodDataType::Twint => "twint",
            CreatePaymentIntentParamsPaymentMethodDataType::Upi => "upi",
            CreatePaymentIntentParamsPaymentMethodDataType::UsBankAccount => "us_bank_account",
            CreatePaymentIntentParamsPaymentMethodDataType::WechatPay => "wechat_pay",
            CreatePaymentIntentParamsPaymentMethodDataType::Zip => "zip",
            CreatePaymentIntentParamsPaymentMethodDataType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodDataType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsPaymentMethodDataType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "acss_debit" => CreatePaymentIntentParamsPaymentMethodDataType::AcssDebit,
            "affirm" => CreatePaymentIntentParamsPaymentMethodDataType::Affirm,
            "afterpay_clearpay" => CreatePaymentIntentParamsPaymentMethodDataType::AfterpayClearpay,
            "alipay" => CreatePaymentIntentParamsPaymentMethodDataType::Alipay,
            "alma" => CreatePaymentIntentParamsPaymentMethodDataType::Alma,
            "amazon_pay" => CreatePaymentIntentParamsPaymentMethodDataType::AmazonPay,
            "au_becs_debit" => CreatePaymentIntentParamsPaymentMethodDataType::AuBecsDebit,
            "bacs_debit" => CreatePaymentIntentParamsPaymentMethodDataType::BacsDebit,
            "bancontact" => CreatePaymentIntentParamsPaymentMethodDataType::Bancontact,
            "billie" => CreatePaymentIntentParamsPaymentMethodDataType::Billie,
            "bizum" => CreatePaymentIntentParamsPaymentMethodDataType::Bizum,
            "blik" => CreatePaymentIntentParamsPaymentMethodDataType::Blik,
            "boleto" => CreatePaymentIntentParamsPaymentMethodDataType::Boleto,
            "cashapp" => CreatePaymentIntentParamsPaymentMethodDataType::Cashapp,
            "crypto" => CreatePaymentIntentParamsPaymentMethodDataType::Crypto,
            "customer_balance" => CreatePaymentIntentParamsPaymentMethodDataType::CustomerBalance,
            "eps" => CreatePaymentIntentParamsPaymentMethodDataType::Eps,
            "fpx" => CreatePaymentIntentParamsPaymentMethodDataType::Fpx,
            "giropay" => CreatePaymentIntentParamsPaymentMethodDataType::Giropay,
            "grabpay" => CreatePaymentIntentParamsPaymentMethodDataType::Grabpay,
            "ideal" => CreatePaymentIntentParamsPaymentMethodDataType::Ideal,
            "kakao_pay" => CreatePaymentIntentParamsPaymentMethodDataType::KakaoPay,
            "klarna" => CreatePaymentIntentParamsPaymentMethodDataType::Klarna,
            "konbini" => CreatePaymentIntentParamsPaymentMethodDataType::Konbini,
            "kr_card" => CreatePaymentIntentParamsPaymentMethodDataType::KrCard,
            "link" => CreatePaymentIntentParamsPaymentMethodDataType::Link,
            "mb_way" => CreatePaymentIntentParamsPaymentMethodDataType::MbWay,
            "mobilepay" => CreatePaymentIntentParamsPaymentMethodDataType::Mobilepay,
            "multibanco" => CreatePaymentIntentParamsPaymentMethodDataType::Multibanco,
            "naver_pay" => CreatePaymentIntentParamsPaymentMethodDataType::NaverPay,
            "nz_bank_account" => CreatePaymentIntentParamsPaymentMethodDataType::NzBankAccount,
            "oxxo" => CreatePaymentIntentParamsPaymentMethodDataType::Oxxo,
            "p24" => CreatePaymentIntentParamsPaymentMethodDataType::P24,
            "pay_by_bank" => CreatePaymentIntentParamsPaymentMethodDataType::PayByBank,
            "payco" => CreatePaymentIntentParamsPaymentMethodDataType::Payco,
            "paynow" => CreatePaymentIntentParamsPaymentMethodDataType::Paynow,
            "paypal" => CreatePaymentIntentParamsPaymentMethodDataType::Paypal,
            "paypay" => CreatePaymentIntentParamsPaymentMethodDataType::Paypay,
            "payto" => CreatePaymentIntentParamsPaymentMethodDataType::Payto,
            "pix" => CreatePaymentIntentParamsPaymentMethodDataType::Pix,
            "promptpay" => CreatePaymentIntentParamsPaymentMethodDataType::Promptpay,
            "revolut_pay" => CreatePaymentIntentParamsPaymentMethodDataType::RevolutPay,
            "samsung_pay" => CreatePaymentIntentParamsPaymentMethodDataType::SamsungPay,
            "satispay" => CreatePaymentIntentParamsPaymentMethodDataType::Satispay,
            "scalapay" => CreatePaymentIntentParamsPaymentMethodDataType::Scalapay,
            "sepa_debit" => CreatePaymentIntentParamsPaymentMethodDataType::SepaDebit,
            "sequra" => CreatePaymentIntentParamsPaymentMethodDataType::Sequra,
            "sofort" => CreatePaymentIntentParamsPaymentMethodDataType::Sofort,
            "sunbit" => CreatePaymentIntentParamsPaymentMethodDataType::Sunbit,
            "swish" => CreatePaymentIntentParamsPaymentMethodDataType::Swish,
            "twint" => CreatePaymentIntentParamsPaymentMethodDataType::Twint,
            "upi" => CreatePaymentIntentParamsPaymentMethodDataType::Upi,
            "us_bank_account" => CreatePaymentIntentParamsPaymentMethodDataType::UsBankAccount,
            "wechat_pay" => CreatePaymentIntentParamsPaymentMethodDataType::WechatPay,
            "zip" => CreatePaymentIntentParamsPaymentMethodDataType::Zip,
            other => CreatePaymentIntentParamsPaymentMethodDataType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodDataType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodDataUsBankAccountAccountType {
    Checking,
    Savings,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodDataUsBankAccountAccountType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodDataUsBankAccountAccountType::Checking => "checking",
            CreatePaymentIntentParamsPaymentMethodDataUsBankAccountAccountType::Savings => "savings",
            CreatePaymentIntentParamsPaymentMethodDataUsBankAccountAccountType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodDataUsBankAccountAccountType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsPaymentMethodDataUsBankAccountAccountType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "checking" => CreatePaymentIntentParamsPaymentMethodDataUsBankAccountAccountType::Checking,
            "savings" => CreatePaymentIntentParamsPaymentMethodDataUsBankAccountAccountType::Savings,
            other => CreatePaymentIntentParamsPaymentMethodDataUsBankAccountAccountType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodDataUsBankAccountAccountType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Business => "business",
            CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Personal => "personal",
            CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Unknown(value) => {
                value
            }
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json>
    for CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "business" => CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Business,
            "personal" => CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Personal,
            other => CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage {
    None,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage::None => "none",
            CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "none" => CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage::None,
            other => CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceFundingType {
    BankTransfer,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceFundingType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceFundingType::BankTransfer => "bank_transfer",
            CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceFundingType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceFundingType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceFundingType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "bank_transfer" => CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceFundingType::BankTransfer,
            other => {
                CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceFundingType::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceFundingType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose {
    Unnamed,
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

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Unnamed => "",
            CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::DependantSupport => {
                "dependant_support"
            }
            CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Government => "government",
            CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Loan => "loan",
            CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Mortgage => "mortgage",
            CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Other => "other",
            CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Pension => "pension",
            CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Personal => "personal",
            CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Retail => "retail",
            CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Salary => "salary",
            CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Tax => "tax",
            CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Utility => "utility",
            CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Unnamed,
            "dependant_support" => {
                CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::DependantSupport
            }
            "government" => CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Government,
            "loan" => CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Loan,
            "mortgage" => CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Mortgage,
            "other" => CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Other,
            "pension" => CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Pension,
            "personal" => CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Personal,
            "retail" => CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Retail,
            "salary" => CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Salary,
            "tax" => CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Tax,
            "utility" => CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Utility,
            other => {
                CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsSofortPreferredLanguage {
    Unnamed,
    De,
    En,
    Es,
    Fr,
    It,
    Nl,
    Pl,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsSofortPreferredLanguage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsSofortPreferredLanguage::Unnamed => "",
            CreatePaymentIntentParamsPaymentMethodOptionsSofortPreferredLanguage::De => "de",
            CreatePaymentIntentParamsPaymentMethodOptionsSofortPreferredLanguage::En => "en",
            CreatePaymentIntentParamsPaymentMethodOptionsSofortPreferredLanguage::Es => "es",
            CreatePaymentIntentParamsPaymentMethodOptionsSofortPreferredLanguage::Fr => "fr",
            CreatePaymentIntentParamsPaymentMethodOptionsSofortPreferredLanguage::It => "it",
            CreatePaymentIntentParamsPaymentMethodOptionsSofortPreferredLanguage::Nl => "nl",
            CreatePaymentIntentParamsPaymentMethodOptionsSofortPreferredLanguage::Pl => "pl",
            CreatePaymentIntentParamsPaymentMethodOptionsSofortPreferredLanguage::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsSofortPreferredLanguage {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsPaymentMethodOptionsSofortPreferredLanguage::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreatePaymentIntentParamsPaymentMethodOptionsSofortPreferredLanguage::Unnamed,
            "de" => CreatePaymentIntentParamsPaymentMethodOptionsSofortPreferredLanguage::De,
            "en" => CreatePaymentIntentParamsPaymentMethodOptionsSofortPreferredLanguage::En,
            "es" => CreatePaymentIntentParamsPaymentMethodOptionsSofortPreferredLanguage::Es,
            "fr" => CreatePaymentIntentParamsPaymentMethodOptionsSofortPreferredLanguage::Fr,
            "it" => CreatePaymentIntentParamsPaymentMethodOptionsSofortPreferredLanguage::It,
            "nl" => CreatePaymentIntentParamsPaymentMethodOptionsSofortPreferredLanguage::Nl,
            "pl" => CreatePaymentIntentParamsPaymentMethodOptionsSofortPreferredLanguage::Pl,
            other => CreatePaymentIntentParamsPaymentMethodOptionsSofortPreferredLanguage::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsSofortPreferredLanguage {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsWechatPayClient {
    Android,
    Ios,
    Web,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsWechatPayClient {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsWechatPayClient::Android => "android",
            CreatePaymentIntentParamsPaymentMethodOptionsWechatPayClient::Ios => "ios",
            CreatePaymentIntentParamsPaymentMethodOptionsWechatPayClient::Web => "web",
            CreatePaymentIntentParamsPaymentMethodOptionsWechatPayClient::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsWechatPayClient {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsPaymentMethodOptionsWechatPayClient::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "android" => CreatePaymentIntentParamsPaymentMethodOptionsWechatPayClient::Android,
            "ios" => CreatePaymentIntentParamsPaymentMethodOptionsWechatPayClient::Ios,
            "web" => CreatePaymentIntentParamsPaymentMethodOptionsWechatPayClient::Web,
            other => CreatePaymentIntentParamsPaymentMethodOptionsWechatPayClient::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsWechatPayClient {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "customers")]
#[derive(Debug, Clone, PartialEq)]
pub enum FundingInstructionsFundingType {
    BankTransfer,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "customers")]
impl FundingInstructionsFundingType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            FundingInstructionsFundingType::BankTransfer => "bank_transfer",
            FundingInstructionsFundingType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "customers")]
impl Default for FundingInstructionsFundingType {
    fn default() -> Self {
        FundingInstructionsFundingType::Unknown(String::new())
    }
}

#[cfg(feature = "customers")]
impl<'json> json::Decode<'json> for FundingInstructionsFundingType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(FundingInstructionsFundingType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "bank_transfer" => FundingInstructionsFundingType::BankTransfer,
            other => FundingInstructionsFundingType::Unknown(other.to_string()),
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
pub struct PaymentFlowsAmountDetailsClientResourceTip {
    #[doc = "Portion of the amount that corresponds to a tip."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
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
pub struct PaymentFlowsAmountDetailsResourceShipping {
    #[doc = "If a physical good is being shipped, the cost of shipping represented in the smallest currency unit..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[doc = "If a physical good is being shipped, the postal code of where it is being shipped from."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub from_postal_code: Option<String>,
    #[doc = "If a physical good is being shipped, the postal code of where it is being shipped to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub to_postal_code: Option<String>,
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
pub struct PaymentFlowsPaymentIntentAsyncWorkflowsResourceInputsResourceTax {
    #[doc = "The [TaxCalculation](https://docs.stripe.com/api/tax/calculations) id"]
    #[json(default)]
    pub calculation: String,
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
pub enum PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType {
    ApplePay,
    GooglePay,
    SamsungPay,
    UnknownValue,
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
impl PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType::ApplePay => "apple_pay",
            PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType::GooglePay => "google_pay",
            PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType::SamsungPay => "samsung_pay",
            PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType::UnknownValue => "unknown",
            PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType::Unknown(value) => value,
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
impl Default for PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType {
    fn default() -> Self {
        PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType::Unknown(String::new())
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
impl<'json> json::Decode<'json> for PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "apple_pay" => PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType::ApplePay,
            "google_pay" => PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType::GooglePay,
            "samsung_pay" => PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType::SamsungPay,
            "unknown" => PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType::UnknownValue,
            other => PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType::Unknown(other.to_string()),
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
pub enum PaymentFlowsPrivatePaymentMethodsKakaoPayPaymentMethodOptionsCaptureMethod {
    Manual,
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
impl PaymentFlowsPrivatePaymentMethodsKakaoPayPaymentMethodOptionsCaptureMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentFlowsPrivatePaymentMethodsKakaoPayPaymentMethodOptionsCaptureMethod::Manual => "manual",
            PaymentFlowsPrivatePaymentMethodsKakaoPayPaymentMethodOptionsCaptureMethod::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PaymentFlowsPrivatePaymentMethodsKakaoPayPaymentMethodOptionsCaptureMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                PaymentFlowsPrivatePaymentMethodsKakaoPayPaymentMethodOptionsCaptureMethod::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "manual" => PaymentFlowsPrivatePaymentMethodsKakaoPayPaymentMethodOptionsCaptureMethod::Manual,
            other => {
                PaymentFlowsPrivatePaymentMethodsKakaoPayPaymentMethodOptionsCaptureMethod::Unknown(other.to_string())
            }
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
pub enum PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsCategory {
    DigitalGoods,
    Donation,
    PhysicalGoods,
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
impl PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsCategory {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsCategory::DigitalGoods => {
                "digital_goods"
            }
            PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsCategory::Donation => {
                "donation"
            }
            PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsCategory::PhysicalGoods => {
                "physical_goods"
            }
            PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsCategory::Unknown(
                value,
            ) => value,
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
impl<'json> json::Decode<'json>
    for PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsCategory
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsCategory::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "digital_goods" => {
                PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsCategory::DigitalGoods
            }
            "donation" => {
                PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsCategory::Donation
            }
            "physical_goods" => {
                PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsCategory::PhysicalGoods
            }
            other => PaymentFlowsPrivatePaymentMethodsPaypalAmountDetailsLineItemPaymentMethodOptionsCategory::Unknown(
                other.to_string(),
            ),
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
pub struct PaymentFlowsPrivatePaymentMethodsSatispaySetupAttemptDetails {}

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
pub struct PaymentIntentNextActionBlikAuthorize {}

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
pub struct PaymentIntentNextActionCardAwaitNotification {
    #[doc = "The time that payment will be attempted."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub charge_attempt_at: Option<i64>,
    #[doc = "For payments greater than INR 15000, the customer must provide explicit approval of the payment with their bank."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_approval_required: Option<bool>,
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
pub struct PaymentIntentNextActionCashappQrCode {
    #[doc = "The date (unix timestamp) when the QR code expires."]
    #[json(default)]
    pub expires_at: i64,
    #[doc = "The image_url_png string used to render QR code"]
    #[json(default)]
    pub image_url_png: String,
    #[doc = "The image_url_svg string used to render QR code"]
    #[json(default)]
    pub image_url_svg: String,
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
pub struct PaymentIntentNextActionKonbiniSeicomart {
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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentIntentNextActionSwishQrCode {
    #[doc = "The raw data string used to generate QR code, it should be used together with QR code library."]
    #[json(default)]
    pub data: String,
    #[doc = "The timestamp at which the QR code expires."]
    #[json(default)]
    pub expires_at: i64,
    #[doc = "The image_url_png string used to render QR code"]
    #[json(default)]
    pub image_url_png: String,
    #[doc = "The image_url_svg string used to render QR code"]
    #[json(default)]
    pub image_url_svg: String,
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
pub struct PaymentIntentNextActionWechatPayRedirectToIosApp {
    #[doc = "An universal link that redirect to WeChat Pay app"]
    #[json(default)]
    pub native_url: String,
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
pub enum PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
    Challenge,
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
impl PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure::Any => "any",
            PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure::Automatic => "automatic",
            PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure::Challenge => "challenge",
            PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "any" => PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure::Any,
            "automatic" => PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure::Automatic,
            "challenge" => PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure::Challenge,
            other => PaymentIntentPaymentMethodOptionsCardRequestThreeDSecure::Unknown(other.to_string()),
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
pub enum PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    Business,
    Personal,
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
impl PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType::Business => "business",
            PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType::Personal => "personal",
            PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "business" => PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType::Business,
            "personal" => PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType::Personal,
            other => {
                PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType::Unknown(other.to_string())
            }
        })
    }
}
