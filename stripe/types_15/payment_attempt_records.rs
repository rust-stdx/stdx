#[cfg(feature = "payment_attempt_records")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentAttemptRecord {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<stripe_types_04::payment_records::PaymentsPrimitivesPaymentRecordsResourceAmount>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_authorized: Option<stripe_types_04::payment_records::PaymentsPrimitivesPaymentRecordsResourceAmount>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_canceled: Option<stripe_types_04::payment_records::PaymentsPrimitivesPaymentRecordsResourceAmount>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_failed: Option<stripe_types_04::payment_records::PaymentsPrimitivesPaymentRecordsResourceAmount>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_guaranteed: Option<stripe_types_04::payment_records::PaymentsPrimitivesPaymentRecordsResourceAmount>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_refunded: Option<stripe_types_04::payment_records::PaymentsPrimitivesPaymentRecordsResourceAmount>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_requested: Option<stripe_types_04::payment_records::PaymentsPrimitivesPaymentRecordsResourceAmount>,
    #[doc = "ID of the Connect application that created the PaymentAttemptRecord."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<String>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Customer information for this payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_details:
        Option<stripe_types_03::payment_records::PaymentsPrimitivesPaymentRecordsResourceCustomerDetails>,
    #[doc = "Indicates whether the customer was present in your checkout flow during this payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_presence: Option<stripe_types_02::payment_attempt_records::PaymentAttemptRecordCustomerPresence>,
    #[doc = "An arbitrary string attached to the object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::payment_attempt_records::PaymentAttemptRecordObject,
    #[doc = "Information about the Payment Method debited for this payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_details:
        Option<stripe_types_14::payment_records::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodDetails>,
    #[doc = "ID of the Payment Record this Payment Attempt Record belongs to."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_record: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub processor_details:
        Option<stripe_types_05::payment_records::PaymentsPrimitivesPaymentRecordsResourceProcessorDetails>,
    #[doc = "Indicates who reported the payment."]
    #[json(default)]
    pub reported_by: stripe_types_04::payment_attempt_records::PaymentAttemptRecordReportedBy,
    #[doc = "Shipping information for this payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub shipping_details:
        Option<stripe_types_05::payment_records::PaymentsPrimitivesPaymentRecordsResourceShippingDetails>,
}

#[cfg(feature = "payment_attempt_records")]
impl crate::PagedItem for PaymentAttemptRecord {
    fn id(&self) -> &str {
        &self.id
    }
}
