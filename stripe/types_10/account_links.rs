#[cfg(feature = "account_links")]
#[derive(Debug, Clone)]
pub struct CreateAccountLinkParams {
    #[doc = "The identifier of the account to create an account link for."]
    pub account: String,
    #[doc = "The collect parameter is deprecated."]
    pub collect: Option<stripe_types_00::account_links::CreateAccountLinkParamsCollect>,
    #[doc = "Specifies the requirements that Stripe collects from connected accounts in the Connect Onboarding flow."]
    pub collection_options: Option<stripe_types_08::account_links::CreateAccountLinkParamsCollectionOptions>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The URL the user will be redirected to if the account link is expired, has been previously-visited, or is otherwise i..."]
    pub refresh_url: Option<String>,
    #[doc = "The URL that the user will be redirected to upon leaving or completing the linked flow."]
    pub return_url: Option<String>,
    #[doc = "The type of account link the user is requesting."]
    pub r#type: stripe_types_00::account_links::CreateAccountLinkParamsType,
}

#[cfg(feature = "account_links")]
impl crate::params::FormEncode for CreateAccountLinkParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("account", &self.account);
        form.field_opt("collect", self.collect.as_ref());
        form.field_opt("collection_options", self.collection_options.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("refresh_url", self.refresh_url.as_ref());
        form.field_opt("return_url", self.return_url.as_ref());
        form.field("type", &self.r#type);
    }
}
