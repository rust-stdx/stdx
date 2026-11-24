#[cfg(feature = "radar")]
#[derive(Debug, Clone, Default)]
pub struct CreateRadarPaymentEvaluationParamsPaymentDetailsPaymentMethodDetailsBillingDetails {
    #[doc = "Billing address."]
    pub address: Option<stripe_types_02::radar::CreateRadarPaymentEvaluationParamsPaymentDetailsPaymentMethodDetailsBillingDetailsAddress>,
    #[doc = "Email address."]
    pub email: Option<String>,
    #[doc = "Full name."]
    pub name: Option<String>,
    #[doc = "Billing phone number (including extension)."]
    pub phone: Option<String>,
}

#[cfg(feature = "radar")]
impl crate::params::FormEncode for CreateRadarPaymentEvaluationParamsPaymentDetailsPaymentMethodDetailsBillingDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("address", self.address.as_ref());
        form.field_opt("email", self.email.as_ref());
        form.field_opt("name", self.name.as_ref());
        form.field_opt("phone", self.phone.as_ref());
    }
}

#[cfg(feature = "radar")]
pub type CreateRadarPaymentEvaluationParamsPaymentDetailsShippingDetailsAddress =
    stripe_types_02::radar::CreateRadarPaymentEvaluationParamsPaymentDetailsPaymentMethodDetailsBillingDetailsAddress;

#[cfg(feature = "radar")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct DeletedRadarValueList {
    #[doc = "Always true for a deleted object"]
    #[json(default)]
    pub deleted: bool,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::radar::RadarValueListObject,
}

#[cfg(feature = "radar")]
impl crate::PagedItem for DeletedRadarValueList {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "radar")]
pub type GetRadarValueListItemParams = stripe_types_02::radar::GetRadarEarlyFraudWarningParams;

#[cfg(feature = "radar")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InsightsResourcesPaymentEvaluationBillingDetails {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_04::radar::InsightsResourcesPaymentEvaluationAddress>,
    #[doc = "Email address."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "Full name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Billing phone number (including extension)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InsightsResourcesPaymentEvaluationDisputeOpened {
    #[doc = "Amount to dispute for this payment."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "Reason given by cardholder for dispute."]
    #[json(default)]
    pub reason: stripe_types_00::radar::InsightsResourcesPaymentEvaluationDisputeOpenedReason,
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InsightsResourcesPaymentEvaluationMoneyMovementCard {
    #[doc = "Describes the presence of the customer during the payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_presence:
        Option<stripe_types_01::radar::InsightsResourcesPaymentEvaluationMoneyMovementCardCustomerPresence>,
    #[doc = "Describes the type of payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<stripe_types_02::radar::InsightsResourcesPaymentEvaluationMoneyMovementCardPaymentType>,
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InsightsResourcesPaymentEvaluationRejectedCard {
    #[doc = "Result of the address line 1 check."]
    #[json(default)]
    pub address_line1_check: stripe_types_04::radar::InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check,
    #[doc = "Indicates whether the cardholder provided a postal code and if it matched the cardholder’s billing address."]
    #[json(default)]
    pub address_postal_code_check:
        stripe_types_04::radar::InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check,
    #[doc = "Result of the CVC check."]
    #[json(default)]
    pub cvc_check: stripe_types_04::radar::InsightsResourcesPaymentEvaluationRejectedCardAddressLine1Check,
    #[doc = "Card issuer's reason for the network decline."]
    #[json(default)]
    pub reason: stripe_types_00::radar::InsightsResourcesPaymentEvaluationRejectedCardReason,
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, Default)]
pub struct ListRadarEarlyFraudWarningsParams {
    pub charge: Option<String>,
    pub created: Option<stripe_types_03::radar::ListRadarEarlyFraudWarningsParamsCreated>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub payment_intent: Option<String>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "radar")]
impl crate::params::FormEncode for ListRadarEarlyFraudWarningsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("charge", self.charge.as_ref());
        form.field_opt("created", self.created.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("payment_intent", self.payment_intent.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct RadarValueListItem {
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "The name or email address of the user who added this item to the value list."]
    #[json(default)]
    pub created_by: String,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_01::radar::RadarValueListItemObject,
    #[doc = "The value of the item."]
    #[json(default)]
    pub value: String,
    #[doc = "The identifier of the value list this item belongs to."]
    #[json(default)]
    pub value_list: String,
}

#[cfg(feature = "radar")]
impl crate::PagedItem for RadarValueListItem {
    fn id(&self) -> &str {
        &self.id
    }
}
