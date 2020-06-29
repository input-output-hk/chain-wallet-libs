use chain_addr::Discrimination;
use chain_impl_mockchain::{
    certificate::CertificateSlice,
    chaintypes::HeaderId,
    fee::FeeAlgorithm as _,
    fee::{LinearFee, PerCertificateFee, PerVoteCertificateFee},
    fragment::Fragment,
    ledger::{Error, Ledger, LedgerParameters, LedgerStaticParameters},
    transaction::Input,
    value::Value,
    vote::CommitteeId,
};
use chain_ser::{
    deser::{Deserialize, Serialize},
    packer::Codec,
};
use chain_time::{
    era::{pack_time_era, unpack_time_era},
    TimeEra,
};
use std::convert::TryInto;
use std::time::{Duration, SystemTime};

#[derive(Clone)]
#[cfg_attr(test, derive(PartialEq, Eq, Debug))]
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

        pack_ledger_static_parameters(&self.static_parameters, &mut codec)?;
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

    let PerCertificateFee {
        certificate_pool_registration,
        certificate_stake_delegation,
        certificate_owner_stake_delegation,
    } = linear_fee.per_certificate_fees;

    pack_non_zero_u64(certificate_pool_registration, codec)?;
    pack_non_zero_u64(certificate_stake_delegation, codec)?;
    pack_non_zero_u64(certificate_owner_stake_delegation, codec)?;

    let PerVoteCertificateFee {
        certificate_vote_cast,
        certificate_vote_plan,
    } = linear_fee.per_vote_certificate_fees;

    pack_non_zero_u64(certificate_vote_plan, codec)?;
    pack_non_zero_u64(certificate_vote_cast, codec)?;

    Ok(())
}

fn unpack_linear_fee<R: std::io::BufRead>(
    codec: &mut Codec<R>,
) -> Result<LinearFee, std::io::Error> {
    let constant = codec.get_u64()?;
    let coefficient = codec.get_u64()?;
    let certificate = codec.get_u64()?;

    let certificate_pool_registration = std::num::NonZeroU64::new(codec.get_u64()?);
    let certificate_stake_delegation = std::num::NonZeroU64::new(codec.get_u64()?);
    let certificate_owner_stake_delegation = std::num::NonZeroU64::new(codec.get_u64()?);

    let certificate_vote_plan = std::num::NonZeroU64::new(codec.get_u64()?);
    let certificate_vote_cast = std::num::NonZeroU64::new(codec.get_u64()?);

    Ok(LinearFee {
        constant,
        coefficient,
        certificate,
        per_certificate_fees: PerCertificateFee {
            certificate_pool_registration,
            certificate_stake_delegation,
            certificate_owner_stake_delegation,
        },
        per_vote_certificate_fees: PerVoteCertificateFee {
            certificate_vote_cast,
            certificate_vote_plan,
        },
    })
}

fn pack_ledger_static_parameters<W: std::io::Write>(
    ledger_static_parameters: &LedgerStaticParameters,
    mut codec: &mut Codec<W>,
) -> Result<(), std::io::Error> {
    ledger_static_parameters
        .block0_initial_hash
        .serialize(&mut codec)?;

    codec.put_u64(ledger_static_parameters.block0_start_time.0)?;

    let discrimination = match ledger_static_parameters.discrimination {
        Discrimination::Production => 0,
        Discrimination::Test => 1,
    };

    codec.put_u8(discrimination)?;

    codec.put_u32(ledger_static_parameters.kes_update_speed)?;
    Ok(())
}

fn unpack_ledger_static_parameters<R: std::io::BufRead>(
    mut codec: &mut Codec<R>,
) -> Result<LedgerStaticParameters, std::io::Error> {
    let block0_initial_hash = HeaderId::deserialize(&mut codec)?;
    let block0_start_time = chain_impl_mockchain::config::Block0Date(codec.get_u64()?);

    let discrimination = match codec.get_u8()? {
        0 => Discrimination::Production,
        1 => Discrimination::Test,
        _ => unreachable!("change this for error"),
    };

    let kes_update_speed = codec.get_u32()?;
    Ok(LedgerStaticParameters {
        block0_initial_hash,
        block0_start_time,
        discrimination,
        kes_update_speed,
    })
}

fn pack_non_zero_u64<W: std::io::Write>(
    n: Option<std::num::NonZeroU64>,
    codec: &mut Codec<W>,
) -> Result<(), std::io::Error> {
    let n = n.map(|v| v.get()).unwrap_or(0);
    codec.put_u64(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_ser_de() {
        use chain_impl_mockchain::block::Block;

        const BLOCK0: &[u8] = include_bytes!("../../test-vectors/block0");
        let block = Block::deserialize(BLOCK0).unwrap();
        let hash = block.header.id();
        let settings = Settings::new(hash, block.contents.iter()).unwrap();

        let raw = settings.serialize().unwrap();

        let after = Settings::deserialize(&raw).unwrap();

        assert_eq!(settings, after);
    }
}
