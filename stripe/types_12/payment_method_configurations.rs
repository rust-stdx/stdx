#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct CreatePaymentMethodConfigurationParams {
    #[doc = "Canadian pre-authorized debit payments, check this [page](https://docs.stripe.com/payments/acss-debit) for more detai..."]
    pub acss_debit:
        Option<stripe_types_09::payment_method_configurations::CreatePaymentMethodConfigurationParamsAcssDebit>,
    #[doc = "[Affirm](https://www.affirm.com/) gives your customers a way to split purchases over a series of payments."]
    pub affirm: Option<stripe_types_10::payment_method_configurations::CreatePaymentMethodConfigurationParamsAffirm>,
    #[doc = "Afterpay gives your customers a way to pay for purchases in installments, check this page..."]
    pub afterpay_clearpay:
        Option<stripe_types_11::payment_method_configurations::CreatePaymentMethodConfigurationParamsAfterpayClearpay>,
    #[doc = "Alipay is a digital wallet in China that has more than a billion active users worldwide."]
    pub alipay: Option<stripe_types_11::payment_method_configurations::CreatePaymentMethodConfigurationParamsAlipay>,
    #[doc = "Alma is a Buy Now, Pay Later payment method that offers customers the ability to pay in 2, 3, or 4 installments."]
    pub alma: Option<stripe_types_10::payment_method_configurations::CreatePaymentMethodConfigurationParamsAlma>,
    #[doc = "Amazon Pay is a wallet payment method that lets your customers check out the same way as on Amazon."]
    pub amazon_pay:
        Option<stripe_types_11::payment_method_configurations::CreatePaymentMethodConfigurationParamsAmazonPay>,
    #[doc = "Stripe users can accept [Apple Pay](https://stripe.com/payments/apple-pay) in iOS applications in iOS 9 and later, an..."]
    pub apple_pay:
        Option<stripe_types_11::payment_method_configurations::CreatePaymentMethodConfigurationParamsApplePay>,
    #[doc = "Apple Pay Later, a payment method for customers to buy now and pay later, gives your customers a way to split purchas..."]
    pub apple_pay_later:
        Option<stripe_types_11::payment_method_configurations::CreatePaymentMethodConfigurationParamsApplePayLater>,
    #[doc = "Stripe users in Australia can accept Bulk Electronic Clearing System (BECS) direct debit payments from customers with..."]
    pub au_becs_debit:
        Option<stripe_types_10::payment_method_configurations::CreatePaymentMethodConfigurationParamsAuBecsDebit>,
    #[doc = "Stripe users in the UK can accept Bacs Direct Debit payments from customers with a UK bank account, check this page..."]
    pub bacs_debit:
        Option<stripe_types_10::payment_method_configurations::CreatePaymentMethodConfigurationParamsBacsDebit>,
    #[doc = "Bancontact is the most popular online payment method in Belgium, with over 15 million cards in circulation."]
    pub bancontact:
        Option<stripe_types_11::payment_method_configurations::CreatePaymentMethodConfigurationParamsBancontact>,
    #[doc = "Billie is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) payment method that offers businesse..."]
    pub billie: Option<stripe_types_10::payment_method_configurations::CreatePaymentMethodConfigurationParamsBillie>,
    #[doc = "To enable Bizum, buyers need a Spanish IBAN from a bank connected to Bizum."]
    pub bizum: Option<stripe_types_10::payment_method_configurations::CreatePaymentMethodConfigurationParamsBizum>,
    #[doc = "BLIK is a [single use](https://docs.stripe.com/payments/payment-methods#usage) payment method that requires customers..."]
    pub blik: Option<stripe_types_11::payment_method_configurations::CreatePaymentMethodConfigurationParamsBlik>,
    #[doc = "Boleto is an official (regulated by the Central Bank of Brazil) payment method in Brazil."]
    pub boleto: Option<stripe_types_11::payment_method_configurations::CreatePaymentMethodConfigurationParamsBoleto>,
    #[doc = "Cards are a popular way for consumers and businesses to pay online or in person."]
    pub card: Option<stripe_types_10::payment_method_configurations::CreatePaymentMethodConfigurationParamsCard>,
    #[doc = "Cartes Bancaires is France's local card network."]
    pub cartes_bancaires:
        Option<stripe_types_10::payment_method_configurations::CreatePaymentMethodConfigurationParamsCartesBancaires>,
    #[doc = "Cash App is a popular consumer app in the US that allows customers to bank, invest, send, and receive money using the..."]
    pub cashapp: Option<stripe_types_11::payment_method_configurations::CreatePaymentMethodConfigurationParamsCashapp>,
    #[doc = "[Stablecoin payments](https://docs.stripe.com/payments/stablecoin-payments) enable customers to pay in stablecoins li..."]
    pub crypto: Option<stripe_types_10::payment_method_configurations::CreatePaymentMethodConfigurationParamsCrypto>,
    #[doc = "Uses a customer’s [cash balance](https://docs.stripe.com/payments/customer-balance) for the payment."]
    pub customer_balance:
        Option<stripe_types_11::payment_method_configurations::CreatePaymentMethodConfigurationParamsCustomerBalance>,
    #[doc = "EPS is an Austria-based payment method that allows customers to complete transactions online using their bank credent..."]
    pub eps: Option<stripe_types_10::payment_method_configurations::CreatePaymentMethodConfigurationParamsEps>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Financial Process Exchange (FPX) is a Malaysia-based payment method that allows customers to complete transactions on..."]
    pub fpx: Option<stripe_types_11::payment_method_configurations::CreatePaymentMethodConfigurationParamsFpx>,
    #[doc = "Meal vouchers in France, or “titres-restaurant”, is a local benefits program commonly offered by employers for their ..."]
    pub fr_meal_voucher_conecs: Option<
        stripe_types_11::payment_method_configurations::CreatePaymentMethodConfigurationParamsFrMealVoucherConecs,
    >,
    #[doc = "giropay is a German payment method based on online banking, introduced in 2006."]
    pub giropay: Option<stripe_types_10::payment_method_configurations::CreatePaymentMethodConfigurationParamsGiropay>,
    #[doc = "Google Pay allows customers to make payments in your app or website using any credit or debit card saved to their Goo..."]
    pub google_pay:
        Option<stripe_types_11::payment_method_configurations::CreatePaymentMethodConfigurationParamsGooglePay>,
    #[doc = "GrabPay is a payment method developed by [Grab](https://www.grab.com/sg/consumer/finance/pay/)."]
    pub grabpay: Option<stripe_types_11::payment_method_configurations::CreatePaymentMethodConfigurationParamsGrabpay>,
    #[doc = "iDEAL is a Netherlands-based payment method that allows customers to complete transactions online using their bank cr..."]
    pub ideal: Option<stripe_types_11::payment_method_configurations::CreatePaymentMethodConfigurationParamsIdeal>,
    #[doc = "JCB is a credit card company based in Japan."]
    pub jcb: Option<stripe_types_10::payment_method_configurations::CreatePaymentMethodConfigurationParamsJcb>,
    #[doc = "Kakao Pay is a popular local wallet available in South Korea."]
    pub kakao_pay:
        Option<stripe_types_10::payment_method_configurations::CreatePaymentMethodConfigurationParamsKakaoPay>,
    #[doc = "Klarna gives customers a range of [payment options](https://docs.stripe.com/payments/klarna#payment-options) during c..."]
    pub klarna: Option<stripe_types_11::payment_method_configurations::CreatePaymentMethodConfigurationParamsKlarna>,
    #[doc = "Konbini allows customers in Japan to pay for bills and online purchases at convenience stores with cash."]
    pub konbini: Option<stripe_types_10::payment_method_configurations::CreatePaymentMethodConfigurationParamsKonbini>,
    #[doc = "Korean cards let users pay using locally issued cards from South Korea."]
    pub kr_card: Option<stripe_types_10::payment_method_configurations::CreatePaymentMethodConfigurationParamsKrCard>,
    #[doc = "[Link](https://docs.stripe.com/payments/link) is a payment method network."]
    pub link: Option<stripe_types_11::payment_method_configurations::CreatePaymentMethodConfigurationParamsLink>,
    #[doc = "MB WAY is the most popular wallet in Portugal."]
    pub mb_way: Option<stripe_types_10::payment_method_configurations::CreatePaymentMethodConfigurationParamsMbWay>,
    #[doc = "MobilePay is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) card wallet payment method used i..."]
    pub mobilepay:
        Option<stripe_types_10::payment_method_configurations::CreatePaymentMethodConfigurationParamsMobilepay>,
    #[doc = "Stripe users in Europe and the United States can accept Multibanco payments from customers in Portugal using Sources..."]
    pub multibanco:
        Option<stripe_types_10::payment_method_configurations::CreatePaymentMethodConfigurationParamsMultibanco>,
    #[doc = "Configuration name."]
    pub name: Option<String>,
    #[doc = "Naver Pay is a popular local wallet available in South Korea."]
    pub naver_pay:
        Option<stripe_types_11::payment_method_configurations::CreatePaymentMethodConfigurationParamsNaverPay>,
    #[doc = "Stripe users in New Zealand can accept Bulk Electronic Clearing System (BECS) direct debit payments from customers wi..."]
    pub nz_bank_account:
        Option<stripe_types_10::payment_method_configurations::CreatePaymentMethodConfigurationParamsNzBankAccount>,
    #[doc = "OXXO is a Mexican chain of convenience stores with thousands of locations across Latin America and represents nearly ..."]
    pub oxxo: Option<stripe_types_10::payment_method_configurations::CreatePaymentMethodConfigurationParamsOxxo>,
    #[doc = "Przelewy24 is a Poland-based payment method aggregator that allows customers to complete transactions online using ba..."]
    pub p24: Option<stripe_types_11::payment_method_configurations::CreatePaymentMethodConfigurationParamsP24>,
    #[doc = "Configuration's parent configuration."]
    pub parent: Option<String>,
    #[doc = "Pay by bank is a redirect payment method backed by bank transfers."]
    pub pay_by_bank:
        Option<stripe_types_11::payment_method_configurations::CreatePaymentMethodConfigurationParamsPayByBank>,
    #[doc = "PAYCO is a single-use..."]
    pub payco: Option<stripe_types_11::payment_method_configurations::CreatePaymentMethodConfigurationParamsPayco>,
    #[doc = "PayNow is a Singapore-based payment method that allows customers to make a payment using their preferred app from par..."]
    pub paynow: Option<stripe_types_11::payment_method_configurations::CreatePaymentMethodConfigurationParamsPaynow>,
    #[doc = "PayPal, a digital wallet popular with customers in Europe, allows your customers worldwide to pay using their PayPal ..."]
    pub paypal: Option<stripe_types_10::payment_method_configurations::CreatePaymentMethodConfigurationParamsPaypal>,
    #[doc = "Customers can pay with PayPay online or using the PayPay app."]
    pub paypay: Option<stripe_types_11::payment_method_configurations::CreatePaymentMethodConfigurationParamsPaypay>,
    #[doc = "PayTo is a [real-time](https://docs.stripe.com/payments/real-time) payment method that enables customers in Australia..."]
    pub payto: Option<stripe_types_10::payment_method_configurations::CreatePaymentMethodConfigurationParamsPayto>,
    #[doc = "Pix is a payment method popular in Brazil."]
    pub pix: Option<stripe_types_10::payment_method_configurations::CreatePaymentMethodConfigurationParamsPix>,
    #[doc = "PromptPay is a Thailand-based payment method that allows customers to make a payment using their preferred app from p..."]
    pub promptpay:
        Option<stripe_types_10::payment_method_configurations::CreatePaymentMethodConfigurationParamsPromptpay>,
    #[doc = "Revolut Pay, developed by Revolut, a global finance app, is a digital wallet payment method."]
    pub revolut_pay:
        Option<stripe_types_11::payment_method_configurations::CreatePaymentMethodConfigurationParamsRevolutPay>,
    #[doc = "Samsung Pay is a single-use..."]
    pub samsung_pay:
        Option<stripe_types_10::payment_method_configurations::CreatePaymentMethodConfigurationParamsSamsungPay>,
    #[doc = "Satispay is a [single-use](/payments/payment-methods#usage) payment method where customers are required to authentic..."]
    pub satispay:
        Option<stripe_types_10::payment_method_configurations::CreatePaymentMethodConfigurationParamsSatispay>,
    #[doc = "Scalapay is a [single-use](/payments/payment-methods#usage) payment method that lets customers pay in 3 or 4 installm..."]
    pub scalapay:
        Option<stripe_types_11::payment_method_configurations::CreatePaymentMethodConfigurationParamsScalapay>,
    #[doc = "The [Single Euro Payments Area (SEPA)](https://en.wikipedia.org/wiki/Single_Euro_Payments_Area) is an initiative of t..."]
    pub sepa_debit:
        Option<stripe_types_11::payment_method_configurations::CreatePaymentMethodConfigurationParamsSepaDebit>,
    #[doc = "SeQura is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) payment method that offers customers..."]
    pub sequra: Option<stripe_types_10::payment_method_configurations::CreatePaymentMethodConfigurationParamsSequra>,
    #[doc = "Stripe users in Europe and the United States can use the Payment Intents API..."]
    pub sofort: Option<stripe_types_11::payment_method_configurations::CreatePaymentMethodConfigurationParamsSofort>,
    #[doc = "Sunbit is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) payment method where customers choos..."]
    pub sunbit: Option<stripe_types_10::payment_method_configurations::CreatePaymentMethodConfigurationParamsSunbit>,
    #[doc = "Swish is a [real-time](https://docs.stripe.com/payments/real-time) payment method popular in Sweden."]
    pub swish: Option<stripe_types_11::payment_method_configurations::CreatePaymentMethodConfigurationParamsSwish>,
    #[doc = "Twint is a payment method popular in Switzerland."]
    pub twint: Option<stripe_types_10::payment_method_configurations::CreatePaymentMethodConfigurationParamsTwint>,
    #[doc = "Unified Payment Interface (UPI) is India's leading payment method with exponential growth since it launched in 2016."]
    pub upi: Option<stripe_types_11::payment_method_configurations::CreatePaymentMethodConfigurationParamsUpi>,
    #[doc = "Stripe users in the United States can accept ACH direct debit payments from customers with a US bank account using th..."]
    pub us_bank_account:
        Option<stripe_types_11::payment_method_configurations::CreatePaymentMethodConfigurationParamsUsBankAccount>,
    #[doc = "WeChat, owned by Tencent, is China's leading mobile app with over 1 billion monthly active users."]
    pub wechat_pay:
        Option<stripe_types_10::payment_method_configurations::CreatePaymentMethodConfigurationParamsWechatPay>,
    #[doc = "Zip gives your customers a way to split purchases over a series of payments."]
    pub zip: Option<stripe_types_10::payment_method_configurations::CreatePaymentMethodConfigurationParamsZip>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for CreatePaymentMethodConfigurationParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("acss_debit", self.acss_debit.as_ref());
        form.field_opt("affirm", self.affirm.as_ref());
        form.field_opt("afterpay_clearpay", self.afterpay_clearpay.as_ref());
        form.field_opt("alipay", self.alipay.as_ref());
        form.field_opt("alma", self.alma.as_ref());
        form.field_opt("amazon_pay", self.amazon_pay.as_ref());
        form.field_opt("apple_pay", self.apple_pay.as_ref());
        form.field_opt("apple_pay_later", self.apple_pay_later.as_ref());
        form.field_opt("au_becs_debit", self.au_becs_debit.as_ref());
        form.field_opt("bacs_debit", self.bacs_debit.as_ref());
        form.field_opt("bancontact", self.bancontact.as_ref());
        form.field_opt("billie", self.billie.as_ref());
        form.field_opt("bizum", self.bizum.as_ref());
        form.field_opt("blik", self.blik.as_ref());
        form.field_opt("boleto", self.boleto.as_ref());
        form.field_opt("card", self.card.as_ref());
        form.field_opt("cartes_bancaires", self.cartes_bancaires.as_ref());
        form.field_opt("cashapp", self.cashapp.as_ref());
        form.field_opt("crypto", self.crypto.as_ref());
        form.field_opt("customer_balance", self.customer_balance.as_ref());
        form.field_opt("eps", self.eps.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("fpx", self.fpx.as_ref());
        form.field_opt("fr_meal_voucher_conecs", self.fr_meal_voucher_conecs.as_ref());
        form.field_opt("giropay", self.giropay.as_ref());
        form.field_opt("google_pay", self.google_pay.as_ref());
        form.field_opt("grabpay", self.grabpay.as_ref());
        form.field_opt("ideal", self.ideal.as_ref());
        form.field_opt("jcb", self.jcb.as_ref());
        form.field_opt("kakao_pay", self.kakao_pay.as_ref());
        form.field_opt("klarna", self.klarna.as_ref());
        form.field_opt("konbini", self.konbini.as_ref());
        form.field_opt("kr_card", self.kr_card.as_ref());
        form.field_opt("link", self.link.as_ref());
        form.field_opt("mb_way", self.mb_way.as_ref());
        form.field_opt("mobilepay", self.mobilepay.as_ref());
        form.field_opt("multibanco", self.multibanco.as_ref());
        form.field_opt("name", self.name.as_ref());
        form.field_opt("naver_pay", self.naver_pay.as_ref());
        form.field_opt("nz_bank_account", self.nz_bank_account.as_ref());
        form.field_opt("oxxo", self.oxxo.as_ref());
        form.field_opt("p24", self.p24.as_ref());
        form.field_opt("parent", self.parent.as_ref());
        form.field_opt("pay_by_bank", self.pay_by_bank.as_ref());
        form.field_opt("payco", self.payco.as_ref());
        form.field_opt("paynow", self.paynow.as_ref());
        form.field_opt("paypal", self.paypal.as_ref());
        form.field_opt("paypay", self.paypay.as_ref());
        form.field_opt("payto", self.payto.as_ref());
        form.field_opt("pix", self.pix.as_ref());
        form.field_opt("promptpay", self.promptpay.as_ref());
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
        form.field_opt("upi", self.upi.as_ref());
        form.field_opt("us_bank_account", self.us_bank_account.as_ref());
        form.field_opt("wechat_pay", self.wechat_pay.as_ref());
        form.field_opt("zip", self.zip.as_ref());
    }
}

#[cfg(feature = "payment_method_configurations")]
#[derive(Debug, Clone, Default)]
pub struct UpdatePaymentMethodConfigurationParams {
    #[doc = "Canadian pre-authorized debit payments, check this [page](https://docs.stripe.com/payments/acss-debit) for more detai..."]
    pub acss_debit:
        Option<stripe_types_11::payment_method_configurations::UpdatePaymentMethodConfigurationParamsAcssDebit>,
    #[doc = "Whether the configuration can be used for new payments."]
    pub active: Option<bool>,
    #[doc = "[Affirm](https://www.affirm.com/) gives your customers a way to split purchases over a series of payments."]
    pub affirm: Option<stripe_types_11::payment_method_configurations::UpdatePaymentMethodConfigurationParamsAffirm>,
    #[doc = "Afterpay gives your customers a way to pay for purchases in installments, check this page..."]
    pub afterpay_clearpay:
        Option<stripe_types_10::payment_method_configurations::UpdatePaymentMethodConfigurationParamsAfterpayClearpay>,
    #[doc = "Alipay is a digital wallet in China that has more than a billion active users worldwide."]
    pub alipay: Option<stripe_types_10::payment_method_configurations::UpdatePaymentMethodConfigurationParamsAlipay>,
    #[doc = "Alma is a Buy Now, Pay Later payment method that offers customers the ability to pay in 2, 3, or 4 installments."]
    pub alma: Option<stripe_types_11::payment_method_configurations::UpdatePaymentMethodConfigurationParamsAlma>,
    #[doc = "Amazon Pay is a wallet payment method that lets your customers check out the same way as on Amazon."]
    pub amazon_pay:
        Option<stripe_types_10::payment_method_configurations::UpdatePaymentMethodConfigurationParamsAmazonPay>,
    #[doc = "Stripe users can accept [Apple Pay](https://stripe.com/payments/apple-pay) in iOS applications in iOS 9 and later, an..."]
    pub apple_pay:
        Option<stripe_types_10::payment_method_configurations::UpdatePaymentMethodConfigurationParamsApplePay>,
    #[doc = "Apple Pay Later, a payment method for customers to buy now and pay later, gives your customers a way to split purchas..."]
    pub apple_pay_later:
        Option<stripe_types_10::payment_method_configurations::UpdatePaymentMethodConfigurationParamsApplePayLater>,
    #[doc = "Stripe users in Australia can accept Bulk Electronic Clearing System (BECS) direct debit payments from customers with..."]
    pub au_becs_debit:
        Option<stripe_types_11::payment_method_configurations::UpdatePaymentMethodConfigurationParamsAuBecsDebit>,
    #[doc = "Stripe users in the UK can accept Bacs Direct Debit payments from customers with a UK bank account, check this page..."]
    pub bacs_debit:
        Option<stripe_types_11::payment_method_configurations::UpdatePaymentMethodConfigurationParamsBacsDebit>,
    #[doc = "Bancontact is the most popular online payment method in Belgium, with over 15 million cards in circulation."]
    pub bancontact:
        Option<stripe_types_11::payment_method_configurations::UpdatePaymentMethodConfigurationParamsBancontact>,
    #[doc = "Billie is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) payment method that offers businesse..."]
    pub billie: Option<stripe_types_11::payment_method_configurations::UpdatePaymentMethodConfigurationParamsBillie>,
    #[doc = "To enable Bizum, buyers need a Spanish IBAN from a bank connected to Bizum."]
    pub bizum: Option<stripe_types_11::payment_method_configurations::UpdatePaymentMethodConfigurationParamsBizum>,
    #[doc = "BLIK is a [single use](https://docs.stripe.com/payments/payment-methods#usage) payment method that requires customers..."]
    pub blik: Option<stripe_types_10::payment_method_configurations::UpdatePaymentMethodConfigurationParamsBlik>,
    #[doc = "Boleto is an official (regulated by the Central Bank of Brazil) payment method in Brazil."]
    pub boleto: Option<stripe_types_10::payment_method_configurations::UpdatePaymentMethodConfigurationParamsBoleto>,
    #[doc = "Cards are a popular way for consumers and businesses to pay online or in person."]
    pub card: Option<stripe_types_11::payment_method_configurations::UpdatePaymentMethodConfigurationParamsCard>,
    #[doc = "Cartes Bancaires is France's local card network."]
    pub cartes_bancaires:
        Option<stripe_types_10::payment_method_configurations::UpdatePaymentMethodConfigurationParamsCartesBancaires>,
    #[doc = "Cash App is a popular consumer app in the US that allows customers to bank, invest, send, and receive money using the..."]
    pub cashapp: Option<stripe_types_11::payment_method_configurations::UpdatePaymentMethodConfigurationParamsCashapp>,
    #[doc = "[Stablecoin payments](https://docs.stripe.com/payments/stablecoin-payments) enable customers to pay in stablecoins li..."]
    pub crypto: Option<stripe_types_11::payment_method_configurations::UpdatePaymentMethodConfigurationParamsCrypto>,
    #[doc = "Uses a customer’s [cash balance](https://docs.stripe.com/payments/customer-balance) for the payment."]
    pub customer_balance:
        Option<stripe_types_11::payment_method_configurations::UpdatePaymentMethodConfigurationParamsCustomerBalance>,
    #[doc = "EPS is an Austria-based payment method that allows customers to complete transactions online using their bank credent..."]
    pub eps: Option<stripe_types_11::payment_method_configurations::UpdatePaymentMethodConfigurationParamsEps>,
    #[doc = "Specifies which fields in the response should be expanded."]
    pub expand: Option<Vec<String>>,
    #[doc = "Financial Process Exchange (FPX) is a Malaysia-based payment method that allows customers to complete transactions on..."]
    pub fpx: Option<stripe_types_10::payment_method_configurations::UpdatePaymentMethodConfigurationParamsFpx>,
    #[doc = "Meal vouchers in France, or “titres-restaurant”, is a local benefits program commonly offered by employers for their ..."]
    pub fr_meal_voucher_conecs: Option<
        stripe_types_10::payment_method_configurations::UpdatePaymentMethodConfigurationParamsFrMealVoucherConecs,
    >,
    #[doc = "giropay is a German payment method based on online banking, introduced in 2006."]
    pub giropay: Option<stripe_types_10::payment_method_configurations::UpdatePaymentMethodConfigurationParamsGiropay>,
    #[doc = "Google Pay allows customers to make payments in your app or website using any credit or debit card saved to their Goo..."]
    pub google_pay:
        Option<stripe_types_10::payment_method_configurations::UpdatePaymentMethodConfigurationParamsGooglePay>,
    #[doc = "GrabPay is a payment method developed by [Grab](https://www.grab.com/sg/consumer/finance/pay/)."]
    pub grabpay: Option<stripe_types_11::payment_method_configurations::UpdatePaymentMethodConfigurationParamsGrabpay>,
    #[doc = "iDEAL is a Netherlands-based payment method that allows customers to complete transactions online using their bank cr..."]
    pub ideal: Option<stripe_types_10::payment_method_configurations::UpdatePaymentMethodConfigurationParamsIdeal>,
    #[doc = "JCB is a credit card company based in Japan."]
    pub jcb: Option<stripe_types_11::payment_method_configurations::UpdatePaymentMethodConfigurationParamsJcb>,
    #[doc = "Kakao Pay is a popular local wallet available in South Korea."]
    pub kakao_pay:
        Option<stripe_types_11::payment_method_configurations::UpdatePaymentMethodConfigurationParamsKakaoPay>,
    #[doc = "Klarna gives customers a range of [payment options](https://docs.stripe.com/payments/klarna#payment-options) during c..."]
    pub klarna: Option<stripe_types_10::payment_method_configurations::UpdatePaymentMethodConfigurationParamsKlarna>,
    #[doc = "Konbini allows customers in Japan to pay for bills and online purchases at convenience stores with cash."]
    pub konbini: Option<stripe_types_10::payment_method_configurations::UpdatePaymentMethodConfigurationParamsKonbini>,
    #[doc = "Korean cards let users pay using locally issued cards from South Korea."]
    pub kr_card: Option<stripe_types_11::payment_method_configurations::UpdatePaymentMethodConfigurationParamsKrCard>,
    #[doc = "[Link](https://docs.stripe.com/payments/link) is a payment method network."]
    pub link: Option<stripe_types_10::payment_method_configurations::UpdatePaymentMethodConfigurationParamsLink>,
    #[doc = "MB WAY is the most popular wallet in Portugal."]
    pub mb_way: Option<stripe_types_11::payment_method_configurations::UpdatePaymentMethodConfigurationParamsMbWay>,
    #[doc = "MobilePay is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) card wallet payment method used i..."]
    pub mobilepay:
        Option<stripe_types_11::payment_method_configurations::UpdatePaymentMethodConfigurationParamsMobilepay>,
    #[doc = "Stripe users in Europe and the United States can accept Multibanco payments from customers in Portugal using Sources..."]
    pub multibanco:
        Option<stripe_types_10::payment_method_configurations::UpdatePaymentMethodConfigurationParamsMultibanco>,
    #[doc = "Configuration name."]
    pub name: Option<String>,
    #[doc = "Naver Pay is a popular local wallet available in South Korea."]
    pub naver_pay:
        Option<stripe_types_10::payment_method_configurations::UpdatePaymentMethodConfigurationParamsNaverPay>,
    #[doc = "Stripe users in New Zealand can accept Bulk Electronic Clearing System (BECS) direct debit payments from customers wi..."]
    pub nz_bank_account:
        Option<stripe_types_11::payment_method_configurations::UpdatePaymentMethodConfigurationParamsNzBankAccount>,
    #[doc = "OXXO is a Mexican chain of convenience stores with thousands of locations across Latin America and represents nearly ..."]
    pub oxxo: Option<stripe_types_11::payment_method_configurations::UpdatePaymentMethodConfigurationParamsOxxo>,
    #[doc = "Przelewy24 is a Poland-based payment method aggregator that allows customers to complete transactions online using ba..."]
    pub p24: Option<stripe_types_10::payment_method_configurations::UpdatePaymentMethodConfigurationParamsP24>,
    #[doc = "Pay by bank is a redirect payment method backed by bank transfers."]
    pub pay_by_bank:
        Option<stripe_types_10::payment_method_configurations::UpdatePaymentMethodConfigurationParamsPayByBank>,
    #[doc = "PAYCO is a single-use..."]
    pub payco: Option<stripe_types_10::payment_method_configurations::UpdatePaymentMethodConfigurationParamsPayco>,
    #[doc = "PayNow is a Singapore-based payment method that allows customers to make a payment using their preferred app from par..."]
    pub paynow: Option<stripe_types_10::payment_method_configurations::UpdatePaymentMethodConfigurationParamsPaynow>,
    #[doc = "PayPal, a digital wallet popular with customers in Europe, allows your customers worldwide to pay using their PayPal ..."]
    pub paypal: Option<stripe_types_11::payment_method_configurations::UpdatePaymentMethodConfigurationParamsPaypal>,
    #[doc = "Customers can pay with PayPay online or using the PayPay app."]
    pub paypay: Option<stripe_types_10::payment_method_configurations::UpdatePaymentMethodConfigurationParamsPaypay>,
    #[doc = "PayTo is a [real-time](https://docs.stripe.com/payments/real-time) payment method that enables customers in Australia..."]
    pub payto: Option<stripe_types_11::payment_method_configurations::UpdatePaymentMethodConfigurationParamsPayto>,
    #[doc = "Pix is a payment method popular in Brazil."]
    pub pix: Option<stripe_types_11::payment_method_configurations::UpdatePaymentMethodConfigurationParamsPix>,
    #[doc = "PromptPay is a Thailand-based payment method that allows customers to make a payment using their preferred app from p..."]
    pub promptpay:
        Option<stripe_types_11::payment_method_configurations::UpdatePaymentMethodConfigurationParamsPromptpay>,
    #[doc = "Revolut Pay, developed by Revolut, a global finance app, is a digital wallet payment method."]
    pub revolut_pay:
        Option<stripe_types_11::payment_method_configurations::UpdatePaymentMethodConfigurationParamsRevolutPay>,
    #[doc = "Samsung Pay is a single-use..."]
    pub samsung_pay:
        Option<stripe_types_10::payment_method_configurations::UpdatePaymentMethodConfigurationParamsSamsungPay>,
    #[doc = "Satispay is a [single-use](/payments/payment-methods#usage) payment method where customers are required to authentic..."]
    pub satispay:
        Option<stripe_types_11::payment_method_configurations::UpdatePaymentMethodConfigurationParamsSatispay>,
    #[doc = "Scalapay is a [single-use](/payments/payment-methods#usage) payment method that lets customers pay in 3 or 4 installm..."]
    pub scalapay:
        Option<stripe_types_10::payment_method_configurations::UpdatePaymentMethodConfigurationParamsScalapay>,
    #[doc = "The [Single Euro Payments Area (SEPA)](https://en.wikipedia.org/wiki/Single_Euro_Payments_Area) is an initiative of t..."]
    pub sepa_debit:
        Option<stripe_types_10::payment_method_configurations::UpdatePaymentMethodConfigurationParamsSepaDebit>,
    #[doc = "SeQura is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) payment method that offers customers..."]
    pub sequra: Option<stripe_types_11::payment_method_configurations::UpdatePaymentMethodConfigurationParamsSequra>,
    #[doc = "Stripe users in Europe and the United States can use the Payment Intents API..."]
    pub sofort: Option<stripe_types_10::payment_method_configurations::UpdatePaymentMethodConfigurationParamsSofort>,
    #[doc = "Sunbit is a [single-use](https://docs.stripe.com/payments/payment-methods#usage) payment method where customers choos..."]
    pub sunbit: Option<stripe_types_11::payment_method_configurations::UpdatePaymentMethodConfigurationParamsSunbit>,
    #[doc = "Swish is a [real-time](https://docs.stripe.com/payments/real-time) payment method popular in Sweden."]
    pub swish: Option<stripe_types_10::payment_method_configurations::UpdatePaymentMethodConfigurationParamsSwish>,
    #[doc = "Twint is a payment method popular in Switzerland."]
    pub twint: Option<stripe_types_11::payment_method_configurations::UpdatePaymentMethodConfigurationParamsTwint>,
    #[doc = "Unified Payment Interface (UPI) is India's leading payment method with exponential growth since it launched in 2016."]
    pub upi: Option<stripe_types_10::payment_method_configurations::UpdatePaymentMethodConfigurationParamsUpi>,
    #[doc = "Stripe users in the United States can accept ACH direct debit payments from customers with a US bank account using th..."]
    pub us_bank_account:
        Option<stripe_types_10::payment_method_configurations::UpdatePaymentMethodConfigurationParamsUsBankAccount>,
    #[doc = "WeChat, owned by Tencent, is China's leading mobile app with over 1 billion monthly active users."]
    pub wechat_pay:
        Option<stripe_types_11::payment_method_configurations::UpdatePaymentMethodConfigurationParamsWechatPay>,
    #[doc = "Zip gives your customers a way to split purchases over a series of payments."]
    pub zip: Option<stripe_types_11::payment_method_configurations::UpdatePaymentMethodConfigurationParamsZip>,
}

#[cfg(feature = "payment_method_configurations")]
impl crate::params::FormEncode for UpdatePaymentMethodConfigurationParams {
    fn encode_form(&self, form: &mut crate::params::FormBuilder) {
        form.field_opt("acss_debit", self.acss_debit.as_ref());
        form.field_opt("active", self.active.as_ref());
        form.field_opt("affirm", self.affirm.as_ref());
        form.field_opt("afterpay_clearpay", self.afterpay_clearpay.as_ref());
        form.field_opt("alipay", self.alipay.as_ref());
        form.field_opt("alma", self.alma.as_ref());
        form.field_opt("amazon_pay", self.amazon_pay.as_ref());
        form.field_opt("apple_pay", self.apple_pay.as_ref());
        form.field_opt("apple_pay_later", self.apple_pay_later.as_ref());
        form.field_opt("au_becs_debit", self.au_becs_debit.as_ref());
        form.field_opt("bacs_debit", self.bacs_debit.as_ref());
        form.field_opt("bancontact", self.bancontact.as_ref());
        form.field_opt("billie", self.billie.as_ref());
        form.field_opt("bizum", self.bizum.as_ref());
        form.field_opt("blik", self.blik.as_ref());
        form.field_opt("boleto", self.boleto.as_ref());
        form.field_opt("card", self.card.as_ref());
        form.field_opt("cartes_bancaires", self.cartes_bancaires.as_ref());
        form.field_opt("cashapp", self.cashapp.as_ref());
        form.field_opt("crypto", self.crypto.as_ref());
        form.field_opt("customer_balance", self.customer_balance.as_ref());
        form.field_opt("eps", self.eps.as_ref());
        if let Some(values) = self.expand.as_ref() {
            form.field_array("expand", values.as_slice());
        }
        form.field_opt("fpx", self.fpx.as_ref());
        form.field_opt("fr_meal_voucher_conecs", self.fr_meal_voucher_conecs.as_ref());
        form.field_opt("giropay", self.giropay.as_ref());
        form.field_opt("google_pay", self.google_pay.as_ref());
        form.field_opt("grabpay", self.grabpay.as_ref());
        form.field_opt("ideal", self.ideal.as_ref());
        form.field_opt("jcb", self.jcb.as_ref());
        form.field_opt("kakao_pay", self.kakao_pay.as_ref());
        form.field_opt("klarna", self.klarna.as_ref());
        form.field_opt("konbini", self.konbini.as_ref());
        form.field_opt("kr_card", self.kr_card.as_ref());
        form.field_opt("link", self.link.as_ref());
        form.field_opt("mb_way", self.mb_way.as_ref());
        form.field_opt("mobilepay", self.mobilepay.as_ref());
        form.field_opt("multibanco", self.multibanco.as_ref());
        form.field_opt("name", self.name.as_ref());
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
        form.field_opt("upi", self.upi.as_ref());
        form.field_opt("us_bank_account", self.us_bank_account.as_ref());
        form.field_opt("wechat_pay", self.wechat_pay.as_ref());
        form.field_opt("zip", self.zip.as_ref());
    }
}
