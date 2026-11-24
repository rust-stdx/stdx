#[cfg(feature = "charges")]
#[derive(Debug, Clone, Default)]
pub struct CreateChargeParams {
    #[doc = "Amount intended to be collected by this payment."]
    pub amount: Option<i64>,
    pub application_fee: Option<i64>,
    #[doc = "A fee in cents (or local equivalent) that will be applied to the charge and transferred to the application owner's St..."]
    pub application_fee_amount: Option<i64>,
    #[doc = "Whether to immediately capture the charge."]
    pub capture: Option<bool>,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: Option<String>,
    #[doc = "The ID of an existing customer that will be charged in this request."]
    pub customer: Option<String>,
    #[doc = "An arbitrary string which you can attach to a `Charge` object."]
    pub description: Option<String>,
    pub destination: Option<stripe_types_01::charges::CreateChargeParamsDestination>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The Stripe account ID for which these funds are intended."]
    pub on_behalf_of: Option<String>,
    #[doc = "Options to configure Radar."]
    pub radar_options: Option<stripe_types_02::charges::CreateChargeParamsRadarOptions>,
    #[doc = "The email address to which this charge's [receipt](https://docs.stripe.com/dashboard/receipts) will be sent."]
    pub receipt_email: Option<String>,
    #[doc = "Shipping information for the charge."]
    pub shipping: Option<stripe_types_06::charges::CreateChargeParamsShipping>,
    #[doc = "A payment source to be charged."]
    pub source: Option<String>,
    #[doc = "For a non-card charge, text that appears on the customer's statement as the statement descriptor."]
    pub statement_descriptor: Option<String>,
    #[doc = "Provides information about a card charge."]
    pub statement_descriptor_suffix: Option<String>,
    #[doc = "An optional dictionary including the account to automatically transfer to as part of a destination charge."]
    pub transfer_data: Option<stripe_types_04::charges::CreateChargeParamsTransferData>,
    #[doc = "A string that identifies this transaction as part of a group."]
    pub transfer_group: Option<String>,
}

#[cfg(feature = "charges")]
impl crate::params::FormEncode for CreateChargeParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("application_fee", self.application_fee.as_ref());
        form.field_opt("application_fee_amount", self.application_fee_amount.as_ref());
        form.field_opt("capture", self.capture.as_ref());
        form.field_opt("currency", self.currency.as_ref());
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field_opt("destination", self.destination.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("on_behalf_of", self.on_behalf_of.as_ref());
        form.field_opt("radar_options", self.radar_options.as_ref());
        form.field_opt("receipt_email", self.receipt_email.as_ref());
        form.field_opt("shipping", self.shipping.as_ref());
        form.field_opt("source", self.source.as_ref());
        form.field_opt("statement_descriptor", self.statement_descriptor.as_ref());
        form.field_opt("statement_descriptor_suffix", self.statement_descriptor_suffix.as_ref());
        form.field_opt("transfer_data", self.transfer_data.as_ref());
        form.field_opt("transfer_group", self.transfer_group.as_ref());
    }
}
