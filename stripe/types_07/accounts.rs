#[cfg(feature = "account_sessions")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct ConnectEmbeddedAccountSessionCreateComponents {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_management: Option<stripe_types_06::accounts::ConnectEmbeddedAccountConfigClaim>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_onboarding: Option<stripe_types_06::accounts::ConnectEmbeddedAccountConfigClaim>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub balance_report: Option<stripe_types_06::accounts::ConnectEmbeddedBaseConfigClaim>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub balances: Option<stripe_types_06::accounts::ConnectEmbeddedPayoutsConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub disputes_list: Option<stripe_types_05::accounts::ConnectEmbeddedDisputesListConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub documents: Option<stripe_types_06::accounts::ConnectEmbeddedBaseConfigClaim>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<stripe_types_06::accounts::ConnectEmbeddedFinancialAccountConfigClaim>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub financial_account_transactions:
        Option<stripe_types_05::accounts::ConnectEmbeddedFinancialAccountTransactionsConfigClaim>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub instant_payouts_promotion: Option<stripe_types_06::accounts::ConnectEmbeddedInstantPayoutsPromotionConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub issuing_card: Option<stripe_types_06::accounts::ConnectEmbeddedIssuingCardConfigClaim>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub issuing_cards_list: Option<stripe_types_05::accounts::ConnectEmbeddedIssuingCardsListConfigClaim>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub notification_banner: Option<stripe_types_06::accounts::ConnectEmbeddedAccountConfigClaim>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_details: Option<stripe_types_05::accounts::ConnectEmbeddedPaymentsConfigClaim>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_disputes: Option<stripe_types_05::accounts::ConnectEmbeddedPaymentDisputesConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_method_settings: Option<stripe_types_05::accounts::ConnectEmbeddedPaymentMethodSettingsConfigClaim>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payments: Option<stripe_types_05::accounts::ConnectEmbeddedPaymentsConfigClaim>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payout_details: Option<stripe_types_06::accounts::ConnectEmbeddedBaseConfigClaim>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payout_reconciliation_report: Option<stripe_types_06::accounts::ConnectEmbeddedBaseConfigClaim>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payouts: Option<stripe_types_06::accounts::ConnectEmbeddedPayoutsConfig>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payouts_list: Option<stripe_types_06::accounts::ConnectEmbeddedBaseConfigClaim>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax_registrations: Option<stripe_types_06::accounts::ConnectEmbeddedBaseConfigClaim>,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax_settings: Option<stripe_types_06::accounts::ConnectEmbeddedBaseConfigClaim>,
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsCapabilities {
    #[doc = "The acss_debit_payments capability."]
    pub acss_debit_payments: Option<stripe_types_04::accounts::CreateAccountParamsCapabilitiesAcssDebitPayments>,
    #[doc = "The affirm_payments capability."]
    pub affirm_payments: Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesAffirmPayments>,
    #[doc = "The afterpay_clearpay_payments capability."]
    pub afterpay_clearpay_payments:
        Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesAfterpayClearpayPayments>,
    #[doc = "The alma_payments capability."]
    pub alma_payments: Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesAlmaPayments>,
    #[doc = "The amazon_pay_payments capability."]
    pub amazon_pay_payments: Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesAmazonPayPayments>,
    #[doc = "The app_distribution capability."]
    pub app_distribution: Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesAppDistribution>,
    #[doc = "The au_becs_debit_payments capability."]
    pub au_becs_debit_payments: Option<stripe_types_06::accounts::CreateAccountParamsCapabilitiesAuBecsDebitPayments>,
    #[doc = "The bacs_debit_payments capability."]
    pub bacs_debit_payments: Option<stripe_types_06::accounts::CreateAccountParamsCapabilitiesBacsDebitPayments>,
    #[doc = "The bancontact_payments capability."]
    pub bancontact_payments: Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesBancontactPayments>,
    #[doc = "The bank_transfer_payments capability."]
    pub bank_transfer_payments: Option<stripe_types_06::accounts::CreateAccountParamsCapabilitiesBankTransferPayments>,
    #[doc = "The billie_payments capability."]
    pub billie_payments: Option<stripe_types_06::accounts::CreateAccountParamsCapabilitiesBilliePayments>,
    #[doc = "The bizum_payments capability."]
    pub bizum_payments: Option<stripe_types_06::accounts::CreateAccountParamsCapabilitiesBizumPayments>,
    #[doc = "The blik_payments capability."]
    pub blik_payments: Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesBlikPayments>,
    #[doc = "The blik_recurring_payments capability."]
    pub blik_recurring_payments:
        Option<stripe_types_06::accounts::CreateAccountParamsCapabilitiesBlikRecurringPayments>,
    #[doc = "The boleto_payments capability."]
    pub boleto_payments: Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesBoletoPayments>,
    #[doc = "The card_issuing capability."]
    pub card_issuing: Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesCardIssuing>,
    #[doc = "The card_payments capability."]
    pub card_payments: Option<stripe_types_06::accounts::CreateAccountParamsCapabilitiesCardPayments>,
    #[doc = "The cartes_bancaires_payments capability."]
    pub cartes_bancaires_payments:
        Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesCartesBancairesPayments>,
    #[doc = "The cashapp_payments capability."]
    pub cashapp_payments: Option<stripe_types_06::accounts::CreateAccountParamsCapabilitiesCashappPayments>,
    #[doc = "The crypto_payments capability."]
    pub crypto_payments: Option<stripe_types_06::accounts::CreateAccountParamsCapabilitiesCryptoPayments>,
    #[doc = "The eps_payments capability."]
    pub eps_payments: Option<stripe_types_06::accounts::CreateAccountParamsCapabilitiesEpsPayments>,
    #[doc = "The fpx_payments capability."]
    pub fpx_payments: Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesFpxPayments>,
    #[doc = "The gb_bank_transfer_payments capability."]
    pub gb_bank_transfer_payments:
        Option<stripe_types_06::accounts::CreateAccountParamsCapabilitiesGbBankTransferPayments>,
    #[doc = "The giropay_payments capability."]
    pub giropay_payments: Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesGiropayPayments>,
    #[doc = "The grabpay_payments capability."]
    pub grabpay_payments: Option<stripe_types_06::accounts::CreateAccountParamsCapabilitiesGrabpayPayments>,
    #[doc = "The ideal_payments capability."]
    pub ideal_payments: Option<stripe_types_06::accounts::CreateAccountParamsCapabilitiesIdealPayments>,
    #[doc = "The india_international_payments capability."]
    pub india_international_payments:
        Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesIndiaInternationalPayments>,
    #[doc = "The jcb_payments capability."]
    pub jcb_payments: Option<stripe_types_06::accounts::CreateAccountParamsCapabilitiesJcbPayments>,
    #[doc = "The jp_bank_transfer_payments capability."]
    pub jp_bank_transfer_payments:
        Option<stripe_types_06::accounts::CreateAccountParamsCapabilitiesJpBankTransferPayments>,
    #[doc = "The kakao_pay_payments capability."]
    pub kakao_pay_payments: Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesKakaoPayPayments>,
    #[doc = "The klarna_payments capability."]
    pub klarna_payments: Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesKlarnaPayments>,
    #[doc = "The konbini_payments capability."]
    pub konbini_payments: Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesKonbiniPayments>,
    #[doc = "The kr_card_payments capability."]
    pub kr_card_payments: Option<stripe_types_06::accounts::CreateAccountParamsCapabilitiesKrCardPayments>,
    #[doc = "The legacy_payments capability."]
    pub legacy_payments: Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesLegacyPayments>,
    #[doc = "The link_payments capability."]
    pub link_payments: Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesLinkPayments>,
    #[doc = "The mb_way_payments capability."]
    pub mb_way_payments: Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesMbWayPayments>,
    #[doc = "The mobilepay_payments capability."]
    pub mobilepay_payments: Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesMobilepayPayments>,
    #[doc = "The multibanco_payments capability."]
    pub multibanco_payments: Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesMultibancoPayments>,
    #[doc = "The mx_bank_transfer_payments capability."]
    pub mx_bank_transfer_payments:
        Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesMxBankTransferPayments>,
    #[doc = "The naver_pay_payments capability."]
    pub naver_pay_payments: Option<stripe_types_06::accounts::CreateAccountParamsCapabilitiesNaverPayPayments>,
    #[doc = "The nz_bank_account_becs_debit_payments capability."]
    pub nz_bank_account_becs_debit_payments:
        Option<stripe_types_06::accounts::CreateAccountParamsCapabilitiesNzBankAccountBecsDebitPayments>,
    #[doc = "The oxxo_payments capability."]
    pub oxxo_payments: Option<stripe_types_06::accounts::CreateAccountParamsCapabilitiesOxxoPayments>,
    #[doc = "The p24_payments capability."]
    pub p24_payments: Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesP24Payments>,
    #[doc = "The pay_by_bank_payments capability."]
    pub pay_by_bank_payments: Option<stripe_types_06::accounts::CreateAccountParamsCapabilitiesPayByBankPayments>,
    #[doc = "The payco_payments capability."]
    pub payco_payments: Option<stripe_types_06::accounts::CreateAccountParamsCapabilitiesPaycoPayments>,
    #[doc = "The paynow_payments capability."]
    pub paynow_payments: Option<stripe_types_06::accounts::CreateAccountParamsCapabilitiesPaynowPayments>,
    #[doc = "The paypay_payments capability."]
    pub paypay_payments: Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesPaypayPayments>,
    #[doc = "The payto_payments capability."]
    pub payto_payments: Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesPaytoPayments>,
    #[doc = "The pix_payments capability."]
    pub pix_payments: Option<stripe_types_06::accounts::CreateAccountParamsCapabilitiesPixPayments>,
    #[doc = "The promptpay_payments capability."]
    pub promptpay_payments: Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesPromptpayPayments>,
    #[doc = "The revolut_pay_payments capability."]
    pub revolut_pay_payments: Option<stripe_types_06::accounts::CreateAccountParamsCapabilitiesRevolutPayPayments>,
    #[doc = "The samsung_pay_payments capability."]
    pub samsung_pay_payments: Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesSamsungPayPayments>,
    #[doc = "The satispay_payments capability."]
    pub satispay_payments: Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesSatispayPayments>,
    #[doc = "The scalapay_payments capability."]
    pub scalapay_payments: Option<stripe_types_06::accounts::CreateAccountParamsCapabilitiesScalapayPayments>,
    #[doc = "The sepa_bank_transfer_payments capability."]
    pub sepa_bank_transfer_payments:
        Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesSepaBankTransferPayments>,
    #[doc = "The sepa_debit_payments capability."]
    pub sepa_debit_payments: Option<stripe_types_06::accounts::CreateAccountParamsCapabilitiesSepaDebitPayments>,
    #[doc = "The sequra_payments capability."]
    pub sequra_payments: Option<stripe_types_06::accounts::CreateAccountParamsCapabilitiesSequraPayments>,
    #[doc = "The sofort_payments capability."]
    pub sofort_payments: Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesSofortPayments>,
    #[doc = "The sunbit_payments capability."]
    pub sunbit_payments: Option<stripe_types_06::accounts::CreateAccountParamsCapabilitiesSunbitPayments>,
    #[doc = "The swish_payments capability."]
    pub swish_payments: Option<stripe_types_06::accounts::CreateAccountParamsCapabilitiesSwishPayments>,
    #[doc = "The tax_reporting_us_1099_k capability."]
    pub tax_reporting_us_1099_k: Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesTaxReportingUs1099K>,
    #[doc = "The tax_reporting_us_1099_misc capability."]
    pub tax_reporting_us_1099_misc:
        Option<stripe_types_06::accounts::CreateAccountParamsCapabilitiesTaxReportingUs1099Misc>,
    #[doc = "The transfers capability."]
    pub transfers: Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesTransfers>,
    #[doc = "The treasury capability."]
    pub treasury: Option<stripe_types_06::accounts::CreateAccountParamsCapabilitiesTreasury>,
    #[doc = "The twint_payments capability."]
    pub twint_payments: Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesTwintPayments>,
    #[doc = "The upi_payments capability."]
    pub upi_payments: Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesUpiPayments>,
    #[doc = "The us_bank_account_ach_payments capability."]
    pub us_bank_account_ach_payments:
        Option<stripe_types_06::accounts::CreateAccountParamsCapabilitiesUsBankAccountAchPayments>,
    #[doc = "The us_bank_transfer_payments capability."]
    pub us_bank_transfer_payments:
        Option<stripe_types_05::accounts::CreateAccountParamsCapabilitiesUsBankTransferPayments>,
    #[doc = "The zip_payments capability."]
    pub zip_payments: Option<stripe_types_06::accounts::CreateAccountParamsCapabilitiesZipPayments>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsCapabilities {
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
pub struct CreateAccountParamsCompany {
    #[doc = "The company's primary address."]
    pub address: Option<stripe_types_05::accounts::CreateAccountParamsCompanyAddress>,
    #[doc = "The Kana variation of the company's primary address (Japan only)."]
    pub address_kana: Option<stripe_types_04::accounts::CreateAccountParamsCompanyAddressKana>,
    #[doc = "The Kanji variation of the company's primary address (Japan only)."]
    pub address_kanji: Option<stripe_types_06::accounts::CreateAccountParamsCompanyAddressKanji>,
    #[doc = "The location where the business is administered."]
    pub administrative_address: Option<stripe_types_05::accounts::CreateAccountParamsCompanyAdministrativeAddress>,
    #[doc = "Whether the company's directors have been provided."]
    pub directors_provided: Option<bool>,
    #[doc = "This hash is used to attest that the directors information provided to Stripe is both current and correct."]
    pub directorship_declaration: Option<stripe_types_04::accounts::CreateAccountParamsCompanyDirectorshipDeclaration>,
    #[doc = "Whether the company's executives have been provided."]
    pub executives_provided: Option<bool>,
    #[doc = "The export license ID number of the company, also referred as Import Export Code (India only)."]
    pub export_license_id: Option<String>,
    #[doc = "The purpose code to use for export transactions (India only)."]
    pub export_purpose_code: Option<String>,
    #[doc = "The company's legal name."]
    pub name: Option<String>,
    #[doc = "The Kana variation of the company's legal name (Japan only)."]
    pub name_kana: Option<String>,
    #[doc = "The Kanji variation of the company's legal name (Japan only)."]
    pub name_kanji: Option<String>,
    #[doc = "Whether the company's owners have been provided."]
    pub owners_provided: Option<bool>,
    #[doc = "This hash is used to attest that the beneficial owner information provided to Stripe is both current and correct."]
    pub ownership_declaration: Option<stripe_types_06::accounts::CreateAccountParamsCompanyOwnershipDeclaration>,
    #[doc = "This value is used to determine if a business is exempt from providing ultimate beneficial owners."]
    pub ownership_exemption_reason:
        Option<stripe_types_03::accounts::CreateAccountParamsCompanyOwnershipExemptionReason>,
    #[doc = "The company's phone number (used for verification)."]
    pub phone: Option<String>,
    #[doc = "The primary location where the business conducts operations."]
    pub principal_place_of_business:
        Option<stripe_types_06::accounts::CreateAccountParamsCompanyPrincipalPlaceOfBusiness>,
    #[doc = "When the business was incorporated or registered."]
    pub registration_date: Option<stripe_types_03::accounts::CreateAccountParamsCompanyRegistrationDate>,
    #[doc = "The identification number given to a company when it is registered or incorporated, if distinct from the identificati..."]
    pub registration_number: Option<String>,
    #[doc = "This hash is used to attest that the representative is authorized to act as the representative of their legal entity."]
    pub representative_declaration:
        Option<stripe_types_05::accounts::CreateAccountParamsCompanyRepresentativeDeclaration>,
    #[doc = "The category identifying the legal structure of the company or legal entity."]
    pub structure: Option<stripe_types_04::accounts::CreateAccountParamsCompanyStructure>,
    #[doc = "The business ID number of the company, as appropriate for the company’s country."]
    pub tax_id: Option<String>,
    #[doc = "The jurisdiction in which the `tax_id` is registered (Germany-based companies only)."]
    pub tax_id_registrar: Option<String>,
    #[doc = "The VAT number of the company."]
    pub vat_id: Option<String>,
    #[doc = "Information on the verification state of the company."]
    pub verification: Option<stripe_types_06::accounts::CreateAccountParamsCompanyVerification>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsCompany {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("address", self.address.as_ref());
        form.field_opt("address_kana", self.address_kana.as_ref());
        form.field_opt("address_kanji", self.address_kanji.as_ref());
        form.field_opt("administrative_address", self.administrative_address.as_ref());
        form.field_opt("directors_provided", self.directors_provided.as_ref());
        form.field_opt("directorship_declaration", self.directorship_declaration.as_ref());
        form.field_opt("executives_provided", self.executives_provided.as_ref());
        form.field_opt("export_license_id", self.export_license_id.as_ref());
        form.field_opt("export_purpose_code", self.export_purpose_code.as_ref());
        form.field_opt("name", self.name.as_ref());
        form.field_opt("name_kana", self.name_kana.as_ref());
        form.field_opt("name_kanji", self.name_kanji.as_ref());
        form.field_opt("owners_provided", self.owners_provided.as_ref());
        form.field_opt("ownership_declaration", self.ownership_declaration.as_ref());
        form.field_opt("ownership_exemption_reason", self.ownership_exemption_reason.as_ref());
        form.field_opt("phone", self.phone.as_ref());
        form.field_opt("principal_place_of_business", self.principal_place_of_business.as_ref());
        form.field_opt("registration_date", self.registration_date.as_ref());
        form.field_opt("registration_number", self.registration_number.as_ref());
        form.field_opt("representative_declaration", self.representative_declaration.as_ref());
        form.field_opt("structure", self.structure.as_ref());
        form.field_opt("tax_id", self.tax_id.as_ref());
        form.field_opt("tax_id_registrar", self.tax_id_registrar.as_ref());
        form.field_opt("vat_id", self.vat_id.as_ref());
        form.field_opt("verification", self.verification.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsIndividualVerification {
    #[doc = "A document showing address, either a passport, local ID card, or utility bill from a well-known utility company."]
    pub additional_document:
        Option<stripe_types_06::accounts::CreateAccountParamsIndividualVerificationAdditionalDocument>,
    #[doc = "An identifying document, either a passport or local ID card."]
    pub document: Option<stripe_types_05::accounts::CreateAccountParamsIndividualVerificationDocument>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsIndividualVerification {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("additional_document", self.additional_document.as_ref());
        form.field_opt("document", self.document.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountParamsSettingsTreasury {
    #[doc = "Details on the account's acceptance of the Stripe Treasury Services Agreement."]
    pub tos_acceptance: Option<stripe_types_05::accounts::CreateAccountParamsSettingsTreasuryTosAcceptance>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountParamsSettingsTreasury {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("tos_acceptance", self.tos_acceptance.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountPersonParamsAdditionalTosAcceptances {
    #[doc = "Details on the legal guardian's acceptance of the main Stripe service agreement."]
    pub account: Option<stripe_types_06::accounts::CreateAccountPersonParamsAdditionalTosAcceptancesAccount>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountPersonParamsAdditionalTosAcceptances {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("account", self.account.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct CreateAccountPersonParamsDocuments {
    #[doc = "One or more documents that demonstrate proof that this person is authorized to represent the company."]
    pub company_authorization:
        Option<stripe_types_01::accounts::CreateAccountPersonParamsDocumentsCompanyAuthorization>,
    #[doc = "One or more documents showing the person's passport page with photo and personal data."]
    pub passport: Option<stripe_types_05::accounts::CreateAccountPersonParamsDocumentsPassport>,
    #[doc = "One or more documents showing the person's visa required for living in the country where they are residing."]
    pub visa: Option<stripe_types_06::accounts::CreateAccountPersonParamsDocumentsVisa>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for CreateAccountPersonParamsDocuments {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("company_authorization", self.company_authorization.as_ref());
        form.field_opt("passport", self.passport.as_ref());
        form.field_opt("visa", self.visa.as_ref());
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
pub struct ExternalAccountRequirements {
    #[doc = "Fields that need to be resolved to keep the external account enabled."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub currently_due: Option<Vec<String>>,
    #[doc = "Fields that are `currently_due` and need to be collected again because validation or verification failed."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<stripe_types_06::account::AccountRequirementsError>>,
    #[doc = "Fields that haven't been resolved by `current_deadline`."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub past_due: Option<Vec<String>>,
    #[doc = "Fields that are being reviewed, or might become required depending on the results of a review."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub pending_verification: Option<Vec<String>>,
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
pub struct PersonRequirements {
    #[doc = "Fields that are due and can be resolved by providing the corresponding alternative fields instead."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub alternatives: Option<Vec<stripe_types_01::account::AccountRequirementsAlternative>>,
    #[doc = "Fields that need to be resolved to keep the person's account enabled."]
    #[json(default)]
    pub currently_due: Vec<String>,
    #[doc = "Fields that are `currently_due` and need to be collected again because validation or verification failed."]
    #[json(default)]
    pub errors: Vec<stripe_types_06::account::AccountRequirementsError>,
    #[doc = "Fields you must collect when all thresholds are reached."]
    #[json(default)]
    pub eventually_due: Vec<String>,
    #[doc = "Fields that haven't been resolved by `current_deadline`."]
    #[json(default)]
    pub past_due: Vec<String>,
    #[doc = "Fields that are being reviewed, or might become required depending on the results of a review."]
    #[json(default)]
    pub pending_verification: Vec<String>,
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
pub struct PersonUsCfpbData {
    #[doc = "The persons ethnicity details"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ethnicity_details: Option<stripe_types_05::accounts::PersonEthnicityDetails>,
    #[doc = "The persons race details"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub race_details: Option<stripe_types_06::accounts::PersonRaceDetails>,
    #[doc = "The persons self-identified gender"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub self_identified_gender: Option<String>,
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct UpdateAccountExternalAccountParamsDocuments {
    #[doc = "One or more documents that support the Bank account ownership verification..."]
    pub bank_account_ownership_verification:
        Option<stripe_types_05::accounts::UpdateAccountExternalAccountParamsDocumentsBankAccountOwnershipVerification>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for UpdateAccountExternalAccountParamsDocuments {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt(
            "bank_account_ownership_verification",
            self.bank_account_ownership_verification.as_ref(),
        );
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct UpdateAccountParamsBusinessProfile {
    #[doc = "The applicant's gross annual revenue for its preceding fiscal year."]
    pub annual_revenue: Option<stripe_types_05::accounts::UpdateAccountParamsBusinessProfileAnnualRevenue>,
    #[doc = "An estimated upper bound of employees, contractors, vendors, etc."]
    pub estimated_worker_count: Option<i64>,
    #[doc = "[The merchant category code for the account](/connect/setting-mcc)."]
    pub mcc: Option<String>,
    #[doc = "Whether the business is a minority-owned, women-owned, and/or LGBTQI+ -owned business."]
    pub minority_owned_business_designation:
        Option<Vec<stripe_types_06::accounts::UpdateAccountParamsBusinessProfileMinorityOwnedBusinessDesignationItem>>,
    #[doc = "An estimate of the monthly revenue of the business."]
    pub monthly_estimated_revenue:
        Option<stripe_types_06::accounts::UpdateAccountParamsBusinessProfileMonthlyEstimatedRevenue>,
    #[doc = "The customer-facing business name."]
    pub name: Option<String>,
    #[doc = "Internal-only description of the product sold by, or service provided by, the business."]
    pub product_description: Option<String>,
    #[doc = "A link to the business's publicly available terms related to the Specified Commercial Transaction Act."]
    pub specified_commercial_transactions_act_url: Option<json::OwnedValue>,
    #[doc = "A publicly available mailing address for sending support issues to."]
    pub support_address: Option<stripe_types_06::accounts::UpdateAccountParamsBusinessProfileSupportAddress>,
    #[doc = "A publicly available email address for sending support issues to."]
    pub support_email: Option<String>,
    #[doc = "A publicly available phone number to call with support issues."]
    pub support_phone: Option<String>,
    #[doc = "A publicly available website for handling support issues."]
    pub support_url: Option<json::OwnedValue>,
    #[doc = "The business's publicly available website."]
    pub url: Option<String>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for UpdateAccountParamsBusinessProfile {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("annual_revenue", self.annual_revenue.as_ref());
        form.field_opt("estimated_worker_count", self.estimated_worker_count.as_ref());
        form.field_opt("mcc", self.mcc.as_ref());
        if let Some(values) = self.minority_owned_business_designation.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("minority_owned_business_designation", index, item);
            }
        }
        form.field_opt("monthly_estimated_revenue", self.monthly_estimated_revenue.as_ref());
        form.field_opt("name", self.name.as_ref());
        form.field_opt("product_description", self.product_description.as_ref());
        form.field_opt("support_address", self.support_address.as_ref());
        form.field_opt("support_email", self.support_email.as_ref());
        form.field_opt("support_phone", self.support_phone.as_ref());
        form.field_opt("url", self.url.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct UpdateAccountParamsSettingsInvoices {
    #[doc = "The list of default Account Tax IDs to automatically include on invoices."]
    pub default_account_tax_ids: Option<json::OwnedValue>,
    #[doc = "Whether to save the payment method after a payment is completed for a one-time invoice or a subscription invoice when..."]
    pub hosted_payment_method_save:
        Option<stripe_types_06::accounts::UpdateAccountParamsSettingsInvoicesHostedPaymentMethodSave>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for UpdateAccountParamsSettingsInvoices {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("hosted_payment_method_save", self.hosted_payment_method_save.as_ref());
    }
}

#[cfg(feature = "accounts")]
#[derive(Debug, Clone, Default)]
pub struct UpdateAccountPersonParamsUsCfpbDataEthnicityDetails {
    #[doc = "The persons ethnicity"]
    pub ethnicity:
        Option<Vec<stripe_types_06::accounts::UpdateAccountPersonParamsUsCfpbDataEthnicityDetailsEthnicityItem>>,
    #[doc = "Please specify your origin, when other is selected."]
    pub ethnicity_other: Option<String>,
}

#[cfg(feature = "accounts")]
impl crate::params::FormEncode for UpdateAccountPersonParamsUsCfpbDataEthnicityDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.ethnicity.as_ref() {
            for (index, item) in values.iter().enumerate() {
                form.field_indexed("ethnicity", index, item);
            }
        }
        form.field_opt("ethnicity_other", self.ethnicity_other.as_ref());
    }
}
