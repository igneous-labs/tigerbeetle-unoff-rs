use std::ptr::addr_of;

use tigerbeetle_unoff_sys::tb_transfer_t;

use super::RespBuf;

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct LookupTransfersResp(pub(crate) RespBuf);

impl LookupTransfersResp {
    #[inline]
    pub(crate) fn from_boxed_respbuf(b: Box<RespBuf>) -> Box<Self> {
        // sound because of repr(transparent)
        unsafe { core::mem::transmute(b) }
    }

    #[inline]
    pub const fn as_slice(&self) -> &[tb_transfer_t] {
        let len = unsafe { self.0.len.assume_init_read() } as usize
            / core::mem::size_of::<tb_transfer_t>();
        unsafe { core::slice::from_raw_parts(addr_of!(self.0.bytes).cast(), len) }
    }
}

impl AsRef<[tb_transfer_t]> for LookupTransfersResp {
    #[inline]
    fn as_ref(&self) -> &[tb_transfer_t] {
        self.as_slice()
    }
}
