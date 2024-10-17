use std::ptr::null_mut;

use tigerbeetle_unoff_sys::{
    tb_account_t, tb_packet_t, TB_OPERATION_TB_OPERATION_CREATE_ACCOUNTS,
    TB_PACKET_STATUS_TB_PACKET_OK,
};

use crate::{
    consts::MAX_ACCOUNTS_PER_MSG, err::TbPacketErr, resp::create_accounts::CreateAccountsResp,
    Client,
};

impl Client {
    /// Caveats:
    /// - those of [`Self::request`] apply
    /// - `accounts.len()` must not exceed [`MAX_ACCOUNTS_PER_MSG`]
    pub async fn create_accounts(
        &self,
        accounts: &[tb_account_t],
    ) -> Result<CreateAccountsResp, TbPacketErr> {
        assert!(accounts.len() <= MAX_ACCOUNTS_PER_MSG);
        let packet = tb_packet_t {
            operation: TB_OPERATION_TB_OPERATION_CREATE_ACCOUNTS as u8,
            status: TB_PACKET_STATUS_TB_PACKET_OK as u8,
            data_size: core::mem::size_of_val(accounts) as u32,
            // cast-safety: request should not modify data but generated bindings take *mut
            data: accounts.as_ptr().cast_mut().cast(),
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
        self.request(packet).await.map(CreateAccountsResp)
    }
}
