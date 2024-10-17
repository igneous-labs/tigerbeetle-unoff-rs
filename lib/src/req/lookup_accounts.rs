use core::ptr::null_mut;

use tigerbeetle_unoff_sys::{
    tb_packet_t, TB_OPERATION_TB_OPERATION_LOOKUP_ACCOUNTS, TB_PACKET_STATUS_TB_PACKET_OK,
};

use crate::{
    consts::MAX_ACCOUNTS_PER_MSG, err::TbPacketErr, resp::lookup_accounts::LookupAccountsResp,
    Client,
};

impl Client {
    /// Caveats:
    /// - those of [`Self::request`] apply
    /// - `accounts.len()` must not exceed [`MAX_ACCOUNTS_PER_MSG`]
    pub async fn lookup_accounts(&self, ids: &[u128]) -> Result<LookupAccountsResp, TbPacketErr> {
        assert!(ids.len() <= MAX_ACCOUNTS_PER_MSG);
        let packet = tb_packet_t {
            operation: TB_OPERATION_TB_OPERATION_LOOKUP_ACCOUNTS as u8,
            status: TB_PACKET_STATUS_TB_PACKET_OK as u8,
            data_size: core::mem::size_of_val(ids) as u32,
            // cast-safety: request should not modify data but generated bindings take *mut
            data: ids.as_ptr().cast_mut().cast(),
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
        self.request(packet).await.map(LookupAccountsResp)
    }
}
