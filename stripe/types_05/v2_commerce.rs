#[cfg(feature = "v2_commerce")]
#[derive(Debug, Clone, Default)]
pub struct ListCommerceProductCatalogImportsParams {
    pub created: Option<String>,
    pub created_gt: Option<String>,
    pub created_gte: Option<String>,
    pub created_lt: Option<String>,
    pub created_lte: Option<String>,
    pub feed_type: Option<stripe_types_00::v2_commerce::ListCommerceProductCatalogImportsParamsFeedType>,
    pub limit: Option<i64>,
    pub status: Option<stripe_types_01::v2_commerce::ListCommerceProductCatalogImportsParamsStatus>,
}

#[cfg(feature = "v2_commerce")]
impl crate::params::FormEncode for ListCommerceProductCatalogImportsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("created", self.created.as_ref());
        form.field_opt("created_gt", self.created_gt.as_ref());
        form.field_opt("created_gte", self.created_gte.as_ref());
        form.field_opt("created_lt", self.created_lt.as_ref());
        form.field_opt("created_lte", self.created_lte.as_ref());
        form.field_opt("feed_type", self.feed_type.as_ref());
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("status", self.status.as_ref());
    }
}
