#[cfg(any(feature = "events", feature = "financial_connections"))]
#[derive(Debug, Clone, PartialEq)]
pub enum BankConnectionsResourceAccountNumberDetailsIdentifierType {
    AccountNumber,
    TokenizedAccountNumber,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl BankConnectionsResourceAccountNumberDetailsIdentifierType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BankConnectionsResourceAccountNumberDetailsIdentifierType::AccountNumber => "account_number",
            BankConnectionsResourceAccountNumberDetailsIdentifierType::TokenizedAccountNumber => {
                "tokenized_account_number"
            }
            BankConnectionsResourceAccountNumberDetailsIdentifierType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl Default for BankConnectionsResourceAccountNumberDetailsIdentifierType {
    fn default() -> Self {
        BankConnectionsResourceAccountNumberDetailsIdentifierType::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl<'json> json::Decode<'json> for BankConnectionsResourceAccountNumberDetailsIdentifierType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BankConnectionsResourceAccountNumberDetailsIdentifierType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account_number" => BankConnectionsResourceAccountNumberDetailsIdentifierType::AccountNumber,
            "tokenized_account_number" => {
                BankConnectionsResourceAccountNumberDetailsIdentifierType::TokenizedAccountNumber
            }
            other => BankConnectionsResourceAccountNumberDetailsIdentifierType::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
#[derive(Debug, Clone, PartialEq)]
pub enum BankConnectionsResourceAccountNumberDetailsStatus {
    Deactivated,
    Expired,
    Pending,
    Transactable,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl BankConnectionsResourceAccountNumberDetailsStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BankConnectionsResourceAccountNumberDetailsStatus::Deactivated => "deactivated",
            BankConnectionsResourceAccountNumberDetailsStatus::Expired => "expired",
            BankConnectionsResourceAccountNumberDetailsStatus::Pending => "pending",
            BankConnectionsResourceAccountNumberDetailsStatus::Transactable => "transactable",
            BankConnectionsResourceAccountNumberDetailsStatus::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl Default for BankConnectionsResourceAccountNumberDetailsStatus {
    fn default() -> Self {
        BankConnectionsResourceAccountNumberDetailsStatus::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl<'json> json::Decode<'json> for BankConnectionsResourceAccountNumberDetailsStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BankConnectionsResourceAccountNumberDetailsStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "deactivated" => BankConnectionsResourceAccountNumberDetailsStatus::Deactivated,
            "expired" => BankConnectionsResourceAccountNumberDetailsStatus::Expired,
            "pending" => BankConnectionsResourceAccountNumberDetailsStatus::Pending,
            "transactable" => BankConnectionsResourceAccountNumberDetailsStatus::Transactable,
            other => BankConnectionsResourceAccountNumberDetailsStatus::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
#[derive(Debug, Clone, PartialEq)]
pub enum BankConnectionsResourceBalanceRefreshStatus {
    Failed,
    Pending,
    Succeeded,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl BankConnectionsResourceBalanceRefreshStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BankConnectionsResourceBalanceRefreshStatus::Failed => "failed",
            BankConnectionsResourceBalanceRefreshStatus::Pending => "pending",
            BankConnectionsResourceBalanceRefreshStatus::Succeeded => "succeeded",
            BankConnectionsResourceBalanceRefreshStatus::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl Default for BankConnectionsResourceBalanceRefreshStatus {
    fn default() -> Self {
        BankConnectionsResourceBalanceRefreshStatus::Unknown(String::new())
    }
}

#[cfg(any(feature = "events", feature = "financial_connections"))]
impl<'json> json::Decode<'json> for BankConnectionsResourceBalanceRefreshStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BankConnectionsResourceBalanceRefreshStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "failed" => BankConnectionsResourceBalanceRefreshStatus::Failed,
            "pending" => BankConnectionsResourceBalanceRefreshStatus::Pending,
            "succeeded" => BankConnectionsResourceBalanceRefreshStatus::Succeeded,
            other => BankConnectionsResourceBalanceRefreshStatus::Unknown(other.to_string()),
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
pub struct CardIssuingAccountTermsOfService {
    #[doc = "The Unix timestamp marking when the account representative accepted the service agreement."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<i64>,
    #[doc = "The IP address from which the account representative accepted the service agreement."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[doc = "The user agent of the browser from which the account representative accepted the service agreement."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[cfg(feature = "payment_methods")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentMethodParamsP24Bank {
    AliorBank,
    BankMillennium,
    BankNowyBfgSa,
    BankPekaoSa,
    BankiSpbdzielcze,
    Blik,
    BnpParibas,
    Boz,
    CitiHandlowy,
    CreditAgricole,
    Envelobank,
    EtransferPocztowy24,
    GetinBank,
    Ideabank,
    Ing,
    Inteligo,
    MbankMtransfer,
    NestPrzelew,
    NoblePay,
    PbacZIpko,
    PlusBank,
    SantanderPrzelew24,
    TmobileUsbugiBankowe,
    ToyotaBank,
    Velobank,
    VolkswagenBank,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_methods")]
impl CreatePaymentMethodParamsP24Bank {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentMethodParamsP24Bank::AliorBank => "alior_bank",
            CreatePaymentMethodParamsP24Bank::BankMillennium => "bank_millennium",
            CreatePaymentMethodParamsP24Bank::BankNowyBfgSa => "bank_nowy_bfg_sa",
            CreatePaymentMethodParamsP24Bank::BankPekaoSa => "bank_pekao_sa",
            CreatePaymentMethodParamsP24Bank::BankiSpbdzielcze => "banki_spbdzielcze",
            CreatePaymentMethodParamsP24Bank::Blik => "blik",
            CreatePaymentMethodParamsP24Bank::BnpParibas => "bnp_paribas",
            CreatePaymentMethodParamsP24Bank::Boz => "boz",
            CreatePaymentMethodParamsP24Bank::CitiHandlowy => "citi_handlowy",
            CreatePaymentMethodParamsP24Bank::CreditAgricole => "credit_agricole",
            CreatePaymentMethodParamsP24Bank::Envelobank => "envelobank",
            CreatePaymentMethodParamsP24Bank::EtransferPocztowy24 => "etransfer_pocztowy24",
            CreatePaymentMethodParamsP24Bank::GetinBank => "getin_bank",
            CreatePaymentMethodParamsP24Bank::Ideabank => "ideabank",
            CreatePaymentMethodParamsP24Bank::Ing => "ing",
            CreatePaymentMethodParamsP24Bank::Inteligo => "inteligo",
            CreatePaymentMethodParamsP24Bank::MbankMtransfer => "mbank_mtransfer",
            CreatePaymentMethodParamsP24Bank::NestPrzelew => "nest_przelew",
            CreatePaymentMethodParamsP24Bank::NoblePay => "noble_pay",
            CreatePaymentMethodParamsP24Bank::PbacZIpko => "pbac_z_ipko",
            CreatePaymentMethodParamsP24Bank::PlusBank => "plus_bank",
            CreatePaymentMethodParamsP24Bank::SantanderPrzelew24 => "santander_przelew24",
            CreatePaymentMethodParamsP24Bank::TmobileUsbugiBankowe => "tmobile_usbugi_bankowe",
            CreatePaymentMethodParamsP24Bank::ToyotaBank => "toyota_bank",
            CreatePaymentMethodParamsP24Bank::Velobank => "velobank",
            CreatePaymentMethodParamsP24Bank::VolkswagenBank => "volkswagen_bank",
            CreatePaymentMethodParamsP24Bank::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_methods")]
impl<'json> json::Decode<'json> for CreatePaymentMethodParamsP24Bank {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentMethodParamsP24Bank::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "alior_bank" => CreatePaymentMethodParamsP24Bank::AliorBank,
            "bank_millennium" => CreatePaymentMethodParamsP24Bank::BankMillennium,
            "bank_nowy_bfg_sa" => CreatePaymentMethodParamsP24Bank::BankNowyBfgSa,
            "bank_pekao_sa" => CreatePaymentMethodParamsP24Bank::BankPekaoSa,
            "banki_spbdzielcze" => CreatePaymentMethodParamsP24Bank::BankiSpbdzielcze,
            "blik" => CreatePaymentMethodParamsP24Bank::Blik,
            "bnp_paribas" => CreatePaymentMethodParamsP24Bank::BnpParibas,
            "boz" => CreatePaymentMethodParamsP24Bank::Boz,
            "citi_handlowy" => CreatePaymentMethodParamsP24Bank::CitiHandlowy,
            "credit_agricole" => CreatePaymentMethodParamsP24Bank::CreditAgricole,
            "envelobank" => CreatePaymentMethodParamsP24Bank::Envelobank,
            "etransfer_pocztowy24" => CreatePaymentMethodParamsP24Bank::EtransferPocztowy24,
            "getin_bank" => CreatePaymentMethodParamsP24Bank::GetinBank,
            "ideabank" => CreatePaymentMethodParamsP24Bank::Ideabank,
            "ing" => CreatePaymentMethodParamsP24Bank::Ing,
            "inteligo" => CreatePaymentMethodParamsP24Bank::Inteligo,
            "mbank_mtransfer" => CreatePaymentMethodParamsP24Bank::MbankMtransfer,
            "nest_przelew" => CreatePaymentMethodParamsP24Bank::NestPrzelew,
            "noble_pay" => CreatePaymentMethodParamsP24Bank::NoblePay,
            "pbac_z_ipko" => CreatePaymentMethodParamsP24Bank::PbacZIpko,
            "plus_bank" => CreatePaymentMethodParamsP24Bank::PlusBank,
            "santander_przelew24" => CreatePaymentMethodParamsP24Bank::SantanderPrzelew24,
            "tmobile_usbugi_bankowe" => CreatePaymentMethodParamsP24Bank::TmobileUsbugiBankowe,
            "toyota_bank" => CreatePaymentMethodParamsP24Bank::ToyotaBank,
            "velobank" => CreatePaymentMethodParamsP24Bank::Velobank,
            "volkswagen_bank" => CreatePaymentMethodParamsP24Bank::VolkswagenBank,
            other => CreatePaymentMethodParamsP24Bank::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_methods")]
impl crate::params::FormEncode for CreatePaymentMethodParamsP24Bank {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_methods")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodParamsSepaDebit {
    #[doc = "IBAN of the bank account."]
    pub iban: String,
}

#[cfg(feature = "payment_methods")]
impl crate::params::FormEncode for CreatePaymentMethodParamsSepaDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("iban", &self.iban);
    }
}

#[cfg(feature = "payment_methods")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentMethodParamsSofortCountry {
    AT,
    BE,
    DE,
    ES,
    IT,
    NL,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_methods")]
impl CreatePaymentMethodParamsSofortCountry {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentMethodParamsSofortCountry::AT => "AT",
            CreatePaymentMethodParamsSofortCountry::BE => "BE",
            CreatePaymentMethodParamsSofortCountry::DE => "DE",
            CreatePaymentMethodParamsSofortCountry::ES => "ES",
            CreatePaymentMethodParamsSofortCountry::IT => "IT",
            CreatePaymentMethodParamsSofortCountry::NL => "NL",
            CreatePaymentMethodParamsSofortCountry::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_methods")]
impl<'json> json::Decode<'json> for CreatePaymentMethodParamsSofortCountry {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentMethodParamsSofortCountry::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "AT" => CreatePaymentMethodParamsSofortCountry::AT,
            "BE" => CreatePaymentMethodParamsSofortCountry::BE,
            "DE" => CreatePaymentMethodParamsSofortCountry::DE,
            "ES" => CreatePaymentMethodParamsSofortCountry::ES,
            "IT" => CreatePaymentMethodParamsSofortCountry::IT,
            "NL" => CreatePaymentMethodParamsSofortCountry::NL,
            other => CreatePaymentMethodParamsSofortCountry::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_methods")]
impl crate::params::FormEncode for CreatePaymentMethodParamsSofortCountry {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_methods")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentMethodParamsUsBankAccountAccountType {
    Checking,
    Savings,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_methods")]
impl CreatePaymentMethodParamsUsBankAccountAccountType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentMethodParamsUsBankAccountAccountType::Checking => "checking",
            CreatePaymentMethodParamsUsBankAccountAccountType::Savings => "savings",
            CreatePaymentMethodParamsUsBankAccountAccountType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_methods")]
impl<'json> json::Decode<'json> for CreatePaymentMethodParamsUsBankAccountAccountType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentMethodParamsUsBankAccountAccountType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "checking" => CreatePaymentMethodParamsUsBankAccountAccountType::Checking,
            "savings" => CreatePaymentMethodParamsUsBankAccountAccountType::Savings,
            other => CreatePaymentMethodParamsUsBankAccountAccountType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_methods")]
impl crate::params::FormEncode for CreatePaymentMethodParamsUsBankAccountAccountType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_methods")]
#[derive(Debug, Clone, PartialEq)]
pub enum ListPaymentMethodsParamsType {
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
    Custom,
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

#[cfg(feature = "payment_methods")]
impl ListPaymentMethodsParamsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            ListPaymentMethodsParamsType::AcssDebit => "acss_debit",
            ListPaymentMethodsParamsType::Affirm => "affirm",
            ListPaymentMethodsParamsType::AfterpayClearpay => "afterpay_clearpay",
            ListPaymentMethodsParamsType::Alipay => "alipay",
            ListPaymentMethodsParamsType::Alma => "alma",
            ListPaymentMethodsParamsType::AmazonPay => "amazon_pay",
            ListPaymentMethodsParamsType::AuBecsDebit => "au_becs_debit",
            ListPaymentMethodsParamsType::BacsDebit => "bacs_debit",
            ListPaymentMethodsParamsType::Bancontact => "bancontact",
            ListPaymentMethodsParamsType::Billie => "billie",
            ListPaymentMethodsParamsType::Bizum => "bizum",
            ListPaymentMethodsParamsType::Blik => "blik",
            ListPaymentMethodsParamsType::Boleto => "boleto",
            ListPaymentMethodsParamsType::Card => "card",
            ListPaymentMethodsParamsType::Cashapp => "cashapp",
            ListPaymentMethodsParamsType::Crypto => "crypto",
            ListPaymentMethodsParamsType::Custom => "custom",
            ListPaymentMethodsParamsType::CustomerBalance => "customer_balance",
            ListPaymentMethodsParamsType::Eps => "eps",
            ListPaymentMethodsParamsType::Fpx => "fpx",
            ListPaymentMethodsParamsType::Giropay => "giropay",
            ListPaymentMethodsParamsType::Grabpay => "grabpay",
            ListPaymentMethodsParamsType::Ideal => "ideal",
            ListPaymentMethodsParamsType::KakaoPay => "kakao_pay",
            ListPaymentMethodsParamsType::Klarna => "klarna",
            ListPaymentMethodsParamsType::Konbini => "konbini",
            ListPaymentMethodsParamsType::KrCard => "kr_card",
            ListPaymentMethodsParamsType::Link => "link",
            ListPaymentMethodsParamsType::MbWay => "mb_way",
            ListPaymentMethodsParamsType::Mobilepay => "mobilepay",
            ListPaymentMethodsParamsType::Multibanco => "multibanco",
            ListPaymentMethodsParamsType::NaverPay => "naver_pay",
            ListPaymentMethodsParamsType::NzBankAccount => "nz_bank_account",
            ListPaymentMethodsParamsType::Oxxo => "oxxo",
            ListPaymentMethodsParamsType::P24 => "p24",
            ListPaymentMethodsParamsType::PayByBank => "pay_by_bank",
            ListPaymentMethodsParamsType::Payco => "payco",
            ListPaymentMethodsParamsType::Paynow => "paynow",
            ListPaymentMethodsParamsType::Paypal => "paypal",
            ListPaymentMethodsParamsType::Paypay => "paypay",
            ListPaymentMethodsParamsType::Payto => "payto",
            ListPaymentMethodsParamsType::Pix => "pix",
            ListPaymentMethodsParamsType::Promptpay => "promptpay",
            ListPaymentMethodsParamsType::RevolutPay => "revolut_pay",
            ListPaymentMethodsParamsType::SamsungPay => "samsung_pay",
            ListPaymentMethodsParamsType::Satispay => "satispay",
            ListPaymentMethodsParamsType::Scalapay => "scalapay",
            ListPaymentMethodsParamsType::SepaDebit => "sepa_debit",
            ListPaymentMethodsParamsType::Sequra => "sequra",
            ListPaymentMethodsParamsType::Sofort => "sofort",
            ListPaymentMethodsParamsType::Sunbit => "sunbit",
            ListPaymentMethodsParamsType::Swish => "swish",
            ListPaymentMethodsParamsType::Twint => "twint",
            ListPaymentMethodsParamsType::Upi => "upi",
            ListPaymentMethodsParamsType::UsBankAccount => "us_bank_account",
            ListPaymentMethodsParamsType::WechatPay => "wechat_pay",
            ListPaymentMethodsParamsType::Zip => "zip",
            ListPaymentMethodsParamsType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_methods")]
impl<'json> json::Decode<'json> for ListPaymentMethodsParamsType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(ListPaymentMethodsParamsType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "acss_debit" => ListPaymentMethodsParamsType::AcssDebit,
            "affirm" => ListPaymentMethodsParamsType::Affirm,
            "afterpay_clearpay" => ListPaymentMethodsParamsType::AfterpayClearpay,
            "alipay" => ListPaymentMethodsParamsType::Alipay,
            "alma" => ListPaymentMethodsParamsType::Alma,
            "amazon_pay" => ListPaymentMethodsParamsType::AmazonPay,
            "au_becs_debit" => ListPaymentMethodsParamsType::AuBecsDebit,
            "bacs_debit" => ListPaymentMethodsParamsType::BacsDebit,
            "bancontact" => ListPaymentMethodsParamsType::Bancontact,
            "billie" => ListPaymentMethodsParamsType::Billie,
            "bizum" => ListPaymentMethodsParamsType::Bizum,
            "blik" => ListPaymentMethodsParamsType::Blik,
            "boleto" => ListPaymentMethodsParamsType::Boleto,
            "card" => ListPaymentMethodsParamsType::Card,
            "cashapp" => ListPaymentMethodsParamsType::Cashapp,
            "crypto" => ListPaymentMethodsParamsType::Crypto,
            "custom" => ListPaymentMethodsParamsType::Custom,
            "customer_balance" => ListPaymentMethodsParamsType::CustomerBalance,
            "eps" => ListPaymentMethodsParamsType::Eps,
            "fpx" => ListPaymentMethodsParamsType::Fpx,
            "giropay" => ListPaymentMethodsParamsType::Giropay,
            "grabpay" => ListPaymentMethodsParamsType::Grabpay,
            "ideal" => ListPaymentMethodsParamsType::Ideal,
            "kakao_pay" => ListPaymentMethodsParamsType::KakaoPay,
            "klarna" => ListPaymentMethodsParamsType::Klarna,
            "konbini" => ListPaymentMethodsParamsType::Konbini,
            "kr_card" => ListPaymentMethodsParamsType::KrCard,
            "link" => ListPaymentMethodsParamsType::Link,
            "mb_way" => ListPaymentMethodsParamsType::MbWay,
            "mobilepay" => ListPaymentMethodsParamsType::Mobilepay,
            "multibanco" => ListPaymentMethodsParamsType::Multibanco,
            "naver_pay" => ListPaymentMethodsParamsType::NaverPay,
            "nz_bank_account" => ListPaymentMethodsParamsType::NzBankAccount,
            "oxxo" => ListPaymentMethodsParamsType::Oxxo,
            "p24" => ListPaymentMethodsParamsType::P24,
            "pay_by_bank" => ListPaymentMethodsParamsType::PayByBank,
            "payco" => ListPaymentMethodsParamsType::Payco,
            "paynow" => ListPaymentMethodsParamsType::Paynow,
            "paypal" => ListPaymentMethodsParamsType::Paypal,
            "paypay" => ListPaymentMethodsParamsType::Paypay,
            "payto" => ListPaymentMethodsParamsType::Payto,
            "pix" => ListPaymentMethodsParamsType::Pix,
            "promptpay" => ListPaymentMethodsParamsType::Promptpay,
            "revolut_pay" => ListPaymentMethodsParamsType::RevolutPay,
            "samsung_pay" => ListPaymentMethodsParamsType::SamsungPay,
            "satispay" => ListPaymentMethodsParamsType::Satispay,
            "scalapay" => ListPaymentMethodsParamsType::Scalapay,
            "sepa_debit" => ListPaymentMethodsParamsType::SepaDebit,
            "sequra" => ListPaymentMethodsParamsType::Sequra,
            "sofort" => ListPaymentMethodsParamsType::Sofort,
            "sunbit" => ListPaymentMethodsParamsType::Sunbit,
            "swish" => ListPaymentMethodsParamsType::Swish,
            "twint" => ListPaymentMethodsParamsType::Twint,
            "upi" => ListPaymentMethodsParamsType::Upi,
            "us_bank_account" => ListPaymentMethodsParamsType::UsBankAccount,
            "wechat_pay" => ListPaymentMethodsParamsType::WechatPay,
            "zip" => ListPaymentMethodsParamsType::Zip,
            other => ListPaymentMethodsParamsType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_methods")]
impl crate::params::FormEncode for ListPaymentMethodsParamsType {
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
pub struct PaymentMethodAlma {}

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
pub struct PaymentMethodBacsDebit {
    #[doc = "Uniquely identifies this particular bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[doc = "Last four digits of the bank account number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[doc = "Sort code of the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<String>,
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
pub struct PaymentMethodBlik {
    #[doc = "A unique and immutable identifier assigned by BLIK to every buyer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub buyer_id: Option<String>,
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
pub struct PaymentMethodBoleto {
    #[doc = "Uniquely identifies the customer tax id (CNPJ or CPF)"]
    #[json(default)]
    pub tax_id: String,
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
pub struct PaymentMethodCrypto {}

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
pub enum PaymentMethodDetailsAchDebitAccountHolderType {
    Company,
    Individual,
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
impl PaymentMethodDetailsAchDebitAccountHolderType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentMethodDetailsAchDebitAccountHolderType::Company => "company",
            PaymentMethodDetailsAchDebitAccountHolderType::Individual => "individual",
            PaymentMethodDetailsAchDebitAccountHolderType::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PaymentMethodDetailsAchDebitAccountHolderType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentMethodDetailsAchDebitAccountHolderType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "company" => PaymentMethodDetailsAchDebitAccountHolderType::Company,
            "individual" => PaymentMethodDetailsAchDebitAccountHolderType::Individual,
            other => PaymentMethodDetailsAchDebitAccountHolderType::Unknown(other.to_string()),
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
pub enum PaymentMethodDetailsCardPresentOfflineType {
    Deferred,
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
impl PaymentMethodDetailsCardPresentOfflineType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentMethodDetailsCardPresentOfflineType::Deferred => "deferred",
            PaymentMethodDetailsCardPresentOfflineType::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PaymentMethodDetailsCardPresentOfflineType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentMethodDetailsCardPresentOfflineType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "deferred" => PaymentMethodDetailsCardPresentOfflineType::Deferred,
            other => PaymentMethodDetailsCardPresentOfflineType::Unknown(other.to_string()),
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
pub struct PaymentMethodDetailsCardWalletApplePay {}

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
pub struct PaymentMethodDetailsKakaoPay {
    #[doc = "A unique identifier for the buyer as determined by the local payment processor."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub buyer_id: Option<String>,
    #[doc = "The Kakao Pay transaction ID associated with this payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
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
pub enum PaymentMethodDetailsP24Bank {
    AliorBank,
    BankMillennium,
    BankNowyBfgSa,
    BankPekaoSa,
    BankiSpbdzielcze,
    Blik,
    BnpParibas,
    Boz,
    CitiHandlowy,
    CreditAgricole,
    Envelobank,
    EtransferPocztowy24,
    GetinBank,
    Ideabank,
    Ing,
    Inteligo,
    MbankMtransfer,
    NestPrzelew,
    NoblePay,
    PbacZIpko,
    PlusBank,
    SantanderPrzelew24,
    TmobileUsbugiBankowe,
    ToyotaBank,
    Velobank,
    VolkswagenBank,
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
impl PaymentMethodDetailsP24Bank {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentMethodDetailsP24Bank::AliorBank => "alior_bank",
            PaymentMethodDetailsP24Bank::BankMillennium => "bank_millennium",
            PaymentMethodDetailsP24Bank::BankNowyBfgSa => "bank_nowy_bfg_sa",
            PaymentMethodDetailsP24Bank::BankPekaoSa => "bank_pekao_sa",
            PaymentMethodDetailsP24Bank::BankiSpbdzielcze => "banki_spbdzielcze",
            PaymentMethodDetailsP24Bank::Blik => "blik",
            PaymentMethodDetailsP24Bank::BnpParibas => "bnp_paribas",
            PaymentMethodDetailsP24Bank::Boz => "boz",
            PaymentMethodDetailsP24Bank::CitiHandlowy => "citi_handlowy",
            PaymentMethodDetailsP24Bank::CreditAgricole => "credit_agricole",
            PaymentMethodDetailsP24Bank::Envelobank => "envelobank",
            PaymentMethodDetailsP24Bank::EtransferPocztowy24 => "etransfer_pocztowy24",
            PaymentMethodDetailsP24Bank::GetinBank => "getin_bank",
            PaymentMethodDetailsP24Bank::Ideabank => "ideabank",
            PaymentMethodDetailsP24Bank::Ing => "ing",
            PaymentMethodDetailsP24Bank::Inteligo => "inteligo",
            PaymentMethodDetailsP24Bank::MbankMtransfer => "mbank_mtransfer",
            PaymentMethodDetailsP24Bank::NestPrzelew => "nest_przelew",
            PaymentMethodDetailsP24Bank::NoblePay => "noble_pay",
            PaymentMethodDetailsP24Bank::PbacZIpko => "pbac_z_ipko",
            PaymentMethodDetailsP24Bank::PlusBank => "plus_bank",
            PaymentMethodDetailsP24Bank::SantanderPrzelew24 => "santander_przelew24",
            PaymentMethodDetailsP24Bank::TmobileUsbugiBankowe => "tmobile_usbugi_bankowe",
            PaymentMethodDetailsP24Bank::ToyotaBank => "toyota_bank",
            PaymentMethodDetailsP24Bank::Velobank => "velobank",
            PaymentMethodDetailsP24Bank::VolkswagenBank => "volkswagen_bank",
            PaymentMethodDetailsP24Bank::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PaymentMethodDetailsP24Bank {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentMethodDetailsP24Bank::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "alior_bank" => PaymentMethodDetailsP24Bank::AliorBank,
            "bank_millennium" => PaymentMethodDetailsP24Bank::BankMillennium,
            "bank_nowy_bfg_sa" => PaymentMethodDetailsP24Bank::BankNowyBfgSa,
            "bank_pekao_sa" => PaymentMethodDetailsP24Bank::BankPekaoSa,
            "banki_spbdzielcze" => PaymentMethodDetailsP24Bank::BankiSpbdzielcze,
            "blik" => PaymentMethodDetailsP24Bank::Blik,
            "bnp_paribas" => PaymentMethodDetailsP24Bank::BnpParibas,
            "boz" => PaymentMethodDetailsP24Bank::Boz,
            "citi_handlowy" => PaymentMethodDetailsP24Bank::CitiHandlowy,
            "credit_agricole" => PaymentMethodDetailsP24Bank::CreditAgricole,
            "envelobank" => PaymentMethodDetailsP24Bank::Envelobank,
            "etransfer_pocztowy24" => PaymentMethodDetailsP24Bank::EtransferPocztowy24,
            "getin_bank" => PaymentMethodDetailsP24Bank::GetinBank,
            "ideabank" => PaymentMethodDetailsP24Bank::Ideabank,
            "ing" => PaymentMethodDetailsP24Bank::Ing,
            "inteligo" => PaymentMethodDetailsP24Bank::Inteligo,
            "mbank_mtransfer" => PaymentMethodDetailsP24Bank::MbankMtransfer,
            "nest_przelew" => PaymentMethodDetailsP24Bank::NestPrzelew,
            "noble_pay" => PaymentMethodDetailsP24Bank::NoblePay,
            "pbac_z_ipko" => PaymentMethodDetailsP24Bank::PbacZIpko,
            "plus_bank" => PaymentMethodDetailsP24Bank::PlusBank,
            "santander_przelew24" => PaymentMethodDetailsP24Bank::SantanderPrzelew24,
            "tmobile_usbugi_bankowe" => PaymentMethodDetailsP24Bank::TmobileUsbugiBankowe,
            "toyota_bank" => PaymentMethodDetailsP24Bank::ToyotaBank,
            "velobank" => PaymentMethodDetailsP24Bank::Velobank,
            "volkswagen_bank" => PaymentMethodDetailsP24Bank::VolkswagenBank,
            other => PaymentMethodDetailsP24Bank::Unknown(other.to_string()),
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
pub struct PaymentMethodDetailsPaymentRecordAcssDebit {
    #[doc = "Name of the bank associated with the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[doc = "Estimated date to debit the customer's bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub expected_debit_date: Option<String>,
    #[doc = "Uniquely identifies this particular bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[doc = "Institution number of the bank account"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub institution_number: Option<String>,
    #[doc = "Last four digits of the bank account number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[doc = "ID of the mandate used to make this payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate: Option<String>,
    #[doc = "Transit number of the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transit_number: Option<String>,
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
pub struct PaymentMethodDetailsPaymentRecordNaverPay {
    #[doc = "A unique identifier for the buyer as determined by the local payment processor."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub buyer_id: Option<String>,
    #[doc = "The Naver Pay transaction ID associated with this payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
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
pub struct PaymentMethodDetailsPaymentRecordNzBankAccount {
    #[doc = "The name on the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<String>,
    #[doc = "The numeric code for the bank account's bank."]
    #[json(default)]
    pub bank_code: String,
    #[doc = "The name of the bank."]
    #[json(default)]
    pub bank_name: String,
    #[doc = "The numeric code for the bank account's bank branch."]
    #[json(default)]
    pub branch_code: String,
    #[doc = "Estimated date to debit the customer's bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub expected_debit_date: Option<String>,
    #[doc = "Last four digits of the bank account number."]
    #[json(default)]
    pub last4: String,
    #[doc = "The suffix of the bank account number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
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
pub struct PaymentMethodDetailsPaymentRecordSamsungPay {
    #[doc = "A unique identifier for the buyer as determined by the local payment processor."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub buyer_id: Option<String>,
    #[doc = "The Samsung Pay transaction ID associated with this payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
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
pub struct PaymentMethodDetailsPaymentRecordWechatPay {
    #[doc = "Uniquely identifies this particular WeChat Pay account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[doc = "ID of the [location](https://docs.stripe.com/api/terminal/locations) that this transaction's reader is assigned to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "ID of the [reader](https://docs.stripe.com/api/terminal/readers) this transaction was made on."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reader: Option<String>,
    #[doc = "Transaction ID of this particular WeChat Pay transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
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
pub struct PaymentMethodDetailsPaypay {}

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
pub struct PaymentMethodDetailsSatispay {
    #[doc = "The Satispay transaction ID associated with this payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
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
pub struct PaymentMethodDetailsSwish {
    #[doc = "Uniquely identifies the payer's Swish account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[doc = "Payer bank reference number for the payment"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_reference: Option<String>,
    #[doc = "The last four digits of the Swish account phone number"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verified_phone_last4: Option<String>,
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
pub struct PaymentMethodDetailsWechatPay {
    #[doc = "Uniquely identifies this particular WeChat Pay account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[doc = "ID of the [location](https://docs.stripe.com/api/terminal/locations) that this transaction's reader is assigned to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "ID of the [reader](https://docs.stripe.com/api/terminal/readers) this transaction was made on."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reader: Option<String>,
    #[doc = "Transaction ID of this particular WeChat Pay transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
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
pub struct PaymentMethodGrabpay {}

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
pub struct PaymentMethodNzBankAccount {
    #[doc = "The name on the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<String>,
    #[doc = "The numeric code for the bank account's bank."]
    #[json(default)]
    pub bank_code: String,
    #[doc = "The name of the bank."]
    #[json(default)]
    pub bank_name: String,
    #[doc = "The numeric code for the bank account's bank branch."]
    #[json(default)]
    pub branch_code: String,
    #[doc = "Last four digits of the bank account number."]
    #[json(default)]
    pub last4: String,
    #[doc = "The suffix of the bank account number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
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
pub enum PaymentMethodOptionsCardMandateOptionsSupportedTypesItem {
    India,
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
impl PaymentMethodOptionsCardMandateOptionsSupportedTypesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentMethodOptionsCardMandateOptionsSupportedTypesItem::India => "india",
            PaymentMethodOptionsCardMandateOptionsSupportedTypesItem::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PaymentMethodOptionsCardMandateOptionsSupportedTypesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentMethodOptionsCardMandateOptionsSupportedTypesItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "india" => PaymentMethodOptionsCardMandateOptionsSupportedTypesItem::India,
            other => PaymentMethodOptionsCardMandateOptionsSupportedTypesItem::Unknown(other.to_string()),
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
pub enum PaymentMethodOptionsCustomerBalanceEuBankAccountCountry {
    BE,
    DE,
    ES,
    FR,
    IE,
    NL,
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
impl PaymentMethodOptionsCustomerBalanceEuBankAccountCountry {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentMethodOptionsCustomerBalanceEuBankAccountCountry::BE => "BE",
            PaymentMethodOptionsCustomerBalanceEuBankAccountCountry::DE => "DE",
            PaymentMethodOptionsCustomerBalanceEuBankAccountCountry::ES => "ES",
            PaymentMethodOptionsCustomerBalanceEuBankAccountCountry::FR => "FR",
            PaymentMethodOptionsCustomerBalanceEuBankAccountCountry::IE => "IE",
            PaymentMethodOptionsCustomerBalanceEuBankAccountCountry::NL => "NL",
            PaymentMethodOptionsCustomerBalanceEuBankAccountCountry::Unknown(value) => value,
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
impl Default for PaymentMethodOptionsCustomerBalanceEuBankAccountCountry {
    fn default() -> Self {
        PaymentMethodOptionsCustomerBalanceEuBankAccountCountry::Unknown(String::new())
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
impl<'json> json::Decode<'json> for PaymentMethodOptionsCustomerBalanceEuBankAccountCountry {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentMethodOptionsCustomerBalanceEuBankAccountCountry::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "BE" => PaymentMethodOptionsCustomerBalanceEuBankAccountCountry::BE,
            "DE" => PaymentMethodOptionsCustomerBalanceEuBankAccountCountry::DE,
            "ES" => PaymentMethodOptionsCustomerBalanceEuBankAccountCountry::ES,
            "FR" => PaymentMethodOptionsCustomerBalanceEuBankAccountCountry::FR,
            "IE" => PaymentMethodOptionsCustomerBalanceEuBankAccountCountry::IE,
            "NL" => PaymentMethodOptionsCustomerBalanceEuBankAccountCountry::NL,
            other => PaymentMethodOptionsCustomerBalanceEuBankAccountCountry::Unknown(other.to_string()),
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
pub enum PaymentMethodOptionsMandateOptionsPixPaymentSchedule {
    Halfyearly,
    Monthly,
    Quarterly,
    Weekly,
    Yearly,
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
impl PaymentMethodOptionsMandateOptionsPixPaymentSchedule {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentMethodOptionsMandateOptionsPixPaymentSchedule::Halfyearly => "halfyearly",
            PaymentMethodOptionsMandateOptionsPixPaymentSchedule::Monthly => "monthly",
            PaymentMethodOptionsMandateOptionsPixPaymentSchedule::Quarterly => "quarterly",
            PaymentMethodOptionsMandateOptionsPixPaymentSchedule::Weekly => "weekly",
            PaymentMethodOptionsMandateOptionsPixPaymentSchedule::Yearly => "yearly",
            PaymentMethodOptionsMandateOptionsPixPaymentSchedule::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PaymentMethodOptionsMandateOptionsPixPaymentSchedule {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentMethodOptionsMandateOptionsPixPaymentSchedule::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "halfyearly" => PaymentMethodOptionsMandateOptionsPixPaymentSchedule::Halfyearly,
            "monthly" => PaymentMethodOptionsMandateOptionsPixPaymentSchedule::Monthly,
            "quarterly" => PaymentMethodOptionsMandateOptionsPixPaymentSchedule::Quarterly,
            "weekly" => PaymentMethodOptionsMandateOptionsPixPaymentSchedule::Weekly,
            "yearly" => PaymentMethodOptionsMandateOptionsPixPaymentSchedule::Yearly,
            other => PaymentMethodOptionsMandateOptionsPixPaymentSchedule::Unknown(other.to_string()),
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
pub struct PaymentMethodOptionsPaypay {}

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
pub struct PaymentMethodPayByBank {}

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
pub struct PaymentMethodSamsungPay {}

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
pub struct PaymentMethodScalapay {}
