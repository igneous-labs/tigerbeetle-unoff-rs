use tigerbeetle_unoff_sys::tb_account_t;

use super::RespBuf;

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct LookupAccountsResp(pub(crate) RespBuf);

impl LookupAccountsResp {
    #[inline]
    pub const fn as_slice(&self) -> &[tb_account_t] {
        let byte_slice = self.0.as_slice();
        let len = byte_slice.len() / core::mem::size_of::<tb_account_t>();
        unsafe { core::slice::from_raw_parts(byte_slice.as_ptr().cast(), len) }
    }
}

impl AsRef<[tb_account_t]> for LookupAccountsResp {
    #[inline]
    fn as_ref(&self) -> &[tb_account_t] {
        self.as_slice()
    }
}
