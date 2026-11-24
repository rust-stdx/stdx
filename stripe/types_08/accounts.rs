#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsController {
    #[doc = "A hash of configuration for who pays Stripe fees for product usage on this account."]
    pub fees: Option<stripe_types_05::accounts::CreateAccountParamsControllerFees>,
    #[doc = "A hash of configuration for products that have negative balance liability, and whether Stripe or a Connect applicatio..."]
    pub losses: Option<stripe_types_05::accounts::CreateAccountParamsControllerLosses>,
    #[doc = "A value indicating responsibility for collecting updated information when requirements on the account are due or change."]
    pub requirement_collection: Option<stripe_types_05::accounts::CreateAccountParamsControllerRequirementCollection>,
    #[doc = "A hash of configuration for Stripe-hosted dashboards."]
    pub stripe_dashboard: Option<stripe_types_06::accounts::CreateAccountParamsControllerStripeDashboard>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsController {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("fees", self.fees.as_ref());
        form.field_opt("losses", self.losses.as_ref());
        form.field_opt("requirement_collection", self.requirement_collection.as_ref());
        form.field_opt("stripe_dashboard", self.stripe_dashboard.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsSettingsPayouts {
    #[doc = "A Boolean indicating whether Stripe should try to reclaim negative balances from an attached bank account."]
    pub debit_negative_balances: Option<bool>,
    #[doc = "Details on when funds from charges are available, and when they are paid out to an external account."]
    pub schedule: Option<stripe_types_05::accounts::CreateAccountParamsSettingsPayoutsSchedule>,
    #[doc = "The text that appears on the bank account statement for payouts."]
    pub statement_descriptor: Option<String>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsSettingsPayouts {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("debit_negative_balances", self.debit_negative_balances.as_ref());
        form.field_opt("schedule", self.schedule.as_ref());
        form.field_opt("statement_descriptor", self.statement_descriptor.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountPersonParamsUsCfpbData {
    #[doc = "The persons ethnicity details"]
    pub ethnicity_details: Option<stripe_types_05::accounts::CreateAccountPersonParamsUsCfpbDataEthnicityDetails>,
    #[doc = "The persons race details"]
    pub race_details: Option<stripe_types_06::accounts::CreateAccountPersonParamsUsCfpbDataRaceDetails>,
    #[doc = "The persons self-identified gender"]
    pub self_identified_gender: Option<String>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountPersonParamsUsCfpbData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("ethnicity_details", self.ethnicity_details.as_ref());
        form.field_opt("race_details", self.race_details.as_ref());
        form.field_opt("self_identified_gender", self.self_identified_gender.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountPersonParamsVerification {
    #[doc = "A document showing address, either a passport, local ID card, or utility bill from a well-known utility company."]
    pub additional_document: Option<stripe_types_06::accounts::CreateAccountPersonParamsVerificationAdditionalDocument>,
    #[doc = "An identifying document, either a passport or local ID card."]
    pub document: Option<stripe_types_05::accounts::CreateAccountPersonParamsVerificationDocument>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountPersonParamsVerification {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("additional_document", self.additional_document.as_ref());
        form.field_opt("document", self.document.as_ref());
    }
}

#[cfg(any(
    feature = "account",
    feature = "accounts",
    feature = "application_fees",
    feature = "balance_transactions",
    feature = "billing",
    feature = "charges",
    feature = "checkout",
    feature = "confirmation_tokens",
    feature = "credit_notes",
    feature = "customer_sessions",
    feature = "customers",
    feature = "disputes",
    feature = "events",
    feature = "financial_connections",
    feature = "invoice_payments",
    feature = "invoiceitems",
    feature = "invoices",
    feature = "issuing",
    feature = "mandates",
    feature = "payment_attempt_records",
    feature = "payment_intents",
    feature = "payment_links",
    feature = "payment_methods",
    feature = "payment_records",
    feature = "payouts",
    feature = "promotion_codes",
    feature = "quotes",
    feature = "radar",
    feature = "refunds",
    feature = "reviews",
    feature = "setup_attempts",
    feature = "setup_intents",
    feature = "subscription_items",
    feature = "subscription_schedules",
    feature = "subscriptions",
    feature = "tax_ids",
    feature = "terminal",
    feature = "test_helpers",
    feature = "three_d_secure",
    feature = "tokens",
    feature = "topups",
    feature = "transfers",
    feature = "treasury"
))]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct LegalEntityPersonVerificationDocument {
    #[doc = "The back of an ID returned by a [file upload](https://docs.stripe.com/api#create_file) with a `purpose` value of `ide..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub back: Option<crate::Expandable<stripe_types_06::files::File>>,
    #[doc = "A user-displayable string describing the verification state of this document."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[doc = "One of `document_corrupt`, `document_country_not_supported`, `document_expired`, `document_failed_copy`, `document_fa..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub details_code: Option<String>,
    #[doc = "The front of an ID returned by a [file upload](https://docs.stripe.com/api#create_file) with a `purpose` value of `id..."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub front: Option<crate::Expandable<stripe_types_06::files::File>>,
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct UpdateAccountParamsCapabilities {
    #[doc = "The acss_debit_payments capability."]
    pub acss_debit_payments: Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesAcssDebitPayments>,
    #[doc = "The affirm_payments capability."]
    pub affirm_payments: Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesAffirmPayments>,
    #[doc = "The afterpay_clearpay_payments capability."]
    pub afterpay_clearpay_payments:
        Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesAfterpayClearpayPayments>,
    #[doc = "The alma_payments capability."]
    pub alma_payments: Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesAlmaPayments>,
    #[doc = "The amazon_pay_payments capability."]
    pub amazon_pay_payments: Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesAmazonPayPayments>,
    #[doc = "The app_distribution capability."]
    pub app_distribution: Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesAppDistribution>,
    #[doc = "The au_becs_debit_payments capability."]
    pub au_becs_debit_payments: Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesAuBecsDebitPayments>,
    #[doc = "The bacs_debit_payments capability."]
    pub bacs_debit_payments: Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesBacsDebitPayments>,
    #[doc = "The bancontact_payments capability."]
    pub bancontact_payments: Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesBancontactPayments>,
    #[doc = "The bank_transfer_payments capability."]
    pub bank_transfer_payments: Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesBankTransferPayments>,
    #[doc = "The billie_payments capability."]
    pub billie_payments: Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesBilliePayments>,
    #[doc = "The bizum_payments capability."]
    pub bizum_payments: Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesBizumPayments>,
    #[doc = "The blik_payments capability."]
    pub blik_payments: Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesBlikPayments>,
    #[doc = "The blik_recurring_payments capability."]
    pub blik_recurring_payments:
        Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesBlikRecurringPayments>,
    #[doc = "The boleto_payments capability."]
    pub boleto_payments: Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesBoletoPayments>,
    #[doc = "The card_issuing capability."]
    pub card_issuing: Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesCardIssuing>,
    #[doc = "The card_payments capability."]
    pub card_payments: Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesCardPayments>,
    #[doc = "The cartes_bancaires_payments capability."]
    pub cartes_bancaires_payments:
        Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesCartesBancairesPayments>,
    #[doc = "The cashapp_payments capability."]
    pub cashapp_payments: Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesCashappPayments>,
    #[doc = "The crypto_payments capability."]
    pub crypto_payments: Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesCryptoPayments>,
    #[doc = "The eps_payments capability."]
    pub eps_payments: Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesEpsPayments>,
    #[doc = "The fpx_payments capability."]
    pub fpx_payments: Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesFpxPayments>,
    #[doc = "The gb_bank_transfer_payments capability."]
    pub gb_bank_transfer_payments:
        Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesGbBankTransferPayments>,
    #[doc = "The giropay_payments capability."]
    pub giropay_payments: Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesGiropayPayments>,
    #[doc = "The grabpay_payments capability."]
    pub grabpay_payments: Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesGrabpayPayments>,
    #[doc = "The ideal_payments capability."]
    pub ideal_payments: Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesIdealPayments>,
    #[doc = "The india_international_payments capability."]
    pub india_international_payments:
        Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesIndiaInternationalPayments>,
    #[doc = "The jcb_payments capability."]
    pub jcb_payments: Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesJcbPayments>,
    #[doc = "The jp_bank_transfer_payments capability."]
    pub jp_bank_transfer_payments:
        Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesJpBankTransferPayments>,
    #[doc = "The kakao_pay_payments capability."]
    pub kakao_pay_payments: Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesKakaoPayPayments>,
    #[doc = "The klarna_payments capability."]
    pub klarna_payments: Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesKlarnaPayments>,
    #[doc = "The konbini_payments capability."]
    pub konbini_payments: Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesKonbiniPayments>,
    #[doc = "The kr_card_payments capability."]
    pub kr_card_payments: Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesKrCardPayments>,
    #[doc = "The legacy_payments capability."]
    pub legacy_payments: Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesLegacyPayments>,
    #[doc = "The link_payments capability."]
    pub link_payments: Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesLinkPayments>,
    #[doc = "The mb_way_payments capability."]
    pub mb_way_payments: Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesMbWayPayments>,
    #[doc = "The mobilepay_payments capability."]
    pub mobilepay_payments: Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesMobilepayPayments>,
    #[doc = "The multibanco_payments capability."]
    pub multibanco_payments: Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesMultibancoPayments>,
    #[doc = "The mx_bank_transfer_payments capability."]
    pub mx_bank_transfer_payments:
        Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesMxBankTransferPayments>,
    #[doc = "The naver_pay_payments capability."]
    pub naver_pay_payments: Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesNaverPayPayments>,
    #[doc = "The nz_bank_account_becs_debit_payments capability."]
    pub nz_bank_account_becs_debit_payments:
        Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesNzBankAccountBecsDebitPayments>,
    #[doc = "The oxxo_payments capability."]
    pub oxxo_payments: Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesOxxoPayments>,
    #[doc = "The p24_payments capability."]
    pub p24_payments: Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesP24Payments>,
    #[doc = "The pay_by_bank_payments capability."]
    pub pay_by_bank_payments: Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesPayByBankPayments>,
    #[doc = "The payco_payments capability."]
    pub payco_payments: Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesPaycoPayments>,
    #[doc = "The paynow_payments capability."]
    pub paynow_payments: Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesPaynowPayments>,
    #[doc = "The paypay_payments capability."]
    pub paypay_payments: Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesPaypayPayments>,
    #[doc = "The payto_payments capability."]
    pub payto_payments: Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesPaytoPayments>,
    #[doc = "The pix_payments capability."]
    pub pix_payments: Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesPixPayments>,
    #[doc = "The promptpay_payments capability."]
    pub promptpay_payments: Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesPromptpayPayments>,
    #[doc = "The revolut_pay_payments capability."]
    pub revolut_pay_payments: Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesRevolutPayPayments>,
    #[doc = "The samsung_pay_payments capability."]
    pub samsung_pay_payments: Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesSamsungPayPayments>,
    #[doc = "The satispay_payments capability."]
    pub satispay_payments: Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesSatispayPayments>,
    #[doc = "The scalapay_payments capability."]
    pub scalapay_payments: Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesScalapayPayments>,
    #[doc = "The sepa_bank_transfer_payments capability."]
    pub sepa_bank_transfer_payments:
        Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesSepaBankTransferPayments>,
    #[doc = "The sepa_debit_payments capability."]
    pub sepa_debit_payments: Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesSepaDebitPayments>,
    #[doc = "The sequra_payments capability."]
    pub sequra_payments: Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesSequraPayments>,
    #[doc = "The sofort_payments capability."]
    pub sofort_payments: Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesSofortPayments>,
    #[doc = "The sunbit_payments capability."]
    pub sunbit_payments: Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesSunbitPayments>,
    #[doc = "The swish_payments capability."]
    pub swish_payments: Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesSwishPayments>,
    #[doc = "The tax_reporting_us_1099_k capability."]
    pub tax_reporting_us_1099_k: Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesTaxReportingUs1099K>,
    #[doc = "The tax_reporting_us_1099_misc capability."]
    pub tax_reporting_us_1099_misc:
        Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesTaxReportingUs1099Misc>,
    #[doc = "The transfers capability."]
    pub transfers: Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesTransfers>,
    #[doc = "The treasury capability."]
    pub treasury: Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesTreasury>,
    #[doc = "The twint_payments capability."]
    pub twint_payments: Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesTwintPayments>,
    #[doc = "The upi_payments capability."]
    pub upi_payments: Option<stripe_types_06::accounts::UpdateAccountParamsCapabilitiesUpiPayments>,
    #[doc = "The us_bank_account_ach_payments capability."]
    pub us_bank_account_ach_payments:
        Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesUsBankAccountAchPayments>,
    #[doc = "The us_bank_transfer_payments capability."]
    pub us_bank_transfer_payments:
        Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesUsBankTransferPayments>,
    #[doc = "The zip_payments capability."]
    pub zip_payments: Option<stripe_types_05::accounts::UpdateAccountParamsCapabilitiesZipPayments>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for UpdateAccountParamsCapabilities {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("acss_debit_payments", self.acss_debit_payments.as_ref());
        form.field_opt("affirm_payments", self.affirm_payments.as_ref());
        form.field_opt("afterpay_clearpay_payments", self.afterpay_clearpay_payments.as_ref());
        form.field_opt("alma_payments", self.alma_payments.as_ref());
        form.field_opt("amazon_pay_payments", self.amazon_pay_payments.as_ref());
        form.field_opt("app_distribution", self.app_distribution.as_ref());
        form.field_opt("au_becs_debit_payments", self.au_becs_debit_payments.as_ref());
        form.field_opt("bacs_debit_payments", self.bacs_debit_payments.as_ref());
        form.field_opt("bancontact_payments", self.bancontact_payments.as_ref());
        form.field_opt("bank_transfer_payments", self.bank_transfer_payments.as_ref());
        form.field_opt("billie_payments", self.billie_payments.as_ref());
        form.field_opt("bizum_payments", self.bizum_payments.as_ref());
        form.field_opt("blik_payments", self.blik_payments.as_ref());
        form.field_opt("blik_recurring_payments", self.blik_recurring_payments.as_ref());
        form.field_opt("boleto_payments", self.boleto_payments.as_ref());
        form.field_opt("card_issuing", self.card_issuing.as_ref());
        form.field_opt("card_payments", self.card_payments.as_ref());
        form.field_opt("cartes_bancaires_payments", self.cartes_bancaires_payments.as_ref());
        form.field_opt("cashapp_payments", self.cashapp_payments.as_ref());
        form.field_opt("crypto_payments", self.crypto_payments.as_ref());
        form.field_opt("eps_payments", self.eps_payments.as_ref());
        form.field_opt("fpx_payments", self.fpx_payments.as_ref());
        form.field_opt("gb_bank_transfer_payments", self.gb_bank_transfer_payments.as_ref());
        form.field_opt("giropay_payments", self.giropay_payments.as_ref());
        form.field_opt("grabpay_payments", self.grabpay_payments.as_ref());
        form.field_opt("ideal_payments", self.ideal_payments.as_ref());
        form.field_opt("india_international_payments", self.india_international_payments.as_ref());
        form.field_opt("jcb_payments", self.jcb_payments.as_ref());
        form.field_opt("jp_bank_transfer_payments", self.jp_bank_transfer_payments.as_ref());
        form.field_opt("kakao_pay_payments", self.kakao_pay_payments.as_ref());
        form.field_opt("klarna_payments", self.klarna_payments.as_ref());
        form.field_opt("konbini_payments", self.konbini_payments.as_ref());
        form.field_opt("kr_card_payments", self.kr_card_payments.as_ref());
        form.field_opt("legacy_payments", self.legacy_payments.as_ref());
        form.field_opt("link_payments", self.link_payments.as_ref());
        form.field_opt("mb_way_payments", self.mb_way_payments.as_ref());
        form.field_opt("mobilepay_payments", self.mobilepay_payments.as_ref());
        form.field_opt("multibanco_payments", self.multibanco_payments.as_ref());
        form.field_opt("mx_bank_transfer_payments", self.mx_bank_transfer_payments.as_ref());
        form.field_opt("naver_pay_payments", self.naver_pay_payments.as_ref());
        form.field_opt(
            "nz_bank_account_becs_debit_payments",
            self.nz_bank_account_becs_debit_payments.as_ref(),
        );
        form.field_opt("oxxo_payments", self.oxxo_payments.as_ref());
        form.field_opt("p24_payments", self.p24_payments.as_ref());
        form.field_opt("pay_by_bank_payments", self.pay_by_bank_payments.as_ref());
        form.field_opt("payco_payments", self.payco_payments.as_ref());
        form.field_opt("paynow_payments", self.paynow_payments.as_ref());
        form.field_opt("paypay_payments", self.paypay_payments.as_ref());
        form.field_opt("payto_payments", self.payto_payments.as_ref());
        form.field_opt("pix_payments", self.pix_payments.as_ref());
        form.field_opt("promptpay_payments", self.promptpay_payments.as_ref());
        form.field_opt("revolut_pay_payments", self.revolut_pay_payments.as_ref());
        form.field_opt("samsung_pay_payments", self.samsung_pay_payments.as_ref());
        form.field_opt("satispay_payments", self.satispay_payments.as_ref());
        form.field_opt("scalapay_payments", self.scalapay_payments.as_ref());
        form.field_opt("sepa_bank_transfer_payments", self.sepa_bank_transfer_payments.as_ref());
        form.field_opt("sepa_debit_payments", self.sepa_debit_payments.as_ref());
        form.field_opt("sequra_payments", self.sequra_payments.as_ref());
        form.field_opt("sofort_payments", self.sofort_payments.as_ref());
        form.field_opt("sunbit_payments", self.sunbit_payments.as_ref());
        form.field_opt("swish_payments", self.swish_payments.as_ref());
        form.field_opt("tax_reporting_us_1099_k", self.tax_reporting_us_1099_k.as_ref());
        form.field_opt("tax_reporting_us_1099_misc", self.tax_reporting_us_1099_misc.as_ref());
        form.field_opt("transfers", self.transfers.as_ref());
        form.field_opt("treasury", self.treasury.as_ref());
        form.field_opt("twint_payments", self.twint_payments.as_ref());
        form.field_opt("upi_payments", self.upi_payments.as_ref());
        form.field_opt("us_bank_account_ach_payments", self.us_bank_account_ach_payments.as_ref());
        form.field_opt("us_bank_transfer_payments", self.us_bank_transfer_payments.as_ref());
        form.field_opt("zip_payments", self.zip_payments.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct UpdateAccountParamsCompanyVerification {
    #[doc = "A document verifying the business."]
    pub document: Option<stripe_types_06::accounts::UpdateAccountParamsCompanyVerificationDocument>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for UpdateAccountParamsCompanyVerification {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("document", self.document.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct UpdateAccountParamsDocumentsProofOfRegistration {
    #[doc = "One or more document ids returned by a [file upload](https://docs.stripe.com/api#create_file) with a `purpose` value ..."]
    pub files: Option<Vec<String>>,
    #[doc = "Information regarding the person signing the document if applicable."]
    pub signer: Option<stripe_types_05::accounts::UpdateAccountParamsDocumentsProofOfRegistrationSigner>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for UpdateAccountParamsDocumentsProofOfRegistration {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.files.as_ref() {
            form.field_array("files", values.as_slice());
        }
        form.field_opt("signer", self.signer.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct UpdateAccountParamsIndividualVerification {
    #[doc = "A document showing address, either a passport, local ID card, or utility bill from a well-known utility company."]
    pub additional_document:
        Option<stripe_types_06::accounts::UpdateAccountParamsIndividualVerificationAdditionalDocument>,
    #[doc = "An identifying document, either a passport or local ID card."]
    pub document: Option<stripe_types_06::accounts::UpdateAccountParamsIndividualVerificationDocument>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for UpdateAccountParamsIndividualVerification {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("additional_document", self.additional_document.as_ref());
        form.field_opt("document", self.document.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct UpdateAccountParamsSettingsPayoutsSchedule {
    #[doc = "The number of days charge funds are held before being paid out."]
    pub delay_days: Option<json::OwnedValue>,
    #[doc = "How frequently available funds are paid out."]
    pub interval: Option<stripe_types_05::accounts::UpdateAccountParamsSettingsPayoutsScheduleInterval>,
    #[doc = "The day of the month when available funds are paid out, specified as a number between 1--31."]
    pub monthly_anchor: Option<i64>,
    #[doc = "The days of the month when available funds are paid out, specified as an array of numbers between 1--31."]
    pub monthly_payout_days: Option<Vec<i64>>,
    #[doc = "The day of the week when available funds are paid out, specified as `monday`, `tuesday`, etc."]
    pub weekly_anchor: Option<stripe_types_06::accounts::UpdateAccountParamsSettingsPayoutsScheduleWeeklyAnchor>,
    #[doc = "The days of the week when available funds are paid out, specified as an array, e.g., `monday`, `tuesday`."]
    pub weekly_payout_days:
        Option<Vec<stripe_types_06::accounts::UpdateAccountParamsSettingsPayoutsScheduleWeeklyPayoutDaysItem>>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for UpdateAccountParamsSettingsPayoutsSchedule {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("interval", self.interval.as_ref());
        form.field_opt("monthly_anchor", self.monthly_anchor.as_ref());
        if let Some(values) = self.monthly_payout_days.as_ref() {
            form.field_array("monthly_payout_days", values.as_slice());
        }
        form.field_opt("weekly_anchor", self.weekly_anchor.as_ref());
        if let Some(values) = self.weekly_payout_days.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("weekly_payout_days", index, item);
            }
        }
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct UpdateAccountParamsSettingsPaypayPaymentsSite {
    #[doc = "Additional information about your business's website."]
    pub accessible: Option<stripe_types_06::accounts::UpdateAccountParamsSettingsPaypayPaymentsSiteAccessible>,
    #[doc = "Additional information about your business's website."]
    pub in_development: Option<stripe_types_06::accounts::UpdateAccountParamsSettingsPaypayPaymentsSiteInDevelopment>,
    #[doc = "Additional information about your business's website."]
    pub restricted: Option<stripe_types_05::accounts::UpdateAccountParamsSettingsPaypayPaymentsSiteRestricted>,
    #[doc = "The status of your business's website."]
    pub r#type: Option<stripe_types_05::accounts::UpdateAccountParamsSettingsPaypayPaymentsSiteType>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for UpdateAccountParamsSettingsPaypayPaymentsSite {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("accessible", self.accessible.as_ref());
        form.field_opt("in_development", self.in_development.as_ref());
        form.field_opt("restricted", self.restricted.as_ref());
        form.field_opt("type", self.r#type.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct UpdateAccountPersonParamsAdditionalTosAcceptances {
    #[doc = "Details on the legal guardian's acceptance of the main Stripe service agreement."]
    pub account: Option<stripe_types_05::accounts::UpdateAccountPersonParamsAdditionalTosAcceptancesAccount>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for UpdateAccountPersonParamsAdditionalTosAcceptances {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct UpdateAccountPersonParamsUsCfpbDataRaceDetails {
    #[doc = "The persons race."]
    pub race: Option<Vec<stripe_types_06::accounts::UpdateAccountPersonParamsUsCfpbDataRaceDetailsRaceItem>>,
    #[doc = "Please specify your race, when other is selected."]
    pub race_other: Option<String>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for UpdateAccountPersonParamsUsCfpbDataRaceDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.race.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("race", index, item);
            }
        }
        form.field_opt("race_other", self.race_other.as_ref());
    }
}
