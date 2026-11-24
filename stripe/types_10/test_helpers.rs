#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsFleet {
    #[doc = "Answers to prompts presented to the cardholder at the point of sale."]
    pub cardholder_prompt_data: Option<stripe_types_06::test_helpers::CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsFleetCardholderPromptData>,
    #[doc = "The type of purchase."]
    pub purchase_type: Option<stripe_types_05::test_helpers::CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsFleetPurchaseType>,
    #[doc = "More information about the total amount."]
    pub reported_breakdown: Option<stripe_types_09::test_helpers::CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsFleetReportedBreakdown>,
    #[doc = "The type of fuel service."]
    pub service_type: Option<stripe_types_06::test_helpers::CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsFleetServiceType>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CaptureTestHelperIssuingAuthorizationParamsPurchaseDetailsFleet {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("cardholder_prompt_data", self.cardholder_prompt_data.as_ref());
        form.field_opt("purchase_type", self.purchase_type.as_ref());
        form.field_opt("reported_breakdown", self.reported_breakdown.as_ref());
        form.field_opt("service_type", self.service_type.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone)]
pub struct CreateTestHelperConfirmationTokenParamsPaymentMethodData {
    #[doc = "If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method."]
    pub acss_debit:
        Option<stripe_types_04::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAcssDebit>,
    #[doc = "If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method."]
    pub affirm: Option<stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAffirm>,
    #[doc = "If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method."]
    pub afterpay_clearpay:
        Option<stripe_types_06::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAfterpayClearpay>,
    #[doc = "If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method."]
    pub alipay: Option<stripe_types_05::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAlipay>,
    #[doc = "This field indicates whether this payment method can be shown again to its customer in a checkout flow."]
    pub allow_redisplay:
        Option<stripe_types_02::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAllowRedisplay>,
    #[doc = "If this is an Alma PaymentMethod, this hash contains details about the Alma payment method."]
    pub alma: Option<stripe_types_06::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAlma>,
    #[doc = "If this is an AmazonPay PaymentMethod, this hash contains details about the AmazonPay payment method."]
    pub amazon_pay:
        Option<stripe_types_05::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAmazonPay>,
    #[doc = "If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account."]
    pub au_becs_debit:
        Option<stripe_types_04::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataAuBecsDebit>,
    #[doc = "If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account."]
    pub bacs_debit:
        Option<stripe_types_03::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataBacsDebit>,
    #[doc = "If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method."]
    pub bancontact:
        Option<stripe_types_06::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataBancontact>,
    #[doc = "If this is a `billie` PaymentMethod, this hash contains details about the Billie payment method."]
    pub billie: Option<stripe_types_06::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataBillie>,
    #[doc = "Billing information associated with the PaymentMethod that may be used or required by particular types of payment met..."]
    pub billing_details:
        Option<stripe_types_05::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataBillingDetails>,
    #[doc = "If this is a `bizum` PaymentMethod, this hash contains details about the Bizum payment method."]
    pub bizum: Option<stripe_types_05::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataBizum>,
    #[doc = "If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method."]
    pub blik: Option<stripe_types_05::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataBlik>,
    #[doc = "If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method."]
    pub boleto: Option<stripe_types_01::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataBoleto>,
    #[doc = "If this is a `cashapp` PaymentMethod, this hash contains details about the Cash App Pay payment method."]
    pub cashapp: Option<stripe_types_05::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataCashapp>,
    #[doc = "If this is a Crypto PaymentMethod, this hash contains details about the Crypto payment method."]
    pub crypto: Option<stripe_types_05::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataCrypto>,
    #[doc = "If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method."]
    pub customer_balance:
        Option<stripe_types_06::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataCustomerBalance>,
    #[doc = "If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method."]
    pub eps: Option<stripe_types_05::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataEps>,
    #[doc = "If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method."]
    pub fpx: Option<stripe_types_05::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataFpx>,
    #[doc = "If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method."]
    pub giropay: Option<stripe_types_06::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataGiropay>,
    #[doc = "If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method."]
    pub grabpay: Option<stripe_types_05::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataGrabpay>,
    #[doc = "If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method."]
    pub ideal: Option<stripe_types_06::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataIdeal>,
    #[doc = "If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method."]
    pub interac_present:
        Option<stripe_types_05::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataInteracPresent>,
    #[doc = "If this is a `kakao_pay` PaymentMethod, this hash contains details about the Kakao Pay payment method."]
    pub kakao_pay:
        Option<stripe_types_06::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataKakaoPay>,
    #[doc = "If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method."]
    pub klarna: Option<stripe_types_06::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataKlarna>,
    #[doc = "If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method."]
    pub konbini: Option<stripe_types_06::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataKonbini>,
    #[doc = "If this is a `kr_card` PaymentMethod, this hash contains details about the Korean Card payment method."]
    pub kr_card: Option<stripe_types_06::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataKrCard>,
    #[doc = "If this is an `Link` PaymentMethod, this hash contains details about the Link payment method (Link is also known as O..."]
    pub link: Option<stripe_types_06::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataLink>,
    #[doc = "If this is a MB WAY PaymentMethod, this hash contains details about the MB WAY payment method."]
    pub mb_way: Option<stripe_types_06::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataMbWay>,
    #[doc = "Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object."]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[doc = "If this is a `mobilepay` PaymentMethod, this hash contains details about the MobilePay payment method."]
    pub mobilepay:
        Option<stripe_types_06::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataMobilepay>,
    #[doc = "If this is a `multibanco` PaymentMethod, this hash contains details about the Multibanco payment method."]
    pub multibanco:
        Option<stripe_types_05::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataMultibanco>,
    #[doc = "If this is a `naver_pay` PaymentMethod, this hash contains details about the Naver Pay payment method."]
    pub naver_pay:
        Option<stripe_types_05::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataNaverPay>,
    #[doc = "If this is an nz_bank_account PaymentMethod, this hash contains details about the nz_bank_account payment method."]
    pub nz_bank_account:
        Option<stripe_types_02::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataNzBankAccount>,
    #[doc = "If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method."]
    pub oxxo: Option<stripe_types_05::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataOxxo>,
    #[doc = "If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method."]
    pub p24: Option<stripe_types_06::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataP24>,
    #[doc = "If this is a `pay_by_bank` PaymentMethod, this hash contains details about the PayByBank payment method."]
    pub pay_by_bank:
        Option<stripe_types_05::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataPayByBank>,
    #[doc = "If this is a `payco` PaymentMethod, this hash contains details about the PAYCO payment method."]
    pub payco: Option<stripe_types_05::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataPayco>,
    #[doc = "If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method."]
    pub paynow: Option<stripe_types_05::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataPaynow>,
    #[doc = "If this is a `paypal` PaymentMethod, this hash contains details about the PayPal payment method."]
    pub paypal: Option<stripe_types_06::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataPaypal>,
    #[doc = "If this is a `paypay` PaymentMethod, this hash contains details about the PayPay payment method."]
    pub paypay: Option<stripe_types_05::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataPaypay>,
    #[doc = "If this is a `payto` PaymentMethod, this hash contains details about the PayTo payment method."]
    pub payto: Option<stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataPayto>,
    #[doc = "If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method."]
    pub pix: Option<stripe_types_06::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataPix>,
    #[doc = "If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method."]
    pub promptpay:
        Option<stripe_types_06::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataPromptpay>,
    #[doc = "Options to configure Radar."]
    pub radar_options:
        Option<stripe_types_04::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataRadarOptions>,
    #[doc = "If this is a `revolut_pay` PaymentMethod, this hash contains details about the Revolut Pay payment method."]
    pub revolut_pay:
        Option<stripe_types_06::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataRevolutPay>,
    #[doc = "If this is a `samsung_pay` PaymentMethod, this hash contains details about the SamsungPay payment method."]
    pub samsung_pay:
        Option<stripe_types_05::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataSamsungPay>,
    #[doc = "If this is a `satispay` PaymentMethod, this hash contains details about the Satispay payment method."]
    pub satispay:
        Option<stripe_types_05::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataSatispay>,
    #[doc = "If this is a Scalapay PaymentMethod, this hash contains details about the Scalapay payment method."]
    pub scalapay:
        Option<stripe_types_06::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataScalapay>,
    #[doc = "If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account."]
    pub sepa_debit:
        Option<stripe_types_02::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataSepaDebit>,
    #[doc = "If this is a SeQura PaymentMethod, this hash contains details about the SeQura payment method."]
    pub sequra: Option<stripe_types_06::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataSequra>,
    #[doc = "If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method."]
    pub sofort: Option<stripe_types_05::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataSofort>,
    #[doc = "If this is a `sunbit` PaymentMethod, this hash contains details about the Sunbit payment method."]
    pub sunbit: Option<stripe_types_05::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataSunbit>,
    #[doc = "If this is a `swish` PaymentMethod, this hash contains details about the Swish payment method."]
    pub swish: Option<stripe_types_06::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataSwish>,
    #[doc = "If this is a TWINT PaymentMethod, this hash contains details about the TWINT payment method."]
    pub twint: Option<stripe_types_05::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataTwint>,
    #[doc = "The type of the PaymentMethod."]
    pub r#type: stripe_types_00::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataType,
    #[doc = "If this is a `upi` PaymentMethod, this hash contains details about the UPI payment method."]
    pub upi: Option<stripe_types_09::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataUpi>,
    #[doc = "If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method."]
    pub us_bank_account:
        Option<stripe_types_07::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataUsBankAccount>,
    #[doc = "If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method."]
    pub wechat_pay:
        Option<stripe_types_05::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataWechatPay>,
    #[doc = "If this is a `zip` PaymentMethod, this hash contains details about the Zip payment method."]
    pub zip: Option<stripe_types_05::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodDataZip>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodData {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("acss_debit", self.acss_debit.as_ref());
        form.field_opt("affirm", self.affirm.as_ref());
        form.field_opt("afterpay_clearpay", self.afterpay_clearpay.as_ref());
        form.field_opt("alipay", self.alipay.as_ref());
        form.field_opt("allow_redisplay", self.allow_redisplay.as_ref());
        form.field_opt("alma", self.alma.as_ref());
        form.field_opt("amazon_pay", self.amazon_pay.as_ref());
        form.field_opt("au_becs_debit", self.au_becs_debit.as_ref());
        form.field_opt("bacs_debit", self.bacs_debit.as_ref());
        form.field_opt("bancontact", self.bancontact.as_ref());
        form.field_opt("billie", self.billie.as_ref());
        form.field_opt("billing_details", self.billing_details.as_ref());
        form.field_opt("bizum", self.bizum.as_ref());
        form.field_opt("blik", self.blik.as_ref());
        form.field_opt("boleto", self.boleto.as_ref());
        form.field_opt("cashapp", self.cashapp.as_ref());
        form.field_opt("crypto", self.crypto.as_ref());
        form.field_opt("customer_balance", self.customer_balance.as_ref());
        form.field_opt("eps", self.eps.as_ref());
        form.field_opt("fpx", self.fpx.as_ref());
        form.field_opt("giropay", self.giropay.as_ref());
        form.field_opt("grabpay", self.grabpay.as_ref());
        form.field_opt("ideal", self.ideal.as_ref());
        form.field_opt("interac_present", self.interac_present.as_ref());
        form.field_opt("kakao_pay", self.kakao_pay.as_ref());
        form.field_opt("klarna", self.klarna.as_ref());
        form.field_opt("konbini", self.konbini.as_ref());
        form.field_opt("kr_card", self.kr_card.as_ref());
        form.field_opt("link", self.link.as_ref());
        form.field_opt("mb_way", self.mb_way.as_ref());
        if let Some(values) = self.metadata.as_ref() {
            for (key, value) in values.iter() {
                form.field_map("metadata", key, value);
            }
        }
        form.field_opt("mobilepay", self.mobilepay.as_ref());
        form.field_opt("multibanco", self.multibanco.as_ref());
        form.field_opt("naver_pay", self.naver_pay.as_ref());
        form.field_opt("nz_bank_account", self.nz_bank_account.as_ref());
        form.field_opt("oxxo", self.oxxo.as_ref());
        form.field_opt("p24", self.p24.as_ref());
        form.field_opt("pay_by_bank", self.pay_by_bank.as_ref());
        form.field_opt("payco", self.payco.as_ref());
        form.field_opt("paynow", self.paynow.as_ref());
        form.field_opt("paypal", self.paypal.as_ref());
        form.field_opt("paypay", self.paypay.as_ref());
        form.field_opt("payto", self.payto.as_ref());
        form.field_opt("pix", self.pix.as_ref());
        form.field_opt("promptpay", self.promptpay.as_ref());
        form.field_opt("radar_options", self.radar_options.as_ref());
        form.field_opt("revolut_pay", self.revolut_pay.as_ref());
        form.field_opt("samsung_pay", self.samsung_pay.as_ref());
        form.field_opt("satispay", self.satispay.as_ref());
        form.field_opt("scalapay", self.scalapay.as_ref());
        form.field_opt("sepa_debit", self.sepa_debit.as_ref());
        form.field_opt("sequra", self.sequra.as_ref());
        form.field_opt("sofort", self.sofort.as_ref());
        form.field_opt("sunbit", self.sunbit.as_ref());
        form.field_opt("swish", self.swish.as_ref());
        form.field_opt("twint", self.twint.as_ref());
        form.field("type", &self.r#type);
        form.field_opt("upi", self.upi.as_ref());
        form.field_opt("us_bank_account", self.us_bank_account.as_ref());
        form.field_opt("wechat_pay", self.wechat_pay.as_ref());
        form.field_opt("zip", self.zip.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperConfirmationTokenParamsPaymentMethodOptionsCard {
    #[doc = "Installment configuration for payments confirmed using this ConfirmationToken."]
    pub installments: Option<
        stripe_types_07::test_helpers::CreateTestHelperConfirmationTokenParamsPaymentMethodOptionsCardInstallments,
    >,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperConfirmationTokenParamsPaymentMethodOptionsCard {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("installments", self.installments.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateTestHelperIssuingAuthorizationParamsFleet {
    #[doc = "Answers to prompts presented to the cardholder at the point of sale."]
    pub cardholder_prompt_data:
        Option<stripe_types_04::test_helpers::CreateTestHelperIssuingAuthorizationParamsFleetCardholderPromptData>,
    #[doc = "The type of purchase."]
    pub purchase_type:
        Option<stripe_types_02::test_helpers::CreateTestHelperIssuingAuthorizationParamsFleetPurchaseType>,
    #[doc = "More information about the total amount."]
    pub reported_breakdown:
        Option<stripe_types_08::test_helpers::CreateTestHelperIssuingAuthorizationParamsFleetReportedBreakdown>,
    #[doc = "The type of fuel service."]
    pub service_type: Option<stripe_types_00::test_helpers::CreateTestHelperIssuingAuthorizationParamsFleetServiceType>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperIssuingAuthorizationParamsFleet {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("cardholder_prompt_data", self.cardholder_prompt_data.as_ref());
        form.field_opt("purchase_type", self.purchase_type.as_ref());
        form.field_opt("reported_breakdown", self.reported_breakdown.as_ref());
        form.field_opt("service_type", self.service_type.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone)]
pub struct CreateTestHelperTreasuryReceivedDebitParams {
    #[doc = "Amount (in cents) to be transferred."]
    pub amount: i64,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase."]
    pub currency: String,
    #[doc = "An arbitrary string attached to the object."]
    pub description: Option<String>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "The FinancialAccount to pull funds from."]
    pub financial_account: String,
    #[doc = "Initiating payment method details for the object."]
    pub initiating_payment_method_details: Option<
        stripe_types_08::test_helpers::CreateTestHelperTreasuryReceivedDebitParamsInitiatingPaymentMethodDetails,
    >,
    #[doc = "Specifies the network rails to be used."]
    pub network: stripe_types_01::test_helpers::CreateTestHelperTreasuryReceivedDebitParamsNetwork,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateTestHelperTreasuryReceivedDebitParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field("amount", &self.amount);
        form.field("currency", &self.currency);
        form.field_opt("description", self.description.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field("financial_account", &self.financial_account);
        form.field_opt(
            "initiating_payment_method_details",
            self.initiating_payment_method_details.as_ref(),
        );
        form.field("network", &self.network);
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct CreateUnlinkedRefundParamsPurchaseDetailsFleet {
    #[doc = "Answers to prompts presented to the cardholder at the point of sale."]
    pub cardholder_prompt_data:
        Option<stripe_types_05::test_helpers::CreateUnlinkedRefundParamsPurchaseDetailsFleetCardholderPromptData>,
    #[doc = "The type of purchase."]
    pub purchase_type:
        Option<stripe_types_06::test_helpers::CreateUnlinkedRefundParamsPurchaseDetailsFleetPurchaseType>,
    #[doc = "More information about the total amount."]
    pub reported_breakdown:
        Option<stripe_types_09::test_helpers::CreateUnlinkedRefundParamsPurchaseDetailsFleetReportedBreakdown>,
    #[doc = "The type of fuel service."]
    pub service_type: Option<stripe_types_06::test_helpers::CreateUnlinkedRefundParamsPurchaseDetailsFleetServiceType>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for CreateUnlinkedRefundParamsPurchaseDetailsFleet {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("cardholder_prompt_data", self.cardholder_prompt_data.as_ref());
        form.field_opt("purchase_type", self.purchase_type.as_ref());
        form.field_opt("reported_breakdown", self.reported_breakdown.as_ref());
        form.field_opt("service_type", self.service_type.as_ref());
    }
}

#[cfg(feature = "test_helpers")]
#[derive(Debug, Clone, Default)]
pub struct ReturnTestHelperTreasuryOutboundTransferParams {
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Details about a returned OutboundTransfer."]
    pub returned_details:
        Option<stripe_types_09::test_helpers::ReturnTestHelperTreasuryOutboundTransferParamsReturnedDetails>,
}

#[cfg(feature = "test_helpers")]
impl crate::params::FormEncode for ReturnTestHelperTreasuryOutboundTransferParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("returned_details", self.returned_details.as_ref());
    }
}
