#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsAllowedPaymentMethodTypesItem {
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
    BokuPromptpay,
    Boleto,
    CapchasePay,
    Card,
    CardPresent,
    Cashapp,
    CheckScan,
    ClickToPay,
    Crypto,
    CustomerBalance,
    DemoPay,
    Duitnow,
    DummyAuthPush,
    DummyPassthroughCard,
    Edenred,
    Eps,
    Fpx,
    Gcash,
    Getbalance,
    GiftCard,
    Giropay,
    Gopay,
    Grabpay,
    IdBankTransfer,
    Ideal,
    InteracPresent,
    KakaoPay,
    Klarna,
    Knet,
    Konbini,
    KrCard,
    KrMarket,
    Kriya,
    Link,
    MbWay,
    Mobilepay,
    Momo,
    Mondu,
    Multibanco,
    NaverPay,
    Netbanking,
    NgBank,
    NgBankTransfer,
    NgCard,
    NgMarket,
    NgUssd,
    NgWallet,
    NzBankAccount,
    Octopus,
    Oxxo,
    P24,
    PaperCheck,
    PayByBank,
    Payco,
    Paynow,
    Paypal,
    Paypay,
    Payto,
    Pix,
    Promptpay,
    Qris,
    Rechnung,
    RevolutPay,
    SamsungPay,
    Satispay,
    Scalapay,
    SepaDebit,
    Sequra,
    ShopPay,
    Shopeepay,
    Sofort,
    SouthKoreaMarket,
    StripeBalance,
    Sunbit,
    Swish,
    Tamara,
    TestPay,
    TouchNGo,
    Truemoney,
    Twint,
    Upi,
    UsBankAccount,
    UsCashVoucher,
    Vipps,
    WechatPay,
    Wero,
    Zip,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsAllowedPaymentMethodTypesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::AcssDebit => "acss_debit",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Affirm => "affirm",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::AfterpayClearpay => "afterpay_clearpay",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Alipay => "alipay",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Alma => "alma",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::AmazonPay => "amazon_pay",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::AuBecsDebit => "au_becs_debit",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::BacsDebit => "bacs_debit",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Bancontact => "bancontact",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Billie => "billie",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Bizum => "bizum",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Blik => "blik",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::BokuPromptpay => "boku_promptpay",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Boleto => "boleto",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::CapchasePay => "capchase_pay",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Card => "card",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::CardPresent => "card_present",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Cashapp => "cashapp",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::CheckScan => "check_scan",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::ClickToPay => "click_to_pay",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Crypto => "crypto",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::CustomerBalance => "customer_balance",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::DemoPay => "demo_pay",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Duitnow => "duitnow",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::DummyAuthPush => "dummy_auth_push",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::DummyPassthroughCard => "dummy_passthrough_card",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Edenred => "edenred",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Eps => "eps",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Fpx => "fpx",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Gcash => "gcash",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Getbalance => "getbalance",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::GiftCard => "gift_card",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Giropay => "giropay",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Gopay => "gopay",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Grabpay => "grabpay",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::IdBankTransfer => "id_bank_transfer",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Ideal => "ideal",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::InteracPresent => "interac_present",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::KakaoPay => "kakao_pay",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Klarna => "klarna",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Knet => "knet",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Konbini => "konbini",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::KrCard => "kr_card",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::KrMarket => "kr_market",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Kriya => "kriya",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Link => "link",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::MbWay => "mb_way",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Mobilepay => "mobilepay",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Momo => "momo",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Mondu => "mondu",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Multibanco => "multibanco",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::NaverPay => "naver_pay",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Netbanking => "netbanking",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::NgBank => "ng_bank",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::NgBankTransfer => "ng_bank_transfer",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::NgCard => "ng_card",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::NgMarket => "ng_market",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::NgUssd => "ng_ussd",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::NgWallet => "ng_wallet",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::NzBankAccount => "nz_bank_account",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Octopus => "octopus",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Oxxo => "oxxo",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::P24 => "p24",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::PaperCheck => "paper_check",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::PayByBank => "pay_by_bank",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Payco => "payco",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Paynow => "paynow",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Paypal => "paypal",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Paypay => "paypay",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Payto => "payto",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Pix => "pix",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Promptpay => "promptpay",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Qris => "qris",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Rechnung => "rechnung",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::RevolutPay => "revolut_pay",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::SamsungPay => "samsung_pay",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Satispay => "satispay",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Scalapay => "scalapay",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::SepaDebit => "sepa_debit",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Sequra => "sequra",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::ShopPay => "shop_pay",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Shopeepay => "shopeepay",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Sofort => "sofort",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::SouthKoreaMarket => "south_korea_market",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::StripeBalance => "stripe_balance",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Sunbit => "sunbit",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Swish => "swish",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Tamara => "tamara",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::TestPay => "test_pay",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::TouchNGo => "touch_n_go",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Truemoney => "truemoney",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Twint => "twint",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Upi => "upi",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::UsBankAccount => "us_bank_account",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::UsCashVoucher => "us_cash_voucher",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Vipps => "vipps",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::WechatPay => "wechat_pay",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Wero => "wero",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Zip => "zip",
            CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsAllowedPaymentMethodTypesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "acss_debit" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::AcssDebit,
            "affirm" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Affirm,
            "afterpay_clearpay" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::AfterpayClearpay,
            "alipay" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Alipay,
            "alma" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Alma,
            "amazon_pay" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::AmazonPay,
            "au_becs_debit" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::AuBecsDebit,
            "bacs_debit" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::BacsDebit,
            "bancontact" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Bancontact,
            "billie" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Billie,
            "bizum" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Bizum,
            "blik" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Blik,
            "boku_promptpay" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::BokuPromptpay,
            "boleto" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Boleto,
            "capchase_pay" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::CapchasePay,
            "card" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Card,
            "card_present" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::CardPresent,
            "cashapp" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Cashapp,
            "check_scan" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::CheckScan,
            "click_to_pay" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::ClickToPay,
            "crypto" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Crypto,
            "customer_balance" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::CustomerBalance,
            "demo_pay" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::DemoPay,
            "duitnow" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Duitnow,
            "dummy_auth_push" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::DummyAuthPush,
            "dummy_passthrough_card" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::DummyPassthroughCard,
            "edenred" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Edenred,
            "eps" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Eps,
            "fpx" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Fpx,
            "gcash" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Gcash,
            "getbalance" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Getbalance,
            "gift_card" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::GiftCard,
            "giropay" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Giropay,
            "gopay" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Gopay,
            "grabpay" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Grabpay,
            "id_bank_transfer" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::IdBankTransfer,
            "ideal" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Ideal,
            "interac_present" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::InteracPresent,
            "kakao_pay" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::KakaoPay,
            "klarna" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Klarna,
            "knet" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Knet,
            "konbini" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Konbini,
            "kr_card" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::KrCard,
            "kr_market" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::KrMarket,
            "kriya" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Kriya,
            "link" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Link,
            "mb_way" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::MbWay,
            "mobilepay" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Mobilepay,
            "momo" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Momo,
            "mondu" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Mondu,
            "multibanco" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Multibanco,
            "naver_pay" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::NaverPay,
            "netbanking" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Netbanking,
            "ng_bank" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::NgBank,
            "ng_bank_transfer" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::NgBankTransfer,
            "ng_card" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::NgCard,
            "ng_market" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::NgMarket,
            "ng_ussd" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::NgUssd,
            "ng_wallet" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::NgWallet,
            "nz_bank_account" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::NzBankAccount,
            "octopus" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Octopus,
            "oxxo" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Oxxo,
            "p24" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::P24,
            "paper_check" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::PaperCheck,
            "pay_by_bank" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::PayByBank,
            "payco" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Payco,
            "paynow" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Paynow,
            "paypal" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Paypal,
            "paypay" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Paypay,
            "payto" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Payto,
            "pix" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Pix,
            "promptpay" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Promptpay,
            "qris" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Qris,
            "rechnung" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Rechnung,
            "revolut_pay" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::RevolutPay,
            "samsung_pay" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::SamsungPay,
            "satispay" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Satispay,
            "scalapay" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Scalapay,
            "sepa_debit" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::SepaDebit,
            "sequra" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Sequra,
            "shop_pay" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::ShopPay,
            "shopeepay" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Shopeepay,
            "sofort" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Sofort,
            "south_korea_market" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::SouthKoreaMarket,
            "stripe_balance" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::StripeBalance,
            "sunbit" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Sunbit,
            "swish" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Swish,
            "tamara" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Tamara,
            "test_pay" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::TestPay,
            "touch_n_go" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::TouchNGo,
            "truemoney" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Truemoney,
            "twint" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Twint,
            "upi" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Upi,
            "us_bank_account" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::UsBankAccount,
            "us_cash_voucher" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::UsCashVoucher,
            "vipps" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Vipps,
            "wechat_pay" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::WechatPay,
            "wero" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Wero,
            "zip" => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Zip,
            other => CreatePaymentIntentParamsAllowedPaymentMethodTypesItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsAllowedPaymentMethodTypesItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodDataAllowRedisplay {
    Always,
    Limited,
    Unspecified,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodDataAllowRedisplay {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodDataAllowRedisplay::Always => "always",
            CreatePaymentIntentParamsPaymentMethodDataAllowRedisplay::Limited => "limited",
            CreatePaymentIntentParamsPaymentMethodDataAllowRedisplay::Unspecified => "unspecified",
            CreatePaymentIntentParamsPaymentMethodDataAllowRedisplay::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodDataAllowRedisplay {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsPaymentMethodDataAllowRedisplay::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "always" => CreatePaymentIntentParamsPaymentMethodDataAllowRedisplay::Always,
            "limited" => CreatePaymentIntentParamsPaymentMethodDataAllowRedisplay::Limited,
            "unspecified" => CreatePaymentIntentParamsPaymentMethodDataAllowRedisplay::Unspecified,
            other => CreatePaymentIntentParamsPaymentMethodDataAllowRedisplay::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodDataAllowRedisplay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodDataBillingDetailsAddress {
    #[doc = "City, district, suburb, town, or village."]
    pub city: Option<String>,
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    pub country: Option<String>,
    #[doc = "Address line 1, such as the street, PO Box, or company name."]
    pub line1: Option<String>,
    #[doc = "Address line 2, such as the apartment, suite, unit, or building."]
    pub line2: Option<String>,
    #[doc = "ZIP or postal code."]
    pub postal_code: Option<String>,
    #[doc = "State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2))."]
    pub state: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodDataBillingDetailsAddress {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("city", self.city.as_ref());
        form.field_opt("country", self.country.as_ref());
        form.field_opt("line1", self.line1.as_ref());
        form.field_opt("line2", self.line2.as_ref());
        form.field_opt("postal_code", self.postal_code.as_ref());
        form.field_opt("state", self.state.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodDataIdealBank {
    AbnAmro,
    Adyen,
    AsnBank,
    Bunq,
    Buut,
    Finom,
    Handelsbanken,
    Ing,
    Knab,
    Mollie,
    Moneyou,
    N26,
    Nn,
    Rabobank,
    Regiobank,
    Revolut,
    SnsBank,
    TriodosBank,
    VanLanschot,
    Yoursafe,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodDataIdealBank {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodDataIdealBank::AbnAmro => "abn_amro",
            CreatePaymentIntentParamsPaymentMethodDataIdealBank::Adyen => "adyen",
            CreatePaymentIntentParamsPaymentMethodDataIdealBank::AsnBank => "asn_bank",
            CreatePaymentIntentParamsPaymentMethodDataIdealBank::Bunq => "bunq",
            CreatePaymentIntentParamsPaymentMethodDataIdealBank::Buut => "buut",
            CreatePaymentIntentParamsPaymentMethodDataIdealBank::Finom => "finom",
            CreatePaymentIntentParamsPaymentMethodDataIdealBank::Handelsbanken => "handelsbanken",
            CreatePaymentIntentParamsPaymentMethodDataIdealBank::Ing => "ing",
            CreatePaymentIntentParamsPaymentMethodDataIdealBank::Knab => "knab",
            CreatePaymentIntentParamsPaymentMethodDataIdealBank::Mollie => "mollie",
            CreatePaymentIntentParamsPaymentMethodDataIdealBank::Moneyou => "moneyou",
            CreatePaymentIntentParamsPaymentMethodDataIdealBank::N26 => "n26",
            CreatePaymentIntentParamsPaymentMethodDataIdealBank::Nn => "nn",
            CreatePaymentIntentParamsPaymentMethodDataIdealBank::Rabobank => "rabobank",
            CreatePaymentIntentParamsPaymentMethodDataIdealBank::Regiobank => "regiobank",
            CreatePaymentIntentParamsPaymentMethodDataIdealBank::Revolut => "revolut",
            CreatePaymentIntentParamsPaymentMethodDataIdealBank::SnsBank => "sns_bank",
            CreatePaymentIntentParamsPaymentMethodDataIdealBank::TriodosBank => "triodos_bank",
            CreatePaymentIntentParamsPaymentMethodDataIdealBank::VanLanschot => "van_lanschot",
            CreatePaymentIntentParamsPaymentMethodDataIdealBank::Yoursafe => "yoursafe",
            CreatePaymentIntentParamsPaymentMethodDataIdealBank::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodDataIdealBank {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsPaymentMethodDataIdealBank::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "abn_amro" => CreatePaymentIntentParamsPaymentMethodDataIdealBank::AbnAmro,
            "adyen" => CreatePaymentIntentParamsPaymentMethodDataIdealBank::Adyen,
            "asn_bank" => CreatePaymentIntentParamsPaymentMethodDataIdealBank::AsnBank,
            "bunq" => CreatePaymentIntentParamsPaymentMethodDataIdealBank::Bunq,
            "buut" => CreatePaymentIntentParamsPaymentMethodDataIdealBank::Buut,
            "finom" => CreatePaymentIntentParamsPaymentMethodDataIdealBank::Finom,
            "handelsbanken" => CreatePaymentIntentParamsPaymentMethodDataIdealBank::Handelsbanken,
            "ing" => CreatePaymentIntentParamsPaymentMethodDataIdealBank::Ing,
            "knab" => CreatePaymentIntentParamsPaymentMethodDataIdealBank::Knab,
            "mollie" => CreatePaymentIntentParamsPaymentMethodDataIdealBank::Mollie,
            "moneyou" => CreatePaymentIntentParamsPaymentMethodDataIdealBank::Moneyou,
            "n26" => CreatePaymentIntentParamsPaymentMethodDataIdealBank::N26,
            "nn" => CreatePaymentIntentParamsPaymentMethodDataIdealBank::Nn,
            "rabobank" => CreatePaymentIntentParamsPaymentMethodDataIdealBank::Rabobank,
            "regiobank" => CreatePaymentIntentParamsPaymentMethodDataIdealBank::Regiobank,
            "revolut" => CreatePaymentIntentParamsPaymentMethodDataIdealBank::Revolut,
            "sns_bank" => CreatePaymentIntentParamsPaymentMethodDataIdealBank::SnsBank,
            "triodos_bank" => CreatePaymentIntentParamsPaymentMethodDataIdealBank::TriodosBank,
            "van_lanschot" => CreatePaymentIntentParamsPaymentMethodDataIdealBank::VanLanschot,
            "yoursafe" => CreatePaymentIntentParamsPaymentMethodDataIdealBank::Yoursafe,
            other => CreatePaymentIntentParamsPaymentMethodDataIdealBank::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodDataIdealBank {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage {
    Unnamed,
    None,
    OffSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage::Unnamed => "",
            CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage::None => "none",
            CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage::OffSession => "off_session",
            CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage::Unnamed,
            "none" => CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage::None,
            "off_session" => CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage::OffSession,
            other => CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsBlikMandateOptions {
    #[doc = "Expiry date of the mandate."]
    pub expires_at: Option<i64>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsBlikMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("expires_at", self.expires_at.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsInterval {
    Day,
    Month,
    Sporadic,
    Week,
    Year,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsInterval {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsInterval::Day => "day",
            CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsInterval::Month => "month",
            CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsInterval::Sporadic => "sporadic",
            CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsInterval::Week => "week",
            CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsInterval::Year => "year",
            CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsInterval::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsInterval {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsInterval::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "day" => CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsInterval::Day,
            "month" => CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsInterval::Month,
            "sporadic" => CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsInterval::Sporadic,
            "week" => CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsInterval::Week,
            "year" => CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsInterval::Year,
            other => {
                CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsInterval::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsInterval {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsSupportedTypesItem {
    India,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsSupportedTypesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsSupportedTypesItem::India => "india",
            CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsSupportedTypesItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsSupportedTypesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsSupportedTypesItem::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "india" => CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsSupportedTypesItem::India,
            other => CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsSupportedTypesItem::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsSupportedTypesItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator {
    N01,
    N02,
    N05,
    N06,
    N07,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::N01 => "01",
            CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::N02 => "02",
            CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::N05 => "05",
            CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::N06 => "06",
            CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::N07 => "07",
            CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::Unknown(
                value,
            ) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json>
    for CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "01" => CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::N01,
            "02" => CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::N02,
            "05" => CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::N05,
            "06" => CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::N06,
            "07" => CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::N07,
            other => CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode
    for CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer {
    #[doc = "The desired country code of the bank account information."]
    pub country: String,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode
    for CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("country", &self.country);
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem {
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

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::Aba => "aba",
            CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::Iban => "iban",
            CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::Sepa => "sepa",
            CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::SortCode => "sort_code",
            CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::Spei => "spei",
            CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::Swift => "swift",
            CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::Zengin => "zengin",
            CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json>
    for CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "aba" => CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::Aba,
            "iban" => CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::Iban,
            "sepa" => CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::Sepa,
            "sort_code" => CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::SortCode,
            "spei" => CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::Spei,
            "swift" => CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::Swift,
            "zengin" => CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::Zengin,
            other => CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode
    for CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
    UsBankTransfer,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferType::EuBankTransfer => {
                "eu_bank_transfer"
            }
            CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferType::GbBankTransfer => {
                "gb_bank_transfer"
            }
            CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferType::JpBankTransfer => {
                "jp_bank_transfer"
            }
            CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferType::MxBankTransfer => {
                "mx_bank_transfer"
            }
            CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferType::UsBankTransfer => {
                "us_bank_transfer"
            }
            CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferType::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "eu_bank_transfer" => {
                CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferType::EuBankTransfer
            }
            "gb_bank_transfer" => {
                CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferType::GbBankTransfer
            }
            "jp_bank_transfer" => {
                CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferType::JpBankTransfer
            }
            "mx_bank_transfer" => {
                CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferType::MxBankTransfer
            }
            "us_bank_transfer" => {
                CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferType::UsBankTransfer
            }
            other => {
                CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferType::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval {
    Day,
    Month,
    Week,
    Year,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval::Day => "day",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval::Month => "month",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval::Week => "week",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval::Year => "year",
            CreatePaymentIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreatePaymentIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "day" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval::Day,
            "month" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval::Month,
            "week" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval::Week,
            "year" => CreatePaymentIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval::Year,
            other => {
                CreatePaymentIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule {
    Halfyearly,
    Monthly,
    Quarterly,
    Weekly,
    Yearly,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl CreatePaymentIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Halfyearly => "halfyearly",
            CreatePaymentIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Monthly => "monthly",
            CreatePaymentIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Quarterly => "quarterly",
            CreatePaymentIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Weekly => "weekly",
            CreatePaymentIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Yearly => "yearly",
            CreatePaymentIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for CreatePaymentIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreatePaymentIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "halfyearly" => CreatePaymentIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Halfyearly,
            "monthly" => CreatePaymentIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Monthly,
            "quarterly" => CreatePaymentIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Quarterly,
            "weekly" => CreatePaymentIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Weekly,
            "yearly" => CreatePaymentIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Yearly,
            other => CreatePaymentIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "customers")]
#[derive(Debug, Clone, PartialEq)]
pub enum FundingInstructionsObject {
    FundingInstructions,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "customers")]
impl FundingInstructionsObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            FundingInstructionsObject::FundingInstructions => "funding_instructions",
            FundingInstructionsObject::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "customers")]
impl Default for FundingInstructionsObject {
    fn default() -> Self {
        FundingInstructionsObject::Unknown(String::new())
    }
}

#[cfg(feature = "customers")]
impl<'json> json::Decode<'json> for FundingInstructionsObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(FundingInstructionsObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "funding_instructions" => FundingInstructionsObject::FundingInstructions,
            other => FundingInstructionsObject::Unknown(other.to_string()),
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
pub struct PaymentFlowsPaymentIntentPresentmentDetails {
    #[doc = "Amount intended to be collected by this payment, denominated in `presentment_currency`."]
    #[json(default)]
    pub presentment_amount: i64,
    #[doc = "Currency presented to the customer during payment."]
    #[json(default)]
    pub presentment_currency: String,
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
pub struct PaymentFlowsPrivatePaymentMethodsAlmaDetailsResourceInstallments {
    #[doc = "The number of installments."]
    #[json(default)]
    pub count: i64,
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
pub enum PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType {
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
impl PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType::ChEin => "ch_ein",
            PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType::DeHrb => "de_hrb",
            PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType::DkCvr => "dk_cvr",
            PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType::EsCif => "es_cif",
            PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType::FiTunnus => "fi_tunnus",
            PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType::FrSiren => "fr_siren",
            PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType::FrSiret => "fr_siret",
            PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType::ItRea => "it_rea",
            PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType::NlKvk => "nl_kvk",
            PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType::NoOrgNumber => "no_org_number",
            PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType::NoPno => "no_pno",
            PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType::SeOrgNumber => "se_org_number",
            PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType::SePno => "se_pno",
            PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType::UkCrn => "uk_crn",
            PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType::Unknown(value) => value,
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
    for PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "ch_ein" => PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType::ChEin,
            "de_hrb" => PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType::DeHrb,
            "dk_cvr" => PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType::DkCvr,
            "es_cif" => PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType::EsCif,
            "fi_tunnus" => PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType::FiTunnus,
            "fr_siren" => PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType::FrSiren,
            "fr_siret" => PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType::FrSiret,
            "it_rea" => PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType::ItRea,
            "nl_kvk" => PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType::NlKvk,
            "no_org_number" => PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType::NoOrgNumber,
            "no_pno" => PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType::NoPno,
            "se_org_number" => PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType::SeOrgNumber,
            "se_pno" => PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType::SePno,
            "uk_crn" => PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType::UkCrn,
            other => PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType::Unknown(other.to_string()),
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
pub enum PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorizationStatus
{
    Available,
    Unavailable,
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
impl PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorizationStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorizationStatus::Available => "available",
            PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorizationStatus::Unavailable => "unavailable",
            PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorizationStatus::Unknown(value) => value,
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
impl Default for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorizationStatus {
    fn default() -> Self {
        PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorizationStatus::Unknown(String::new())
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
impl<'json> json::Decode<'json> for PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorizationStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorizationStatus::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "available" => PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorizationStatus::Available,
            "unavailable" => PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorizationStatus::Unavailable,
            other => PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorizationStatus::Unknown(other.to_string()),
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
pub struct PaymentFlowsPrivatePaymentMethodsCardPresentAmountDetailsLineItemPaymentMethodOptions {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub commodity_code: Option<String>,
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
pub enum PaymentFlowsPrivatePaymentMethodsKakaoPayPaymentMethodOptionsSetupFutureUsage {
    None,
    OffSession,
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
impl PaymentFlowsPrivatePaymentMethodsKakaoPayPaymentMethodOptionsSetupFutureUsage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentFlowsPrivatePaymentMethodsKakaoPayPaymentMethodOptionsSetupFutureUsage::None => "none",
            PaymentFlowsPrivatePaymentMethodsKakaoPayPaymentMethodOptionsSetupFutureUsage::OffSession => "off_session",
            PaymentFlowsPrivatePaymentMethodsKakaoPayPaymentMethodOptionsSetupFutureUsage::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PaymentFlowsPrivatePaymentMethodsKakaoPayPaymentMethodOptionsSetupFutureUsage {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                PaymentFlowsPrivatePaymentMethodsKakaoPayPaymentMethodOptionsSetupFutureUsage::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "none" => PaymentFlowsPrivatePaymentMethodsKakaoPayPaymentMethodOptionsSetupFutureUsage::None,
            "off_session" => PaymentFlowsPrivatePaymentMethodsKakaoPayPaymentMethodOptionsSetupFutureUsage::OffSession,
            other => PaymentFlowsPrivatePaymentMethodsKakaoPayPaymentMethodOptionsSetupFutureUsage::Unknown(
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
pub struct PaymentFlowsPrivatePaymentMethodsKlarnaDob {
    #[doc = "The day of birth, between 1 and 31."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub day: Option<i64>,
    #[doc = "The month of birth, between 1 and 12."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub month: Option<i64>,
    #[doc = "The four-digit year of birth."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub year: Option<i64>,
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
pub enum PaymentIntentCancellationReason {
    Abandoned,
    Automatic,
    Duplicate,
    Expired,
    FailedInvoice,
    Fraudulent,
    RequestedByCustomer,
    VoidInvoice,
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
impl PaymentIntentCancellationReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentIntentCancellationReason::Abandoned => "abandoned",
            PaymentIntentCancellationReason::Automatic => "automatic",
            PaymentIntentCancellationReason::Duplicate => "duplicate",
            PaymentIntentCancellationReason::Expired => "expired",
            PaymentIntentCancellationReason::FailedInvoice => "failed_invoice",
            PaymentIntentCancellationReason::Fraudulent => "fraudulent",
            PaymentIntentCancellationReason::RequestedByCustomer => "requested_by_customer",
            PaymentIntentCancellationReason::VoidInvoice => "void_invoice",
            PaymentIntentCancellationReason::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PaymentIntentCancellationReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentIntentCancellationReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "abandoned" => PaymentIntentCancellationReason::Abandoned,
            "automatic" => PaymentIntentCancellationReason::Automatic,
            "duplicate" => PaymentIntentCancellationReason::Duplicate,
            "expired" => PaymentIntentCancellationReason::Expired,
            "failed_invoice" => PaymentIntentCancellationReason::FailedInvoice,
            "fraudulent" => PaymentIntentCancellationReason::Fraudulent,
            "requested_by_customer" => PaymentIntentCancellationReason::RequestedByCustomer,
            "void_invoice" => PaymentIntentCancellationReason::VoidInvoice,
            other => PaymentIntentCancellationReason::Unknown(other.to_string()),
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
pub enum PaymentIntentCaptureMethod {
    Automatic,
    AutomaticAsync,
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
impl PaymentIntentCaptureMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentIntentCaptureMethod::Automatic => "automatic",
            PaymentIntentCaptureMethod::AutomaticAsync => "automatic_async",
            PaymentIntentCaptureMethod::Manual => "manual",
            PaymentIntentCaptureMethod::Unknown(value) => value,
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
impl Default for PaymentIntentCaptureMethod {
    fn default() -> Self {
        PaymentIntentCaptureMethod::Unknown(String::new())
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
impl<'json> json::Decode<'json> for PaymentIntentCaptureMethod {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentIntentCaptureMethod::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "automatic" => PaymentIntentCaptureMethod::Automatic,
            "automatic_async" => PaymentIntentCaptureMethod::AutomaticAsync,
            "manual" => PaymentIntentCaptureMethod::Manual,
            other => PaymentIntentCaptureMethod::Unknown(other.to_string()),
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
pub struct PaymentIntentNextActionPromptpayDisplayQrCode {
    #[doc = "The raw data string used to generate QR code, it should be used together with QR code library."]
    #[json(default)]
    pub data: String,
    #[doc = "The URL to the hosted PromptPay instructions page, which allows customers to view the PromptPay QR code."]
    #[json(default)]
    pub hosted_instructions_url: String,
    #[doc = "The PNG path used to render the QR code, can be used as the source in an HTML img tag"]
    #[json(default)]
    pub image_url_png: String,
    #[doc = "The SVG path used to render the QR code, can be used as the source in an HTML img tag"]
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
pub struct PaymentIntentNextActionRedirectToUrl {
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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentIntentNextActionWechatPayDisplayQrCode {
    #[doc = "The data being used to generate QR code"]
    #[json(default)]
    pub data: String,
    #[doc = "The URL to the hosted WeChat Pay instructions page, which allows customers to view the WeChat Pay QR code."]
    #[json(default)]
    pub hosted_instructions_url: String,
    #[doc = "The base64 image data for a pre-generated QR code"]
    #[json(default)]
    pub image_data_url: String,
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
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization {
    IfAvailable,
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
impl PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization::IfAvailable => "if_available",
            PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization::Never => "never",
            PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "if_available" => PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization::IfAvailable,
            "never" => PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization::Never,
            other => PaymentIntentPaymentMethodOptionsCardRequestExtendedAuthorization::Unknown(other.to_string()),
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
pub enum PaymentIntentPaymentMethodOptionsMandateOptionsPaytoAmountType {
    Fixed,
    Maximum,
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
impl PaymentIntentPaymentMethodOptionsMandateOptionsPaytoAmountType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentIntentPaymentMethodOptionsMandateOptionsPaytoAmountType::Fixed => "fixed",
            PaymentIntentPaymentMethodOptionsMandateOptionsPaytoAmountType::Maximum => "maximum",
            PaymentIntentPaymentMethodOptionsMandateOptionsPaytoAmountType::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PaymentIntentPaymentMethodOptionsMandateOptionsPaytoAmountType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentIntentPaymentMethodOptionsMandateOptionsPaytoAmountType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "fixed" => PaymentIntentPaymentMethodOptionsMandateOptionsPaytoAmountType::Fixed,
            "maximum" => PaymentIntentPaymentMethodOptionsMandateOptionsPaytoAmountType::Maximum,
            other => PaymentIntentPaymentMethodOptionsMandateOptionsPaytoAmountType::Unknown(other.to_string()),
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
pub enum PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule {
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
impl PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Adhoc => "adhoc",
            PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Annual => "annual",
            PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Daily => "daily",
            PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Fortnightly => "fortnightly",
            PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Monthly => "monthly",
            PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Quarterly => "quarterly",
            PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::SemiAnnual => "semi_annual",
            PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Weekly => "weekly",
            PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "adhoc" => PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Adhoc,
            "annual" => PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Annual,
            "daily" => PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Daily,
            "fortnightly" => PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Fortnightly,
            "monthly" => PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Monthly,
            "quarterly" => PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Quarterly,
            "semi_annual" => PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::SemiAnnual,
            "weekly" => PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Weekly,
            other => PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule::Unknown(other.to_string()),
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
pub struct PaymentIntentPaymentMethodOptionsMandateOptionsSepaDebit {
    #[doc = "Prefix used to generate the Mandate reference."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reference_prefix: Option<String>,
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
pub enum PaymentIntentPaymentMethodOptionsUsBankAccountTransactionPurpose {
    Goods,
    Other,
    Services,
    Unspecified,
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
impl PaymentIntentPaymentMethodOptionsUsBankAccountTransactionPurpose {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentIntentPaymentMethodOptionsUsBankAccountTransactionPurpose::Goods => "goods",
            PaymentIntentPaymentMethodOptionsUsBankAccountTransactionPurpose::Other => "other",
            PaymentIntentPaymentMethodOptionsUsBankAccountTransactionPurpose::Services => "services",
            PaymentIntentPaymentMethodOptionsUsBankAccountTransactionPurpose::Unspecified => "unspecified",
            PaymentIntentPaymentMethodOptionsUsBankAccountTransactionPurpose::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PaymentIntentPaymentMethodOptionsUsBankAccountTransactionPurpose {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentIntentPaymentMethodOptionsUsBankAccountTransactionPurpose::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "goods" => PaymentIntentPaymentMethodOptionsUsBankAccountTransactionPurpose::Goods,
            "other" => PaymentIntentPaymentMethodOptionsUsBankAccountTransactionPurpose::Other,
            "services" => PaymentIntentPaymentMethodOptionsUsBankAccountTransactionPurpose::Services,
            "unspecified" => PaymentIntentPaymentMethodOptionsUsBankAccountTransactionPurpose::Unspecified,
            other => PaymentIntentPaymentMethodOptionsUsBankAccountTransactionPurpose::Unknown(other.to_string()),
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
pub struct PaymentIntentProcessingCustomerNotification {
    #[doc = "Whether customer approval has been requested for this payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub approval_requested: Option<bool>,
    #[doc = "If customer approval is required, they need to provide approval before this time."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub completes_at: Option<i64>,
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
pub enum PaymentIntentSetupFutureUsage {
    OffSession,
    OnSession,
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
impl PaymentIntentSetupFutureUsage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentIntentSetupFutureUsage::OffSession => "off_session",
            PaymentIntentSetupFutureUsage::OnSession => "on_session",
            PaymentIntentSetupFutureUsage::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PaymentIntentSetupFutureUsage {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentIntentSetupFutureUsage::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "off_session" => PaymentIntentSetupFutureUsage::OffSession,
            "on_session" => PaymentIntentSetupFutureUsage::OnSession,
            other => PaymentIntentSetupFutureUsage::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct SearchPaymentIntentsParams {
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub page: Option<String>,
    pub query: String,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for SearchPaymentIntentsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("page", self.page.as_ref());
        form.field("query", &self.query);
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum UpdatePaymentIntentParamsSetupFutureUsage {
    Unnamed,
    OffSession,
    OnSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_intents")]
impl UpdatePaymentIntentParamsSetupFutureUsage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            UpdatePaymentIntentParamsSetupFutureUsage::Unnamed => "",
            UpdatePaymentIntentParamsSetupFutureUsage::OffSession => "off_session",
            UpdatePaymentIntentParamsSetupFutureUsage::OnSession => "on_session",
            UpdatePaymentIntentParamsSetupFutureUsage::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_intents")]
impl<'json> json::Decode<'json> for UpdatePaymentIntentParamsSetupFutureUsage {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(UpdatePaymentIntentParamsSetupFutureUsage::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => UpdatePaymentIntentParamsSetupFutureUsage::Unnamed,
            "off_session" => UpdatePaymentIntentParamsSetupFutureUsage::OffSession,
            "on_session" => UpdatePaymentIntentParamsSetupFutureUsage::OnSession,
            other => UpdatePaymentIntentParamsSetupFutureUsage::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for UpdatePaymentIntentParamsSetupFutureUsage {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}
