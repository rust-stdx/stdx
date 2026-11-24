#[cfg(feature = "plans")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct DeletedPlan {
    #[doc = "Always true for a deleted object"]
    #[json(default)]
    pub deleted: bool,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::plans::PlanObject,
}

#[cfg(feature = "plans")]
impl crate::PagedItem for DeletedPlan {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "plans")]
#[derive(Debug, Clone, Default)]
pub struct ListPlansParams {
    pub active: Option<bool>,
    pub created: Option<stripe_types_02::plans::ListPlansParamsCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub product: Option<String>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "plans")]
impl crate::params::FormEncode for ListPlansParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("active", self.active.as_ref());
        form.field_opt("created", self.created.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("product", self.product.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}
