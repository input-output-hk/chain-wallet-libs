use chain_addr::Discrimination;
use chain_impl_mockchain::{
    certificate::CertificateSlice,
    chaintypes::HeaderId,
    fee::FeeAlgorithm as _,
    fee::{LinearFee, PerCertificateFee, PerVoteCertificateFee},
    fragment::Fragment,
    ledger::recovery::{
        pack_ledger_static_parameters, pack_time_era, unpack_ledger_static_parameters,
        unpack_time_era,
    },
    ledger::{Error, Ledger, LedgerParameters, LedgerStaticParameters},
    transaction::Input,
    value::Value,
    vote::CommitteeId,
};
use chain_ser::packer::Codec;
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

    pub fn serialize(&self) -> Result<Box<[u8]>, std::io::Error> {
        let buf = vec![];
        let mut codec = Codec::new(buf);

        pack_ledger_static_parameters(&self.static_parameters, &mut codec)
            .expect("failed to serialize static parameters");

        pack_time_era(&self.time_era, &mut codec)?;

        pack_linear_fee(&self.fees, &mut codec)?;

        codec.put_u64(self.committees.len() as u64)?;
        for committee in &self.committees {
            codec.put_bytes(committee.as_ref())?;
        }

        Ok(codec.into_inner().into_boxed_slice())
    }

    pub fn deserialize(raw: &[u8]) -> Result<Self, std::io::Error> {
        let reader = std::io::BufReader::new(raw);
        let mut codec = Codec::new(reader);
        let static_parameters = unpack_ledger_static_parameters(&mut codec)?;
        let time_era = unpack_time_era(&mut codec)?;
        let fees = unpack_linear_fee(&mut codec)?;

        let committees_len = codec.get_u64()?;

        use std::convert::TryInto;
        let committees: Result<Vec<CommitteeId>, std::io::Error> = (0..committees_len)
            .map(|_| {
                let raw = codec.get_bytes(CommitteeId::COMMITTEE_ID_SIZE)?;
                let arr: [u8; CommitteeId::COMMITTEE_ID_SIZE] = raw[..].try_into().unwrap();

                Ok(arr.into())
            })
            .collect();

        Ok(Settings {
            static_parameters,
            time_era,
            fees,
            committees: committees?,
        })
    }
}

fn pack_linear_fee<W: std::io::Write>(
    linear_fee: &LinearFee,
    codec: &mut Codec<W>,
) -> Result<(), std::io::Error> {
    codec.put_u64(linear_fee.constant)?;
    codec.put_u64(linear_fee.coefficient)?;
    codec.put_u64(linear_fee.certificate)?;
    pack_per_certificate_fee(&linear_fee.per_certificate_fees, codec)?;
    pack_per_vote_certificate_fee(&linear_fee.per_vote_certificate_fees, codec)?;
    Ok(())
}

fn unpack_linear_fee<R: std::io::BufRead>(
    codec: &mut Codec<R>,
) -> Result<LinearFee, std::io::Error> {
    let constant = codec.get_u64()?;
    let coefficient = codec.get_u64()?;
    let certificate = codec.get_u64()?;
    let per_certificate_fees = unpack_per_certificate_fee(codec)?;
    let per_vote_certificate_fees = unpack_per_vote_certificate_fee(codec)?;
    Ok(LinearFee {
        constant,
        coefficient,
        certificate,
        per_certificate_fees,
        per_vote_certificate_fees,
    })
}

fn pack_per_certificate_fee<W: std::io::Write>(
    per_certificate_fee: &PerCertificateFee,
    codec: &mut Codec<W>,
) -> Result<(), std::io::Error> {
    codec.put_u64(
        per_certificate_fee
            .certificate_pool_registration
            .map(|v| v.get())
            .unwrap_or(0),
    )?;
    codec.put_u64(
        per_certificate_fee
            .certificate_stake_delegation
            .map(|v| v.get())
            .unwrap_or(0),
    )?;
    codec.put_u64(
        per_certificate_fee
            .certificate_owner_stake_delegation
            .map(|v| v.get())
            .unwrap_or(0),
    )?;
    Ok(())
}

fn pack_per_vote_certificate_fee<W: std::io::Write>(
    per_vote_certificate_fee: &PerVoteCertificateFee,
    codec: &mut Codec<W>,
) -> Result<(), std::io::Error> {
    codec.put_u64(
        per_vote_certificate_fee
            .certificate_vote_plan
            .map(|v| v.get())
            .unwrap_or(0),
    )?;
    codec.put_u64(
        per_vote_certificate_fee
            .certificate_vote_cast
            .map(|v| v.get())
            .unwrap_or(0),
    )?;
    Ok(())
}

fn unpack_per_certificate_fee<R: std::io::BufRead>(
    codec: &mut Codec<R>,
) -> Result<PerCertificateFee, std::io::Error> {
    let certificate_pool_registration = std::num::NonZeroU64::new(codec.get_u64()?);
    let certificate_stake_delegation = std::num::NonZeroU64::new(codec.get_u64()?);
    let certificate_owner_stake_delegation = std::num::NonZeroU64::new(codec.get_u64()?);

    Ok(PerCertificateFee {
        certificate_pool_registration,
        certificate_stake_delegation,
        certificate_owner_stake_delegation,
    })
}

fn unpack_per_vote_certificate_fee<R: std::io::BufRead>(
    codec: &mut Codec<R>,
) -> Result<PerVoteCertificateFee, std::io::Error> {
    let certificate_vote_plan = std::num::NonZeroU64::new(codec.get_u64()?);
    let certificate_vote_cast = std::num::NonZeroU64::new(codec.get_u64()?);

    Ok(PerVoteCertificateFee {
        certificate_vote_plan,
        certificate_vote_cast,
    })
}
