#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsExcludedPaymentMethodTypesItem {
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

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsExcludedPaymentMethodTypesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::AcssDebit => "acss_debit",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Affirm => "affirm",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::AfterpayClearpay => "afterpay_clearpay",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Alipay => "alipay",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Alma => "alma",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::AmazonPay => "amazon_pay",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::AuBecsDebit => "au_becs_debit",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::BacsDebit => "bacs_debit",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Bancontact => "bancontact",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Billie => "billie",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Bizum => "bizum",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Blik => "blik",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Boleto => "boleto",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Card => "card",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Cashapp => "cashapp",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Crypto => "crypto",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::CustomerBalance => "customer_balance",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Eps => "eps",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Fpx => "fpx",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Giropay => "giropay",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Grabpay => "grabpay",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Ideal => "ideal",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::KakaoPay => "kakao_pay",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Klarna => "klarna",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Konbini => "konbini",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::KrCard => "kr_card",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::MbWay => "mb_way",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Mobilepay => "mobilepay",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Multibanco => "multibanco",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::NaverPay => "naver_pay",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::NzBankAccount => "nz_bank_account",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Oxxo => "oxxo",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::P24 => "p24",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::PayByBank => "pay_by_bank",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Payco => "payco",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Paynow => "paynow",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Paypal => "paypal",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Paypay => "paypay",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Payto => "payto",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Pix => "pix",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Promptpay => "promptpay",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::RevolutPay => "revolut_pay",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::SamsungPay => "samsung_pay",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Satispay => "satispay",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Scalapay => "scalapay",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::SepaDebit => "sepa_debit",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Sequra => "sequra",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Sofort => "sofort",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Sunbit => "sunbit",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Swish => "swish",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Twint => "twint",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Upi => "upi",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::UsBankAccount => "us_bank_account",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::WechatPay => "wechat_pay",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Zip => "zip",
            CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsExcludedPaymentMethodTypesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "acss_debit" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::AcssDebit,
            "affirm" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Affirm,
            "afterpay_clearpay" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::AfterpayClearpay,
            "alipay" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Alipay,
            "alma" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Alma,
            "amazon_pay" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::AmazonPay,
            "au_becs_debit" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::AuBecsDebit,
            "bacs_debit" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::BacsDebit,
            "bancontact" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Bancontact,
            "billie" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Billie,
            "bizum" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Bizum,
            "blik" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Blik,
            "boleto" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Boleto,
            "card" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Card,
            "cashapp" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Cashapp,
            "crypto" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Crypto,
            "customer_balance" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::CustomerBalance,
            "eps" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Eps,
            "fpx" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Fpx,
            "giropay" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Giropay,
            "grabpay" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Grabpay,
            "ideal" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Ideal,
            "kakao_pay" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::KakaoPay,
            "klarna" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Klarna,
            "konbini" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Konbini,
            "kr_card" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::KrCard,
            "mb_way" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::MbWay,
            "mobilepay" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Mobilepay,
            "multibanco" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Multibanco,
            "naver_pay" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::NaverPay,
            "nz_bank_account" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::NzBankAccount,
            "oxxo" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Oxxo,
            "p24" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::P24,
            "pay_by_bank" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::PayByBank,
            "payco" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Payco,
            "paynow" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Paynow,
            "paypal" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Paypal,
            "paypay" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Paypay,
            "payto" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Payto,
            "pix" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Pix,
            "promptpay" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Promptpay,
            "revolut_pay" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::RevolutPay,
            "samsung_pay" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::SamsungPay,
            "satispay" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Satispay,
            "scalapay" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Scalapay,
            "sepa_debit" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::SepaDebit,
            "sequra" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Sequra,
            "sofort" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Sofort,
            "sunbit" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Sunbit,
            "swish" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Swish,
            "twint" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Twint,
            "upi" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Upi,
            "us_bank_account" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::UsBankAccount,
            "wechat_pay" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::WechatPay,
            "zip" => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Zip,
            other => CreateSetupIntentParamsExcludedPaymentMethodTypesItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsExcludedPaymentMethodTypesItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodDataNzBankAccount {
    #[doc = "The name on the bank account."]
    pub account_holder_name: Option<String>,
    #[doc = "The account number for the bank account."]
    pub account_number: String,
    #[doc = "The numeric code for the bank account's bank."]
    pub bank_code: String,
    #[doc = "The numeric code for the bank account's bank branch."]
    pub branch_code: String,
    pub reference: Option<String>,
    #[doc = "The suffix of the bank account number."]
    pub suffix: String,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodDataNzBankAccount {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account_holder_name", self.account_holder_name.as_ref());
        form.field("account_number", &self.account_number);
        form.field("bank_code", &self.bank_code);
        form.field("branch_code", &self.branch_code);
        form.field_opt("reference", self.reference.as_ref());
        form.field("suffix", &self.suffix);
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Business => "business",
            CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Personal => "personal",
            CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "business" => CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Business,
            "personal" => CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Personal,
            other => CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsSupportedTypesItem {
    India,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsSupportedTypesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsSupportedTypesItem::India => "india",
            CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsSupportedTypesItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsSupportedTypesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsSupportedTypesItem::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "india" => CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsSupportedTypesItem::India,
            other => CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsSupportedTypesItem::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsSupportedTypesItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
    Challenge,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodOptionsCardRequestThreeDSecure {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodOptionsCardRequestThreeDSecure::Any => "any",
            CreateSetupIntentParamsPaymentMethodOptionsCardRequestThreeDSecure::Automatic => "automatic",
            CreateSetupIntentParamsPaymentMethodOptionsCardRequestThreeDSecure::Challenge => "challenge",
            CreateSetupIntentParamsPaymentMethodOptionsCardRequestThreeDSecure::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsPaymentMethodOptionsCardRequestThreeDSecure {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSetupIntentParamsPaymentMethodOptionsCardRequestThreeDSecure::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "any" => CreateSetupIntentParamsPaymentMethodOptionsCardRequestThreeDSecure::Any,
            "automatic" => CreateSetupIntentParamsPaymentMethodOptionsCardRequestThreeDSecure::Automatic,
            "challenge" => CreateSetupIntentParamsPaymentMethodOptionsCardRequestThreeDSecure::Challenge,
            other => CreateSetupIntentParamsPaymentMethodOptionsCardRequestThreeDSecure::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsCardRequestThreeDSecure {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator {
    N01,
    N02,
    N05,
    N06,
    N07,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::N01 => "01",
            CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::N02 => "02",
            CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::N05 => "05",
            CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::N06 => "06",
            CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::N07 => "07",
            CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::Unknown(value) => {
                value
            }
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json>
    for CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "01" => CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::N01,
            "02" => CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::N02,
            "05" => CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::N05,
            "06" => CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::N06,
            "07" => CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::N07,
            other => CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode
    for CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureVersion {
    N102,
    N210,
    N220,
    N230,
    N231,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureVersion {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureVersion::N102 => "1.0.2",
            CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureVersion::N210 => "2.1.0",
            CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureVersion::N220 => "2.2.0",
            CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureVersion::N230 => "2.3.0",
            CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureVersion::N231 => "2.3.1",
            CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureVersion::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureVersion {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureVersion::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "1.0.2" => CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureVersion::N102,
            "2.1.0" => CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureVersion::N210,
            "2.2.0" => CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureVersion::N220,
            "2.3.0" => CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureVersion::N230,
            "2.3.1" => CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureVersion::N231,
            other => CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureVersion::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureVersion {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsSingleUse {
    #[doc = "Amount the customer is granting permission to collect later."]
    pub amount: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsSingleUse {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        form.field("currency", &self.currency);
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct ListSetupIntentsParamsCreated {
    #[doc = "Minimum value to filter by (exclusive)"]
    pub gt: Option<i64>,
    #[doc = "Minimum value to filter by (inclusive)"]
    pub gte: Option<i64>,
    #[doc = "Maximum value to filter by (exclusive)"]
    pub lt: Option<i64>,
    #[doc = "Maximum value to filter by (inclusive)"]
    pub lte: Option<i64>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for ListSetupIntentsParamsCreated {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("gt", self.gt.as_ref());
        form.field_opt("gte", self.gte.as_ref());
        form.field_opt("lt", self.lt.as_ref());
        form.field_opt("lte", self.lte.as_ref());
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
pub enum SetupIntentCancellationReason {
    Abandoned,
    Duplicate,
    RequestedByCustomer,
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
impl SetupIntentCancellationReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SetupIntentCancellationReason::Abandoned => "abandoned",
            SetupIntentCancellationReason::Duplicate => "duplicate",
            SetupIntentCancellationReason::RequestedByCustomer => "requested_by_customer",
            SetupIntentCancellationReason::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for SetupIntentCancellationReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SetupIntentCancellationReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "abandoned" => SetupIntentCancellationReason::Abandoned,
            "duplicate" => SetupIntentCancellationReason::Duplicate,
            "requested_by_customer" => SetupIntentCancellationReason::RequestedByCustomer,
            other => SetupIntentCancellationReason::Unknown(other.to_string()),
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
pub enum SetupIntentPaymentMethodOptionsAcssDebitCurrency {
    Cad,
    Usd,
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
impl SetupIntentPaymentMethodOptionsAcssDebitCurrency {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SetupIntentPaymentMethodOptionsAcssDebitCurrency::Cad => "cad",
            SetupIntentPaymentMethodOptionsAcssDebitCurrency::Usd => "usd",
            SetupIntentPaymentMethodOptionsAcssDebitCurrency::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for SetupIntentPaymentMethodOptionsAcssDebitCurrency {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SetupIntentPaymentMethodOptionsAcssDebitCurrency::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "cad" => SetupIntentPaymentMethodOptionsAcssDebitCurrency::Cad,
            "usd" => SetupIntentPaymentMethodOptionsAcssDebitCurrency::Usd,
            other => SetupIntentPaymentMethodOptionsAcssDebitCurrency::Unknown(other.to_string()),
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
pub enum SetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    Day,
    Month,
    Sporadic,
    Week,
    Year,
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
impl SetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SetupIntentPaymentMethodOptionsCardMandateOptionsInterval::Day => "day",
            SetupIntentPaymentMethodOptionsCardMandateOptionsInterval::Month => "month",
            SetupIntentPaymentMethodOptionsCardMandateOptionsInterval::Sporadic => "sporadic",
            SetupIntentPaymentMethodOptionsCardMandateOptionsInterval::Week => "week",
            SetupIntentPaymentMethodOptionsCardMandateOptionsInterval::Year => "year",
            SetupIntentPaymentMethodOptionsCardMandateOptionsInterval::Unknown(value) => value,
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
impl Default for SetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn default() -> Self {
        SetupIntentPaymentMethodOptionsCardMandateOptionsInterval::Unknown(String::new())
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
impl<'json> json::Decode<'json> for SetupIntentPaymentMethodOptionsCardMandateOptionsInterval {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SetupIntentPaymentMethodOptionsCardMandateOptionsInterval::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "day" => SetupIntentPaymentMethodOptionsCardMandateOptionsInterval::Day,
            "month" => SetupIntentPaymentMethodOptionsCardMandateOptionsInterval::Month,
            "sporadic" => SetupIntentPaymentMethodOptionsCardMandateOptionsInterval::Sporadic,
            "week" => SetupIntentPaymentMethodOptionsCardMandateOptionsInterval::Week,
            "year" => SetupIntentPaymentMethodOptionsCardMandateOptionsInterval::Year,
            other => SetupIntentPaymentMethodOptionsCardMandateOptionsInterval::Unknown(other.to_string()),
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
pub struct SetupIntentPaymentMethodOptionsCardPresent {}

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
pub enum SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
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
impl SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::Combined => "combined",
            SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::Interval => "interval",
            SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::Sporadic => "sporadic",
            SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "combined" => SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::Combined,
            "interval" => SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::Interval,
            "sporadic" => SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::Sporadic,
            other => SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule::Unknown(other.to_string()),
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
pub struct SetupIntentPaymentMethodOptionsPaypal {
    #[doc = "The PayPal Billing Agreement ID (BAID)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billing_agreement_id: Option<String>,
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
pub enum SetupIntentStatus {
    Canceled,
    Processing,
    RequiresAction,
    RequiresConfirmation,
    RequiresPaymentMethod,
    Succeeded,
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
impl SetupIntentStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SetupIntentStatus::Canceled => "canceled",
            SetupIntentStatus::Processing => "processing",
            SetupIntentStatus::RequiresAction => "requires_action",
            SetupIntentStatus::RequiresConfirmation => "requires_confirmation",
            SetupIntentStatus::RequiresPaymentMethod => "requires_payment_method",
            SetupIntentStatus::Succeeded => "succeeded",
            SetupIntentStatus::Unknown(value) => value,
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
impl Default for SetupIntentStatus {
    fn default() -> Self {
        SetupIntentStatus::Unknown(String::new())
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
impl<'json> json::Decode<'json> for SetupIntentStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SetupIntentStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "canceled" => SetupIntentStatus::Canceled,
            "processing" => SetupIntentStatus::Processing,
            "requires_action" => SetupIntentStatus::RequiresAction,
            "requires_confirmation" => SetupIntentStatus::RequiresConfirmation,
            "requires_payment_method" => SetupIntentStatus::RequiresPaymentMethod,
            "succeeded" => SetupIntentStatus::Succeeded,
            other => SetupIntentStatus::Unknown(other.to_string()),
        })
    }
}
