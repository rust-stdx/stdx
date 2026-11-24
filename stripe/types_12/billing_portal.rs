#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, Default)]
pub struct CreateBillingPortalConfigurationParamsFeatures {
    #[doc = "Information about updating the customer details in the portal."]
    pub customer_update:
        Option<stripe_types_00::billing_portal::CreateBillingPortalConfigurationParamsFeaturesCustomerUpdate>,
    #[doc = "Information about showing the billing history in the portal."]
    pub invoice_history:
        Option<stripe_types_04::billing_portal::CreateBillingPortalConfigurationParamsFeaturesInvoiceHistory>,
    #[doc = "Information about updating payment methods in the portal."]
    pub payment_method_update:
        Option<stripe_types_03::billing_portal::CreateBillingPortalConfigurationParamsFeaturesPaymentMethodUpdate>,
    #[doc = "Information about canceling subscriptions in the portal."]
    pub subscription_cancel:
        Option<stripe_types_05::billing_portal::CreateBillingPortalConfigurationParamsFeaturesSubscriptionCancel>,
    #[doc = "Information about updating subscriptions in the portal."]
    pub subscription_update:
        Option<stripe_types_11::billing_portal::CreateBillingPortalConfigurationParamsFeaturesSubscriptionUpdate>,
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for CreateBillingPortalConfigurationParamsFeatures {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("customer_update", self.customer_update.as_ref());
        form.field_opt("invoice_history", self.invoice_history.as_ref());
        form.field_opt("payment_method_update", self.payment_method_update.as_ref());
        form.field_opt("subscription_cancel", self.subscription_cancel.as_ref());
        form.field_opt("subscription_update", self.subscription_update.as_ref());
    }
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, Default)]
pub struct CreateBillingPortalSessionParams {
    #[doc = "The ID of an existing [configuration](https://docs.stripe.com/api/customer_portal/configurations) to use for this ses..."]
    pub configuration: Option<String>,
    #[doc = "The ID of an existing customer."]
    pub customer: Option<String>,
    #[doc = "The ID of an existing account."]
    pub customer_account: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Information about a specific flow for the customer to go through."]
    pub flow_data: Option<stripe_types_11::billing_portal::CreateBillingPortalSessionParamsFlowData>,
    #[doc = "The IETF language tag of the locale customer portal is displayed in."]
    pub locale: Option<stripe_types_02::billing_portal::CreateBillingPortalSessionParamsLocale>,
    #[doc = "The `on_behalf_of` account to use for this session."]
    pub on_behalf_of: Option<String>,
    #[doc = "The default URL to redirect customers to when they click on the portal's link to return to your website."]
    pub return_url: Option<String>,
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for CreateBillingPortalSessionParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("configuration", self.configuration.as_ref());
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("customer_account", self.customer_account.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("flow_data", self.flow_data.as_ref());
        form.field_opt("locale", self.locale.as_ref());
        form.field_opt("on_behalf_of", self.on_behalf_of.as_ref());
        form.field_opt("return_url", self.return_url.as_ref());
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PortalFeatures {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_update: Option<stripe_types_05::billing_portal::PortalCustomerUpdate>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub invoice_history: Option<stripe_types_03::billing_portal::PortalInvoiceList>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_update: Option<stripe_types_01::billing_portal::PortalPaymentMethodUpdate>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub subscription_cancel: Option<stripe_types_11::billing_portal::PortalSubscriptionCancel>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub subscription_update: Option<stripe_types_11::billing_portal::PortalSubscriptionUpdate>,
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone, Default)]
pub struct UpdateBillingPortalConfigurationParams {
    #[doc = "Whether the configuration is active and can be used to create portal sessions."]
    pub active: Option<bool>,
    #[doc = "The business information shown to customers in the portal."]
    pub business_profile:
        Option<stripe_types_03::billing_portal::UpdateBillingPortalConfigurationParamsBusinessProfile>,
    #[doc = "The default URL to redirect customers to when they click on the portal's link to return to your website."]
    pub default_return_url: Option<json::OwnedValue>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Information about the features available in the portal."]
    pub features: Option<stripe_types_11::billing_portal::UpdateBillingPortalConfigurationParamsFeatures>,
    #[doc = "The hosted login page for this configuration."]
    pub login_page: Option<stripe_types_06::billing_portal::UpdateBillingPortalConfigurationParamsLoginPage>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The name of the configuration."]
    pub name: Option<json::OwnedValue>,
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for UpdateBillingPortalConfigurationParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("active", self.active.as_ref());
        form.field_opt("business_profile", self.business_profile.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("features", self.features.as_ref());
        form.field_opt("login_page", self.login_page.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
    }
}
