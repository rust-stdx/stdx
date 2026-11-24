#[cfg(feature = "terminal")]
pub type CancelActionParams = stripe_types_02::terminal::GetTerminalConfigurationParams;

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, Default)]
pub struct CollectInputsParamsInputsItemSelectionChoicesItem {
    #[doc = "The unique identifier for this choice"]
    pub id: String,
    #[doc = "The style of the button which will be shown for this choice."]
    pub style: Option<stripe_types_03::terminal::CollectInputsParamsInputsItemSelectionChoicesItemStyle>,
    #[doc = "The text which will be shown on the button for this choice"]
    pub text: String,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for CollectInputsParamsInputsItemSelectionChoicesItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("id", &self.id);
        form.field_opt("style", self.style.as_ref());
        form.field("text", &self.text);
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, Default)]
pub struct CollectInputsParamsInputsItemTogglesItem {
    #[doc = "The default value of the toggle."]
    pub default_value: Option<stripe_types_00::terminal::CollectInputsParamsInputsItemTogglesItemDefaultValue>,
    #[doc = "The description which will be displayed for the toggle."]
    pub description: Option<String>,
    #[doc = "The title which will be displayed for the toggle."]
    pub title: Option<String>,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for CollectInputsParamsInputsItemTogglesItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("default_value", self.default_value.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field_opt("title", self.title.as_ref());
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, Default)]
pub struct ConfirmPaymentIntentParams {
    #[doc = "Configuration overrides for this confirmation, such as surcharge settings and return URL."]
    pub confirm_config: Option<stripe_types_04::terminal::ConfirmPaymentIntentParamsConfirmConfig>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The ID of the PaymentIntent to confirm."]
    pub payment_intent: String,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for ConfirmPaymentIntentParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("confirm_config", self.confirm_config.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("payment_intent", &self.payment_intent);
    }
}

#[cfg(feature = "terminal")]
pub type CreateTerminalConfigurationParamsStripeS710 =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsBbposWisepad3;

#[cfg(feature = "terminal")]
pub type CreateTerminalConfigurationParamsTippingCad =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type CreateTerminalConfigurationParamsTippingCzk =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type CreateTerminalConfigurationParamsTippingEur =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type CreateTerminalConfigurationParamsTippingGip =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type CreateTerminalConfigurationParamsTippingHuf =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type CreateTerminalConfigurationParamsTippingMxn =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type CreateTerminalConfigurationParamsTippingNok =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type CreateTerminalConfigurationParamsTippingPln =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type CreateTerminalConfigurationParamsTippingSek =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type CreateTerminalConfigurationParamsTippingUsd =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type CreateTerminalConfigurationParamsVerifoneM425 =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsBbposWisepad3;

#[cfg(feature = "terminal")]
pub type CreateTerminalConfigurationParamsVerifoneP630 =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsBbposWisepad3;

#[cfg(feature = "terminal")]
pub type CreateTerminalConfigurationParamsVerifoneUx700 =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsBbposWisepad3;

#[cfg(feature = "terminal")]
pub type CreateTerminalLocationParamsAddressKanji = stripe_types_02::terminal::CreateTerminalLocationParamsAddressKana;

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct DeletedTerminalConfiguration {
    #[doc = "Always true for a deleted object"]
    #[json(default)]
    pub deleted: bool,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::terminal::TerminalConfigurationObject,
}

#[cfg(feature = "terminal")]
impl crate::PagedItem for DeletedTerminalConfiguration {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct DeletedTerminalLocation {
    #[doc = "Always true for a deleted object"]
    #[json(default)]
    pub deleted: bool,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_02::terminal::TerminalLocationObject,
}

#[cfg(feature = "terminal")]
impl crate::PagedItem for DeletedTerminalLocation {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct DeletedTerminalReader {
    #[doc = "Always true for a deleted object"]
    #[json(default)]
    pub deleted: bool,
    #[doc = "Device type of the reader."]
    #[json(default)]
    pub device_type: stripe_types_00::terminal::TerminalReaderDeviceType,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_02::terminal::TerminalReaderObject,
    #[doc = "Serial number of the reader."]
    #[json(default)]
    pub serial_number: String,
}

#[cfg(feature = "terminal")]
impl crate::PagedItem for DeletedTerminalReader {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "terminal")]
pub type GetTerminalLocationParams = stripe_types_02::terminal::GetTerminalConfigurationParams;

#[cfg(feature = "terminal")]
pub type GetTerminalReaderParams = stripe_types_02::terminal::GetTerminalConfigurationParams;

#[cfg(feature = "terminal")]
pub type ProcessPaymentIntentParamsProcessConfigTipping =
    stripe_types_02::terminal::CollectPaymentMethodParamsCollectConfigTipping;

#[cfg(feature = "terminal")]
pub type RefundPaymentParamsRefundPaymentConfig = stripe_types_04::terminal::ProcessSetupIntentParamsProcessConfig;

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, Default)]
pub struct SetReaderDisplayParamsCart {
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "Array of line items to display."]
    pub line_items: Vec<stripe_types_00::terminal::SetReaderDisplayParamsCartLineItemsItem>,
    #[doc = "The amount of tax in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal)."]
    pub tax: Option<i64>,
    #[doc = "Total balance of cart due in the [smallest currency unit](https://docs.stripe.com/currencies#zero-decimal)."]
    pub total: i64,
}

#[cfg(feature = "terminal")]
impl crate::params::FormEncode for SetReaderDisplayParamsCart {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("currency", &self.currency);
        for (index, item) in self.line_items.iter().enumerate() {
            form.field_indexed("line_items", index, item);
        }
        form.field_opt("tax", self.tax.as_ref());
        form.field("total", &self.total);
    }
}

#[cfg(feature = "terminal")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalConfigurationConfigurationResourceWifiConfig {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub enterprise_eap_peap:
        Option<stripe_types_00::terminal::TerminalConfigurationConfigurationResourceEnterprisePeapWifi>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub enterprise_eap_tls:
        Option<stripe_types_02::terminal::TerminalConfigurationConfigurationResourceEnterpriseTlsWifi>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub personal_psk: Option<stripe_types_01::terminal::TerminalConfigurationConfigurationResourcePersonalPskWifi>,
    #[doc = "Security type of the WiFi network."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::terminal::TerminalConfigurationConfigurationResourceWifiConfigType,
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalLocation {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_04::terminal::AddressApiResourceTerminal>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<stripe_types_04::accounts::LegalEntityJapanAddress>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<stripe_types_04::accounts::LegalEntityJapanAddress>,
    #[doc = "The ID of a configuration that will be used to customize all readers in this location."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub configuration_overrides: Option<String>,
    #[doc = "The display name of the location."]
    #[json(default)]
    pub display_name: String,
    #[doc = "The Kana variation of the display name of the location."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub display_name_kana: Option<String>,
    #[doc = "The Kanji variation of the display name of the location."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub display_name_kanji: Option<String>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_02::terminal::TerminalLocationObject,
    #[doc = "The phone number of the location."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
impl crate::PagedItem for TerminalLocation {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalReaderReaderResourceChoice {
    #[doc = "The identifier for the selected choice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The button style for the choice."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<stripe_types_03::terminal::TerminalReaderReaderResourceChoiceStyle>,
    #[doc = "The text to be selected."]
    #[json(default)]
    pub text: String,
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalReaderReaderResourceProcessConfig {
    #[doc = "Enable customer-initiated cancellation when processing this payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub enable_customer_cancellation: Option<bool>,
    #[doc = "If the customer doesn't abandon authenticating the payment, they're redirected to this URL after completion."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[doc = "Override showing a tipping selection screen on this transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub skip_tipping: Option<bool>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tipping: Option<stripe_types_04::terminal::TerminalReaderReaderResourceTippingConfig>,
}

#[cfg(any(feature = "events", feature = "terminal", feature = "test_helpers"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalReaderReaderResourceToggle {
    #[doc = "The toggle's default value."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<stripe_types_00::terminal::TerminalReaderReaderResourceToggleDefaultValue>,
    #[doc = "The toggle's description text."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The toggle's title text."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The toggle's collected value."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<stripe_types_00::terminal::TerminalReaderReaderResourceToggleDefaultValue>,
}

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsBbposWiseposE =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsBbposWisepad3;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsCellular =
    stripe_types_01::terminal::CreateTerminalConfigurationParamsCellular;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsOffline =
    stripe_types_01::terminal::CreateTerminalConfigurationParamsCellular;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsRebootWindow =
    stripe_types_01::terminal::CreateTerminalConfigurationParamsRebootWindow;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsStripeS710 =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsBbposWisepad3;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsTippingAud =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsTippingChf =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsTippingDkk =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsTippingGbp =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsTippingHkd =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsTippingJpy =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsTippingMyr =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsTippingNzd =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsTippingRon =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsTippingSgd =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsTippingAed;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsVerifoneM425 =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsBbposWisepad3;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsVerifoneP630 =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsBbposWisepad3;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsVerifoneV660p =
    stripe_types_02::terminal::CreateTerminalConfigurationParamsBbposWisepad3;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsWifiEnterpriseEapPeap =
    stripe_types_03::terminal::CreateTerminalConfigurationParamsWifiEnterpriseEapPeap;

#[cfg(feature = "terminal")]
pub type UpdateTerminalConfigurationParamsWifiEnterpriseEapTls =
    stripe_types_01::terminal::CreateTerminalConfigurationParamsWifiEnterpriseEapTls;
