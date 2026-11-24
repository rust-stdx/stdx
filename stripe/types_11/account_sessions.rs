#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountSessionParamsComponents {
    #[doc = "Configuration for the [account management](/connect/supported-embedded-components/account-management/) embedded compo..."]
    pub account_management:
        Option<stripe_types_05::account_sessions::CreateAccountSessionParamsComponentsAccountManagement>,
    #[doc = "Configuration for the [account onboarding](/connect/supported-embedded-components/account-onboarding/) embedded compo..."]
    pub account_onboarding:
        Option<stripe_types_07::account_sessions::CreateAccountSessionParamsComponentsAccountOnboarding>,
    #[doc = "Configuration for the [balance report](/connect/supported-embedded-components/financial-reports#balance-report) embed..."]
    pub balance_report: Option<stripe_types_06::account_sessions::CreateAccountSessionParamsComponentsBalanceReport>,
    #[doc = "Configuration for the [balances](/connect/supported-embedded-components/balances/) embedded component."]
    pub balances: Option<stripe_types_06::account_sessions::CreateAccountSessionParamsComponentsBalances>,
    #[doc = "Configuration for the [disputes list](/connect/supported-embedded-components/disputes-list/) embedded component."]
    pub disputes_list: Option<stripe_types_06::account_sessions::CreateAccountSessionParamsComponentsDisputesList>,
    #[doc = "Configuration for the [documents](/connect/supported-embedded-components/documents/) embedded component."]
    pub documents: Option<stripe_types_07::account_sessions::CreateAccountSessionParamsComponentsDocuments>,
    #[doc = "Configuration for the [financial account](/connect/supported-embedded-components/financial-account/) embedded component."]
    pub financial_account:
        Option<stripe_types_06::account_sessions::CreateAccountSessionParamsComponentsFinancialAccount>,
    #[doc = "Configuration for the financial account transactions..."]
    pub financial_account_transactions:
        Option<stripe_types_06::account_sessions::CreateAccountSessionParamsComponentsFinancialAccountTransactions>,
    #[doc = "Configuration for the [instant payouts promotion](/connect/supported-embedded-components/instant-payouts-promotion/) ..."]
    pub instant_payouts_promotion:
        Option<stripe_types_06::account_sessions::CreateAccountSessionParamsComponentsInstantPayoutsPromotion>,
    #[doc = "Configuration for the [issuing card](/connect/supported-embedded-components/issuing-card/) embedded component."]
    pub issuing_card: Option<stripe_types_06::account_sessions::CreateAccountSessionParamsComponentsIssuingCard>,
    #[doc = "Configuration for the [issuing cards list](/connect/supported-embedded-components/issuing-cards-list/) embedded compo..."]
    pub issuing_cards_list:
        Option<stripe_types_05::account_sessions::CreateAccountSessionParamsComponentsIssuingCardsList>,
    #[doc = "Configuration for the [notification banner](/connect/supported-embedded-components/notification-banner/) embedded com..."]
    pub notification_banner:
        Option<stripe_types_08::account_sessions::CreateAccountSessionParamsComponentsNotificationBanner>,
    #[doc = "Configuration for the [payment details](/connect/supported-embedded-components/payment-details/) embedded component."]
    pub payment_details: Option<stripe_types_07::account_sessions::CreateAccountSessionParamsComponentsPaymentDetails>,
    #[doc = "Configuration for the [payment disputes](/connect/supported-embedded-components/payment-disputes/) embedded component."]
    pub payment_disputes:
        Option<stripe_types_06::account_sessions::CreateAccountSessionParamsComponentsPaymentDisputes>,
    #[doc = "Configuration for the [payment method settings](/connect/supported-embedded-components/payment-method-settings/) embe..."]
    pub payment_method_settings:
        Option<stripe_types_05::account_sessions::CreateAccountSessionParamsComponentsPaymentMethodSettings>,
    #[doc = "Configuration for the [payments](/connect/supported-embedded-components/payments/) embedded component."]
    pub payments: Option<stripe_types_09::account_sessions::CreateAccountSessionParamsComponentsPayments>,
    #[doc = "Configuration for the [payout details](/connect/supported-embedded-components/payout-details/) embedded component."]
    pub payout_details: Option<stripe_types_09::account_sessions::CreateAccountSessionParamsComponentsPayoutDetails>,
    #[doc = "Configuration for the payout reconciliation report..."]
    pub payout_reconciliation_report:
        Option<stripe_types_09::account_sessions::CreateAccountSessionParamsComponentsPayoutReconciliationReport>,
    #[doc = "Configuration for the [payouts](/connect/supported-embedded-components/payouts/) embedded component."]
    pub payouts: Option<stripe_types_08::account_sessions::CreateAccountSessionParamsComponentsPayouts>,
    #[doc = "Configuration for the [payouts list](/connect/supported-embedded-components/payouts-list/) embedded component."]
    pub payouts_list: Option<stripe_types_07::account_sessions::CreateAccountSessionParamsComponentsPayoutsList>,
    #[doc = "Configuration for the [tax registrations](/connect/supported-embedded-components/tax-registrations/) embedded component."]
    pub tax_registrations:
        Option<stripe_types_09::account_sessions::CreateAccountSessionParamsComponentsTaxRegistrations>,
    #[doc = "Configuration for the [tax settings](/connect/supported-embedded-components/tax-settings/) embedded component."]
    pub tax_settings: Option<stripe_types_09::account_sessions::CreateAccountSessionParamsComponentsTaxSettings>,
}

#[cfg(feature = "account_sessions")]
impl crate::params::FormEncode for CreateAccountSessionParamsComponents {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account_management", self.account_management.as_ref());
        form.field_opt("account_onboarding", self.account_onboarding.as_ref());
        form.field_opt("balance_report", self.balance_report.as_ref());
        form.field_opt("balances", self.balances.as_ref());
        form.field_opt("disputes_list", self.disputes_list.as_ref());
        form.field_opt("documents", self.documents.as_ref());
        form.field_opt("financial_account", self.financial_account.as_ref());
        form.field_opt("financial_account_transactions", self.financial_account_transactions.as_ref());
        form.field_opt("instant_payouts_promotion", self.instant_payouts_promotion.as_ref());
        form.field_opt("issuing_card", self.issuing_card.as_ref());
        form.field_opt("issuing_cards_list", self.issuing_cards_list.as_ref());
        form.field_opt("notification_banner", self.notification_banner.as_ref());
        form.field_opt("payment_details", self.payment_details.as_ref());
        form.field_opt("payment_disputes", self.payment_disputes.as_ref());
        form.field_opt("payment_method_settings", self.payment_method_settings.as_ref());
        form.field_opt("payments", self.payments.as_ref());
        form.field_opt("payout_details", self.payout_details.as_ref());
        form.field_opt("payout_reconciliation_report", self.payout_reconciliation_report.as_ref());
        form.field_opt("payouts", self.payouts.as_ref());
        form.field_opt("payouts_list", self.payouts_list.as_ref());
        form.field_opt("tax_registrations", self.tax_registrations.as_ref());
        form.field_opt("tax_settings", self.tax_settings.as_ref());
    }
}
