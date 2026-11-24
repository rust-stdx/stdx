#[cfg(feature = "terminal")]
#[derive(Debug, Clone)]
pub struct CollectInputsParamsInputsItem {
    #[doc = "Customize the text which will be displayed while collecting this input"]
    pub custom_text: stripe_types_02::terminal::CollectInputsParamsInputsItemCustomText,
    #[doc = "Indicate that this input is required, disabling the skip button"]
    pub required: Option<bool>,
    #[doc = "Options for the `selection` input"]
    pub selection: Option<stripe_types_07::terminal::CollectInputsParamsInputsItemSelection>,
    #[doc = "List of toggles to be displayed and customization for the toggles"]
    pub toggles: Option<Vec<stripe_types_06::terminal::CollectInputsParamsInputsItemTogglesItem>>,
    #[doc = "The type of input to collect"]
    pub r#type: stripe_types_01::terminal::CollectInputsParamsInputsItemType,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for CollectInputsParamsInputsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("custom_text", &self.custom_text);
        form.field_opt("required", self.required.as_ref());
        form.field_opt("selection", self.selection.as_ref());
        if let Some(values) = self.toggles.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("toggles", index, item);
            }
        }
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, Default)]
pub struct ProcessPaymentIntentParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The ID of the PaymentIntent to process on the reader."]
    pub payment_intent: String,
    #[doc = "Configuration overrides for this transaction, such as tipping and customer cancellation settings."]
    pub process_config: Option<stripe_types_08::terminal::ProcessPaymentIntentParamsProcessConfig>,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for ProcessPaymentIntentParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("payment_intent", &self.payment_intent);
        form.field_opt("process_config", self.process_config.as_ref());
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalConfiguration {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bbpos_wisepad3:
        Option<stripe_types_08::terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bbpos_wisepos_e:
        Option<stripe_types_08::terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub cellular: Option<stripe_types_01::terminal::TerminalConfigurationConfigurationResourceCellularConfig>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "Whether this Configuration is the default for your account"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub is_account_default: Option<bool>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String indicating the name of the Configuration object, set by the user"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::terminal::TerminalConfigurationObject,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub offline: Option<stripe_types_03::terminal::TerminalConfigurationConfigurationResourceOfflineConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reboot_window: Option<stripe_types_00::terminal::TerminalConfigurationConfigurationResourceRebootWindow>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub stripe_s700:
        Option<stripe_types_08::terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub stripe_s710:
        Option<stripe_types_08::terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tipping: Option<stripe_types_05::terminal::TerminalConfigurationConfigurationResourceTipping>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verifone_m425:
        Option<stripe_types_08::terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verifone_p400:
        Option<stripe_types_08::terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verifone_p630:
        Option<stripe_types_08::terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verifone_ux700:
        Option<stripe_types_08::terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub verifone_v660p:
        Option<stripe_types_08::terminal::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub wifi: Option<stripe_types_06::terminal::TerminalConfigurationConfigurationResourceWifiConfig>,
}

#[cfg(feature = "terminal")]
impl crate::PagedItem for TerminalConfiguration {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalReaderReaderResourceInput {
    #[doc = "Default text of input being collected."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub custom_text: Option<stripe_types_04::terminal::TerminalReaderReaderResourceCustomText>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<stripe_types_01::terminal::TerminalReaderReaderResourceEmail>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub numeric: Option<stripe_types_02::terminal::TerminalReaderReaderResourceNumeric>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<stripe_types_04::terminal::TerminalReaderReaderResourcePhone>,
    #[doc = "Indicate that this input is required, disabling the skip button."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub selection: Option<stripe_types_08::terminal::TerminalReaderReaderResourceSelection>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<stripe_types_04::terminal::TerminalReaderReaderResourceSignature>,
    #[doc = "Indicate that this input was skipped by the user."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub skipped: Option<bool>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<stripe_types_03::terminal::TerminalReaderReaderResourceText>,
    #[doc = "List of toggles being collected."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub toggles: Option<Vec<stripe_types_06::terminal::TerminalReaderReaderResourceToggle>>,
    #[doc = "Type of input being collected."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::terminal::TerminalReaderReaderResourceInputType,
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, Default)]
pub struct UpdateTerminalConfigurationParams {
    #[doc = "An object containing device type specific settings for BBPOS WisePad 3 readers."]
    pub bbpos_wisepad3: Option<stripe_types_05::terminal::UpdateTerminalConfigurationParamsBbposWisepad3>,
    #[doc = "An object containing device type specific settings for BBPOS WisePOS E readers."]
    pub bbpos_wisepos_e: Option<stripe_types_06::terminal::UpdateTerminalConfigurationParamsBbposWiseposE>,
    #[doc = "Configuration for cellular connectivity."]
    pub cellular: Option<stripe_types_06::terminal::UpdateTerminalConfigurationParamsCellular>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Name of the configuration"]
    pub name: Option<String>,
    #[doc = "Configurations for collecting transactions offline."]
    pub offline: Option<stripe_types_06::terminal::UpdateTerminalConfigurationParamsOffline>,
    #[doc = "Reboot time settings for readers."]
    pub reboot_window: Option<stripe_types_06::terminal::UpdateTerminalConfigurationParamsRebootWindow>,
    #[doc = "An object containing device type specific settings for Stripe S700 readers."]
    pub stripe_s700: Option<stripe_types_05::terminal::UpdateTerminalConfigurationParamsStripeS700>,
    #[doc = "An object containing device type specific settings for Stripe S710 readers."]
    pub stripe_s710: Option<stripe_types_06::terminal::UpdateTerminalConfigurationParamsStripeS710>,
    #[doc = "Tipping configurations for readers that support on-reader tips."]
    pub tipping: Option<stripe_types_09::terminal::UpdateTerminalConfigurationParamsTipping>,
    #[doc = "An object containing device type specific settings for Verifone M425 readers."]
    pub verifone_m425: Option<stripe_types_06::terminal::UpdateTerminalConfigurationParamsVerifoneM425>,
    #[doc = "An object containing device type specific settings for Verifone P400 readers."]
    pub verifone_p400: Option<stripe_types_05::terminal::UpdateTerminalConfigurationParamsVerifoneP400>,
    #[doc = "An object containing device type specific settings for Verifone P630 readers."]
    pub verifone_p630: Option<stripe_types_06::terminal::UpdateTerminalConfigurationParamsVerifoneP630>,
    #[doc = "An object containing device type specific settings for Verifone UX700 readers."]
    pub verifone_ux700: Option<stripe_types_05::terminal::UpdateTerminalConfigurationParamsVerifoneUx700>,
    #[doc = "An object containing device type specific settings for Verifone V660p readers."]
    pub verifone_v660p: Option<stripe_types_06::terminal::UpdateTerminalConfigurationParamsVerifoneV660p>,
    #[doc = "Configurations for connecting to a WiFi network."]
    pub wifi: Option<stripe_types_08::terminal::UpdateTerminalConfigurationParamsWifi>,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for UpdateTerminalConfigurationParams {
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
