#[cfg(feature = "sources")]
#[derive(Debug, Clone, Default)]
pub struct CreateSourceParamsOwner {
    #[doc = "Owner's address."]
    pub address: Option<stripe_types_01::sources::CreateSourceParamsOwnerAddress>,
    #[doc = "Owner's email address."]
    pub email: Option<String>,
    #[doc = "Owner's full name."]
    pub name: Option<String>,
    #[doc = "Owner's phone number."]
    pub phone: Option<String>,
}

#[cfg(feature = "sources")]
impl crate::params::FormEncode for CreateSourceParamsOwner {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("address", self.address.as_ref());
        form.field_opt("email", self.email.as_ref());
        form.field_opt("name", self.name.as_ref());
        form.field_opt("phone", self.phone.as_ref());
    }
}

#[cfg(feature = "sources")]
#[derive(Debug, Clone, Default)]
pub struct CreateSourceParamsReceiver {
    #[doc = "The method Stripe should use to request information needed to process a refund or mispayment."]
    pub refund_attributes_method: Option<stripe_types_00::sources::CreateSourceParamsReceiverRefundAttributesMethod>,
}

#[cfg(feature = "sources")]
impl crate::params::FormEncode for CreateSourceParamsReceiver {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("refund_attributes_method", self.refund_attributes_method.as_ref());
    }
}

#[cfg(feature = "sources")]
#[derive(Debug, Clone, Default)]
pub struct CreateSourceParamsSourceOrderItemsItem {
    pub amount: Option<i64>,
    pub currency: Option<String>,
    pub description: Option<String>,
    #[doc = "The ID of the SKU being ordered."]
    pub parent: Option<String>,
    #[doc = "The quantity of this order item."]
    pub quantity: Option<i64>,
    pub r#type: Option<stripe_types_00::sources::CreateSourceParamsSourceOrderItemsItemType>,
}

#[cfg(feature = "sources")]
impl crate::params::FormEncode for CreateSourceParamsSourceOrderItemsItem {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("currency", self.currency.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field_opt("parent", self.parent.as_ref());
        form.field_opt("quantity", self.quantity.as_ref());
        form.field_opt("type", self.r#type.as_ref());
    }
}

#[cfg(feature = "sources")]
#[derive(Debug, Clone)]
pub struct CreateSourceParamsSourceOrderShipping {
    #[doc = "Shipping address."]
    pub address: stripe_types_01::sources::CreateSourceParamsSourceOrderShippingAddress,
    #[doc = "The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc."]
    pub carrier: Option<String>,
    #[doc = "Recipient name."]
    pub name: Option<String>,
    #[doc = "Recipient phone (including extension)."]
    pub phone: Option<String>,
    #[doc = "The tracking number for a physical product, obtained from the delivery service."]
    pub tracking_number: Option<String>,
}

#[cfg(feature = "sources")]
impl crate::params::FormEncode for CreateSourceParamsSourceOrderShipping {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("address", &self.address);
        form.field_opt("carrier", self.carrier.as_ref());
        form.field_opt("name", self.name.as_ref());
        form.field_opt("phone", self.phone.as_ref());
        form.field_opt("tracking_number", self.tracking_number.as_ref());
    }
}

#[cfg(any(feature = "events", feature = "sources"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SourceTransaction {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ach_credit_transfer: Option<stripe_types_01::sources::SourceTransactionAchCreditTransferData>,
    #[doc = "A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zer..."]
    #[json(default)]
    pub amount: i64,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub chf_credit_transfer: Option<stripe_types_04::sources::SourceTransactionChfCreditTransferData>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub gbp_credit_transfer: Option<stripe_types_03::sources::SourceTransactionGbpCreditTransferData>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::sources::SourceTransactionObject,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub paper_check: Option<stripe_types_00::sources::SourceTransactionPaperCheckData>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub sepa_credit_transfer: Option<stripe_types_03::sources::SourceTransactionSepaCreditTransferData>,
    #[doc = "The ID of the source this transaction is attached to."]
    #[json(default)]
    pub source: String,
    #[doc = "The status of the transaction, one of `succeeded`, `pending`, or `failed`."]
    #[json(default)]
    pub status: String,
    #[doc = "The type of source this transaction is attached to."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_04::sources::SourceTransactionType,
}

#[cfg(any(feature = "events", feature = "sources"))]
impl crate::PagedItem for SourceTransaction {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "sources")]
pub type UpdateSourceParamsMandateAcceptanceOnline =
    stripe_types_01::sources::CreateSourceParamsMandateAcceptanceOnline;

#[cfg(feature = "sources")]
pub type UpdateSourceParamsMandateInterval = stripe_types_01::sources::CreateSourceParamsMandateInterval;

#[cfg(feature = "sources")]
pub type UpdateSourceParamsMandateNotificationMethod =
    stripe_types_04::sources::CreateSourceParamsMandateNotificationMethod;

#[cfg(feature = "sources")]
pub type UpdateSourceParamsOwnerAddress = stripe_types_01::sources::CreateSourceParamsOwnerAddress;

#[cfg(feature = "sources")]
pub type UpdateSourceParamsSourceOrderItemsItemType =
    stripe_types_00::sources::CreateSourceParamsSourceOrderItemsItemType;
