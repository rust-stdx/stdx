#[cfg(feature = "tax_ids")]
#[derive(Debug, Clone)]
pub struct CreateTaxIdParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The account or customer the tax ID belongs to."]
    pub owner: Option<stripe_types_08::tax_ids::CreateTaxIdParamsOwner>,
    #[doc = "Type of the tax ID, one of `ad_nrt`, `ae_trn`, `al_tin`, `am_tin`, `ao_tin`, `ar_cuit`, `au_abn`, `au_arn`, `aw_tin`,..."]
    pub r#type: stripe_types_03::tax_ids::CreateTaxIdParamsType,
    #[doc = "Value of the tax ID."]
    pub value: String,
}

#[cfg(feature = "tax_ids")]
impl crate::params::FormEncode for CreateTaxIdParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("owner", self.owner.as_ref());
        form.field("type", &self.r#type);
        form.field("value", &self.value);
    }
}
