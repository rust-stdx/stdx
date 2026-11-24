#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum BillingPortalSessionLocale {
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

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl BillingPortalSessionLocale {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            BillingPortalSessionLocale::Auto => "auto",
            BillingPortalSessionLocale::Bg => "bg",
            BillingPortalSessionLocale::Cs => "cs",
            BillingPortalSessionLocale::Da => "da",
            BillingPortalSessionLocale::De => "de",
            BillingPortalSessionLocale::El => "el",
            BillingPortalSessionLocale::En => "en",
            BillingPortalSessionLocale::EnAU => "en-AU",
            BillingPortalSessionLocale::EnCA => "en-CA",
            BillingPortalSessionLocale::EnGB => "en-GB",
            BillingPortalSessionLocale::EnIE => "en-IE",
            BillingPortalSessionLocale::EnIN => "en-IN",
            BillingPortalSessionLocale::EnNZ => "en-NZ",
            BillingPortalSessionLocale::EnSG => "en-SG",
            BillingPortalSessionLocale::Es => "es",
            BillingPortalSessionLocale::Es419 => "es-419",
            BillingPortalSessionLocale::Et => "et",
            BillingPortalSessionLocale::Fi => "fi",
            BillingPortalSessionLocale::Fil => "fil",
            BillingPortalSessionLocale::Fr => "fr",
            BillingPortalSessionLocale::FrCA => "fr-CA",
            BillingPortalSessionLocale::Hr => "hr",
            BillingPortalSessionLocale::Hu => "hu",
            BillingPortalSessionLocale::Id => "id",
            BillingPortalSessionLocale::It => "it",
            BillingPortalSessionLocale::Ja => "ja",
            BillingPortalSessionLocale::Ko => "ko",
            BillingPortalSessionLocale::Lt => "lt",
            BillingPortalSessionLocale::Lv => "lv",
            BillingPortalSessionLocale::Ms => "ms",
            BillingPortalSessionLocale::Mt => "mt",
            BillingPortalSessionLocale::Nb => "nb",
            BillingPortalSessionLocale::Nl => "nl",
            BillingPortalSessionLocale::Pl => "pl",
            BillingPortalSessionLocale::Pt => "pt",
            BillingPortalSessionLocale::PtBR => "pt-BR",
            BillingPortalSessionLocale::Ro => "ro",
            BillingPortalSessionLocale::Ru => "ru",
            BillingPortalSessionLocale::Sk => "sk",
            BillingPortalSessionLocale::Sl => "sl",
            BillingPortalSessionLocale::Sv => "sv",
            BillingPortalSessionLocale::Th => "th",
            BillingPortalSessionLocale::Tr => "tr",
            BillingPortalSessionLocale::Vi => "vi",
            BillingPortalSessionLocale::Zh => "zh",
            BillingPortalSessionLocale::ZhHK => "zh-HK",
            BillingPortalSessionLocale::ZhTW => "zh-TW",
            BillingPortalSessionLocale::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl<'json> json::Decode<'json> for BillingPortalSessionLocale {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(BillingPortalSessionLocale::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "auto" => BillingPortalSessionLocale::Auto,
            "bg" => BillingPortalSessionLocale::Bg,
            "cs" => BillingPortalSessionLocale::Cs,
            "da" => BillingPortalSessionLocale::Da,
            "de" => BillingPortalSessionLocale::De,
            "el" => BillingPortalSessionLocale::El,
            "en" => BillingPortalSessionLocale::En,
            "en-AU" => BillingPortalSessionLocale::EnAU,
            "en-CA" => BillingPortalSessionLocale::EnCA,
            "en-GB" => BillingPortalSessionLocale::EnGB,
            "en-IE" => BillingPortalSessionLocale::EnIE,
            "en-IN" => BillingPortalSessionLocale::EnIN,
            "en-NZ" => BillingPortalSessionLocale::EnNZ,
            "en-SG" => BillingPortalSessionLocale::EnSG,
            "es" => BillingPortalSessionLocale::Es,
            "es-419" => BillingPortalSessionLocale::Es419,
            "et" => BillingPortalSessionLocale::Et,
            "fi" => BillingPortalSessionLocale::Fi,
            "fil" => BillingPortalSessionLocale::Fil,
            "fr" => BillingPortalSessionLocale::Fr,
            "fr-CA" => BillingPortalSessionLocale::FrCA,
            "hr" => BillingPortalSessionLocale::Hr,
            "hu" => BillingPortalSessionLocale::Hu,
            "id" => BillingPortalSessionLocale::Id,
            "it" => BillingPortalSessionLocale::It,
            "ja" => BillingPortalSessionLocale::Ja,
            "ko" => BillingPortalSessionLocale::Ko,
            "lt" => BillingPortalSessionLocale::Lt,
            "lv" => BillingPortalSessionLocale::Lv,
            "ms" => BillingPortalSessionLocale::Ms,
            "mt" => BillingPortalSessionLocale::Mt,
            "nb" => BillingPortalSessionLocale::Nb,
            "nl" => BillingPortalSessionLocale::Nl,
            "pl" => BillingPortalSessionLocale::Pl,
            "pt" => BillingPortalSessionLocale::Pt,
            "pt-BR" => BillingPortalSessionLocale::PtBR,
            "ro" => BillingPortalSessionLocale::Ro,
            "ru" => BillingPortalSessionLocale::Ru,
            "sk" => BillingPortalSessionLocale::Sk,
            "sl" => BillingPortalSessionLocale::Sl,
            "sv" => BillingPortalSessionLocale::Sv,
            "th" => BillingPortalSessionLocale::Th,
            "tr" => BillingPortalSessionLocale::Tr,
            "vi" => BillingPortalSessionLocale::Vi,
            "zh" => BillingPortalSessionLocale::Zh,
            "zh-HK" => BillingPortalSessionLocale::ZhHK,
            "zh-TW" => BillingPortalSessionLocale::ZhTW,
            other => BillingPortalSessionLocale::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, Default)]
pub struct CreateBillingPortalConfigurationParamsBusinessProfile {
    #[doc = "The messaging shown to customers in the portal."]
    pub headline: Option<json::OwnedValue>,
    #[doc = "A link to the business’s publicly available privacy policy."]
    pub privacy_policy_url: Option<String>,
    #[doc = "A link to the business’s publicly available terms of service."]
    pub terms_of_service_url: Option<String>,
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for CreateBillingPortalConfigurationParamsBusinessProfile {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("privacy_policy_url", self.privacy_policy_url.as_ref());
        form.field_opt("terms_of_service_url", self.terms_of_service_url.as_ref());
    }
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, Default)]
pub struct CreateBillingPortalConfigurationParamsFeaturesCustomerUpdate {
    #[doc = "The types of customer updates that are supported."]
    pub allowed_updates: Option<json::OwnedValue>,
    #[doc = "Whether the feature is enabled."]
    pub enabled: bool,
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for CreateBillingPortalConfigurationParamsFeaturesCustomerUpdate {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("enabled", &self.enabled);
    }
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateTrialUpdateBehavior {
    ContinueTrial,
    EndTrial,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "billing_portal")]
impl CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateTrialUpdateBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateTrialUpdateBehavior::ContinueTrial => {
                "continue_trial"
            }
            CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateTrialUpdateBehavior::EndTrial => {
                "end_trial"
            }
            CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateTrialUpdateBehavior::Unknown(value) => {
                value
            }
        }
    }
}

#[cfg(feature = "billing_portal")]
impl<'json> json::Decode<'json>
    for CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateTrialUpdateBehavior
{
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(
                CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateTrialUpdateBehavior::Unknown(
                    String::new(),
                ),
            );
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "continue_trial" => {
                CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateTrialUpdateBehavior::ContinueTrial
            }
            "end_trial" => {
                CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateTrialUpdateBehavior::EndTrial
            }
            other => CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateTrialUpdateBehavior::Unknown(
                other.to_string(),
            ),
        })
    }
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdateTrialUpdateBehavior {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateBillingPortalSessionParamsFlowDataAfterCompletionType {
    HostedConfirmation,
    PortalHomepage,
    Redirect,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "billing_portal")]
impl CreateBillingPortalSessionParamsFlowDataAfterCompletionType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateBillingPortalSessionParamsFlowDataAfterCompletionType::HostedConfirmation => "hosted_confirmation",
            CreateBillingPortalSessionParamsFlowDataAfterCompletionType::PortalHomepage => "portal_homepage",
            CreateBillingPortalSessionParamsFlowDataAfterCompletionType::Redirect => "redirect",
            CreateBillingPortalSessionParamsFlowDataAfterCompletionType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "billing_portal")]
impl<'json> json::Decode<'json> for CreateBillingPortalSessionParamsFlowDataAfterCompletionType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateBillingPortalSessionParamsFlowDataAfterCompletionType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "hosted_confirmation" => CreateBillingPortalSessionParamsFlowDataAfterCompletionType::HostedConfirmation,
            "portal_homepage" => CreateBillingPortalSessionParamsFlowDataAfterCompletionType::PortalHomepage,
            "redirect" => CreateBillingPortalSessionParamsFlowDataAfterCompletionType::Redirect,
            other => CreateBillingPortalSessionParamsFlowDataAfterCompletionType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for CreateBillingPortalSessionParamsFlowDataAfterCompletionType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, PartialEq)]
pub enum CreateBillingPortalSessionParamsFlowDataSubscriptionCancelRetentionType {
    CouponOffer,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "billing_portal")]
impl CreateBillingPortalSessionParamsFlowDataSubscriptionCancelRetentionType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            CreateBillingPortalSessionParamsFlowDataSubscriptionCancelRetentionType::CouponOffer => "coupon_offer",
            CreateBillingPortalSessionParamsFlowDataSubscriptionCancelRetentionType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "billing_portal")]
impl<'json> json::Decode<'json> for CreateBillingPortalSessionParamsFlowDataSubscriptionCancelRetentionType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(CreateBillingPortalSessionParamsFlowDataSubscriptionCancelRetentionType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "coupon_offer" => CreateBillingPortalSessionParamsFlowDataSubscriptionCancelRetentionType::CouponOffer,
            other => {
                CreateBillingPortalSessionParamsFlowDataSubscriptionCancelRetentionType::Unknown(other.to_string())
            }
        })
    }
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for CreateBillingPortalSessionParamsFlowDataSubscriptionCancelRetentionType {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.set_str(self.as_str());
    }
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, Default)]
pub struct CreateBillingPortalSessionParamsFlowDataSubscriptionUpdateConfirmDiscountsItem {
    #[doc = "The ID of the coupon to apply to this subscription update."]
    pub coupon: Option<String>,
    #[doc = "The ID of a promotion code to apply to this subscription update."]
    pub promotion_code: Option<String>,
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for CreateBillingPortalSessionParamsFlowDataSubscriptionUpdateConfirmDiscountsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("coupon", self.coupon.as_ref());
        form.field_opt("promotion_code", self.promotion_code.as_ref());
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PortalBusinessProfile {
    #[doc = "The messaging shown to customers in the portal."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub headline: Option<String>,
    #[doc = "A link to the business’s publicly available privacy policy."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub privacy_policy_url: Option<String>,
    #[doc = "A link to the business’s publicly available terms of service."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub terms_of_service_url: Option<String>,
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PortalFlowsAfterCompletionRedirect {
    #[doc = "The URL the customer will be redirected to after the flow is completed."]
    #[json(default)]
    pub return_url: String,
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PortalFlowsFlowCustomerUpdate {}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PortalFlowsSubscriptionUpdateConfirmDiscount {
    #[doc = "The ID of the coupon to apply to this subscription update."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,
    #[doc = "The ID of a promotion code to apply to this subscription update."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PortalLoginPage {
    #[doc = "If `true`, a shareable `url` will be generated that will take your customers to a hosted login page for the customer ..."]
    #[json(default)]
    pub enabled: bool,
    #[doc = "A shareable URL to the hosted portal login page."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, PartialEq)]
pub enum PortalSubscriptionUpdateTrialUpdateBehavior {
    ContinueTrial,
    EndTrial,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl PortalSubscriptionUpdateTrialUpdateBehavior {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            PortalSubscriptionUpdateTrialUpdateBehavior::ContinueTrial => "continue_trial",
            PortalSubscriptionUpdateTrialUpdateBehavior::EndTrial => "end_trial",
            PortalSubscriptionUpdateTrialUpdateBehavior::Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl Default for PortalSubscriptionUpdateTrialUpdateBehavior {
    fn default() -> Self {
        PortalSubscriptionUpdateTrialUpdateBehavior::Unknown(String::new())
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl<'json> json::Decode<'json> for PortalSubscriptionUpdateTrialUpdateBehavior {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(PortalSubscriptionUpdateTrialUpdateBehavior::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "continue_trial" => PortalSubscriptionUpdateTrialUpdateBehavior::ContinueTrial,
            "end_trial" => PortalSubscriptionUpdateTrialUpdateBehavior::EndTrial,
            other => PortalSubscriptionUpdateTrialUpdateBehavior::Unknown(other.to_string()),
        })
    }
}
