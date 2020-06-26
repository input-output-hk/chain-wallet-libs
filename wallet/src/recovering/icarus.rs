use crate::{
    transaction::{self, Dump, WitnessBuilder},
    AccountId,
};
use chain_addr::Kind;
use chain_crypto::PublicKey;
use chain_impl_mockchain::{
    fragment::{Fragment, FragmentId},
    legacy::OldAddress,
    transaction::{Input, Transaction, UtxoPointer},
    value::Value,
};
use chain_path_derivation::{
    bip44::{self, Bip44},
    DerivationPath, HardDerivation, SoftDerivation, SoftDerivationRange,
};
use ed25519_bip32::{XPrv, XPub};
use hdkeygen::bip44::{Account, Address, SSKey, Wallet};
use std::collections::HashMap;

const CHANGE_EXTERNAL: SoftDerivation = DerivationPath::<Bip44<bip44::Account>>::EXTERNAL;
const CHANGE_INTERNAL: SoftDerivation = DerivationPath::<Bip44<bip44::Account>>::INTERNAL;
const DEFAULT_GAG_LIMIT: u32 = 20;

use super::RecoveryError;
use std::collections::hash_map::Entry;

pub struct RecoveringIcarus {
    wallet: Wallet,
    accounts: Vec<RecoveringAccount>,
    value_total: Value,
    utxos: HashMap<UtxoPointer, (bool, Address<XPrv>)>,
}

#[derive(Clone)]
pub struct StakeAccount {
    stake_key: SSKey<XPrv>,
    stake_pub_key: SSKey<XPub>,
    committed_amount: Value,
    value: Value,
    counter: u32,
}

struct RecoveringAccount {
    id: HardDerivation,
    account: Account<XPrv>,
    next_index: SoftDerivation,
    soft_derivation_range_length: u32,
    legacy_addresses: HashMap<OldAddress, Address<XPrv>>,
    addresses: HashMap<chain_addr::Address, Address<XPrv>>,

    stake: StakeAccount,
}

impl StakeAccount {
    pub fn account_id(&self) -> AccountId {
        self.stake_pub_key.account_id()
    }

    /// unsafely access the private key
    pub fn private_key(&self) -> &SSKey<XPrv> {
        &self.stake_key
    }

    pub fn set_account_state(&mut self, value: Value, counter: u32) {
        self.value = value;
        self.counter = counter;
    }

    pub fn value(&self) -> Value {
        self.value
    }

    pub fn committed_amount(&self) -> Value {
        self.committed_amount
    }

    fn current_value(&self) -> Value {
        (self.value() - self.committed_amount()).unwrap_or_else(|_| Value::zero())
    }
}

impl RecoveringAccount {
    fn new(id: HardDerivation, account: Account<XPrv>) -> Self {
        let stake_key = account.stake_key();
        let stake_pub_key = stake_key.public();

        let stake = StakeAccount {
            stake_key,
            stake_pub_key,
            committed_amount: Value::zero(),
            value: Value::zero(),
            counter: 0,
        };

        let mut ra = Self {
            id,
            account,
            next_index: SoftDerivation::min_value(),
            soft_derivation_range_length: DEFAULT_GAG_LIMIT,
            legacy_addresses: HashMap::with_capacity(128),
            addresses: HashMap::with_capacity(128),

            stake,
        };

        ra.extend_range();

        ra
    }

    #[inline]
    fn lookup_legacy(&self, old_address: &OldAddress) -> Option<&Address<XPrv>> {
        self.legacy_addresses.get(old_address)
    }

    #[inline]
    fn lookup(&self, address: &chain_addr::Address) -> Option<Option<&Address<XPrv>>> {
        match address.kind() {
            Kind::Account(account) => {
                if self.stake.stake_pub_key.key().public_key_slice() == account.as_ref() {
                    Some(None)
                } else {
                    None
                }
            }
            Kind::Single(_) => self.addresses.get(address).map(Some),
            Kind::Group(_, _) => self.addresses.get(address).map(Some),
            Kind::Multisig(_) => {
                // XXX: multisig not supported yet
                None
            }
            Kind::Script(_) => {
                // XXX: script not supported yet
                None
            }
        }
    }

    #[inline]
    fn within_last_range(&self, address: &Address<XPrv>) -> bool {
        let idx = address.path().address();
        let checked = idx.saturating_add(self.soft_derivation_range_length);

        checked >= self.next_index
    }

    fn extend_range(&mut self) {
        let start = self.next_index;
        let end = start.saturating_add(self.soft_derivation_range_length);
        self.next_index = end;

        let range = SoftDerivationRange::new(start..end);

        let internal_addresses = self.account.addresses(CHANGE_INTERNAL, range.clone());
        let external_addresses = self.account.addresses(CHANGE_EXTERNAL, range);

        let addresses = internal_addresses.chain(external_addresses);

        let stake_pub = self.stake.stake_pub_key.public_key().public_key_slice();
        let stake_pub = PublicKey::from_binary(stake_pub).unwrap();
        for address in addresses {
            let pub_key = address.key().public();
            let pub_key = pub_key.public_key_slice();
            let pub_key = PublicKey::from_binary(pub_key).unwrap();
            let xpub = address.key().as_ref().public();

            let old_address = cardano_legacy_address::ExtendedAddr::new_simple(&xpub, None);
            let old_address = old_address.to_address();
            self.legacy_addresses.insert(old_address, address.clone());

            let addr = chain_addr::Address(
                chain_addr::Discrimination::Production,
                Kind::Group(pub_key, stake_pub.clone()),
            );
            self.addresses.insert(addr, address);
        }
    }
}

impl RecoveringIcarus {
    pub(crate) fn new(wallet: Wallet) -> Self {
        let mut wallet = Self {
            wallet,
            accounts: Vec::new(),
            value_total: Value::zero(),
            utxos: HashMap::with_capacity(128),
        };

        wallet.populate_first_account();

        wallet
    }

    pub fn stake_accounts(&self) -> Vec<&StakeAccount> {
        self.accounts.iter().map(|ra| &ra.stake).collect()
    }

    pub fn stake_accounts_mut(&mut self) -> Vec<&mut StakeAccount> {
        self.accounts.iter_mut().map(|ra| &mut ra.stake).collect()
    }

    pub fn remove(&mut self, pointer: UtxoPointer) {
        if self.utxos.remove(&pointer).is_some() {
            self.value_total = self
                .value_total
                .checked_sub(pointer.value)
                .unwrap_or_else(|_| Value::zero())
        }
    }

    pub fn value_total(&self) -> Value {
        self.value_total
    }

    fn populate_first_account(&mut self) {
        let account_id = HardDerivation::min_value();
        let account = self.wallet.create_account(account_id);

        let account = RecoveringAccount::new(account_id, account);
        self.accounts.push(account);
    }

    fn populate_new_account(&mut self) {
        let last_id = self
            .accounts
            .last()
            .expect("there is always one at least")
            .id;

        if let Some(id) = last_id.checked_add(1) {
            let account = self.wallet.create_account(id);
            let account = RecoveringAccount::new(id, account);
            self.accounts.push(account);
        } else {
            // DO NOTHING... we have reached 2^31 accounts already
        }
    }

    pub(super) fn check_address(&mut self, address: chain_addr::Address) -> Option<Address<XPrv>> {
        let mut accounts = self.accounts.iter_mut();
        let mut result = None;

        while let Some(account) = accounts.next() {
            if let Some(address) = account.lookup(&address).map(|v| v.cloned()) {
                if let Some(address) = address {
                    if account.within_last_range(&address) {
                        account.extend_range();
                    }

                    result = Some(address);
                    break;
                } else {
                    todo!()
                }
            }
        }

        // this is true if we found an address in the last account
        //
        // so we always have 1 account with UTxO ahead
        if result.is_some() && accounts.next().is_none() {
            self.populate_new_account();
        }

        result
    }

    pub(super) fn check_legacy_address(&mut self, address: &OldAddress) -> Option<Address<XPrv>> {
        let mut accounts = self.accounts.iter_mut();
        let mut result = None;

        while let Some(account) = accounts.next() {
            if let Some(address) = account.lookup_legacy(address).cloned() {
                if account.within_last_range(&address) {
                    account.extend_range();
                }

                result = Some(address);
                break;
            }
        }

        // this is true if we found an address in the last account
        //
        // so we always have 1 account with UTxO ahead
        if result.is_some() && accounts.next().is_none() {
            self.populate_new_account();
        }

        result
    }

    /// convenient function to parse a block and check for owned token
    pub fn check_fragments<'a>(
        &mut self,
        fragments: impl Iterator<Item = &'a Fragment>,
    ) -> Result<(), RecoveryError> {
        for fragment in fragments {
            self.check_fragment(fragment)?
        }
        Ok(())
    }

    pub fn check_fragment(&mut self, fragment: &Fragment) -> Result<(), RecoveryError> {
        let id = fragment.hash();
        match fragment {
            Fragment::OldUtxoDeclaration(utxos) => {
                for (output_index, (address, value)) in utxos.addrs.iter().enumerate() {
                    let pointer = UtxoPointer {
                        transaction_id: id,
                        output_index: output_index as u8,
                        value: *value,
                    };

                    self.check_legacy(pointer, address)?;
                }
                Ok(())
            }
            Fragment::Initial(_) => Ok(()),
            Fragment::UpdateProposal(_) => Ok(()),
            Fragment::UpdateVote(_) => Ok(()),
            Fragment::Transaction(tx) => self.check_tx(id, tx),
            Fragment::OwnerStakeDelegation(tx) => self.check_tx(id, tx),
            Fragment::StakeDelegation(tx) => self.check_tx(id, tx),
            Fragment::PoolRegistration(tx) => self.check_tx(id, tx),
            Fragment::PoolRetirement(tx) => self.check_tx(id, tx),
            Fragment::PoolUpdate(tx) => self.check_tx(id, tx),
            Fragment::VotePlan(tx) => self.check_tx(id, tx),
            Fragment::VoteCast(tx) => self.check_tx(id, tx),
            Fragment::VoteTally(tx) => self.check_tx(id, tx),
        }
    }

    fn check_tx<E>(&mut self, id: FragmentId, tx: &Transaction<E>) -> Result<(), RecoveryError> {
        // TODO: check spending (the inputs)

        for (index, output) in tx.as_slice().outputs().iter().enumerate() {
            let pointer = UtxoPointer {
                transaction_id: id,
                output_index: index as u8,
                value: output.value,
            };

            self.check(pointer, output.address)?;
        }

        Ok(())
    }

    fn check_legacy(
        &mut self,
        pointer: UtxoPointer,
        address: &OldAddress,
    ) -> Result<(), RecoveryError> {
        if let Some(key) = self.check_legacy_address(address) {
            match self.utxos.entry(pointer) {
                Entry::Occupied(_entry) => return Err(RecoveryError::DuplicatedUtxo),
                Entry::Vacant(entry) => entry.insert((true, key)),
            };
            self.value_total = self.value_total.saturating_add(pointer.value);
        }
        Ok(())
    }

    fn check(
        &mut self,
        pointer: UtxoPointer,
        address: chain_addr::Address,
    ) -> Result<(), RecoveryError> {
        if let Some(key) = self.check_address(address) {
            match self.utxos.entry(pointer) {
                Entry::Occupied(_entry) => return Err(RecoveryError::DuplicatedUtxo),
                Entry::Vacant(entry) => entry.insert((false, key)),
            };
            self.value_total = self.value_total.saturating_add(pointer.value);
        }
        Ok(())
    }

    /// dump all the inputs
    pub fn dump_in(&self, dump: &mut Dump) {
        for (pointer, (is_legacy, key)) in self.utxos.iter() {
            let witness_builder = if *is_legacy {
                WitnessBuilder::OldUtxo {
                    xprv: key.key().as_ref().clone(),
                }
            } else {
                WitnessBuilder::Utxo {
                    xprv: key.key().as_ref().clone(),
                }
            };

            dump.push(Input::from_utxo(*pointer), witness_builder)
        }
    }
}

impl transaction::InputGenerator for StakeAccount {
    fn input_to_cover(&mut self, value: Value) -> Option<transaction::GeneratedInput> {
        if self.current_value() < value {
            None
        } else {
            let input = Input::from_account_public_key(self.account_id().into(), value);
            let witness_builder = transaction::WitnessBuilder::UtxoStakeAccount {
                xprv: self.stake_key.key().as_ref().clone(),
                spending_counter: self.counter,
            };

            self.committed_amount = self.committed_amount.saturating_add(value);
            self.counter += 1;

            Some(transaction::GeneratedInput {
                input,
                witness_builder,
            })
        }
    }
}
