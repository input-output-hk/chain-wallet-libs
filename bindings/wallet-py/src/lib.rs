use pyo3::exceptions;
use pyo3::prelude::*;
use std::ptr::null_mut;
use wallet_core::c::*;

#[pyclass]
struct PyWallet {
    wallet_ptr: u64,
}

#[pymodule]
fn pyjormungandrwallet(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "wallet_recover")]
    fn py_wallet_recover(
        _py: Python,
        mnemonics: &str,
        password: &[u8],
        password_length: usize,
    ) -> PyResult<PyWallet> {
        let mut wallet_ptr: WalletPtr = null_mut();
        let res = unsafe {
            wallet_recover(
                mnemonics,
                password.as_ptr(),
                password_length,
                &mut wallet_ptr,
            )
        };
        if let Some(e) = res.error() {
            return PyResult::Err(exceptions::Exception::py_err(e.to_string()));
        }
        PyResult::Ok(PyWallet {
            wallet_ptr: wallet_ptr as u64,
        })
    }

    #[pyfn(m, "wallet_delete")]
    fn py_wallet_delete(_py: Python, wallet: &PyWallet) -> PyResult<()> {
        let wallet_ptr: WalletPtr = wallet.wallet_ptr as WalletPtr;
        if !wallet_ptr.is_null() {
            wallet_delete_wallet(wallet_ptr);
        }
        Ok(())
    }
    Ok(())
}
