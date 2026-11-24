#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CancelSetupIntentParams {
    #[doc = "Reason for canceling this SetupIntent."]
    pub cancellation_reason: Option<stripe_types_00::setup_intents::CancelSetupIntentParamsCancellationReason>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CancelSetupIntentParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("cancellation_reason", self.cancellation_reason.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
    }
}

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodDataAffirm =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodDataAlma =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodDataAmazonPay =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodDataAuBecsDebit =
    stripe_types_00::setup_intents::CreateSetupIntentParamsPaymentMethodDataAuBecsDebit;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodDataBacsDebit =
    stripe_types_02::setup_intents::CreateSetupIntentParamsPaymentMethodDataBacsDebit;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodDataBancontact =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodDataBillie =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodDataBizum =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodDataBoleto =
    stripe_types_04::setup_intents::CreateSetupIntentParamsPaymentMethodDataBoleto;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodDataCustomerBalance =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodDataFpxAccountHolderType =
    stripe_types_02::setup_intents::CreateSetupIntentParamsPaymentMethodDataFpxAccountHolderType;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodDataFpxBank =
    stripe_types_00::setup_intents::CreateSetupIntentParamsPaymentMethodDataFpxBank;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodDataGiropay =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodDataKakaoPay =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodDataKlarnaDob =
    stripe_types_01::setup_intents::CreateSetupIntentParamsPaymentMethodDataKlarnaDob;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodDataKonbini =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodDataKrCard =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodDataLink =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodDataNaverPayFunding =
    stripe_types_02::setup_intents::CreateSetupIntentParamsPaymentMethodDataNaverPayFunding;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodDataPayByBank =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodDataPayco =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodDataPaypal =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodDataPayto =
    stripe_types_00::setup_intents::CreateSetupIntentParamsPaymentMethodDataPayto;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodDataRevolutPay =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodDataScalapay =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodDataSequra =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodDataSofortCountry =
    stripe_types_02::setup_intents::CreateSetupIntentParamsPaymentMethodDataSofortCountry;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodDataSwish =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodDataType =
    stripe_types_00::setup_intents::CreateSetupIntentParamsPaymentMethodDataType;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodDataUsBankAccountAccountHolderType =
    stripe_types_02::setup_intents::CreateSetupIntentParamsPaymentMethodDataFpxAccountHolderType;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodDataUsBankAccountAccountType =
    stripe_types_00::setup_intents::CreateSetupIntentParamsPaymentMethodDataUsBankAccountAccountType;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodDataWechatPay =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodDataZip =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule =
    stripe_types_02::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodOptionsAmazonPay =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodOptionsBacsDebitMandateOptions =
    stripe_types_00::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsBacsDebitMandateOptions;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodOptionsBlikMandateOptions =
    stripe_types_02::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsBlikMandateOptions;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodOptionsCardMandateOptionsInterval =
    stripe_types_01::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsInterval;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodOptionsCardPresent =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus =
    stripe_types_00::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodOptionsCardThreeDSecureVersion =
    stripe_types_03::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureVersion;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodOptionsLink =
    stripe_types_04::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsLink;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodOptionsPaypal =
    stripe_types_01::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsPaypal;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType =
    stripe_types_01::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule =
    stripe_types_00::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule;

#[cfg(feature = "setup_intents")]
pub type ConfirmSetupIntentParamsPaymentMethodOptionsPixMandateOptionsAmountType =
    stripe_types_01::setup_intents::CreateSetupIntentParamsPaymentMethodDataUpiMandateOptionsAmountType;

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsAutomaticPaymentMethods {
    #[doc = "Controls whether this SetupIntent will accept redirect-based payment methods."]
    pub allow_redirects:
        Option<stripe_types_00::setup_intents::CreateSetupIntentParamsAutomaticPaymentMethodsAllowRedirects>,
    #[doc = "Whether this feature is enabled."]
    pub enabled: bool,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsAutomaticPaymentMethods {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("allow_redirects", self.allow_redirects.as_ref());
        form.field("enabled", &self.enabled);
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone)]
pub struct CreateSetupIntentParamsMandateDataCustomerAcceptance {
    #[doc = "The time at which the customer accepted the Mandate."]
    pub accepted_at: Option<i64>,
    #[doc = "If this is a Mandate accepted offline, this hash contains details about the offline acceptance."]
    pub offline: Option<stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline>,
    #[doc = "If this is a Mandate accepted online, this hash contains details about the online acceptance."]
    pub online: Option<stripe_types_01::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOnline>,
    #[doc = "The type of customer acceptance information included with the Mandate."]
    pub r#type: stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceType,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsMandateDataCustomerAcceptance {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("accepted_at", self.accepted_at.as_ref());
        form.field_opt("offline", self.offline.as_ref());
        form.field_opt("online", self.online.as_ref());
        form.field("type", &self.r#type);
    }
}

#[cfg(feature = "setup_intents")]
pub type CreateSetupIntentParamsPaymentMethodDataAffirm =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type CreateSetupIntentParamsPaymentMethodDataAmazonPay =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type CreateSetupIntentParamsPaymentMethodDataBancontact =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type CreateSetupIntentParamsPaymentMethodDataBillie =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type CreateSetupIntentParamsPaymentMethodDataBizum =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type CreateSetupIntentParamsPaymentMethodDataBlik =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type CreateSetupIntentParamsPaymentMethodDataCashapp =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type CreateSetupIntentParamsPaymentMethodDataGrabpay =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type CreateSetupIntentParamsPaymentMethodDataInteracPresent =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type CreateSetupIntentParamsPaymentMethodDataKrCard =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodDataNaverPay {
    #[doc = "Whether to use Naver Pay points or a card to fund this transaction."]
    pub funding: Option<stripe_types_02::setup_intents::CreateSetupIntentParamsPaymentMethodDataNaverPayFunding>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodDataNaverPay {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("funding", self.funding.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
pub type CreateSetupIntentParamsPaymentMethodDataOxxo =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodDataP24 {
    #[doc = "The customer's bank."]
    pub bank: Option<stripe_types_02::setup_intents::CreateSetupIntentParamsPaymentMethodDataP24Bank>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodDataP24 {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("bank", self.bank.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
pub type CreateSetupIntentParamsPaymentMethodDataPayByBank =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type CreateSetupIntentParamsPaymentMethodDataPayco =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type CreateSetupIntentParamsPaymentMethodDataPaypal =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type CreateSetupIntentParamsPaymentMethodDataRevolutPay =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type CreateSetupIntentParamsPaymentMethodDataSatispay =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type CreateSetupIntentParamsPaymentMethodDataSequra =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type CreateSetupIntentParamsPaymentMethodDataTwint =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodDataUpiMandateOptions {
    #[doc = "Amount to be charged for future payments."]
    pub amount: Option<i64>,
    #[doc = "One of `fixed` or `maximum`."]
    pub amount_type:
        Option<stripe_types_01::setup_intents::CreateSetupIntentParamsPaymentMethodDataUpiMandateOptionsAmountType>,
    #[doc = "A description of the mandate or subscription that is meant to be displayed to the customer."]
    pub description: Option<String>,
    #[doc = "End date of the mandate or subscription."]
    pub end_date: Option<i64>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodDataUpiMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount", self.amount.as_ref());
        form.field_opt("amount_type", self.amount_type.as_ref());
        form.field_opt("description", self.description.as_ref());
        form.field_opt("end_date", self.end_date.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
pub type CreateSetupIntentParamsPaymentMethodDataWechatPay =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type CreateSetupIntentParamsPaymentMethodDataZip =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodOptionsBacsDebit {
    #[doc = "Additional fields for Mandate creation"]
    pub mandate_options:
        Option<stripe_types_00::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsBacsDebitMandateOptions>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsBacsDebit {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("mandate_options", self.mandate_options.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
pub type CreateSetupIntentParamsPaymentMethodOptionsBizum =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsAmountType =
    stripe_types_01::setup_intents::CreateSetupIntentParamsPaymentMethodDataUpiMandateOptionsAmountType;

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodOptionsKlarnaOnDemand {
    #[doc = "Your average amount value."]
    pub average_amount: Option<i64>,
    #[doc = "The maximum value you may charge a customer per purchase."]
    pub maximum_amount: Option<i64>,
    #[doc = "The lowest or minimum value you may charge a customer per purchase."]
    pub minimum_amount: Option<i64>,
    #[doc = "Interval at which the customer is making purchases"]
    pub purchase_interval: Option<
        stripe_types_04::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsKlarnaOnDemandPurchaseInterval,
    >,
    #[doc = "The number of `purchase_interval` between charges"]
    pub purchase_interval_count: Option<i64>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsKlarnaOnDemand {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("average_amount", self.average_amount.as_ref());
        form.field_opt("maximum_amount", self.maximum_amount.as_ref());
        form.field_opt("minimum_amount", self.minimum_amount.as_ref());
        form.field_opt("purchase_interval", self.purchase_interval.as_ref());
        form.field_opt("purchase_interval_count", self.purchase_interval_count.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptions {
    #[doc = "Amount that will be collected."]
    pub amount: Option<json::OwnedValue>,
    #[doc = "The type of amount that will be collected."]
    pub amount_type: Option<
        stripe_types_01::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsAmountType,
    >,
    #[doc = "Date, in YYYY-MM-DD format, after which payments will not be collected."]
    pub end_date: Option<json::OwnedValue>,
    #[doc = "The periodicity at which payments will be collected."]
    pub payment_schedule: Option<
        stripe_types_00::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule,
    >,
    #[doc = "The number of payments that will be made during a payment period."]
    pub payments_per_period: Option<json::OwnedValue>,
    #[doc = "The purpose for which payments are made."]
    pub purpose:
        Option<stripe_types_04::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPurpose>,
    #[doc = "Date, in YYYY-MM-DD format, from which payments will be collected."]
    pub start_date: Option<json::OwnedValue>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("amount_type", self.amount_type.as_ref());
        form.field_opt("payment_schedule", self.payment_schedule.as_ref());
        form.field_opt("purpose", self.purpose.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
pub type CreateSetupIntentParamsPaymentMethodOptionsPixMandateOptionsAmountType =
    stripe_types_01::setup_intents::CreateSetupIntentParamsPaymentMethodDataUpiMandateOptionsAmountType;

#[cfg(feature = "setup_intents")]
#[derive(Debug, Clone, Default)]
pub struct CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountMandateOptions {
    #[doc = "The method used to collect offline mandate customer acceptance."]
    pub collection_method: Option<stripe_types_04::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod>,
}

#[cfg(feature = "setup_intents")]
impl crate::params::FormEncode for CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountMandateOptions {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("collection_method", self.collection_method.as_ref());
    }
}

#[cfg(feature = "setup_intents")]
pub type CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountVerificationMethod =
    stripe_types_04::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod;

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
pub struct SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit {
    #[doc = "A URL for custom mandate text"]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<String>,
    #[doc = "List of Stripe products where this mandate can be selected automatically."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub default_for: Option<
        Vec<stripe_types_04::setup_intents::SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitDefaultForItem>,
    >,
    #[doc = "Description of the interval."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<String>,
    #[doc = "Payment schedule for the mandate."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub payment_schedule:
        Option<stripe_types_03::setup_intents::SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitPaymentSchedule>,
    #[doc = "Transaction type of the mandate."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub transaction_type:
        Option<stripe_types_00::setup_intents::SetupIntentPaymentMethodOptionsMandateOptionsAcssDebitTransactionType>,
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
pub struct SetupIntentPaymentMethodOptionsMandateOptionsBlik {
    #[doc = "Date at which the mandate expires."]
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[doc = "Type of the mandate."]
    #[json(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<stripe_types_01::setup_intents::SetupIntentPaymentMethodOptionsMandateOptionsBlikType>,
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
pub struct SetupIntentPaymentMethodOptionsSepaDebit {
    #[json(default, skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<stripe_types_02::setup_intents::SetupIntentPaymentMethodOptionsMandateOptionsSepaDebit>,
}

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsFlowDirectionsItem =
    stripe_types_02::setup_intents::CreateSetupIntentParamsFlowDirectionsItem;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodDataAcssDebit =
    stripe_types_01::setup_intents::CreateSetupIntentParamsPaymentMethodDataAcssDebit;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodDataAffirm =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodDataAfterpayClearpay =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodDataAmazonPay =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodDataBancontact =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodDataBillie =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodDataBillingDetailsAddress =
    stripe_types_04::setup_intents::CreateSetupIntentParamsPaymentMethodDataBillingDetailsAddress;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodDataBlik =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodDataCashapp =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodDataCustomerBalance =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodDataEpsBank =
    stripe_types_01::setup_intents::CreateSetupIntentParamsPaymentMethodDataEpsBank;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodDataFpxAccountHolderType =
    stripe_types_02::setup_intents::CreateSetupIntentParamsPaymentMethodDataFpxAccountHolderType;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodDataGrabpay =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodDataIdealBank =
    stripe_types_01::setup_intents::CreateSetupIntentParamsPaymentMethodDataIdealBank;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodDataInteracPresent =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodDataKrCard =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodDataMbWay =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodDataNaverPayFunding =
    stripe_types_02::setup_intents::CreateSetupIntentParamsPaymentMethodDataNaverPayFunding;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodDataNzBankAccount =
    stripe_types_03::setup_intents::CreateSetupIntentParamsPaymentMethodDataNzBankAccount;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodDataOxxo =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodDataP24Bank =
    stripe_types_02::setup_intents::CreateSetupIntentParamsPaymentMethodDataP24Bank;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodDataPayByBank =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodDataPaypal =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodDataPayto =
    stripe_types_00::setup_intents::CreateSetupIntentParamsPaymentMethodDataPayto;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodDataPix =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodDataRadarOptions =
    stripe_types_00::setup_intents::CreateSetupIntentParamsPaymentMethodDataRadarOptions;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodDataRevolutPay =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodDataSatispay =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodDataSepaDebit =
    stripe_types_00::setup_intents::CreateSetupIntentParamsPaymentMethodDataSepaDebit;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodDataSequra =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodDataTwint =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodDataUsBankAccountAccountHolderType =
    stripe_types_02::setup_intents::CreateSetupIntentParamsPaymentMethodDataFpxAccountHolderType;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodDataWechatPay =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule =
    stripe_types_02::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod =
    stripe_types_04::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodOptionsAmazonPay =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodOptionsBacsDebitMandateOptions =
    stripe_types_00::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsBacsDebitMandateOptions;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodOptionsBizum =
    stripe_types_04::setup_intents::CreateSetupIntentParamsMandateDataCustomerAcceptanceOffline;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsInterval =
    stripe_types_01::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsCardMandateOptionsInterval;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus =
    stripe_types_00::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureAresTransStatus;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator = stripe_types_03::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureElectronicCommerceIndicator;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo = stripe_types_02::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureNetworkOptionsCartesBancairesCbAvalgo;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureVersion =
    stripe_types_03::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsCardThreeDSecureVersion;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale =
    stripe_types_02::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsKlarnaPreferredLocale;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule =
    stripe_types_00::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsPaytoMandateOptionsPaymentSchedule;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodOptionsPixMandateOptionsAmountIncludesIof =
    stripe_types_00::setup_intents::CreateSetupIntentParamsAutomaticPaymentMethodsAllowRedirects;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodOptionsUpiMandateOptionsAmountType =
    stripe_types_01::setup_intents::CreateSetupIntentParamsPaymentMethodDataUpiMandateOptionsAmountType;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodOptionsUpiSetupFutureUsage =
    stripe_types_00::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsUpiSetupFutureUsage;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsFiltersAccountSubcategoriesItem =
    stripe_types_00::setup_intents::CreateSetupIntentParamsPaymentMethodDataUsBankAccountAccountType;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem = stripe_types_00::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissionsItem;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem = stripe_types_01::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetchItem;

#[cfg(feature = "setup_intents")]
pub type UpdateSetupIntentParamsPaymentMethodOptionsUsBankAccountVerificationMethod =
    stripe_types_04::setup_intents::CreateSetupIntentParamsPaymentMethodOptionsAcssDebitVerificationMethod;
