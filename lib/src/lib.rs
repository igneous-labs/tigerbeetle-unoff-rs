// Needs std to
// - link properly
// - currently boxing ResponseBufs because 1MB size seems to trigger stack overflow

use core::{mem::MaybeUninit, ptr::addr_of_mut};

use err::TbStatusErr;
use num_traits::FromPrimitive;
use req::on_completion;

// re-export everything in sys
pub use tigerbeetle_unoff_sys::*;

pub mod consts;
pub mod data_model;
pub mod err;
pub mod req;
pub mod resp;
pub mod u128_id;

#[derive(Debug)]
pub struct Client {
    ptr: tb_client_t,
}

impl Drop for Client {
    fn drop(&mut self) {
        unsafe { tb_client_deinit(self.ptr) }
    }
}

impl Client {
    /// Address must be either a valid port number of IPV4:port_number. Examples:
    /// - "3000"
    /// - "127.0.0.1:3000"
    ///
    /// Invalid examples:
    /// - "localhost:3000"
    pub fn init(cluster_id: u128, address: &str) -> Result<Self, TbStatusErr> {
        let mut res: MaybeUninit<Self> = MaybeUninit::uninit();
        let status = unsafe {
            tb_client_init(
                addr_of_mut!((*res.as_mut_ptr()).ptr),
                cluster_id,
                address.as_ptr().cast(),
                address
                    .len()
                    .try_into()
                    .map_err(|_e| TbStatusErr::Unexpected)?,
                0, // null ptr, no need for a global context?
                Some(on_completion),
            )
        };
        match status {
            TB_STATUS_TB_STATUS_SUCCESS => Ok(unsafe { res.assume_init() }),
            status => Err(TbStatusErr::from_u32(status).unwrap()),
        }
    }
}
