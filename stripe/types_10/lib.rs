#![allow(missing_docs)]
#![allow(clippy::all)]
#![allow(non_camel_case_types)]
#![allow(unused_crate_dependencies)]

pub use stripe_core::*;

pub mod account_links;
pub mod account_sessions;
pub mod accounts;
pub mod apps;
pub mod balance_settings;
pub mod billing;
pub mod charges;
pub mod checkout;
pub mod climate;
pub mod credit_notes;
pub mod customer_sessions;
pub mod customers;
pub mod disputes;
pub mod entitlements;
pub mod events;
pub mod financial_connections;
pub mod identity;
pub mod invoice_payments;
pub mod invoiceitems;
pub mod invoices;
pub mod issuing;
pub mod mandates;
pub mod payment_intents;
pub mod payment_links;
pub mod payment_method_configurations;
pub mod payment_methods;
pub mod payment_records;
pub mod prices;
pub mod promotion_codes;
pub mod quotes;
pub mod setup_intents;
pub mod shipping_rates;
pub mod sources;
pub mod subscription_items;
pub mod subscription_schedules;
pub mod subscriptions;
pub mod tax;
pub mod terminal;
pub mod test_helpers;
pub mod three_d_secure;
pub mod tokens;
pub mod topups;
pub mod treasury;
pub mod v2_core;
#[allow(unused_imports)]
use stripe_types_00 as _;
#[allow(unused_imports)]
use stripe_types_01 as _;
#[allow(unused_imports)]
use stripe_types_02 as _;
#[allow(unused_imports)]
use stripe_types_03 as _;
#[allow(unused_imports)]
use stripe_types_04 as _;
#[allow(unused_imports)]
use stripe_types_05 as _;
#[allow(unused_imports)]
use stripe_types_06 as _;
#[allow(unused_imports)]
use stripe_types_07 as _;
#[allow(unused_imports)]
use stripe_types_08 as _;
#[allow(unused_imports)]
use stripe_types_09 as _;
