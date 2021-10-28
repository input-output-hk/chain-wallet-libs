use chain_crypto::bech32::Bech32;
use chain_impl_mockchain::certificate::VotePlanId;
use chain_impl_mockchain::config;
use chain_impl_mockchain::fee;
use chain_vote::ElectionPublicKey;
use std::num::NonZeroU64;
use std::sync::Arc;
use std::sync::Mutex;
use wallet_core::Error as CoreError;
use wallet_core::Options;
use wallet_core::Settings as InnerSettings;
use wallet_core::Wallet as InnerWallet;

uniffi_macros::include_scaffolding!("lib");

#[derive(Debug, thiserror::Error)]
pub enum WalletError {
    #[error("malformed encryption key")]
    InvalidEncryptionKey,
    #[error("malformed voteplan id")]
    MalformedVotePlanId,
    #[error("malformed block0 hash")]
    MalformedBlock0Hash,
    #[error("core error {0}")]
    CoreError(#[from] CoreError),
}

pub struct Wallet(Mutex<InnerWallet>);

// this is technically unsound, but from here onwards (after taking the Mutex) we don't spawn a
// different thread, so we can't Send anything.
// anyway I think the only thing non Send is the utxo-store, because it uses Rc
unsafe impl Send for Wallet {}
unsafe impl Sync for Wallet {}

pub struct Settings(Mutex<InnerSettings>);

pub struct Proposal {
    pub vote_plan_id: Vec<u8>,
    pub index: u8,
    pub options: u8,
    pub payload_type: PayloadTypeConfig,
}

pub struct SettingsInit {
    pub fees: LinearFee,
    pub discrimination: Discrimination,
    pub block0_hash: Vec<u8>,
    pub block0_date: u64,
    pub slot_duration: u8,
    pub time_era: TimeEra,
    pub transaction_max_expiry_epochs: u8,
}

pub struct LinearFee {
    pub constant: u64,
    pub coefficient: u64,
    pub certificate: u64,
    pub per_certificate_fees: PerCertificateFee,
    pub per_vote_certificate_fees: PerVoteCertificateFee,
}

pub struct PerCertificateFee {
    pub certificate_pool_registration: u64,
    pub certificate_stake_delegation: u64,
    pub certificate_owner_stake_delegation: u64,
}

pub struct PerVoteCertificateFee {
    pub certificate_vote_plan: u64,
    pub certificate_vote_cast: u64,
}

pub enum Discrimination {
    Production,
    Test,
}

pub struct TimeEra {
    pub epoch_start: u32,
    pub slot_start: u64,
    pub slots_per_epoch: u32,
}

pub enum PayloadTypeConfig {
    Public,
    Private { encryption_key: String },
}

pub struct BlockDate {
    epoch: u32,
    slot: u32,
}

impl Wallet {
    pub fn new(account_key: Vec<u8>) -> Result<Self, WalletError> {
        let inner = InnerWallet::recover_free_keys(account_key.as_ref(), &[])
            .map_err(WalletError::CoreError)?;

        Ok(Self(Mutex::new(inner)))
    }

    pub fn set_state(&self, value: u64, counter: u32) {
        let mut guard = self.0.lock().unwrap();

        guard.set_state(wallet_core::Value(value), counter);
    }

    pub fn vote(
        &self,
        settings: Arc<Settings>,
        proposal: Proposal,
        choice: u8,
        valid_until: BlockDate,
    ) -> Result<Vec<u8>, WalletError> {
        let settings = settings.0.lock().unwrap();
        let mut wallet = self.0.lock().unwrap();

        wallet
            .vote(
                settings.clone(),
                &proposal.try_into()?,
                wallet_core::Choice::new(choice),
                &valid_until.into(),
            )
            .map(|bytes| bytes.into_vec())
            .map_err(WalletError::from)
    }
}

impl Settings {
    pub fn new(settings_init: SettingsInit) -> Result<Self, WalletError> {
        let SettingsInit {
            fees,
            discrimination,
            block0_hash,
            block0_date,
            slot_duration,
            time_era,
            transaction_max_expiry_epochs,
        } = settings_init;

        let discrimination = match discrimination {
            Discrimination::Production => chain_addr::Discrimination::Production,
            Discrimination::Test => chain_addr::Discrimination::Test,
        };

        let linear_fee = fee::LinearFee {
            constant: fees.constant,
            coefficient: fees.coefficient,
            certificate: fees.certificate,
            per_certificate_fees: fee::PerCertificateFee {
                certificate_pool_registration: NonZeroU64::new(
                    fees.per_certificate_fees.certificate_pool_registration,
                ),
                certificate_stake_delegation: NonZeroU64::new(
                    fees.per_certificate_fees.certificate_stake_delegation,
                ),
                certificate_owner_stake_delegation: NonZeroU64::new(
                    fees.per_certificate_fees.certificate_owner_stake_delegation,
                ),
            },
            per_vote_certificate_fees: fee::PerVoteCertificateFee {
                certificate_vote_plan: NonZeroU64::new(
                    fees.per_vote_certificate_fees.certificate_vote_plan,
                ),
                certificate_vote_cast: NonZeroU64::new(
                    fees.per_vote_certificate_fees.certificate_vote_cast,
                ),
            },
        };

        let block0_hash: [u8; 32] = block0_hash
            .try_into()
            .map_err(|_| WalletError::MalformedBlock0Hash)?;

        Ok(Self(Mutex::new(InnerSettings {
            fees: linear_fee,
            discrimination,
            block0_initial_hash: block0_hash.into(),
            block0_date: config::Block0Date(block0_date),
            slot_duration,
            time_era: time_era.into(),
            transaction_max_expiry_epochs,
        })))
    }

    pub fn settings_raw(&self) -> SettingsInit {
        let guard = self.0.lock().unwrap();

        SettingsInit {
            fees: LinearFee {
                constant: guard.fees.constant,
                coefficient: guard.fees.coefficient,
                certificate: guard.fees.certificate,
                per_certificate_fees: PerCertificateFee {
                    certificate_pool_registration: guard
                        .fees
                        .per_certificate_fees
                        .certificate_pool_registration
                        .map(NonZeroU64::get)
                        .unwrap_or(0),
                    certificate_stake_delegation: guard
                        .fees
                        .per_certificate_fees
                        .certificate_stake_delegation
                        .map(NonZeroU64::get)
                        .unwrap_or(0),
                    certificate_owner_stake_delegation: guard
                        .fees
                        .per_certificate_fees
                        .certificate_owner_stake_delegation
                        .map(NonZeroU64::get)
                        .unwrap_or(0),
                },
                per_vote_certificate_fees: PerVoteCertificateFee {
                    certificate_vote_plan: guard
                        .fees
                        .per_vote_certificate_fees
                        .certificate_vote_plan
                        .map(NonZeroU64::get)
                        .unwrap_or(0),
                    certificate_vote_cast: guard
                        .fees
                        .per_vote_certificate_fees
                        .certificate_vote_cast
                        .map(NonZeroU64::get)
                        .unwrap_or(0),
                },
            },
            discrimination: match guard.discrimination {
                chain_addr::Discrimination::Production => Discrimination::Production,
                chain_addr::Discrimination::Test => Discrimination::Test,
            },
            block0_hash: guard
                .block0_initial_hash
                .as_bytes()
                .iter()
                .cloned()
                .collect(),
            block0_date: guard.block0_date.0,
            slot_duration: guard.slot_duration,
            time_era: TimeEra {
                // TODO: expose these things in chain_libs, they are not going to be anything else
                // than 0 for now, but just in case
                epoch_start: 0,
                slot_start: 0,
                slots_per_epoch: guard.time_era.slots_per_epoch(),
            },
            transaction_max_expiry_epochs: guard.transaction_max_expiry_epochs,
        }
    }
}

impl From<TimeEra> for chain_time::TimeEra {
    fn from(te: TimeEra) -> Self {
        chain_time::TimeEra::new(
            te.slot_start.into(),
            chain_time::Epoch(te.epoch_start),
            te.slots_per_epoch,
        )
    }
}

impl TryFrom<Proposal> for wallet_core::Proposal {
    type Error = WalletError;

    fn try_from(p: Proposal) -> Result<Self, Self::Error> {
        Ok(wallet_core::Proposal::new(
            VotePlanId::try_from(p.vote_plan_id.as_ref())
                .map_err(|_| WalletError::MalformedVotePlanId)?,
            p.index,
            Options::new_length(p.options).unwrap(),
            match p.payload_type {
                PayloadTypeConfig::Public => wallet_core::PayloadTypeConfig::Public,
                PayloadTypeConfig::Private { encryption_key } => {
                    let encryption_key = ElectionPublicKey::try_from_bech32_str(&encryption_key)
                        .map_err(|_| WalletError::InvalidEncryptionKey)?;
                    wallet_core::PayloadTypeConfig::Private(encryption_key)
                }
            },
        ))
    }
}

impl From<BlockDate> for chain_impl_mockchain::block::BlockDate {
    fn from(d: BlockDate) -> Self {
        chain_impl_mockchain::block::BlockDate {
            epoch: d.epoch,
            slot_id: d.slot,
        }
    }
}
