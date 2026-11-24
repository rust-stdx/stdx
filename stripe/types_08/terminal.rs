#[cfg(feature = "terminal")]
#[derive(Debug, Clone, Default)]
pub struct CreateTerminalConfigurationParamsTipping {
    #[doc = "Tipping configuration for AED"]
    pub aed: Option<stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed>,
    #[doc = "Tipping configuration for AUD"]
    pub aud: Option<stripe_types_05::terminal::CreateTerminalConfigurationParamsTippingAud>,
    #[doc = "Tipping configuration for CAD"]
    pub cad: Option<stripe_types_06::terminal::CreateTerminalConfigurationParamsTippingCad>,
    #[doc = "Tipping configuration for CHF"]
    pub chf: Option<stripe_types_05::terminal::CreateTerminalConfigurationParamsTippingChf>,
    #[doc = "Tipping configuration for CZK"]
    pub czk: Option<stripe_types_06::terminal::CreateTerminalConfigurationParamsTippingCzk>,
    #[doc = "Tipping configuration for DKK"]
    pub dkk: Option<stripe_types_05::terminal::CreateTerminalConfigurationParamsTippingDkk>,
    #[doc = "Tipping configuration for EUR"]
    pub eur: Option<stripe_types_06::terminal::CreateTerminalConfigurationParamsTippingEur>,
    #[doc = "Tipping configuration for GBP"]
    pub gbp: Option<stripe_types_05::terminal::CreateTerminalConfigurationParamsTippingGbp>,
    #[doc = "Tipping configuration for GIP"]
    pub gip: Option<stripe_types_06::terminal::CreateTerminalConfigurationParamsTippingGip>,
    #[doc = "Tipping configuration for HKD"]
    pub hkd: Option<stripe_types_05::terminal::CreateTerminalConfigurationParamsTippingHkd>,
    #[doc = "Tipping configuration for HUF"]
    pub huf: Option<stripe_types_06::terminal::CreateTerminalConfigurationParamsTippingHuf>,
    #[doc = "Tipping configuration for JPY"]
    pub jpy: Option<stripe_types_05::terminal::CreateTerminalConfigurationParamsTippingJpy>,
    #[doc = "Tipping configuration for MXN"]
    pub mxn: Option<stripe_types_06::terminal::CreateTerminalConfigurationParamsTippingMxn>,
    #[doc = "Tipping configuration for MYR"]
    pub myr: Option<stripe_types_05::terminal::CreateTerminalConfigurationParamsTippingMyr>,
    #[doc = "Tipping configuration for NOK"]
    pub nok: Option<stripe_types_06::terminal::CreateTerminalConfigurationParamsTippingNok>,
    #[doc = "Tipping configuration for NZD"]
    pub nzd: Option<stripe_types_05::terminal::CreateTerminalConfigurationParamsTippingNzd>,
    #[doc = "Tipping configuration for PLN"]
    pub pln: Option<stripe_types_06::terminal::CreateTerminalConfigurationParamsTippingPln>,
    #[doc = "Tipping configuration for RON"]
    pub ron: Option<stripe_types_05::terminal::CreateTerminalConfigurationParamsTippingRon>,
    #[doc = "Tipping configuration for SEK"]
    pub sek: Option<stripe_types_06::terminal::CreateTerminalConfigurationParamsTippingSek>,
    #[doc = "Tipping configuration for SGD"]
    pub sgd: Option<stripe_types_05::terminal::CreateTerminalConfigurationParamsTippingSgd>,
    #[doc = "Tipping configuration for USD"]
    pub usd: Option<stripe_types_06::terminal::CreateTerminalConfigurationParamsTippingUsd>,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for CreateTerminalConfigurationParamsTipping {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("aed", self.aed.as_ref());
        form.field_opt("aud", self.aud.as_ref());
        form.field_opt("cad", self.cad.as_ref());
        form.field_opt("chf", self.chf.as_ref());
        form.field_opt("czk", self.czk.as_ref());
        form.field_opt("dkk", self.dkk.as_ref());
        form.field_opt("eur", self.eur.as_ref());
        form.field_opt("gbp", self.gbp.as_ref());
        form.field_opt("gip", self.gip.as_ref());
        form.field_opt("hkd", self.hkd.as_ref());
        form.field_opt("huf", self.huf.as_ref());
        form.field_opt("jpy", self.jpy.as_ref());
        form.field_opt("mxn", self.mxn.as_ref());
        form.field_opt("myr", self.myr.as_ref());
        form.field_opt("nok", self.nok.as_ref());
        form.field_opt("nzd", self.nzd.as_ref());
        form.field_opt("pln", self.pln.as_ref());
        form.field_opt("ron", self.ron.as_ref());
        form.field_opt("sek", self.sek.as_ref());
        form.field_opt("sgd", self.sgd.as_ref());
        form.field_opt("usd", self.usd.as_ref());
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone)]
pub enum GetTerminalLocationResponseExpanded {
    /// Expanded `terminal.location`.
    TerminalLocation(Box<stripe_types_06::terminal::TerminalLocation>),
    /// An object type not known to this version of the crate.
    Other(json::OwnedValue),
}

#[cfg(feature = "terminal")]
impl<'json> json::Decode<'json> for GetTerminalLocationResponseExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "terminal.location" => GetTerminalLocationResponseExpanded::TerminalLocation(Box::new(
                json::from_slice_owned(&json::to_vec(&value))?,
            )),
            _ => GetTerminalLocationResponseExpanded::Other(value),
        })
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, Default)]
pub struct ProcessPaymentIntentParamsProcessConfig {
    #[doc = "This field indicates whether this payment method can be shown again to its customer in a checkout flow."]
    pub allow_redisplay: Option<stripe_types_05::terminal::ProcessPaymentIntentParamsProcessConfigAllowRedisplay>,
    #[doc = "Enables cancel button on transaction screens."]
    pub enable_customer_cancellation: Option<bool>,
    #[doc = "The URL to redirect your customer back to after they authenticate or cancel their payment on the payment method's app..."]
    pub return_url: Option<String>,
    #[doc = "Override showing a tipping selection screen on this transaction."]
    pub skip_tipping: Option<bool>,
    #[doc = "Tipping configuration for this transaction."]
    pub tipping: Option<stripe_types_06::terminal::ProcessPaymentIntentParamsProcessConfigTipping>,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for ProcessPaymentIntentParamsProcessConfig {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("allow_redisplay", self.allow_redisplay.as_ref());
        form.field_opt("enable_customer_cancellation", self.enable_customer_cancellation.as_ref());
        form.field_opt("return_url", self.return_url.as_ref());
        form.field_opt("skip_tipping", self.skip_tipping.as_ref());
        form.field_opt("tipping", self.tipping.as_ref());
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone)]
pub struct ProcessSetupIntentParams {
    #[doc = "This field indicates whether this payment method can be shown again to its customer in a checkout flow."]
    pub allow_redisplay: stripe_types_05::terminal::ProcessSetupIntentParamsAllowRedisplay,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Configuration overrides for this setup, such as MOTO and customer cancellation settings."]
    pub process_config: Option<stripe_types_04::terminal::ProcessSetupIntentParamsProcessConfig>,
    #[doc = "The ID of the SetupIntent to process on the reader."]
    pub setup_intent: String,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for ProcessSetupIntentParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("allow_redisplay", &self.allow_redisplay);
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("process_config", self.process_config.as_ref());
        form.field("setup_intent", &self.setup_intent);
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, Default)]
pub struct RefundPaymentParams {
    #[doc = "A positive integer in __cents__ representing how much of this charge to refund."]
    pub amount: Option<i64>,
    #[doc = "ID of the Charge to refund."]
    pub charge: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "ID of the PaymentIntent to refund."]
    pub payment_intent: Option<String>,
    #[doc = "Boolean indicating whether the application fee should be refunded when refunding this charge."]
    pub refund_application_fee: Option<bool>,
    #[doc = "Configuration overrides for this refund, such as customer cancellation settings."]
    pub refund_payment_config: Option<stripe_types_06::terminal::RefundPaymentParamsRefundPaymentConfig>,
    #[doc = "Boolean indicating whether the transfer should be reversed when refunding this charge."]
    pub reverse_transfer: Option<bool>,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for RefundPaymentParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("charge", self.charge.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("payment_intent", self.payment_intent.as_ref());
        form.field_opt("refund_application_fee", self.refund_application_fee.as_ref());
        form.field_opt("refund_payment_config", self.refund_payment_config.as_ref());
        form.field_opt("reverse_transfer", self.reverse_transfer.as_ref());
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone)]
pub struct SetReaderDisplayParams {
    #[doc = "Cart details to display on the reader screen, including line items, amounts, and currency."]
    pub cart: Option<stripe_types_06::terminal::SetReaderDisplayParamsCart>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Type of information to display."]
    pub r#type: stripe_types_00::terminal::SetReaderDisplayParamsType,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for SetReaderDisplayParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("cart", self.cart.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig {
    #[doc = "A File ID representing an image to display on the reader"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<crate::Expandable<stripe_types_06::files::File>>,
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalOnboardingLink {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub link_options: Option<stripe_types_05::terminal::TerminalOnboardingLinkLinkOptions>,
    #[doc = "The type of link being generated."]
    #[json(default)]
    pub link_type: stripe_types_03::terminal::TerminalOnboardingLinkLinkType,
    #[json(default)]
    pub object: stripe_types_01::terminal::TerminalOnboardingLinkObject,
    #[doc = "Stripe account ID to generate the link for."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
    #[doc = "The link passed back to the user for their onboarding."]
    #[json(default)]
    pub redirect_url: String,
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalReaderReaderResourceSelection {
    #[doc = "List of possible choices to be selected"]
    #[json(default)]
    pub choices: Vec<stripe_types_06::terminal::TerminalReaderReaderResourceChoice>,
    #[doc = "The id of the selected choice"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The text of the selected choice"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalReaderReaderResourceSetReaderDisplayAction {
    #[doc = "Cart object to be displayed by the reader, including line items, amounts, and currency."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cart: Option<stripe_types_05::terminal::TerminalReaderReaderResourceCart>,
    #[doc = "Type of information to be displayed by the reader."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::terminal::TerminalReaderReaderResourceSetReaderDisplayActionType,
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone)]
pub struct UpdateTerminalConfigurationParamsWifi {
    #[doc = "Credentials for a WPA-Enterprise WiFi network using the EAP-PEAP authentication method."]
    pub enterprise_eap_peap: Option<stripe_types_06::terminal::UpdateTerminalConfigurationParamsWifiEnterpriseEapPeap>,
    #[doc = "Credentials for a WPA-Enterprise WiFi network using the EAP-TLS authentication method."]
    pub enterprise_eap_tls: Option<stripe_types_06::terminal::UpdateTerminalConfigurationParamsWifiEnterpriseEapTls>,
    #[doc = "Credentials for a WPA-Personal WiFi network."]
    pub personal_psk: Option<stripe_types_05::terminal::UpdateTerminalConfigurationParamsWifiPersonalPsk>,
    #[doc = "Security type of the WiFi network."]
    pub r#type: stripe_types_05::terminal::UpdateTerminalConfigurationParamsWifiType,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for UpdateTerminalConfigurationParamsWifi {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("enterprise_eap_peap", self.enterprise_eap_peap.as_ref());
        form.field_opt("enterprise_eap_tls", self.enterprise_eap_tls.as_ref());
        form.field_opt("personal_psk", self.personal_psk.as_ref());
        form.field("type", &self.r#type);
    }
}
