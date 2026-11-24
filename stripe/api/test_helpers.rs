#[cfg(feature = "test_helpers")]
#[doc = "Activate a testmode personalization design"]
pub async fn activate_test_helper_issuing_personalization_design(
    client: &crate::Client,
    personalization_design: &str,
    params: &stripe_types_05::test_helpers::ActivateTestHelperIssuingPersonalizationDesignParams,
) -> Result<stripe_types_10::issuing::IssuingPersonalizationDesign, crate::Error> {
    let path = crate::params::format_path(
        "/v1/test_helpers/issuing/personalization_designs/{personalization_design}/activate",
        &[("personalization_design", personalization_design)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Advance a test clock"]
pub async fn advance_test_helper_test_clock(
    client: &crate::Client,
    test_clock: &str,
    params: &stripe_types_03::test_helpers::AdvanceTestHelperTestClockParams,
) -> Result<stripe_types_08::test_helpers::TestHelpersTestClock, crate::Error> {
    let path = crate::params::format_path(
        "/v1/test_helpers/test_clocks/{test_clock}/advance",
        &[("test_clock", test_clock)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Capture a test-mode authorization"]
pub async fn capture_test_helper_issuing_authorization(
    client: &crate::Client,
    authorization: &str,
    params: &stripe_types_13::test_helpers::CaptureTestHelperIssuingAuthorizationParams,
) -> Result<stripe_types_14::issuing::IssuingAuthorization, crate::Error> {
    let path = crate::params::format_path(
        "/v1/test_helpers/issuing/authorizations/{authorization}/capture",
        &[("authorization", authorization)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Create a test-mode force capture"]
pub async fn create_force_capture(
    client: &crate::Client,
    params: &stripe_types_13::test_helpers::CreateForceCaptureParams,
) -> Result<stripe_types_14::issuing::IssuingTransaction, crate::Error> {
    let path = crate::params::format_path("/v1/test_helpers/issuing/transactions/create_force_capture", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Create a test Confirmation Token"]
pub async fn create_test_helper_confirmation_token(
    client: &crate::Client,
    params: &stripe_types_13::test_helpers::CreateTestHelperConfirmationTokenParams,
) -> Result<stripe_types_15::confirmation_tokens::ConfirmationToken, crate::Error> {
    let path = crate::params::format_path("/v1/test_helpers/confirmation_tokens", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Create a test-mode authorization"]
pub async fn create_test_helper_issuing_authorization(
    client: &crate::Client,
    params: &stripe_types_12::test_helpers::CreateTestHelperIssuingAuthorizationParams,
) -> Result<stripe_types_14::issuing::IssuingAuthorization, crate::Error> {
    let path = crate::params::format_path("/v1/test_helpers/issuing/authorizations", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Create a test clock"]
pub async fn create_test_helper_test_clock(
    client: &crate::Client,
    params: &stripe_types_03::test_helpers::CreateTestHelperTestClockParams,
) -> Result<stripe_types_08::test_helpers::TestHelpersTestClock, crate::Error> {
    let path = crate::params::format_path("/v1/test_helpers/test_clocks", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Test mode: Create a ReceivedCredit"]
pub async fn create_test_helper_treasury_received_credit(
    client: &crate::Client,
    params: &stripe_types_07::test_helpers::CreateTestHelperTreasuryReceivedCreditParams,
) -> Result<stripe_types_15::treasury::TreasuryReceivedCredit, crate::Error> {
    let path = crate::params::format_path("/v1/test_helpers/treasury/received_credits", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Test mode: Create a ReceivedDebit"]
pub async fn create_test_helper_treasury_received_debit(
    client: &crate::Client,
    params: &stripe_types_10::test_helpers::CreateTestHelperTreasuryReceivedDebitParams,
) -> Result<stripe_types_15::treasury::TreasuryReceivedDebit, crate::Error> {
    let path = crate::params::format_path("/v1/test_helpers/treasury/received_debits", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Create a test-mode unlinked refund"]
pub async fn create_unlinked_refund(
    client: &crate::Client,
    params: &stripe_types_13::test_helpers::CreateUnlinkedRefundParams,
) -> Result<stripe_types_14::issuing::IssuingTransaction, crate::Error> {
    let path = crate::params::format_path("/v1/test_helpers/issuing/transactions/create_unlinked_refund", &[]);
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Deactivate a testmode personalization design"]
pub async fn deactivate_test_helper_issuing_personalization_design(
    client: &crate::Client,
    personalization_design: &str,
    params: &stripe_types_05::test_helpers::DeactivateTestHelperIssuingPersonalizationDesignParams,
) -> Result<stripe_types_10::issuing::IssuingPersonalizationDesign, crate::Error> {
    let path = crate::params::format_path(
        "/v1/test_helpers/issuing/personalization_designs/{personalization_design}/deactivate",
        &[("personalization_design", personalization_design)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Delete a test clock"]
pub async fn delete_test_helper_test_clock(
    client: &crate::Client,
    test_clock: &str,
    params: &crate::params::Empty,
) -> Result<stripe_types_06::test_helpers::DeletedTestHelpersTestClock, crate::Error> {
    let path = crate::params::format_path("/v1/test_helpers/test_clocks/{test_clock}", &[("test_clock", test_clock)]);
    client.delete_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Deliver a testmode card"]
pub async fn deliver_test_helper_issuing_card_shipping(
    client: &crate::Client,
    card: &str,
    params: &stripe_types_05::test_helpers::DeliverTestHelperIssuingCardShippingParams,
) -> Result<stripe_types_13::issuing::IssuingCard, crate::Error> {
    let path = crate::params::format_path("/v1/test_helpers/issuing/cards/{card}/shipping/deliver", &[("card", card)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Expire a test-mode authorization"]
pub async fn expire_test_helper_issuing_authorization(
    client: &crate::Client,
    authorization: &str,
    params: &stripe_types_03::test_helpers::ExpireTestHelperIssuingAuthorizationParams,
) -> Result<stripe_types_14::issuing::IssuingAuthorization, crate::Error> {
    let path = crate::params::format_path(
        "/v1/test_helpers/issuing/authorizations/{authorization}/expire",
        &[("authorization", authorization)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Expire a pending refund."]
pub async fn expire_test_helper_refund(
    client: &crate::Client,
    refund: &str,
    params: &stripe_types_06::test_helpers::ExpireTestHelperRefundParams,
) -> Result<stripe_types_14::refunds::Refund, crate::Error> {
    let path = crate::params::format_path("/v1/test_helpers/refunds/{refund}/expire", &[("refund", refund)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Fail a testmode card"]
pub async fn fail_test_helper_issuing_card_shipping(
    client: &crate::Client,
    card: &str,
    params: &stripe_types_05::test_helpers::FailTestHelperIssuingCardShippingParams,
) -> Result<stripe_types_13::issuing::IssuingCard, crate::Error> {
    let path = crate::params::format_path("/v1/test_helpers/issuing/cards/{card}/shipping/fail", &[("card", card)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Test mode: Fail an InboundTransfer"]
pub async fn fail_test_helper_treasury_inbound_transfer(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_08::test_helpers::FailTestHelperTreasuryInboundTransferParams,
) -> Result<stripe_types_15::treasury::TreasuryInboundTransfer, crate::Error> {
    let path = crate::params::format_path("/v1/test_helpers/treasury/inbound_transfers/{id}/fail", &[("id", id)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Test mode: Fail an OutboundPayment"]
pub async fn fail_test_helper_treasury_outbound_payment(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_06::test_helpers::FailTestHelperTreasuryOutboundPaymentParams,
) -> Result<stripe_types_15::treasury::TreasuryOutboundPayment, crate::Error> {
    let path = crate::params::format_path("/v1/test_helpers/treasury/outbound_payments/{id}/fail", &[("id", id)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Test mode: Fail an OutboundTransfer"]
pub async fn fail_test_helper_treasury_outbound_transfer(
    client: &crate::Client,
    outbound_transfer: &str,
    params: &stripe_types_05::test_helpers::FailTestHelperTreasuryOutboundTransferParams,
) -> Result<stripe_types_15::treasury::TreasuryOutboundTransfer, crate::Error> {
    let path = crate::params::format_path(
        "/v1/test_helpers/treasury/outbound_transfers/{outbound_transfer}/fail",
        &[("outbound_transfer", outbound_transfer)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Finalize a test-mode authorization's amount"]
pub async fn finalize_amount(
    client: &crate::Client,
    authorization: &str,
    params: &stripe_types_12::test_helpers::FinalizeAmountParams,
) -> Result<stripe_types_14::issuing::IssuingAuthorization, crate::Error> {
    let path = crate::params::format_path(
        "/v1/test_helpers/issuing/authorizations/{authorization}/finalize_amount",
        &[("authorization", authorization)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Fund a test mode cash balance"]
pub async fn fund_cash_balance(
    client: &crate::Client,
    customer: &str,
    params: &stripe_types_00::test_helpers::FundCashBalanceParams,
) -> Result<stripe_types_14::customers::CustomerCashBalanceTransaction, crate::Error> {
    let path = crate::params::format_path(
        "/v1/test_helpers/customers/{customer}/fund_cash_balance",
        &[("customer", customer)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Retrieve a test clock"]
pub async fn get_test_helper_test_clock(
    client: &crate::Client,
    test_clock: &str,
    params: &stripe_types_05::test_helpers::GetTestHelperTestClockParams,
) -> Result<stripe_types_08::test_helpers::TestHelpersTestClock, crate::Error> {
    let path = crate::params::format_path("/v1/test_helpers/test_clocks/{test_clock}", &[("test_clock", test_clock)]);
    client.get(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Increment a test-mode authorization"]
pub async fn increment_test_helper_issuing_authorization(
    client: &crate::Client,
    authorization: &str,
    params: &stripe_types_01::test_helpers::IncrementTestHelperIssuingAuthorizationParams,
) -> Result<stripe_types_14::issuing::IssuingAuthorization, crate::Error> {
    let path = crate::params::format_path(
        "/v1/test_helpers/issuing/authorizations/{authorization}/increment",
        &[("authorization", authorization)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "List all test clocks"]
pub async fn list_test_helper_test_clocks(
    client: &crate::Client,
    params: &stripe_types_04::test_helpers::ListTestHelperTestClocksParams,
) -> Result<crate::List<stripe_types_08::test_helpers::TestHelpersTestClock>, crate::Error> {
    let path = crate::params::format_path("/v1/test_helpers/test_clocks", &[]);
    client.get(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Test mode: Post an OutboundPayment"]
pub async fn post_test_helper_treasury_outbound_payment(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_05::test_helpers::PostTestHelperTreasuryOutboundPaymentParams,
) -> Result<stripe_types_15::treasury::TreasuryOutboundPayment, crate::Error> {
    let path = crate::params::format_path("/v1/test_helpers/treasury/outbound_payments/{id}/post", &[("id", id)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Test mode: Post an OutboundTransfer"]
pub async fn post_test_helper_treasury_outbound_transfer(
    client: &crate::Client,
    outbound_transfer: &str,
    params: &stripe_types_06::test_helpers::PostTestHelperTreasuryOutboundTransferParams,
) -> Result<stripe_types_15::treasury::TreasuryOutboundTransfer, crate::Error> {
    let path = crate::params::format_path(
        "/v1/test_helpers/treasury/outbound_transfers/{outbound_transfer}/post",
        &[("outbound_transfer", outbound_transfer)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Simulate presenting a payment method"]
pub async fn present_payment_method(
    client: &crate::Client,
    reader: &str,
    params: &stripe_types_08::test_helpers::PresentPaymentMethodParams,
) -> Result<stripe_types_15::terminal::TerminalReader, crate::Error> {
    let path = crate::params::format_path(
        "/v1/test_helpers/terminal/readers/{reader}/present_payment_method",
        &[("reader", reader)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Refund a test-mode transaction"]
pub async fn refund_test_helper_issuing_transaction(
    client: &crate::Client,
    transaction: &str,
    params: &stripe_types_02::test_helpers::RefundTestHelperIssuingTransactionParams,
) -> Result<stripe_types_14::issuing::IssuingTransaction, crate::Error> {
    let path = crate::params::format_path(
        "/v1/test_helpers/issuing/transactions/{transaction}/refund",
        &[("transaction", transaction)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Reject a testmode personalization design"]
pub async fn reject_test_helper_issuing_personalization_design(
    client: &crate::Client,
    personalization_design: &str,
    params: &stripe_types_09::test_helpers::RejectTestHelperIssuingPersonalizationDesignParams,
) -> Result<stripe_types_10::issuing::IssuingPersonalizationDesign, crate::Error> {
    let path = crate::params::format_path(
        "/v1/test_helpers/issuing/personalization_designs/{personalization_design}/reject",
        &[("personalization_design", personalization_design)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Respond to fraud challenge"]
pub async fn respond_test_helper_issuing_authorization_fraud_challenge(
    client: &crate::Client,
    authorization: &str,
    params: &stripe_types_03::test_helpers::RespondTestHelperIssuingAuthorizationFraudChallengeParams,
) -> Result<stripe_types_14::issuing::IssuingAuthorization, crate::Error> {
    let path = crate::params::format_path(
        "/v1/test_helpers/issuing/authorizations/{authorization}/fraud_challenges/respond",
        &[("authorization", authorization)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Return a testmode card"]
pub async fn return_test_helper_issuing_card_shipping(
    client: &crate::Client,
    card: &str,
    params: &stripe_types_05::test_helpers::ReturnTestHelperIssuingCardShippingParams,
) -> Result<stripe_types_13::issuing::IssuingCard, crate::Error> {
    let path = crate::params::format_path("/v1/test_helpers/issuing/cards/{card}/shipping/return", &[("card", card)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Test mode: Return an InboundTransfer"]
pub async fn return_test_helper_treasury_inbound_transfer(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_06::test_helpers::ReturnTestHelperTreasuryInboundTransferParams,
) -> Result<stripe_types_15::treasury::TreasuryInboundTransfer, crate::Error> {
    let path = crate::params::format_path("/v1/test_helpers/treasury/inbound_transfers/{id}/return", &[("id", id)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Test mode: Return an OutboundPayment"]
pub async fn return_test_helper_treasury_outbound_payment(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_09::test_helpers::ReturnTestHelperTreasuryOutboundPaymentParams,
) -> Result<stripe_types_15::treasury::TreasuryOutboundPayment, crate::Error> {
    let path = crate::params::format_path("/v1/test_helpers/treasury/outbound_payments/{id}/return", &[("id", id)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Test mode: Return an OutboundTransfer"]
pub async fn return_test_helper_treasury_outbound_transfer(
    client: &crate::Client,
    outbound_transfer: &str,
    params: &stripe_types_10::test_helpers::ReturnTestHelperTreasuryOutboundTransferParams,
) -> Result<stripe_types_15::treasury::TreasuryOutboundTransfer, crate::Error> {
    let path = crate::params::format_path(
        "/v1/test_helpers/treasury/outbound_transfers/{outbound_transfer}/return",
        &[("outbound_transfer", outbound_transfer)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Reverse a test-mode authorization"]
pub async fn reverse_test_helper_issuing_authorization(
    client: &crate::Client,
    authorization: &str,
    params: &stripe_types_00::test_helpers::ReverseTestHelperIssuingAuthorizationParams,
) -> Result<stripe_types_14::issuing::IssuingAuthorization, crate::Error> {
    let path = crate::params::format_path(
        "/v1/test_helpers/issuing/authorizations/{authorization}/reverse",
        &[("authorization", authorization)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Ship a testmode card"]
pub async fn ship_test_helper_issuing_card_shipping(
    client: &crate::Client,
    card: &str,
    params: &stripe_types_06::test_helpers::ShipTestHelperIssuingCardShippingParams,
) -> Result<stripe_types_13::issuing::IssuingCard, crate::Error> {
    let path = crate::params::format_path("/v1/test_helpers/issuing/cards/{card}/shipping/ship", &[("card", card)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Submit a testmode card"]
pub async fn submit_test_helper_issuing_card_shipping(
    client: &crate::Client,
    card: &str,
    params: &stripe_types_06::test_helpers::SubmitTestHelperIssuingCardShippingParams,
) -> Result<stripe_types_13::issuing::IssuingCard, crate::Error> {
    let path = crate::params::format_path("/v1/test_helpers/issuing/cards/{card}/shipping/submit", &[("card", card)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Simulate a successful input collection"]
pub async fn succeed_input_collection(
    client: &crate::Client,
    reader: &str,
    params: &stripe_types_05::test_helpers::SucceedInputCollectionParams,
) -> Result<stripe_types_15::terminal::TerminalReader, crate::Error> {
    let path = crate::params::format_path(
        "/v1/test_helpers/terminal/readers/{reader}/succeed_input_collection",
        &[("reader", reader)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Test mode: Succeed an InboundTransfer"]
pub async fn succeed_test_helper_treasury_inbound_transfer(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_06::test_helpers::SucceedTestHelperTreasuryInboundTransferParams,
) -> Result<stripe_types_15::treasury::TreasuryInboundTransfer, crate::Error> {
    let path = crate::params::format_path("/v1/test_helpers/treasury/inbound_transfers/{id}/succeed", &[("id", id)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Simulate an input collection timeout"]
pub async fn timeout_input_collection(
    client: &crate::Client,
    reader: &str,
    params: &stripe_types_06::test_helpers::TimeoutInputCollectionParams,
) -> Result<stripe_types_15::terminal::TerminalReader, crate::Error> {
    let path = crate::params::format_path(
        "/v1/test_helpers/terminal/readers/{reader}/timeout_input_collection",
        &[("reader", reader)],
    );
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Test mode: Update an OutboundPayment"]
pub async fn update_test_helper_treasury_outbound_payment(
    client: &crate::Client,
    id: &str,
    params: &stripe_types_09::test_helpers::UpdateTestHelperTreasuryOutboundPaymentParams,
) -> Result<stripe_types_15::treasury::TreasuryOutboundPayment, crate::Error> {
    let path = crate::params::format_path("/v1/test_helpers/treasury/outbound_payments/{id}", &[("id", id)]);
    client.post_form(&path, params).await
}

#[cfg(feature = "test_helpers")]
#[doc = "Test mode: Update an OutboundTransfer"]
pub async fn update_test_helper_treasury_outbound_transfer(
    client: &crate::Client,
    outbound_transfer: &str,
    params: &stripe_types_11::test_helpers::UpdateTestHelperTreasuryOutboundTransferParams,
) -> Result<stripe_types_15::treasury::TreasuryOutboundTransfer, crate::Error> {
    let path = crate::params::format_path(
        "/v1/test_helpers/treasury/outbound_transfers/{outbound_transfer}",
        &[("outbound_transfer", outbound_transfer)],
    );
    client.post_form(&path, params).await
}
