#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsItemType {
    DecreasingItemAmount,
    ShorteningInterval,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "billing_portal")]
impl CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsItemType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsItemType::DecreasingItemAmount => "decreasing_item_amount",
            CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsItemType::ShorteningInterval => "shortening_interval",
            CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsItemType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "billing_portal")]
impl<'json> json::Decode<'json>
    for CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsItemType
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsItemType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "decreasing_item_amount" => CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsItemType::DecreasingItemAmount,
            "shortening_interval" => CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsItemType::ShorteningInterval,
            other => CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsItemType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode
    for CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateScheduleAtPeriodEndConditionsItemType
{
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, Default)]
pub struct CreateBillingPortalSessionParamsFlowDataAfterCompletionRedirect {
    #[doc = "The URL the customer will be redirected to after the flow is completed."]
    pub return_url: String,
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for CreateBillingPortalSessionParamsFlowDataAfterCompletionRedirect {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("return_url", &self.return_url);
    }
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, Default)]
pub struct CreateBillingPortalSessionParamsFlowDataSubscriptionCancelRetentionCouponOffer {
    #[doc = "The ID of the coupon to be offered."]
    pub coupon: String,
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for CreateBillingPortalSessionParamsFlowDataSubscriptionCancelRetentionCouponOffer {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("coupon", &self.coupon);
    }
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateBillingPortalSessionParamsLocale {
    Auto,
    Bg,
    Cs,
    Da,
    De,
    El,
    En,
    EnAU,
    EnCA,
    EnGB,
    EnIE,
    EnIN,
    EnNZ,
    EnSG,
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

#[cfg(feature = "billing_portal")]
impl CreateBillingPortalSessionParamsLocale {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateBillingPortalSessionParamsLocale::Auto => "auto",
            CreateBillingPortalSessionParamsLocale::Bg => "bg",
            CreateBillingPortalSessionParamsLocale::Cs => "cs",
            CreateBillingPortalSessionParamsLocale::Da => "da",
            CreateBillingPortalSessionParamsLocale::De => "de",
            CreateBillingPortalSessionParamsLocale::El => "el",
            CreateBillingPortalSessionParamsLocale::En => "en",
            CreateBillingPortalSessionParamsLocale::EnAU => "en-AU",
            CreateBillingPortalSessionParamsLocale::EnCA => "en-CA",
            CreateBillingPortalSessionParamsLocale::EnGB => "en-GB",
            CreateBillingPortalSessionParamsLocale::EnIE => "en-IE",
            CreateBillingPortalSessionParamsLocale::EnIN => "en-IN",
            CreateBillingPortalSessionParamsLocale::EnNZ => "en-NZ",
            CreateBillingPortalSessionParamsLocale::EnSG => "en-SG",
            CreateBillingPortalSessionParamsLocale::Es => "es",
            CreateBillingPortalSessionParamsLocale::Es419 => "es-419",
            CreateBillingPortalSessionParamsLocale::Et => "et",
            CreateBillingPortalSessionParamsLocale::Fi => "fi",
            CreateBillingPortalSessionParamsLocale::Fil => "fil",
            CreateBillingPortalSessionParamsLocale::Fr => "fr",
            CreateBillingPortalSessionParamsLocale::FrCA => "fr-CA",
            CreateBillingPortalSessionParamsLocale::Hr => "hr",
            CreateBillingPortalSessionParamsLocale::Hu => "hu",
            CreateBillingPortalSessionParamsLocale::Id => "id",
            CreateBillingPortalSessionParamsLocale::It => "it",
            CreateBillingPortalSessionParamsLocale::Ja => "ja",
            CreateBillingPortalSessionParamsLocale::Ko => "ko",
            CreateBillingPortalSessionParamsLocale::Lt => "lt",
            CreateBillingPortalSessionParamsLocale::Lv => "lv",
            CreateBillingPortalSessionParamsLocale::Ms => "ms",
            CreateBillingPortalSessionParamsLocale::Mt => "mt",
            CreateBillingPortalSessionParamsLocale::Nb => "nb",
            CreateBillingPortalSessionParamsLocale::Nl => "nl",
            CreateBillingPortalSessionParamsLocale::Pl => "pl",
            CreateBillingPortalSessionParamsLocale::Pt => "pt",
            CreateBillingPortalSessionParamsLocale::PtBR => "pt-BR",
            CreateBillingPortalSessionParamsLocale::Ro => "ro",
            CreateBillingPortalSessionParamsLocale::Ru => "ru",
            CreateBillingPortalSessionParamsLocale::Sk => "sk",
            CreateBillingPortalSessionParamsLocale::Sl => "sl",
            CreateBillingPortalSessionParamsLocale::Sv => "sv",
            CreateBillingPortalSessionParamsLocale::Th => "th",
            CreateBillingPortalSessionParamsLocale::Tr => "tr",
            CreateBillingPortalSessionParamsLocale::Vi => "vi",
            CreateBillingPortalSessionParamsLocale::Zh => "zh",
            CreateBillingPortalSessionParamsLocale::ZhHK => "zh-HK",
            CreateBillingPortalSessionParamsLocale::ZhTW => "zh-TW",
            CreateBillingPortalSessionParamsLocale::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "billing_portal")]
impl<'json> json::Decode<'json> for CreateBillingPortalSessionParamsLocale {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateBillingPortalSessionParamsLocale::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "auto" => CreateBillingPortalSessionParamsLocale::Auto,
            "bg" => CreateBillingPortalSessionParamsLocale::Bg,
            "cs" => CreateBillingPortalSessionParamsLocale::Cs,
            "da" => CreateBillingPortalSessionParamsLocale::Da,
            "de" => CreateBillingPortalSessionParamsLocale::De,
            "el" => CreateBillingPortalSessionParamsLocale::El,
            "en" => CreateBillingPortalSessionParamsLocale::En,
            "en-AU" => CreateBillingPortalSessionParamsLocale::EnAU,
            "en-CA" => CreateBillingPortalSessionParamsLocale::EnCA,
            "en-GB" => CreateBillingPortalSessionParamsLocale::EnGB,
            "en-IE" => CreateBillingPortalSessionParamsLocale::EnIE,
            "en-IN" => CreateBillingPortalSessionParamsLocale::EnIN,
            "en-NZ" => CreateBillingPortalSessionParamsLocale::EnNZ,
            "en-SG" => CreateBillingPortalSessionParamsLocale::EnSG,
            "es" => CreateBillingPortalSessionParamsLocale::Es,
            "es-419" => CreateBillingPortalSessionParamsLocale::Es419,
            "et" => CreateBillingPortalSessionParamsLocale::Et,
            "fi" => CreateBillingPortalSessionParamsLocale::Fi,
            "fil" => CreateBillingPortalSessionParamsLocale::Fil,
            "fr" => CreateBillingPortalSessionParamsLocale::Fr,
            "fr-CA" => CreateBillingPortalSessionParamsLocale::FrCA,
            "hr" => CreateBillingPortalSessionParamsLocale::Hr,
            "hu" => CreateBillingPortalSessionParamsLocale::Hu,
            "id" => CreateBillingPortalSessionParamsLocale::Id,
            "it" => CreateBillingPortalSessionParamsLocale::It,
            "ja" => CreateBillingPortalSessionParamsLocale::Ja,
            "ko" => CreateBillingPortalSessionParamsLocale::Ko,
            "lt" => CreateBillingPortalSessionParamsLocale::Lt,
            "lv" => CreateBillingPortalSessionParamsLocale::Lv,
            "ms" => CreateBillingPortalSessionParamsLocale::Ms,
            "mt" => CreateBillingPortalSessionParamsLocale::Mt,
            "nb" => CreateBillingPortalSessionParamsLocale::Nb,
            "nl" => CreateBillingPortalSessionParamsLocale::Nl,
            "pl" => CreateBillingPortalSessionParamsLocale::Pl,
            "pt" => CreateBillingPortalSessionParamsLocale::Pt,
            "pt-BR" => CreateBillingPortalSessionParamsLocale::PtBR,
            "ro" => CreateBillingPortalSessionParamsLocale::Ro,
            "ru" => CreateBillingPortalSessionParamsLocale::Ru,
            "sk" => CreateBillingPortalSessionParamsLocale::Sk,
            "sl" => CreateBillingPortalSessionParamsLocale::Sl,
            "sv" => CreateBillingPortalSessionParamsLocale::Sv,
            "th" => CreateBillingPortalSessionParamsLocale::Th,
            "tr" => CreateBillingPortalSessionParamsLocale::Tr,
            "vi" => CreateBillingPortalSessionParamsLocale::Vi,
            "zh" => CreateBillingPortalSessionParamsLocale::Zh,
            "zh-HK" => CreateBillingPortalSessionParamsLocale::ZhHK,
            "zh-TW" => CreateBillingPortalSessionParamsLocale::ZhTW,
            other => CreateBillingPortalSessionParamsLocale::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for CreateBillingPortalSessionParamsLocale {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum PortalResourceScheduleUpdateAtPeriodEndConditionType {
    DecreasingItemAmount,
    ShorteningInterval,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl PortalResourceScheduleUpdateAtPeriodEndConditionType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PortalResourceScheduleUpdateAtPeriodEndConditionType::DecreasingItemAmount => "decreasing_item_amount",
            PortalResourceScheduleUpdateAtPeriodEndConditionType::ShorteningInterval => "shortening_interval",
            PortalResourceScheduleUpdateAtPeriodEndConditionType::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl Default for PortalResourceScheduleUpdateAtPeriodEndConditionType {
    fn default() -> Self {
        PortalResourceScheduleUpdateAtPeriodEndConditionType::Unknown(String::new())
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl<'json> json::Decode<'json> for PortalResourceScheduleUpdateAtPeriodEndConditionType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PortalResourceScheduleUpdateAtPeriodEndConditionType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "decreasing_item_amount" => PortalResourceScheduleUpdateAtPeriodEndConditionType::DecreasingItemAmount,
            "shortening_interval" => PortalResourceScheduleUpdateAtPeriodEndConditionType::ShorteningInterval,
            other => PortalResourceScheduleUpdateAtPeriodEndConditionType::Unknown(other.to_string()),
        })
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum PortalSubscriptionUpdateBillingCycleAnchor {
    Now,
    Unchanged,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl PortalSubscriptionUpdateBillingCycleAnchor {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PortalSubscriptionUpdateBillingCycleAnchor::Now => "now",
            PortalSubscriptionUpdateBillingCycleAnchor::Unchanged => "unchanged",
            PortalSubscriptionUpdateBillingCycleAnchor::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl<'json> json::Decode<'json> for PortalSubscriptionUpdateBillingCycleAnchor {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PortalSubscriptionUpdateBillingCycleAnchor::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "now" => PortalSubscriptionUpdateBillingCycleAnchor::Now,
            "unchanged" => PortalSubscriptionUpdateBillingCycleAnchor::Unchanged,
            other => PortalSubscriptionUpdateBillingCycleAnchor::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, Default)]
pub struct UpdateBillingPortalConfigurationParamsFeaturesCustomerUpdate {
    #[doc = "The types of customer updates that are supported."]
    pub allowed_updates: Option<json::OwnedValue>,
    #[doc = "Whether the feature is enabled."]
    pub enabled: Option<bool>,
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for UpdateBillingPortalConfigurationParamsFeaturesCustomerUpdate {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("enabled", self.enabled.as_ref());
    }
}
