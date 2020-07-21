use pyo3::exceptions;
use pyo3::prelude::*;
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

#[pyclass]
pub struct Proposal {
    proposal: wallet_core::Proposal,
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

    /// retrieve funds from daedalus or yoroi wallet in the given block0 (or
    /// any other blocks).
    ///
    /// Execute this function then you can check who much funds you have
    /// retrieved from the given block.
    ///
    /// this function may take sometimes so it is better to only call this
    /// function if needed.
    ///
    /// also, this function should not be called twice with the same block.
    pub fn retrieve_funds(&mut self, block0: &[u8]) -> PyResult<Settings> {
        self.wallet
            .retrieve_funds(block0)
            .map_err(|e| exceptions::Exception::py_err(e.to_string()))
            .map(|settings| Settings { settings })
    }

    /// get the total value in the wallet
    ///
    /// make sure to call `retrieve_funds` prior to calling this function
    /// otherwise you will always have `0`
    pub fn total_value(&self) -> u64 {
        self.wallet.total_value().0
    }

    /// update the wallet account state
    ///
    /// this is the value retrieved from any jormungandr endpoint that allows to query
    /// for the account state. It gives the value associated to the account as well as
    /// the counter.
    ///
    /// It is important to be sure to have an updated wallet state before doing any
    /// transactions otherwise future transactions may fail to be accepted by any
    /// nodes of the blockchain because of invalid signature state.
    ///
    pub fn set_state(&mut self, value: u64, counter: u32) {
        self.wallet.set_state(wallet_core::Value(value), counter);
    }

    /// Cast a vote
    ///
    /// This function outputs a fragment containing a voting transaction.
    ///
    /// # Parameters
    ///
    /// * `settings` - ledger settings.
    /// * `proposal` - proposal information including the range of values
    ///   allowed in `choice`.
    /// * `choice` - the option to vote for.
    ///
    /// # Errors
    ///
    /// The error is returned when `choice` does not fall withing the range of
    /// available choices specified in `proposal`.
    pub fn vote(
        &mut self,
        settings: &Settings,
        proposal: &Proposal,
        choice: u8,
    ) -> PyResult<Vec<u8>> {
        self.wallet
            .vote(
                settings.settings.clone(),
                &proposal.proposal,
                wallet_core::Choice::new(choice),
            )
            .map_err(|e| exceptions::Exception::py_err(e.to_string()))
            .map(|res| res.as_ref().to_vec())
    }
}

#[pymethods]
impl Conversion {
    /// retrieve the total number of ignored UTxOs in the conversion
    /// transactions
    ///
    /// this is the number of utxos that are not included in the conversions
    /// because it is more expensive to use them than to ignore them. This is
    /// called dust.
    pub fn num_ignored(&self) -> usize {
        self.conversion.ignored().len()
    }

    /// retrieve the total value lost in dust utxos
    ///
    /// this is the total value of all the ignored UTxOs because
    /// they are too expensive to use in any transactions.
    ///
    /// I.e. their individual fee to add as an input is higher
    /// than the value they individually holds
    pub fn total_value_ignored(&self) -> u64 {
        self.conversion
            .ignored()
            .iter()
            .map(|i| *i.value().as_ref())
            .sum::<u64>()
    }

    /// the number of transactions built for the conversion
    pub fn transactions_len(&self) -> usize {
        self.conversion.transactions().len()
    }

    pub fn transactions_get(&self, index: usize) -> Option<Vec<u8>> {
        self.conversion
            .transactions()
            .get(index)
            .map(|t| t.to_owned())
    }
}

#[pymodule]
fn pyjormungandrwallet(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyWallet>()?;
    Ok(())
}
