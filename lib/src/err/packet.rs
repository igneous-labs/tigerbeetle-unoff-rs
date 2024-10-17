/// Generated with scripts/gen_u32_err_enum.py packet. DO NOT MODIFY MANUALLY.
use num_derive::{FromPrimitive, ToPrimitive};
use tigerbeetle_unoff_sys::*;

#[repr(u32)]
#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum TbPacketErr {
    TooMuchData = TB_PACKET_STATUS_TB_PACKET_TOO_MUCH_DATA,
    ClientShutdown = TB_PACKET_STATUS_TB_PACKET_CLIENT_SHUTDOWN,
    InvalidOperation = TB_PACKET_STATUS_TB_PACKET_INVALID_OPERATION,
    InvalidDataSize = TB_PACKET_STATUS_TB_PACKET_INVALID_DATA_SIZE,
}
