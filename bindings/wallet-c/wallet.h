/**
 * Wallet for Jörmungandr blockchain
 *
 * Provide support for recovering funds from both Yoroi and Daedalus wallets.
 *
 * Copyright 2020, Input Output HK Ltd
 * Licensed with: MIT OR Apache-2.0
 */

#ifndef IOHK_CHAIN_WALLET_LIBC_
#define IOHK_CHAIN_WALLET_LIBC_

/* Generated with cbindgen:0.20.0 */

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum Discrimination
{
  Discrimination_Production = 0,
  Discrimination_Test,
} Discrimination;

typedef struct Error
{

} Error;

typedef struct Error *ErrorPtr;

typedef struct Settings
{

} Settings;

typedef struct BlockDate
{
  uint32_t epoch;
  uint32_t slot;
} BlockDate;

typedef struct Fragment
{

} Fragment;

typedef struct Fragment *FragmentPtr;

typedef struct SpendingCounters
{
  uint8_t *data;
  uintptr_t len;
} SpendingCounters;

typedef struct Proposal
{

} Proposal;

typedef struct Proposal *ProposalPtr;

typedef struct Settings *SettingsPtr;

typedef struct Wallet
{

} Wallet;

typedef struct Wallet *WalletPtr;

typedef struct PerCertificateFee
{
  uint64_t certificate_pool_registration;
  uint64_t certificate_stake_delegation;
  uint64_t certificate_owner_stake_delegation;
} PerCertificateFee;

typedef struct PerVoteCertificateFee
{
  uint64_t certificate_vote_plan;
  uint64_t certificate_vote_cast;
} PerVoteCertificateFee;

/**
 * Linear fee using the basic affine formula
 * `COEFFICIENT * bytes(COUNT(tx.inputs) + COUNT(tx.outputs)) + CONSTANT + CERTIFICATE*COUNT(certificates)`.
 */
typedef struct LinearFee
{
  uint64_t constant;
  uint64_t coefficient;
  uint64_t certificate;
  struct PerCertificateFee per_certificate_fees;
  struct PerVoteCertificateFee per_vote_certificate_fees;
} LinearFee;

typedef uint32_t Epoch;

typedef uint64_t Slot;

typedef struct TimeEra
{
  Epoch epoch_start;
  Slot slot_start;
  uint32_t slots_per_epoch;
} TimeEra;

typedef struct SettingsInit
{
  struct LinearFee fees;
  enum Discrimination discrimination;
  /**
   * block_0_initial_hash is assumed to point to 32 bytes of readable memory
   */
  const uint8_t *block0_initial_hash;
  /**
   * Unix timestamp of the genesis block.
   * Provides an anchor to compute block dates from calendar date/time.
   */
  uint64_t block0_date;
  uint8_t slot_duration;
  struct TimeEra time_era;
  uint8_t transaction_max_expiry_epochs;
} SettingsInit;

typedef struct TransactionOut
{
  const uint8_t *data;
  uintptr_t len;
} TransactionOut;

/**
 * This function dereference raw pointers. Even though the function checks if
 * the pointers are null. Mind not to put random values in or you may see
 * unexpected behaviors.
 *
 * # Arguments
 *
 * *settings*: the blockchain settings previously allocated with this library.
 * *date*: desired date of expiration for a fragment. It must be expressed in seconds since the
 * unix epoch.
 * *block_date_out*: pointer to an allocated BlockDate structure, the memory should be writable.
 *
 * # Safety
 *
 * pointers should be allocated by this library and be valid.
 * null pointers are checked and will result in an error.
 *
 */
ErrorPtr iohk_jormungandr_block_date_from_system_time(const struct Settings *settings,
                                                      uint64_t date,
                                                      struct BlockDate *block_date_out);

/**
 * delete the pointer
 *
 * # Safety
 *
 * This function dereference raw pointers. Even though
 * the function checks if the pointers are null. Mind not to put random values
 * in or you may see unexpected behaviors
 */
void iohk_jormungandr_delete_fragment(FragmentPtr fragment);

/**
 * delete the inner buffer that was allocated by this library
 *
 * # Safety
 *
 * This function dereference raw pointers. Even though
 * the function checks if the pointers are null. Mind not to put random values
 * in or you may see unexpected behaviors
 */
void iohk_jormungandr_delete_spending_counters(struct SpendingCounters spending_counters);

/**
 * deserialize a fragment from bytes
 *
 * # Parameters
 *
 * * `buffer` -- a pointer to the serialized fragment bytes.
 * * `buffer_length` -- the length of the serialized fragment bytes array.
 * * `fragment` -- the location of the pointer to the deserialized fragemnt.
 *
 * # Errors
 *
 * This functions may fail if:
 *
 * * `buffer` is a null pointer.
 * * `fragment` is a null pointer.
 * * `buffer` contains invalid fragment bytes.
 *
 * # Safety
 *
 * This function dereference raw pointers. Even though
 * the function checks if the pointers are null. Mind not to put random values
 * in or you may see unexpected behaviors
 *
 * Don't forget to delete the fragment object with `iohk_jormungandr_delete_fragment`.
 */
ErrorPtr iohk_jormungandr_fragment_from_raw(const uint8_t *buffer,
                                            uintptr_t buffer_length,
                                            FragmentPtr *fragment_out);

/**
 * get the ID of the provided fragment
 *
 * # Parameters
 *
 * * `fragment` -- a pointer to fragment.
 * * `fragment_id_out` -- a pointer to a pre-allocated 32 bytes array.
 *
 * # Errors
 *
 * This function would return an error if either of the provided pointers is null.
 *
 * # Safety
 *
 * This function dereference raw pointers. Even though
 * the function checks if the pointers are null. Mind not to put random values
 * in or you may see unexpected behaviors.
 *
 * `fragment_id_out` is expected to be an already allocated 32 byte array. Doing otherwise may
 * potentially result into an undefined behavior.
 */
ErrorPtr iohk_jormungandr_fragment_id(FragmentPtr fragment,
                                      uint8_t *fragment_id_out);

/**
 * This function dereference raw pointers. Even though the function checks if
 * the pointers are null. Mind not to put random values in or you may see
 * unexpected behaviors.
 *
 * # Arguments
 *
 * *settings*: the blockchain settings previously allocated with this library.
 * *current_time*: Current real time. It must be expressed in seconds since the unix epoch.
 * *block_date_out*: pointer to an allocated BlockDate structure, the memory should be writable.
 *
 * # Safety
 *
 * pointers should be allocated by this library and be valid.
 * null pointers are checked and will result in an error.
 *
 */
ErrorPtr iohk_jormungandr_max_expiration_date(const struct Settings *settings,
                                              uint64_t current_time,
                                              struct BlockDate *block_date_out);

/**
 * decrypt payload of the wallet transfer protocol
 *
 * Parameters
 *
 * password: byte buffer with the encryption password
 * password_length: length of the password buffer
 * ciphertext: byte buffer with the encryption password
 * ciphertext_length: length of the password buffer
 * plaintext_out: used to return a pointer to a byte buffer with the decrypted text
 * plaintext_out_length: used to return the length of decrypted text
 *
 * The returned buffer is in the heap, so make sure to call the delete_buffer function
 *
 * # Safety
 *
 * This function dereference raw pointers. Even though the function checks if
 * the pointers are null. Mind not to put random values in or you may see
 * unexpected behaviors.
 */
ErrorPtr iohk_jormungandr_symmetric_cipher_decrypt(const uint8_t *password,
                                                   uintptr_t password_length,
                                                   const uint8_t *ciphertext,
                                                   uintptr_t ciphertext_length,
                                                   const uint8_t **plaintext_out,
                                                   uintptr_t *plaintext_out_length);

/**
 * build the proposal object
 *
 * * `vote_encryption_key`: a null terminated string (c-string) with the bech32
 * representation of the encryption vote key
 *
 * # Errors
 *
 * This function may fail if:
 *
 * * `proposal_out` is null.
 * * `num_choices` is out of the allowed range.
 *
 * # Safety
 *
 * This function dereference raw pointers. Even though the function checks if
 * the pointers are null. Mind not to put random values in or you may see
 * unexpected behaviors.
 */
ErrorPtr iohk_jormungandr_vote_proposal_new_private(const uint8_t *vote_plan_id,
                                                    uint8_t index,
                                                    uint8_t num_choices,
                                                    const char *vote_encryption_key,
                                                    ProposalPtr *proposal_out);

/**
 * build the proposal object
 *
 * # Errors
 *
 * This function may fail if:
 *
 * * `proposal_out` is null.
 * * `num_choices` is out of the allowed range.
 *
 * # Safety
 *
 * This function dereference raw pointers. Even though the function checks if
 * the pointers are null. Mind not to put random values in or you may see
 * unexpected behaviors.
 */
ErrorPtr iohk_jormungandr_vote_proposal_new_public(const uint8_t *vote_plan_id,
                                                   uint8_t index,
                                                   uint8_t num_choices,
                                                   ProposalPtr *proposal_out);

/**
 * Delete a binary buffer that was returned by this library alongside with its
 * length.
 *
 * # Safety
 *
 * This function dereference raw pointers. Even though
 * the function checks if the pointers are null. Mind not to put random values
 * in or you may see unexpected behaviors
 */
void iohk_jormungandr_wallet_delete_buffer(uint8_t *ptr, uintptr_t length);

/**
 * delete the pointer and free the allocated memory
 *
 * # Safety
 *
 * This function dereference raw pointers. Even though
 * the function checks if the pointers are null. Mind not to put random values
 * in or you may see unexpected behaviors
 *
 */
void iohk_jormungandr_wallet_delete_error(ErrorPtr error);

/**
 * delete the pointer
 *
 * # Safety
 *
 * This function dereference raw pointers. Even though
 * the function checks if the pointers are null. Mind not to put random values
 * in or you may see unexpected behaviors
 *
 */
void iohk_jormungandr_wallet_delete_proposal(ProposalPtr proposal);

/**
 * delete the pointer and free the allocated memory
 *
 * # Safety
 *
 * This function dereference raw pointers. Even though
 * the function checks if the pointers are null. Mind not to put random values
 * in or you may see unexpected behaviors
 *
 */
void iohk_jormungandr_wallet_delete_settings(SettingsPtr settings);

/**
 * Delete a null terminated string that was allocated by this library
 *
 * # Safety
 *
 * This function dereference raw pointers. Even though
 * the function checks if the pointers are null. Mind not to put random values
 * in or you may see unexpected behaviors
 *
 */
void iohk_jormungandr_wallet_delete_string(char *ptr);

/**
 * delete the pointer, zero all the keys and free the allocated memory
 *
 * # Safety
 *
 * This function dereference raw pointers. Even though
 * the function checks if the pointers are null. Mind not to put random values
 * in or you may see unexpected behaviors
 *
 */
void iohk_jormungandr_wallet_delete_wallet(WalletPtr wallet);

/**
 * Get a string describing the error, this will return an allocated
 * null terminated string providing extra details regarding the source
 * of the error.
 *
 * If the given error is a `NULL` pointer, the string is and always
 * is `"success"`. If no details are available the function will return
 * `"no more details"`. This string still need to be deleted with the
 * `iohk_jormungandr_wallet_delete_string` function.
 *
 * This function returns an allocated null terminated pointer. Don't
 * forget to free the memory with: `iohk_jormungandr_wallet_delete_string`.
 *
 * # Safety
 *
 * This function dereference raw pointers. Even though
 * the function checks if the pointers are null. Mind not to put random values
 * in or you may see unexpected behaviors
 *
 */
char *iohk_jormungandr_wallet_error_details(ErrorPtr error);

/**
 * Get a string describing the error, this will return an allocated
 * null terminated string describing the error.
 *
 * If the given error is a `NULL` pointer, the string is and always
 * is `"success"`. This string still need to be deleted with the
 * `iohk_jormungandr_wallet_delete_string` function.
 *
 * This function returns an allocated null terminated pointer. Don't
 * forget to free the memory with: `iohk_jormungandr_wallet_delete_string`.
 *
 * # Safety
 *
 * This function dereference raw pointers. Even though
 * the function checks if the pointers are null. Mind not to put random values
 * in or you may see unexpected behaviors
 *
 */
char *iohk_jormungandr_wallet_error_to_string(ErrorPtr error);

/**
 * get the wallet id
 *
 * This ID is the identifier to use against the blockchain/explorer to retrieve
 * the state of the wallet (counter, total value etc...)
 *
 * # Parameters
 *
 * * wallet: the recovered wallet (see recover function);
 * * id_out: a ready allocated pointer to an array of 32bytes. If this array is not
 *   32bytes this may result in a buffer overflow.
 *
 * # Safety
 *
 * This function dereference raw pointers (wallet, block0 and settings_out). Even though
 * the function checks if the pointers are null. Mind not to put random values
 * in or you may see unexpected behaviors
 *
 * the `id_out` needs to be ready allocated 32bytes memory. If not this will result
 * in an undefined behavior, in the best scenario it will be a buffer overflow.
 *
 * # Errors
 *
 * On error the function returns a `ErrorPtr`. On success `NULL` is returned.
 * The `ErrorPtr` can then be observed to gathered details of the error.
 * Don't forget to call `iohk_jormungandr_wallet_delete_error` to free
 * the `ErrorPtr` from memory and avoid memory leaks.
 *
 * * this function may fail if the wallet pointer is null;
 *
 */
ErrorPtr iohk_jormungandr_wallet_id(WalletPtr wallet,
                                    uint8_t *id_out);

/**
 * recover a wallet from an account and a list of utxo keys
 *
 * You can also use this function to recover a wallet even after you have
 * transferred all the funds to the new format (see the _convert_ function)
 *
 * The recovered wallet will be returned in `wallet_out`.
 *
 * # parameters
 *
 * * account_key: the Ed25519 extended key used wallet's account address private key
 *     in the form of a 64 bytes array.
 * * utxo_keys: an array of Ed25519 extended keys in the form of 64 bytes, used as utxo
 *     keys for the wallet
 * * utxo_keys_len: the number of keys in the utxo_keys array (not the number of bytes)
 * * wallet_out: the recovered wallet
 *
 * # Safety
 *
 * This function dereference raw pointers (password and wallet_out). Even though
 * the function checks if the pointers are null. Mind not to put random values
 * in or you may see unexpected behaviors
 *
 * # errors
 *
 * The function may fail if:
 *
 * * the `wallet_out` is null pointer
 *
 */
ErrorPtr iohk_jormungandr_wallet_import_keys(const uint8_t *account_key,
                                             const uint8_t *utxo_keys,
                                             uintptr_t utxo_keys_len,
                                             WalletPtr *wallet_out);

/**
 * update the wallet account state
 *
 * this is the value retrieved from any jormungandr endpoint that allows to query
 * for the account state. It gives the value associated to the account as well as
 * the counter.
 *
 * It is important to be sure to have an updated wallet state before doing any
 * transactions otherwise future transactions may fail to be accepted by any
 * nodes of the blockchain because of invalid signature state.
 *
 * # Errors
 *
 * * this function may fail if the wallet pointer is null;
 *
 * On error the function returns a `ErrorPtr`. On success `NULL` is returned.
 * The `ErrorPtr` can then be observed to gathered details of the error.
 * Don't forget to call `iohk_jormungandr_wallet_delete_error` to free
 * the `ErrorPtr` from memory and avoid memory leaks.
 *
 * # Safety
 *
 * This function dereference raw pointers. Even though
 * the function checks if the pointers are null. Mind not to put random values
 * in or you may see unexpected behaviors
 *
 */
ErrorPtr iohk_jormungandr_wallet_set_state(WalletPtr wallet,
                                           uint64_t value,
                                           struct SpendingCounters counters);

/**
 * # Safety
 *
 *   This function assumes block0_hash points to 32 bytes of valid memory
 *   This function also assumes that settings is a valid pointer previously
 *   obtained with this library, a null check is performed, but is important that
 *   the data it points to is valid
 */
ErrorPtr iohk_jormungandr_wallet_settings_block0_hash(SettingsPtr settings,
                                                      uint8_t *block0_hash);

/**
 * # Safety
 *
 *   This function also assumes that settings is a valid pointer previously
 *   obtained with this library, a null check is performed, but is important that
 *   the data it points to is valid
 *
 *   discrimination_out must point to valid writable memory, a null check is
 *   performed
 */
ErrorPtr iohk_jormungandr_wallet_settings_discrimination(SettingsPtr settings,
                                                         enum Discrimination *discrimination_out);

/**
 * # Safety
 *
 *   This function also assumes that settings is a valid pointer previously
 *   obtained with this library, a null check is performed, but is important that
 *   the data it points to is valid
 *
 *   linear_fee_out must point to valid writable memory, a null check is
 *   performed
 */
ErrorPtr iohk_jormungandr_wallet_settings_fees(SettingsPtr settings,
                                               struct LinearFee *linear_fee_out);

/**
 * # Safety
 *
 * settings_out must point to valid writable memory
 * block_0_hash is assumed to point to 32 bytes of readable memory
 */
ErrorPtr iohk_jormungandr_wallet_settings_new(struct SettingsInit settings_init,
                                              SettingsPtr *settings_out);

/**
 * get the current spending counters for the (only) account in this wallet
 *
 * iohk_jormungandr_spending_counters_delete should be called to deallocate the memory when it's
 * not longer needed
 *
 * # Errors
 *
 * * this function may fail if the wallet pointer is null;
 *
 * # Safety
 *
 * This function dereference raw pointers. Even though
 * the function checks if the pointers are null. Mind not to put random values
 * in or you may see unexpected behaviors
 *
 */
ErrorPtr iohk_jormungandr_wallet_spending_counters(WalletPtr wallet,
                                                   struct SpendingCounters *spending_counters_ptr);

/**
 * get the total value in the wallet
 *
 * make sure to call `retrieve_funds` prior to calling this function
 * otherwise you will always have `0`
 *
 * After calling this function the results is returned in the `total_out`.
 *
 * # Errors
 *
 * * this function may fail if the wallet pointer is null;
 *
 * On error the function returns a `ErrorPtr`. On success `NULL` is returned.
 * The `ErrorPtr` can then be observed to gathered details of the error.
 * Don't forget to call `iohk_jormungandr_wallet_delete_error` to free
 * the `ErrorPtr` from memory and avoid memory leaks.
 *
 * If the `total_out` pointer is null, this function does nothing
 *
 * # Safety
 *
 * This function dereference raw pointers. Even though
 * the function checks if the pointers are null. Mind not to put random values
 * in or you may see unexpected behaviors
 *
 */
ErrorPtr iohk_jormungandr_wallet_total_value(WalletPtr wallet,
                                             uint64_t *total_out);

/**
 * build the vote cast transaction
 *
 * # Errors
 *
 * This function may fail upon receiving a null pointer or a `choice` value
 * that does not fall within the range specified in `proposal`.
 *
 * # Safety
 *
 * This function dereference raw pointers. Even though the function checks if
 * the pointers are null. Mind not to put random values in or you may see
 * unexpected behaviors.
 *
 * Don't forget to remove `transaction_out` with
 * `iohk_jormungandr_waller_delete_buffer`.
 */
ErrorPtr iohk_jormungandr_wallet_vote_cast(WalletPtr wallet,
                                           SettingsPtr settings,
                                           ProposalPtr proposal,
                                           uint8_t choice,
                                           struct BlockDate valid_until,
                                           uint8_t lane,
                                           struct TransactionOut *transaction_out);

#endif /* IOHK_CHAIN_WALLET_LIBC_ */
