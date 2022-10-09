//! Mpesa sdk is library for making api requests to safaricom Mpesa. it acts as wrappper for mpesa

// TODO: Error Handling
// NOTE: Enable cfg feature
// HACK: Upload v1 crate
// TODO: Cargo Docs
///
mod client;
pub use crate::client::*;

mod services;
pub use crate::services::*;
