use super::commons::out_return_buffer;
use crate::{Error, Result};

/// decrypt payload of the wallet transfer protocol
///
/// Parameters
///
/// password: byte buffer with the encryption password
/// password_length: length of the password buffer
/// ciphertext: byte buffer with the encryption password
/// ciphertext_length: length of the password buffer
/// plaintext_out: used to return a pointer to a byte buffer with the decrypted text
/// plaintext_out_length: used to return the length of decrypted text
///
/// The returned buffer is in the heap, so make sure to call the delete_buffer function
///
/// # Safety
///
/// This function dereference raw pointers. Even though the function checks if
/// the pointers are null. Mind not to put random values in or you may see
/// unexpected behaviors.
pub unsafe fn symmetric_cipher_decrypt(
    password: *const u8,
    password_length: usize,
    ciphertext: *const u8,
    ciphertext_length: usize,
    plaintext_out: *mut *const u8,
    plaintext_out_length: *mut usize,
) -> Result {
    let password = non_null!(password);
    let ciphertext = non_null!(ciphertext);

    let password = std::slice::from_raw_parts(password, password_length);
    let ciphertext = std::slice::from_raw_parts(ciphertext, ciphertext_length);

    match symmetric_cipher::decrypt(password, ciphertext) {
        Ok(plaintext) => {
            let out = non_null_mut!(plaintext_out);
            let out_len = non_null_mut!(plaintext_out_length);

            out_return_buffer(plaintext, out, out_len);

            Result::success()
        }
        Err(err) => Error::symmetric_cipher_error(err).into(),
    }
}
