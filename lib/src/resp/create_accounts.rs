use std::ptr::addr_of;

use num_traits::FromPrimitive;
use tigerbeetle_unoff_sys::{
    tb_create_accounts_result_t, TB_CREATE_ACCOUNT_RESULT_TB_CREATE_ACCOUNT_OK,
};

use crate::err::CreateAccountErr;

use super::RespBuf;

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct CreateAccountsResp(pub(crate) RespBuf);

impl CreateAccountsResp {
    #[inline]
    pub(crate) fn from_boxed_respbuf(b: Box<RespBuf>) -> Box<Self> {
        // sound because of repr(transparent)
        unsafe { core::mem::transmute(b) }
    }

    #[inline]
    pub const fn as_slice(&self) -> &[tb_create_accounts_result_t] {
        let len = unsafe { self.0.len.assume_init_read() } as usize
            / core::mem::size_of::<tb_create_accounts_result_t>();
        unsafe { core::slice::from_raw_parts(addr_of!(self.0.bytes).cast(), len) }
    }

    /// Yields Ok(index) for successes, Err((index, error)) for failures
    #[inline]
    pub fn iter_results(&self) -> impl Iterator<Item = Result<u32, (u32, CreateAccountErr)>> + '_ {
        self.as_slice().iter().map(
            |tb_create_accounts_result_t { result, index }| match *result {
                TB_CREATE_ACCOUNT_RESULT_TB_CREATE_ACCOUNT_OK => Ok(*index),
                n => Err((*index, CreateAccountErr::from_u32(n).unwrap())),
            },
        )
    }
}

impl AsRef<[tb_create_accounts_result_t]> for CreateAccountsResp {
    #[inline]
    fn as_ref(&self) -> &[tb_create_accounts_result_t] {
        self.as_slice()
    }
}
