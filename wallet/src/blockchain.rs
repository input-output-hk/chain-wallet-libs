use chain_addr::Discrimination;
use chain_impl_mockchain::{
    certificate::CertificateSlice,
    chaintypes::HeaderId,
    fee::FeeAlgorithm as _,
    fee::LinearFee,
    fragment::Fragment,
    ledger::{Error, Ledger, LedgerParameters, LedgerStaticParameters},
    transaction::Input,
    value::Value,
    vote::CommitteeId,
};
use chain_time::TimeEra;
use std::time::{Duration, SystemTime};

#[derive(Clone)]
pub struct Settings {
    pub static_parameters: LedgerStaticParameters,
    pub time_era: TimeEra,
    pub fees: LinearFee,
    pub committees: Vec<CommitteeId>,
}

impl Settings {
    pub fn new<'a, I>(block0_initial_hash: HeaderId, contents: I) -> Result<Self, Error>
    where
        I: Iterator<Item = &'a Fragment>,
    {
        let (static_parameters, fees, committees, time_era) = {
            let ledger = Ledger::new(block0_initial_hash, contents)?;

            let static_parameters = ledger.get_static_parameters().clone();
            let time_era = ledger.era().clone();

            let LedgerParameters {
                fees, committees, ..
            } = ledger.get_ledger_parameters();

            (static_parameters, fees, committees, time_era)
        };

        Ok(Self {
            static_parameters,
            fees,
            committees: std::sync::Arc::try_unwrap(committees).unwrap(),
            time_era,
        })
    }

    /// convenient function to check if a given input
    /// is covering at least its own input fees for a given transaction
    pub fn is_input_worth(&self, input: &Input) -> bool {
        let value = input.value();
        let minimal_value = self.fees.fees_for_inputs_outputs(1, 0);

        value > minimal_value
    }

    // extract the block0 date-time
    // seconds in unix time (seconds elapsed since 1-Jan-1970)
    pub fn start_date_time(&self) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_secs(self.static_parameters.block0_start_time.0)
    }

    pub fn discrimination(&self) -> Discrimination {
        self.static_parameters.discrimination
    }

    pub fn time_era(&self) -> &TimeEra {
        &self.time_era
    }

    pub fn committee(&self) -> &[CommitteeId] {
        self.committees.as_slice()
    }

    pub fn calculate_fees(&self, cert: Option<CertificateSlice>, input: u8, output: u8) -> Value {
        self.fees.calculate(cert, input, output)
    }

    pub fn block0_hash(&self) -> HeaderId {
        self.static_parameters.block0_initial_hash
    }
}
