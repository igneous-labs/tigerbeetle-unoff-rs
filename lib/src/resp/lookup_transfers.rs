use tigerbeetle_unoff_sys::tb_transfer_t;

use super::RespBuf;

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct LookupTransfersResp(pub(crate) RespBuf);

impl LookupTransfersResp {
    #[inline]
    pub const fn as_slice(&self) -> &[tb_transfer_t] {
        let byte_slice = self.0.as_slice();
        let len = byte_slice.len() / core::mem::size_of::<tb_transfer_t>();
        unsafe { core::slice::from_raw_parts(byte_slice.as_ptr().cast(), len) }
    }
}

impl AsRef<[tb_transfer_t]> for LookupTransfersResp {
    #[inline]
    fn as_ref(&self) -> &[tb_transfer_t] {
        self.as_slice()
    }
}
