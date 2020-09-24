use crate::Result;

pub type PendingTransactionsPtr = *mut PendingTransactions;

/// opaque handle over a list of pending transaction ids
pub struct PendingTransactions {
    fragment_ids: Box<[chain_impl_mockchain::fragment::FragmentId]>,
}

///
/// # Safety
///
/// This function dereference raw pointers (wallet, fragment_id). Even though
/// the function checks if the pointers are null. Mind not to put random values
/// in or you may see unexpected behaviors.
///
pub unsafe fn pending_transactions_len(
    transactions: PendingTransactionsPtr,
    len_out: *mut usize,
) -> Result {
    let pending_transactions = non_null!(transactions);

    *len_out = pending_transactions.fragment_ids.len();

    Result::success()
}

///
/// # Safety
///
/// This function dereference raw pointers (wallet, fragment_id). Even though
/// the function checks if the pointers are null. Mind not to put random values
/// in or you may see unexpected behaviors.
///
pub unsafe fn pending_transactions_get(
    transactions: PendingTransactionsPtr,
    index: usize,
    id_out: *mut *const u8,
) -> Result {
    let pending_transactions = non_null!(transactions);

    let fragment_id: &[u8] = pending_transactions.fragment_ids[index].as_ref();

    *id_out = fragment_id.as_ptr();

    Result::success()
}

/// delete the pointer and free the allocated memory
///
/// # Safety
///
/// This function dereference raw pointers (wallet, fragment_id). Even though
/// the function checks if the pointers are null. Mind not to put random values
/// in or you may see unexpected behaviors.
///
pub unsafe fn pending_transactions_delete(pending: PendingTransactionsPtr) {
    if !pending.is_null() {
        let boxed = Box::from_raw(pending);

        std::mem::drop(boxed);
    }
}
