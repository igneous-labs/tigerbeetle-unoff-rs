use std::ptr::null_mut;

use tigerbeetle_unoff_sys::{
    tb_account_filter_t, tb_packet_t, TB_OPERATION_TB_OPERATION_GET_ACCOUNT_TRANSFERS,
    TB_PACKET_STATUS_TB_PACKET_OK,
};

use crate::{err::TbPacketErr, resp::lookup_transfers::LookupTransfersResp, Client};

impl Client {
    /// Caveats:
    /// - those of [`Self::request`] apply
    /// - `filter.limit` must not exceed [`crate::MAX_TRANSFERS_PER_MSG`]
    pub async fn get_account_transfers(
        &self,
        filter: &tb_account_filter_t,
    ) -> Result<LookupTransfersResp, TbPacketErr> {
        let packet = tb_packet_t {
            operation: TB_OPERATION_TB_OPERATION_GET_ACCOUNT_TRANSFERS as u8,
            status: TB_PACKET_STATUS_TB_PACKET_OK as u8,
            data_size: core::mem::size_of_val(filter) as u32,
            // cast-safety: request should not modify data but generated bindings take *mut
            data: (filter as *const tb_account_filter_t).cast_mut().cast(),
            // set by [`Req::poll()`]
            user_data: null_mut(),
            // dont-cares?
            next: null_mut(),
            batch_next: null_mut(),
            batch_tail: null_mut(),
            batch_size: 0,
            batch_allowed: 0,
            reserved: [0u8; 7],
        };
        self.request(packet).await.map(LookupTransfersResp)
    }
}
