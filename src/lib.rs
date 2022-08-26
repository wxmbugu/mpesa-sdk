//! Mpesa sdk is library for making api requests to safaricom Mpesa. it acts as wrappper for mpesa

// TODO: Error Handling
// FIX: C2B implementation result
// NOTE: Enable cfg feature
// HACK: Upload v1 crate
mod client;
pub use crate::client::*;

mod services;
pub use crate::services::*;
