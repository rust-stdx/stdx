#[cfg(feature = "terminal")]
#[derive(Debug, Clone, Default)]
pub struct CollectPaymentMethodParamsCollectConfig {
    #[doc = "This field indicates whether this payment method can be shown again to its customer in a checkout flow."]
    pub allow_redisplay: Option<stripe_types_01::terminal::CollectPaymentMethodParamsCollectConfigAllowRedisplay>,
    #[doc = "Enables cancel button on transaction screens."]
    pub enable_customer_cancellation: Option<bool>,
    #[doc = "Override showing a tipping selection screen on this transaction."]
    pub skip_tipping: Option<bool>,
    #[doc = "Tipping configuration for this transaction."]
    pub tipping: Option<stripe_types_02::terminal::CollectPaymentMethodParamsCollectConfigTipping>,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for CollectPaymentMethodParamsCollectConfig {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("allow_redisplay", self.allow_redisplay.as_ref());
        form.field_opt("enable_customer_cancellation", self.enable_customer_cancellation.as_ref());
        form.field_opt("skip_tipping", self.skip_tipping.as_ref());
        form.field_opt("tipping", self.tipping.as_ref());
    }
}

#[cfg(feature = "terminal")]
pub type CreateTerminalConfigurationParamsBbposWiseposE =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsBbposWisepad3;

#[cfg(feature = "terminal")]
pub type CreateTerminalConfigurationParamsOffline =
    stripe_types_01::terminal::CreateTerminalConfigurationParamsCellular;

#[cfg(feature = "terminal")]
pub type CreateTerminalConfigurationParamsStripeS700 =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsBbposWisepad3;

#[cfg(feature = "terminal")]
pub type CreateTerminalConfigurationParamsTippingAud =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type CreateTerminalConfigurationParamsTippingChf =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type CreateTerminalConfigurationParamsTippingDkk =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type CreateTerminalConfigurationParamsTippingGbp =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type CreateTerminalConfigurationParamsTippingHkd =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type CreateTerminalConfigurationParamsTippingJpy =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type CreateTerminalConfigurationParamsTippingMyr =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type CreateTerminalConfigurationParamsTippingNzd =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type CreateTerminalConfigurationParamsTippingRon =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type CreateTerminalConfigurationParamsTippingSgd =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type CreateTerminalConfigurationParamsVerifoneP400 =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsBbposWisepad3;

#[cfg(feature = "terminal")]
pub type CreateTerminalConfigurationParamsVerifoneV660p =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsBbposWisepad3;

#[cfg(feature = "terminal")]
#[derive(Debug, Clone)]
pub struct CreateTerminalConfigurationParamsWifi {
    #[doc = "Credentials for a WPA-Enterprise WiFi network using the EAP-PEAP authentication method."]
    pub enterprise_eap_peap: Option<stripe_types_03::terminal::CreateTerminalConfigurationParamsWifiEnterpriseEapPeap>,
    #[doc = "Credentials for a WPA-Enterprise WiFi network using the EAP-TLS authentication method."]
    pub enterprise_eap_tls: Option<stripe_types_01::terminal::CreateTerminalConfigurationParamsWifiEnterpriseEapTls>,
    #[doc = "Credentials for a WPA-Personal WiFi network."]
    pub personal_psk: Option<stripe_types_03::terminal::CreateTerminalConfigurationParamsWifiPersonalPsk>,
    #[doc = "Security type of the WiFi network."]
    pub r#type: stripe_types_01::terminal::CreateTerminalConfigurationParamsWifiType,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for CreateTerminalConfigurationParamsWifi {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("enterprise_eap_peap", self.enterprise_eap_peap.as_ref());
        form.field_opt("enterprise_eap_tls", self.enterprise_eap_tls.as_ref());
        form.field_opt("personal_psk", self.personal_psk.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, Default)]
pub struct CreateTerminalOnboardingLinkParamsLinkOptions {
    #[doc = "The options associated with the Apple Terms and Conditions link type."]
    pub apple_terms_and_conditions:
        Option<stripe_types_04::terminal::CreateTerminalOnboardingLinkParamsLinkOptionsAppleTermsAndConditions>,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for CreateTerminalOnboardingLinkParamsLinkOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("apple_terms_and_conditions", self.apple_terms_and_conditions.as_ref());
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, Default)]
pub struct ListTerminalReadersParams {
    pub device_type: Option<stripe_types_00::terminal::ListTerminalReadersParamsDeviceType>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub location: Option<String>,
    pub serial_number: Option<String>,
    pub starting_after: Option<String>,
    pub status: Option<stripe_types_04::terminal::ListTerminalReadersParamsStatus>,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for ListTerminalReadersParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("device_type", self.device_type.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("location", self.location.as_ref());
        form.field_opt("serial_number", self.serial_number.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("status", self.status.as_ref());
    }
}

#[cfg(feature = "terminal")]
pub type ProcessPaymentIntentParamsProcessConfigAllowRedisplay =
    stripe_types_01::terminal::CollectPaymentMethodParamsCollectConfigAllowRedisplay;

#[cfg(feature = "terminal")]
pub type ProcessSetupIntentParamsAllowRedisplay =
    stripe_types_01::terminal::CollectPaymentMethodParamsCollectConfigAllowRedisplay;

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalConfigurationConfigurationResourceTipping {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub aed: Option<stripe_types_04::terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub aud: Option<stripe_types_04::terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cad: Option<stripe_types_04::terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub chf: Option<stripe_types_04::terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub czk: Option<stripe_types_04::terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub dkk: Option<stripe_types_04::terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub eur: Option<stripe_types_04::terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub gbp: Option<stripe_types_04::terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub gip: Option<stripe_types_04::terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub hkd: Option<stripe_types_04::terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub huf: Option<stripe_types_04::terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub jpy: Option<stripe_types_04::terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mxn: Option<stripe_types_04::terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub myr: Option<stripe_types_04::terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub nok: Option<stripe_types_04::terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub nzd: Option<stripe_types_04::terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pln: Option<stripe_types_04::terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ron: Option<stripe_types_04::terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sek: Option<stripe_types_04::terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sgd: Option<stripe_types_04::terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub usd: Option<stripe_types_04::terminal::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalConnectionToken {
    #[doc = "The id of the location that this connection token is scoped to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::terminal::TerminalConnectionTokenObject,
    #[doc = "Your application should pass this token to the Stripe Terminal SDK."]
    #[json(default)]
    pub secret: String,
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalOnboardingLinkLinkOptions {
    #[doc = "The options associated with the Apple Terms and Conditions link type."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub apple_terms_and_conditions: Option<stripe_types_02::terminal::TerminalOnboardingLinkAppleTermsAndConditions>,
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalReaderReaderResourceCart {
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "List of line items in the cart."]
    #[json(default)]
    pub line_items: Vec<stripe_types_03::terminal::TerminalReaderReaderResourceLineItem>,
    #[doc = "Tax amount for the entire cart."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax: Option<i64>,
    #[doc = "Total amount for the entire cart, including tax."]
    #[json(default)]
    pub total: i64,
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalReaderReaderResourceCollectConfig {
    #[doc = "Enable customer-initiated cancellation when processing this payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub enable_customer_cancellation: Option<bool>,
    #[doc = "Override showing a tipping selection screen on this transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub skip_tipping: Option<bool>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tipping: Option<stripe_types_04::terminal::TerminalReaderReaderResourceTippingConfig>,
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalReaderReaderResourcePrintContent {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<stripe_types_01::terminal::TerminalReaderReaderResourceFileMetadata>,
    #[doc = "The type of content to print."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::terminal::TerminalReaderReaderResourcePrintContentType,
}

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsBbposWisepad3 =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsBbposWisepad3;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsStripeS700 =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsBbposWisepad3;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsTippingAed =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsTippingCad =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsTippingCzk =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsTippingEur =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsTippingGip =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsTippingHuf =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsTippingMxn =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsTippingNok =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsTippingPln =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsTippingSek =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsTippingUsd =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsVerifoneP400 =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsBbposWisepad3;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsVerifoneUx700 =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsBbposWisepad3;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsWifiPersonalPsk =
    stripe_types_03::terminal::CreateTerminalConfigurationParamsWifiPersonalPsk;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsWifiType =
    stripe_types_01::terminal::CreateTerminalConfigurationParamsWifiType;

#[cfg(feature = "terminal")]
pub type UpdateTerminalLocationParamsAddressKana = stripe_types_02::terminal::CreateTerminalLocationParamsAddressKana;

#[cfg(feature = "terminal")]
pub type UpdateTerminalLocationParamsAddressKanji = stripe_types_02::terminal::CreateTerminalLocationParamsAddressKana;
