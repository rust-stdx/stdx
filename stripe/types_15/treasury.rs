#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InboundTransfers {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<stripe_types_06::treasury::TreasurySharedResourceBillingDetails>,
    #[doc = "The type of the payment method used in the InboundTransfer."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::treasury::InboundTransfersType,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<crate::treasury::InboundTransfersPaymentMethodDetailsUsBankAccount>,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InboundTransfersPaymentMethodDetailsUsBankAccount {
    #[doc = "Account holder type: individual or company."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_holder_type:
        Option<stripe_types_02::treasury::InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType>,
    #[doc = "Account type: checkings or savings."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_type: Option<stripe_types_00::treasury::InboundTransfersPaymentMethodDetailsUsBankAccountAccountType>,
    #[doc = "Name of the bank associated with the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[doc = "Uniquely identifies this particular bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[doc = "Last four digits of the bank account number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[doc = "ID of the mandate used to make this payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate: Option<crate::Expandable<stripe_types_14::mandates::Mandate>>,
    #[doc = "The network rails used."]
    #[json(default)]
    pub network: stripe_types_02::treasury::InboundTransfersPaymentMethodDetailsUsBankAccountNetwork,
    #[doc = "Routing number of the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct OutboundPaymentsPaymentMethodDetails {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<stripe_types_06::treasury::TreasurySharedResourceBillingDetails>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<stripe_types_05::treasury::OutboundPaymentsPaymentMethodDetailsFinancialAccount>,
    #[doc = "The type of the payment method used in the OutboundPayment."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::treasury::OutboundPaymentsPaymentMethodDetailsType,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<crate::treasury::OutboundPaymentsPaymentMethodDetailsUsBankAccount>,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct OutboundPaymentsPaymentMethodDetailsUsBankAccount {
    #[doc = "Account holder type: individual or company."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_holder_type:
        Option<stripe_types_02::treasury::InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType>,
    #[doc = "Account type: checkings or savings."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_type: Option<stripe_types_00::treasury::InboundTransfersPaymentMethodDetailsUsBankAccountAccountType>,
    #[doc = "Name of the bank associated with the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[doc = "Uniquely identifies this particular bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[doc = "Last four digits of the bank account number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[doc = "ID of the mandate used to make this payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate: Option<crate::Expandable<stripe_types_14::mandates::Mandate>>,
    #[doc = "The network rails used."]
    #[json(default)]
    pub network: stripe_types_04::treasury::OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork,
    #[doc = "Routing number of the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct OutboundTransfersPaymentMethodDetails {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<stripe_types_06::treasury::TreasurySharedResourceBillingDetails>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<stripe_types_05::treasury::OutboundTransfersPaymentMethodDetailsFinancialAccount>,
    #[doc = "The type of the payment method used in the OutboundTransfer."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_03::treasury::OutboundPaymentsPaymentMethodDetailsType,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<crate::treasury::OutboundTransfersPaymentMethodDetailsUsBankAccount>,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct OutboundTransfersPaymentMethodDetailsUsBankAccount {
    #[doc = "Account holder type: individual or company."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_holder_type:
        Option<stripe_types_02::treasury::InboundTransfersPaymentMethodDetailsUsBankAccountAccountHolderType>,
    #[doc = "Account type: checkings or savings."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_type: Option<stripe_types_00::treasury::InboundTransfersPaymentMethodDetailsUsBankAccountAccountType>,
    #[doc = "Name of the bank associated with the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[doc = "Uniquely identifies this particular bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[doc = "Last four digits of the bank account number."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[doc = "ID of the mandate used to make this payment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate: Option<crate::Expandable<stripe_types_14::mandates::Mandate>>,
    #[doc = "The network rails used."]
    #[json(default)]
    pub network: stripe_types_04::treasury::OutboundPaymentsPaymentMethodDetailsUsBankAccountNetwork,
    #[doc = "Routing number of the bank account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ReserveHold {
    #[doc = "Amount reserved."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Amount in cents that can be released from this ReserveHold"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_releasable: Option<i64>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Indicates which party created this ReserveHold."]
    #[json(default)]
    pub created_by: stripe_types_01::treasury::ReserveHoldCreatedBy,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "The balance destination to which the reserved funds are sent."]
    #[json(default)]
    pub destination: stripe_types_04::treasury::ReserveHoldDestination,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "Whether there are any funds available to release on this ReserveHold."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub is_releasable: Option<bool>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::treasury::ReserveHoldObject,
    #[doc = "The reason for the ReserveHold."]
    #[json(default)]
    pub reason: stripe_types_00::treasury::ReserveHoldReason,
    #[doc = "List of ReserveReleases and the amounts released from this ReserveHold."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub release_details: Option<Vec<stripe_types_00::treasury::ReservesReserveHoldsResourcesReleaseDetail>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub release_schedule: Option<stripe_types_00::treasury::ReservesReserveHoldsResourcesReleaseSchedule>,
    #[doc = "The ReservePlan which produced this ReserveHold (i.e., resplan_123)"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reserve_plan: Option<crate::Expandable<stripe_types_06::treasury::ReservePlan>>,
    #[doc = "The Charge which funded this ReserveHold (e.g., ch_123)"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub source_charge: Option<crate::Expandable<stripe_types_14::charges::Charge>>,
    #[doc = "Which source balance type this ReserveHold reserves funds from."]
    #[json(default)]
    pub source_type: stripe_types_00::treasury::ReserveHoldSourceType,
}

#[cfg(feature = "events")]
impl crate::PagedItem for ReserveHold {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ReserveRelease {
    #[doc = "Amount released."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Indicates which party created this ReserveRelease."]
    #[json(default)]
    pub created_by: stripe_types_01::treasury::ReserveHoldCreatedBy,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "The balance destination to which the released funds are sent."]
    #[json(default)]
    pub destination: stripe_types_00::treasury::ReserveReleaseDestination,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_01::treasury::ReserveReleaseObject,
    #[doc = "The reason for the ReserveRelease, indicating why the funds were released."]
    #[json(default)]
    pub reason: stripe_types_04::treasury::ReserveReleaseReason,
    #[doc = "The release timestamp of the funds."]
    #[json(default)]
    pub released_at: i64,
    #[doc = "The ReserveHold this ReserveRelease is associated with."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reserve_hold: Option<crate::Expandable<crate::treasury::ReserveHold>>,
    #[doc = "The ReservePlan ID this ReserveRelease is associated with."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reserve_plan: Option<crate::Expandable<stripe_types_06::treasury::ReservePlan>>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub source_transaction: Option<crate::treasury::ReservesReserveReleasesResourcesSourceTransaction>,
}

#[cfg(feature = "events")]
impl crate::PagedItem for ReserveRelease {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ReservesReserveReleasesResourcesSourceTransaction {
    #[doc = "The ID of the dispute."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub dispute: Option<crate::Expandable<stripe_types_14::disputes::Dispute>>,
    #[doc = "The ID of the refund."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub refund: Option<crate::Expandable<stripe_types_14::refunds::Refund>>,
    #[doc = "The type of source transaction."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::treasury::ReservesReserveReleasesResourcesSourceTransactionType,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryCreditReversal {
    #[doc = "Amount (in cents) transferred."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "The FinancialAccount to reverse funds from."]
    #[json(default)]
    pub financial_account: String,
    #[doc = "A [hosted transaction receipt](https://docs.stripe.com/treasury/moving-money/regulatory-receipts) URL that is provide..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub hosted_regulatory_receipt_url: Option<String>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "The rails used to reverse the funds."]
    #[json(default)]
    pub network: stripe_types_01::treasury::TreasuryCreditReversalNetwork,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::treasury::TreasuryCreditReversalObject,
    #[doc = "The ReceivedCredit being reversed."]
    #[json(default)]
    pub received_credit: String,
    #[doc = "Status of the CreditReversal"]
    #[json(default)]
    pub status: stripe_types_04::treasury::TreasuryCreditReversalStatus,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub status_transitions: Option<stripe_types_02::treasury::TreasuryReceivedCreditsResourceStatusTransitions>,
    #[doc = "The Transaction associated with this object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transaction: Option<crate::Expandable<crate::treasury::TreasuryTransaction>>,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl crate::PagedItem for TreasuryCreditReversal {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryDebitReversal {
    #[doc = "Amount (in cents) transferred."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "The FinancialAccount to reverse funds from."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<String>,
    #[doc = "A [hosted transaction receipt](https://docs.stripe.com/treasury/moving-money/regulatory-receipts) URL that is provide..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub hosted_regulatory_receipt_url: Option<String>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "Other flows linked to a DebitReversal."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub linked_flows: Option<stripe_types_03::treasury::TreasuryReceivedDebitsResourceDebitReversalLinkedFlows>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "The rails used to reverse the funds."]
    #[json(default)]
    pub network: stripe_types_02::treasury::TreasuryDebitReversalNetwork,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_04::treasury::TreasuryDebitReversalObject,
    #[doc = "The ReceivedDebit being reversed."]
    #[json(default)]
    pub received_debit: String,
    #[doc = "Status of the DebitReversal"]
    #[json(default)]
    pub status: stripe_types_00::treasury::TreasuryDebitReversalStatus,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub status_transitions: Option<stripe_types_04::treasury::TreasuryReceivedDebitsResourceStatusTransitions>,
    #[doc = "The Transaction associated with this object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transaction: Option<crate::Expandable<crate::treasury::TreasuryTransaction>>,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl crate::PagedItem for TreasuryDebitReversal {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryInboundTransfer {
    #[doc = "Amount (in cents) transferred."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Returns `true` if the InboundTransfer is able to be canceled."]
    #[json(default)]
    pub cancelable: bool,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "An arbitrary string attached to the object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Details about this InboundTransfer's failure."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub failure_details: Option<stripe_types_05::treasury::TreasuryInboundTransfersResourceFailureDetails>,
    #[doc = "The FinancialAccount that received the funds."]
    #[json(default)]
    pub financial_account: String,
    #[doc = "A [hosted transaction receipt](https://docs.stripe.com/treasury/moving-money/regulatory-receipts) URL that is provide..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub hosted_regulatory_receipt_url: Option<String>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub linked_flows:
        Option<stripe_types_02::treasury::TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    #[json(default)]
    pub metadata: std::collections::HashMap<String, String>,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::treasury::TreasuryInboundTransferObject,
    #[doc = "The origin payment method to be debited for an InboundTransfer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub origin_payment_method: Option<String>,
    #[doc = "Details about the PaymentMethod for an InboundTransfer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub origin_payment_method_details: Option<crate::treasury::InboundTransfers>,
    #[doc = "Returns `true` if the funds for an InboundTransfer were returned after the InboundTransfer went to the `succeeded` st..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub returned: Option<bool>,
    #[doc = "Statement descriptor shown when funds are debited from the source."]
    #[json(default)]
    pub statement_descriptor: String,
    #[doc = "Status of the InboundTransfer: `processing`, `succeeded`, `failed`, and `canceled`."]
    #[json(default)]
    pub status: stripe_types_02::treasury::TreasuryInboundTransferStatus,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub status_transitions:
        Option<stripe_types_04::treasury::TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitions>,
    #[doc = "The Transaction associated with this object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transaction: Option<crate::Expandable<crate::treasury::TreasuryTransaction>>,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl crate::PagedItem for TreasuryInboundTransfer {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryOutboundPayment {
    #[doc = "Amount (in cents) transferred."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Returns `true` if the object can be canceled, and `false` otherwise."]
    #[json(default)]
    pub cancelable: bool,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "ID of the [customer](https://docs.stripe.com/api/customers) to whom an OutboundPayment is sent."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    #[doc = "An arbitrary string attached to the object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The PaymentMethod via which an OutboundPayment is sent."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub destination_payment_method: Option<String>,
    #[doc = "Details about the PaymentMethod for an OutboundPayment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub destination_payment_method_details: Option<crate::treasury::OutboundPaymentsPaymentMethodDetails>,
    #[doc = "Details about the end user."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub end_user_details:
        Option<stripe_types_04::treasury::TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetails>,
    #[doc = "The date when funds are expected to arrive in the destination account."]
    #[json(default)]
    pub expected_arrival_date: i64,
    #[doc = "The FinancialAccount that funds were pulled from."]
    #[json(default)]
    pub financial_account: String,
    #[doc = "A [hosted transaction receipt](https://docs.stripe.com/treasury/moving-money/regulatory-receipts) URL that is provide..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub hosted_regulatory_receipt_url: Option<String>,
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
    pub object: stripe_types_01::treasury::TreasuryOutboundPaymentObject,
    #[doc = "Details about a returned OutboundPayment."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub returned_details: Option<crate::treasury::TreasuryOutboundPaymentsResourceReturnedStatus>,
    #[doc = "The description that appears on the receiving end for an OutboundPayment (for example, bank statement for external ba..."]
    #[json(default)]
    pub statement_descriptor: String,
    #[doc = "Current status of the OutboundPayment: `processing`, `failed`, `posted`, `returned`, `canceled`."]
    #[json(default)]
    pub status: stripe_types_02::treasury::TreasuryOutboundPaymentStatus,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub status_transitions:
        Option<stripe_types_02::treasury::TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitions>,
    #[doc = "Details about network-specific tracking information if available."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tracking_details:
        Option<stripe_types_06::treasury::TreasuryOutboundPaymentsResourceOutboundPaymentResourceTrackingDetails>,
    #[doc = "The Transaction associated with this object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transaction: Option<crate::Expandable<crate::treasury::TreasuryTransaction>>,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl crate::PagedItem for TreasuryOutboundPayment {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryOutboundPaymentsResourceReturnedStatus {
    #[doc = "Reason for the return."]
    #[json(default)]
    pub code: stripe_types_01::treasury::TreasuryOutboundPaymentsResourceReturnedStatusCode,
    #[doc = "The Transaction associated with this object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transaction: Option<crate::Expandable<crate::treasury::TreasuryTransaction>>,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryOutboundTransfer {
    #[doc = "Amount (in cents) transferred."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Returns `true` if the object can be canceled, and `false` otherwise."]
    #[json(default)]
    pub cancelable: bool,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "An arbitrary string attached to the object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The PaymentMethod used as the payment instrument for an OutboundTransfer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub destination_payment_method: Option<String>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub destination_payment_method_details: Option<crate::treasury::OutboundTransfersPaymentMethodDetails>,
    #[doc = "The date when funds are expected to arrive in the destination account."]
    #[json(default)]
    pub expected_arrival_date: i64,
    #[doc = "The FinancialAccount that funds were pulled from."]
    #[json(default)]
    pub financial_account: String,
    #[doc = "A [hosted transaction receipt](https://docs.stripe.com/treasury/moving-money/regulatory-receipts) URL that is provide..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub hosted_regulatory_receipt_url: Option<String>,
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
    pub object: stripe_types_02::treasury::TreasuryOutboundTransferObject,
    #[doc = "Details about a returned OutboundTransfer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub returned_details: Option<crate::treasury::TreasuryOutboundTransfersResourceReturnedDetails>,
    #[doc = "Information about the OutboundTransfer to be sent to the recipient account."]
    #[json(default)]
    pub statement_descriptor: String,
    #[doc = "Current status of the OutboundTransfer: `processing`, `failed`, `canceled`, `posted`, `returned`."]
    #[json(default)]
    pub status: stripe_types_02::treasury::TreasuryOutboundPaymentStatus,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub status_transitions: Option<stripe_types_01::treasury::TreasuryOutboundTransfersResourceStatusTransitions>,
    #[doc = "Details about network-specific tracking information if available."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tracking_details:
        Option<stripe_types_06::treasury::TreasuryOutboundTransfersResourceOutboundTransferResourceTrackingDetails>,
    #[doc = "The Transaction associated with this object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transaction: Option<crate::Expandable<crate::treasury::TreasuryTransaction>>,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl crate::PagedItem for TreasuryOutboundTransfer {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryOutboundTransfersResourceReturnedDetails {
    #[doc = "Reason for the return."]
    #[json(default)]
    pub code: stripe_types_01::treasury::TreasuryOutboundPaymentsResourceReturnedStatusCode,
    #[doc = "The Transaction associated with this object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transaction: Option<crate::Expandable<crate::treasury::TreasuryTransaction>>,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryReceivedCredit {
    #[doc = "Amount (in cents) transferred."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "An arbitrary string attached to the object."]
    #[json(default)]
    pub description: String,
    #[doc = "Reason for the failure."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<stripe_types_04::treasury::TreasuryReceivedCreditFailureCode>,
    #[doc = "The FinancialAccount that received the funds."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<String>,
    #[doc = "A [hosted transaction receipt](https://docs.stripe.com/treasury/moving-money/regulatory-receipts) URL that is provide..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub hosted_regulatory_receipt_url: Option<String>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub initiating_payment_method_details: Option<
        stripe_types_07::treasury::TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails,
    >,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub linked_flows: Option<crate::treasury::TreasuryReceivedCreditsResourceLinkedFlows>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "The rails used to send the funds."]
    #[json(default)]
    pub network: stripe_types_02::treasury::TreasuryReceivedCreditNetwork,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_02::treasury::TreasuryReceivedCreditObject,
    #[doc = "Details describing when a ReceivedCredit may be reversed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reversal_details: Option<stripe_types_05::treasury::TreasuryReceivedCreditsResourceReversalDetails>,
    #[doc = "Status of the ReceivedCredit."]
    #[json(default)]
    pub status: stripe_types_00::treasury::TreasuryReceivedCreditStatus,
    #[doc = "The Transaction associated with this object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transaction: Option<crate::Expandable<crate::treasury::TreasuryTransaction>>,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl crate::PagedItem for TreasuryReceivedCredit {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryReceivedCreditsResourceLinkedFlows {
    #[doc = "The CreditReversal created as a result of this ReceivedCredit being reversed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub credit_reversal: Option<String>,
    #[doc = "Set if the ReceivedCredit was created due to an Issuing Authorization..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub issuing_authorization: Option<String>,
    #[doc = "Set if the ReceivedCredit is also viewable as an Issuing transaction..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub issuing_transaction: Option<String>,
    #[doc = "ID of the source flow."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub source_flow: Option<String>,
    #[doc = "The expandable object of the source flow."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub source_flow_details: Option<crate::treasury::TreasuryReceivedCreditsResourceSourceFlowsDetails>,
    #[doc = "The type of flow that originated the ReceivedCredit (for example, `outbound_payment`)."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub source_flow_type: Option<String>,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryReceivedCreditsResourceSourceFlowsDetails {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub credit_reversal: Option<crate::treasury::TreasuryCreditReversal>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub outbound_payment: Option<crate::treasury::TreasuryOutboundPayment>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub outbound_transfer: Option<crate::treasury::TreasuryOutboundTransfer>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payout: Option<stripe_types_14::payouts::Payout>,
    #[doc = "The type of the source flow that originated the ReceivedCredit."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::treasury::TreasuryReceivedCreditsResourceSourceFlowsDetailsType,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryReceivedDebit {
    #[doc = "Amount (in cents) transferred."]
    #[json(default)]
    pub amount: i64,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "An arbitrary string attached to the object."]
    #[json(default)]
    pub description: String,
    #[doc = "Reason for the failure."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<stripe_types_01::treasury::TreasuryReceivedDebitFailureCode>,
    #[doc = "The FinancialAccount that funds were pulled from."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<String>,
    #[doc = "A [hosted transaction receipt](https://docs.stripe.com/treasury/moving-money/regulatory-receipts) URL that is provide..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub hosted_regulatory_receipt_url: Option<String>,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub initiating_payment_method_details: Option<
        stripe_types_07::treasury::TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails,
    >,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub linked_flows: Option<stripe_types_04::treasury::TreasuryReceivedDebitsResourceLinkedFlows>,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "The network used for the ReceivedDebit."]
    #[json(default)]
    pub network: stripe_types_04::treasury::TreasuryReceivedDebitNetwork,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_02::treasury::TreasuryReceivedDebitObject,
    #[doc = "Details describing when a ReceivedDebit might be reversed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reversal_details: Option<stripe_types_06::treasury::TreasuryReceivedDebitsResourceReversalDetails>,
    #[doc = "Status of the ReceivedDebit."]
    #[json(default)]
    pub status: stripe_types_00::treasury::TreasuryReceivedCreditStatus,
    #[doc = "The Transaction associated with this object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transaction: Option<crate::Expandable<crate::treasury::TreasuryTransaction>>,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl crate::PagedItem for TreasuryReceivedDebit {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryTransaction {
    #[doc = "Amount (in cents) transferred."]
    #[json(default)]
    pub amount: i64,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub balance_impact: Option<stripe_types_03::treasury::TreasuryTransactionsResourceBalanceImpact>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "An arbitrary string attached to the object."]
    #[json(default)]
    pub description: String,
    #[doc = "A list of TransactionEntries that are part of this Transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub entries: Option<crate::List<crate::treasury::TreasuryTransactionEntry>>,
    #[doc = "The FinancialAccount associated with this object."]
    #[json(default)]
    pub financial_account: String,
    #[doc = "ID of the flow that created the Transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub flow: Option<String>,
    #[doc = "Details of the flow that created the Transaction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub flow_details: Option<crate::treasury::TreasuryTransactionsResourceFlowDetails>,
    #[doc = "Type of the flow that created the Transaction."]
    #[json(default)]
    pub flow_type: stripe_types_01::treasury::TreasuryTransactionFlowType,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_00::treasury::TreasuryTransactionObject,
    #[doc = "Status of the Transaction."]
    #[json(default)]
    pub status: stripe_types_02::treasury::TreasuryTransactionStatus,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub status_transitions:
        Option<stripe_types_03::treasury::TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions>,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl crate::PagedItem for TreasuryTransaction {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryTransactionEntry {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub balance_impact: Option<stripe_types_03::treasury::TreasuryTransactionsResourceBalanceImpact>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    #[json(default)]
    pub currency: String,
    #[doc = "When the TransactionEntry will impact the FinancialAccount's balance."]
    #[json(default)]
    pub effective_at: i64,
    #[doc = "The FinancialAccount associated with this object."]
    #[json(default)]
    pub financial_account: String,
    #[doc = "Token of the flow associated with the TransactionEntry."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub flow: Option<String>,
    #[doc = "Details of the flow associated with the TransactionEntry."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub flow_details: Option<crate::treasury::TreasuryTransactionsResourceFlowDetails>,
    #[doc = "Type of the flow associated with the TransactionEntry."]
    #[json(default)]
    pub flow_type: stripe_types_01::treasury::TreasuryTransactionFlowType,
    #[doc = "Unique identifier for the object."]
    #[json(default)]
    pub id: String,
    #[doc = "If the object exists in live mode, the value is `true`."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::treasury::TreasuryTransactionEntryObject,
    #[doc = "The Transaction associated with this object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transaction: Option<crate::Expandable<crate::treasury::TreasuryTransaction>>,
    #[doc = "The specific money movement that generated the TransactionEntry."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_00::treasury::TreasuryTransactionEntryType,
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
impl crate::PagedItem for TreasuryTransactionEntry {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(any(feature = "events", feature = "test_helpers", feature = "treasury"))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryTransactionsResourceFlowDetails {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub credit_reversal: Option<crate::treasury::TreasuryCreditReversal>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub debit_reversal: Option<crate::treasury::TreasuryDebitReversal>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub inbound_transfer: Option<crate::treasury::TreasuryInboundTransfer>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub issuing_authorization: Option<stripe_types_14::issuing::IssuingAuthorization>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub outbound_payment: Option<crate::treasury::TreasuryOutboundPayment>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub outbound_transfer: Option<crate::treasury::TreasuryOutboundTransfer>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub received_credit: Option<crate::treasury::TreasuryReceivedCredit>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub received_debit: Option<crate::treasury::TreasuryReceivedDebit>,
    #[doc = "Type of the flow that created the Transaction."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_01::treasury::TreasuryTransactionFlowType,
}
