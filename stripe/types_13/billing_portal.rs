#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BillingPortalConfiguration {
    #[doc = "Whether the configuration is active and can be used to create portal sessions."]
    #[json(default)]
    pub active: bool,
    #[doc = "ID of the Connect Application that created the configuration."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub application:
        Option<crate::Expandable<stripe_types_09::billing_portal::BillingPortalConfigurationApplicationExpanded>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub business_profile: Option<stripe_types_00::billing_portal::PortalBusinessProfile>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "The default URL to redirect customers to when they click on the portal's link to return to your website."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub default_return_url: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<stripe_types_12::billing_portal::PortalFeatures>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "Whether the configuration is the default."]
    #[json(default)]
    pub is_default: bool,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub login_page: Option<stripe_types_00::billing_portal::PortalLoginPage>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The name of the configuration."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_01::billing_portal::BillingPortalConfigurationObject,
    #[doc = "Time at which the object was last updated."]
    #[json(default)]
    pub updated: i64,
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl crate::PagedItem for BillingPortalConfiguration {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BillingPortalSession {
    #[doc = "The configuration used by this session, describing the features available."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<crate::Expandable<crate::billing_portal::BillingPortalConfiguration>>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "The ID of the customer for this session."]
    #[json(default)]
    pub customer: String,
    #[doc = "The ID of the account for this session."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_account: Option<String>,
    #[doc = "Information about a specific flow for the customer to go through."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub flow: Option<stripe_types_11::billing_portal::PortalFlowsFlow>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "The IETF language tag of the locale Customer Portal is displayed in."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<stripe_types_00::billing_portal::BillingPortalSessionLocale>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::billing_portal::BillingPortalSessionObject,
    #[doc = "The account for which the session was created on behalf of."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
    #[doc = "The URL to redirect customers to when they click on the portal's link to return to your website."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[doc = "The short-lived URL of the session that gives customers access to the customer portal."]
    #[json(default)]
    pub url: String,
}

#[cfg(any(feature = "billing_portal", feature = "events"))]
impl crate::PagedItem for BillingPortalSession {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "billing_portal")]
#[derive(Debug, Clone)]
pub struct CreateBillingPortalConfigurationParams {
    #[doc = "The business information shown to customers in the portal."]
    pub business_profile:
        Option<stripe_types_00::billing_portal::CreateBillingPortalConfigurationParamsBusinessProfile>,
    #[doc = "The default URL to redirect customers to when they click on the portal's link to return to your website."]
    pub default_return_url: Option<json::OwnedValue>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Information about the features available in the portal."]
    pub features: stripe_types_12::billing_portal::CreateBillingPortalConfigurationParamsFeatures,
    #[doc = "The hosted login page for this configuration."]
    pub login_page: Option<stripe_types_05::billing_portal::CreateBillingPortalConfigurationParamsLoginPage>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The name of the configuration."]
    pub name: Option<json::OwnedValue>,
}

#[cfg(feature = "billing_portal")]
impl crate::params::FormEncode for CreateBillingPortalConfigurationParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("business_profile", self.business_profile.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("features", &self.features);
        form.field_opt("login_page", self.login_page.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
    }
}
