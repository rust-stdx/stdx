#[cfg(feature = "climate")]
#[derive(Debug, Clone, Default)]
pub struct CreateClimateOrderParamsBeneficiary {
    #[doc = "Publicly displayable name for the end beneficiary of carbon removal."]
    pub public_name: String,
}

#[cfg(feature = "climate")]
impl crate::params::FormEncode for CreateClimateOrderParamsBeneficiary {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("public_name", &self.public_name);
    }
}
