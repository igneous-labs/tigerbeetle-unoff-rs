use core::alloc::Layout;
use core::{mem::MaybeUninit, ptr::addr_of_mut};
use std::alloc::alloc;

use crate::consts::MAX_MSG_BYTES;

pub mod create_accounts;
pub mod create_transfers;
pub mod lookup_accounts;
pub mod lookup_transfers;

#[repr(C, align(16))] // max-align of possible structs is 16 (tb_account_t). Doing this allows us to just cast pointers
#[derive(Clone, Copy, Debug)]
pub struct RespBuf {
    bytes: [MaybeUninit<u8>; MAX_MSG_BYTES],
    len: MaybeUninit<u32>,
}

impl RespBuf {
    fn uninit_boxed() -> Box<Self> {
        let layout = Layout::new::<Self>();
        unsafe {
            let ptr = alloc(layout) as *mut Self;
            Box::from_raw(ptr)
        }
    }

    pub(crate) fn from_raw_boxed(data: *const u8, data_size: u32) -> Box<Self> {
        assert!(data_size <= MAX_MSG_BYTES as u32);
        let mut res = Self::uninit_boxed();
        res.len.write(data_size);
        // If request ends prematurely, data = NULL, and copy_nonoverlapping will panic
        if data_size > 0 {
            unsafe {
                // nonoverlapping guarantee: we just allocated res in uninit_boxed()
                core::ptr::copy_nonoverlapping(
                    data,
                    addr_of_mut!(res.bytes).cast(),
                    // as-safety: data_size asserted above
                    data_size as usize,
                );
            }
        }
        res
    }
}
