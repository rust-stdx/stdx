#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CancelSetupIntentParamsCancellationReason {
    Abandoned,
    Duplicate,
    RequestedByCustomer,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "setup_intents")]
impl CancelSetupIntentParamsCancellationReason {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CancelSetupIntentParamsCancellationReason::Abandoned => "abandoned",
            CancelSetupIntentParamsCancellationReason::Duplicate => "duplicate",
            CancelSetupIntentParamsCancellationReason::RequestedByCustomer => "requested_by_customer",
            CancelSetupIntentParamsCancellationReason::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CancelSetupIntentParamsCancellationReason {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CancelSetupIntentParamsCancellationReason::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "abandoned" => CancelSetupIntentParamsCancellationReason::Abandoned,
            "duplicate" => CancelSetupIntentParamsCancellationReason::Duplicate,
            "requested_by_customer" => CancelSetupIntentParamsCancellationReason::RequestedByCustomer,
            other => CancelSetupIntentParamsCancellationReason::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CancelSetupIntentParamsCancellationReason {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsAllowedPaymentMethodTypesItem {
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

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsAllowedPaymentMethodTypesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::AcssDebit => "acss_debit",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Affirm => "affirm",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::AfterpayClearpay => "afterpay_clearpay",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Alipay => "alipay",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Alma => "alma",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::AmazonPay => "amazon_pay",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::AuBecsDebit => "au_becs_debit",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::BacsDebit => "bacs_debit",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Bancontact => "bancontact",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Billie => "billie",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Bizum => "bizum",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Blik => "blik",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::BokuPromptpay => "boku_promptpay",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Boleto => "boleto",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::CapchasePay => "capchase_pay",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Card => "card",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::CardPresent => "card_present",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Cashapp => "cashapp",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::CheckScan => "check_scan",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::ClickToPay => "click_to_pay",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Crypto => "crypto",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::CustomerBalance => "customer_balance",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::DemoPay => "demo_pay",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Duitnow => "duitnow",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::DummyAuthPush => "dummy_auth_push",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::DummyPassthroughCard => "dummy_passthrough_card",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Edenred => "edenred",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Eps => "eps",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Fpx => "fpx",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Gcash => "gcash",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Getbalance => "getbalance",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::GiftCard => "gift_card",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Giropay => "giropay",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Gopay => "gopay",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Grabpay => "grabpay",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::IdBankTransfer => "id_bank_transfer",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Ideal => "ideal",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::InteracPresent => "interac_present",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::KakaoPay => "kakao_pay",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Klarna => "klarna",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Knet => "knet",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Konbini => "konbini",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::KrCard => "kr_card",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::KrMarket => "kr_market",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Kriya => "kriya",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Link => "link",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::MbWay => "mb_way",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Mobilepay => "mobilepay",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Momo => "momo",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Mondu => "mondu",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Multibanco => "multibanco",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::NaverPay => "naver_pay",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Netbanking => "netbanking",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::NgBank => "ng_bank",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::NgBankTransfer => "ng_bank_transfer",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::NgCard => "ng_card",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::NgMarket => "ng_market",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::NgUssd => "ng_ussd",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::NgWallet => "ng_wallet",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::NzBankAccount => "nz_bank_account",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Octopus => "octopus",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Oxxo => "oxxo",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::P24 => "p24",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::PaperCheck => "paper_check",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::PayByBank => "pay_by_bank",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Payco => "payco",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Paynow => "paynow",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Paypal => "paypal",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Paypay => "paypay",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Payto => "payto",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Pix => "pix",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Promptpay => "promptpay",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Qris => "qris",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Rechnung => "rechnung",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::RevolutPay => "revolut_pay",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::SamsungPay => "samsung_pay",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Satispay => "satispay",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Scalapay => "scalapay",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::SepaDebit => "sepa_debit",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Sequra => "sequra",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::ShopPay => "shop_pay",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Shopeepay => "shopeepay",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Sofort => "sofort",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::SouthKoreaMarket => "south_korea_market",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::StripeBalance => "stripe_balance",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Sunbit => "sunbit",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Swish => "swish",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Tamara => "tamara",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::TestPay => "test_pay",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::TouchNGo => "touch_n_go",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Truemoney => "truemoney",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Twint => "twint",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Upi => "upi",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::UsBankAccount => "us_bank_account",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::UsCashVoucher => "us_cash_voucher",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Vipps => "vipps",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::WechatPay => "wechat_pay",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Wero => "wero",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Zip => "zip",
            CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsAllowedPaymentMethodTypesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "acss_debit" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::AcssDebit,
            "affirm" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Affirm,
            "afterpay_clearpay" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::AfterpayClearpay,
            "alipay" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Alipay,
            "alma" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Alma,
            "amazon_pay" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::AmazonPay,
            "au_becs_debit" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::AuBecsDebit,
            "bacs_debit" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::BacsDebit,
            "bancontact" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Bancontact,
            "billie" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Billie,
            "bizum" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Bizum,
            "blik" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Blik,
            "boku_promptpay" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::BokuPromptpay,
            "boleto" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Boleto,
            "capchase_pay" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::CapchasePay,
            "card" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Card,
            "card_present" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::CardPresent,
            "cashapp" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Cashapp,
            "check_scan" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::CheckScan,
            "click_to_pay" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::ClickToPay,
            "crypto" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Crypto,
            "customer_balance" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::CustomerBalance,
            "demo_pay" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::DemoPay,
            "duitnow" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Duitnow,
            "dummy_auth_push" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::DummyAuthPush,
            "dummy_passthrough_card" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::DummyPassthroughCard,
            "edenred" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Edenred,
            "eps" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Eps,
            "fpx" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Fpx,
            "gcash" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Gcash,
            "getbalance" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Getbalance,
            "gift_card" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::GiftCard,
            "giropay" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Giropay,
            "gopay" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Gopay,
            "grabpay" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Grabpay,
            "id_bank_transfer" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::IdBankTransfer,
            "ideal" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Ideal,
            "interac_present" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::InteracPresent,
            "kakao_pay" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::KakaoPay,
            "klarna" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Klarna,
            "knet" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Knet,
            "konbini" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Konbini,
            "kr_card" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::KrCard,
            "kr_market" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::KrMarket,
            "kriya" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Kriya,
            "link" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Link,
            "mb_way" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::MbWay,
            "mobilepay" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Mobilepay,
            "momo" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Momo,
            "mondu" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Mondu,
            "multibanco" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Multibanco,
            "naver_pay" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::NaverPay,
            "netbanking" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Netbanking,
            "ng_bank" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::NgBank,
            "ng_bank_transfer" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::NgBankTransfer,
            "ng_card" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::NgCard,
            "ng_market" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::NgMarket,
            "ng_ussd" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::NgUssd,
            "ng_wallet" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::NgWallet,
            "nz_bank_account" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::NzBankAccount,
            "octopus" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Octopus,
            "oxxo" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Oxxo,
            "p24" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::P24,
            "paper_check" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::PaperCheck,
            "pay_by_bank" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::PayByBank,
            "payco" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Payco,
            "paynow" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Paynow,
            "paypal" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Paypal,
            "paypay" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Paypay,
            "payto" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Payto,
            "pix" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Pix,
            "promptpay" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Promptpay,
            "qris" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Qris,
            "rechnung" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Rechnung,
            "revolut_pay" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::RevolutPay,
            "samsung_pay" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::SamsungPay,
            "satispay" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Satispay,
            "scalapay" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Scalapay,
            "sepa_debit" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::SepaDebit,
            "sequra" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Sequra,
            "shop_pay" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::ShopPay,
            "shopeepay" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Shopeepay,
            "sofort" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Sofort,
            "south_korea_market" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::SouthKoreaMarket,
            "stripe_balance" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::StripeBalance,
            "sunbit" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Sunbit,
            "swish" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Swish,
            "tamara" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Tamara,
            "test_pay" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::TestPay,
            "touch_n_go" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::TouchNGo,
            "truemoney" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Truemoney,
            "twint" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Twint,
            "upi" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Upi,
            "us_bank_account" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::UsBankAccount,
            "us_cash_voucher" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::UsCashVoucher,
            "vipps" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Vipps,
            "wechat_pay" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::WechatPay,
            "wero" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Wero,
            "zip" => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Zip,
            other => CreateSetupIntentParamsAllowedPaymentMethodTypesItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsAllowedPaymentMethodTypesItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsAutomaticPaymentMethodsAllowRedirects {
    Always,
    Never,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsAutomaticPaymentMethodsAllowRedirects {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsAutomaticPaymentMethodsAllowRedirects::Always => "always",
            CreateSetupIntentParamsAutomaticPaymentMethodsAllowRedirects::Never => "never",
            CreateSetupIntentParamsAutomaticPaymentMethodsAllowRedirects::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsAutomaticPaymentMethodsAllowRedirects {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSetupIntentParamsAutomaticPaymentMethodsAllowRedirects::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "always" => CreateSetupIntentParamsAutomaticPaymentMethodsAllowRedirects::Always,
            "never" => CreateSetupIntentParamsAutomaticPaymentMethodsAllowRedirects::Never,
            other => CreateSetupIntentParamsAutomaticPaymentMethodsAllowRedirects::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsAutomaticPaymentMethodsAllowRedirects {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodDataAuBecsDebit {
    #[doc = "The account number for the bank account."]
    pub account_number: String,
    #[doc = "Bank-State-Branch number of the bank account."]
    pub bsb_number: String,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodDataAuBecsDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("account_number", &self.account_number);
        form.field("bsb_number", &self.bsb_number);
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodDataFpxBank {
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

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodDataFpxBank {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodDataFpxBank::AffinBank => "affin_bank",
            CreateSetupIntentParamsPaymentMethodDataFpxBank::Agrobank => "agrobank",
            CreateSetupIntentParamsPaymentMethodDataFpxBank::AllianceBank => "alliance_bank",
            CreateSetupIntentParamsPaymentMethodDataFpxBank::Ambank => "ambank",
            CreateSetupIntentParamsPaymentMethodDataFpxBank::BankIslam => "bank_islam",
            CreateSetupIntentParamsPaymentMethodDataFpxBank::BankMuamalat => "bank_muamalat",
            CreateSetupIntentParamsPaymentMethodDataFpxBank::BankOfChina => "bank_of_china",
            CreateSetupIntentParamsPaymentMethodDataFpxBank::BankRakyat => "bank_rakyat",
            CreateSetupIntentParamsPaymentMethodDataFpxBank::BnpParibas => "bnp_paribas",
            CreateSetupIntentParamsPaymentMethodDataFpxBank::Bsn => "bsn",
            CreateSetupIntentParamsPaymentMethodDataFpxBank::Cimb => "cimb",
            CreateSetupIntentParamsPaymentMethodDataFpxBank::Citibank => "citibank",
            CreateSetupIntentParamsPaymentMethodDataFpxBank::DeutscheBank => "deutsche_bank",
            CreateSetupIntentParamsPaymentMethodDataFpxBank::HongLeongBank => "hong_leong_bank",
            CreateSetupIntentParamsPaymentMethodDataFpxBank::Hsbc => "hsbc",
            CreateSetupIntentParamsPaymentMethodDataFpxBank::Kfh => "kfh",
            CreateSetupIntentParamsPaymentMethodDataFpxBank::Maybank2e => "maybank2e",
            CreateSetupIntentParamsPaymentMethodDataFpxBank::Maybank2u => "maybank2u",
            CreateSetupIntentParamsPaymentMethodDataFpxBank::MbsbBank => "mbsb_bank",
            CreateSetupIntentParamsPaymentMethodDataFpxBank::Ocbc => "ocbc",
            CreateSetupIntentParamsPaymentMethodDataFpxBank::PbEnterprise => "pb_enterprise",
            CreateSetupIntentParamsPaymentMethodDataFpxBank::PublicBank => "public_bank",
            CreateSetupIntentParamsPaymentMethodDataFpxBank::Rhb => "rhb",
            CreateSetupIntentParamsPaymentMethodDataFpxBank::StandardChartered => "standard_chartered",
            CreateSetupIntentParamsPaymentMethodDataFpxBank::Uob => "uob",
            CreateSetupIntentParamsPaymentMethodDataFpxBank::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsPaymentMethodDataFpxBank {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSetupIntentParamsPaymentMethodDataFpxBank::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "affin_bank" => CreateSetupIntentParamsPaymentMethodDataFpxBank::AffinBank,
            "agrobank" => CreateSetupIntentParamsPaymentMethodDataFpxBank::Agrobank,
            "alliance_bank" => CreateSetupIntentParamsPaymentMethodDataFpxBank::AllianceBank,
            "ambank" => CreateSetupIntentParamsPaymentMethodDataFpxBank::Ambank,
            "bank_islam" => CreateSetupIntentParamsPaymentMethodDataFpxBank::BankIslam,
            "bank_muamalat" => CreateSetupIntentParamsPaymentMethodDataFpxBank::BankMuamalat,
            "bank_of_china" => CreateSetupIntentParamsPaymentMethodDataFpxBank::BankOfChina,
            "bank_rakyat" => CreateSetupIntentParamsPaymentMethodDataFpxBank::BankRakyat,
            "bnp_paribas" => CreateSetupIntentParamsPaymentMethodDataFpxBank::BnpParibas,
            "bsn" => CreateSetupIntentParamsPaymentMethodDataFpxBank::Bsn,
            "cimb" => CreateSetupIntentParamsPaymentMethodDataFpxBank::Cimb,
            "citibank" => CreateSetupIntentParamsPaymentMethodDataFpxBank::Citibank,
            "deutsche_bank" => CreateSetupIntentParamsPaymentMethodDataFpxBank::DeutscheBank,
            "hong_leong_bank" => CreateSetupIntentParamsPaymentMethodDataFpxBank::HongLeongBank,
            "hsbc" => CreateSetupIntentParamsPaymentMethodDataFpxBank::Hsbc,
            "kfh" => CreateSetupIntentParamsPaymentMethodDataFpxBank::Kfh,
            "maybank2e" => CreateSetupIntentParamsPaymentMethodDataFpxBank::Maybank2e,
            "maybank2u" => CreateSetupIntentParamsPaymentMethodDataFpxBank::Maybank2u,
            "mbsb_bank" => CreateSetupIntentParamsPaymentMethodDataFpxBank::MbsbBank,
            "ocbc" => CreateSetupIntentParamsPaymentMethodDataFpxBank::Ocbc,
            "pb_enterprise" => CreateSetupIntentParamsPaymentMethodDataFpxBank::PbEnterprise,
            "public_bank" => CreateSetupIntentParamsPaymentMethodDataFpxBank::PublicBank,
            "rhb" => CreateSetupIntentParamsPaymentMethodDataFpxBank::Rhb,
            "standard_chartered" => CreateSetupIntentParamsPaymentMethodDataFpxBank::StandardChartered,
            "uob" => CreateSetupIntentParamsPaymentMethodDataFpxBank::Uob,
            other => CreateSetupIntentParamsPaymentMethodDataFpxBank::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodDataFpxBank {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodDataPayto {
    #[doc = "The account number for the bank account."]
    pub account_number: Option<String>,
    #[doc = "Bank-State-Branch number of the bank account."]
    pub bsb_number: Option<String>,
    #[doc = "The PayID alias for the bank account."]
    pub pay_id: Option<String>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodDataPayto {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account_number", self.account_number.as_ref());
        form.field_opt("bsb_number", self.bsb_number.as_ref());
        form.field_opt("pay_id", self.pay_id.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodDataRadarOptions {
    #[doc = "A [Radar Session](https://docs.stripe.com/radar/radar-session) is a snapshot of the browser metadata and device detai..."]
    pub session: Option<String>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodDataRadarOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("session", self.session.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodDataSepaDebit {
    #[doc = "IBAN of the bank account."]
    pub iban: String,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodDataSepaDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("iban", &self.iban);
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodDataType {
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

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodDataType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodDataType::AcssDebit => "acss_debit",
            CreateSetupIntentParamsPaymentMethodDataType::Affirm => "affirm",
            CreateSetupIntentParamsPaymentMethodDataType::AfterpayClearpay => "afterpay_clearpay",
            CreateSetupIntentParamsPaymentMethodDataType::Alipay => "alipay",
            CreateSetupIntentParamsPaymentMethodDataType::Alma => "alma",
            CreateSetupIntentParamsPaymentMethodDataType::AmazonPay => "amazon_pay",
            CreateSetupIntentParamsPaymentMethodDataType::AuBecsDebit => "au_becs_debit",
            CreateSetupIntentParamsPaymentMethodDataType::BacsDebit => "bacs_debit",
            CreateSetupIntentParamsPaymentMethodDataType::Bancontact => "bancontact",
            CreateSetupIntentParamsPaymentMethodDataType::Billie => "billie",
            CreateSetupIntentParamsPaymentMethodDataType::Bizum => "bizum",
            CreateSetupIntentParamsPaymentMethodDataType::Blik => "blik",
            CreateSetupIntentParamsPaymentMethodDataType::Boleto => "boleto",
            CreateSetupIntentParamsPaymentMethodDataType::Cashapp => "cashapp",
            CreateSetupIntentParamsPaymentMethodDataType::Crypto => "crypto",
            CreateSetupIntentParamsPaymentMethodDataType::CustomerBalance => "customer_balance",
            CreateSetupIntentParamsPaymentMethodDataType::Eps => "eps",
            CreateSetupIntentParamsPaymentMethodDataType::Fpx => "fpx",
            CreateSetupIntentParamsPaymentMethodDataType::Giropay => "giropay",
            CreateSetupIntentParamsPaymentMethodDataType::Grabpay => "grabpay",
            CreateSetupIntentParamsPaymentMethodDataType::Ideal => "ideal",
            CreateSetupIntentParamsPaymentMethodDataType::KakaoPay => "kakao_pay",
            CreateSetupIntentParamsPaymentMethodDataType::Klarna => "klarna",
            CreateSetupIntentParamsPaymentMethodDataType::Konbini => "konbini",
            CreateSetupIntentParamsPaymentMethodDataType::KrCard => "kr_card",
            CreateSetupIntentParamsPaymentMethodDataType::Link => "link",
            CreateSetupIntentParamsPaymentMethodDataType::MbWay => "mb_way",
            CreateSetupIntentParamsPaymentMethodDataType::Mobilepay => "mobilepay",
            CreateSetupIntentParamsPaymentMethodDataType::Multibanco => "multibanco",
            CreateSetupIntentParamsPaymentMethodDataType::NaverPay => "naver_pay",
            CreateSetupIntentParamsPaymentMethodDataType::NzBankAccount => "nz_bank_account",
            CreateSetupIntentParamsPaymentMethodDataType::Oxxo => "oxxo",
            CreateSetupIntentParamsPaymentMethodDataType::P24 => "p24",
            CreateSetupIntentParamsPaymentMethodDataType::PayByBank => "pay_by_bank",
            CreateSetupIntentParamsPaymentMethodDataType::Payco => "payco",
            CreateSetupIntentParamsPaymentMethodDataType::Paynow => "paynow",
            CreateSetupIntentParamsPaymentMethodDataType::Paypal => "paypal",
            CreateSetupIntentParamsPaymentMethodDataType::Paypay => "paypay",
            CreateSetupIntentParamsPaymentMethodDataType::Payto => "payto",
            CreateSetupIntentParamsPaymentMethodDataType::Pix => "pix",
            CreateSetupIntentParamsPaymentMethodDataType::Promptpay => "promptpay",
            CreateSetupIntentParamsPaymentMethodDataType::RevolutPay => "revolut_pay",
            CreateSetupIntentParamsPaymentMethodDataType::SamsungPay => "samsung_pay",
            CreateSetupIntentParamsPaymentMethodDataType::Satispay => "satispay",
            CreateSetupIntentParamsPaymentMethodDataType::Scalapay => "scalapay",
            CreateSetupIntentParamsPaymentMethodDataType::SepaDebit => "sepa_debit",
            CreateSetupIntentParamsPaymentMethodDataType::Sequra => "sequra",
            CreateSetupIntentParamsPaymentMethodDataType::Sofort => "sofort",
            CreateSetupIntentParamsPaymentMethodDataType::Sunbit => "sunbit",
            CreateSetupIntentParamsPaymentMethodDataType::Swish => "swish",
            CreateSetupIntentParamsPaymentMethodDataType::Twint => "twint",
            CreateSetupIntentParamsPaymentMethodDataType::Upi => "upi",
            CreateSetupIntentParamsPaymentMethodDataType::UsBankAccount => "us_bank_account",
            CreateSetupIntentParamsPaymentMethodDataType::WechatPay => "wechat_pay",
            CreateSetupIntentParamsPaymentMethodDataType::Zip => "zip",
            CreateSetupIntentParamsPaymentMethodDataType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsPaymentMethodDataType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSetupIntentParamsPaymentMethodDataType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "acss_debit" => CreateSetupIntentParamsPaymentMethodDataType::AcssDebit,
            "affirm" => CreateSetupIntentParamsPaymentMethodDataType::Affirm,
            "afterpay_clearpay" => CreateSetupIntentParamsPaymentMethodDataType::AfterpayClearpay,
            "alipay" => CreateSetupIntentParamsPaymentMethodDataType::Alipay,
            "alma" => CreateSetupIntentParamsPaymentMethodDataType::Alma,
            "amazon_pay" => CreateSetupIntentParamsPaymentMethodDataType::AmazonPay,
            "au_becs_debit" => CreateSetupIntentParamsPaymentMethodDataType::AuBecsDebit,
            "bacs_debit" => CreateSetupIntentParamsPaymentMethodDataType::BacsDebit,
            "bancontact" => CreateSetupIntentParamsPaymentMethodDataType::Bancontact,
            "billie" => CreateSetupIntentParamsPaymentMethodDataType::Billie,
            "bizum" => CreateSetupIntentParamsPaymentMethodDataType::Bizum,
            "blik" => CreateSetupIntentParamsPaymentMethodDataType::Blik,
            "boleto" => CreateSetupIntentParamsPaymentMethodDataType::Boleto,
            "cashapp" => CreateSetupIntentParamsPaymentMethodDataType::Cashapp,
            "crypto" => CreateSetupIntentParamsPaymentMethodDataType::Crypto,
            "customer_balance" => CreateSetupIntentParamsPaymentMethodDataType::CustomerBalance,
            "eps" => CreateSetupIntentParamsPaymentMethodDataType::Eps,
            "fpx" => CreateSetupIntentParamsPaymentMethodDataType::Fpx,
            "giropay" => CreateSetupIntentParamsPaymentMethodDataType::Giropay,
            "grabpay" => CreateSetupIntentParamsPaymentMethodDataType::Grabpay,
            "ideal" => CreateSetupIntentParamsPaymentMethodDataType::Ideal,
            "kakao_pay" => CreateSetupIntentParamsPaymentMethodDataType::KakaoPay,
            "klarna" => CreateSetupIntentParamsPaymentMethodDataType::Klarna,
            "konbini" => CreateSetupIntentParamsPaymentMethodDataType::Konbini,
            "kr_card" => CreateSetupIntentParamsPaymentMethodDataType::KrCard,
            "link" => CreateSetupIntentParamsPaymentMethodDataType::Link,
            "mb_way" => CreateSetupIntentParamsPaymentMethodDataType::MbWay,
            "mobilepay" => CreateSetupIntentParamsPaymentMethodDataType::Mobilepay,
            "multibanco" => CreateSetupIntentParamsPaymentMethodDataType::Multibanco,
            "naver_pay" => CreateSetupIntentParamsPaymentMethodDataType::NaverPay,
            "nz_bank_account" => CreateSetupIntentParamsPaymentMethodDataType::NzBankAccount,
            "oxxo" => CreateSetupIntentParamsPaymentMethodDataType::Oxxo,
            "p24" => CreateSetupIntentParamsPaymentMethodDataType::P24,
            "pay_by_bank" => CreateSetupIntentParamsPaymentMethodDataType::PayByBank,
            "payco" => CreateSetupIntentParamsPaymentMethodDataType::Payco,
            "paynow" => CreateSetupIntentParamsPaymentMethodDataType::Paynow,
            "paypal" => CreateSetupIntentParamsPaymentMethodDataType::Paypal,
            "paypay" => CreateSetupIntentParamsPaymentMethodDataType::Paypay,
            "payto" => CreateSetupIntentParamsPaymentMethodDataType::Payto,
            "pix" => CreateSetupIntentParamsPaymentMethodDataType::Pix,
            "promptpay" => CreateSetupIntentParamsPaymentMethodDataType::Promptpay,
            "revolut_pay" => CreateSetupIntentParamsPaymentMethodDataType::RevolutPay,
            "samsung_pay" => CreateSetupIntentParamsPaymentMethodDataType::SamsungPay,
            "satispay" => CreateSetupIntentParamsPaymentMethodDataType::Satispay,
            "scalapay" => CreateSetupIntentParamsPaymentMethodDataType::Scalapay,
            "sepa_debit" => CreateSetupIntentParamsPaymentMethodDataType::SepaDebit,
            "sequra" => CreateSetupIntentParamsPaymentMethodDataType::Sequra,
            "sofort" => CreateSetupIntentParamsPaymentMethodDataType::Sofort,
            "sunbit" => CreateSetupIntentParamsPaymentMethodDataType::Sunbit,
            "swish" => CreateSetupIntentParamsPaymentMethodDataType::Swish,
            "twint" => CreateSetupIntentParamsPaymentMethodDataType::Twint,
            "upi" => CreateSetupIntentParamsPaymentMethodDataType::Upi,
            "us_bank_account" => CreateSetupIntentParamsPaymentMethodDataType::UsBankAccount,
            "wechat_pay" => CreateSetupIntentParamsPaymentMethodDataType::WechatPay,
            "zip" => CreateSetupIntentParamsPaymentMethodDataType::Zip,
            other => CreateSetupIntentParamsPaymentMethodDataType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodDataType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodDataUsBankAccountAccountType {
    Checking,
    Savings,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodDataUsBankAccountAccountType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodDataUsBankAccountAccountType::Checking => "checking",
            CreateSetupIntentParamsPaymentMethodDataUsBankAccountAccountType::Savings => "savings",
            CreateSetupIntentParamsPaymentMethodDataUsBankAccountAccountType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsPaymentMethodDataUsBankAccountAccountType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSetupIntentParamsPaymentMethodDataUsBankAccountAccountType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "checking" => CreateSetupIntentParamsPaymentMethodDataUsBankAccountAccountType::Checking,
            "savings" => CreateSetupIntentParamsPaymentMethodDataUsBankAccountAccountType::Savings,
            other => CreateSetupIntentParamsPaymentMethodDataUsBankAccountAccountType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodDataUsBankAccountAccountType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodOptionsBacsDebitMandateOptions {
    #[doc = "Prefix used to generate the Mandate reference."]
    pub reference_prefix: Option<json::OwnedValue>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsBacsDebitMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        let _ = form;
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus {
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

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::A => "A",
            CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::C => "C",
            CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::I => "I",
            CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::N => "N",
            CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::R => "R",
            CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::U => "U",
            CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::Y => "Y",
            CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "A" => CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::A,
            "C" => CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::C,
            "I" => CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::I,
            "N" => CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::N,
            "R" => CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::R,
            "U" => CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::U,
            "Y" => CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::Y,
            other => {
                CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule {
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

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Unnamed => "",
            CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Adhoc => "adhoc",
            CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Annual => "annual",
            CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Daily => "daily",
            CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Fortnightly => "fortnightly",
            CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Monthly => "monthly",
            CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Quarterly => "quarterly",
            CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::SemiAnnual => "semi_annual",
            CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Weekly => "weekly",
            CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Unnamed,
            "adhoc" => CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Adhoc,
            "annual" => CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Annual,
            "daily" => CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Daily,
            "fortnightly" => CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Fortnightly,
            "monthly" => CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Monthly,
            "quarterly" => CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Quarterly,
            "semi_annual" => CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::SemiAnnual,
            "weekly" => CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Weekly,
            other => CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule {
    Halfyearly,
    Monthly,
    Quarterly,
    Weekly,
    Yearly,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Halfyearly => "halfyearly",
            CreateSetupIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Monthly => "monthly",
            CreateSetupIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Quarterly => "quarterly",
            CreateSetupIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Weekly => "weekly",
            CreateSetupIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Yearly => "yearly",
            CreateSetupIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateSetupIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "halfyearly" => CreateSetupIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Halfyearly,
            "monthly" => CreateSetupIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Monthly,
            "quarterly" => CreateSetupIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Quarterly,
            "weekly" => CreateSetupIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Weekly,
            "yearly" => CreateSetupIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Yearly,
            other => {
                CreateSetupIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodOptionsUpiSetupFutureUsage {
    Unnamed,
    None,
    OffSession,
    OnSession,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodOptionsUpiSetupFutureUsage {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodOptionsUpiSetupFutureUsage::Unnamed => "",
            CreateSetupIntentParamsPaymentMethodOptionsUpiSetupFutureUsage::None => "none",
            CreateSetupIntentParamsPaymentMethodOptionsUpiSetupFutureUsage::OffSession => "off_session",
            CreateSetupIntentParamsPaymentMethodOptionsUpiSetupFutureUsage::OnSession => "on_session",
            CreateSetupIntentParamsPaymentMethodOptionsUpiSetupFutureUsage::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json> for CreateSetupIntentParamsPaymentMethodOptionsUpiSetupFutureUsage {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateSetupIntentParamsPaymentMethodOptionsUpiSetupFutureUsage::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "" => CreateSetupIntentParamsPaymentMethodOptionsUpiSetupFutureUsage::Unnamed,
            "none" => CreateSetupIntentParamsPaymentMethodOptionsUpiSetupFutureUsage::None,
            "off_session" => CreateSetupIntentParamsPaymentMethodOptionsUpiSetupFutureUsage::OffSession,
            "on_session" => CreateSetupIntentParamsPaymentMethodOptionsUpiSetupFutureUsage::OnSession,
            other => CreateSetupIntentParamsPaymentMethodOptionsUpiSetupFutureUsage::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsUpiSetupFutureUsage {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "setup_intents")]
impl CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Balances => "balances",
            CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Ownership => "ownership",
            CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::PaymentMethod => "payment_method",
            CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Transactions => "transactions",
            CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "setup_intents")]
impl<'json> json::Decode<'json>
    for CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "balances" => CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Balances,
            "ownership" => CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Ownership,
            "payment_method" => CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::PaymentMethod,
            "transactions" => CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Transactions,
            other => CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode
    for CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem
{
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
pub struct SetupIntentNextActionRedirectToUrl {
    #[doc = "If the customer does not exit their browser while authenticating, they will be redirected to this specified URL after..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[doc = "The URL you must redirect your customer to in order to authenticate."]
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
pub enum SetupIntentObject {
    SetupIntent,
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
impl SetupIntentObject {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SetupIntentObject::SetupIntent => "setup_intent",
            SetupIntentObject::Unknown(value) => value,
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
impl Default for SetupIntentObject {
    fn default() -> Self {
        SetupIntentObject::Unknown(String::new())
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
impl<'json> json::Decode<'json> for SetupIntentObject {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SetupIntentObject::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "setup_intent" => SetupIntentObject::SetupIntent,
            other => SetupIntentObject::Unknown(other.to_string()),
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
pub struct SetupIntentPaymentMethodOptionsBizum {}

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
pub enum SetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
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
impl SetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SetupIntentPaymentMethodOptionsCardMandateOptionsAmountType::Fixed => "fixed",
            SetupIntentPaymentMethodOptionsCardMandateOptionsAmountType::Maximum => "maximum",
            SetupIntentPaymentMethodOptionsCardMandateOptionsAmountType::Unknown(value) => value,
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
impl Default for SetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    fn default() -> Self {
        SetupIntentPaymentMethodOptionsCardMandateOptionsAmountType::Unknown(String::new())
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
impl<'json> json::Decode<'json> for SetupIntentPaymentMethodOptionsCardMandateOptionsAmountType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SetupIntentPaymentMethodOptionsCardMandateOptionsAmountType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "fixed" => SetupIntentPaymentMethodOptionsCardMandateOptionsAmountType::Fixed,
            "maximum" => SetupIntentPaymentMethodOptionsCardMandateOptionsAmountType::Maximum,
            other => SetupIntentPaymentMethodOptionsCardMandateOptionsAmountType::Unknown(other.to_string()),
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
pub enum SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
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
impl SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType::Business => "business",
            SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType::Personal => "personal",
            SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "business" => SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType::Business,
            "personal" => SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType::Personal,
            other => SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType::Unknown(other.to_string()),
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
pub struct SetupIntentPaymentMethodOptionsMandateOptionsBacsDebit {
    #[doc = "Prefix used to generate the Mandate reference."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reference_prefix: Option<String>,
}
