#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct AccountExternalAccountCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::events::AccountExternalAccountCreatedObjectExpanded>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone)]
pub enum AccountExternalAccountCreatedObjectExpanded {
    /// Expanded `bank_account`.
    BankAccount(Box<stripe_types_14::payment_methods::BankAccount>),
    /// Expanded `card`.
    Card(Box<stripe_types_14::payment_methods::Card>),
    /// Expanded `source`.
    Source(Box<stripe_types_10::sources::Source>),
    /// An object type not known to this version of the crate.
    Other(json::OwnedValue),
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for AccountExternalAccountCreatedObjectExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "bank_account" => AccountExternalAccountCreatedObjectExpanded::BankAccount(Box::new(
                json::from_slice_owned(&json::to_vec(&value))?,
            )),
            "card" => AccountExternalAccountCreatedObjectExpanded::Card(Box::new(json::from_slice_owned(
                &json::to_vec(&value),
            )?)),
            "source" => AccountExternalAccountCreatedObjectExpanded::Source(Box::new(json::from_slice_owned(
                &json::to_vec(&value),
            )?)),
            _ => AccountExternalAccountCreatedObjectExpanded::Other(value),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct AccountExternalAccountDeleted {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::events::AccountExternalAccountCreatedObjectExpanded>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct AccountExternalAccountUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::events::AccountExternalAccountCreatedObjectExpanded>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct AccountUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::account::Account>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ApplicationFeeCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::application_fees::ApplicationFee>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ApplicationFeeRefundUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::common::FeeRefund>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ApplicationFeeRefunded {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::application_fees::ApplicationFee>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BillingAlertTriggered {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::billing::BillingAlertTriggered>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BillingCreditBalanceTransactionCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::billing::BillingCreditBalanceTransaction>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BillingCreditGrantCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::billing::BillingCreditGrant>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct BillingCreditGrantUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::billing::BillingCreditGrant>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CapabilityUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::accounts::Capability>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ChargeCaptured {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::charges::Charge>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ChargeDisputeClosed {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::disputes::Dispute>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ChargeDisputeCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::disputes::Dispute>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ChargeDisputeFundsReinstated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::disputes::Dispute>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ChargeDisputeFundsWithdrawn {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::disputes::Dispute>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ChargeDisputeUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::disputes::Dispute>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ChargeExpired {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::charges::Charge>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ChargeFailed {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::charges::Charge>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ChargePending {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::charges::Charge>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ChargeRefundUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::refunds::Refund>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ChargeRefunded {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::charges::Charge>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ChargeSucceeded {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::charges::Charge>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ChargeUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::charges::Charge>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutSessionAsyncPaymentFailed {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::checkout::CheckoutSession>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutSessionAsyncPaymentSucceeded {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::checkout::CheckoutSession>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutSessionCompleted {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::checkout::CheckoutSession>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CheckoutSessionExpired {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::checkout::CheckoutSession>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CreditNoteCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::credit_notes::CreditNote>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CreditNoteUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::credit_notes::CreditNote>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CreditNoteVoided {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::credit_notes::CreditNote>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerCashBalanceTransactionCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::customers::CustomerCashBalanceTransaction>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::customers::Customer>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerDeleted {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::customers::Customer>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerDiscountCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::coupons::Discount>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerDiscountDeleted {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::coupons::Discount>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerDiscountUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::coupons::Discount>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerSourceCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::events::AccountExternalAccountCreatedObjectExpanded>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerSourceDeleted {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::events::AccountExternalAccountCreatedObjectExpanded>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerSourceExpiring {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::events::CustomerSourceExpiringObjectExpanded>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone)]
pub enum CustomerSourceExpiringObjectExpanded {
    /// Expanded `card`.
    Card(Box<stripe_types_14::payment_methods::Card>),
    /// Expanded `source`.
    Source(Box<stripe_types_10::sources::Source>),
    /// An object type not known to this version of the crate.
    Other(json::OwnedValue),
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for CustomerSourceExpiringObjectExpanded {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        let raw = decoder.raw_value()?;
        let value: json::OwnedValue = json::from_slice_owned(raw.get())?;
        let tag = value.get("object").and_then(json::Value::as_str).unwrap_or("");
        Ok(match tag {
            "card" => {
                CustomerSourceExpiringObjectExpanded::Card(Box::new(json::from_slice_owned(&json::to_vec(&value))?))
            }
            "source" => {
                CustomerSourceExpiringObjectExpanded::Source(Box::new(json::from_slice_owned(&json::to_vec(&value))?))
            }
            _ => CustomerSourceExpiringObjectExpanded::Other(value),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerSourceUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::events::AccountExternalAccountCreatedObjectExpanded>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerSubscriptionCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::subscriptions::Subscription>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerSubscriptionDeleted {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::subscriptions::Subscription>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerSubscriptionPaused {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::subscriptions::Subscription>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerSubscriptionPendingUpdateApplied {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::subscriptions::Subscription>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerSubscriptionPendingUpdateExpired {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::subscriptions::Subscription>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerSubscriptionResumed {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::subscriptions::Subscription>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerSubscriptionTrialWillEnd {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::subscriptions::Subscription>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerSubscriptionUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::subscriptions::Subscription>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerTaxIdCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::tax_ids::TaxId>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerTaxIdDeleted {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::tax_ids::TaxId>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerTaxIdUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::tax_ids::TaxId>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct CustomerUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::customers::Customer>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct FinancialConnectionsAccountAccountNumbersUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::financial_connections::FinancialConnectionsAccount>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct FinancialConnectionsAccountCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::financial_connections::FinancialConnectionsAccount>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct FinancialConnectionsAccountDeactivated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::financial_connections::FinancialConnectionsAccount>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct FinancialConnectionsAccountDisconnected {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::financial_connections::FinancialConnectionsAccount>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct FinancialConnectionsAccountExpectedDeactivationDateUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::financial_connections::FinancialConnectionsAccount>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct FinancialConnectionsAccountReactivated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::financial_connections::FinancialConnectionsAccount>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct FinancialConnectionsAccountRefreshedBalance {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::financial_connections::FinancialConnectionsAccount>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct FinancialConnectionsAccountRefreshedOwnership {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::financial_connections::FinancialConnectionsAccount>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct FinancialConnectionsAccountRefreshedTransactions {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::financial_connections::FinancialConnectionsAccount>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct FinancialConnectionsAccountSupportedPaymentMethodTypesUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::financial_connections::FinancialConnectionsAccount>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct FinancialConnectionsAccountUpcomingAccountNumberExpiry {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::financial_connections::FinancialConnectionsAccount>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct FinancialConnectionsAccountUpcomingDeactivation {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::financial_connections::FinancialConnectionsAccount>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InvoiceCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::invoices::Invoice>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InvoiceDeleted {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::invoices::Invoice>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InvoiceFinalizationFailed {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::invoices::Invoice>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InvoiceFinalized {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::invoices::Invoice>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InvoiceMarkedUncollectible {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::invoices::Invoice>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InvoiceOverdue {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::invoices::Invoice>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InvoiceOverpaid {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::invoices::Invoice>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InvoicePaid {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::invoices::Invoice>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InvoicePaymentActionRequired {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::invoices::Invoice>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InvoicePaymentAttemptRequired {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::invoices::Invoice>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InvoicePaymentFailed {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::invoices::Invoice>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InvoicePaymentPaid {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::invoice_payments::InvoicePayment>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InvoicePaymentSucceeded {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::invoices::Invoice>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InvoiceSent {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::invoices::Invoice>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InvoiceUpcoming {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::invoices::Invoice>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InvoiceUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::invoices::Invoice>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InvoiceVoided {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::invoices::Invoice>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InvoiceWillBeDue {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::invoices::Invoice>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InvoiceitemCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::invoiceitems::Invoiceitem>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct InvoiceitemDeleted {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::invoiceitems::Invoiceitem>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IssuingAuthorizationCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::issuing::IssuingAuthorization>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IssuingAuthorizationRequest {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::issuing::IssuingAuthorization>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IssuingAuthorizationUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::issuing::IssuingAuthorization>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IssuingDisputeClosed {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::issuing::IssuingDispute>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IssuingDisputeCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::issuing::IssuingDispute>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IssuingDisputeFundsReinstated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::issuing::IssuingDispute>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IssuingDisputeFundsRescinded {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::issuing::IssuingDispute>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IssuingDisputeSubmitted {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::issuing::IssuingDispute>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IssuingDisputeUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::issuing::IssuingDispute>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IssuingTransactionCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::issuing::IssuingTransaction>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IssuingTransactionPurchaseDetailsReceiptUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::issuing::IssuingTransaction>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct IssuingTransactionUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::issuing::IssuingTransaction>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct MandateUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::mandates::Mandate>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentIntentAmountCapturableUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::payment_intents::PaymentIntent>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentIntentCanceled {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::payment_intents::PaymentIntent>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentIntentCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::payment_intents::PaymentIntent>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentIntentPartiallyFunded {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::payment_intents::PaymentIntent>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentIntentPaymentFailed {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::payment_intents::PaymentIntent>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentIntentProcessing {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::payment_intents::PaymentIntent>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentIntentRequiresAction {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::payment_intents::PaymentIntent>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentIntentSucceeded {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::payment_intents::PaymentIntent>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentLinkCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::payment_links::PaymentLink>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentLinkUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::payment_links::PaymentLink>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentMethodAttached {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::payment_methods::PaymentMethod>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentMethodAutomaticallyUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::payment_methods::PaymentMethod>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentMethodDetached {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::payment_methods::PaymentMethod>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PaymentMethodUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::payment_methods::PaymentMethod>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PayoutCanceled {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::payouts::Payout>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PayoutCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::payouts::Payout>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PayoutFailed {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::payouts::Payout>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PayoutPaid {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::payouts::Payout>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PayoutReconciliationCompleted {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::payouts::Payout>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PayoutUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::payouts::Payout>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PromotionCodeCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::promotion_codes::PromotionCode>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct PromotionCodeUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::promotion_codes::PromotionCode>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct QuoteAccepted {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::quotes::Quote>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct QuoteCanceled {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::quotes::Quote>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct QuoteCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::quotes::Quote>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct QuoteFinalized {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::quotes::Quote>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct RadarEarlyFraudWarningCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::radar::RadarEarlyFraudWarning>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct RadarEarlyFraudWarningUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::radar::RadarEarlyFraudWarning>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct RefundCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::refunds::Refund>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct RefundFailed {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::refunds::Refund>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct RefundUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::refunds::Refund>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ReserveHoldCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::treasury::ReserveHold>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ReserveHoldUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::treasury::ReserveHold>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ReserveReleaseCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::treasury::ReserveRelease>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ReviewClosed {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::reviews::Review>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ReviewOpened {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::reviews::Review>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SetupIntentCanceled {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::setup_intents::SetupIntent>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SetupIntentCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::setup_intents::SetupIntent>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SetupIntentRequiresAction {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::setup_intents::SetupIntent>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SetupIntentSetupFailed {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::setup_intents::SetupIntent>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SetupIntentSucceeded {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::setup_intents::SetupIntent>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SubscriptionScheduleAborted {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::subscription_schedules::SubscriptionSchedule>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SubscriptionScheduleCanceled {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::subscription_schedules::SubscriptionSchedule>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SubscriptionScheduleCompleted {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::subscription_schedules::SubscriptionSchedule>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SubscriptionScheduleCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::subscription_schedules::SubscriptionSchedule>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SubscriptionScheduleExpiring {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::subscription_schedules::SubscriptionSchedule>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SubscriptionScheduleReleased {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::subscription_schedules::SubscriptionSchedule>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct SubscriptionScheduleUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::subscription_schedules::SubscriptionSchedule>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalReaderActionFailed {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::terminal::TerminalReader>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalReaderActionSucceeded {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::terminal::TerminalReader>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TerminalReaderActionUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::terminal::TerminalReader>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TopupCanceled {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::topups::Topup>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TopupCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::topups::Topup>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TopupFailed {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::topups::Topup>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TopupReversed {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::topups::Topup>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TopupSucceeded {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::topups::Topup>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TransferCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::transfers::Transfer>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TransferReversed {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::transfers::Transfer>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TransferUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<stripe_types_14::transfers::Transfer>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryCreditReversalCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::treasury::TreasuryCreditReversal>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryCreditReversalPosted {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::treasury::TreasuryCreditReversal>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryDebitReversalCompleted {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::treasury::TreasuryDebitReversal>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryDebitReversalCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::treasury::TreasuryDebitReversal>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryDebitReversalInitialCreditGranted {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::treasury::TreasuryDebitReversal>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryInboundTransferCanceled {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::treasury::TreasuryInboundTransfer>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryInboundTransferCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::treasury::TreasuryInboundTransfer>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryInboundTransferFailed {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::treasury::TreasuryInboundTransfer>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryInboundTransferSucceeded {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::treasury::TreasuryInboundTransfer>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryOutboundPaymentCanceled {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::treasury::TreasuryOutboundPayment>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryOutboundPaymentCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::treasury::TreasuryOutboundPayment>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryOutboundPaymentExpectedArrivalDateUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::treasury::TreasuryOutboundPayment>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryOutboundPaymentFailed {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::treasury::TreasuryOutboundPayment>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryOutboundPaymentPosted {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::treasury::TreasuryOutboundPayment>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryOutboundPaymentReturned {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::treasury::TreasuryOutboundPayment>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryOutboundPaymentTrackingDetailsUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::treasury::TreasuryOutboundPayment>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryOutboundTransferCanceled {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::treasury::TreasuryOutboundTransfer>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryOutboundTransferCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::treasury::TreasuryOutboundTransfer>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryOutboundTransferExpectedArrivalDateUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::treasury::TreasuryOutboundTransfer>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryOutboundTransferFailed {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::treasury::TreasuryOutboundTransfer>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryOutboundTransferPosted {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::treasury::TreasuryOutboundTransfer>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryOutboundTransferReturned {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::treasury::TreasuryOutboundTransfer>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryOutboundTransferTrackingDetailsUpdated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::treasury::TreasuryOutboundTransfer>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryReceivedCreditCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::treasury::TreasuryReceivedCredit>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryReceivedCreditFailed {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::treasury::TreasuryReceivedCredit>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryReceivedCreditSucceeded {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::treasury::TreasuryReceivedCredit>,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct TreasuryReceivedDebitCreated {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<crate::treasury::TreasuryReceivedDebit>,
}
