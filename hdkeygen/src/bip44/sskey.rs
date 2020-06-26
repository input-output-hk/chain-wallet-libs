use crate::{account::AccountId, bip44::COIN_TYPE, Key};
use chain_path_derivation::{
    bip44::{self, Bip44},
    DerivationPath,
};
use ed25519_bip32::{DerivationScheme, XPrv, XPub};
use std::ops::Deref;

/// Silly Stake Keys are defined as the address level but a different derivation
/// path at the `Change`.
pub struct SSKey<K> {
    key: Key<K, Bip44<bip44::Address>>,
}

impl SSKey<XPrv> {
    pub fn public(&self) -> SSKey<XPub> {
        SSKey {
            key: self.key.public(),
        }
    }

    /// retrieve the account identifier
    pub fn account_id(&self) -> AccountId {
        self.key.public().public_key().public_key().into()
    }
}

impl SSKey<XPub> {
    /// retrieve the account identifier
    pub fn account_id(&self) -> AccountId {
        self.key.public_key().public_key().into()
    }
}

impl<K> SSKey<K> {
    pub fn new(key: Key<K, Bip44<bip44::Address>>) -> Self {
        Self { key }
    }

    /// load the account key from the given Key, DerivationScheme and index
    ///
    /// Here it is expected that K has been derived already on the 5 first
    /// levels of the bip44 for Cardano Ada coin type
    ///
    /// # panics
    ///
    /// This function will panic if path's coin_type is not Cardano ADA
    /// coin type.
    pub fn from_key(
        key: K,
        derivation_scheme: DerivationScheme,
        path: DerivationPath<Bip44<bip44::Address>>,
    ) -> Self {
        assert_eq!(
            path.coin_type(),
            COIN_TYPE,
            "Expecting Cardano ADA coin type"
        );

        let key = Key::new_unchecked(key, path, derivation_scheme);
        Self::new(key)
    }

    pub fn key(&self) -> &Key<K, Bip44<bip44::Address>> {
        &self.key
    }
}

impl<K> Deref for SSKey<K> {
    type Target = Key<K, Bip44<bip44::Address>>;
    fn deref(&self) -> &Self::Target {
        self.key()
    }
}

impl<K: Clone> Clone for SSKey<K> {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
        }
    }
}
