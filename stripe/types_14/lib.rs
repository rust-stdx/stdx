#![allow(missing_docs)]
#![allow(clippy::all)]
#![allow(non_camel_case_types)]
#![allow(unused_crate_dependencies)]

pub use stripe_core::*;

pub mod account;
pub mod accounts;
pub mod application_fees;
pub mod balance_transactions;
pub mod billing;
pub mod charges;
pub mod common;
pub mod coupons;
pub mod customers;
pub mod disputes;
pub mod events;
pub mod invoice_payments;
pub mod invoices;
pub mod issuing;
pub mod mandates;
pub mod payment_intents;
pub mod payment_methods;
pub mod payment_records;
pub mod payouts;
pub mod promotion_codes;
pub mod refunds;
pub mod reviews;
pub mod setup_attempts;
pub mod setup_intents;
pub mod subscription_items;
pub mod subscription_schedules;
pub mod subscriptions;
pub mod tax;
pub mod tax_ids;
pub mod topups;
pub mod transfers;
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
#[allow(unused_imports)]
use stripe_types_10 as _;
#[allow(unused_imports)]
use stripe_types_11 as _;
#[allow(unused_imports)]
use stripe_types_12 as _;
#[allow(unused_imports)]
use stripe_types_13 as _;
