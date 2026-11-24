#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum CheckoutAcssDebitPaymentMethodOptionsCurrency {
    Cad,
    Usd,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl CheckoutAcssDebitPaymentMethodOptionsCurrency {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CheckoutAcssDebitPaymentMethodOptionsCurrency::Cad => "cad",
            CheckoutAcssDebitPaymentMethodOptionsCurrency::Usd => "usd",
            CheckoutAcssDebitPaymentMethodOptionsCurrency::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for CheckoutAcssDebitPaymentMethodOptionsCurrency {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CheckoutAcssDebitPaymentMethodOptionsCurrency::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "cad" => CheckoutAcssDebitPaymentMethodOptionsCurrency::Cad,
            "usd" => CheckoutAcssDebitPaymentMethodOptionsCurrency::Usd,
            other => CheckoutAcssDebitPaymentMethodOptionsCurrency::Unknown(other.to_string()),
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
pub enum CheckoutCardPaymentMethodOptionsRequestExtendedAuthorization {
    IfAvailable,
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
impl CheckoutCardPaymentMethodOptionsRequestExtendedAuthorization {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CheckoutCardPaymentMethodOptionsRequestExtendedAuthorization::IfAvailable => "if_available",
            CheckoutCardPaymentMethodOptionsRequestExtendedAuthorization::Never => "never",
            CheckoutCardPaymentMethodOptionsRequestExtendedAuthorization::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for CheckoutCardPaymentMethodOptionsRequestExtendedAuthorization {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CheckoutCardPaymentMethodOptionsRequestExtendedAuthorization::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "if_available" => CheckoutCardPaymentMethodOptionsRequestExtendedAuthorization::IfAvailable,
            "never" => CheckoutCardPaymentMethodOptionsRequestExtendedAuthorization::Never,
            other => CheckoutCardPaymentMethodOptionsRequestExtendedAuthorization::Unknown(other.to_string()),
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
pub enum CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
    UsBankTransfer,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType::EuBankTransfer => "eu_bank_transfer",
            CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType::GbBankTransfer => "gb_bank_transfer",
            CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType::JpBankTransfer => "jp_bank_transfer",
            CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType::MxBankTransfer => "mx_bank_transfer",
            CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType::UsBankTransfer => "us_bank_transfer",
            CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "eu_bank_transfer" => CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType::EuBankTransfer,
            "gb_bank_transfer" => CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType::GbBankTransfer,
            "jp_bank_transfer" => CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType::JpBankTransfer,
            "mx_bank_transfer" => CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType::MxBankTransfer,
            "us_bank_transfer" => CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType::UsBankTransfer,
            other => CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType::Unknown(other.to_string()),
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
pub enum CheckoutSessionOriginContext {
    MobileApp,
    Web,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl CheckoutSessionOriginContext {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CheckoutSessionOriginContext::MobileApp => "mobile_app",
            CheckoutSessionOriginContext::Web => "web",
            CheckoutSessionOriginContext::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for CheckoutSessionOriginContext {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CheckoutSessionOriginContext::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "mobile_app" => CheckoutSessionOriginContext::MobileApp,
            "web" => CheckoutSessionOriginContext::Web,
            other => CheckoutSessionOriginContext::Unknown(other.to_string()),
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
pub enum CheckoutSessionStatus {
    Complete,
    Expired,
    Open,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl CheckoutSessionStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CheckoutSessionStatus::Complete => "complete",
            CheckoutSessionStatus::Expired => "expired",
            CheckoutSessionStatus::Open => "open",
            CheckoutSessionStatus::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for CheckoutSessionStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CheckoutSessionStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "complete" => CheckoutSessionStatus::Complete,
            "expired" => CheckoutSessionStatus::Expired,
            "open" => CheckoutSessionStatus::Open,
            other => CheckoutSessionStatus::Unknown(other.to_string()),
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
pub enum CheckoutSessionUiMode {
    Elements,
    EmbeddedPage,
    Form,
    HostedPage,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl CheckoutSessionUiMode {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CheckoutSessionUiMode::Elements => "elements",
            CheckoutSessionUiMode::EmbeddedPage => "embedded_page",
            CheckoutSessionUiMode::Form => "form",
            CheckoutSessionUiMode::HostedPage => "hosted_page",
            CheckoutSessionUiMode::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for CheckoutSessionUiMode {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CheckoutSessionUiMode::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "elements" => CheckoutSessionUiMode::Elements,
            "embedded_page" => CheckoutSessionUiMode::EmbeddedPage,
            "form" => CheckoutSessionUiMode::Form,
            "hosted_page" => CheckoutSessionUiMode::HostedPage,
            other => CheckoutSessionUiMode::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem {
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

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::AcssDebit => "acss_debit",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Affirm => "affirm",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::AfterpayClearpay => "afterpay_clearpay",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Alipay => "alipay",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Alma => "alma",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::AmazonPay => "amazon_pay",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::AuBecsDebit => "au_becs_debit",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::BacsDebit => "bacs_debit",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Bancontact => "bancontact",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Billie => "billie",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Bizum => "bizum",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Blik => "blik",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Boleto => "boleto",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Card => "card",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Cashapp => "cashapp",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Crypto => "crypto",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::CustomerBalance => "customer_balance",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Eps => "eps",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Fpx => "fpx",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Giropay => "giropay",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Grabpay => "grabpay",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Ideal => "ideal",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::KakaoPay => "kakao_pay",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Klarna => "klarna",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Konbini => "konbini",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::KrCard => "kr_card",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Link => "link",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::MbWay => "mb_way",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Mobilepay => "mobilepay",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Multibanco => "multibanco",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::NaverPay => "naver_pay",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::NzBankAccount => "nz_bank_account",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Oxxo => "oxxo",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::P24 => "p24",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::PayByBank => "pay_by_bank",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Payco => "payco",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Paynow => "paynow",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Paypal => "paypal",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Paypay => "paypay",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Payto => "payto",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Pix => "pix",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Promptpay => "promptpay",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::RevolutPay => "revolut_pay",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::SamsungPay => "samsung_pay",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Satispay => "satispay",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Scalapay => "scalapay",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::SepaDebit => "sepa_debit",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Sequra => "sequra",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Sofort => "sofort",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Sunbit => "sunbit",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Swish => "swish",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Twint => "twint",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Upi => "upi",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::UsBankAccount => "us_bank_account",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::WechatPay => "wechat_pay",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Zip => "zip",
            CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "acss_debit" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::AcssDebit,
            "affirm" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Affirm,
            "afterpay_clearpay" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::AfterpayClearpay,
            "alipay" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Alipay,
            "alma" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Alma,
            "amazon_pay" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::AmazonPay,
            "au_becs_debit" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::AuBecsDebit,
            "bacs_debit" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::BacsDebit,
            "bancontact" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Bancontact,
            "billie" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Billie,
            "bizum" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Bizum,
            "blik" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Blik,
            "boleto" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Boleto,
            "card" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Card,
            "cashapp" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Cashapp,
            "crypto" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Crypto,
            "customer_balance" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::CustomerBalance,
            "eps" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Eps,
            "fpx" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Fpx,
            "giropay" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Giropay,
            "grabpay" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Grabpay,
            "ideal" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Ideal,
            "kakao_pay" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::KakaoPay,
            "klarna" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Klarna,
            "konbini" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Konbini,
            "kr_card" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::KrCard,
            "link" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Link,
            "mb_way" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::MbWay,
            "mobilepay" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Mobilepay,
            "multibanco" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Multibanco,
            "naver_pay" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::NaverPay,
            "nz_bank_account" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::NzBankAccount,
            "oxxo" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Oxxo,
            "p24" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::P24,
            "pay_by_bank" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::PayByBank,
            "payco" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Payco,
            "paynow" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Paynow,
            "paypal" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Paypal,
            "paypay" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Paypay,
            "payto" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Payto,
            "pix" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Pix,
            "promptpay" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Promptpay,
            "revolut_pay" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::RevolutPay,
            "samsung_pay" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::SamsungPay,
            "satispay" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Satispay,
            "scalapay" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Scalapay,
            "sepa_debit" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::SepaDebit,
            "sequra" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Sequra,
            "sofort" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Sofort,
            "sunbit" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Sunbit,
            "swish" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Swish,
            "twint" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Twint,
            "upi" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Upi,
            "us_bank_account" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::UsBankAccount,
            "wechat_pay" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::WechatPay,
            "zip" => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Zip,
            other => CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsAllowedPaymentMethodTypesItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsCustomFieldsItemDropdownOptionsItem {
    #[doc = "The label for the option, displayed to the customer."]
    pub label: String,
    #[doc = "The value for this option, not displayed to the customer, used by your integration to reconcile the option selected b..."]
    pub value: String,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsCustomFieldsItemDropdownOptionsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("label", &self.label);
        form.field("value", &self.value);
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsCustomerCreation {
    Always,
    IfRequired,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsCustomerCreation {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsCustomerCreation::Always => "always",
            CreateCheckoutSessionParamsCustomerCreation::IfRequired => "if_required",
            CreateCheckoutSessionParamsCustomerCreation::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsCustomerCreation {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsCustomerCreation::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "always" => CreateCheckoutSessionParamsCustomerCreation::Always,
            "if_required" => CreateCheckoutSessionParamsCustomerCreation::IfRequired,
            other => CreateCheckoutSessionParamsCustomerCreation::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsCustomerCreation {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsCustomerUpdateAddress {
    Auto,
    Never,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsCustomerUpdateAddress {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsCustomerUpdateAddress::Auto => "auto",
            CreateCheckoutSessionParamsCustomerUpdateAddress::Never => "never",
            CreateCheckoutSessionParamsCustomerUpdateAddress::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsCustomerUpdateAddress {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsCustomerUpdateAddress::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "auto" => CreateCheckoutSessionParamsCustomerUpdateAddress::Auto,
            "never" => CreateCheckoutSessionParamsCustomerUpdateAddress::Never,
            other => CreateCheckoutSessionParamsCustomerUpdateAddress::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsCustomerUpdateAddress {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsOriginContext {
    MobileApp,
    Web,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsOriginContext {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsOriginContext::MobileApp => "mobile_app",
            CreateCheckoutSessionParamsOriginContext::Web => "web",
            CreateCheckoutSessionParamsOriginContext::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsOriginContext {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsOriginContext::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "mobile_app" => CreateCheckoutSessionParamsOriginContext::MobileApp,
            "web" => CreateCheckoutSessionParamsOriginContext::Web,
            other => CreateCheckoutSessionParamsOriginContext::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsOriginContext {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentIntentDataShippingAddress {
    #[doc = "City, district, suburb, town, or village."]
    pub city: Option<String>,
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub country: Option<String>,
    #[doc = "Address line 1, such as the street, PO Box, or company name."]
    pub line1: String,
    #[doc = "Address line 2, such as the apartment, suite, unit, or building."]
    pub line2: Option<String>,
    #[doc = "ZIP or postal code."]
    pub postal_code: Option<String>,
    #[doc = "State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2))."]
    pub state: Option<String>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentIntentDataShippingAddress {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("city", self.city.as_ref());
        form.field_opt("country", self.country.as_ref());
        form.field("line1", &self.line1);
        form.field_opt("line2", self.line2.as_ref());
        form.field_opt("postal_code", self.postal_code.as_ref());
        form.field_opt("state", self.state.as_ref());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitCurrency {
    Cad,
    Usd,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitCurrency {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitCurrency::Cad => "cad",
            CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitCurrency::Usd => "usd",
            CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitCurrency::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitCurrency {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitCurrency::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "cad" => CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitCurrency::Cad,
            "usd" => CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitCurrency::Usd,
            other => CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitCurrency::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsAcssDebitCurrency {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPaymentMethodOptionsAffirmSetupFutureUsage {
    None,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPaymentMethodOptionsAffirmSetupFutureUsage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPaymentMethodOptionsAffirmSetupFutureUsage::None => "none",
            CreateCheckoutSessionParamsPaymentMethodOptionsAffirmSetupFutureUsage::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsPaymentMethodOptionsAffirmSetupFutureUsage {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsPaymentMethodOptionsAffirmSetupFutureUsage::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "none" => CreateCheckoutSessionParamsPaymentMethodOptionsAffirmSetupFutureUsage::None,
            other => CreateCheckoutSessionParamsPaymentMethodOptionsAffirmSetupFutureUsage::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsAffirmSetupFutureUsage {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, Default)]
pub struct CreateCheckoutSessionParamsPaymentMethodOptionsBacsDebitMandateOptions {
    #[doc = "Prefix used to generate the Mandate reference."]
    pub reference_prefix: Option<json::OwnedValue>,
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsBacsDebitMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        let _ = form;
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPaymentMethodOptionsCardRestrictionsFundingTypesBlockedItem {
    Credit,
    Debit,
    Prepaid,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPaymentMethodOptionsCardRestrictionsFundingTypesBlockedItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPaymentMethodOptionsCardRestrictionsFundingTypesBlockedItem::Credit => "credit",
            CreateCheckoutSessionParamsPaymentMethodOptionsCardRestrictionsFundingTypesBlockedItem::Debit => "debit",
            CreateCheckoutSessionParamsPaymentMethodOptionsCardRestrictionsFundingTypesBlockedItem::Prepaid => {
                "prepaid"
            }
            CreateCheckoutSessionParamsPaymentMethodOptionsCardRestrictionsFundingTypesBlockedItem::Unknown(value) => {
                value
            }
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json>
    for CreateCheckoutSessionParamsPaymentMethodOptionsCardRestrictionsFundingTypesBlockedItem
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCheckoutSessionParamsPaymentMethodOptionsCardRestrictionsFundingTypesBlockedItem::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "credit" => CreateCheckoutSessionParamsPaymentMethodOptionsCardRestrictionsFundingTypesBlockedItem::Credit,
            "debit" => CreateCheckoutSessionParamsPaymentMethodOptionsCardRestrictionsFundingTypesBlockedItem::Debit,
            "prepaid" => {
                CreateCheckoutSessionParamsPaymentMethodOptionsCardRestrictionsFundingTypesBlockedItem::Prepaid
            }
            other => CreateCheckoutSessionParamsPaymentMethodOptionsCardRestrictionsFundingTypesBlockedItem::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode
    for CreateCheckoutSessionParamsPaymentMethodOptionsCardRestrictionsFundingTypesBlockedItem
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceFundingType {
    BankTransfer,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceFundingType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceFundingType::BankTransfer => "bank_transfer",
            CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceFundingType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceFundingType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceFundingType::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "bank_transfer" => CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceFundingType::BankTransfer,
            other => {
                CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceFundingType::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsCustomerBalanceFundingType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPaymentMethodOptionsPaypalCaptureMethod {
    Unnamed,
    Manual,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPaymentMethodOptionsPaypalCaptureMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPaymentMethodOptionsPaypalCaptureMethod::Unnamed => "",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaypalCaptureMethod::Manual => "manual",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaypalCaptureMethod::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsPaymentMethodOptionsPaypalCaptureMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsPaymentMethodOptionsPaypalCaptureMethod::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreateCheckoutSessionParamsPaymentMethodOptionsPaypalCaptureMethod::Unnamed,
            "manual" => CreateCheckoutSessionParamsPaymentMethodOptionsPaypalCaptureMethod::Manual,
            other => CreateCheckoutSessionParamsPaymentMethodOptionsPaypalCaptureMethod::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsPaypalCaptureMethod {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule {
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

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Unnamed => "",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Adhoc => "adhoc",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Annual => "annual",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Daily => "daily",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Fortnightly => {
                "fortnightly"
            }
            CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Monthly => "monthly",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Quarterly => "quarterly",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::SemiAnnual => {
                "semi_annual"
            }
            CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Weekly => "weekly",
            CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Unnamed,
            "adhoc" => CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Adhoc,
            "annual" => CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Annual,
            "daily" => CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Daily,
            "fortnightly" => {
                CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Fortnightly
            }
            "monthly" => CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Monthly,
            "quarterly" => CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Quarterly,
            "semi_annual" => {
                CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::SemiAnnual
            }
            "weekly" => CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Weekly,
            other => CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsPaymentMethodOptionsWechatPayClient {
    Android,
    Ios,
    Web,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsPaymentMethodOptionsWechatPayClient {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsPaymentMethodOptionsWechatPayClient::Android => "android",
            CreateCheckoutSessionParamsPaymentMethodOptionsWechatPayClient::Ios => "ios",
            CreateCheckoutSessionParamsPaymentMethodOptionsWechatPayClient::Web => "web",
            CreateCheckoutSessionParamsPaymentMethodOptionsWechatPayClient::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsPaymentMethodOptionsWechatPayClient {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsPaymentMethodOptionsWechatPayClient::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "android" => CreateCheckoutSessionParamsPaymentMethodOptionsWechatPayClient::Android,
            "ios" => CreateCheckoutSessionParamsPaymentMethodOptionsWechatPayClient::Ios,
            "web" => CreateCheckoutSessionParamsPaymentMethodOptionsWechatPayClient::Web,
            other => CreateCheckoutSessionParamsPaymentMethodOptionsWechatPayClient::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsPaymentMethodOptionsWechatPayClient {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataDeliveryEstimateMaximumUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataDeliveryEstimateMaximumUnit {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataDeliveryEstimateMaximumUnit::BusinessDay => {
                "business_day"
            }
            CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataDeliveryEstimateMaximumUnit::Day => "day",
            CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataDeliveryEstimateMaximumUnit::Hour => "hour",
            CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataDeliveryEstimateMaximumUnit::Month => "month",
            CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataDeliveryEstimateMaximumUnit::Week => "week",
            CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataDeliveryEstimateMaximumUnit::Unknown(
                value,
            ) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json>
    for CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataDeliveryEstimateMaximumUnit
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataDeliveryEstimateMaximumUnit::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "business_day" => {
                CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataDeliveryEstimateMaximumUnit::BusinessDay
            }
            "day" => CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataDeliveryEstimateMaximumUnit::Day,
            "hour" => CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataDeliveryEstimateMaximumUnit::Hour,
            "month" => CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataDeliveryEstimateMaximumUnit::Month,
            "week" => CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataDeliveryEstimateMaximumUnit::Week,
            other => {
                CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataDeliveryEstimateMaximumUnit::Unknown(
                    other.to_string(),
                )
            }
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode
    for CreateCheckoutSessionParamsShippingOptionsItemShippingRateDataDeliveryEstimateMaximumUnit
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "checkout")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateCheckoutSessionParamsSubscriptionDataBillingModeType {
    Classic,
    Flexible,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "checkout")]
impl CreateCheckoutSessionParamsSubscriptionDataBillingModeType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateCheckoutSessionParamsSubscriptionDataBillingModeType::Classic => "classic",
            CreateCheckoutSessionParamsSubscriptionDataBillingModeType::Flexible => "flexible",
            CreateCheckoutSessionParamsSubscriptionDataBillingModeType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "checkout")]
impl<'json> json::Decode<'json> for CreateCheckoutSessionParamsSubscriptionDataBillingModeType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateCheckoutSessionParamsSubscriptionDataBillingModeType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "classic" => CreateCheckoutSessionParamsSubscriptionDataBillingModeType::Classic,
            "flexible" => CreateCheckoutSessionParamsSubscriptionDataBillingModeType::Flexible,
            other => CreateCheckoutSessionParamsSubscriptionDataBillingModeType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "checkout")]
impl crate::params::FormEncode for CreateCheckoutSessionParamsSubscriptionDataBillingModeType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
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
pub struct PaymentPagesCheckoutSessionCurrencyConversion {
    #[doc = "Total of all items in source currency before discounts or taxes are applied."]
    #[json(default)]
    pub amount_subtotal: i64,
    #[doc = "Total of all items in source currency after discounts and taxes are applied."]
    #[json(default)]
    pub amount_total: i64,
    #[doc = "Exchange rate used to convert source currency amounts to customer currency amounts"]
    #[json(default)]
    pub fx_rate: String,
    #[doc = "Creation currency of the CheckoutSession before localization"]
    #[json(default)]
    pub source_currency: String,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentPagesCheckoutSessionCustomFieldsNumeric {
    #[doc = "The value that pre-fills the field on the payment page."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[doc = "The maximum character length constraint for the customer's input."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub maximum_length: Option<i64>,
    #[doc = "The minimum character length requirement for the customer's input."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<i64>,
    #[doc = "The value entered by the customer, containing only digits."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentPagesCheckoutSessionCustomFieldsOption {
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
    feature = "events"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentPagesCheckoutSessionIndividualName {
    #[doc = "Indicates whether individual name collection is enabled for the session"]
    #[json(default)]
    pub enabled: bool,
    #[doc = "Whether the customer is required to complete the field before completing the Checkout Session."]
    #[json(default)]
    pub optional: bool,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFiltersItem {
    Always,
    Limited,
    Unspecified,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFiltersItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFiltersItem::Always => "always",
            PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFiltersItem::Limited => "limited",
            PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFiltersItem::Unspecified => "unspecified",
            PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFiltersItem::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFiltersItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFiltersItem::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "always" => PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFiltersItem::Always,
            "limited" => PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFiltersItem::Limited,
            "unspecified" => PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFiltersItem::Unspecified,
            other => PaymentPagesCheckoutSessionSavedPaymentMethodOptionsAllowRedisplayFiltersItem::Unknown(
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
pub enum PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem {
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

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::AC => "AC",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::AD => "AD",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::AE => "AE",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::AF => "AF",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::AG => "AG",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::AI => "AI",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::AL => "AL",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::AM => "AM",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::AO => "AO",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::AQ => "AQ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::AR => "AR",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::AT => "AT",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::AU => "AU",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::AW => "AW",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::AX => "AX",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::AZ => "AZ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BA => "BA",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BB => "BB",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BD => "BD",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BE => "BE",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BF => "BF",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BG => "BG",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BH => "BH",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BI => "BI",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BJ => "BJ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BL => "BL",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BM => "BM",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BN => "BN",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BO => "BO",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BQ => "BQ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BR => "BR",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BS => "BS",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BT => "BT",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BV => "BV",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BW => "BW",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BY => "BY",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BZ => "BZ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::CA => "CA",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::CD => "CD",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::CF => "CF",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::CG => "CG",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::CH => "CH",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::CI => "CI",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::CK => "CK",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::CL => "CL",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::CM => "CM",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::CN => "CN",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::CO => "CO",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::CR => "CR",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::CV => "CV",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::CW => "CW",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::CY => "CY",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::CZ => "CZ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::DE => "DE",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::DJ => "DJ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::DK => "DK",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::DM => "DM",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::DO => "DO",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::DZ => "DZ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::EC => "EC",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::EE => "EE",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::EG => "EG",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::EH => "EH",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::ER => "ER",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::ES => "ES",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::ET => "ET",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::FI => "FI",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::FJ => "FJ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::FK => "FK",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::FO => "FO",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::FR => "FR",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GA => "GA",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GB => "GB",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GD => "GD",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GE => "GE",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GF => "GF",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GG => "GG",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GH => "GH",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GI => "GI",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GL => "GL",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GM => "GM",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GN => "GN",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GP => "GP",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GQ => "GQ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GR => "GR",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GS => "GS",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GT => "GT",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GU => "GU",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GW => "GW",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GY => "GY",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::HK => "HK",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::HN => "HN",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::HR => "HR",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::HT => "HT",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::HU => "HU",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::ID => "ID",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::IE => "IE",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::IL => "IL",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::IM => "IM",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::IN => "IN",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::IO => "IO",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::IQ => "IQ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::IS => "IS",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::IT => "IT",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::JE => "JE",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::JM => "JM",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::JO => "JO",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::JP => "JP",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::KE => "KE",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::KG => "KG",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::KH => "KH",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::KI => "KI",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::KM => "KM",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::KN => "KN",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::KR => "KR",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::KW => "KW",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::KY => "KY",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::KZ => "KZ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::LA => "LA",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::LB => "LB",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::LC => "LC",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::LI => "LI",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::LK => "LK",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::LR => "LR",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::LS => "LS",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::LT => "LT",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::LU => "LU",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::LV => "LV",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::LY => "LY",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MA => "MA",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MC => "MC",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MD => "MD",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::ME => "ME",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MF => "MF",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MG => "MG",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MK => "MK",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::ML => "ML",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MM => "MM",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MN => "MN",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MO => "MO",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MQ => "MQ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MR => "MR",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MS => "MS",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MT => "MT",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MU => "MU",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MV => "MV",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MW => "MW",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MX => "MX",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MY => "MY",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MZ => "MZ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::NA => "NA",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::NC => "NC",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::NE => "NE",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::NG => "NG",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::NI => "NI",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::NL => "NL",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::NO => "NO",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::NP => "NP",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::NR => "NR",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::NU => "NU",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::NZ => "NZ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::OM => "OM",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::PA => "PA",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::PE => "PE",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::PF => "PF",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::PG => "PG",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::PH => "PH",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::PK => "PK",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::PL => "PL",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::PM => "PM",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::PN => "PN",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::PR => "PR",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::PS => "PS",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::PT => "PT",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::PY => "PY",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::QA => "QA",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::RE => "RE",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::RO => "RO",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::RS => "RS",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::RU => "RU",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::RW => "RW",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SA => "SA",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SB => "SB",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SC => "SC",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SD => "SD",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SE => "SE",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SG => "SG",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SH => "SH",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SI => "SI",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SJ => "SJ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SK => "SK",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SL => "SL",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SM => "SM",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SN => "SN",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SO => "SO",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SR => "SR",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SS => "SS",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::ST => "ST",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SV => "SV",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SX => "SX",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SZ => "SZ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TA => "TA",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TC => "TC",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TD => "TD",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TF => "TF",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TG => "TG",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TH => "TH",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TJ => "TJ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TK => "TK",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TL => "TL",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TM => "TM",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TN => "TN",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TO => "TO",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TR => "TR",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TT => "TT",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TV => "TV",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TW => "TW",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TZ => "TZ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::UA => "UA",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::UG => "UG",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::US => "US",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::UY => "UY",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::UZ => "UZ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::VA => "VA",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::VC => "VC",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::VE => "VE",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::VG => "VG",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::VN => "VN",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::VU => "VU",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::WF => "WF",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::WS => "WS",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::XK => "XK",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::YE => "YE",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::YT => "YT",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::ZA => "ZA",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::ZM => "ZM",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::ZW => "ZW",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::ZZ => "ZZ",
            PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::Unknown(value) => value,
        }
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events"
))]
impl<'json> json::Decode<'json> for PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "AC" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::AC,
            "AD" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::AD,
            "AE" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::AE,
            "AF" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::AF,
            "AG" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::AG,
            "AI" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::AI,
            "AL" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::AL,
            "AM" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::AM,
            "AO" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::AO,
            "AQ" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::AQ,
            "AR" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::AR,
            "AT" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::AT,
            "AU" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::AU,
            "AW" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::AW,
            "AX" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::AX,
            "AZ" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::AZ,
            "BA" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BA,
            "BB" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BB,
            "BD" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BD,
            "BE" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BE,
            "BF" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BF,
            "BG" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BG,
            "BH" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BH,
            "BI" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BI,
            "BJ" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BJ,
            "BL" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BL,
            "BM" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BM,
            "BN" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BN,
            "BO" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BO,
            "BQ" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BQ,
            "BR" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BR,
            "BS" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BS,
            "BT" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BT,
            "BV" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BV,
            "BW" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BW,
            "BY" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BY,
            "BZ" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::BZ,
            "CA" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::CA,
            "CD" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::CD,
            "CF" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::CF,
            "CG" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::CG,
            "CH" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::CH,
            "CI" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::CI,
            "CK" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::CK,
            "CL" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::CL,
            "CM" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::CM,
            "CN" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::CN,
            "CO" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::CO,
            "CR" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::CR,
            "CV" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::CV,
            "CW" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::CW,
            "CY" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::CY,
            "CZ" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::CZ,
            "DE" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::DE,
            "DJ" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::DJ,
            "DK" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::DK,
            "DM" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::DM,
            "DO" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::DO,
            "DZ" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::DZ,
            "EC" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::EC,
            "EE" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::EE,
            "EG" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::EG,
            "EH" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::EH,
            "ER" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::ER,
            "ES" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::ES,
            "ET" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::ET,
            "FI" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::FI,
            "FJ" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::FJ,
            "FK" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::FK,
            "FO" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::FO,
            "FR" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::FR,
            "GA" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GA,
            "GB" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GB,
            "GD" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GD,
            "GE" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GE,
            "GF" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GF,
            "GG" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GG,
            "GH" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GH,
            "GI" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GI,
            "GL" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GL,
            "GM" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GM,
            "GN" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GN,
            "GP" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GP,
            "GQ" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GQ,
            "GR" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GR,
            "GS" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GS,
            "GT" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GT,
            "GU" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GU,
            "GW" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GW,
            "GY" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::GY,
            "HK" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::HK,
            "HN" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::HN,
            "HR" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::HR,
            "HT" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::HT,
            "HU" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::HU,
            "ID" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::ID,
            "IE" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::IE,
            "IL" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::IL,
            "IM" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::IM,
            "IN" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::IN,
            "IO" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::IO,
            "IQ" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::IQ,
            "IS" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::IS,
            "IT" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::IT,
            "JE" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::JE,
            "JM" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::JM,
            "JO" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::JO,
            "JP" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::JP,
            "KE" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::KE,
            "KG" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::KG,
            "KH" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::KH,
            "KI" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::KI,
            "KM" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::KM,
            "KN" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::KN,
            "KR" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::KR,
            "KW" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::KW,
            "KY" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::KY,
            "KZ" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::KZ,
            "LA" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::LA,
            "LB" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::LB,
            "LC" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::LC,
            "LI" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::LI,
            "LK" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::LK,
            "LR" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::LR,
            "LS" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::LS,
            "LT" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::LT,
            "LU" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::LU,
            "LV" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::LV,
            "LY" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::LY,
            "MA" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MA,
            "MC" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MC,
            "MD" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MD,
            "ME" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::ME,
            "MF" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MF,
            "MG" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MG,
            "MK" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MK,
            "ML" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::ML,
            "MM" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MM,
            "MN" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MN,
            "MO" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MO,
            "MQ" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MQ,
            "MR" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MR,
            "MS" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MS,
            "MT" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MT,
            "MU" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MU,
            "MV" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MV,
            "MW" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MW,
            "MX" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MX,
            "MY" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MY,
            "MZ" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::MZ,
            "NA" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::NA,
            "NC" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::NC,
            "NE" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::NE,
            "NG" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::NG,
            "NI" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::NI,
            "NL" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::NL,
            "NO" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::NO,
            "NP" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::NP,
            "NR" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::NR,
            "NU" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::NU,
            "NZ" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::NZ,
            "OM" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::OM,
            "PA" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::PA,
            "PE" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::PE,
            "PF" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::PF,
            "PG" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::PG,
            "PH" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::PH,
            "PK" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::PK,
            "PL" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::PL,
            "PM" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::PM,
            "PN" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::PN,
            "PR" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::PR,
            "PS" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::PS,
            "PT" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::PT,
            "PY" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::PY,
            "QA" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::QA,
            "RE" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::RE,
            "RO" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::RO,
            "RS" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::RS,
            "RU" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::RU,
            "RW" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::RW,
            "SA" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SA,
            "SB" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SB,
            "SC" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SC,
            "SD" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SD,
            "SE" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SE,
            "SG" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SG,
            "SH" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SH,
            "SI" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SI,
            "SJ" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SJ,
            "SK" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SK,
            "SL" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SL,
            "SM" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SM,
            "SN" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SN,
            "SO" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SO,
            "SR" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SR,
            "SS" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SS,
            "ST" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::ST,
            "SV" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SV,
            "SX" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SX,
            "SZ" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::SZ,
            "TA" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TA,
            "TC" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TC,
            "TD" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TD,
            "TF" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TF,
            "TG" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TG,
            "TH" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TH,
            "TJ" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TJ,
            "TK" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TK,
            "TL" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TL,
            "TM" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TM,
            "TN" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TN,
            "TO" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TO,
            "TR" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TR,
            "TT" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TT,
            "TV" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TV,
            "TW" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TW,
            "TZ" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::TZ,
            "UA" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::UA,
            "UG" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::UG,
            "US" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::US,
            "UY" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::UY,
            "UZ" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::UZ,
            "VA" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::VA,
            "VC" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::VC,
            "VE" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::VE,
            "VG" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::VG,
            "VN" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::VN,
            "VU" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::VU,
            "WF" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::WF,
            "WS" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::WS,
            "XK" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::XK,
            "YE" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::YE,
            "YT" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::YT,
            "ZA" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::ZA,
            "ZM" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::ZM,
            "ZW" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::ZW,
            "ZZ" => PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::ZZ,
            other => {
                PaymentPagesCheckoutSessionShippingAddressCollectionAllowedCountriesItem::Unknown(other.to_string())
            }
        })
    }
}
