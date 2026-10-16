//! The generated API surface: the naming contact with users.
//!
//! Method and type names are derived from the OpenAPI paths by `stripe_codegen`.
//! If a rule change renames one of the items referenced here, this file stops
//! compiling, which is exactly the review prompt we want: renames are breaking
//! changes and should be deliberate.

#![cfg(feature = "core")]

use std::any::type_name;

use stripe::Client;

/// References every method whose name is part of the contract.
///
/// This is a compile-time check only; it is never called.
#[allow(dead_code)]
fn naming_contract(client: &Client) {
    // Top-level resources: `{verb}_{resource}`.
    let _ = stripe::customers::create_customer;
    let _ = stripe::customers::get_customer;
    let _ = stripe::customers::update_customer;
    let _ = stripe::customers::delete_customer;
    let _ = stripe::customers::list_customers;
    let _ = stripe::customers::search_customers;

    // Sub-resources keep the parent in the name.
    let _ = stripe::customers::list_customer_sources;
    let _ = stripe::customers::create_customer_source;
    let _ = stripe::customers::get_customer_source;
    let _ = stripe::customers::verify_customer_source;
    let _ = stripe::customers::get_customer_cash_balance;
    let _ = stripe::customers::update_customer_cash_balance;

    // API areas keep the domain in the name.
    let _ = stripe::checkout::create_checkout_session;
    let _ = stripe::checkout::expire_checkout_session;
    let _ = stripe::checkout::list_checkout_sessions;
    let _ = stripe::checkout::list_checkout_session_line_items;

    // Single-word actions are prefixed to the resource; multi-word actions read
    // as a whole operation.
    let _ = stripe::payment_intents::confirm_payment_intent;
    let _ = stripe::payment_intents::capture_payment_intent;
    let _ = stripe::payment_intents::cancel_payment_intent;
    let _ = stripe::payment_intents::increment_authorization;
    let _ = stripe::payment_intents::apply_customer_balance;
    let _ = stripe::payment_intents::verify_microdeposits;
    let _ = stripe::payment_intents::search_payment_intents;
    let _ = stripe::invoices::add_lines;
    let _ = stripe::invoices::remove_lines;
    let _ = stripe::invoices::update_lines;
    let _ = stripe::invoices::pay_invoice;
    let _ = stripe::invoices::finalize_invoice;
    let _ = stripe::invoices::send_invoice;
    let _ = stripe::invoices::mark_uncollectible;
    let _ = stripe::invoices::create_preview;
    let _ = stripe::invoices::list_invoice_lines;
    let _ = stripe::invoices::update_invoice_line;
    let _ = stripe::subscription_schedules::cancel_subscription_schedule;
    let _ = stripe::subscription_schedules::release_subscription_schedule;

    // Singleton and collection endpoints. These belong to optional features, so
    // they only join the contract when that feature is enabled.
    #[cfg(feature = "account")]
    let _ = stripe::account::get_account;
    #[cfg(feature = "accounts")]
    let _ = stripe::accounts::get_account;
    #[cfg(feature = "balance")]
    let _ = stripe::balance::get_balance;
    #[cfg(feature = "files")]
    let _ = stripe::files::create_file;
    #[cfg(feature = "files")]
    let _ = stripe::files::list_files;

    let _ = client;
}

/// The parameter and response types are part of the contract too.
#[allow(dead_code)]
fn type_contract(client: &Client) {
    let _: Option<stripe::customers::Customer> = None;
    let _: Option<stripe::customers::GetCustomerParams> = None;
    let _: Option<stripe::customers::CreateCustomerParams> = None;
    let _: Option<stripe::customers::ListCustomersParams> = None;
    let _: Option<stripe::checkout::CreateCheckoutSessionParams> = None;
    let _: Option<stripe::checkout::CheckoutSession> = None;
    let _: Option<stripe::payment_intents::ConfirmPaymentIntentParams> = None;
    let _: Option<stripe::payment_intents::PaymentIntent> = None;
    let _ = client;
}

#[test]
fn types_are_defined_in_their_resource_module() {
    // `type_name` reports the defining path, so this pins where a type lives.
    assert_eq!(type_name::<stripe::customers::Customer>(), "stripe::customers::Customer");
    assert_eq!(
        type_name::<stripe::customers::GetCustomerParams>(),
        "stripe::customers::GetCustomerParams"
    );
    assert_eq!(
        type_name::<stripe::checkout::CreateCheckoutSessionParams>(),
        "stripe::checkout::CreateCheckoutSessionParams"
    );
    assert_eq!(
        type_name::<stripe::payment_intents::PaymentIntent>(),
        "stripe::payment_intents::PaymentIntent"
    );
    // Shared shapes live in `common`, not in a resource module.
    assert_eq!(type_name::<stripe::common::Address>(), "stripe::common::Address");
}

#[test]
fn spec_version_is_recorded() {
    assert_eq!(stripe::SPEC_VERSION, "2026-09-30.endive");
}
