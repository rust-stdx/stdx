#[cfg(feature = "payouts")]
pub type CancelPayoutParams = stripe_types_02::payouts::GetPayoutParams;

#[cfg(feature = "payouts")]
#[derive(Debug, Clone, Default)]
pub struct CreatePayoutParams {
    #[doc = "A positive integer in cents representing how much to pay out."]
    pub amount: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "An arbitrary string attached to the object."]
    pub description: Option<String>,
    #[doc = "The ID of a bank account or a card to send the payout to."]
    pub destination: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The method used to send this payout, which is `standard` or `instant`."]
    pub method: Option<stripe_types_01::payouts::CreatePayoutParamsMethod>,
    #[doc = "The ID of a v2 FinancialAccount to send funds to."]
    pub payout_method: Option<String>,
    #[doc = "The balance type of your Stripe balance to draw this payout from."]
    pub source_type: Option<stripe_types_00::payouts::CreatePayoutParamsSourceType>,
    #[doc = "A string that displays on the recipient's bank or card statement (up to 22 characters)."]
    pub statement_descriptor: Option<String>,
}

#[cfg(feature = "payouts")]
impl crate::params::FormEncode for CreatePayoutParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        form.field("currency", &self.currency);
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
        form.field_opt("method", self.method.as_ref());
        form.field_opt("payout_method", self.payout_method.as_ref());
        form.field_opt("source_type", self.source_type.as_ref());
        form.field_opt("statement_descriptor", self.statement_descriptor.as_ref());
    }
}

#[cfg(feature = "payouts")]
pub type ListPayoutsParamsCreated = stripe_types_02::payouts::ListPayoutsParamsArrivalDate;

#[cfg(feature = "payouts")]
pub type ReversePayoutParams = stripe_types_04::payouts::UpdatePayoutParams;
