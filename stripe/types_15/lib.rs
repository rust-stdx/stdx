#![allow(missing_docs)]
#![allow(clippy::all)]
#![allow(non_camel_case_types)]
#![allow(unused_crate_dependencies)]

pub use stripe_core::*;

pub mod accounts;
pub mod billing;
pub mod checkout;
pub mod common;
pub mod confirmation_tokens;
pub mod coupons;
pub mod credit_notes;
pub mod customer_sessions;
pub mod customers;
pub mod events;
pub mod financial_connections;
pub mod invoiceitems;
pub mod invoices;
pub mod payment_attempt_records;
pub mod payment_links;
pub mod payment_methods;
pub mod quotes;
pub mod radar;
pub mod terminal;
pub mod three_d_secure;
pub mod tokens;
pub mod treasury;
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
#[allow(unused_imports)]
use stripe_types_14 as _;
