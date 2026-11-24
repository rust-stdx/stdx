#[cfg(feature = "sources")]
#[derive(Debug, Clone, Default)]
pub struct CreateSourceParams {
    #[doc = "Amount associated with the source."]
    pub amount: Option<i64>,
    #[doc = "Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) associated with the source."]
    pub currency: Option<String>,
    #[doc = "The `Customer` to whom the original source is attached to."]
    pub customer: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The authentication `flow` of the source to create."]
    pub flow: Option<stripe_types_03::sources::CreateSourceParamsFlow>,
    #[doc = "Information about a mandate possibility attached to a source object (generally for bank debits) as well as its accept..."]
    pub mandate: Option<stripe_types_08::sources::CreateSourceParamsMandate>,
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The source to share."]
    pub original_source: Option<String>,
    #[doc = "Information about the owner of the payment instrument that may be used or required by particular source types."]
    pub owner: Option<stripe_types_06::sources::CreateSourceParamsOwner>,
    #[doc = "Optional parameters for the receiver flow."]
    pub receiver: Option<stripe_types_06::sources::CreateSourceParamsReceiver>,
    #[doc = "Parameters required for the redirect flow."]
    pub redirect: Option<stripe_types_03::sources::CreateSourceParamsRedirect>,
    #[doc = "Information about the items and shipping associated with the source."]
    pub source_order: Option<stripe_types_09::sources::CreateSourceParamsSourceOrder>,
    #[doc = "An arbitrary string to be displayed on your customer's statement."]
    pub statement_descriptor: Option<String>,
    #[doc = "An optional token used to create the source."]
    pub token: Option<String>,
    #[doc = "The `type` of the source to create."]
    pub r#type: Option<String>,
    pub usage: Option<stripe_types_02::sources::CreateSourceParamsUsage>,
}

#[cfg(feature = "sources")]
impl crate::params::FormEncode for CreateSourceParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("currency", self.currency.as_ref());
        form.field_opt("customer", self.customer.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("flow", self.flow.as_ref());
        form.field_opt("mandate", self.mandate.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("original_source", self.original_source.as_ref());
        form.field_opt("owner", self.owner.as_ref());
        form.field_opt("receiver", self.receiver.as_ref());
        form.field_opt("redirect", self.redirect.as_ref());
        form.field_opt("source_order", self.source_order.as_ref());
        form.field_opt("statement_descriptor", self.statement_descriptor.as_ref());
        form.field_opt("token", self.token.as_ref());
        form.field_opt("type", self.r#type.as_ref());
        form.field_opt("usage", self.usage.as_ref());
    }
}
