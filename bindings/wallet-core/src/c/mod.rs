//! This module expose handy C compatible functions to reuse in the different
//! C style bindings that we have (wallet-c, wallet-jni...)
//!
#[macro_use]
mod commons;

pub mod bip39;
pub mod pending_transactions;
pub mod symmetric_cipher;
pub mod wallet;

pub use self::symmetric_cipher::symmetric_cipher_decrypt;
pub use self::wallet::*;
use crate::{Conversion, Error, Proposal, Wallet};
pub use ::wallet::Settings;
use thiserror::Error;

pub type WalletPtr = *mut Wallet;
pub type SettingsPtr = *mut Settings;
pub type ConversionPtr = *mut Conversion;
pub type ProposalPtr = *mut Proposal;
pub type ErrorPtr = *mut Error;

#[derive(Debug, Error)]
#[error("access out of bound")]
struct OutOfBound;

/// Delete a binary buffer that was returned by this library alongside with its
/// length.
///
/// # Safety
///
/// This function dereference raw pointers. Even though
/// the function checks if the pointers are null. Mind not to put random values
/// in or you may see unexpected behaviors
pub unsafe fn delete_buffer(ptr: *mut u8, length: usize) {
    if !ptr.is_null() {
        let data = std::slice::from_raw_parts_mut(ptr, length);
        let data = Box::from_raw(data as *mut [u8]);
        std::mem::drop(data);
    }
}

/// Delete a binary buffer that was returned by this library alongside with its
/// length. This function also clears the buffer contents
///
/// # Safety
///
/// This function dereference raw pointers. Even though
/// the function checks if the pointers are null. Mind not to put random values
/// in or you may see unexpected behaviors
pub unsafe fn secure_delete_buffer(ptr: *mut u8, length: usize) {
    if !ptr.is_null() {
        let data = std::slice::from_raw_parts_mut(ptr, length);
        let mut data = Box::from_raw(data as *mut [u8]);
        cryptoxide::util::secure_memset(&mut data, 0);
        std::mem::drop(data);
    }
}
