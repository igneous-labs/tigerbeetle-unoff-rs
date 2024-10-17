use core::{borrow::Borrow, num::NonZeroU128};

use bitflags::bitflags;
use tigerbeetle_unoff_sys::tb_transfer_t;

use crate::u128_id::U128Id;

use super::{
    HasCode, HasId, HasLedger, HasTimestamp, HasUserDataU128, HasUserDataU32, HasUserDataU64,
};

pub trait HasDebitAccountId {
    fn debit_account_id(&self) -> U128Id;
}

pub trait HasCreditAccountId {
    fn credit_account_id(&self) -> U128Id;
}

pub trait HasTransferAmt {
    fn transfer_amt(&self) -> NonZeroU128;
}

pub trait HasPendingId {
    /// Must be 0 for transfer that are not post-pending or void-pending
    fn pending_id(&self) -> u128;
}

pub trait HasTimeout {
    /// Must be 0 for non-init-two-phase transfers
    fn timeout(&self) -> u32;
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct TransferFlags: u16 {
        const NONE = 0;
        const LINKED = 1 << 0;
        const PENDING = 1 << 1;
        const POST_PENDING_TRANSFER = 1 << 2;
        const VOID_PENDING_TRANSFER = 1 << 3;
        const BALANCING_DEBIT = 1 << 4;
        const BALANCING_CREDIT = 1 << 5;
        const CLOSING_DEBIT = 1 << 6;
        const CLOSING_CREDIT = 1 << 7;
        const IMPORTED = 1 << 8;

    }
}

pub trait HasTransferFlags {
    fn transfer_flags(&self) -> TransferFlags;
}

#[inline]
pub fn transfer_to_create<
    T: Borrow<
        impl HasId
            + HasUserDataU128
            + HasUserDataU64
            + HasUserDataU32
            + HasLedger
            + HasCode
            + HasTimestamp
            + HasTransferFlags
            + HasDebitAccountId
            + HasCreditAccountId
            + HasTransferAmt
            + HasPendingId
            + HasTimeout,
    >,
>(
    trf: T,
) -> tb_transfer_t {
    let trf = trf.borrow();
    tb_transfer_t {
        id: trf.id().into(),
        debit_account_id: trf.debit_account_id().into(),
        credit_account_id: trf.credit_account_id().into(),
        amount: trf.transfer_amt().into(),
        pending_id: trf.pending_id(),
        user_data_128: trf.user_data_128(),
        user_data_64: trf.user_data_64(),
        user_data_32: trf.user_data_32(),
        timeout: trf.timeout(),
        ledger: trf.ledger().into(),
        code: trf.code().into(),
        flags: trf.transfer_flags().bits(),
        timestamp: trf.timestamp(),
    }
}
