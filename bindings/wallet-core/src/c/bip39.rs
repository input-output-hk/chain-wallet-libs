use super::commons::out_return_buffer;
use crate::{Error, Result};

///generate entropy from the given random generator
///
/// # Safety
/// This function dereferences raw pointers, ensure those point to valid values
pub unsafe fn bip39_entropy_from_random(
    words: u8,
    gen: extern "C" fn() -> u8,
    entropy_ptr: *mut *const u8,
    entropy_size: *mut usize,
) -> Result {
    let words = match bip39::Type::from_word_count(words as usize) {
        Ok(v) => v,
        Err(e) => return Result::from(Error::bip39_error(e)),
    };

    #[allow(clippy::redundant_closure)]
    let entropy = bip39::Entropy::generate(words, || gen()).to_vec();

    out_return_buffer(
        entropy.into_boxed_slice(),
        non_null_mut!(entropy_ptr),
        non_null_mut!(entropy_size),
    );

    Result::success()
}

/// get a mnemonic UTF-8 (NFKD) string from the given entropy
///
/// # Safety
/// This function dereferences raw pointers, ensure those point to valid values
pub unsafe fn bip39_english_mnemonics_from_entropy(
    entropy_ptr: *const u8,
    entropy_size: usize,
    out_mnemonic_string: *mut *const u8,
    out_mnemonic_string_size: *mut usize,
) -> Result {
    let entropy_slice: &[u8] =
        std::slice::from_raw_parts(non_null!(entropy_ptr) as *const u8, entropy_size);

    let entropy = match bip39::Entropy::from_slice(entropy_slice).map_err(Error::bip39_error) {
        Ok(entropy) => entropy,
        Err(e) => return Result::from(e),
    };

    let mnemonics = entropy.to_mnemonics();
    let string = mnemonics.to_string(&bip39::dictionary::ENGLISH);

    let boxed = string.into_boxed_slice();

    out_return_buffer(
        boxed,
        non_null_mut!(out_mnemonic_string),
        non_null_mut!(out_mnemonic_string_size),
    );

    Result::success()
}
