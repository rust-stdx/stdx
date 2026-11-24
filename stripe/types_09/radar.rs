#[cfg(feature = "radar")]
#[derive(Debug, Clone, Default)]
pub struct CreateRadarPaymentEvaluationParamsPaymentDetailsShippingDetails {
    #[doc = "Shipping address."]
    pub address: Option<stripe_types_06::radar::CreateRadarPaymentEvaluationParamsPaymentDetailsShippingDetailsAddress>,
    #[doc = "Shipping name."]
    pub name: Option<String>,
    #[doc = "Shipping phone number."]
    pub phone: Option<String>,
}

#[cfg(feature = "radar")]
impl crate::params::FormEncode for CreateRadarPaymentEvaluationParamsPaymentDetailsShippingDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("address", self.address.as_ref());
        form.field_opt("name", self.name.as_ref());
        form.field_opt("phone", self.phone.as_ref());
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InsightsResourcesPaymentEvaluationSucceeded {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types_05::radar::InsightsResourcesPaymentEvaluationSucceededCard>,
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, Default)]
pub struct ListRadarValueListItemsParams {
    pub created: Option<stripe_types_05::radar::ListRadarValueListItemsParamsCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub value: Option<String>,
    pub value_list: String,
}

#[cfg(feature = "radar")]
impl crate::params::FormEncode for ListRadarValueListItemsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("created", self.created.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
        form.field_opt("value", self.value.as_ref());
        form.field("value_list", &self.value_list);
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct RadarValueList {
    #[doc = "The name of the value list for use in rules."]
    #[json(default)]
    pub alias: String,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "The name or email address of the user who created this value list."]
    #[json(default)]
    pub created_by: String,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "The type of items in the value list."]
    #[json(default)]
    pub item_type: stripe_types_01::radar::RadarValueListItemType,
    #[doc = "List of items contained within this value list."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub list_items: Option<crate::List<stripe_types_06::radar::RadarValueListItem>>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "The name of the value list."]
    #[json(default)]
    pub name: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::radar::RadarValueListObject,
}

#[cfg(feature = "radar")]
impl crate::PagedItem for RadarValueList {
    fn id(&self) -> &str {
        &self.id
    }
}
