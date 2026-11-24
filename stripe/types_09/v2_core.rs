#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountLinkParamsUseCaseAccountOnboarding {
    #[doc = "Specifies the requirements that Stripe collects from v2/core/accounts in the Onboarding flow."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub collection_options:
        Option<stripe_types_05::v2_core::CreateCoreAccountLinkParamsUseCaseAccountOnboardingCollectionOptions>,
    #[doc = "The URL the user will be redirected to if the AccountLink is expired, has been used, or is otherwise invalid."]
    pub refresh_url: String,
    #[doc = "The URL that the user will be redirected to upon completing the linked flow."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsConfigurationMerchantCapabilities {
    #[doc = "Allow the merchant to process ACH debit payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub ach_debit_payments:
        Option<stripe_types_05::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesAchDebitPayments>,
    #[doc = "Allow the merchant to process ACSS debit payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub acss_debit_payments:
        Option<stripe_types_05::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesAcssDebitPayments>,
    #[doc = "Allow the merchant to process Affirm payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub affirm_payments:
        Option<stripe_types_05::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesAffirmPayments>,
    #[doc = "Allow the merchant to process Afterpay/Clearpay payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay_payments: Option<
        stripe_types_06::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesAfterpayClearpayPayments,
    >,
    #[doc = "Allow the merchant to process Alma payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub alma_payments:
        Option<stripe_types_06::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesAlmaPayments>,
    #[doc = "Allow the merchant to process Amazon Pay payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub amazon_pay_payments:
        Option<stripe_types_06::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesAmazonPayPayments>,
    #[doc = "Allow the merchant to process Australian BECS Direct Debit payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit_payments:
        Option<stripe_types_06::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesAuBecsDebitPayments>,
    #[doc = "Allow the merchant to process BACS Direct Debit payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub bacs_debit_payments:
        Option<stripe_types_05::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesBacsDebitPayments>,
    #[doc = "Allow the merchant to process Bancontact payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub bancontact_payments:
        Option<stripe_types_05::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesBancontactPayments>,
    #[doc = "Allow the merchant to process BLIK payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub blik_payments:
        Option<stripe_types_05::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesBlikPayments>,
    #[doc = "Allow the merchant to process recurring BLIK payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub blik_recurring_payments:
        Option<stripe_types_05::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesBlikRecurringPayments>,
    #[doc = "Allow the merchant to process Boleto payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub boleto_payments:
        Option<stripe_types_06::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesBoletoPayments>,
    #[doc = "Allow the merchant to collect card payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub card_payments:
        Option<stripe_types_06::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesCardPayments>,
    #[doc = "Allow the merchant to process Cartes Bancaires payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires_payments: Option<
        stripe_types_05::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesCartesBancairesPayments,
    >,
    #[doc = "Allow the merchant to process Cash App payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub cashapp_payments:
        Option<stripe_types_05::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesCashappPayments>,
    #[doc = "Allow the merchant to process EPS payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub eps_payments:
        Option<stripe_types_06::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesEpsPayments>,
    #[doc = "Allow the merchant to process FPX payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub fpx_payments:
        Option<stripe_types_05::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesFpxPayments>,
    #[doc = "Allow the merchant to process UK bank transfer payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub gb_bank_transfer_payments: Option<
        stripe_types_06::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesGbBankTransferPayments,
    >,
    #[doc = "Allow the merchant to process GrabPay payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub grabpay_payments:
        Option<stripe_types_06::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesGrabpayPayments>,
    #[doc = "Allow the merchant to process iDEAL payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub ideal_payments:
        Option<stripe_types_05::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesIdealPayments>,
    #[doc = "Allow the merchant to process JCB card payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub jcb_payments:
        Option<stripe_types_06::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesJcbPayments>,
    #[doc = "Allow the merchant to process Japanese bank transfer payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub jp_bank_transfer_payments: Option<
        stripe_types_05::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesJpBankTransferPayments,
    >,
    #[doc = "Allow the merchant to process Kakao Pay payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kakao_pay_payments:
        Option<stripe_types_06::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesKakaoPayPayments>,
    #[doc = "Allow the merchant to process Klarna payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub klarna_payments:
        Option<stripe_types_05::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesKlarnaPayments>,
    #[doc = "Allow the merchant to process Konbini convenience store payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub konbini_payments:
        Option<stripe_types_05::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesKonbiniPayments>,
    #[doc = "Allow the merchant to process Korean card payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kr_card_payments:
        Option<stripe_types_06::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesKrCardPayments>,
    #[doc = "Allow the merchant to process Link payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub link_payments:
        Option<stripe_types_05::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesLinkPayments>,
    #[doc = "Allow the merchant to process MobilePay payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub mobilepay_payments:
        Option<stripe_types_06::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesMobilepayPayments>,
    #[doc = "Allow the merchant to process Multibanco payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub multibanco_payments:
        Option<stripe_types_06::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesMultibancoPayments>,
    #[doc = "Allow the merchant to process Mexican bank transfer payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub mx_bank_transfer_payments: Option<
        stripe_types_06::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesMxBankTransferPayments,
    >,
    #[doc = "Allow the merchant to process Naver Pay payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub naver_pay_payments:
        Option<stripe_types_05::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesNaverPayPayments>,
    #[doc = "Allow the merchant to process OXXO payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub oxxo_payments:
        Option<stripe_types_06::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesOxxoPayments>,
    #[doc = "Allow the merchant to process Przelewy24 (P24) payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub p24_payments:
        Option<stripe_types_05::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesP24Payments>,
    #[doc = "Allow the merchant to process Pay by Bank payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub pay_by_bank_payments:
        Option<stripe_types_05::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesPayByBankPayments>,
    #[doc = "Allow the merchant to process PAYCO payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub payco_payments:
        Option<stripe_types_06::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesPaycoPayments>,
    #[doc = "Allow the merchant to process PayNow payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub paynow_payments:
        Option<stripe_types_05::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesPaynowPayments>,
    #[doc = "Allow the merchant to process PromptPay payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub promptpay_payments:
        Option<stripe_types_06::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesPromptpayPayments>,
    #[doc = "Allow the merchant to process Revolut Pay payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub revolut_pay_payments:
        Option<stripe_types_05::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesRevolutPayPayments>,
    #[doc = "Allow the merchant to process Samsung Pay payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub samsung_pay_payments:
        Option<stripe_types_06::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesSamsungPayPayments>,
    #[doc = "Allow the merchant to process Satispay payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub satispay_payments:
        Option<stripe_types_06::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesSatispayPayments>,
    #[doc = "Allow the merchant to process SEPA bank transfer payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub sepa_bank_transfer_payments: Option<
        stripe_types_05::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesSepaBankTransferPayments,
    >,
    #[doc = "Allow the merchant to process SEPA Direct Debit payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub sepa_debit_payments:
        Option<stripe_types_05::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesSepaDebitPayments>,
    #[doc = "Allow the merchant to process SeQura payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub sequra_payments:
        Option<stripe_types_06::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesSequraPayments>,
    #[doc = "Allow the merchant to process Sunbit payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub sunbit_payments:
        Option<stripe_types_05::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesSunbitPayments>,
    #[doc = "Allow the merchant to process Swish payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub swish_payments:
        Option<stripe_types_05::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesSwishPayments>,
    #[doc = "Allow the merchant to process TWINT payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub twint_payments:
        Option<stripe_types_06::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesTwintPayments>,
    #[doc = "Allow the merchant to process US bank transfer payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub us_bank_transfer_payments: Option<
        stripe_types_05::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesUsBankTransferPayments,
    >,
    #[doc = "Allow the merchant to process Zip payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub zip_payments:
        Option<stripe_types_06::v2_core::CreateCoreAccountParamsConfigurationMerchantCapabilitiesZipPayments>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityBusinessDetailsAdditionalAddressesItem {
    #[doc = "City, district, suburb, town, or village."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "Address line 1 (e.g., street, PO Box, or company name)."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    #[doc = "Address line 2 (e.g., apartment, suite, unit, or building)."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    #[doc = "ZIP or postal code."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[doc = "Purpose of additional address."]
    pub purpose: stripe_types_05::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsAdditionalAddressesItemPurpose,
    #[doc = "State, county, province, or region."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "Town or district."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityBusinessDetailsAnnualRevenue {
    #[doc = "A non-negative integer representing the amount in the smallest currency unit."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub amount: Option<stripe_types_05::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsAnnualRevenueAmount>,
    #[doc = "The close-out date of the preceding fiscal year in ISO 8601 format."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub fiscal_year_end: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyLicense {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_05::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyLicenseType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyMemorandumOfAssociation {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_05::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyMemorandumOfAssociationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyRegistrationVerification {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_06::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyRegistrationVerificationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyTaxIdVerification {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type:
        stripe_types_06::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyTaxIdVerificationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityBusinessDetailsDocumentsPrimaryVerification {
    #[doc = "The [file upload](https://docs.stripe.com/api/persons/update#create_file) tokens referring to each side of the document."]
    pub front_back:
        stripe_types_01::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsDocumentsPrimaryVerificationFrontBack,
    #[doc = "The format of the verification document."]
    #[json(rename = "type")]
    pub r#type:
        stripe_types_05::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsDocumentsPrimaryVerificationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityBusinessDetailsScriptNames {
    #[doc = "Kana name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kana: Option<stripe_types_06::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsScriptNamesKana>,
    #[doc = "Kanji name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kanji: Option<stripe_types_06::v2_core::CreateCoreAccountParamsIdentityBusinessDetailsScriptNamesKanji>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountParamsIdentityIndividualDocumentsSecondaryVerification {
    #[doc = "The [file upload](https://docs.stripe.com/api/persons/update#create_file) tokens referring to each side of the document."]
    pub front_back:
        stripe_types_05::v2_core::CreateCoreAccountParamsIdentityIndividualDocumentsSecondaryVerificationFrontBack,
    #[doc = "The format of the verification document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_05::v2_core::CreateCoreAccountParamsIdentityIndividualDocumentsSecondaryVerificationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountPersonParamsAdditionalNamesItem {
    #[doc = "The person's full name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[doc = "The person's first or given name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[doc = "The purpose or type of the additional name."]
    pub purpose: stripe_types_05::v2_core::CreateCoreAccountPersonParamsAdditionalNamesItemPurpose,
    #[doc = "The person's last or family name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountPersonParamsDocumentsVisa {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_06::v2_core::CreateCoreAccountPersonParamsDocumentsVisaType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountPersonTokenParamsAdditionalAddressesItem {
    #[doc = "City, district, suburb, town, or village."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "Address line 1 (e.g., street, PO Box, or company name)."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    #[doc = "Address line 2 (e.g., apartment, suite, unit, or building)."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    #[doc = "ZIP or postal code."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[doc = "Purpose of additional address."]
    pub purpose: stripe_types_06::v2_core::CreateCoreAccountPersonTokenParamsAdditionalAddressesItemPurpose,
    #[doc = "State, county, province, or region."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "Town or district."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountPersonTokenParamsAdditionalNamesItem {
    #[doc = "The person's full name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[doc = "The person's first or given name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[doc = "The purpose or type of the additional name."]
    pub purpose: stripe_types_06::v2_core::CreateCoreAccountPersonTokenParamsAdditionalNamesItemPurpose,
    #[doc = "The person's last or family name."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountPersonTokenParamsDocumentsCompanyAuthorization {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_06::v2_core::CreateCoreAccountPersonTokenParamsDocumentsCompanyAuthorizationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountPersonTokenParamsDocumentsPassport {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_06::v2_core::CreateCoreAccountPersonTokenParamsDocumentsPassportType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountPersonTokenParamsDocumentsPrimaryVerification {
    #[doc = "The [file upload](https://docs.stripe.com/api/persons/update#create_file) tokens referring to each side of the document."]
    pub front_back: stripe_types_06::v2_core::CreateCoreAccountPersonTokenParamsDocumentsPrimaryVerificationFrontBack,
    #[doc = "The format of the verification document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_05::v2_core::CreateCoreAccountPersonTokenParamsDocumentsPrimaryVerificationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountPersonTokenParamsDocumentsVisa {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_06::v2_core::CreateCoreAccountPersonTokenParamsDocumentsVisaType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountPersonTokenParamsIdNumbersItem {
    #[doc = "The ID number type of an individual."]
    #[json(rename = "type")]
    pub r#type: stripe_types_06::v2_core::CreateCoreAccountPersonTokenParamsIdNumbersItemType,
    #[doc = "The value of the ID number."]
    pub value: String,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsCompanyMemorandumOfAssociation {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_05::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsCompanyMemorandumOfAssociationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsCompanyMinisterialDecree {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_05::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsDocumentsCompanyMinisterialDecreeType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityBusinessDetailsMonthlyEstimatedRevenue {
    #[doc = "A non-negative integer representing the amount in the smallest currency unit."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub amount: Option<
        stripe_types_05::v2_core::CreateCoreAccountTokenParamsIdentityBusinessDetailsMonthlyEstimatedRevenueAmount,
    >,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityIndividualDocumentsSecondaryVerification {
    #[doc = "The [file upload](https://docs.stripe.com/api/persons/update#create_file) tokens referring to each side of the document."]
    pub front_back:
        stripe_types_05::v2_core::CreateCoreAccountTokenParamsIdentityIndividualDocumentsSecondaryVerificationFrontBack,
    #[doc = "The format of the verification document."]
    #[json(rename = "type")]
    pub r#type:
        stripe_types_06::v2_core::CreateCoreAccountTokenParamsIdentityIndividualDocumentsSecondaryVerificationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityIndividualScriptAddresses {
    #[doc = "Kana Address."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kana: Option<stripe_types_05::v2_core::CreateCoreAccountTokenParamsIdentityIndividualScriptAddressesKana>,
    #[doc = "Kanji Address."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kanji: Option<stripe_types_06::v2_core::CreateCoreAccountTokenParamsIdentityIndividualScriptAddressesKanji>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct CreateCoreAccountTokenParamsIdentityIndividualScriptNames {
    #[doc = "Persons name in kana script."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kana: Option<stripe_types_00::v2_core::CreateCoreAccountTokenParamsIdentityIndividualScriptNamesKana>,
    #[doc = "Persons name in kanji script."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kanji: Option<stripe_types_06::v2_core::CreateCoreAccountTokenParamsIdentityIndividualScriptNamesKanji>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsConfigurationCustomerAutomaticIndirectTax {
    #[doc = "The customer account's tax exemption status: `none`, `exempt`, or `reverse`."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub exempt:
        Option<stripe_types_05::v2_core::UpdateCoreAccountParamsConfigurationCustomerAutomaticIndirectTaxExempt>,
    #[doc = "A recent IP address of the customer used for tax reporting and tax location inference."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "A per-request flag that indicates when Stripe should validate the customer tax location - defaults to `auto`."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub validate_location: Option<
        stripe_types_01::v2_core::UpdateCoreAccountParamsConfigurationCustomerAutomaticIndirectTaxValidateLocation,
    >,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsConfigurationMerchantCapabilities {
    #[doc = "Allow the merchant to process ACH debit payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub ach_debit_payments:
        Option<stripe_types_06::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesAchDebitPayments>,
    #[doc = "Allow the merchant to process ACSS debit payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub acss_debit_payments:
        Option<stripe_types_06::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesAcssDebitPayments>,
    #[doc = "Allow the merchant to process Affirm payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub affirm_payments:
        Option<stripe_types_06::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesAffirmPayments>,
    #[doc = "Allow the merchant to process Afterpay/Clearpay payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay_payments: Option<
        stripe_types_05::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesAfterpayClearpayPayments,
    >,
    #[doc = "Allow the merchant to process Alma payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub alma_payments:
        Option<stripe_types_05::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesAlmaPayments>,
    #[doc = "Allow the merchant to process Amazon Pay payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub amazon_pay_payments:
        Option<stripe_types_05::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesAmazonPayPayments>,
    #[doc = "Allow the merchant to process Australian BECS Direct Debit payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit_payments:
        Option<stripe_types_06::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesAuBecsDebitPayments>,
    #[doc = "Allow the merchant to process BACS Direct Debit payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub bacs_debit_payments:
        Option<stripe_types_06::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesBacsDebitPayments>,
    #[doc = "Allow the merchant to process Bancontact payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub bancontact_payments:
        Option<stripe_types_06::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesBancontactPayments>,
    #[doc = "Allow the merchant to process BLIK payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub blik_payments:
        Option<stripe_types_06::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesBlikPayments>,
    #[doc = "Allow the merchant to process recurring BLIK payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub blik_recurring_payments:
        Option<stripe_types_06::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesBlikRecurringPayments>,
    #[doc = "Allow the merchant to process Boleto payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub boleto_payments:
        Option<stripe_types_05::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesBoletoPayments>,
    #[doc = "Allow the merchant to collect card payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub card_payments:
        Option<stripe_types_05::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesCardPayments>,
    #[doc = "Allow the merchant to process Cartes Bancaires payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires_payments: Option<
        stripe_types_06::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesCartesBancairesPayments,
    >,
    #[doc = "Allow the merchant to process Cash App payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub cashapp_payments:
        Option<stripe_types_05::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesCashappPayments>,
    #[doc = "Allow the merchant to process EPS payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub eps_payments:
        Option<stripe_types_05::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesEpsPayments>,
    #[doc = "Allow the merchant to process FPX payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub fpx_payments:
        Option<stripe_types_06::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesFpxPayments>,
    #[doc = "Allow the merchant to process UK bank transfer payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub gb_bank_transfer_payments: Option<
        stripe_types_05::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesGbBankTransferPayments,
    >,
    #[doc = "Allow the merchant to process GrabPay payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub grabpay_payments:
        Option<stripe_types_06::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesGrabpayPayments>,
    #[doc = "Allow the merchant to process iDEAL payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub ideal_payments:
        Option<stripe_types_06::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesIdealPayments>,
    #[doc = "Allow the merchant to process JCB card payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub jcb_payments:
        Option<stripe_types_05::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesJcbPayments>,
    #[doc = "Allow the merchant to process Japanese bank transfer payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub jp_bank_transfer_payments: Option<
        stripe_types_06::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesJpBankTransferPayments,
    >,
    #[doc = "Allow the merchant to process Kakao Pay payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kakao_pay_payments:
        Option<stripe_types_05::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesKakaoPayPayments>,
    #[doc = "Allow the merchant to process Klarna payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub klarna_payments:
        Option<stripe_types_06::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesKlarnaPayments>,
    #[doc = "Allow the merchant to process Konbini convenience store payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub konbini_payments:
        Option<stripe_types_05::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesKonbiniPayments>,
    #[doc = "Allow the merchant to process Korean card payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kr_card_payments:
        Option<stripe_types_05::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesKrCardPayments>,
    #[doc = "Allow the merchant to process Link payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub link_payments:
        Option<stripe_types_06::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesLinkPayments>,
    #[doc = "Allow the merchant to process MobilePay payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub mobilepay_payments:
        Option<stripe_types_05::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesMobilepayPayments>,
    #[doc = "Allow the merchant to process Multibanco payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub multibanco_payments:
        Option<stripe_types_05::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesMultibancoPayments>,
    #[doc = "Allow the merchant to process Mexican bank transfer payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub mx_bank_transfer_payments: Option<
        stripe_types_05::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesMxBankTransferPayments,
    >,
    #[doc = "Allow the merchant to process Naver Pay payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub naver_pay_payments:
        Option<stripe_types_06::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesNaverPayPayments>,
    #[doc = "Allow the merchant to process OXXO payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub oxxo_payments:
        Option<stripe_types_05::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesOxxoPayments>,
    #[doc = "Allow the merchant to process Przelewy24 (P24) payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub p24_payments:
        Option<stripe_types_06::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesP24Payments>,
    #[doc = "Allow the merchant to process Pay by Bank payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub pay_by_bank_payments:
        Option<stripe_types_06::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesPayByBankPayments>,
    #[doc = "Allow the merchant to process PAYCO payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub payco_payments:
        Option<stripe_types_05::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesPaycoPayments>,
    #[doc = "Allow the merchant to process PayNow payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub paynow_payments:
        Option<stripe_types_06::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesPaynowPayments>,
    #[doc = "Allow the merchant to process PromptPay payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub promptpay_payments:
        Option<stripe_types_05::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesPromptpayPayments>,
    #[doc = "Allow the merchant to process Revolut Pay payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub revolut_pay_payments:
        Option<stripe_types_06::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesRevolutPayPayments>,
    #[doc = "Allow the merchant to process Samsung Pay payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub samsung_pay_payments:
        Option<stripe_types_05::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesSamsungPayPayments>,
    #[doc = "Allow the merchant to process Satispay payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub satispay_payments:
        Option<stripe_types_05::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesSatispayPayments>,
    #[doc = "Allow the merchant to process SEPA bank transfer payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub sepa_bank_transfer_payments: Option<
        stripe_types_06::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesSepaBankTransferPayments,
    >,
    #[doc = "Allow the merchant to process SEPA Direct Debit payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub sepa_debit_payments:
        Option<stripe_types_06::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesSepaDebitPayments>,
    #[doc = "Allow the merchant to process SeQura payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub sequra_payments:
        Option<stripe_types_05::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesSequraPayments>,
    #[doc = "Allow the merchant to process Sunbit payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub sunbit_payments:
        Option<stripe_types_06::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesSunbitPayments>,
    #[doc = "Allow the merchant to process Swish payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub swish_payments:
        Option<stripe_types_06::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesSwishPayments>,
    #[doc = "Allow the merchant to process TWINT payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub twint_payments:
        Option<stripe_types_05::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesTwintPayments>,
    #[doc = "Allow the merchant to process US bank transfer payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub us_bank_transfer_payments: Option<
        stripe_types_06::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesUsBankTransferPayments,
    >,
    #[doc = "Allow the merchant to process Zip payments."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub zip_payments:
        Option<stripe_types_05::v2_core::UpdateCoreAccountParamsConfigurationMerchantCapabilitiesZipPayments>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsConfigurationMerchantCardPayments {
    #[doc = "Automatically declines certain charge types regardless of whether the card issuer accepted or declined the charge."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub decline_on: Option<stripe_types_05::v2_core::UpdateCoreAccountParamsConfigurationMerchantCardPaymentsDeclineOn>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsConfigurationMerchantScriptStatementDescriptor {
    #[doc = "The Kana variation of statement_descriptor used for charges in Japan."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kana:
        Option<stripe_types_05::v2_core::UpdateCoreAccountParamsConfigurationMerchantScriptStatementDescriptorKana>,
    #[doc = "The Kanji variation of statement_descriptor used for charges in Japan."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kanji:
        Option<stripe_types_06::v2_core::UpdateCoreAccountParamsConfigurationMerchantScriptStatementDescriptorKanji>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsConfigurationRecipientCapabilitiesStripeBalance {
    #[doc = "Enables this Account to receive /v1/transfers into their Stripe Balance (/v1/balance)."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub stripe_transfers: Option<
        stripe_types_05::v2_core::UpdateCoreAccountParamsConfigurationRecipientCapabilitiesStripeBalanceStripeTransfers,
    >,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsIdentityAttestationsTermsOfService {
    #[doc = "Details on the Account's acceptance of the Stripe Services Agreement..."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub account: Option<stripe_types_06::v2_core::UpdateCoreAccountParamsIdentityAttestationsTermsOfServiceAccount>,
    #[doc = "Details on the Account's acceptance of Crypto-specific terms of service."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub crypto_money_manager:
        Option<stripe_types_06::v2_core::UpdateCoreAccountParamsIdentityAttestationsTermsOfServiceCryptoMoneyManager>,
    #[doc = "Details on the Account's acceptance of Treasury-specific terms of service."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub money_manager:
        Option<stripe_types_05::v2_core::UpdateCoreAccountParamsIdentityAttestationsTermsOfServiceMoneyManager>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyMinisterialDecree {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type:
        stripe_types_06::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsCompanyMinisterialDecreeType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsProofOfAddress {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_05::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsProofOfAddressType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsProofOfRegistration {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "Person that is signing the document."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub signer: Option<
        stripe_types_05::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsProofOfRegistrationSigner,
    >,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type:
        stripe_types_05::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsProofOfRegistrationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsProofOfUltimateBeneficialOwnership {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "Person that is signing the document."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub signer: Option<stripe_types_06::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsProofOfUltimateBeneficialOwnershipSigner>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_06::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsDocumentsProofOfUltimateBeneficialOwnershipType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsIdentityBusinessDetailsIdNumbersItem {
    #[doc = "The registrar of the ID number (Only valid for DE ID number types)."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub registrar: Option<String>,
    #[doc = "Open Enum."]
    #[json(rename = "type")]
    pub r#type: stripe_types_05::v2_core::UpdateCoreAccountParamsIdentityBusinessDetailsIdNumbersItemType,
    #[doc = "The value of the ID number."]
    pub value: String,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsIdentityIndividualDocumentsCompanyAuthorization {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_06::v2_core::UpdateCoreAccountParamsIdentityIndividualDocumentsCompanyAuthorizationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsIdentityIndividualDocumentsPassport {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_05::v2_core::UpdateCoreAccountParamsIdentityIndividualDocumentsPassportType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode, Default)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountParamsIdentityIndividualScriptNames {
    #[doc = "Persons name in kana script."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kana: Option<stripe_types_05::v2_core::UpdateCoreAccountParamsIdentityIndividualScriptNamesKana>,
    #[doc = "Persons name in kanji script."]
    #[json(skip_serializing_if = "Option::is_none")]
    pub kanji: Option<stripe_types_05::v2_core::UpdateCoreAccountParamsIdentityIndividualScriptNamesKanji>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountPersonParamsDocumentsPassport {
    #[doc = "One or more document IDs returned by a [file upload](https://docs.stripe.com/api/persons/update#create_file) with a p..."]
    pub files: Vec<String>,
    #[doc = "The format of the document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_06::v2_core::UpdateCoreAccountPersonParamsDocumentsPassportType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountPersonParamsDocumentsSecondaryVerification {
    #[doc = "The [file upload](https://docs.stripe.com/api/persons/update#create_file) tokens referring to each side of the document."]
    pub front_back: stripe_types_06::v2_core::UpdateCoreAccountPersonParamsDocumentsSecondaryVerificationFrontBack,
    #[doc = "The format of the verification document."]
    #[json(rename = "type")]
    pub r#type: stripe_types_05::v2_core::UpdateCoreAccountPersonParamsDocumentsSecondaryVerificationType,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Encode)]
#[json(deny_duplicate_fields)]
pub struct UpdateCoreAccountPersonParamsIdNumbersItem {
    #[doc = "The ID number type of an individual."]
    #[json(rename = "type")]
    pub r#type: stripe_types_05::v2_core::UpdateCoreAccountPersonParamsIdNumbersItemType,
    #[doc = "The value of the ID number."]
    pub value: String,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountConfigurationMerchantKonbiniPayments {
    #[doc = "Support for Konbini payments."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub support: Option<stripe_types_05::v2_core::V2CoreAccountConfigurationMerchantKonbiniPaymentsSupport>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItem {
    #[doc = "The name of the Capability which will be restricted."]
    #[json(default)]
    pub capability:
        stripe_types_03::v2_core::V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemCapability,
    #[doc = "The configuration which specifies the Capability which will be restricted."]
    #[json(default)]
    pub configuration: stripe_types_04::v2_core::V2CoreAccountAppliedConfigurationsItem,
    #[doc = "Details about when in the account lifecycle the requirement must be collected by the avoid the Capability restriction."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub deadline: Option<
        stripe_types_06::v2_core::V2CoreAccountFutureRequirementsEntriesItemImpactRestrictsCapabilitiesItemDeadline,
    >,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreAccountIdentityIndividualDocuments {
    #[doc = "One or more documents that demonstrate proof that this person is authorized to represent the company."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub company_authorization:
        Option<stripe_types_06::v2_core::V2CoreAccountIdentityBusinessDetailsDocumentsBankAccountOwnershipVerification>,
    #[doc = "One or more documents showing the person's passport page with photo and personal data."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub passport:
        Option<stripe_types_06::v2_core::V2CoreAccountIdentityBusinessDetailsDocumentsBankAccountOwnershipVerification>,
    #[doc = "An identifying document showing the person's name, either a passport or local ID card."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub primary_verification:
        Option<stripe_types_05::v2_core::V2CoreAccountIdentityBusinessDetailsDocumentsPrimaryVerification>,
    #[doc = "A document showing address, either a passport, local ID card, or utility bill from a well-known utility company."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub secondary_verification:
        Option<stripe_types_05::v2_core::V2CoreAccountIdentityBusinessDetailsDocumentsPrimaryVerification>,
    #[doc = "One or more documents showing the person's visa required for living in the country where they are residing."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub visa:
        Option<stripe_types_06::v2_core::V2CoreAccountIdentityBusinessDetailsDocumentsBankAccountOwnershipVerification>,
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreEvent {
    #[doc = "Before and after changes for the primary related object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub changes: Option<json::OwnedValue>,
    #[doc = "Authentication context needed to fetch the event or related object."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[doc = "Time at which the object was created."]
    #[json(default)]
    pub created: String,
    #[doc = "Unique identifier for the event."]
    #[json(default)]
    pub id: String,
    #[doc = "Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode."]
    #[json(default)]
    pub livemode: bool,
    #[doc = "String representing the object's type."]
    #[json(default)]
    pub object: stripe_types_03::v2_core::V2CoreEventObject,
    #[doc = "Reason for the event."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<stripe_types_06::v2_core::V2CoreEventReason>,
    #[doc = "For interop events, this is the snapshot event ID."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_event: Option<String>,
    #[doc = "The type of the event."]
    #[json(rename = "type", default)]
    pub r#type: String,
}

#[cfg(feature = "v2_core")]
impl crate::PagedItem for V2CoreEvent {
    fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(feature = "v2_core")]
#[derive(Debug, Clone, json::Decode)]
#[json(deny_duplicate_fields)]
pub struct V2CoreEventDestinationStatusDetails {
    #[doc = "Present when the destination was disabled; identifies the cause, time, and provider-side object involved when available."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<stripe_types_06::v2_core::V2CoreEventDestinationStatusDetailsDisabled>,
}
