#[cfg(feature = "sources")]
#[derive(Debug, Clone, Default)]
pub struct CreateSourceParamsSourceOrder {
    #[doc = "List of items constituting the order."]
    pub items: Option<Vec<stripe_types_06::sources::CreateSourceParamsSourceOrderItemsItem>>,
    #[doc = "Shipping address for the order."]
    pub shipping: Option<stripe_types_06::sources::CreateSourceParamsSourceOrderShipping>,
}

#[cfg(feature = "sources")]
impl crate::params::FormEncode for CreateSourceParamsSourceOrder {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.items.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("items", index, item);
            }
        }
        form.field_opt("shipping", self.shipping.as_ref());
    }
}
