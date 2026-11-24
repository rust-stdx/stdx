#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CancelPaymentIntentParams {
    #[doc = "Reason for canceling this PaymentIntent."]
    pub cancellation_reason: Option<stripe_types_02::payment_intents::CancelPaymentIntentParamsCancellationReason>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CancelPaymentIntentParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("cancellation_reason", self.cancellation_reason.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}

#[cfg(feature = "payment_intents")]
pub type CapturePaymentIntentParamsPaymentDetails =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentDetails;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsAmountDetailsShipping =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsAmountDetailsShipping;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsAmountDetailsTax =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsAmountDetailsTax;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsCaptureMethod =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsCaptureMethod;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentDetails =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentDetails;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodDataAcssDebit =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsPaymentMethodDataAcssDebit;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodDataAfterpayClearpay =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodDataAlipay =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodDataAllowRedisplay =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodDataAllowRedisplay;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodDataAlma =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodDataBancontact =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodDataBillingDetailsAddress =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodDataBillingDetailsAddress;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodDataBizum =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodDataCashapp =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodDataCrypto =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodDataCustomerBalance =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodDataFpxBank =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodDataFpxBank;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodDataGrabpay =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodDataIdealBank =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodDataIdealBank;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodDataLink =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodDataMobilepay =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodDataNzBankAccount =
    stripe_types_01::payment_intents::CreatePaymentIntentParamsPaymentMethodDataNzBankAccount;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodDataPayco =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodDataPaynow =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodDataPaypay =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodDataPix =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodDataPromptpay =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodDataRevolutPay =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodDataSatispay =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodDataSepaDebit =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsPaymentMethodDataSepaDebit;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodDataSunbit =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodDataSwish =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodDataUpiMandateOptionsAmountType =
    stripe_types_01::payment_intents::CreatePaymentIntentParamsPaymentMethodDataUpiMandateOptionsAmountType;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule = stripe_types_03::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType = stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsAmazonPayCaptureMethod =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsAmazonPaySetupFutureUsage =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsBacsDebitMandateOptions =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsBacsDebitMandateOptions;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsBacsDebitSetupFutureUsage =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsBancontactPreferredLanguage =
    stripe_types_01::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsBancontactPreferredLanguage;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsBancontactSetupFutureUsage =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsBizum =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsBlikMandateOptions =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsBlikMandateOptions;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsBlikSetupFutureUsage =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlanInterval =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlanInterval;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsCardMandateOptionsInterval =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsInterval;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsCardNetwork =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardNetwork;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsCardPresentCaptureMethod =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardPresentCaptureMethod;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsCardRequestMulticapture =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardRequestExtendedAuthorization;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsCardSetupFutureUsage =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo = stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsCardThreeDSecureVersion =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureVersion;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsCashappCaptureMethod =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsCryptoSetupFutureUsage =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsCustomerBalanceSetupFutureUsage =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsFpxSetupFutureUsage =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsGrabpaySetupFutureUsage =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsInteracPresent =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsKakaoPayCaptureMethod =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsKlarnaCaptureMethod =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsKrCardSetupFutureUsage =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsLinkCaptureMethod =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsLinkSetupFutureUsage =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsMbWaySetupFutureUsage =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsMobilepaySetupFutureUsage =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsMultibancoSetupFutureUsage =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsNaverPaySetupFutureUsage =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsNzBankAccountSetupFutureUsage =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsP24SetupFutureUsage =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsPaypalCaptureMethod =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsPaypalSetupFutureUsage =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsPaytoSetupFutureUsage =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsPixAmountIncludesIof =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsAutomaticPaymentMethodsAllowRedirects;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsPixMandateOptionsAmountType =
    stripe_types_01::payment_intents::CreatePaymentIntentParamsPaymentMethodDataUpiMandateOptionsAmountType;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsSamsungPayCaptureMethod =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsSamsungPaySetupFutureUsage =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsSatispayCaptureMethod =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsSatispaySetupFutureUsage =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsSequraCaptureMethod =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsSofortPreferredLanguage =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsSofortPreferredLanguage;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsSofortSetupFutureUsage =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsTwintSetupFutureUsage =
    stripe_types_01::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPixSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsUpiSetupFutureUsage =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodDataUsBankAccountAccountType;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem = stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod = stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsUsBankAccountTransactionPurpose =
    stripe_types_01::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountTransactionPurpose;

#[cfg(feature = "payment_intents")]
pub type ConfirmPaymentIntentParamsPaymentMethodOptionsWechatPaySetupFutureUsage =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodDataAffirm =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodDataAlma =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodDataBancontact =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodDataBillie =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodDataBillingDetails {
    #[doc = "Billing address."]
    pub address:
        Option<stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodDataBillingDetailsAddress>,
    #[doc = "Email address."]
    pub email: Option<json::OwnedValue>,
    #[doc = "Full name."]
    pub name: Option<json::OwnedValue>,
    #[doc = "Billing phone number (including extension)."]
    pub phone: Option<json::OwnedValue>,
    #[doc = "Taxpayer identification number."]
    pub tax_id: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodDataBillingDetails {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("address", self.address.as_ref());
        form.field_opt("tax_id", self.tax_id.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodDataBizum =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodDataCustomerBalance =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodDataGiropay =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodDataIdeal {
    #[doc = "The customer's bank."]
    pub bank: Option<stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodDataIdealBank>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodDataIdeal {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("bank", self.bank.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodDataInteracPresent =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodDataKakaoPay =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodDataKlarna {
    #[doc = "Customer's date of birth"]
    pub dob: Option<stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodDataKlarnaDob>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodDataKlarna {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("dob", self.dob.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodDataKonbini =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodDataKrCard =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodDataLink =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodDataMobilepay =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodDataNaverPay {
    #[doc = "Whether to use Naver Pay points or a card to fund this transaction."]
    pub funding: Option<stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodDataNaverPayFunding>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodDataNaverPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("funding", self.funding.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodDataP24 {
    #[doc = "The customer's bank."]
    pub bank: Option<stripe_types_01::payment_intents::CreatePaymentIntentParamsPaymentMethodDataP24Bank>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodDataP24 {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("bank", self.bank.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodDataPayco =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodDataPaypal =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodDataPromptpay =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodDataRevolutPay =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodDataScalapay =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodDataSequra =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodDataTwint =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodDataUpiMandateOptions {
    #[doc = "Amount to be charged for future payments."]
    pub amount: Option<i64>,
    #[doc = "One of `fixed` or `maximum`."]
    pub amount_type:
        Option<stripe_types_01::payment_intents::CreatePaymentIntentParamsPaymentMethodDataUpiMandateOptionsAmountType>,
    #[doc = "A description of the mandate or subscription that is meant to be displayed to the customer."]
    pub description: Option<String>,
    #[doc = "End date of the mandate or subscription."]
    pub end_date: Option<i64>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodDataUpiMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("amount_type", self.amount_type.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field_opt("end_date", self.end_date.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodDataUsBankAccountAccountHolderType =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodDataFpxAccountHolderType;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodDataZip =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsAffirm {
    #[doc = "Controls when the funds are captured from the customer's account."]
    pub capture_method:
        Option<stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod>,
    #[doc = "Preferred language of the Affirm authorization page that the customer is redirected to."]
    pub preferred_locale: Option<String>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsAffirm {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("capture_method", self.capture_method.as_ref());
        form.field_opt("preferred_locale", self.preferred_locale.as_ref());
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsAlipay {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    pub setup_future_usage:
        Option<stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsAlipay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("setup_future_usage", self.setup_future_usage.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsAuBecsDebitSetupFutureUsage =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsBancontactSetupFutureUsage =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegisteredAddress =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodDataBillingDetailsAddress;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsBlikSetupFutureUsage =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsBoletoSetupFutureUsage =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsCardCaptureMethod =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsCashappCaptureMethod =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsEpsSetupFutureUsage =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsGiropaySetupFutureUsage =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsKakaoPaySetupFutureUsage =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsKlarnaCaptureMethod =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsKonbiniSetupFutureUsage =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsKrCardSetupFutureUsage =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsMbWaySetupFutureUsage =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsMobilepayCaptureMethod =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsMultibancoSetupFutureUsage =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsNaverPayCaptureMethod =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsNzBankAccountSetupFutureUsage =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsOxxoSetupFutureUsage =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsPaypalCaptureMethod =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsPaypalSetupFutureUsage =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsPaypay =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptions {
    #[doc = "Amount that will be collected."]
    pub amount: Option<json::OwnedValue>,
    #[doc = "The type of amount that will be collected."]
    pub amount_type: Option<stripe_types_03::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType>,
    #[doc = "Date, in YYYY-MM-DD format, after which payments will not be collected."]
    pub end_date: Option<json::OwnedValue>,
    #[doc = "The periodicity at which payments will be collected."]
    pub payment_schedule: Option<stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule>,
    #[doc = "The number of payments that will be made during a payment period."]
    pub payments_per_period: Option<json::OwnedValue>,
    #[doc = "The purpose for which payments are made."]
    pub purpose: Option<stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount_type", self.amount_type.as_ref());
        form.field_opt("payment_schedule", self.payment_schedule.as_ref());
        form.field_opt("purpose", self.purpose.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsPaytoSetupFutureUsage =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsPixAmountIncludesIof =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsAutomaticPaymentMethodsAllowRedirects;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsPixMandateOptionsAmountIncludesIof =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsAutomaticPaymentMethodsAllowRedirects;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsPromptpaySetupFutureUsage =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsSamsungPayCaptureMethod =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsSamsungPaySetupFutureUsage =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsScalapayCaptureMethod =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsSepaDebitMandateOptions =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsBacsDebitMandateOptions;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsSepaDebitSetupFutureUsage =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsSofortSetupFutureUsage =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsSunbitCaptureMethod =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsUpiMandateOptionsAmountType =
    stripe_types_01::payment_intents::CreatePaymentIntentParamsPaymentMethodDataUpiMandateOptionsAmountType;

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsUpiSetupFutureUsage =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage;

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountMandateOptions {
    #[doc = "The method used to collect offline mandate customer acceptance."]
    pub collection_method: Option<stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("collection_method", self.collection_method.as_ref());
    }
}

#[cfg(feature = "payment_intents")]
pub type CreatePaymentIntentParamsPaymentMethodOptionsZipSetupFutureUsage =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage;

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
pub struct FundingInstructionsBankTransferAbaRecord {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_holder_address: Option<stripe_types_01::common::Address>,
    #[doc = "The account holder name"]
    #[json(default)]
    pub account_holder_name: String,
    #[doc = "The ABA account number"]
    #[json(default)]
    pub account_number: String,
    #[doc = "The account type"]
    #[json(default)]
    pub account_type: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_address: Option<stripe_types_01::common::Address>,
    #[doc = "The bank name"]
    #[json(default)]
    pub bank_name: String,
    #[doc = "The ABA routing number"]
    #[json(default)]
    pub routing_number: String,
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
pub struct FundingInstructionsBankTransferIbanRecord {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_holder_address: Option<stripe_types_01::common::Address>,
    #[doc = "The name of the person or business that owns the bank account"]
    #[json(default)]
    pub account_holder_name: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub bank_address: Option<stripe_types_01::common::Address>,
    #[doc = "The BIC/SWIFT code of the account."]
    #[json(default)]
    pub bic: String,
    #[doc = "Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))."]
    #[json(default)]
    pub country: String,
    #[doc = "The IBAN of the account."]
    #[json(default)]
    pub iban: String,
}

#[cfg(feature = "payment_intents")]
pub type IncrementAuthorizationParamsAmountDetailsShipping =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsAmountDetailsShipping;

#[cfg(feature = "payment_intents")]
pub type IncrementAuthorizationParamsAmountDetailsTax =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsAmountDetailsTax;

#[cfg(feature = "payment_intents")]
pub type IncrementAuthorizationParamsHooksInputsTax =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsHooksInputsTax;

#[cfg(feature = "payment_intents")]
pub type IncrementAuthorizationParamsTransferData =
    stripe_types_01::payment_intents::CapturePaymentIntentParamsTransferData;

#[cfg(feature = "payment_intents")]
#[derive(Debug, Clone, Default)]
pub struct ListPaymentIntentsParams {
    pub created: Option<stripe_types_03::payment_intents::ListPaymentIntentsParamsCreated>,
    pub customer: Option<String>,
    pub customer_account: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}

#[cfg(feature = "payment_intents")]
impl crate::params::FormEncode for ListPaymentIntentsParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("created", self.created.as_ref());
        form.field_opt("customer", self.customer.as_ref());
        form.field_opt("customer_account", self.customer_account.as_ref());
        form.field_opt("ending_before", self.ending_before.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("limit", self.limit.as_ref());
        form.field_opt("starting_after", self.starting_after.as_ref());
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
pub struct PaymentFlowsAutomaticPaymentMethodsPaymentIntent {
    #[doc = "Controls whether this PaymentIntent will accept redirect-based payment methods."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub allow_redirects:
        Option<stripe_types_00::payment_intents::PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects>,
    #[doc = "Automatically calculates compatible payment methods"]
    #[json(default)]
    pub enabled: bool,
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
pub struct PaymentFlowsAutomaticPaymentMethodsSetupIntent {
    #[doc = "Controls whether this SetupIntent will accept redirect-based payment methods."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub allow_redirects:
        Option<stripe_types_00::payment_intents::PaymentFlowsAutomaticPaymentMethodsPaymentIntentAllowRedirects>,
    #[doc = "Automatically calculates compatible payment methods"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
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
pub struct PaymentFlowsPaymentIntentAsyncWorkflowsResourceInputs {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub tax: Option<stripe_types_02::payment_intents::PaymentFlowsPaymentIntentAsyncWorkflowsResourceInputsResourceTax>,
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
pub struct PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetails {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub registered_address: Option<stripe_types_01::payment_intents::PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsResourceCompanyDetailsRegisteredAddress>,
    #[doc = "Company or entity name."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub registered_name: Option<String>,
    #[doc = "The official registration number for the given registration type."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,
    #[doc = "Type of registration the company or entity holds in their registered country."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub registration_type: Option<stripe_types_04::payment_intents::PaymentFlowsPrivatePaymentMethodsBilliePaymentMethodOptionsResourceCompanyDetailsRegistrationType>,
    #[doc = "VAT id number"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub vat: Option<String>,
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
pub struct PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorization {
    #[doc = "Indicates whether or not the capture window is extended beyond the standard authorization."]
    #[json(default)]
    pub status: stripe_types_01::payment_intents::PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesExtendedAuthorizationExtendedAuthorizationStatus,
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
pub struct PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceMulticapture {
    #[doc = "Indicates whether or not multiple captures are supported."]
    #[json(default)]
    pub status: stripe_types_04::payment_intents::PaymentFlowsPrivatePaymentMethodsCardDetailsApiResourceEnterpriseFeaturesIncrementalAuthorizationIncrementalAuthorizationStatus,
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
pub struct PaymentFlowsPrivatePaymentMethodsCardPresentCommonWallet {
    #[doc = "The type of mobile wallet, one of `apple_pay`, `google_pay`, `samsung_pay`, or `unknown`."]
    #[json(rename = "type", default)]
    pub r#type: stripe_types_02::payment_intents::PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType,
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
pub struct PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFilters {
    #[doc = "The account subcategories to use to filter for possible accounts to link."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub account_subcategories: Option<Vec<stripe_types_03::payment_intents::PaymentFlowsPrivatePaymentMethodsFinancialConnectionsCommonLinkedAccountOptionsFiltersAccountSubcategoriesItem>>,
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
pub struct PaymentFlowsPrivatePaymentMethodsKakaoPayPaymentMethodOptions {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<
        stripe_types_02::payment_intents::PaymentFlowsPrivatePaymentMethodsKakaoPayPaymentMethodOptionsCaptureMethod,
    >,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<
        stripe_types_04::payment_intents::PaymentFlowsPrivatePaymentMethodsKakaoPayPaymentMethodOptionsSetupFutureUsage,
    >,
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
pub struct PaymentFlowsPrivatePaymentMethodsPaycoPaymentMethodOptions {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<
        stripe_types_02::payment_intents::PaymentFlowsPrivatePaymentMethodsKakaoPayPaymentMethodOptionsCaptureMethod,
    >,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<
        stripe_types_01::payment_intents::PaymentFlowsPrivatePaymentMethodsPaycoPaymentMethodOptionsSetupFutureUsage,
    >,
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
pub struct PaymentIntentCardProcessing {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub customer_notification: Option<stripe_types_04::payment_intents::PaymentIntentProcessingCustomerNotification>,
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
pub struct PaymentIntentNextActionCashappHandleRedirectOrDisplayQrCode {
    #[doc = "The URL to the hosted Cash App Pay instructions page, which allows customers to view the QR code, and supports QR cod..."]
    #[json(default)]
    pub hosted_instructions_url: String,
    #[doc = "The url for mobile redirect based auth"]
    #[json(default)]
    pub mobile_auth_url: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub qr_code: Option<stripe_types_02::payment_intents::PaymentIntentNextActionCashappQrCode>,
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
pub struct PaymentIntentNextActionKonbiniStores {
    #[doc = "FamilyMart instruction details."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub familymart: Option<stripe_types_00::payment_intents::PaymentIntentNextActionKonbiniFamilymart>,
    #[doc = "Lawson instruction details."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub lawson: Option<stripe_types_01::payment_intents::PaymentIntentNextActionKonbiniLawson>,
    #[doc = "Ministop instruction details."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub ministop: Option<stripe_types_01::payment_intents::PaymentIntentNextActionKonbiniMinistop>,
    #[doc = "Seicomart instruction details."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub seicomart: Option<stripe_types_02::payment_intents::PaymentIntentNextActionKonbiniSeicomart>,
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
pub struct PaymentIntentNextActionSwishHandleRedirectOrDisplayQrCode {
    #[doc = "The URL to the hosted Swish instructions page, which allows customers to view the QR code."]
    #[json(default)]
    pub hosted_instructions_url: String,
    #[doc = "The url for mobile redirect based auth (for internal use only and not typically available in standard API requests)."]
    #[json(default)]
    pub mobile_auth_url: String,
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub qr_code: Option<stripe_types_02::payment_intents::PaymentIntentNextActionSwishQrCode>,
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
pub struct PaymentIntentNextActionVerifyWithMicrodeposits {
    #[doc = "The timestamp when the microdeposits are expected to land."]
    #[json(default)]
    pub arrival_date: i64,
    #[doc = "The URL for the hosted verification page, which allows customers to verify their bank account."]
    #[json(default)]
    pub hosted_verification_url: String,
    #[doc = "The type of the microdeposit sent to the customer."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub microdeposit_type:
        Option<stripe_types_01::payment_intents::PaymentIntentNextActionVerifyWithMicrodepositsMicrodepositType>,
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
pub struct PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebit {
    #[doc = "A URL for custom mandate text"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<String>,
    #[doc = "Description of the interval."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<String>,
    #[doc = "Payment schedule for the mandate."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_schedule: Option<
        stripe_types_00::payment_intents::PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule,
    >,
    #[doc = "Transaction type of the mandate."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<
        stripe_types_02::payment_intents::PaymentIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType,
    >,
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
pub struct PaymentIntentPaymentMethodOptionsMandateOptionsPayto {
    #[doc = "Amount that will be collected."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[doc = "The type of amount that will be collected."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub amount_type:
        Option<stripe_types_04::payment_intents::PaymentIntentPaymentMethodOptionsMandateOptionsPaytoAmountType>,
    #[doc = "Date, in YYYY-MM-DD format, after which payments will not be collected."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[doc = "The periodicity at which payments will be collected."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_schedule:
        Option<stripe_types_04::payment_intents::PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPaymentSchedule>,
    #[doc = "The number of payments that will be made during a payment period."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payments_per_period: Option<i64>,
    #[doc = "The purpose for which payments are made."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<stripe_types_01::payment_intents::PaymentIntentPaymentMethodOptionsMandateOptionsPaytoPurpose>,
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
pub struct PaymentIntentPaymentMethodOptionsMobilepay {
    #[doc = "Controls when the funds will be captured from the customer's account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<
        stripe_types_02::payment_intents::PaymentFlowsPrivatePaymentMethodsKakaoPayPaymentMethodOptionsCaptureMethod,
    >,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<
        stripe_types_01::payment_intents::PaymentFlowsPrivatePaymentMethodsPaycoPaymentMethodOptionsSetupFutureUsage,
    >,
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
pub struct PaymentIntentPaymentMethodOptionsNzBankAccount {
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<stripe_types_03::payment_intents::PaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage>,
    #[doc = "Controls when Stripe will attempt to debit the funds from the customer's account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub target_date: Option<String>,
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
pub struct PaymentIntentPaymentMethodOptionsSepaDebit {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<stripe_types_04::payment_intents::PaymentIntentPaymentMethodOptionsMandateOptionsSepaDebit>,
    #[doc = "Indicates that you intend to make future payments with this PaymentIntent's payment method."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<stripe_types_03::payment_intents::PaymentIntentPaymentMethodOptionsAcssDebitSetupFutureUsage>,
    #[doc = "Controls when Stripe will attempt to debit the funds from the customer's account."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub target_date: Option<String>,
}

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsCaptureMethod =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsCaptureMethod;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentDetails =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentDetails;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodDataAffirm =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodDataAfterpayClearpay =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodDataAlma =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodDataAmazonPay =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodDataBacsDebit =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodDataBacsDebit;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodDataBancontact =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodDataBillie =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodDataBillingDetailsAddress =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodDataBillingDetailsAddress;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodDataBizum =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodDataEpsBank =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodDataEpsBank;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodDataFpxAccountHolderType =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodDataFpxAccountHolderType;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodDataGiropay =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodDataKakaoPay =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodDataKlarnaDob =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodDataKlarnaDob;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodDataKonbini =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodDataKrCard =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodDataLink =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodDataNaverPayFunding =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodDataNaverPayFunding;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodDataP24Bank =
    stripe_types_01::payment_intents::CreatePaymentIntentParamsPaymentMethodDataP24Bank;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodDataPayByBank =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodDataPayco =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodDataPaypal =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodDataPayto =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsPaymentMethodDataPayto;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodDataPix =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodDataRevolutPay =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodDataScalapay =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodDataSequra =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodDataSofortCountry =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsPaymentMethodDataSofortCountry;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodDataSwish =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodDataType =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodDataType;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodDataWechatPay =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType = stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsAfterpayClearpayCaptureMethod =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsAmazonPaySetupFutureUsage =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsAuBecsDebitSetupFutureUsage =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsBacsDebitSetupFutureUsage =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsBancontactPreferredLanguage =
    stripe_types_01::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsBancontactPreferredLanguage;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsBillieCaptureMethod =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegisteredAddress =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodDataBillingDetailsAddress;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsBillieCompanyDetailsRegistrationType;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsBlikMandateOptions =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsBlikMandateOptions;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsBlikSetupFutureUsage =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlanType =
    stripe_types_01::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardInstallmentsPlanType;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsInterval =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardMandateOptionsInterval;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsCardPresentRoutingRequestedPriority =
    stripe_types_01::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardPresentRoutingRequestedPriority;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsCardRequestExtendedAuthorization =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardRequestExtendedAuthorization;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsCardRequestIncrementalAuthorization =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardRequestExtendedAuthorization;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsCardRequestThreeDSecure =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardRequestThreeDSecure;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureExemptionIndicator =
    stripe_types_01::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCardThreeDSecureExemptionIndicator;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsCashappSetupFutureUsage =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsCryptoSetupFutureUsage =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer = stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem = stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypesItem;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsCustomerBalanceSetupFutureUsage =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsEpsSetupFutureUsage =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsGiropaySetupFutureUsage =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsIdealSetupFutureUsage =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsInteracPresent =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsKakaoPayCaptureMethod =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsKakaoPaySetupFutureUsage =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsKonbiniSetupFutureUsage =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsKrCardCaptureMethod =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsKrCardSetupFutureUsage =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsLinkCaptureMethod =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsLinkSetupFutureUsage =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsMobilepayCaptureMethod =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsMobilepaySetupFutureUsage =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsNzBankAccountSetupFutureUsage =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsPayByBank =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsPaycoSetupFutureUsage =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPaypalPreferredLocale;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsPaypalSetupFutureUsage =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsPaypay =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsPixAmountIncludesIof =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsAutomaticPaymentMethodsAllowRedirects;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsPixMandateOptionsAmountType =
    stripe_types_01::payment_intents::CreatePaymentIntentParamsPaymentMethodDataUpiMandateOptionsAmountType;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPixMandateOptionsPaymentSchedule;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsPixSetupFutureUsage =
    stripe_types_01::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPixSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsRevolutPaySetupFutureUsage =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsSamsungPayCaptureMethod =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsSatispayCaptureMethod =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsSatispaySetupFutureUsage =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsSepaDebitMandateOptions =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsBacsDebitMandateOptions;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsSofortSetupFutureUsage =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAlipaySetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsSunbitCaptureMethod =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmCaptureMethod;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsSwishSetupFutureUsage =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsTwintSetupFutureUsage =
    stripe_types_01::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsPixSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsUpiSetupFutureUsage =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAcssDebitSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodDataUsBankAccountAccountType;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem = stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod = stripe_types_00::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsUsBankAccountNetworksRequestedItem =
    stripe_types_03::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountNetworksRequestedItem;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsUsBankAccountTransactionPurpose =
    stripe_types_01::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsUsBankAccountTransactionPurpose;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsWechatPaySetupFutureUsage =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsPaymentMethodOptionsZipSetupFutureUsage =
    stripe_types_02::payment_intents::CreatePaymentIntentParamsPaymentMethodOptionsAffirmSetupFutureUsage;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsShippingAddress =
    stripe_types_04::payment_intents::CreatePaymentIntentParamsPaymentMethodDataBillingDetailsAddress;

#[cfg(feature = "payment_intents")]
pub type UpdatePaymentIntentParamsTransferDataPaymentData =
    stripe_types_00::payment_intents::CreatePaymentIntentParamsTransferDataPaymentData;
