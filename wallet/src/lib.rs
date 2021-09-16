mod account;
mod blockchain;
mod keygen;
mod password;
mod recovering;
pub mod scheme;
mod states;
mod store;
pub mod time;
pub mod transaction;

pub use self::{
    account::Wallet as AccountWallet,
    blockchain::Settings,
    password::{Password, ScrubbedBytes},
    recovering::{RecoveryBuilder, RecoveryError},
    transaction::{AccountWitnessBuilder, TransactionBuilder},
};
use chain_impl_mockchain::{
    fragment::{Fragment, FragmentId},
    value::Value,
};
pub use hdkeygen::account::AccountId;
use states::States;

pub trait WalletState {
    fn value(&self) -> Value;
}

pub trait Wallet {
    type State: WalletState;

    fn get_states(&self) -> &States<FragmentId, Self::State>;
    fn get_states_mut(&mut self) -> &mut States<FragmentId, Self::State>;

    /// get all the pending transactions of the wallet
    ///
    /// If empty it means there's no pending transactions waiting confirmation
    ///
    fn pending_transactions<'a>(&'a self) -> Box<dyn Iterator<Item = &'a FragmentId> + 'a> {
        Box::new(
            self.get_states()
                .iter()
                .filter_map(|(k, s)| Some(k).filter(|_| s.is_pending())),
        )
    }

    /// confirm a pending transaction
    ///
    /// to only do once it is confirmed a transaction is on chain
    /// and is far enough in the blockchain history to be confirmed
    /// as immutable
    ///
    fn confirm(&mut self, fragment_id: &FragmentId) {
        self.get_states_mut().confirm(fragment_id);
    }

    /// get the confirmed value of the wallet
    fn confirmed_value(&self) -> Value {
        self.get_states().confirmed_state().state().value()
    }

    /// get the unconfirmed value of the wallet
    ///
    /// if `None`, it means there is no unconfirmed state of the wallet
    /// and the value can be known from `confirmed_value`.
    ///
    /// The returned value is the value we expect to see at some point on
    /// chain once all transactions are on chain confirmed.
    ///
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

    fn new_state(&self, state: &Self::State, fragment: &Fragment) -> (bool, Self::State);
}
