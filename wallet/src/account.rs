use super::transaction::AccountWitnessBuilder;
use crate::states::States;
use crate::{
    scheme::{on_tx_input_and_witnesses, on_tx_output},
    WalletState,
};
use chain_crypto::{Ed25519, Ed25519Extended, PublicKey, SecretKey};
use chain_impl_mockchain::{
    account::SpendingCounter,
    fragment::{Fragment, FragmentId},
    transaction::{Input, InputEnum},
    value::Value,
};
pub use hdkeygen::account::AccountId;
use hdkeygen::account::{Account, Seed};
use thiserror::Error;

impl WalletState for State {
    fn value(&self) -> Value {
        self.value
    }
}

impl crate::Wallet for Wallet {
    type State = State;

    fn get_states(&self) -> &States<FragmentId, Self::State> {
        &self.state
    }

    fn get_states_mut(&mut self) -> &mut States<FragmentId, Self::State> {
        &mut self.state
    }

    fn new_state(&self, state: &Self::State, fragment: &Fragment) -> (bool, Self::State) {
        let mut new_value = state.value;

        let mut increment_counter = false;
        let mut at_least_one_output = false;

        match fragment {
            Fragment::Initial(_config_params) => {}
            Fragment::UpdateProposal(_update_proposal) => {}
            Fragment::UpdateVote(_signed_update) => {}
            Fragment::OldUtxoDeclaration(_utxos) => {}
            _ => {
                on_tx_input_and_witnesses(fragment, |(input, _witness)| {
                    if let InputEnum::AccountInput(id, input_value) = input.to_enum() {
                        if self.account_id().as_ref() == id.as_ref() {
                            new_value = new_value.checked_sub(input_value).expect("value overflow");
                        }
                        increment_counter = true;
                    }

                    // TODO: check monotonicity by signing and comparing
                    // if let Witness::Account(witness) = witness {
                    //
                    // }
                });
                on_tx_output(fragment, |(_, output)| {
                    if output
                        .address
                        .public_key()
                        .map(|pk| *pk == Into::<PublicKey<Ed25519>>::into(self.account_id()))
                        .unwrap_or(false)
                    {
                        new_value = new_value.checked_add(output.value).unwrap();
                        at_least_one_output = true;
                    }
                })
            }
        };

        let counter = if increment_counter {
            state.counter.increment().expect("account counter overflow")
        } else {
            state.counter
        };

        let new_state = State {
            counter,
            value: new_value,
        };

        (at_least_one_output || increment_counter, new_state)
    }

    fn pending_transactions<'a>(&'a self) -> Box<dyn Iterator<Item = &'a FragmentId> + 'a> {
        Box::new(
            self.get_states()
                .iter()
                .filter_map(|(k, s)| Some(k).filter(|_| s.is_pending())),
        )
    }

    fn confirm(&mut self, fragment_id: &FragmentId) {
        self.get_states_mut().confirm(fragment_id);
    }

    fn confirmed_value(&self) -> Value {
        self.get_states().confirmed_state().state().value()
    }

    fn unconfirmed_value(&self) -> Option<Value> {
        let s = self.get_states().last_state();

        Some(s)
            .filter(|s| !s.is_confirmed())
            .map(|s| s.state().value())
    }

    fn check_fragment(&mut self, fragment: &Fragment) -> bool {
        if self.get_states().contains(&fragment.hash()) {
            return true;
        }

        let state = self.get_states().last_state().state();

        let (modified_state, new_state) = self.new_state(state, fragment);

        self.get_states_mut().push(fragment.hash(), new_state);

        modified_state
    }
}

pub struct Wallet {
    account: EitherAccount,
    state: States<FragmentId, State>,
}

#[derive(Debug)]
pub struct State {
    value: Value,
    counter: SpendingCounter,
}

pub struct WalletBuildTx<'a> {
    wallet: &'a mut Wallet,
    needed_input: Value,
    next_value: Value,
    current_counter: SpendingCounter,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("not enough funds, needed {needed:?}, available {current:?}")]
    NotEnoughFunds { current: Value, needed: Value },
}

enum EitherAccount {
    Seed(Account<Ed25519>),
    Extended(Account<Ed25519Extended>),
}

impl Wallet {
    pub fn new_from_seed(seed: Seed) -> Wallet {
        Wallet {
            account: EitherAccount::Seed(Account::from_seed(seed)),
            state: States::new(
                FragmentId::zero_hash(),
                State {
                    value: Value::zero(),
                    counter: SpendingCounter::zero(),
                },
            ),
        }
    }

    pub fn new_from_key(key: SecretKey<Ed25519Extended>) -> Wallet {
        Wallet {
            account: EitherAccount::Extended(Account::from_secret_key(key)),
            state: States::new(
                FragmentId::zero_hash(),
                State {
                    value: Value::zero(),
                    counter: SpendingCounter::zero(),
                },
            ),
        }
    }

    pub fn account_id(&self) -> AccountId {
        match &self.account {
            EitherAccount::Extended(account) => account.account_id(),
            EitherAccount::Seed(account) => account.account_id(),
        }
    }

    /// set the state counter so we can sync with the blockchain and the
    /// local state
    ///
    /// TODO: some handling to provide information if needed:
    ///
    /// - [ ] check the counter is not regressing?
    /// - [ ] check that there is continuity?
    ///
    /// TODO: change to a constructor/initializator?, or just make it so it resets the state
    ///
    pub fn update_state(&mut self, value: Value, counter: SpendingCounter) {
        self.state = States::new(FragmentId::zero_hash(), State { value, counter });
    }

    pub fn spending_counter(&self) -> SpendingCounter {
        self.state.last_state().state().counter
    }

    pub fn value(&self) -> Value {
        self.state.last_state().state().value
    }

    pub fn new_transaction(&mut self, needed_input: Value) -> Result<WalletBuildTx, Error> {
        let state = self.state.last_state().state();
        let current_counter = state.counter;
        let next_value =
            state
                .value
                .checked_sub(needed_input)
                .map_err(|_| Error::NotEnoughFunds {
                    current: state.value,
                    needed: needed_input,
                })?;

        Ok(WalletBuildTx {
            wallet: self,
            needed_input,
            current_counter,
            next_value,
        })
    }
}

impl<'a> WalletBuildTx<'a> {
    pub fn input(&self) -> Input {
        Input::from_account_public_key(self.wallet.account_id().into(), self.needed_input)
    }

    pub fn witness_builder(&self) -> AccountWitnessBuilder {
        match &self.wallet.account {
            EitherAccount::Seed(account) => crate::transaction::AccountWitnessBuilder::Ed25519(
                account.secret().clone(),
                self.current_counter,
            ),
            EitherAccount::Extended(account) => {
                crate::transaction::AccountWitnessBuilder::Ed25519Extended(
                    account.secret().clone(),
                    self.current_counter,
                )
            }
        }
    }

    pub fn add_fragment_id(self, fragment_id: FragmentId) {
        self.wallet.state.push(
            fragment_id,
            State {
                value: self.next_value,
                counter: self.current_counter.increment().unwrap(),
            },
        );
    }
}
