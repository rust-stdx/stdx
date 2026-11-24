#[cfg(feature = "radar")]
#[derive(Debug, Clone, Default)]
pub struct CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsCard {
    #[doc = "Describes the presence of the customer during the payment."]
    pub customer_presence: Option<stripe_types_02::radar::CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsCardCustomerPresence>,
    #[doc = "Describes the type of payment."]
    pub payment_type: Option<stripe_types_01::radar::CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsCardPaymentType>,
}

#[cfg(feature = "radar")]
impl crate::params::FormEncode for CreateRadarPaymentEvaluationParamsPaymentDetailsMoneyMovementDetailsCard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("customer_presence", self.customer_presence.as_ref());
        form.field_opt("payment_type", self.payment_type.as_ref());
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, Default)]
pub struct CreateRadarValueListParams {
    #[doc = "The name of the value list for use in rules."]
    pub alias: String,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Type of the items in the value list."]
    pub item_type: Option<stripe_types_02::radar::CreateRadarValueListParamsItemType>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "The human-readable name of the value list."]
    pub name: String,
}

#[cfg(feature = "radar")]
impl crate::params::FormEncode for CreateRadarValueListParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("alias", &self.alias);
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("item_type", self.item_type.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field("name", &self.name);
    }
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct DeletedRadarValueListItem {
    #[doc = "Always true for a deleted object"]
    #[json(default)]
    pub deleted: bool,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_01::radar::RadarValueListItemObject,
}

#[cfg(feature = "radar")]
impl crate::PagedItem for DeletedRadarValueListItem {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "radar")]
pub type GetRadarValueListParams = stripe_types_02::radar::GetRadarEarlyFraudWarningParams;

#[cfg(feature = "radar")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InsightsResourcesPaymentEvaluationEarlyFraudWarningReceived {
    #[doc = "The type of fraud labeled by the issuer."]
    #[json(default)]
    pub fraud_type: stripe_types_00::radar::InsightsResourcesPaymentEvaluationEarlyFraudWarningReceivedFraudType,
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InsightsResourcesPaymentEvaluationMerchantBlocked {
    #[doc = "The reason the payment was blocked by the merchant."]
    #[json(default)]
    pub reason: stripe_types_04::radar::InsightsResourcesPaymentEvaluationMerchantBlockedReason,
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InsightsResourcesPaymentEvaluationRefunded {
    #[doc = "Amount refunded for this payment."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "Indicates the reason for the refund."]
    #[json(default)]
    pub reason: stripe_types_00::radar::InsightsResourcesPaymentEvaluationRefundedReason,
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InsightsResourcesPaymentEvaluationShipping {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<stripe_types_04::radar::InsightsResourcesPaymentEvaluationAddress>,
    #[doc = "Shipping name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Shipping phone number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InsightsResourcesPaymentEvaluationSignalV2 {
    #[doc = "The time when this signal was evaluated."]
    #[json(default)]
    pub evaluated_at: i64,
    #[doc = "Risk level of this signal, based on the score."]
    #[json(default)]
    pub risk_level: stripe_types_04::radar::InsightsResourcesPaymentEvaluationSignalV2RiskLevel,
    #[doc = "Numeric score for this signal, returned with two decimal places."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InsightsResourcesPaymentEvaluationSucceededCard {
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
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InsightsResourcesPaymentEvaluationUserInterventionRaised {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<stripe_types_01::radar::InsightsResourcesPaymentEvaluationUserInterventionRaisedCustom>,
    #[doc = "Unique identifier for the user intervention event."]
    #[json(default)]
    pub key: String,
    #[doc = "Type of user intervention raised."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::radar::InsightsResourcesPaymentEvaluationUserInterventionRaisedType,
}

#[cfg(feature = "radar")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InsightsResourcesPaymentEvaluationUserInterventionResolved {
    #[doc = "Unique ID of this intervention."]
    #[json(default)]
    pub key: String,
    #[doc = "Result of the intervention if it has been completed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<stripe_types_00::radar::InsightsResourcesPaymentEvaluationUserInterventionResolvedOutcome>,
}

#[cfg(feature = "radar")]
pub type ListRadarValueListItemsParamsCreated = stripe_types_03::radar::ListRadarEarlyFraudWarningsParamsCreated;

#[cfg(feature = "radar")]
pub type ListRadarValueListsParamsCreated = stripe_types_03::radar::ListRadarEarlyFraudWarningsParamsCreated;
