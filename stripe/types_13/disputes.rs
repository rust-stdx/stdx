#[cfg(feature = "disputes")]
#[derive(Debug, Clone, Default)]
pub struct UpdateDisputeParams {
    #[doc = "Evidence to upload, to respond to a dispute."]
    pub evidence: Option<crate::disputes::UpdateDisputeParamsEvidence>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "Whether to immediately submit evidence to the bank."]
    pub submit: Option<bool>,
}

#[cfg(feature = "disputes")]
impl crate::params::FormEncode for UpdateDisputeParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("evidence", self.evidence.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("submit", self.submit.as_ref());
    }
}

#[cfg(feature = "disputes")]
#[derive(Debug, Clone, Default)]
pub struct UpdateDisputeParamsEvidence {
    #[doc = "Any server or activity logs showing proof that the customer accessed or downloaded the purchased digital product."]
    pub access_activity_log: Option<String>,
    #[doc = "The billing address provided by the customer."]
    pub billing_address: Option<String>,
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Your subscription cancellation policy, as shown t..."]
    pub cancellation_policy: Option<String>,
    #[doc = "An explanation of how and when the customer was shown your refund policy prior to purchase."]
    pub cancellation_policy_disclosure: Option<String>,
    #[doc = "A justification for why the customer's subscription was not canceled."]
    pub cancellation_rebuttal: Option<String>,
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Any communication with the customer that you feel..."]
    pub customer_communication: Option<String>,
    #[doc = "The email address of the customer."]
    pub customer_email_address: Option<String>,
    #[doc = "The name of the customer."]
    pub customer_name: Option<String>,
    #[doc = "The IP address that the customer used when making the purchase."]
    pub customer_purchase_ip: Option<String>,
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) A relevant document or contract showing the custo..."]
    pub customer_signature: Option<String>,
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Documentation for the prior charge that can uniqu..."]
    pub duplicate_charge_documentation: Option<String>,
    #[doc = "An explanation of the difference between the disputed charge versus the prior charge that appears to be a duplicate."]
    pub duplicate_charge_explanation: Option<String>,
    #[doc = "The Stripe ID for the prior charge which appears to be a duplicate of the disputed charge."]
    pub duplicate_charge_id: Option<String>,
    #[doc = "Additional evidence for qualifying evidence programs."]
    pub enhanced_evidence: Option<stripe_types_12::disputes::UpdateDisputeParamsEvidenceEnhancedEvidence>,
    #[doc = "A description of the product or service that was sold."]
    pub product_description: Option<String>,
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Any receipt or message sent to the customer notif..."]
    pub receipt: Option<String>,
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Your refund policy, as shown to the customer."]
    pub refund_policy: Option<String>,
    #[doc = "Documentation demonstrating that the customer was shown your refund policy prior to purchase."]
    pub refund_policy_disclosure: Option<String>,
    #[doc = "A justification for why the customer is not entitled to a refund."]
    pub refund_refusal_explanation: Option<String>,
    #[doc = "The date on which the customer received or began receiving the purchased service, in a clear human-readable format."]
    pub service_date: Option<String>,
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Documentation showing proof that a service was pr..."]
    pub service_documentation: Option<String>,
    #[doc = "The address to which a physical product was shipped."]
    pub shipping_address: Option<String>,
    #[doc = "The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc."]
    pub shipping_carrier: Option<String>,
    #[doc = "The date on which a physical product began its route to the shipping address, in a clear human-readable format."]
    pub shipping_date: Option<String>,
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Documentation showing proof that a product was sh..."]
    pub shipping_documentation: Option<String>,
    #[doc = "The tracking number for a physical product, obtained from the delivery service."]
    pub shipping_tracking_number: Option<String>,
    #[doc = "(ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Any additional evidence or statements."]
    pub uncategorized_file: Option<String>,
    #[doc = "Any additional evidence or statements."]
    pub uncategorized_text: Option<String>,
}

#[cfg(feature = "disputes")]
impl crate::params::FormEncode for UpdateDisputeParamsEvidence {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("access_activity_log", self.access_activity_log.as_ref());
        form.field_opt("billing_address", self.billing_address.as_ref());
        form.field_opt("cancellation_policy", self.cancellation_policy.as_ref());
        form.field_opt("cancellation_policy_disclosure", self.cancellation_policy_disclosure.as_ref());
        form.field_opt("cancellation_rebuttal", self.cancellation_rebuttal.as_ref());
        form.field_opt("customer_communication", self.customer_communication.as_ref());
        form.field_opt("customer_email_address", self.customer_email_address.as_ref());
        form.field_opt("customer_name", self.customer_name.as_ref());
        form.field_opt("customer_purchase_ip", self.customer_purchase_ip.as_ref());
        form.field_opt("customer_signature", self.customer_signature.as_ref());
        form.field_opt("duplicate_charge_documentation", self.duplicate_charge_documentation.as_ref());
        form.field_opt("duplicate_charge_explanation", self.duplicate_charge_explanation.as_ref());
        form.field_opt("duplicate_charge_id", self.duplicate_charge_id.as_ref());
        form.field_opt("enhanced_evidence", self.enhanced_evidence.as_ref());
        form.field_opt("product_description", self.product_description.as_ref());
        form.field_opt("receipt", self.receipt.as_ref());
        form.field_opt("refund_policy", self.refund_policy.as_ref());
        form.field_opt("refund_policy_disclosure", self.refund_policy_disclosure.as_ref());
        form.field_opt("refund_refusal_explanation", self.refund_refusal_explanation.as_ref());
        form.field_opt("service_date", self.service_date.as_ref());
        form.field_opt("service_documentation", self.service_documentation.as_ref());
        form.field_opt("shipping_address", self.shipping_address.as_ref());
        form.field_opt("shipping_carrier", self.shipping_carrier.as_ref());
        form.field_opt("shipping_date", self.shipping_date.as_ref());
        form.field_opt("shipping_documentation", self.shipping_documentation.as_ref());
        form.field_opt("shipping_tracking_number", self.shipping_tracking_number.as_ref());
        form.field_opt("uncategorized_file", self.uncategorized_file.as_ref());
        form.field_opt("uncategorized_text", self.uncategorized_text.as_ref());
    }
}
