use tigerbeetle_unoff_sys::{tb_account_t, tb_transfer_t};

// https://github.com/tigerbeetle/tigerbeetle/blob/a082ff0237d5083f35e70d56c550ce271b6e4bf7/src/clients/c/samples/main.c#L17C1-L18C45
pub const MAX_MSG_BYTES: usize = (1024 * 1024) - 256; // TODO: this size (~1MB) causes stack overflow on default settings, so we're boxing all ResponseBufs for now

pub const MAX_TRANSFERS_PER_MSG: usize = MAX_MSG_BYTES / core::mem::size_of::<tb_transfer_t>();

pub const MAX_ACCOUNTS_PER_MSG: usize = MAX_MSG_BYTES / core::mem::size_of::<tb_account_t>();
