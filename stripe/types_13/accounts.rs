#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct UpdateAccountParams {
    #[doc = "An [account token](https://docs.stripe.com/api#create_account_token), used to securely provide details to the account."]
    pub account_token: Option<String>,
    #[doc = "Business information about the account."]
    pub business_profile: Option<stripe_types_07::accounts::UpdateAccountParamsBusinessProfile>,
    #[doc = "The business type."]
    pub business_type: Option<stripe_types_06::accounts::UpdateAccountParamsBusinessType>,
    #[doc = "Each key of the dictionary represents a capability, and each capability maps to its settings (for example, whether it..."]
    pub capabilities: Option<stripe_types_08::accounts::UpdateAccountParamsCapabilities>,
    #[doc = "Information about the company or business."]
    pub company: Option<stripe_types_11::accounts::UpdateAccountParamsCompany>,
    #[doc = "Three-letter ISO currency code representing the default currency for the account."]
    pub default_currency: Option<String>,
    #[doc = "Documents that may be submitted to satisfy various informational requests."]
    pub documents: Option<stripe_types_10::accounts::UpdateAccountParamsDocuments>,
    #[doc = "The email address of the account holder."]
    pub email: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "A card or bank account to attach to the account for receiving [payouts](/connect/bank-debit-card-payouts) (you won’t ..."]
    pub external_account: Option<String>,
    #[doc = "A hash of account group type to tokens."]
    pub groups: Option<stripe_types_05::accounts::UpdateAccountParamsGroups>,
    #[doc = "Information about the person represented by the account."]
    pub individual: Option<stripe_types_10::accounts::UpdateAccountParamsIndividual>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Options for customizing how the account functions within Stripe."]
    pub settings: Option<stripe_types_12::accounts::UpdateAccountParamsSettings>,
    #[doc = "Details on the account's acceptance of the [Stripe Services Agreement](/connect/updating-accounts#tos-acceptance)."]
    pub tos_acceptance: Option<stripe_types_05::accounts::UpdateAccountParamsTosAcceptance>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for UpdateAccountParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account_token", self.account_token.as_ref());
        form.field_opt("business_profile", self.business_profile.as_ref());
        form.field_opt("business_type", self.business_type.as_ref());
        form.field_opt("capabilities", self.capabilities.as_ref());
        form.field_opt("company", self.company.as_ref());
        form.field_opt("default_currency", self.default_currency.as_ref());
        form.field_opt("documents", self.documents.as_ref());
        form.field_opt("email", self.email.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("external_account", self.external_account.as_ref());
        form.field_opt("groups", self.groups.as_ref());
        form.field_opt("individual", self.individual.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("settings", self.settings.as_ref());
        form.field_opt("tos_acceptance", self.tos_acceptance.as_ref());
    }
}
