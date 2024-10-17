use core::alloc::Layout;
use core::{mem::MaybeUninit, ptr::addr_of_mut};
use std::ptr::{addr_of, null_mut};

pub mod create_accounts;
pub mod create_transfers;
pub mod lookup_accounts;
pub mod lookup_transfers;

const MAX_INLINE_BYTE_LEN: usize = 512;
const MAX_INLINE_BYTE_LEN_U32: u32 = MAX_INLINE_BYTE_LEN as u32;

/// A tigerbeetle DB responses with the bytes either stored inline if size is small (<= [`MAX_INLINE_BYTE_LEN`])
/// or on a heap allocation otherwise
#[repr(C, align(16))] // max-align of possible structs is 16 (tb_account_t). Doing this allows us to just cast pointers
#[derive(Debug)]
pub struct RespBuf {
    // put data first to guarantee 16-byte alignment using repr(C)
    inline_data: [MaybeUninit<u8>; MAX_INLINE_BYTE_LEN],
    ptr: *mut u8,
    byte_len: u32,
}

impl Clone for RespBuf {
    #[inline]
    fn clone(&self) -> Self {
        let slice = self.as_slice();
        Self::from_raw(slice.as_ptr(), self.byte_len)
    }
}

impl Drop for RespBuf {
    #[inline]
    fn drop(&mut self) {
        if !self.is_inline() {
            unsafe {
                std::alloc::dealloc(
                    self.ptr,
                    Layout::from_size_align_unchecked(self.byte_len as usize, 16),
                );
            }
        }
    }
}

impl RespBuf {
    #[inline]
    pub(crate) fn from_raw(data: *const u8, data_size: u32) -> Self {
        let data_size_usize = data_size as usize;
        match data_size {
            0 => Self {
                inline_data: [const { MaybeUninit::uninit() }; MAX_INLINE_BYTE_LEN],
                ptr: null_mut(),
                byte_len: data_size,
            },
            1..=MAX_INLINE_BYTE_LEN_U32 => {
                let mut res = Self {
                    inline_data: [const { MaybeUninit::uninit() }; MAX_INLINE_BYTE_LEN],
                    ptr: null_mut(),
                    byte_len: data_size,
                };
                unsafe {
                    core::ptr::copy_nonoverlapping(
                        data,
                        addr_of_mut!(res.inline_data).cast(),
                        data_size_usize,
                    );
                }
                res
            }
            _ => {
                let ptr = unsafe {
                    std::alloc::alloc(Layout::from_size_align_unchecked(data_size_usize, 16))
                };
                unsafe {
                    core::ptr::copy_nonoverlapping(data, ptr, data_size_usize);
                }
                Self {
                    inline_data: [const { MaybeUninit::uninit() }; MAX_INLINE_BYTE_LEN],
                    ptr,
                    byte_len: data_size,
                }
            }
        }
    }

    #[inline]
    const fn is_inline(&self) -> bool {
        self.byte_len <= MAX_INLINE_BYTE_LEN_U32
    }

    #[inline]
    const fn as_slice(&self) -> &[u8] {
        match self.is_inline() {
            true => unsafe {
                core::slice::from_raw_parts(
                    addr_of!(self.inline_data).cast(),
                    self.byte_len as usize,
                )
            },
            false => unsafe {
                core::slice::from_raw_parts(self.ptr.cast_const(), self.byte_len as usize)
            },
        }
    }
}
