#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentLinkParamsPaymentMethodOptionsCardRestrictionsBrandsBlockedItem {
    AmericanExpress,
    DiscoverGlobalNetwork,
    Mastercard,
    Visa,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_links")]
impl CreatePaymentLinkParamsPaymentMethodOptionsCardRestrictionsBrandsBlockedItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentLinkParamsPaymentMethodOptionsCardRestrictionsBrandsBlockedItem::AmericanExpress => {
                "american_express"
            }
            CreatePaymentLinkParamsPaymentMethodOptionsCardRestrictionsBrandsBlockedItem::DiscoverGlobalNetwork => {
                "discover_global_network"
            }
            CreatePaymentLinkParamsPaymentMethodOptionsCardRestrictionsBrandsBlockedItem::Mastercard => "mastercard",
            CreatePaymentLinkParamsPaymentMethodOptionsCardRestrictionsBrandsBlockedItem::Visa => "visa",
            CreatePaymentLinkParamsPaymentMethodOptionsCardRestrictionsBrandsBlockedItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_links")]
impl<'json> json::Decode<'json> for CreatePaymentLinkParamsPaymentMethodOptionsCardRestrictionsBrandsBlockedItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreatePaymentLinkParamsPaymentMethodOptionsCardRestrictionsBrandsBlockedItem::Unknown(String::new()),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "american_express" => {
                CreatePaymentLinkParamsPaymentMethodOptionsCardRestrictionsBrandsBlockedItem::AmericanExpress
            }
            "discover_global_network" => {
                CreatePaymentLinkParamsPaymentMethodOptionsCardRestrictionsBrandsBlockedItem::DiscoverGlobalNetwork
            }
            "mastercard" => CreatePaymentLinkParamsPaymentMethodOptionsCardRestrictionsBrandsBlockedItem::Mastercard,
            "visa" => CreatePaymentLinkParamsPaymentMethodOptionsCardRestrictionsBrandsBlockedItem::Visa,
            other => {
                CreatePaymentLinkParamsPaymentMethodOptionsCardRestrictionsBrandsBlockedItem::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsPaymentMethodOptionsCardRestrictionsBrandsBlockedItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentLinkParamsPaymentMethodTypesItem {
    Affirm,
    AfterpayClearpay,
    Alipay,
    Alma,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Billie,
    Bizum,
    Blik,
    Boleto,
    Card,
    Cashapp,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Klarna,
    Konbini,
    Link,
    MbWay,
    Mobilepay,
    Multibanco,
    Oxxo,
    P24,
    PayByBank,
    Paynow,
    Paypal,
    Paypay,
    Payto,
    Pix,
    Promptpay,
    Satispay,
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

#[cfg(feature = "payment_links")]
impl CreatePaymentLinkParamsPaymentMethodTypesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentLinkParamsPaymentMethodTypesItem::Affirm => "affirm",
            CreatePaymentLinkParamsPaymentMethodTypesItem::AfterpayClearpay => "afterpay_clearpay",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Alipay => "alipay",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Alma => "alma",
            CreatePaymentLinkParamsPaymentMethodTypesItem::AuBecsDebit => "au_becs_debit",
            CreatePaymentLinkParamsPaymentMethodTypesItem::BacsDebit => "bacs_debit",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Bancontact => "bancontact",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Billie => "billie",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Bizum => "bizum",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Blik => "blik",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Boleto => "boleto",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Card => "card",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Cashapp => "cashapp",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Eps => "eps",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Fpx => "fpx",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Giropay => "giropay",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Grabpay => "grabpay",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Ideal => "ideal",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Klarna => "klarna",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Konbini => "konbini",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Link => "link",
            CreatePaymentLinkParamsPaymentMethodTypesItem::MbWay => "mb_way",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Mobilepay => "mobilepay",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Multibanco => "multibanco",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Oxxo => "oxxo",
            CreatePaymentLinkParamsPaymentMethodTypesItem::P24 => "p24",
            CreatePaymentLinkParamsPaymentMethodTypesItem::PayByBank => "pay_by_bank",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Paynow => "paynow",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Paypal => "paypal",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Paypay => "paypay",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Payto => "payto",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Pix => "pix",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Promptpay => "promptpay",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Satispay => "satispay",
            CreatePaymentLinkParamsPaymentMethodTypesItem::SepaDebit => "sepa_debit",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Sequra => "sequra",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Sofort => "sofort",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Sunbit => "sunbit",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Swish => "swish",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Twint => "twint",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Upi => "upi",
            CreatePaymentLinkParamsPaymentMethodTypesItem::UsBankAccount => "us_bank_account",
            CreatePaymentLinkParamsPaymentMethodTypesItem::WechatPay => "wechat_pay",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Zip => "zip",
            CreatePaymentLinkParamsPaymentMethodTypesItem::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "payment_links")]
impl<'json> json::Decode<'json> for CreatePaymentLinkParamsPaymentMethodTypesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreatePaymentLinkParamsPaymentMethodTypesItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "affirm" => CreatePaymentLinkParamsPaymentMethodTypesItem::Affirm,
            "afterpay_clearpay" => CreatePaymentLinkParamsPaymentMethodTypesItem::AfterpayClearpay,
            "alipay" => CreatePaymentLinkParamsPaymentMethodTypesItem::Alipay,
            "alma" => CreatePaymentLinkParamsPaymentMethodTypesItem::Alma,
            "au_becs_debit" => CreatePaymentLinkParamsPaymentMethodTypesItem::AuBecsDebit,
            "bacs_debit" => CreatePaymentLinkParamsPaymentMethodTypesItem::BacsDebit,
            "bancontact" => CreatePaymentLinkParamsPaymentMethodTypesItem::Bancontact,
            "billie" => CreatePaymentLinkParamsPaymentMethodTypesItem::Billie,
            "bizum" => CreatePaymentLinkParamsPaymentMethodTypesItem::Bizum,
            "blik" => CreatePaymentLinkParamsPaymentMethodTypesItem::Blik,
            "boleto" => CreatePaymentLinkParamsPaymentMethodTypesItem::Boleto,
            "card" => CreatePaymentLinkParamsPaymentMethodTypesItem::Card,
            "cashapp" => CreatePaymentLinkParamsPaymentMethodTypesItem::Cashapp,
            "eps" => CreatePaymentLinkParamsPaymentMethodTypesItem::Eps,
            "fpx" => CreatePaymentLinkParamsPaymentMethodTypesItem::Fpx,
            "giropay" => CreatePaymentLinkParamsPaymentMethodTypesItem::Giropay,
            "grabpay" => CreatePaymentLinkParamsPaymentMethodTypesItem::Grabpay,
            "ideal" => CreatePaymentLinkParamsPaymentMethodTypesItem::Ideal,
            "klarna" => CreatePaymentLinkParamsPaymentMethodTypesItem::Klarna,
            "konbini" => CreatePaymentLinkParamsPaymentMethodTypesItem::Konbini,
            "link" => CreatePaymentLinkParamsPaymentMethodTypesItem::Link,
            "mb_way" => CreatePaymentLinkParamsPaymentMethodTypesItem::MbWay,
            "mobilepay" => CreatePaymentLinkParamsPaymentMethodTypesItem::Mobilepay,
            "multibanco" => CreatePaymentLinkParamsPaymentMethodTypesItem::Multibanco,
            "oxxo" => CreatePaymentLinkParamsPaymentMethodTypesItem::Oxxo,
            "p24" => CreatePaymentLinkParamsPaymentMethodTypesItem::P24,
            "pay_by_bank" => CreatePaymentLinkParamsPaymentMethodTypesItem::PayByBank,
            "paynow" => CreatePaymentLinkParamsPaymentMethodTypesItem::Paynow,
            "paypal" => CreatePaymentLinkParamsPaymentMethodTypesItem::Paypal,
            "paypay" => CreatePaymentLinkParamsPaymentMethodTypesItem::Paypay,
            "payto" => CreatePaymentLinkParamsPaymentMethodTypesItem::Payto,
            "pix" => CreatePaymentLinkParamsPaymentMethodTypesItem::Pix,
            "promptpay" => CreatePaymentLinkParamsPaymentMethodTypesItem::Promptpay,
            "satispay" => CreatePaymentLinkParamsPaymentMethodTypesItem::Satispay,
            "sepa_debit" => CreatePaymentLinkParamsPaymentMethodTypesItem::SepaDebit,
            "sequra" => CreatePaymentLinkParamsPaymentMethodTypesItem::Sequra,
            "sofort" => CreatePaymentLinkParamsPaymentMethodTypesItem::Sofort,
            "sunbit" => CreatePaymentLinkParamsPaymentMethodTypesItem::Sunbit,
            "swish" => CreatePaymentLinkParamsPaymentMethodTypesItem::Swish,
            "twint" => CreatePaymentLinkParamsPaymentMethodTypesItem::Twint,
            "upi" => CreatePaymentLinkParamsPaymentMethodTypesItem::Upi,
            "us_bank_account" => CreatePaymentLinkParamsPaymentMethodTypesItem::UsBankAccount,
            "wechat_pay" => CreatePaymentLinkParamsPaymentMethodTypesItem::WechatPay,
            "zip" => CreatePaymentLinkParamsPaymentMethodTypesItem::Zip,
            other => CreatePaymentLinkParamsPaymentMethodTypesItem::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsPaymentMethodTypesItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentLinkParamsPhoneNumberCollection {
    #[doc = "Set to `true` to enable phone number collection."]
    pub enabled: bool,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsPhoneNumberCollection {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreatePaymentLinkParamsSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod {
    Cancel,
    CreateInvoice,
    Pause,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "payment_links")]
impl CreatePaymentLinkParamsSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreatePaymentLinkParamsSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::Cancel => "cancel",
            CreatePaymentLinkParamsSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::CreateInvoice => {
                "create_invoice"
            }
            CreatePaymentLinkParamsSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::Pause => "pause",
            CreatePaymentLinkParamsSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::Unknown(value) => {
                value
            }
        }
    }
}

#[cfg(feature = "payment_links")]
impl<'json> json::Decode<'json>
    for CreatePaymentLinkParamsSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreatePaymentLinkParamsSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "cancel" => CreatePaymentLinkParamsSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::Cancel,
            "create_invoice" => {
                CreatePaymentLinkParamsSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::CreateInvoice
            }
            "pause" => CreatePaymentLinkParamsSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::Pause,
            other => CreatePaymentLinkParamsSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for CreatePaymentLinkParamsSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "payment_links")]
#[derive(Debug, Clone, Default)]
pub struct ListPaymentLinksParams {
    pub active: Option<bool>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "payment_links")]
impl crate::params::FormEncode for ListPaymentLinksParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("active", self.active.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
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
pub enum PaymentLinkCustomerCreation {
    Always,
    IfRequired,
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
impl PaymentLinkCustomerCreation {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentLinkCustomerCreation::Always => "always",
            PaymentLinkCustomerCreation::IfRequired => "if_required",
            PaymentLinkCustomerCreation::Unknown(value) => value,
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
impl Default for PaymentLinkCustomerCreation {
    fn default() -> Self {
        PaymentLinkCustomerCreation::Unknown(String::new())
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
impl<'json> json::Decode<'json> for PaymentLinkCustomerCreation {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentLinkCustomerCreation::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "always" => PaymentLinkCustomerCreation::Always,
            "if_required" => PaymentLinkCustomerCreation::IfRequired,
            other => PaymentLinkCustomerCreation::Unknown(other.to_string()),
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
pub enum PaymentLinkSubmitType {
    Auto,
    Book,
    Donate,
    Pay,
    Subscribe,
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
impl PaymentLinkSubmitType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentLinkSubmitType::Auto => "auto",
            PaymentLinkSubmitType::Book => "book",
            PaymentLinkSubmitType::Donate => "donate",
            PaymentLinkSubmitType::Pay => "pay",
            PaymentLinkSubmitType::Subscribe => "subscribe",
            PaymentLinkSubmitType::Unknown(value) => value,
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
impl Default for PaymentLinkSubmitType {
    fn default() -> Self {
        PaymentLinkSubmitType::Unknown(String::new())
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
impl<'json> json::Decode<'json> for PaymentLinkSubmitType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentLinkSubmitType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "auto" => PaymentLinkSubmitType::Auto,
            "book" => PaymentLinkSubmitType::Book,
            "donate" => PaymentLinkSubmitType::Donate,
            "pay" => PaymentLinkSubmitType::Pay,
            "subscribe" => PaymentLinkSubmitType::Subscribe,
            other => PaymentLinkSubmitType::Unknown(other.to_string()),
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
pub enum PaymentLinksResourceCardRestrictionsBrandsBlockedItem {
    AmericanExpress,
    DiscoverGlobalNetwork,
    Mastercard,
    Visa,
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
impl PaymentLinksResourceCardRestrictionsBrandsBlockedItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentLinksResourceCardRestrictionsBrandsBlockedItem::AmericanExpress => "american_express",
            PaymentLinksResourceCardRestrictionsBrandsBlockedItem::DiscoverGlobalNetwork => "discover_global_network",
            PaymentLinksResourceCardRestrictionsBrandsBlockedItem::Mastercard => "mastercard",
            PaymentLinksResourceCardRestrictionsBrandsBlockedItem::Visa => "visa",
            PaymentLinksResourceCardRestrictionsBrandsBlockedItem::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PaymentLinksResourceCardRestrictionsBrandsBlockedItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentLinksResourceCardRestrictionsBrandsBlockedItem::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "american_express" => PaymentLinksResourceCardRestrictionsBrandsBlockedItem::AmericanExpress,
            "discover_global_network" => PaymentLinksResourceCardRestrictionsBrandsBlockedItem::DiscoverGlobalNetwork,
            "mastercard" => PaymentLinksResourceCardRestrictionsBrandsBlockedItem::Mastercard,
            "visa" => PaymentLinksResourceCardRestrictionsBrandsBlockedItem::Visa,
            other => PaymentLinksResourceCardRestrictionsBrandsBlockedItem::Unknown(other.to_string()),
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
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentLinksResourceCompletedSessions {
    #[doc = "The current number of checkout sessions that have been completed on the payment link which count towards the `complet..."]
    #[json(default)]
    pub count: i64,
    #[doc = "The maximum number of checkout sessions that can be completed for the `completed_sessions` restriction to be met."]
    #[json(default)]
    pub limit: i64,
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentLinksResourcePaymentMethodReuseAgreementPosition {
    Auto,
    Hidden,
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
impl PaymentLinksResourcePaymentMethodReuseAgreementPosition {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentLinksResourcePaymentMethodReuseAgreementPosition::Auto => "auto",
            PaymentLinksResourcePaymentMethodReuseAgreementPosition::Hidden => "hidden",
            PaymentLinksResourcePaymentMethodReuseAgreementPosition::Unknown(value) => value,
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
impl Default for PaymentLinksResourcePaymentMethodReuseAgreementPosition {
    fn default() -> Self {
        PaymentLinksResourcePaymentMethodReuseAgreementPosition::Unknown(String::new())
    }
}

#[cfg(any(
    feature = "checkout",
    feature = "credit_notes",
    feature = "customers",
    feature = "events",
    feature = "payment_links"
))]
impl<'json> json::Decode<'json> for PaymentLinksResourcePaymentMethodReuseAgreementPosition {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentLinksResourcePaymentMethodReuseAgreementPosition::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "auto" => PaymentLinksResourcePaymentMethodReuseAgreementPosition::Auto,
            "hidden" => PaymentLinksResourcePaymentMethodReuseAgreementPosition::Hidden,
            other => PaymentLinksResourcePaymentMethodReuseAgreementPosition::Unknown(other.to_string()),
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
pub enum PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem {
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
    feature = "events",
    feature = "payment_links"
))]
impl PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::AC => "AC",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::AD => "AD",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::AE => "AE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::AF => "AF",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::AG => "AG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::AI => "AI",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::AL => "AL",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::AM => "AM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::AO => "AO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::AQ => "AQ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::AR => "AR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::AT => "AT",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::AU => "AU",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::AW => "AW",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::AX => "AX",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::AZ => "AZ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BA => "BA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BB => "BB",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BD => "BD",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BE => "BE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BF => "BF",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BG => "BG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BH => "BH",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BI => "BI",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BJ => "BJ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BL => "BL",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BM => "BM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BN => "BN",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BO => "BO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BQ => "BQ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BR => "BR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BS => "BS",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BT => "BT",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BV => "BV",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BW => "BW",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BY => "BY",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BZ => "BZ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::CA => "CA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::CD => "CD",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::CF => "CF",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::CG => "CG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::CH => "CH",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::CI => "CI",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::CK => "CK",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::CL => "CL",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::CM => "CM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::CN => "CN",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::CO => "CO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::CR => "CR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::CV => "CV",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::CW => "CW",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::CY => "CY",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::CZ => "CZ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::DE => "DE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::DJ => "DJ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::DK => "DK",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::DM => "DM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::DO => "DO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::DZ => "DZ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::EC => "EC",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::EE => "EE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::EG => "EG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::EH => "EH",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::ER => "ER",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::ES => "ES",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::ET => "ET",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::FI => "FI",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::FJ => "FJ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::FK => "FK",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::FO => "FO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::FR => "FR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GA => "GA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GB => "GB",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GD => "GD",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GE => "GE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GF => "GF",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GG => "GG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GH => "GH",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GI => "GI",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GL => "GL",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GM => "GM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GN => "GN",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GP => "GP",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GQ => "GQ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GR => "GR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GS => "GS",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GT => "GT",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GU => "GU",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GW => "GW",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GY => "GY",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::HK => "HK",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::HN => "HN",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::HR => "HR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::HT => "HT",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::HU => "HU",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::ID => "ID",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::IE => "IE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::IL => "IL",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::IM => "IM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::IN => "IN",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::IO => "IO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::IQ => "IQ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::IS => "IS",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::IT => "IT",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::JE => "JE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::JM => "JM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::JO => "JO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::JP => "JP",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::KE => "KE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::KG => "KG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::KH => "KH",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::KI => "KI",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::KM => "KM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::KN => "KN",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::KR => "KR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::KW => "KW",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::KY => "KY",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::KZ => "KZ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::LA => "LA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::LB => "LB",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::LC => "LC",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::LI => "LI",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::LK => "LK",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::LR => "LR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::LS => "LS",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::LT => "LT",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::LU => "LU",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::LV => "LV",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::LY => "LY",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MA => "MA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MC => "MC",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MD => "MD",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::ME => "ME",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MF => "MF",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MG => "MG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MK => "MK",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::ML => "ML",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MM => "MM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MN => "MN",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MO => "MO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MQ => "MQ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MR => "MR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MS => "MS",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MT => "MT",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MU => "MU",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MV => "MV",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MW => "MW",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MX => "MX",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MY => "MY",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MZ => "MZ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::NA => "NA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::NC => "NC",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::NE => "NE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::NG => "NG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::NI => "NI",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::NL => "NL",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::NO => "NO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::NP => "NP",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::NR => "NR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::NU => "NU",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::NZ => "NZ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::OM => "OM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::PA => "PA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::PE => "PE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::PF => "PF",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::PG => "PG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::PH => "PH",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::PK => "PK",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::PL => "PL",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::PM => "PM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::PN => "PN",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::PR => "PR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::PS => "PS",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::PT => "PT",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::PY => "PY",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::QA => "QA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::RE => "RE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::RO => "RO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::RS => "RS",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::RU => "RU",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::RW => "RW",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SA => "SA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SB => "SB",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SC => "SC",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SD => "SD",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SE => "SE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SG => "SG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SH => "SH",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SI => "SI",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SJ => "SJ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SK => "SK",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SL => "SL",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SM => "SM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SN => "SN",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SO => "SO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SR => "SR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SS => "SS",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::ST => "ST",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SV => "SV",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SX => "SX",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SZ => "SZ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TA => "TA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TC => "TC",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TD => "TD",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TF => "TF",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TG => "TG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TH => "TH",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TJ => "TJ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TK => "TK",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TL => "TL",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TM => "TM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TN => "TN",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TO => "TO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TR => "TR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TT => "TT",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TV => "TV",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TW => "TW",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TZ => "TZ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::UA => "UA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::UG => "UG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::US => "US",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::UY => "UY",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::UZ => "UZ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::VA => "VA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::VC => "VC",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::VE => "VE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::VG => "VG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::VN => "VN",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::VU => "VU",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::WF => "WF",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::WS => "WS",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::XK => "XK",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::YE => "YE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::YT => "YT",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::ZA => "ZA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::ZM => "ZM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::ZW => "ZW",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::ZZ => "ZZ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::Unknown(value) => value,
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
impl<'json> json::Decode<'json> for PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "AC" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::AC,
            "AD" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::AD,
            "AE" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::AE,
            "AF" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::AF,
            "AG" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::AG,
            "AI" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::AI,
            "AL" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::AL,
            "AM" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::AM,
            "AO" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::AO,
            "AQ" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::AQ,
            "AR" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::AR,
            "AT" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::AT,
            "AU" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::AU,
            "AW" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::AW,
            "AX" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::AX,
            "AZ" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::AZ,
            "BA" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BA,
            "BB" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BB,
            "BD" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BD,
            "BE" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BE,
            "BF" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BF,
            "BG" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BG,
            "BH" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BH,
            "BI" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BI,
            "BJ" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BJ,
            "BL" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BL,
            "BM" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BM,
            "BN" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BN,
            "BO" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BO,
            "BQ" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BQ,
            "BR" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BR,
            "BS" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BS,
            "BT" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BT,
            "BV" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BV,
            "BW" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BW,
            "BY" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BY,
            "BZ" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::BZ,
            "CA" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::CA,
            "CD" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::CD,
            "CF" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::CF,
            "CG" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::CG,
            "CH" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::CH,
            "CI" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::CI,
            "CK" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::CK,
            "CL" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::CL,
            "CM" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::CM,
            "CN" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::CN,
            "CO" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::CO,
            "CR" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::CR,
            "CV" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::CV,
            "CW" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::CW,
            "CY" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::CY,
            "CZ" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::CZ,
            "DE" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::DE,
            "DJ" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::DJ,
            "DK" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::DK,
            "DM" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::DM,
            "DO" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::DO,
            "DZ" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::DZ,
            "EC" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::EC,
            "EE" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::EE,
            "EG" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::EG,
            "EH" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::EH,
            "ER" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::ER,
            "ES" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::ES,
            "ET" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::ET,
            "FI" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::FI,
            "FJ" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::FJ,
            "FK" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::FK,
            "FO" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::FO,
            "FR" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::FR,
            "GA" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GA,
            "GB" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GB,
            "GD" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GD,
            "GE" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GE,
            "GF" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GF,
            "GG" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GG,
            "GH" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GH,
            "GI" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GI,
            "GL" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GL,
            "GM" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GM,
            "GN" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GN,
            "GP" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GP,
            "GQ" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GQ,
            "GR" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GR,
            "GS" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GS,
            "GT" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GT,
            "GU" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GU,
            "GW" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GW,
            "GY" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::GY,
            "HK" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::HK,
            "HN" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::HN,
            "HR" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::HR,
            "HT" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::HT,
            "HU" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::HU,
            "ID" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::ID,
            "IE" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::IE,
            "IL" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::IL,
            "IM" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::IM,
            "IN" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::IN,
            "IO" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::IO,
            "IQ" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::IQ,
            "IS" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::IS,
            "IT" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::IT,
            "JE" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::JE,
            "JM" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::JM,
            "JO" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::JO,
            "JP" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::JP,
            "KE" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::KE,
            "KG" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::KG,
            "KH" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::KH,
            "KI" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::KI,
            "KM" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::KM,
            "KN" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::KN,
            "KR" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::KR,
            "KW" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::KW,
            "KY" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::KY,
            "KZ" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::KZ,
            "LA" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::LA,
            "LB" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::LB,
            "LC" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::LC,
            "LI" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::LI,
            "LK" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::LK,
            "LR" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::LR,
            "LS" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::LS,
            "LT" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::LT,
            "LU" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::LU,
            "LV" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::LV,
            "LY" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::LY,
            "MA" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MA,
            "MC" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MC,
            "MD" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MD,
            "ME" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::ME,
            "MF" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MF,
            "MG" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MG,
            "MK" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MK,
            "ML" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::ML,
            "MM" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MM,
            "MN" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MN,
            "MO" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MO,
            "MQ" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MQ,
            "MR" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MR,
            "MS" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MS,
            "MT" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MT,
            "MU" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MU,
            "MV" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MV,
            "MW" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MW,
            "MX" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MX,
            "MY" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MY,
            "MZ" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::MZ,
            "NA" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::NA,
            "NC" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::NC,
            "NE" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::NE,
            "NG" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::NG,
            "NI" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::NI,
            "NL" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::NL,
            "NO" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::NO,
            "NP" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::NP,
            "NR" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::NR,
            "NU" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::NU,
            "NZ" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::NZ,
            "OM" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::OM,
            "PA" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::PA,
            "PE" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::PE,
            "PF" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::PF,
            "PG" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::PG,
            "PH" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::PH,
            "PK" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::PK,
            "PL" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::PL,
            "PM" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::PM,
            "PN" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::PN,
            "PR" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::PR,
            "PS" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::PS,
            "PT" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::PT,
            "PY" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::PY,
            "QA" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::QA,
            "RE" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::RE,
            "RO" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::RO,
            "RS" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::RS,
            "RU" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::RU,
            "RW" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::RW,
            "SA" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SA,
            "SB" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SB,
            "SC" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SC,
            "SD" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SD,
            "SE" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SE,
            "SG" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SG,
            "SH" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SH,
            "SI" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SI,
            "SJ" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SJ,
            "SK" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SK,
            "SL" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SL,
            "SM" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SM,
            "SN" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SN,
            "SO" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SO,
            "SR" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SR,
            "SS" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SS,
            "ST" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::ST,
            "SV" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SV,
            "SX" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SX,
            "SZ" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::SZ,
            "TA" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TA,
            "TC" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TC,
            "TD" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TD,
            "TF" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TF,
            "TG" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TG,
            "TH" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TH,
            "TJ" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TJ,
            "TK" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TK,
            "TL" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TL,
            "TM" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TM,
            "TN" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TN,
            "TO" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TO,
            "TR" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TR,
            "TT" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TT,
            "TV" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TV,
            "TW" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TW,
            "TZ" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::TZ,
            "UA" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::UA,
            "UG" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::UG,
            "US" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::US,
            "UY" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::UY,
            "UZ" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::UZ,
            "VA" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::VA,
            "VC" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::VC,
            "VE" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::VE,
            "VG" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::VG,
            "VN" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::VN,
            "VU" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::VU,
            "WF" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::WF,
            "WS" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::WS,
            "XK" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::XK,
            "YE" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::YE,
            "YT" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::YT,
            "ZA" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::ZA,
            "ZM" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::ZM,
            "ZW" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::ZW,
            "ZZ" => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::ZZ,
            other => PaymentLinksResourceShippingAddressCollectionAllowedCountriesItem::Unknown(other.to_string()),
        })
    }
}
