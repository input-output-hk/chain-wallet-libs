use pyo3::exceptions;
use pyo3::prelude::*;
use std::ptr::null_mut;
use wallet_core;

#[pyclass]
pub struct PyWallet {
    wallet: wallet_core::Wallet,
}

#[pyclass]
pub struct Settings {
    settings: wallet_core::Settings,
}

#[pyclass]
pub struct Conversion {
    conversion: wallet_core::Conversion,
}

#[pymethods]
impl PyWallet {
    /// retrieve a wallet from the given mnemonics and password
    ///
    /// this function will work for all yoroi, daedalus and other wallets
    /// as it will try every kind of wallet anyway
    ///
    /// You can also use this function to recover a wallet even after you have
    /// transferred all the funds to the new format (see the _convert_ function)
    ///
    /// the mnemonics should be in english
    #[staticmethod]
    pub fn recover(mnemonics: &str, password: &[u8]) -> PyResult<PyWallet> {
        wallet_core::Wallet::recover(mnemonics, password)
            .map_err(|e| exceptions::Exception::py_err(e.to_string()))
            .map(|wallet| PyWallet { wallet })
    }

    /// get the account ID bytes
    ///
    /// This ID is also the account public key, it can be used to retrieve the
    /// account state (the value, transaction counter etc...).
    pub fn id(&self) -> Vec<u8> {
        self.wallet.id().as_ref().to_vec()
    }

    pub fn convert(&mut self, settings: &Settings) -> PyResult<Conversion> {
        Ok(Conversion {
            conversion: self.wallet.convert(settings.settings.clone()),
        })
    }

    pub fn retrieve_funds(&mut self, block0: &[u8]) -> Result<Settings, JsValue> {
        self.0
            .retrieve_funds(block0)
            .map_err(|e| JsValue::from(e.to_string()))
            .map(Settings)
    }
}

#[pymodule]
fn pyjormungandrwallet(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyWallet>()?;
    Ok(())
}
