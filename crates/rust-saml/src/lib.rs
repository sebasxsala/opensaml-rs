//! Thin re-export crate. All SAML logic lives in [`opensaml`].
//!
//! This crate exists only to reserve the `rust-saml` name on crates.io and
//! offer an alternate crate name. Prefer depending on `opensaml` directly.

#![forbid(unsafe_code)]

pub use opensaml::*;
