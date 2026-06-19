//! Token-2022 transfer-hook **allowlist** reference program.
//!
//! A minimal transfer hook that gates transfers on a destination-owner
//! allowlist. It implements the SPL transfer-hook interface and adds two admin
//! instructions to manage the allowlist.
//!
//! Reference program for the `token-2022-extensions` skill. MIT licensed.
//! See `README.md`.

#![allow(clippy::result_large_err)]

pub mod instruction;
pub mod processor;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;
