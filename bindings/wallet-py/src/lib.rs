use pyo3::exceptions;
use pyo3::prelude::*;
use std::ptr::null_mut;
use wallet_core::c::*;

#[pyclass]
struct PyWallet {
    wallet_ptr: Option<u64>,
}

#[pyclass]
struct PyWalletSettings {
    settings_ptr: Option<u64>,
}

#[pymethods]
impl PyWallet {
    fn delete(&mut self) -> PyResult<()> {
        if let Some(wallet_ptr) = self.wallet_ptr {
            let wallet_ptr: WalletPtr = wallet_ptr as WalletPtr;
            // double check here, supposedly it could never be null.
            if !wallet_ptr.is_null() {
                wallet_delete_wallet(wallet_ptr);
            }
            self.wallet_ptr = None;
        }
        Ok(())
    }

    fn total_value(&self) -> PyResult<u64> {
        if let Some(wallet_ptr) = self.wallet_ptr {
            let mut value: u64 = 0;
            if let Some(e) =
                unsafe { wallet_total_value(wallet_ptr as WalletPtr, &mut value) }.error()
            {
                return PyResult::Err(exceptions::Exception::py_err(e.to_string()));
            }
            Ok(value)
        } else {
            PyResult::Err(exceptions::ValueError::py_err(
                "Wallet object do not references any wallet",
            ))
        }
    }

    fn initial_funds(&self, block0: &[u8]) -> PyResult<PyWalletSettings> {
        if let Some(wallet_ptr) = self.wallet_ptr {
            let mut settings: SettingsPtr = null_mut();
            let settings_ptr: *mut SettingsPtr = &mut settings;
            if let Some(e) = unsafe {
                wallet_retrieve_funds(
                    wallet_ptr as WalletPtr,
                    block0.as_ptr() as *const u8,
                    block0.len(),
                    settings_ptr,
                )
            }
            .error()
            {
                return PyResult::Err(exceptions::Exception::py_err(e.to_string()));
            }
            Ok(PyWalletSettings {
                settings_ptr: Some(settings_ptr as u64),
            })
        } else {
            PyResult::Err(exceptions::ValueError::py_err(
                "Wallet object do not references any wallet",
            ))
        }
    }
}

#[pymodule]
fn pyjormungandrwallet(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyWallet>()?;
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
            wallet_ptr: Some(wallet_ptr as u64),
        })
    }
    Ok(())
}
