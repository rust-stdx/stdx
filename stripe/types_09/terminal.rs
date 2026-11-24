#[cfg(feature = "terminal")]
#[derive(Debug, Clone, Default)]
pub struct CreateTerminalLocationParams {
    #[doc = "The full address of the location."]
    pub address: Option<stripe_types_03::terminal::CreateTerminalLocationParamsAddress>,
    #[doc = "The Kana variation of the full address of the location (Japan only)."]
    pub address_kana: Option<stripe_types_02::terminal::CreateTerminalLocationParamsAddressKana>,
    #[doc = "The Kanji variation of the full address of the location (Japan only)."]
    pub address_kanji: Option<stripe_types_06::terminal::CreateTerminalLocationParamsAddressKanji>,
    #[doc = "The ID of a configuration that will be used to customize all readers in this location."]
    pub configuration_overrides: Option<String>,
    #[doc = "A name for the location."]
    pub display_name: Option<String>,
    #[doc = "The Kana variation of the name for the location (Japan only)."]
    pub display_name_kana: Option<String>,
    #[doc = "The Kanji variation of the name for the location (Japan only)."]
    pub display_name_kanji: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The phone number for the location."]
    pub phone: Option<String>,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for CreateTerminalLocationParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("address", self.address.as_ref());
        form.field_opt("address_kana", self.address_kana.as_ref());
        form.field_opt("address_kanji", self.address_kanji.as_ref());
        form.field_opt("configuration_overrides", self.configuration_overrides.as_ref());
        form.field_opt("display_name", self.display_name.as_ref());
        form.field_opt("display_name_kana", self.display_name_kana.as_ref());
        form.field_opt("display_name_kanji", self.display_name_kanji.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("phone", self.phone.as_ref());
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone)]
pub struct CreateTerminalOnboardingLinkParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Specific fields needed to generate the desired link type."]
    pub link_options: stripe_types_05::terminal::CreateTerminalOnboardingLinkParamsLinkOptions,
    #[doc = "The type of link being generated."]
    pub link_type: stripe_types_00::terminal::CreateTerminalOnboardingLinkParamsLinkType,
    #[doc = "Stripe account ID to generate the link for."]
    pub on_behalf_of: Option<String>,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for CreateTerminalOnboardingLinkParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("link_options", &self.link_options);
        form.field("link_type", &self.link_type);
        form.field_opt("on_behalf_of", self.on_behalf_of.as_ref());
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, Default)]
pub struct UpdateTerminalConfigurationParamsTipping {
    #[doc = "Tipping configuration for AED"]
    pub aed: Option<stripe_types_05::terminal::UpdateTerminalConfigurationParamsTippingAed>,
    #[doc = "Tipping configuration for AUD"]
    pub aud: Option<stripe_types_06::terminal::UpdateTerminalConfigurationParamsTippingAud>,
    #[doc = "Tipping configuration for CAD"]
    pub cad: Option<stripe_types_05::terminal::UpdateTerminalConfigurationParamsTippingCad>,
    #[doc = "Tipping configuration for CHF"]
    pub chf: Option<stripe_types_06::terminal::UpdateTerminalConfigurationParamsTippingChf>,
    #[doc = "Tipping configuration for CZK"]
    pub czk: Option<stripe_types_05::terminal::UpdateTerminalConfigurationParamsTippingCzk>,
    #[doc = "Tipping configuration for DKK"]
    pub dkk: Option<stripe_types_06::terminal::UpdateTerminalConfigurationParamsTippingDkk>,
    #[doc = "Tipping configuration for EUR"]
    pub eur: Option<stripe_types_05::terminal::UpdateTerminalConfigurationParamsTippingEur>,
    #[doc = "Tipping configuration for GBP"]
    pub gbp: Option<stripe_types_06::terminal::UpdateTerminalConfigurationParamsTippingGbp>,
    #[doc = "Tipping configuration for GIP"]
    pub gip: Option<stripe_types_05::terminal::UpdateTerminalConfigurationParamsTippingGip>,
    #[doc = "Tipping configuration for HKD"]
    pub hkd: Option<stripe_types_06::terminal::UpdateTerminalConfigurationParamsTippingHkd>,
    #[doc = "Tipping configuration for HUF"]
    pub huf: Option<stripe_types_05::terminal::UpdateTerminalConfigurationParamsTippingHuf>,
    #[doc = "Tipping configuration for JPY"]
    pub jpy: Option<stripe_types_06::terminal::UpdateTerminalConfigurationParamsTippingJpy>,
    #[doc = "Tipping configuration for MXN"]
    pub mxn: Option<stripe_types_05::terminal::UpdateTerminalConfigurationParamsTippingMxn>,
    #[doc = "Tipping configuration for MYR"]
    pub myr: Option<stripe_types_06::terminal::UpdateTerminalConfigurationParamsTippingMyr>,
    #[doc = "Tipping configuration for NOK"]
    pub nok: Option<stripe_types_05::terminal::UpdateTerminalConfigurationParamsTippingNok>,
    #[doc = "Tipping configuration for NZD"]
    pub nzd: Option<stripe_types_06::terminal::UpdateTerminalConfigurationParamsTippingNzd>,
    #[doc = "Tipping configuration for PLN"]
    pub pln: Option<stripe_types_05::terminal::UpdateTerminalConfigurationParamsTippingPln>,
    #[doc = "Tipping configuration for RON"]
    pub ron: Option<stripe_types_06::terminal::UpdateTerminalConfigurationParamsTippingRon>,
    #[doc = "Tipping configuration for SEK"]
    pub sek: Option<stripe_types_05::terminal::UpdateTerminalConfigurationParamsTippingSek>,
    #[doc = "Tipping configuration for SGD"]
    pub sgd: Option<stripe_types_06::terminal::UpdateTerminalConfigurationParamsTippingSgd>,
    #[doc = "Tipping configuration for USD"]
    pub usd: Option<stripe_types_05::terminal::UpdateTerminalConfigurationParamsTippingUsd>,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for UpdateTerminalConfigurationParamsTipping {
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
#[derive(Debug, Clone, Default)]
pub struct UpdateTerminalLocationParams {
    #[doc = "The full address of the location."]
    pub address: Option<stripe_types_00::terminal::UpdateTerminalLocationParamsAddress>,
    #[doc = "The Kana variation of the full address of the location (Japan only)."]
    pub address_kana: Option<stripe_types_05::terminal::UpdateTerminalLocationParamsAddressKana>,
    #[doc = "The Kanji variation of the full address of the location (Japan only)."]
    pub address_kanji: Option<stripe_types_05::terminal::UpdateTerminalLocationParamsAddressKanji>,
    #[doc = "The ID of a configuration that will be used to customize all readers in this location."]
    pub configuration_overrides: Option<json::OwnedValue>,
    #[doc = "A name for the location."]
    pub display_name: Option<json::OwnedValue>,
    #[doc = "The Kana variation of the name for the location (Japan only)."]
    pub display_name_kana: Option<json::OwnedValue>,
    #[doc = "The Kanji variation of the name for the location (Japan only)."]
    pub display_name_kanji: Option<json::OwnedValue>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The phone number for the location."]
    pub phone: Option<json::OwnedValue>,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for UpdateTerminalLocationParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("address", self.address.as_ref());
        form.field_opt("address_kana", self.address_kana.as_ref());
        form.field_opt("address_kanji", self.address_kanji.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
    }
}
