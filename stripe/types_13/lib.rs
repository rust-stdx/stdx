#![allow(missing_docs)]
#![allow(clippy::all)]
#![allow(non_camel_case_types)]
#![allow(unused_crate_dependencies)]

pub use stripe_core::*;

pub mod accounts;
pub mod billing;
pub mod billing_portal;
pub mod checkout;
pub mod customer_sessions;
pub mod disputes;
pub mod events;
pub mod invoices;
pub mod issuing;
pub mod payment_intents;
pub mod payment_links;
pub mod plans;
pub mod product_catalog;
pub mod quotes;
pub mod setup_intents;
pub mod subscription_schedules;
pub mod subscriptions;
pub mod tax;
pub mod test_helpers;
pub mod three_d_secure;
pub mod tokens;
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
#[allow(unused_imports)]
use stripe_types_10 as _;
#[allow(unused_imports)]
use stripe_types_11 as _;
#[allow(unused_imports)]
use stripe_types_12 as _;
