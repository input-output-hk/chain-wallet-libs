use wallet_core::c::*;
use pyo3::prelude::*;
use pyo3::exceptions;
use std::ptr::null_mut;

#[pymodule]
fn pyjormungandrwallet(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "wallet_recover")]
    fn py_wallet_recover(_py: Python, mnemonics: &str, password: &[u8], password_length: usize) -> PyResult<()> {
        let mut wallet_ptr : WalletPtr = null_mut();
        let res = unsafe { wallet_recover(mnemonics, password.as_ptr(), password_length, &mut wallet_ptr) };
        if res.is_err() {
            return PyResult::Err(exceptions::NotImplementedError::py_err("No Implemented"));
        }
        wallet_delete_wallet(wallet_ptr);
        PyResult::Ok(())
    }
    Ok(())
}
