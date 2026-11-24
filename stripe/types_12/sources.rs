#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SourceMandateNotification {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<stripe_types_01::sources::SourceMandateNotificationAcssDebitData>,
    #[doc = "A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zer..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<stripe_types_00::sources::SourceMandateNotificationBacsDebitData>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::sources::SourceMandateNotificationObject,
    #[doc = "The reason of the mandate notification."]
    #[json(default)]
    pub reason: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<stripe_types_04::sources::SourceMandateNotificationSepaDebitData>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<stripe_types_10::sources::Source>,
    #[doc = "The status of the mandate notification."]
    #[json(default)]
    pub status: String,
    #[doc = "The type of source this mandate notification is attached to."]
    #[json(rename = "type", default)]
    pub r#type: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for SourceMandateNotification {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "sources")]
#[derive(Debug, Clone, Default)]
pub struct UpdateSourceParams {
    #[doc = "Amount associated with the source."]
    pub amount: Option<i64>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Information about a mandate possibility attached to a source object (generally for bank debits) as well as its accept..."]
    pub mandate: Option<stripe_types_10::sources::UpdateSourceParamsMandate>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Information about the owner of the payment instrument that may be used or required by particular source types."]
    pub owner: Option<stripe_types_08::sources::UpdateSourceParamsOwner>,
    #[doc = "Information about the items and shipping associated with the source."]
    pub source_order: Option<stripe_types_10::sources::UpdateSourceParamsSourceOrder>,
}

#[cfg(feature = "sources")]
impl crate::params::FormEncode for UpdateSourceParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("mandate", self.mandate.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("owner", self.owner.as_ref());
        form.field_opt("source_order", self.source_order.as_ref());
    }
}
