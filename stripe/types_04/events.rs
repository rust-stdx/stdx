#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    AccountApplicationAuthorized,
    AccountApplicationDeauthorized,
    AccountExternalAccountCreated,
    AccountExternalAccountDeleted,
    AccountExternalAccountUpdated,
    AccountUpdated,
    ApplicationFeeCreated,
    ApplicationFeeRefundUpdated,
    ApplicationFeeRefunded,
    AppsInstallCreated,
    AppsInstallDeleted,
    AppsInstallUpdated,
    BalanceAvailable,
    BalanceSettingsUpdated,
    BillingAlertTriggered,
    BillingCreditBalanceTransactionCreated,
    BillingCreditGrantCreated,
    BillingCreditGrantUpdated,
    BillingMeterCreated,
    BillingMeterDeactivated,
    BillingMeterReactivated,
    BillingMeterUpdated,
    BillingPortalConfigurationCreated,
    BillingPortalConfigurationUpdated,
    BillingPortalSessionCreated,
    CapabilityUpdated,
    CashBalanceFundsAvailable,
    ChargeCaptured,
    ChargeDisputeClosed,
    ChargeDisputeCreated,
    ChargeDisputeFundsReinstated,
    ChargeDisputeFundsWithdrawn,
    ChargeDisputeUpdated,
    ChargeExpired,
    ChargeFailed,
    ChargePending,
    ChargeRefundUpdated,
    ChargeRefunded,
    ChargeSucceeded,
    ChargeUpdated,
    CheckoutSessionAsyncPaymentFailed,
    CheckoutSessionAsyncPaymentSucceeded,
    CheckoutSessionCompleted,
    CheckoutSessionExpired,
    ClimateOrderCanceled,
    ClimateOrderCreated,
    ClimateOrderDelayed,
    ClimateOrderDelivered,
    ClimateOrderProductSubstituted,
    ClimateProductCreated,
    ClimateProductPricingUpdated,
    CouponCreated,
    CouponDeleted,
    CouponUpdated,
    CreditNoteCreated,
    CreditNoteUpdated,
    CreditNoteVoided,
    CustomerCreated,
    CustomerDeleted,
    CustomerDiscountCreated,
    CustomerDiscountDeleted,
    CustomerDiscountUpdated,
    CustomerSourceCreated,
    CustomerSourceDeleted,
    CustomerSourceExpiring,
    CustomerSourceUpdated,
    CustomerSubscriptionCreated,
    CustomerSubscriptionDeleted,
    CustomerSubscriptionPaused,
    CustomerSubscriptionPendingUpdateApplied,
    CustomerSubscriptionPendingUpdateExpired,
    CustomerSubscriptionResumed,
    CustomerSubscriptionTrialWillEnd,
    CustomerSubscriptionUpdated,
    CustomerTaxIdCreated,
    CustomerTaxIdDeleted,
    CustomerTaxIdUpdated,
    CustomerUpdated,
    CustomerCashBalanceTransactionCreated,
    EntitlementsActiveEntitlementSummaryUpdated,
    FileCreated,
    FinancialConnectionsAccountAccountNumbersUpdated,
    FinancialConnectionsAccountCreated,
    FinancialConnectionsAccountDeactivated,
    FinancialConnectionsAccountDisconnected,
    FinancialConnectionsAccountExpectedDeactivationDateUpdated,
    FinancialConnectionsAccountReactivated,
    FinancialConnectionsAccountRefreshedBalance,
    FinancialConnectionsAccountRefreshedOwnership,
    FinancialConnectionsAccountRefreshedTransactions,
    FinancialConnectionsAccountSupportedPaymentMethodTypesUpdated,
    FinancialConnectionsAccountUpcomingAccountNumberExpiry,
    FinancialConnectionsAccountUpcomingDeactivation,
    FinancialConnectionsAuthorizationExpectedDeactivationDateUpdated,
    FinancialConnectionsAuthorizationUpcomingDeactivation,
    IdentityVerificationSessionCanceled,
    IdentityVerificationSessionCreated,
    IdentityVerificationSessionProcessing,
    IdentityVerificationSessionRedacted,
    IdentityVerificationSessionRequiresInput,
    IdentityVerificationSessionVerified,
    InvoiceCreated,
    InvoiceDeleted,
    InvoiceFinalizationFailed,
    InvoiceFinalized,
    InvoiceMarkedUncollectible,
    InvoiceOverdue,
    InvoiceOverpaid,
    InvoicePaid,
    InvoicePaymentActionRequired,
    InvoicePaymentAttemptRequired,
    InvoicePaymentFailed,
    InvoicePaymentSucceeded,
    InvoiceSent,
    InvoiceUpcoming,
    InvoiceUpdated,
    InvoiceVoided,
    InvoiceWillBeDue,
    InvoicePaymentPaid,
    InvoiceitemCreated,
    InvoiceitemDeleted,
    IssuingAuthorizationCreated,
    IssuingAuthorizationRequest,
    IssuingAuthorizationUpdated,
    IssuingCardCreated,
    IssuingCardUpdated,
    IssuingCardholderCreated,
    IssuingCardholderUpdated,
    IssuingDisputeClosed,
    IssuingDisputeCreated,
    IssuingDisputeFundsReinstated,
    IssuingDisputeFundsRescinded,
    IssuingDisputeSubmitted,
    IssuingDisputeUpdated,
    IssuingPersonalizationDesignActivated,
    IssuingPersonalizationDesignDeactivated,
    IssuingPersonalizationDesignRejected,
    IssuingPersonalizationDesignUpdated,
    IssuingTokenCreated,
    IssuingTokenUpdated,
    IssuingTransactionCreated,
    IssuingTransactionPurchaseDetailsReceiptUpdated,
    IssuingTransactionUpdated,
    MandateUpdated,
    PaymentIntentAmountCapturableUpdated,
    PaymentIntentCanceled,
    PaymentIntentCreated,
    PaymentIntentPartiallyFunded,
    PaymentIntentPaymentFailed,
    PaymentIntentProcessing,
    PaymentIntentRequiresAction,
    PaymentIntentSucceeded,
    PaymentLinkCreated,
    PaymentLinkUpdated,
    PaymentMethodAttached,
    PaymentMethodAutomaticallyUpdated,
    PaymentMethodDetached,
    PaymentMethodUpdated,
    PayoutCanceled,
    PayoutCreated,
    PayoutFailed,
    PayoutPaid,
    PayoutReconciliationCompleted,
    PayoutUpdated,
    PersonCreated,
    PersonDeleted,
    PersonUpdated,
    PlanCreated,
    PlanDeleted,
    PlanUpdated,
    PriceCreated,
    PriceDeleted,
    PriceUpdated,
    ProductCreated,
    ProductDeleted,
    ProductUpdated,
    PromotionCodeCreated,
    PromotionCodeUpdated,
    QuoteAccepted,
    QuoteCanceled,
    QuoteCreated,
    QuoteFinalized,
    RadarEarlyFraudWarningCreated,
    RadarEarlyFraudWarningUpdated,
    RefundCreated,
    RefundFailed,
    RefundUpdated,
    ReportingReportRunFailed,
    ReportingReportRunSucceeded,
    ReportingReportTypeUpdated,
    ReserveHoldCreated,
    ReserveHoldUpdated,
    ReservePlanCreated,
    ReservePlanDisabled,
    ReservePlanExpired,
    ReservePlanUpdated,
    ReserveReleaseCreated,
    ReviewClosed,
    ReviewOpened,
    SetupIntentCanceled,
    SetupIntentCreated,
    SetupIntentRequiresAction,
    SetupIntentSetupFailed,
    SetupIntentSucceeded,
    SigmaScheduledQueryRunCreated,
    SourceCanceled,
    SourceChargeable,
    SourceFailed,
    SourceMandateNotification,
    SourceRefundAttributesRequired,
    SourceTransactionCreated,
    SourceTransactionUpdated,
    SubscriptionScheduleAborted,
    SubscriptionScheduleCanceled,
    SubscriptionScheduleCompleted,
    SubscriptionScheduleCreated,
    SubscriptionScheduleExpiring,
    SubscriptionScheduleReleased,
    SubscriptionScheduleUpdated,
    TaxSettingsUpdated,
    TaxRateCreated,
    TaxRateUpdated,
    TerminalReaderActionFailed,
    TerminalReaderActionSucceeded,
    TerminalReaderActionUpdated,
    TestHelpersTestClockAdvancing,
    TestHelpersTestClockCreated,
    TestHelpersTestClockDeleted,
    TestHelpersTestClockInternalFailure,
    TestHelpersTestClockReady,
    TopupCanceled,
    TopupCreated,
    TopupFailed,
    TopupReversed,
    TopupSucceeded,
    TransferCreated,
    TransferReversed,
    TransferUpdated,
    TreasuryCreditReversalCreated,
    TreasuryCreditReversalPosted,
    TreasuryDebitReversalCompleted,
    TreasuryDebitReversalCreated,
    TreasuryDebitReversalInitialCreditGranted,
    TreasuryFinancialAccountClosed,
    TreasuryFinancialAccountCreated,
    TreasuryFinancialAccountFeaturesStatusUpdated,
    TreasuryInboundTransferCanceled,
    TreasuryInboundTransferCreated,
    TreasuryInboundTransferFailed,
    TreasuryInboundTransferSucceeded,
    TreasuryOutboundPaymentCanceled,
    TreasuryOutboundPaymentCreated,
    TreasuryOutboundPaymentExpectedArrivalDateUpdated,
    TreasuryOutboundPaymentFailed,
    TreasuryOutboundPaymentPosted,
    TreasuryOutboundPaymentReturned,
    TreasuryOutboundPaymentTrackingDetailsUpdated,
    TreasuryOutboundTransferCanceled,
    TreasuryOutboundTransferCreated,
    TreasuryOutboundTransferExpectedArrivalDateUpdated,
    TreasuryOutboundTransferFailed,
    TreasuryOutboundTransferPosted,
    TreasuryOutboundTransferReturned,
    TreasuryOutboundTransferTrackingDetailsUpdated,
    TreasuryReceivedCreditCreated,
    TreasuryReceivedCreditFailed,
    TreasuryReceivedCreditSucceeded,
    TreasuryReceivedDebitCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl EventType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            EventType::AccountApplicationAuthorized => "account.application.authorized",
            EventType::AccountApplicationDeauthorized => "account.application.deauthorized",
            EventType::AccountExternalAccountCreated => "account.external_account.created",
            EventType::AccountExternalAccountDeleted => "account.external_account.deleted",
            EventType::AccountExternalAccountUpdated => "account.external_account.updated",
            EventType::AccountUpdated => "account.updated",
            EventType::ApplicationFeeCreated => "application_fee.created",
            EventType::ApplicationFeeRefundUpdated => "application_fee.refund.updated",
            EventType::ApplicationFeeRefunded => "application_fee.refunded",
            EventType::AppsInstallCreated => "apps.install.created",
            EventType::AppsInstallDeleted => "apps.install.deleted",
            EventType::AppsInstallUpdated => "apps.install.updated",
            EventType::BalanceAvailable => "balance.available",
            EventType::BalanceSettingsUpdated => "balance_settings.updated",
            EventType::BillingAlertTriggered => "billing.alert.triggered",
            EventType::BillingCreditBalanceTransactionCreated => "billing.credit_balance_transaction.created",
            EventType::BillingCreditGrantCreated => "billing.credit_grant.created",
            EventType::BillingCreditGrantUpdated => "billing.credit_grant.updated",
            EventType::BillingMeterCreated => "billing.meter.created",
            EventType::BillingMeterDeactivated => "billing.meter.deactivated",
            EventType::BillingMeterReactivated => "billing.meter.reactivated",
            EventType::BillingMeterUpdated => "billing.meter.updated",
            EventType::BillingPortalConfigurationCreated => "billing_portal.configuration.created",
            EventType::BillingPortalConfigurationUpdated => "billing_portal.configuration.updated",
            EventType::BillingPortalSessionCreated => "billing_portal.session.created",
            EventType::CapabilityUpdated => "capability.updated",
            EventType::CashBalanceFundsAvailable => "cash_balance.funds_available",
            EventType::ChargeCaptured => "charge.captured",
            EventType::ChargeDisputeClosed => "charge.dispute.closed",
            EventType::ChargeDisputeCreated => "charge.dispute.created",
            EventType::ChargeDisputeFundsReinstated => "charge.dispute.funds_reinstated",
            EventType::ChargeDisputeFundsWithdrawn => "charge.dispute.funds_withdrawn",
            EventType::ChargeDisputeUpdated => "charge.dispute.updated",
            EventType::ChargeExpired => "charge.expired",
            EventType::ChargeFailed => "charge.failed",
            EventType::ChargePending => "charge.pending",
            EventType::ChargeRefundUpdated => "charge.refund.updated",
            EventType::ChargeRefunded => "charge.refunded",
            EventType::ChargeSucceeded => "charge.succeeded",
            EventType::ChargeUpdated => "charge.updated",
            EventType::CheckoutSessionAsyncPaymentFailed => "checkout.session.async_payment_failed",
            EventType::CheckoutSessionAsyncPaymentSucceeded => "checkout.session.async_payment_succeeded",
            EventType::CheckoutSessionCompleted => "checkout.session.completed",
            EventType::CheckoutSessionExpired => "checkout.session.expired",
            EventType::ClimateOrderCanceled => "climate.order.canceled",
            EventType::ClimateOrderCreated => "climate.order.created",
            EventType::ClimateOrderDelayed => "climate.order.delayed",
            EventType::ClimateOrderDelivered => "climate.order.delivered",
            EventType::ClimateOrderProductSubstituted => "climate.order.product_substituted",
            EventType::ClimateProductCreated => "climate.product.created",
            EventType::ClimateProductPricingUpdated => "climate.product.pricing_updated",
            EventType::CouponCreated => "coupon.created",
            EventType::CouponDeleted => "coupon.deleted",
            EventType::CouponUpdated => "coupon.updated",
            EventType::CreditNoteCreated => "credit_note.created",
            EventType::CreditNoteUpdated => "credit_note.updated",
            EventType::CreditNoteVoided => "credit_note.voided",
            EventType::CustomerCreated => "customer.created",
            EventType::CustomerDeleted => "customer.deleted",
            EventType::CustomerDiscountCreated => "customer.discount.created",
            EventType::CustomerDiscountDeleted => "customer.discount.deleted",
            EventType::CustomerDiscountUpdated => "customer.discount.updated",
            EventType::CustomerSourceCreated => "customer.source.created",
            EventType::CustomerSourceDeleted => "customer.source.deleted",
            EventType::CustomerSourceExpiring => "customer.source.expiring",
            EventType::CustomerSourceUpdated => "customer.source.updated",
            EventType::CustomerSubscriptionCreated => "customer.subscription.created",
            EventType::CustomerSubscriptionDeleted => "customer.subscription.deleted",
            EventType::CustomerSubscriptionPaused => "customer.subscription.paused",
            EventType::CustomerSubscriptionPendingUpdateApplied => "customer.subscription.pending_update_applied",
            EventType::CustomerSubscriptionPendingUpdateExpired => "customer.subscription.pending_update_expired",
            EventType::CustomerSubscriptionResumed => "customer.subscription.resumed",
            EventType::CustomerSubscriptionTrialWillEnd => "customer.subscription.trial_will_end",
            EventType::CustomerSubscriptionUpdated => "customer.subscription.updated",
            EventType::CustomerTaxIdCreated => "customer.tax_id.created",
            EventType::CustomerTaxIdDeleted => "customer.tax_id.deleted",
            EventType::CustomerTaxIdUpdated => "customer.tax_id.updated",
            EventType::CustomerUpdated => "customer.updated",
            EventType::CustomerCashBalanceTransactionCreated => "customer_cash_balance_transaction.created",
            EventType::EntitlementsActiveEntitlementSummaryUpdated => "entitlements.active_entitlement_summary.updated",
            EventType::FileCreated => "file.created",
            EventType::FinancialConnectionsAccountAccountNumbersUpdated => {
                "financial_connections.account.account_numbers_updated"
            }
            EventType::FinancialConnectionsAccountCreated => "financial_connections.account.created",
            EventType::FinancialConnectionsAccountDeactivated => "financial_connections.account.deactivated",
            EventType::FinancialConnectionsAccountDisconnected => "financial_connections.account.disconnected",
            EventType::FinancialConnectionsAccountExpectedDeactivationDateUpdated => {
                "financial_connections.account.expected_deactivation_date_updated"
            }
            EventType::FinancialConnectionsAccountReactivated => "financial_connections.account.reactivated",
            EventType::FinancialConnectionsAccountRefreshedBalance => "financial_connections.account.refreshed_balance",
            EventType::FinancialConnectionsAccountRefreshedOwnership => {
                "financial_connections.account.refreshed_ownership"
            }
            EventType::FinancialConnectionsAccountRefreshedTransactions => {
                "financial_connections.account.refreshed_transactions"
            }
            EventType::FinancialConnectionsAccountSupportedPaymentMethodTypesUpdated => {
                "financial_connections.account.supported_payment_method_types_updated"
            }
            EventType::FinancialConnectionsAccountUpcomingAccountNumberExpiry => {
                "financial_connections.account.upcoming_account_number_expiry"
            }
            EventType::FinancialConnectionsAccountUpcomingDeactivation => {
                "financial_connections.account.upcoming_deactivation"
            }
            EventType::FinancialConnectionsAuthorizationExpectedDeactivationDateUpdated => {
                "financial_connections.authorization.expected_deactivation_date_updated"
            }
            EventType::FinancialConnectionsAuthorizationUpcomingDeactivation => {
                "financial_connections.authorization.upcoming_deactivation"
            }
            EventType::IdentityVerificationSessionCanceled => "identity.verification_session.canceled",
            EventType::IdentityVerificationSessionCreated => "identity.verification_session.created",
            EventType::IdentityVerificationSessionProcessing => "identity.verification_session.processing",
            EventType::IdentityVerificationSessionRedacted => "identity.verification_session.redacted",
            EventType::IdentityVerificationSessionRequiresInput => "identity.verification_session.requires_input",
            EventType::IdentityVerificationSessionVerified => "identity.verification_session.verified",
            EventType::InvoiceCreated => "invoice.created",
            EventType::InvoiceDeleted => "invoice.deleted",
            EventType::InvoiceFinalizationFailed => "invoice.finalization_failed",
            EventType::InvoiceFinalized => "invoice.finalized",
            EventType::InvoiceMarkedUncollectible => "invoice.marked_uncollectible",
            EventType::InvoiceOverdue => "invoice.overdue",
            EventType::InvoiceOverpaid => "invoice.overpaid",
            EventType::InvoicePaid => "invoice.paid",
            EventType::InvoicePaymentActionRequired => "invoice.payment_action_required",
            EventType::InvoicePaymentAttemptRequired => "invoice.payment_attempt_required",
            EventType::InvoicePaymentFailed => "invoice.payment_failed",
            EventType::InvoicePaymentSucceeded => "invoice.payment_succeeded",
            EventType::InvoiceSent => "invoice.sent",
            EventType::InvoiceUpcoming => "invoice.upcoming",
            EventType::InvoiceUpdated => "invoice.updated",
            EventType::InvoiceVoided => "invoice.voided",
            EventType::InvoiceWillBeDue => "invoice.will_be_due",
            EventType::InvoicePaymentPaid => "invoice_payment.paid",
            EventType::InvoiceitemCreated => "invoiceitem.created",
            EventType::InvoiceitemDeleted => "invoiceitem.deleted",
            EventType::IssuingAuthorizationCreated => "issuing_authorization.created",
            EventType::IssuingAuthorizationRequest => "issuing_authorization.request",
            EventType::IssuingAuthorizationUpdated => "issuing_authorization.updated",
            EventType::IssuingCardCreated => "issuing_card.created",
            EventType::IssuingCardUpdated => "issuing_card.updated",
            EventType::IssuingCardholderCreated => "issuing_cardholder.created",
            EventType::IssuingCardholderUpdated => "issuing_cardholder.updated",
            EventType::IssuingDisputeClosed => "issuing_dispute.closed",
            EventType::IssuingDisputeCreated => "issuing_dispute.created",
            EventType::IssuingDisputeFundsReinstated => "issuing_dispute.funds_reinstated",
            EventType::IssuingDisputeFundsRescinded => "issuing_dispute.funds_rescinded",
            EventType::IssuingDisputeSubmitted => "issuing_dispute.submitted",
            EventType::IssuingDisputeUpdated => "issuing_dispute.updated",
            EventType::IssuingPersonalizationDesignActivated => "issuing_personalization_design.activated",
            EventType::IssuingPersonalizationDesignDeactivated => "issuing_personalization_design.deactivated",
            EventType::IssuingPersonalizationDesignRejected => "issuing_personalization_design.rejected",
            EventType::IssuingPersonalizationDesignUpdated => "issuing_personalization_design.updated",
            EventType::IssuingTokenCreated => "issuing_token.created",
            EventType::IssuingTokenUpdated => "issuing_token.updated",
            EventType::IssuingTransactionCreated => "issuing_transaction.created",
            EventType::IssuingTransactionPurchaseDetailsReceiptUpdated => {
                "issuing_transaction.purchase_details_receipt_updated"
            }
            EventType::IssuingTransactionUpdated => "issuing_transaction.updated",
            EventType::MandateUpdated => "mandate.updated",
            EventType::PaymentIntentAmountCapturableUpdated => "payment_intent.amount_capturable_updated",
            EventType::PaymentIntentCanceled => "payment_intent.canceled",
            EventType::PaymentIntentCreated => "payment_intent.created",
            EventType::PaymentIntentPartiallyFunded => "payment_intent.partially_funded",
            EventType::PaymentIntentPaymentFailed => "payment_intent.payment_failed",
            EventType::PaymentIntentProcessing => "payment_intent.processing",
            EventType::PaymentIntentRequiresAction => "payment_intent.requires_action",
            EventType::PaymentIntentSucceeded => "payment_intent.succeeded",
            EventType::PaymentLinkCreated => "payment_link.created",
            EventType::PaymentLinkUpdated => "payment_link.updated",
            EventType::PaymentMethodAttached => "payment_method.attached",
            EventType::PaymentMethodAutomaticallyUpdated => "payment_method.automatically_updated",
            EventType::PaymentMethodDetached => "payment_method.detached",
            EventType::PaymentMethodUpdated => "payment_method.updated",
            EventType::PayoutCanceled => "payout.canceled",
            EventType::PayoutCreated => "payout.created",
            EventType::PayoutFailed => "payout.failed",
            EventType::PayoutPaid => "payout.paid",
            EventType::PayoutReconciliationCompleted => "payout.reconciliation_completed",
            EventType::PayoutUpdated => "payout.updated",
            EventType::PersonCreated => "person.created",
            EventType::PersonDeleted => "person.deleted",
            EventType::PersonUpdated => "person.updated",
            EventType::PlanCreated => "plan.created",
            EventType::PlanDeleted => "plan.deleted",
            EventType::PlanUpdated => "plan.updated",
            EventType::PriceCreated => "price.created",
            EventType::PriceDeleted => "price.deleted",
            EventType::PriceUpdated => "price.updated",
            EventType::ProductCreated => "product.created",
            EventType::ProductDeleted => "product.deleted",
            EventType::ProductUpdated => "product.updated",
            EventType::PromotionCodeCreated => "promotion_code.created",
            EventType::PromotionCodeUpdated => "promotion_code.updated",
            EventType::QuoteAccepted => "quote.accepted",
            EventType::QuoteCanceled => "quote.canceled",
            EventType::QuoteCreated => "quote.created",
            EventType::QuoteFinalized => "quote.finalized",
            EventType::RadarEarlyFraudWarningCreated => "radar.early_fraud_warning.created",
            EventType::RadarEarlyFraudWarningUpdated => "radar.early_fraud_warning.updated",
            EventType::RefundCreated => "refund.created",
            EventType::RefundFailed => "refund.failed",
            EventType::RefundUpdated => "refund.updated",
            EventType::ReportingReportRunFailed => "reporting.report_run.failed",
            EventType::ReportingReportRunSucceeded => "reporting.report_run.succeeded",
            EventType::ReportingReportTypeUpdated => "reporting.report_type.updated",
            EventType::ReserveHoldCreated => "reserve.hold.created",
            EventType::ReserveHoldUpdated => "reserve.hold.updated",
            EventType::ReservePlanCreated => "reserve.plan.created",
            EventType::ReservePlanDisabled => "reserve.plan.disabled",
            EventType::ReservePlanExpired => "reserve.plan.expired",
            EventType::ReservePlanUpdated => "reserve.plan.updated",
            EventType::ReserveReleaseCreated => "reserve.release.created",
            EventType::ReviewClosed => "review.closed",
            EventType::ReviewOpened => "review.opened",
            EventType::SetupIntentCanceled => "setup_intent.canceled",
            EventType::SetupIntentCreated => "setup_intent.created",
            EventType::SetupIntentRequiresAction => "setup_intent.requires_action",
            EventType::SetupIntentSetupFailed => "setup_intent.setup_failed",
            EventType::SetupIntentSucceeded => "setup_intent.succeeded",
            EventType::SigmaScheduledQueryRunCreated => "sigma.scheduled_query_run.created",
            EventType::SourceCanceled => "source.canceled",
            EventType::SourceChargeable => "source.chargeable",
            EventType::SourceFailed => "source.failed",
            EventType::SourceMandateNotification => "source.mandate_notification",
            EventType::SourceRefundAttributesRequired => "source.refund_attributes_required",
            EventType::SourceTransactionCreated => "source.transaction.created",
            EventType::SourceTransactionUpdated => "source.transaction.updated",
            EventType::SubscriptionScheduleAborted => "subscription_schedule.aborted",
            EventType::SubscriptionScheduleCanceled => "subscription_schedule.canceled",
            EventType::SubscriptionScheduleCompleted => "subscription_schedule.completed",
            EventType::SubscriptionScheduleCreated => "subscription_schedule.created",
            EventType::SubscriptionScheduleExpiring => "subscription_schedule.expiring",
            EventType::SubscriptionScheduleReleased => "subscription_schedule.released",
            EventType::SubscriptionScheduleUpdated => "subscription_schedule.updated",
            EventType::TaxSettingsUpdated => "tax.settings.updated",
            EventType::TaxRateCreated => "tax_rate.created",
            EventType::TaxRateUpdated => "tax_rate.updated",
            EventType::TerminalReaderActionFailed => "terminal.reader.action_failed",
            EventType::TerminalReaderActionSucceeded => "terminal.reader.action_succeeded",
            EventType::TerminalReaderActionUpdated => "terminal.reader.action_updated",
            EventType::TestHelpersTestClockAdvancing => "test_helpers.test_clock.advancing",
            EventType::TestHelpersTestClockCreated => "test_helpers.test_clock.created",
            EventType::TestHelpersTestClockDeleted => "test_helpers.test_clock.deleted",
            EventType::TestHelpersTestClockInternalFailure => "test_helpers.test_clock.internal_failure",
            EventType::TestHelpersTestClockReady => "test_helpers.test_clock.ready",
            EventType::TopupCanceled => "topup.canceled",
            EventType::TopupCreated => "topup.created",
            EventType::TopupFailed => "topup.failed",
            EventType::TopupReversed => "topup.reversed",
            EventType::TopupSucceeded => "topup.succeeded",
            EventType::TransferCreated => "transfer.created",
            EventType::TransferReversed => "transfer.reversed",
            EventType::TransferUpdated => "transfer.updated",
            EventType::TreasuryCreditReversalCreated => "treasury.credit_reversal.created",
            EventType::TreasuryCreditReversalPosted => "treasury.credit_reversal.posted",
            EventType::TreasuryDebitReversalCompleted => "treasury.debit_reversal.completed",
            EventType::TreasuryDebitReversalCreated => "treasury.debit_reversal.created",
            EventType::TreasuryDebitReversalInitialCreditGranted => "treasury.debit_reversal.initial_credit_granted",
            EventType::TreasuryFinancialAccountClosed => "treasury.financial_account.closed",
            EventType::TreasuryFinancialAccountCreated => "treasury.financial_account.created",
            EventType::TreasuryFinancialAccountFeaturesStatusUpdated => {
                "treasury.financial_account.features_status_updated"
            }
            EventType::TreasuryInboundTransferCanceled => "treasury.inbound_transfer.canceled",
            EventType::TreasuryInboundTransferCreated => "treasury.inbound_transfer.created",
            EventType::TreasuryInboundTransferFailed => "treasury.inbound_transfer.failed",
            EventType::TreasuryInboundTransferSucceeded => "treasury.inbound_transfer.succeeded",
            EventType::TreasuryOutboundPaymentCanceled => "treasury.outbound_payment.canceled",
            EventType::TreasuryOutboundPaymentCreated => "treasury.outbound_payment.created",
            EventType::TreasuryOutboundPaymentExpectedArrivalDateUpdated => {
                "treasury.outbound_payment.expected_arrival_date_updated"
            }
            EventType::TreasuryOutboundPaymentFailed => "treasury.outbound_payment.failed",
            EventType::TreasuryOutboundPaymentPosted => "treasury.outbound_payment.posted",
            EventType::TreasuryOutboundPaymentReturned => "treasury.outbound_payment.returned",
            EventType::TreasuryOutboundPaymentTrackingDetailsUpdated => {
                "treasury.outbound_payment.tracking_details_updated"
            }
            EventType::TreasuryOutboundTransferCanceled => "treasury.outbound_transfer.canceled",
            EventType::TreasuryOutboundTransferCreated => "treasury.outbound_transfer.created",
            EventType::TreasuryOutboundTransferExpectedArrivalDateUpdated => {
                "treasury.outbound_transfer.expected_arrival_date_updated"
            }
            EventType::TreasuryOutboundTransferFailed => "treasury.outbound_transfer.failed",
            EventType::TreasuryOutboundTransferPosted => "treasury.outbound_transfer.posted",
            EventType::TreasuryOutboundTransferReturned => "treasury.outbound_transfer.returned",
            EventType::TreasuryOutboundTransferTrackingDetailsUpdated => {
                "treasury.outbound_transfer.tracking_details_updated"
            }
            EventType::TreasuryReceivedCreditCreated => "treasury.received_credit.created",
            EventType::TreasuryReceivedCreditFailed => "treasury.received_credit.failed",
            EventType::TreasuryReceivedCreditSucceeded => "treasury.received_credit.succeeded",
            EventType::TreasuryReceivedDebitCreated => "treasury.received_debit.created",
            EventType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for EventType {
    fn default() -> Self {
        EventType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for EventType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(EventType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account.application.authorized" => EventType::AccountApplicationAuthorized,
            "account.application.deauthorized" => EventType::AccountApplicationDeauthorized,
            "account.external_account.created" => EventType::AccountExternalAccountCreated,
            "account.external_account.deleted" => EventType::AccountExternalAccountDeleted,
            "account.external_account.updated" => EventType::AccountExternalAccountUpdated,
            "account.updated" => EventType::AccountUpdated,
            "application_fee.created" => EventType::ApplicationFeeCreated,
            "application_fee.refund.updated" => EventType::ApplicationFeeRefundUpdated,
            "application_fee.refunded" => EventType::ApplicationFeeRefunded,
            "apps.install.created" => EventType::AppsInstallCreated,
            "apps.install.deleted" => EventType::AppsInstallDeleted,
            "apps.install.updated" => EventType::AppsInstallUpdated,
            "balance.available" => EventType::BalanceAvailable,
            "balance_settings.updated" => EventType::BalanceSettingsUpdated,
            "billing.alert.triggered" => EventType::BillingAlertTriggered,
            "billing.credit_balance_transaction.created" => EventType::BillingCreditBalanceTransactionCreated,
            "billing.credit_grant.created" => EventType::BillingCreditGrantCreated,
            "billing.credit_grant.updated" => EventType::BillingCreditGrantUpdated,
            "billing.meter.created" => EventType::BillingMeterCreated,
            "billing.meter.deactivated" => EventType::BillingMeterDeactivated,
            "billing.meter.reactivated" => EventType::BillingMeterReactivated,
            "billing.meter.updated" => EventType::BillingMeterUpdated,
            "billing_portal.configuration.created" => EventType::BillingPortalConfigurationCreated,
            "billing_portal.configuration.updated" => EventType::BillingPortalConfigurationUpdated,
            "billing_portal.session.created" => EventType::BillingPortalSessionCreated,
            "capability.updated" => EventType::CapabilityUpdated,
            "cash_balance.funds_available" => EventType::CashBalanceFundsAvailable,
            "charge.captured" => EventType::ChargeCaptured,
            "charge.dispute.closed" => EventType::ChargeDisputeClosed,
            "charge.dispute.created" => EventType::ChargeDisputeCreated,
            "charge.dispute.funds_reinstated" => EventType::ChargeDisputeFundsReinstated,
            "charge.dispute.funds_withdrawn" => EventType::ChargeDisputeFundsWithdrawn,
            "charge.dispute.updated" => EventType::ChargeDisputeUpdated,
            "charge.expired" => EventType::ChargeExpired,
            "charge.failed" => EventType::ChargeFailed,
            "charge.pending" => EventType::ChargePending,
            "charge.refund.updated" => EventType::ChargeRefundUpdated,
            "charge.refunded" => EventType::ChargeRefunded,
            "charge.succeeded" => EventType::ChargeSucceeded,
            "charge.updated" => EventType::ChargeUpdated,
            "checkout.session.async_payment_failed" => EventType::CheckoutSessionAsyncPaymentFailed,
            "checkout.session.async_payment_succeeded" => EventType::CheckoutSessionAsyncPaymentSucceeded,
            "checkout.session.completed" => EventType::CheckoutSessionCompleted,
            "checkout.session.expired" => EventType::CheckoutSessionExpired,
            "climate.order.canceled" => EventType::ClimateOrderCanceled,
            "climate.order.created" => EventType::ClimateOrderCreated,
            "climate.order.delayed" => EventType::ClimateOrderDelayed,
            "climate.order.delivered" => EventType::ClimateOrderDelivered,
            "climate.order.product_substituted" => EventType::ClimateOrderProductSubstituted,
            "climate.product.created" => EventType::ClimateProductCreated,
            "climate.product.pricing_updated" => EventType::ClimateProductPricingUpdated,
            "coupon.created" => EventType::CouponCreated,
            "coupon.deleted" => EventType::CouponDeleted,
            "coupon.updated" => EventType::CouponUpdated,
            "credit_note.created" => EventType::CreditNoteCreated,
            "credit_note.updated" => EventType::CreditNoteUpdated,
            "credit_note.voided" => EventType::CreditNoteVoided,
            "customer.created" => EventType::CustomerCreated,
            "customer.deleted" => EventType::CustomerDeleted,
            "customer.discount.created" => EventType::CustomerDiscountCreated,
            "customer.discount.deleted" => EventType::CustomerDiscountDeleted,
            "customer.discount.updated" => EventType::CustomerDiscountUpdated,
            "customer.source.created" => EventType::CustomerSourceCreated,
            "customer.source.deleted" => EventType::CustomerSourceDeleted,
            "customer.source.expiring" => EventType::CustomerSourceExpiring,
            "customer.source.updated" => EventType::CustomerSourceUpdated,
            "customer.subscription.created" => EventType::CustomerSubscriptionCreated,
            "customer.subscription.deleted" => EventType::CustomerSubscriptionDeleted,
            "customer.subscription.paused" => EventType::CustomerSubscriptionPaused,
            "customer.subscription.pending_update_applied" => EventType::CustomerSubscriptionPendingUpdateApplied,
            "customer.subscription.pending_update_expired" => EventType::CustomerSubscriptionPendingUpdateExpired,
            "customer.subscription.resumed" => EventType::CustomerSubscriptionResumed,
            "customer.subscription.trial_will_end" => EventType::CustomerSubscriptionTrialWillEnd,
            "customer.subscription.updated" => EventType::CustomerSubscriptionUpdated,
            "customer.tax_id.created" => EventType::CustomerTaxIdCreated,
            "customer.tax_id.deleted" => EventType::CustomerTaxIdDeleted,
            "customer.tax_id.updated" => EventType::CustomerTaxIdUpdated,
            "customer.updated" => EventType::CustomerUpdated,
            "customer_cash_balance_transaction.created" => EventType::CustomerCashBalanceTransactionCreated,
            "entitlements.active_entitlement_summary.updated" => EventType::EntitlementsActiveEntitlementSummaryUpdated,
            "file.created" => EventType::FileCreated,
            "financial_connections.account.account_numbers_updated" => {
                EventType::FinancialConnectionsAccountAccountNumbersUpdated
            }
            "financial_connections.account.created" => EventType::FinancialConnectionsAccountCreated,
            "financial_connections.account.deactivated" => EventType::FinancialConnectionsAccountDeactivated,
            "financial_connections.account.disconnected" => EventType::FinancialConnectionsAccountDisconnected,
            "financial_connections.account.expected_deactivation_date_updated" => {
                EventType::FinancialConnectionsAccountExpectedDeactivationDateUpdated
            }
            "financial_connections.account.reactivated" => EventType::FinancialConnectionsAccountReactivated,
            "financial_connections.account.refreshed_balance" => EventType::FinancialConnectionsAccountRefreshedBalance,
            "financial_connections.account.refreshed_ownership" => {
                EventType::FinancialConnectionsAccountRefreshedOwnership
            }
            "financial_connections.account.refreshed_transactions" => {
                EventType::FinancialConnectionsAccountRefreshedTransactions
            }
            "financial_connections.account.supported_payment_method_types_updated" => {
                EventType::FinancialConnectionsAccountSupportedPaymentMethodTypesUpdated
            }
            "financial_connections.account.upcoming_account_number_expiry" => {
                EventType::FinancialConnectionsAccountUpcomingAccountNumberExpiry
            }
            "financial_connections.account.upcoming_deactivation" => {
                EventType::FinancialConnectionsAccountUpcomingDeactivation
            }
            "financial_connections.authorization.expected_deactivation_date_updated" => {
                EventType::FinancialConnectionsAuthorizationExpectedDeactivationDateUpdated
            }
            "financial_connections.authorization.upcoming_deactivation" => {
                EventType::FinancialConnectionsAuthorizationUpcomingDeactivation
            }
            "identity.verification_session.canceled" => EventType::IdentityVerificationSessionCanceled,
            "identity.verification_session.created" => EventType::IdentityVerificationSessionCreated,
            "identity.verification_session.processing" => EventType::IdentityVerificationSessionProcessing,
            "identity.verification_session.redacted" => EventType::IdentityVerificationSessionRedacted,
            "identity.verification_session.requires_input" => EventType::IdentityVerificationSessionRequiresInput,
            "identity.verification_session.verified" => EventType::IdentityVerificationSessionVerified,
            "invoice.created" => EventType::InvoiceCreated,
            "invoice.deleted" => EventType::InvoiceDeleted,
            "invoice.finalization_failed" => EventType::InvoiceFinalizationFailed,
            "invoice.finalized" => EventType::InvoiceFinalized,
            "invoice.marked_uncollectible" => EventType::InvoiceMarkedUncollectible,
            "invoice.overdue" => EventType::InvoiceOverdue,
            "invoice.overpaid" => EventType::InvoiceOverpaid,
            "invoice.paid" => EventType::InvoicePaid,
            "invoice.payment_action_required" => EventType::InvoicePaymentActionRequired,
            "invoice.payment_attempt_required" => EventType::InvoicePaymentAttemptRequired,
            "invoice.payment_failed" => EventType::InvoicePaymentFailed,
            "invoice.payment_succeeded" => EventType::InvoicePaymentSucceeded,
            "invoice.sent" => EventType::InvoiceSent,
            "invoice.upcoming" => EventType::InvoiceUpcoming,
            "invoice.updated" => EventType::InvoiceUpdated,
            "invoice.voided" => EventType::InvoiceVoided,
            "invoice.will_be_due" => EventType::InvoiceWillBeDue,
            "invoice_payment.paid" => EventType::InvoicePaymentPaid,
            "invoiceitem.created" => EventType::InvoiceitemCreated,
            "invoiceitem.deleted" => EventType::InvoiceitemDeleted,
            "issuing_authorization.created" => EventType::IssuingAuthorizationCreated,
            "issuing_authorization.request" => EventType::IssuingAuthorizationRequest,
            "issuing_authorization.updated" => EventType::IssuingAuthorizationUpdated,
            "issuing_card.created" => EventType::IssuingCardCreated,
            "issuing_card.updated" => EventType::IssuingCardUpdated,
            "issuing_cardholder.created" => EventType::IssuingCardholderCreated,
            "issuing_cardholder.updated" => EventType::IssuingCardholderUpdated,
            "issuing_dispute.closed" => EventType::IssuingDisputeClosed,
            "issuing_dispute.created" => EventType::IssuingDisputeCreated,
            "issuing_dispute.funds_reinstated" => EventType::IssuingDisputeFundsReinstated,
            "issuing_dispute.funds_rescinded" => EventType::IssuingDisputeFundsRescinded,
            "issuing_dispute.submitted" => EventType::IssuingDisputeSubmitted,
            "issuing_dispute.updated" => EventType::IssuingDisputeUpdated,
            "issuing_personalization_design.activated" => EventType::IssuingPersonalizationDesignActivated,
            "issuing_personalization_design.deactivated" => EventType::IssuingPersonalizationDesignDeactivated,
            "issuing_personalization_design.rejected" => EventType::IssuingPersonalizationDesignRejected,
            "issuing_personalization_design.updated" => EventType::IssuingPersonalizationDesignUpdated,
            "issuing_token.created" => EventType::IssuingTokenCreated,
            "issuing_token.updated" => EventType::IssuingTokenUpdated,
            "issuing_transaction.created" => EventType::IssuingTransactionCreated,
            "issuing_transaction.purchase_details_receipt_updated" => {
                EventType::IssuingTransactionPurchaseDetailsReceiptUpdated
            }
            "issuing_transaction.updated" => EventType::IssuingTransactionUpdated,
            "mandate.updated" => EventType::MandateUpdated,
            "payment_intent.amount_capturable_updated" => EventType::PaymentIntentAmountCapturableUpdated,
            "payment_intent.canceled" => EventType::PaymentIntentCanceled,
            "payment_intent.created" => EventType::PaymentIntentCreated,
            "payment_intent.partially_funded" => EventType::PaymentIntentPartiallyFunded,
            "payment_intent.payment_failed" => EventType::PaymentIntentPaymentFailed,
            "payment_intent.processing" => EventType::PaymentIntentProcessing,
            "payment_intent.requires_action" => EventType::PaymentIntentRequiresAction,
            "payment_intent.succeeded" => EventType::PaymentIntentSucceeded,
            "payment_link.created" => EventType::PaymentLinkCreated,
            "payment_link.updated" => EventType::PaymentLinkUpdated,
            "payment_method.attached" => EventType::PaymentMethodAttached,
            "payment_method.automatically_updated" => EventType::PaymentMethodAutomaticallyUpdated,
            "payment_method.detached" => EventType::PaymentMethodDetached,
            "payment_method.updated" => EventType::PaymentMethodUpdated,
            "payout.canceled" => EventType::PayoutCanceled,
            "payout.created" => EventType::PayoutCreated,
            "payout.failed" => EventType::PayoutFailed,
            "payout.paid" => EventType::PayoutPaid,
            "payout.reconciliation_completed" => EventType::PayoutReconciliationCompleted,
            "payout.updated" => EventType::PayoutUpdated,
            "person.created" => EventType::PersonCreated,
            "person.deleted" => EventType::PersonDeleted,
            "person.updated" => EventType::PersonUpdated,
            "plan.created" => EventType::PlanCreated,
            "plan.deleted" => EventType::PlanDeleted,
            "plan.updated" => EventType::PlanUpdated,
            "price.created" => EventType::PriceCreated,
            "price.deleted" => EventType::PriceDeleted,
            "price.updated" => EventType::PriceUpdated,
            "product.created" => EventType::ProductCreated,
            "product.deleted" => EventType::ProductDeleted,
            "product.updated" => EventType::ProductUpdated,
            "promotion_code.created" => EventType::PromotionCodeCreated,
            "promotion_code.updated" => EventType::PromotionCodeUpdated,
            "quote.accepted" => EventType::QuoteAccepted,
            "quote.canceled" => EventType::QuoteCanceled,
            "quote.created" => EventType::QuoteCreated,
            "quote.finalized" => EventType::QuoteFinalized,
            "radar.early_fraud_warning.created" => EventType::RadarEarlyFraudWarningCreated,
            "radar.early_fraud_warning.updated" => EventType::RadarEarlyFraudWarningUpdated,
            "refund.created" => EventType::RefundCreated,
            "refund.failed" => EventType::RefundFailed,
            "refund.updated" => EventType::RefundUpdated,
            "reporting.report_run.failed" => EventType::ReportingReportRunFailed,
            "reporting.report_run.succeeded" => EventType::ReportingReportRunSucceeded,
            "reporting.report_type.updated" => EventType::ReportingReportTypeUpdated,
            "reserve.hold.created" => EventType::ReserveHoldCreated,
            "reserve.hold.updated" => EventType::ReserveHoldUpdated,
            "reserve.plan.created" => EventType::ReservePlanCreated,
            "reserve.plan.disabled" => EventType::ReservePlanDisabled,
            "reserve.plan.expired" => EventType::ReservePlanExpired,
            "reserve.plan.updated" => EventType::ReservePlanUpdated,
            "reserve.release.created" => EventType::ReserveReleaseCreated,
            "review.closed" => EventType::ReviewClosed,
            "review.opened" => EventType::ReviewOpened,
            "setup_intent.canceled" => EventType::SetupIntentCanceled,
            "setup_intent.created" => EventType::SetupIntentCreated,
            "setup_intent.requires_action" => EventType::SetupIntentRequiresAction,
            "setup_intent.setup_failed" => EventType::SetupIntentSetupFailed,
            "setup_intent.succeeded" => EventType::SetupIntentSucceeded,
            "sigma.scheduled_query_run.created" => EventType::SigmaScheduledQueryRunCreated,
            "source.canceled" => EventType::SourceCanceled,
            "source.chargeable" => EventType::SourceChargeable,
            "source.failed" => EventType::SourceFailed,
            "source.mandate_notification" => EventType::SourceMandateNotification,
            "source.refund_attributes_required" => EventType::SourceRefundAttributesRequired,
            "source.transaction.created" => EventType::SourceTransactionCreated,
            "source.transaction.updated" => EventType::SourceTransactionUpdated,
            "subscription_schedule.aborted" => EventType::SubscriptionScheduleAborted,
            "subscription_schedule.canceled" => EventType::SubscriptionScheduleCanceled,
            "subscription_schedule.completed" => EventType::SubscriptionScheduleCompleted,
            "subscription_schedule.created" => EventType::SubscriptionScheduleCreated,
            "subscription_schedule.expiring" => EventType::SubscriptionScheduleExpiring,
            "subscription_schedule.released" => EventType::SubscriptionScheduleReleased,
            "subscription_schedule.updated" => EventType::SubscriptionScheduleUpdated,
            "tax.settings.updated" => EventType::TaxSettingsUpdated,
            "tax_rate.created" => EventType::TaxRateCreated,
            "tax_rate.updated" => EventType::TaxRateUpdated,
            "terminal.reader.action_failed" => EventType::TerminalReaderActionFailed,
            "terminal.reader.action_succeeded" => EventType::TerminalReaderActionSucceeded,
            "terminal.reader.action_updated" => EventType::TerminalReaderActionUpdated,
            "test_helpers.test_clock.advancing" => EventType::TestHelpersTestClockAdvancing,
            "test_helpers.test_clock.created" => EventType::TestHelpersTestClockCreated,
            "test_helpers.test_clock.deleted" => EventType::TestHelpersTestClockDeleted,
            "test_helpers.test_clock.internal_failure" => EventType::TestHelpersTestClockInternalFailure,
            "test_helpers.test_clock.ready" => EventType::TestHelpersTestClockReady,
            "topup.canceled" => EventType::TopupCanceled,
            "topup.created" => EventType::TopupCreated,
            "topup.failed" => EventType::TopupFailed,
            "topup.reversed" => EventType::TopupReversed,
            "topup.succeeded" => EventType::TopupSucceeded,
            "transfer.created" => EventType::TransferCreated,
            "transfer.reversed" => EventType::TransferReversed,
            "transfer.updated" => EventType::TransferUpdated,
            "treasury.credit_reversal.created" => EventType::TreasuryCreditReversalCreated,
            "treasury.credit_reversal.posted" => EventType::TreasuryCreditReversalPosted,
            "treasury.debit_reversal.completed" => EventType::TreasuryDebitReversalCompleted,
            "treasury.debit_reversal.created" => EventType::TreasuryDebitReversalCreated,
            "treasury.debit_reversal.initial_credit_granted" => EventType::TreasuryDebitReversalInitialCreditGranted,
            "treasury.financial_account.closed" => EventType::TreasuryFinancialAccountClosed,
            "treasury.financial_account.created" => EventType::TreasuryFinancialAccountCreated,
            "treasury.financial_account.features_status_updated" => {
                EventType::TreasuryFinancialAccountFeaturesStatusUpdated
            }
            "treasury.inbound_transfer.canceled" => EventType::TreasuryInboundTransferCanceled,
            "treasury.inbound_transfer.created" => EventType::TreasuryInboundTransferCreated,
            "treasury.inbound_transfer.failed" => EventType::TreasuryInboundTransferFailed,
            "treasury.inbound_transfer.succeeded" => EventType::TreasuryInboundTransferSucceeded,
            "treasury.outbound_payment.canceled" => EventType::TreasuryOutboundPaymentCanceled,
            "treasury.outbound_payment.created" => EventType::TreasuryOutboundPaymentCreated,
            "treasury.outbound_payment.expected_arrival_date_updated" => {
                EventType::TreasuryOutboundPaymentExpectedArrivalDateUpdated
            }
            "treasury.outbound_payment.failed" => EventType::TreasuryOutboundPaymentFailed,
            "treasury.outbound_payment.posted" => EventType::TreasuryOutboundPaymentPosted,
            "treasury.outbound_payment.returned" => EventType::TreasuryOutboundPaymentReturned,
            "treasury.outbound_payment.tracking_details_updated" => {
                EventType::TreasuryOutboundPaymentTrackingDetailsUpdated
            }
            "treasury.outbound_transfer.canceled" => EventType::TreasuryOutboundTransferCanceled,
            "treasury.outbound_transfer.created" => EventType::TreasuryOutboundTransferCreated,
            "treasury.outbound_transfer.expected_arrival_date_updated" => {
                EventType::TreasuryOutboundTransferExpectedArrivalDateUpdated
            }
            "treasury.outbound_transfer.failed" => EventType::TreasuryOutboundTransferFailed,
            "treasury.outbound_transfer.posted" => EventType::TreasuryOutboundTransferPosted,
            "treasury.outbound_transfer.returned" => EventType::TreasuryOutboundTransferReturned,
            "treasury.outbound_transfer.tracking_details_updated" => {
                EventType::TreasuryOutboundTransferTrackingDetailsUpdated
            }
            "treasury.received_credit.created" => EventType::TreasuryReceivedCreditCreated,
            "treasury.received_credit.failed" => EventType::TreasuryReceivedCreditFailed,
            "treasury.received_credit.succeeded" => EventType::TreasuryReceivedCreditSucceeded,
            "treasury.received_debit.created" => EventType::TreasuryReceivedDebitCreated,
            other => EventType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, Default)]
pub struct ListEventsParamsCreated {
    #[doc = "Minimum value to filter by (exclusive)"]
    pub gt: Option<i64>,
    #[doc = "Minimum value to filter by (inclusive)"]
    pub gte: Option<i64>,
    #[doc = "Maximum value to filter by (exclusive)"]
    pub lt: Option<i64>,
    #[doc = "Maximum value to filter by (inclusive)"]
    pub lte: Option<i64>,
}

#[cfg(feature = "events")]
impl crate::params::FormEncode for ListEventsParamsCreated {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("gt", self.gt.as_ref());
        form.field_opt("gte", self.gte.as_ref());
        form.field_opt("lt", self.lt.as_ref());
        form.field_opt("lte", self.lte.as_ref());
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1AccountApplicationAuthorizedReasonRequest {
    #[doc = "ID of the API request that caused the event."]
    #[json(default)]
    pub id: String,
    #[doc = "The idempotency key transmitted during the request."]
    #[json(default)]
    pub idempotency_key: String,
}

#[cfg(feature = "events")]
impl crate::PagedItem for V1AccountApplicationAuthorizedReasonRequest {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1AccountExternalAccountCreatedType {
    V1AccountExternalAccountCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1AccountExternalAccountCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1AccountExternalAccountCreatedType::V1AccountExternalAccountCreated => {
                "v1.account.external_account.created"
            }
            V1AccountExternalAccountCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1AccountExternalAccountCreatedType {
    fn default() -> Self {
        V1AccountExternalAccountCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1AccountExternalAccountCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1AccountExternalAccountCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.account.external_account.created" => {
                V1AccountExternalAccountCreatedType::V1AccountExternalAccountCreated
            }
            other => V1AccountExternalAccountCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1AccountUpdatedRelatedObjectType {
    Account,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1AccountUpdatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1AccountUpdatedRelatedObjectType::Account => "account",
            V1AccountUpdatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1AccountUpdatedRelatedObjectType {
    fn default() -> Self {
        V1AccountUpdatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1AccountUpdatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1AccountUpdatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "account" => V1AccountUpdatedRelatedObjectType::Account,
            other => V1AccountUpdatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1BalanceSettingsUpdatedRelatedObjectType {
    BalanceSettings,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1BalanceSettingsUpdatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1BalanceSettingsUpdatedRelatedObjectType::BalanceSettings => "balance_settings",
            V1BalanceSettingsUpdatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1BalanceSettingsUpdatedRelatedObjectType {
    fn default() -> Self {
        V1BalanceSettingsUpdatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1BalanceSettingsUpdatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1BalanceSettingsUpdatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "balance_settings" => V1BalanceSettingsUpdatedRelatedObjectType::BalanceSettings,
            other => V1BalanceSettingsUpdatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1BillingAlertTriggeredType {
    V1BillingAlertTriggered,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1BillingAlertTriggeredType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1BillingAlertTriggeredType::V1BillingAlertTriggered => "v1.billing.alert.triggered",
            V1BillingAlertTriggeredType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1BillingAlertTriggeredType {
    fn default() -> Self {
        V1BillingAlertTriggeredType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1BillingAlertTriggeredType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1BillingAlertTriggeredType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.billing.alert.triggered" => V1BillingAlertTriggeredType::V1BillingAlertTriggered,
            other => V1BillingAlertTriggeredType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1BillingCreditBalanceTransactionCreatedRelatedObjectType {
    BillingCreditBalanceTransaction,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1BillingCreditBalanceTransactionCreatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1BillingCreditBalanceTransactionCreatedRelatedObjectType::BillingCreditBalanceTransaction => {
                "billing.credit_balance_transaction"
            }
            V1BillingCreditBalanceTransactionCreatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1BillingCreditBalanceTransactionCreatedRelatedObjectType {
    fn default() -> Self {
        V1BillingCreditBalanceTransactionCreatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1BillingCreditBalanceTransactionCreatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1BillingCreditBalanceTransactionCreatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "billing.credit_balance_transaction" => {
                V1BillingCreditBalanceTransactionCreatedRelatedObjectType::BillingCreditBalanceTransaction
            }
            other => V1BillingCreditBalanceTransactionCreatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1BillingCreditGrantCreatedRelatedObjectType {
    BillingCreditGrant,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1BillingCreditGrantCreatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1BillingCreditGrantCreatedRelatedObjectType::BillingCreditGrant => "billing.credit_grant",
            V1BillingCreditGrantCreatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1BillingCreditGrantCreatedRelatedObjectType {
    fn default() -> Self {
        V1BillingCreditGrantCreatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1BillingCreditGrantCreatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1BillingCreditGrantCreatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "billing.credit_grant" => V1BillingCreditGrantCreatedRelatedObjectType::BillingCreditGrant,
            other => V1BillingCreditGrantCreatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1BillingMeterCreatedRelatedObjectType {
    BillingMeter,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1BillingMeterCreatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1BillingMeterCreatedRelatedObjectType::BillingMeter => "billing.meter",
            V1BillingMeterCreatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1BillingMeterCreatedRelatedObjectType {
    fn default() -> Self {
        V1BillingMeterCreatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1BillingMeterCreatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1BillingMeterCreatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "billing.meter" => V1BillingMeterCreatedRelatedObjectType::BillingMeter,
            other => V1BillingMeterCreatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V1BillingMeterErrorReportTriggeredDataReasonErrorTypesItemSampleErrorsItemRequest {
    #[doc = "The request idempotency key."]
    #[json(default)]
    pub identifier: String,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CapabilityUpdatedRelatedObjectType {
    Capability,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CapabilityUpdatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CapabilityUpdatedRelatedObjectType::Capability => "capability",
            V1CapabilityUpdatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CapabilityUpdatedRelatedObjectType {
    fn default() -> Self {
        V1CapabilityUpdatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CapabilityUpdatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CapabilityUpdatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "capability" => V1CapabilityUpdatedRelatedObjectType::Capability,
            other => V1CapabilityUpdatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CapabilityUpdatedType {
    V1CapabilityUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CapabilityUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CapabilityUpdatedType::V1CapabilityUpdated => "v1.capability.updated",
            V1CapabilityUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CapabilityUpdatedType {
    fn default() -> Self {
        V1CapabilityUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CapabilityUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CapabilityUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.capability.updated" => V1CapabilityUpdatedType::V1CapabilityUpdated,
            other => V1CapabilityUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ChargeDisputeFundsReinstatedType {
    V1ChargeDisputeFundsReinstated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ChargeDisputeFundsReinstatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ChargeDisputeFundsReinstatedType::V1ChargeDisputeFundsReinstated => "v1.charge.dispute.funds_reinstated",
            V1ChargeDisputeFundsReinstatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ChargeDisputeFundsReinstatedType {
    fn default() -> Self {
        V1ChargeDisputeFundsReinstatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ChargeDisputeFundsReinstatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ChargeDisputeFundsReinstatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.charge.dispute.funds_reinstated" => V1ChargeDisputeFundsReinstatedType::V1ChargeDisputeFundsReinstated,
            other => V1ChargeDisputeFundsReinstatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ChargeDisputeFundsWithdrawnType {
    V1ChargeDisputeFundsWithdrawn,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ChargeDisputeFundsWithdrawnType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ChargeDisputeFundsWithdrawnType::V1ChargeDisputeFundsWithdrawn => "v1.charge.dispute.funds_withdrawn",
            V1ChargeDisputeFundsWithdrawnType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ChargeDisputeFundsWithdrawnType {
    fn default() -> Self {
        V1ChargeDisputeFundsWithdrawnType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ChargeDisputeFundsWithdrawnType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ChargeDisputeFundsWithdrawnType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.charge.dispute.funds_withdrawn" => V1ChargeDisputeFundsWithdrawnType::V1ChargeDisputeFundsWithdrawn,
            other => V1ChargeDisputeFundsWithdrawnType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ChargeDisputeUpdatedType {
    V1ChargeDisputeUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ChargeDisputeUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ChargeDisputeUpdatedType::V1ChargeDisputeUpdated => "v1.charge.dispute.updated",
            V1ChargeDisputeUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ChargeDisputeUpdatedType {
    fn default() -> Self {
        V1ChargeDisputeUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ChargeDisputeUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ChargeDisputeUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.charge.dispute.updated" => V1ChargeDisputeUpdatedType::V1ChargeDisputeUpdated,
            other => V1ChargeDisputeUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ChargeRefundUpdatedRelatedObjectType {
    Refund,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ChargeRefundUpdatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ChargeRefundUpdatedRelatedObjectType::Refund => "refund",
            V1ChargeRefundUpdatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ChargeRefundUpdatedRelatedObjectType {
    fn default() -> Self {
        V1ChargeRefundUpdatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ChargeRefundUpdatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ChargeRefundUpdatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "refund" => V1ChargeRefundUpdatedRelatedObjectType::Refund,
            other => V1ChargeRefundUpdatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ChargeRefundUpdatedType {
    V1ChargeRefundUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ChargeRefundUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ChargeRefundUpdatedType::V1ChargeRefundUpdated => "v1.charge.refund.updated",
            V1ChargeRefundUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ChargeRefundUpdatedType {
    fn default() -> Self {
        V1ChargeRefundUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ChargeRefundUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ChargeRefundUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.charge.refund.updated" => V1ChargeRefundUpdatedType::V1ChargeRefundUpdated,
            other => V1ChargeRefundUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CheckoutSessionAsyncPaymentSucceededType {
    V1CheckoutSessionAsyncPaymentSucceeded,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CheckoutSessionAsyncPaymentSucceededType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CheckoutSessionAsyncPaymentSucceededType::V1CheckoutSessionAsyncPaymentSucceeded => {
                "v1.checkout.session.async_payment_succeeded"
            }
            V1CheckoutSessionAsyncPaymentSucceededType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CheckoutSessionAsyncPaymentSucceededType {
    fn default() -> Self {
        V1CheckoutSessionAsyncPaymentSucceededType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CheckoutSessionAsyncPaymentSucceededType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CheckoutSessionAsyncPaymentSucceededType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.checkout.session.async_payment_succeeded" => {
                V1CheckoutSessionAsyncPaymentSucceededType::V1CheckoutSessionAsyncPaymentSucceeded
            }
            other => V1CheckoutSessionAsyncPaymentSucceededType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CouponDeletedType {
    V1CouponDeleted,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CouponDeletedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CouponDeletedType::V1CouponDeleted => "v1.coupon.deleted",
            V1CouponDeletedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CouponDeletedType {
    fn default() -> Self {
        V1CouponDeletedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CouponDeletedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CouponDeletedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.coupon.deleted" => V1CouponDeletedType::V1CouponDeleted,
            other => V1CouponDeletedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CreditNoteCreatedType {
    V1CreditNoteCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CreditNoteCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CreditNoteCreatedType::V1CreditNoteCreated => "v1.credit_note.created",
            V1CreditNoteCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CreditNoteCreatedType {
    fn default() -> Self {
        V1CreditNoteCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CreditNoteCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CreditNoteCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.credit_note.created" => V1CreditNoteCreatedType::V1CreditNoteCreated,
            other => V1CreditNoteCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CustomerDiscountCreatedType {
    V1CustomerDiscountCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CustomerDiscountCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CustomerDiscountCreatedType::V1CustomerDiscountCreated => "v1.customer.discount.created",
            V1CustomerDiscountCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CustomerDiscountCreatedType {
    fn default() -> Self {
        V1CustomerDiscountCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CustomerDiscountCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CustomerDiscountCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.customer.discount.created" => V1CustomerDiscountCreatedType::V1CustomerDiscountCreated,
            other => V1CustomerDiscountCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CustomerSubscriptionCreatedRelatedObjectType {
    Subscription,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CustomerSubscriptionCreatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CustomerSubscriptionCreatedRelatedObjectType::Subscription => "subscription",
            V1CustomerSubscriptionCreatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CustomerSubscriptionCreatedRelatedObjectType {
    fn default() -> Self {
        V1CustomerSubscriptionCreatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CustomerSubscriptionCreatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CustomerSubscriptionCreatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "subscription" => V1CustomerSubscriptionCreatedRelatedObjectType::Subscription,
            other => V1CustomerSubscriptionCreatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CustomerSubscriptionPausedType {
    V1CustomerSubscriptionPaused,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CustomerSubscriptionPausedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CustomerSubscriptionPausedType::V1CustomerSubscriptionPaused => "v1.customer.subscription.paused",
            V1CustomerSubscriptionPausedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CustomerSubscriptionPausedType {
    fn default() -> Self {
        V1CustomerSubscriptionPausedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CustomerSubscriptionPausedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CustomerSubscriptionPausedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.customer.subscription.paused" => V1CustomerSubscriptionPausedType::V1CustomerSubscriptionPaused,
            other => V1CustomerSubscriptionPausedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CustomerSubscriptionPendingUpdateAppliedType {
    V1CustomerSubscriptionPendingUpdateApplied,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CustomerSubscriptionPendingUpdateAppliedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CustomerSubscriptionPendingUpdateAppliedType::V1CustomerSubscriptionPendingUpdateApplied => {
                "v1.customer.subscription.pending_update_applied"
            }
            V1CustomerSubscriptionPendingUpdateAppliedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CustomerSubscriptionPendingUpdateAppliedType {
    fn default() -> Self {
        V1CustomerSubscriptionPendingUpdateAppliedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CustomerSubscriptionPendingUpdateAppliedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CustomerSubscriptionPendingUpdateAppliedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.customer.subscription.pending_update_applied" => {
                V1CustomerSubscriptionPendingUpdateAppliedType::V1CustomerSubscriptionPendingUpdateApplied
            }
            other => V1CustomerSubscriptionPendingUpdateAppliedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1CustomerSubscriptionResumedType {
    V1CustomerSubscriptionResumed,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1CustomerSubscriptionResumedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1CustomerSubscriptionResumedType::V1CustomerSubscriptionResumed => "v1.customer.subscription.resumed",
            V1CustomerSubscriptionResumedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1CustomerSubscriptionResumedType {
    fn default() -> Self {
        V1CustomerSubscriptionResumedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1CustomerSubscriptionResumedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1CustomerSubscriptionResumedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.customer.subscription.resumed" => V1CustomerSubscriptionResumedType::V1CustomerSubscriptionResumed,
            other => V1CustomerSubscriptionResumedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1FinancialConnectionsAccountDeactivatedType {
    V1FinancialConnectionsAccountDeactivated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1FinancialConnectionsAccountDeactivatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1FinancialConnectionsAccountDeactivatedType::V1FinancialConnectionsAccountDeactivated => {
                "v1.financial_connections.account.deactivated"
            }
            V1FinancialConnectionsAccountDeactivatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1FinancialConnectionsAccountDeactivatedType {
    fn default() -> Self {
        V1FinancialConnectionsAccountDeactivatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1FinancialConnectionsAccountDeactivatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1FinancialConnectionsAccountDeactivatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.financial_connections.account.deactivated" => {
                V1FinancialConnectionsAccountDeactivatedType::V1FinancialConnectionsAccountDeactivated
            }
            other => V1FinancialConnectionsAccountDeactivatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1FinancialConnectionsAccountDisconnectedType {
    V1FinancialConnectionsAccountDisconnected,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1FinancialConnectionsAccountDisconnectedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1FinancialConnectionsAccountDisconnectedType::V1FinancialConnectionsAccountDisconnected => {
                "v1.financial_connections.account.disconnected"
            }
            V1FinancialConnectionsAccountDisconnectedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1FinancialConnectionsAccountDisconnectedType {
    fn default() -> Self {
        V1FinancialConnectionsAccountDisconnectedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1FinancialConnectionsAccountDisconnectedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1FinancialConnectionsAccountDisconnectedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.financial_connections.account.disconnected" => {
                V1FinancialConnectionsAccountDisconnectedType::V1FinancialConnectionsAccountDisconnected
            }
            other => V1FinancialConnectionsAccountDisconnectedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1FinancialConnectionsAccountSupportedPaymentMethodTypesUpdatedType {
    V1FinancialConnectionsAccountSupportedPaymentMethodTypesUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1FinancialConnectionsAccountSupportedPaymentMethodTypesUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1FinancialConnectionsAccountSupportedPaymentMethodTypesUpdatedType::V1FinancialConnectionsAccountSupportedPaymentMethodTypesUpdated => "v1.financial_connections.account.supported_payment_method_types_updated",
            V1FinancialConnectionsAccountSupportedPaymentMethodTypesUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1FinancialConnectionsAccountSupportedPaymentMethodTypesUpdatedType {
    fn default() -> Self {
        V1FinancialConnectionsAccountSupportedPaymentMethodTypesUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1FinancialConnectionsAccountSupportedPaymentMethodTypesUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1FinancialConnectionsAccountSupportedPaymentMethodTypesUpdatedType::Unknown(
                String::new(),
            ));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.financial_connections.account.supported_payment_method_types_updated" => V1FinancialConnectionsAccountSupportedPaymentMethodTypesUpdatedType::V1FinancialConnectionsAccountSupportedPaymentMethodTypesUpdated,
            other => V1FinancialConnectionsAccountSupportedPaymentMethodTypesUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1FinancialConnectionsAccountUpcomingDeactivationType {
    V1FinancialConnectionsAccountUpcomingDeactivation,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1FinancialConnectionsAccountUpcomingDeactivationType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1FinancialConnectionsAccountUpcomingDeactivationType::V1FinancialConnectionsAccountUpcomingDeactivation => "v1.financial_connections.account.upcoming_deactivation",
            V1FinancialConnectionsAccountUpcomingDeactivationType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1FinancialConnectionsAccountUpcomingDeactivationType {
    fn default() -> Self {
        V1FinancialConnectionsAccountUpcomingDeactivationType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1FinancialConnectionsAccountUpcomingDeactivationType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1FinancialConnectionsAccountUpcomingDeactivationType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.financial_connections.account.upcoming_deactivation" => {
                V1FinancialConnectionsAccountUpcomingDeactivationType::V1FinancialConnectionsAccountUpcomingDeactivation
            }
            other => V1FinancialConnectionsAccountUpcomingDeactivationType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IdentityVerificationSessionCreatedType {
    V1IdentityVerificationSessionCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IdentityVerificationSessionCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IdentityVerificationSessionCreatedType::V1IdentityVerificationSessionCreated => {
                "v1.identity.verification_session.created"
            }
            V1IdentityVerificationSessionCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IdentityVerificationSessionCreatedType {
    fn default() -> Self {
        V1IdentityVerificationSessionCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IdentityVerificationSessionCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IdentityVerificationSessionCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.identity.verification_session.created" => {
                V1IdentityVerificationSessionCreatedType::V1IdentityVerificationSessionCreated
            }
            other => V1IdentityVerificationSessionCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IdentityVerificationSessionProcessingType {
    V1IdentityVerificationSessionProcessing,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IdentityVerificationSessionProcessingType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IdentityVerificationSessionProcessingType::V1IdentityVerificationSessionProcessing => {
                "v1.identity.verification_session.processing"
            }
            V1IdentityVerificationSessionProcessingType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IdentityVerificationSessionProcessingType {
    fn default() -> Self {
        V1IdentityVerificationSessionProcessingType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IdentityVerificationSessionProcessingType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IdentityVerificationSessionProcessingType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.identity.verification_session.processing" => {
                V1IdentityVerificationSessionProcessingType::V1IdentityVerificationSessionProcessing
            }
            other => V1IdentityVerificationSessionProcessingType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1InvoiceDeletedType {
    V1InvoiceDeleted,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1InvoiceDeletedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1InvoiceDeletedType::V1InvoiceDeleted => "v1.invoice.deleted",
            V1InvoiceDeletedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1InvoiceDeletedType {
    fn default() -> Self {
        V1InvoiceDeletedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1InvoiceDeletedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1InvoiceDeletedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.invoice.deleted" => V1InvoiceDeletedType::V1InvoiceDeleted,
            other => V1InvoiceDeletedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1InvoicePaymentActionRequiredType {
    V1InvoicePaymentActionRequired,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1InvoicePaymentActionRequiredType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1InvoicePaymentActionRequiredType::V1InvoicePaymentActionRequired => "v1.invoice.payment_action_required",
            V1InvoicePaymentActionRequiredType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1InvoicePaymentActionRequiredType {
    fn default() -> Self {
        V1InvoicePaymentActionRequiredType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1InvoicePaymentActionRequiredType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1InvoicePaymentActionRequiredType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.invoice.payment_action_required" => V1InvoicePaymentActionRequiredType::V1InvoicePaymentActionRequired,
            other => V1InvoicePaymentActionRequiredType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1InvoiceSentType {
    V1InvoiceSent,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1InvoiceSentType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1InvoiceSentType::V1InvoiceSent => "v1.invoice.sent",
            V1InvoiceSentType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1InvoiceSentType {
    fn default() -> Self {
        V1InvoiceSentType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1InvoiceSentType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1InvoiceSentType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.invoice.sent" => V1InvoiceSentType::V1InvoiceSent,
            other => V1InvoiceSentType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1InvoiceitemDeletedType {
    V1InvoiceitemDeleted,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1InvoiceitemDeletedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1InvoiceitemDeletedType::V1InvoiceitemDeleted => "v1.invoiceitem.deleted",
            V1InvoiceitemDeletedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1InvoiceitemDeletedType {
    fn default() -> Self {
        V1InvoiceitemDeletedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1InvoiceitemDeletedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1InvoiceitemDeletedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.invoiceitem.deleted" => V1InvoiceitemDeletedType::V1InvoiceitemDeleted,
            other => V1InvoiceitemDeletedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IssuingAuthorizationUpdatedType {
    V1IssuingAuthorizationUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IssuingAuthorizationUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IssuingAuthorizationUpdatedType::V1IssuingAuthorizationUpdated => "v1.issuing_authorization.updated",
            V1IssuingAuthorizationUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IssuingAuthorizationUpdatedType {
    fn default() -> Self {
        V1IssuingAuthorizationUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IssuingAuthorizationUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IssuingAuthorizationUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.issuing_authorization.updated" => V1IssuingAuthorizationUpdatedType::V1IssuingAuthorizationUpdated,
            other => V1IssuingAuthorizationUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IssuingDisputeClosedRelatedObjectType {
    IssuingDispute,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IssuingDisputeClosedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IssuingDisputeClosedRelatedObjectType::IssuingDispute => "issuing.dispute",
            V1IssuingDisputeClosedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IssuingDisputeClosedRelatedObjectType {
    fn default() -> Self {
        V1IssuingDisputeClosedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IssuingDisputeClosedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IssuingDisputeClosedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "issuing.dispute" => V1IssuingDisputeClosedRelatedObjectType::IssuingDispute,
            other => V1IssuingDisputeClosedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IssuingDisputeFundsReinstatedType {
    V1IssuingDisputeFundsReinstated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IssuingDisputeFundsReinstatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IssuingDisputeFundsReinstatedType::V1IssuingDisputeFundsReinstated => {
                "v1.issuing_dispute.funds_reinstated"
            }
            V1IssuingDisputeFundsReinstatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IssuingDisputeFundsReinstatedType {
    fn default() -> Self {
        V1IssuingDisputeFundsReinstatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IssuingDisputeFundsReinstatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IssuingDisputeFundsReinstatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.issuing_dispute.funds_reinstated" => {
                V1IssuingDisputeFundsReinstatedType::V1IssuingDisputeFundsReinstated
            }
            other => V1IssuingDisputeFundsReinstatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IssuingPersonalizationDesignActivatedRelatedObjectType {
    IssuingPersonalizationDesign,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IssuingPersonalizationDesignActivatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IssuingPersonalizationDesignActivatedRelatedObjectType::IssuingPersonalizationDesign => {
                "issuing.personalization_design"
            }
            V1IssuingPersonalizationDesignActivatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IssuingPersonalizationDesignActivatedRelatedObjectType {
    fn default() -> Self {
        V1IssuingPersonalizationDesignActivatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IssuingPersonalizationDesignActivatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IssuingPersonalizationDesignActivatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "issuing.personalization_design" => {
                V1IssuingPersonalizationDesignActivatedRelatedObjectType::IssuingPersonalizationDesign
            }
            other => V1IssuingPersonalizationDesignActivatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IssuingTokenCreatedRelatedObjectType {
    IssuingToken,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IssuingTokenCreatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IssuingTokenCreatedRelatedObjectType::IssuingToken => "issuing.token",
            V1IssuingTokenCreatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IssuingTokenCreatedRelatedObjectType {
    fn default() -> Self {
        V1IssuingTokenCreatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IssuingTokenCreatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IssuingTokenCreatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "issuing.token" => V1IssuingTokenCreatedRelatedObjectType::IssuingToken,
            other => V1IssuingTokenCreatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1IssuingTransactionCreatedType {
    V1IssuingTransactionCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1IssuingTransactionCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1IssuingTransactionCreatedType::V1IssuingTransactionCreated => "v1.issuing_transaction.created",
            V1IssuingTransactionCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1IssuingTransactionCreatedType {
    fn default() -> Self {
        V1IssuingTransactionCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1IssuingTransactionCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1IssuingTransactionCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.issuing_transaction.created" => V1IssuingTransactionCreatedType::V1IssuingTransactionCreated,
            other => V1IssuingTransactionCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PaymentIntentCanceledType {
    V1PaymentIntentCanceled,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PaymentIntentCanceledType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PaymentIntentCanceledType::V1PaymentIntentCanceled => "v1.payment_intent.canceled",
            V1PaymentIntentCanceledType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PaymentIntentCanceledType {
    fn default() -> Self {
        V1PaymentIntentCanceledType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PaymentIntentCanceledType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PaymentIntentCanceledType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.payment_intent.canceled" => V1PaymentIntentCanceledType::V1PaymentIntentCanceled,
            other => V1PaymentIntentCanceledType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PaymentIntentProcessingType {
    V1PaymentIntentProcessing,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PaymentIntentProcessingType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PaymentIntentProcessingType::V1PaymentIntentProcessing => "v1.payment_intent.processing",
            V1PaymentIntentProcessingType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PaymentIntentProcessingType {
    fn default() -> Self {
        V1PaymentIntentProcessingType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PaymentIntentProcessingType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PaymentIntentProcessingType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.payment_intent.processing" => V1PaymentIntentProcessingType::V1PaymentIntentProcessing,
            other => V1PaymentIntentProcessingType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PaymentLinkUpdatedType {
    V1PaymentLinkUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PaymentLinkUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PaymentLinkUpdatedType::V1PaymentLinkUpdated => "v1.payment_link.updated",
            V1PaymentLinkUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PaymentLinkUpdatedType {
    fn default() -> Self {
        V1PaymentLinkUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PaymentLinkUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PaymentLinkUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.payment_link.updated" => V1PaymentLinkUpdatedType::V1PaymentLinkUpdated,
            other => V1PaymentLinkUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PaymentMethodUpdatedType {
    V1PaymentMethodUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PaymentMethodUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PaymentMethodUpdatedType::V1PaymentMethodUpdated => "v1.payment_method.updated",
            V1PaymentMethodUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PaymentMethodUpdatedType {
    fn default() -> Self {
        V1PaymentMethodUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PaymentMethodUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PaymentMethodUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.payment_method.updated" => V1PaymentMethodUpdatedType::V1PaymentMethodUpdated,
            other => V1PaymentMethodUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PayoutFailedType {
    V1PayoutFailed,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PayoutFailedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PayoutFailedType::V1PayoutFailed => "v1.payout.failed",
            V1PayoutFailedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PayoutFailedType {
    fn default() -> Self {
        V1PayoutFailedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PayoutFailedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PayoutFailedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.payout.failed" => V1PayoutFailedType::V1PayoutFailed,
            other => V1PayoutFailedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PersonCreatedRelatedObjectType {
    Person,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PersonCreatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PersonCreatedRelatedObjectType::Person => "person",
            V1PersonCreatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PersonCreatedRelatedObjectType {
    fn default() -> Self {
        V1PersonCreatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PersonCreatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PersonCreatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "person" => V1PersonCreatedRelatedObjectType::Person,
            other => V1PersonCreatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1PersonCreatedType {
    V1PersonCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1PersonCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1PersonCreatedType::V1PersonCreated => "v1.person.created",
            V1PersonCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1PersonCreatedType {
    fn default() -> Self {
        V1PersonCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1PersonCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1PersonCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.person.created" => V1PersonCreatedType::V1PersonCreated,
            other => V1PersonCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1ProductCreatedType {
    V1ProductCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1ProductCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1ProductCreatedType::V1ProductCreated => "v1.product.created",
            V1ProductCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1ProductCreatedType {
    fn default() -> Self {
        V1ProductCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1ProductCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1ProductCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.product.created" => V1ProductCreatedType::V1ProductCreated,
            other => V1ProductCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1QuoteAcceptedRelatedObjectType {
    Quote,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1QuoteAcceptedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1QuoteAcceptedRelatedObjectType::Quote => "quote",
            V1QuoteAcceptedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1QuoteAcceptedRelatedObjectType {
    fn default() -> Self {
        V1QuoteAcceptedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1QuoteAcceptedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1QuoteAcceptedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "quote" => V1QuoteAcceptedRelatedObjectType::Quote,
            other => V1QuoteAcceptedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1QuoteCreatedType {
    V1QuoteCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1QuoteCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1QuoteCreatedType::V1QuoteCreated => "v1.quote.created",
            V1QuoteCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1QuoteCreatedType {
    fn default() -> Self {
        V1QuoteCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1QuoteCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1QuoteCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.quote.created" => V1QuoteCreatedType::V1QuoteCreated,
            other => V1QuoteCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1RadarEarlyFraudWarningCreatedType {
    V1RadarEarlyFraudWarningCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1RadarEarlyFraudWarningCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1RadarEarlyFraudWarningCreatedType::V1RadarEarlyFraudWarningCreated => {
                "v1.radar.early_fraud_warning.created"
            }
            V1RadarEarlyFraudWarningCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1RadarEarlyFraudWarningCreatedType {
    fn default() -> Self {
        V1RadarEarlyFraudWarningCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1RadarEarlyFraudWarningCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1RadarEarlyFraudWarningCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.radar.early_fraud_warning.created" => {
                V1RadarEarlyFraudWarningCreatedType::V1RadarEarlyFraudWarningCreated
            }
            other => V1RadarEarlyFraudWarningCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1RefundCreatedType {
    V1RefundCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1RefundCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1RefundCreatedType::V1RefundCreated => "v1.refund.created",
            V1RefundCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1RefundCreatedType {
    fn default() -> Self {
        V1RefundCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1RefundCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1RefundCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.refund.created" => V1RefundCreatedType::V1RefundCreated,
            other => V1RefundCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1SetupIntentCanceledRelatedObjectType {
    SetupIntent,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1SetupIntentCanceledRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1SetupIntentCanceledRelatedObjectType::SetupIntent => "setup_intent",
            V1SetupIntentCanceledRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1SetupIntentCanceledRelatedObjectType {
    fn default() -> Self {
        V1SetupIntentCanceledRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1SetupIntentCanceledRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1SetupIntentCanceledRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "setup_intent" => V1SetupIntentCanceledRelatedObjectType::SetupIntent,
            other => V1SetupIntentCanceledRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1SetupIntentCanceledType {
    V1SetupIntentCanceled,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1SetupIntentCanceledType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1SetupIntentCanceledType::V1SetupIntentCanceled => "v1.setup_intent.canceled",
            V1SetupIntentCanceledType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1SetupIntentCanceledType {
    fn default() -> Self {
        V1SetupIntentCanceledType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1SetupIntentCanceledType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1SetupIntentCanceledType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.setup_intent.canceled" => V1SetupIntentCanceledType::V1SetupIntentCanceled,
            other => V1SetupIntentCanceledType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1SigmaScheduledQueryRunCreatedType {
    V1SigmaScheduledQueryRunCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1SigmaScheduledQueryRunCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1SigmaScheduledQueryRunCreatedType::V1SigmaScheduledQueryRunCreated => {
                "v1.sigma.scheduled_query_run.created"
            }
            V1SigmaScheduledQueryRunCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1SigmaScheduledQueryRunCreatedType {
    fn default() -> Self {
        V1SigmaScheduledQueryRunCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1SigmaScheduledQueryRunCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1SigmaScheduledQueryRunCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.sigma.scheduled_query_run.created" => {
                V1SigmaScheduledQueryRunCreatedType::V1SigmaScheduledQueryRunCreated
            }
            other => V1SigmaScheduledQueryRunCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1SubscriptionScheduleReleasedType {
    V1SubscriptionScheduleReleased,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1SubscriptionScheduleReleasedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1SubscriptionScheduleReleasedType::V1SubscriptionScheduleReleased => "v1.subscription_schedule.released",
            V1SubscriptionScheduleReleasedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1SubscriptionScheduleReleasedType {
    fn default() -> Self {
        V1SubscriptionScheduleReleasedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1SubscriptionScheduleReleasedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1SubscriptionScheduleReleasedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.subscription_schedule.released" => V1SubscriptionScheduleReleasedType::V1SubscriptionScheduleReleased,
            other => V1SubscriptionScheduleReleasedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1TaxRateCreatedRelatedObjectType {
    TaxRate,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1TaxRateCreatedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1TaxRateCreatedRelatedObjectType::TaxRate => "tax_rate",
            V1TaxRateCreatedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1TaxRateCreatedRelatedObjectType {
    fn default() -> Self {
        V1TaxRateCreatedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1TaxRateCreatedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1TaxRateCreatedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "tax_rate" => V1TaxRateCreatedRelatedObjectType::TaxRate,
            other => V1TaxRateCreatedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1TaxRateUpdatedType {
    V1TaxRateUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1TaxRateUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1TaxRateUpdatedType::V1TaxRateUpdated => "v1.tax_rate.updated",
            V1TaxRateUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1TaxRateUpdatedType {
    fn default() -> Self {
        V1TaxRateUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1TaxRateUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1TaxRateUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.tax_rate.updated" => V1TaxRateUpdatedType::V1TaxRateUpdated,
            other => V1TaxRateUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1TerminalReaderActionFailedRelatedObjectType {
    TerminalReader,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1TerminalReaderActionFailedRelatedObjectType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1TerminalReaderActionFailedRelatedObjectType::TerminalReader => "terminal.reader",
            V1TerminalReaderActionFailedRelatedObjectType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1TerminalReaderActionFailedRelatedObjectType {
    fn default() -> Self {
        V1TerminalReaderActionFailedRelatedObjectType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1TerminalReaderActionFailedRelatedObjectType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1TerminalReaderActionFailedRelatedObjectType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "terminal.reader" => V1TerminalReaderActionFailedRelatedObjectType::TerminalReader,
            other => V1TerminalReaderActionFailedRelatedObjectType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1TestHelpersTestClockDeletedType {
    V1TestHelpersTestClockDeleted,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1TestHelpersTestClockDeletedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1TestHelpersTestClockDeletedType::V1TestHelpersTestClockDeleted => "v1.test_helpers.test_clock.deleted",
            V1TestHelpersTestClockDeletedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1TestHelpersTestClockDeletedType {
    fn default() -> Self {
        V1TestHelpersTestClockDeletedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1TestHelpersTestClockDeletedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1TestHelpersTestClockDeletedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.test_helpers.test_clock.deleted" => V1TestHelpersTestClockDeletedType::V1TestHelpersTestClockDeleted,
            other => V1TestHelpersTestClockDeletedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1TopupCreatedType {
    V1TopupCreated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1TopupCreatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1TopupCreatedType::V1TopupCreated => "v1.topup.created",
            V1TopupCreatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1TopupCreatedType {
    fn default() -> Self {
        V1TopupCreatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1TopupCreatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1TopupCreatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.topup.created" => V1TopupCreatedType::V1TopupCreated,
            other => V1TopupCreatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1TopupSucceededType {
    V1TopupSucceeded,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1TopupSucceededType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1TopupSucceededType::V1TopupSucceeded => "v1.topup.succeeded",
            V1TopupSucceededType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1TopupSucceededType {
    fn default() -> Self {
        V1TopupSucceededType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1TopupSucceededType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1TopupSucceededType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.topup.succeeded" => V1TopupSucceededType::V1TopupSucceeded,
            other => V1TopupSucceededType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1TransferReversedType {
    V1TransferReversed,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1TransferReversedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1TransferReversedType::V1TransferReversed => "v1.transfer.reversed",
            V1TransferReversedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1TransferReversedType {
    fn default() -> Self {
        V1TransferReversedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1TransferReversedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1TransferReversedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.transfer.reversed" => V1TransferReversedType::V1TransferReversed,
            other => V1TransferReversedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V1TransferUpdatedType {
    V1TransferUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V1TransferUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V1TransferUpdatedType::V1TransferUpdated => "v1.transfer.updated",
            V1TransferUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V1TransferUpdatedType {
    fn default() -> Self {
        V1TransferUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V1TransferUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V1TransferUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v1.transfer.updated" => V1TransferUpdatedType::V1TransferUpdated,
            other => V1TransferUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CommerceProductCatalogImportsProcessingType {
    V2CommerceProductCatalogImportsProcessing,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V2CommerceProductCatalogImportsProcessingType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CommerceProductCatalogImportsProcessingType::V2CommerceProductCatalogImportsProcessing => {
                "v2.commerce.product_catalog.imports.processing"
            }
            V2CommerceProductCatalogImportsProcessingType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V2CommerceProductCatalogImportsProcessingType {
    fn default() -> Self {
        V2CommerceProductCatalogImportsProcessingType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V2CommerceProductCatalogImportsProcessingType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CommerceProductCatalogImportsProcessingType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.commerce.product_catalog.imports.processing" => {
                V2CommerceProductCatalogImportsProcessingType::V2CommerceProductCatalogImportsProcessing
            }
            other => V2CommerceProductCatalogImportsProcessingType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountConfigurationCustomerUpdatedType {
    V2CoreAccountConfigurationCustomerUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V2CoreAccountConfigurationCustomerUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountConfigurationCustomerUpdatedType::V2CoreAccountConfigurationCustomerUpdated => {
                "v2.core.account[configuration.customer].updated"
            }
            V2CoreAccountConfigurationCustomerUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V2CoreAccountConfigurationCustomerUpdatedType {
    fn default() -> Self {
        V2CoreAccountConfigurationCustomerUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V2CoreAccountConfigurationCustomerUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountConfigurationCustomerUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.core.account[configuration.customer].updated" => {
                V2CoreAccountConfigurationCustomerUpdatedType::V2CoreAccountConfigurationCustomerUpdated
            }
            other => V2CoreAccountConfigurationCustomerUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountDefaultsUpdatedType {
    V2CoreAccountDefaultsUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V2CoreAccountDefaultsUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountDefaultsUpdatedType::V2CoreAccountDefaultsUpdated => "v2.core.account[defaults].updated",
            V2CoreAccountDefaultsUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V2CoreAccountDefaultsUpdatedType {
    fn default() -> Self {
        V2CoreAccountDefaultsUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V2CoreAccountDefaultsUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountDefaultsUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.core.account[defaults].updated" => V2CoreAccountDefaultsUpdatedType::V2CoreAccountDefaultsUpdated,
            other => V2CoreAccountDefaultsUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountIdentityUpdatedType {
    V2CoreAccountIdentityUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V2CoreAccountIdentityUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountIdentityUpdatedType::V2CoreAccountIdentityUpdated => "v2.core.account[identity].updated",
            V2CoreAccountIdentityUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V2CoreAccountIdentityUpdatedType {
    fn default() -> Self {
        V2CoreAccountIdentityUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V2CoreAccountIdentityUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountIdentityUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.core.account[identity].updated" => V2CoreAccountIdentityUpdatedType::V2CoreAccountIdentityUpdated,
            other => V2CoreAccountIdentityUpdatedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountLinkReturnedType {
    V2CoreAccountLinkReturned,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V2CoreAccountLinkReturnedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountLinkReturnedType::V2CoreAccountLinkReturned => "v2.core.account_link.returned",
            V2CoreAccountLinkReturnedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V2CoreAccountLinkReturnedType {
    fn default() -> Self {
        V2CoreAccountLinkReturnedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V2CoreAccountLinkReturnedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountLinkReturnedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.core.account_link.returned" => V2CoreAccountLinkReturnedType::V2CoreAccountLinkReturned,
            other => V2CoreAccountLinkReturnedType::Unknown(other.to_string()),
        })
    }
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountPersonDeletedChanges {
    #[doc = "Updated values of properties that the event changed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<json::OwnedValue>,
    #[doc = "Values of properties before the event changes."]
    #[json(default)]
    pub before: json::OwnedValue,
}

#[cfg(feature = "events")]
#[derive(Debug, Clone, PartialEq)]
pub enum V2CoreAccountRequirementsUpdatedType {
    V2CoreAccountRequirementsUpdated,
    /// A value not known to this version of the crate.
    Unknown(String),
}

#[cfg(feature = "events")]
impl V2CoreAccountRequirementsUpdatedType {
    /// Returns the value as it appears on the wire.
    pub fn as_str(&self) -> &str {
        match self {
            V2CoreAccountRequirementsUpdatedType::V2CoreAccountRequirementsUpdated => {
                "v2.core.account[requirements].updated"
            }
            V2CoreAccountRequirementsUpdatedType::Unknown(value) => value,
        }
    }
}

#[cfg(feature = "events")]
impl Default for V2CoreAccountRequirementsUpdatedType {
    fn default() -> Self {
        V2CoreAccountRequirementsUpdatedType::Unknown(String::new())
    }
}

#[cfg(feature = "events")]
impl<'json> json::Decode<'json> for V2CoreAccountRequirementsUpdatedType {
    fn decode(decoder: &mut json::Decoder<'json>) -> json::Result<Self> {
        decoder.skip_ws()?;
        if decoder.peek_byte() == Some(b'n') {
            decoder.decode_null()?;
            return Ok(V2CoreAccountRequirementsUpdatedType::Unknown(String::new()));
        }
        let value = decoder.decode_string()?;
        Ok(match value.as_str() {
            "v2.core.account[requirements].updated" => {
                V2CoreAccountRequirementsUpdatedType::V2CoreAccountRequirementsUpdated
            }
            other => V2CoreAccountRequirementsUpdatedType::Unknown(other.to_string()),
        })
    }
}
