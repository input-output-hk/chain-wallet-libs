//! JavaScript and TypeScript bindings for the Jormungandr wallet SDK.

use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;
use std::convert::TryInto;

use wasm_bindgen::prelude::*;

mod utils;

pub use utils::set_panic_hook;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// A Wallet gives the user control over an account address
/// controlled by a private key. It can also be used to convert other funds
/// minted as UTxOs in the genesis block.
#[wasm_bindgen]
pub struct Wallet(wallet_core::Wallet);

/// Encapsulates blockchain settings needed for some operations.
#[wasm_bindgen]
pub struct Settings(wallet_core::Settings);

/// Information about a proposal in a vote plan deployed onto the blockchain.
#[wasm_bindgen]
pub struct Proposal(wallet_core::Proposal);

/// Identifier for a vote plan deployed onto the blockchain.
#[wasm_bindgen]
pub struct VotePlanId([u8; wallet_core::VOTE_PLAN_ID_LENGTH]);

#[wasm_bindgen]
pub struct Options(wallet_core::Options);

impl_secret_key!(
    Ed25519ExtendedPrivate,
    chain_crypto::Ed25519Extended,
    Ed25519Public
);
impl_secret_key!(Ed25519Private, chain_crypto::Ed25519, Ed25519Public);

impl_public_key!(Ed25519Public, chain_crypto::Ed25519);

/// Signature obtained with the Ed25519 algorithm.
#[wasm_bindgen]
pub struct Ed25519Signature(chain_crypto::Signature<Box<[u8]>, chain_crypto::Ed25519>);

/// Identifier of a block fragment, such as a vote transaction posted on the blockchain.
#[wasm_bindgen]
pub struct FragmentId(wallet_core::FragmentId);

/// A public key for the election protocol that is used to encrypt private ballots.
#[wasm_bindgen]
pub struct ElectionPublicKey(chain_vote::ElectionPublicKey);

/// this is used only for giving the Array a type in the typescript generated notation
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Array<FragmentId>")]
    pub type FragmentIds;
}

#[wasm_bindgen]
pub struct BlockDate(chain_impl_mockchain::block::BlockDate);

#[wasm_bindgen]
impl Wallet {
    /// Imports private keys to create a wallet.
    ///
    /// The `account` parameter gives the Ed25519Extended private key
    /// of the account.
    ///
    /// The `keys` parameter should be a concatenation of Ed25519Extended
    /// private keys that will be used to retrieve the associated UTxOs.
    /// Pass an empty buffer when this functionality is not needed.
    pub fn import_keys(account: &[u8], keys: &[u8]) -> Result<Wallet, JsValue> {
        if keys.len() % 64 != 0 {
            return Err(JsValue::from_str("invalid keys array length"));
        }

        let keys: &[[u8; 64]] = unsafe {
            std::slice::from_raw_parts(keys.as_ptr().cast::<[u8; 64]>(), keys.len() / 64)
        };

        wallet_core::Wallet::recover_free_keys(account, keys.iter())
            .map_err(|e| JsValue::from(e.to_string()))
            .map(Wallet)
    }

    /// get the account ID bytes
    ///
    /// This ID is also the account public key, it can be used to retrieve the
    /// account state (the value, transaction counter etc...).
    pub fn id(&self) -> Vec<u8> {
        self.0.id().as_ref().to_vec()
    }

    /// Get the total value in the wallet.
    ///
    /// Make sure to call `retrieve_funds` prior to calling this function,
    /// otherwise the function will return `0`.
    pub fn total_value(&self) -> u64 {
        self.0.total_value().0
    }

    /// Update the wallet account state.
    ///
    /// The values to update the account state with can be retrieved from a
    /// node API endpoint. It sets the balance value on the account
    /// as well as the current spending counter.
    ///
    /// It is important to be sure to have an up to date wallet state
    /// before doing any transactions, otherwise future transactions may fail
    /// to be accepted by the blockchain nodes because of an invalid witness
    /// signature.
    pub fn set_state(&mut self, value: u64, counter: u32) {
        self.0.set_state(wallet_core::Value(value), counter);
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
    /// * `valid_until` - the date until this transaction can be applied
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
        valid_until: &BlockDate,
    ) -> Result<Box<[u8]>, JsValue> {
        self.0
            .vote(
                settings.0.clone(),
                &proposal.0,
                wallet_core::Choice::new(choice),
                &valid_until.0,
            )
            .map_err(|e| JsValue::from(e.to_string()))
    }

    /// Confirms that a transaction has been confirmed on the blockchain.
    ///
    /// This function will update the state of the wallet tracking pending
    /// transactions on fund conversion.
    pub fn confirm_transaction(&mut self, fragment: &FragmentId) {
        self.0.confirm_transaction(fragment.0);
    }
}

#[wasm_bindgen]
impl Proposal {
    /// Constructs a description of a public vote proposal from its constituent data.
    ///
    /// Parameters:
    /// * `vote_plan_id`: Identifier of the vote plan.
    /// * `index`: 0-based index of the proposal in the vote plan.
    /// * `options`: Descriptor of vote plan options, detailing the number of choices.
    pub fn new_public(vote_plan_id: VotePlanId, index: u8, options: Options) -> Self {
        Proposal(wallet_core::Proposal::new(
            vote_plan_id.0.into(),
            index,
            options.0,
            wallet_core::PayloadTypeConfig::Public,
        ))
    }

    /// Constructs a description of a private vote proposalfrom its constituent data.
    ///
    /// Parameters:
    /// * `vote_plan_id`: Identifier of the vote plan.
    /// * `index`: 0-based index of the proposal in the vote plan.
    /// * `options`: Descriptor of vote plan options, detailing the number of choices.
    /// * `election_key`: The public key for the vote plan used to encrypt ballots.
    pub fn new_private(
        vote_plan_id: VotePlanId,
        index: u8,
        options: Options,
        election_key: ElectionPublicKey,
    ) -> Self {
        Proposal(wallet_core::Proposal::new_private(
            vote_plan_id.0.into(),
            index,
            options.0,
            election_key.0,
        ))
    }
}

#[wasm_bindgen]
impl VotePlanId {
    /// Constructs a VotePlanId value from its byte array representation.
    pub fn from_bytes(bytes: &[u8]) -> Result<VotePlanId, JsValue> {
        let array: [u8; wallet_core::VOTE_PLAN_ID_LENGTH] = bytes
            .try_into()
            .map_err(|_| JsValue::from_str("Invalid vote plan id length"))?;

        Ok(VotePlanId(array))
    }

    /// Deprecated; use `from_bytes`.
    pub fn new_from_bytes(bytes: &[u8]) -> Result<VotePlanId, JsValue> {
        Self::from_bytes(bytes)
    }
}

#[wasm_bindgen]
impl Options {
    pub fn new_length(length: u8) -> Result<Options, JsValue> {
        wallet_core::Options::new_length(length)
            .map_err(|e| JsValue::from(e.to_string()))
            .map(Options)
    }
}

#[wasm_bindgen]
impl Ed25519Signature {
    /// Constructs a signature object from its byte array representation.
    pub fn from_bytes(signature: &[u8]) -> Result<Ed25519Signature, JsValue> {
        chain_crypto::Signature::from_binary(signature)
            .map(Self)
            .map_err(|e| JsValue::from_str(&format!("Invalid signature {}", e)))
    }

    /// Deprecated; use `from_bytes`.
    pub fn from_binary(signature: &[u8]) -> Result<Ed25519Signature, JsValue> {
        Self::from_bytes(signature)
    }

    /// Returns a byte array representation of the signature.
    pub fn to_bytes(&self) -> Box<[u8]> {
        self.0.as_ref().into()
    }
}

#[macro_export]
macro_rules! impl_public_key {
    ($name:ident, $wrapped_type:ty) => {
        #[wasm_bindgen]
        pub struct $name(chain_crypto::PublicKey<$wrapped_type>);

        #[wasm_bindgen]
        impl $name {
            /// Returns a byte array representation of the public key.
            // TODO: rename to `to_bytes` for harmonization with the rest of the API?
            pub fn bytes(&self) -> Box<[u8]> {
                self.0.as_ref().into()
            }

            /// Returns the key formatted as a string in Bech32 format.
            pub fn bech32(&self) -> String {
                use chain_crypto::bech32::Bech32 as _;
                self.0.to_bech32_str()
            }

            /// Uses the given signature to verify the given message.
            pub fn verify(&self, signature: &Ed25519Signature, msg: &[u8]) -> bool {
                let verification = signature.0.verify_slice(&self.0, msg);
                match verification {
                    chain_crypto::Verification::Success => true,
                    chain_crypto::Verification::Failed => false,
                }
            }
        }
    };
}

/// macro arguments:
///     the exported name of the type
///     the inner/mangled key type
///     the name of the exported public key associated type
#[macro_export]
macro_rules! impl_secret_key {
    ($name:ident, $wrapped_type:ty, $public:ident) => {
        #[wasm_bindgen]
        pub struct $name(chain_crypto::SecretKey<$wrapped_type>);

        #[wasm_bindgen]
        impl $name {
            /// Generates the key using OS-provided entropy.
            pub fn generate() -> $name {
                Self(chain_crypto::SecretKey::<$wrapped_type>::generate(
                    rand::rngs::OsRng,
                ))
            }

            /// Generates the key from a seed value.
            /// For the same entropy value of 32 bytes, the same key will be generated.
            /// This seed will be fed to ChaChaRNG and allow pseudo random key generation.
            /// Do not use if you are not sure.
            pub fn from_seed(seed: &[u8]) -> Result<$name, JsValue> {
                let seed: [u8; 32] = seed
                    .try_into()
                    .map_err(|_| JsValue::from_str("Invalid seed, expected 32 bytes"))?;

                let rng = ChaCha20Rng::from_seed(seed);

                Ok(Self(chain_crypto::SecretKey::<$wrapped_type>::generate(
                    rng,
                )))
            }

            /// Returns the public key corresponding to this secret key.
            pub fn public(&self) -> $public {
                $public(self.0.to_public())
            }

            /// Returns the key represented by an array of bytes.
            /// Use with care: the secret key should not be revealed to external
            /// observers or exposed to untrusted code.
            // TODO: rename to leak_bytes() to emphasize the security caveats?
            pub fn bytes(&self) -> Box<[u8]> {
                self.0.clone().leak_secret().as_ref().into()
            }

            /// Signs the provided message with this secret key.
            pub fn sign(&self, msg: &[u8]) -> Ed25519Signature {
                Ed25519Signature::from_bytes(self.0.sign(&msg).as_ref()).unwrap()
            }
        }
    };
}

#[wasm_bindgen]
impl FragmentId {
    /// Constructs a fragment identifier from its byte array representation.
    pub fn from_bytes(bytes: &[u8]) -> Result<FragmentId, JsValue> {
        let array: [u8; std::mem::size_of::<wallet_core::FragmentId>()] = bytes
            .try_into()
            .map_err(|_| JsValue::from_str("Invalid fragment id"))?;

        Ok(FragmentId(array.into()))
    }

    /// Deprecated; use `from_bytes`.
    pub fn new_from_bytes(bytes: &[u8]) -> Result<FragmentId, JsValue> {
        Self::from_bytes(bytes)
    }

    /// Returns a byte array representation of the fragment identifier.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.as_bytes().to_vec()
    }
}

#[wasm_bindgen]
impl ElectionPublicKey {
    /// Constructs a key from its byte array representation.
    pub fn from_bytes(bytes: &[u8]) -> Result<ElectionPublicKey, JsValue> {
        chain_vote::ElectionPublicKey::from_bytes(bytes)
            .ok_or_else(|| JsValue::from_str("invalid binary format"))
            .map(Self)
    }

    /// Decodes the key from a string in Bech32 format.
    pub fn from_bech32(bech32_str: &str) -> Result<ElectionPublicKey, JsValue> {
        use chain_crypto::bech32::Bech32;
        chain_vote::ElectionPublicKey::try_from_bech32_str(bech32_str)
            .map_err(|e| JsValue::from_str(&format!("invalid bech32 string {}", e)))
            .map(Self)
    }
}

#[wasm_bindgen]
pub fn symmetric_decrypt(password: &[u8], data: &[u8]) -> Result<Box<[u8]>, JsValue> {
    symmetric_cipher::decrypt(password, data)
        .map_err(|e| JsValue::from_str(&format!("decryption failed {}", e)))
}
