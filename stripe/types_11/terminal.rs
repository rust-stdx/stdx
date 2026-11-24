#[cfg(feature = "terminal")]
#[derive(Debug, Clone, Default)]
pub struct CreateTerminalConfigurationParams {
    #[doc = "An object containing device type specific settings for BBPOS WisePad 3 readers."]
    pub bbpos_wisepad3: Option<stripe_types_02::terminal::CreateTerminalConfigurationParamsBbposWisepad3>,
    #[doc = "An object containing device type specific settings for BBPOS WisePOS E readers."]
    pub bbpos_wisepos_e: Option<stripe_types_05::terminal::CreateTerminalConfigurationParamsBbposWiseposE>,
    #[doc = "Configuration for cellular connectivity."]
    pub cellular: Option<stripe_types_01::terminal::CreateTerminalConfigurationParamsCellular>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Name of the configuration"]
    pub name: Option<String>,
    #[doc = "Configurations for collecting transactions offline."]
    pub offline: Option<stripe_types_05::terminal::CreateTerminalConfigurationParamsOffline>,
    #[doc = "Reboot time settings for readers."]
    pub reboot_window: Option<stripe_types_01::terminal::CreateTerminalConfigurationParamsRebootWindow>,
    #[doc = "An object containing device type specific settings for Stripe S700 readers."]
    pub stripe_s700: Option<stripe_types_05::terminal::CreateTerminalConfigurationParamsStripeS700>,
    #[doc = "An object containing device type specific settings for Stripe S710 readers."]
    pub stripe_s710: Option<stripe_types_06::terminal::CreateTerminalConfigurationParamsStripeS710>,
    #[doc = "Tipping configurations for readers that support on-reader tips."]
    pub tipping: Option<stripe_types_08::terminal::CreateTerminalConfigurationParamsTipping>,
    #[doc = "An object containing device type specific settings for Verifone M425 readers."]
    pub verifone_m425: Option<stripe_types_06::terminal::CreateTerminalConfigurationParamsVerifoneM425>,
    #[doc = "An object containing device type specific settings for Verifone P400 readers."]
    pub verifone_p400: Option<stripe_types_05::terminal::CreateTerminalConfigurationParamsVerifoneP400>,
    #[doc = "An object containing device type specific settings for Verifone P630 readers."]
    pub verifone_p630: Option<stripe_types_06::terminal::CreateTerminalConfigurationParamsVerifoneP630>,
    #[doc = "An object containing device type specific settings for Verifone UX700 readers."]
    pub verifone_ux700: Option<stripe_types_06::terminal::CreateTerminalConfigurationParamsVerifoneUx700>,
    #[doc = "An object containing device type specific settings for Verifone V660p readers."]
    pub verifone_v660p: Option<stripe_types_05::terminal::CreateTerminalConfigurationParamsVerifoneV660p>,
    #[doc = "Configurations for connecting to a WiFi network."]
    pub wifi: Option<stripe_types_05::terminal::CreateTerminalConfigurationParamsWifi>,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for CreateTerminalConfigurationParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("bbpos_wisepad3", self.bbpos_wisepad3.as_ref());
        form.field_opt("bbpos_wisepos_e", self.bbpos_wisepos_e.as_ref());
        form.field_opt("cellular", self.cellular.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("name", self.name.as_ref());
        form.field_opt("offline", self.offline.as_ref());
        form.field_opt("reboot_window", self.reboot_window.as_ref());
        form.field_opt("stripe_s700", self.stripe_s700.as_ref());
        form.field_opt("stripe_s710", self.stripe_s710.as_ref());
        form.field_opt("tipping", self.tipping.as_ref());
        form.field_opt("verifone_m425", self.verifone_m425.as_ref());
        form.field_opt("verifone_p400", self.verifone_p400.as_ref());
        form.field_opt("verifone_p630", self.verifone_p630.as_ref());
        form.field_opt("verifone_ux700", self.verifone_ux700.as_ref());
        form.field_opt("verifone_v660p", self.verifone_v660p.as_ref());
        form.field_opt("wifi", self.wifi.as_ref());
    }
}
