/// Generated with scripts/gen_u32_err_enum.py status. DO NOT MODIFY MANUALLY.
use num_derive::{FromPrimitive, ToPrimitive};
use tigerbeetle_unoff_sys::*;

#[repr(u32)]
#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum TbStatusErr {
    Unexpected = TB_STATUS_TB_STATUS_UNEXPECTED,
    OutOfMemory = TB_STATUS_TB_STATUS_OUT_OF_MEMORY,
    AddressInvalid = TB_STATUS_TB_STATUS_ADDRESS_INVALID,
    AddressLimitExceeded = TB_STATUS_TB_STATUS_ADDRESS_LIMIT_EXCEEDED,
    SystemResources = TB_STATUS_TB_STATUS_SYSTEM_RESOURCES,
    NetworkSubsystem = TB_STATUS_TB_STATUS_NETWORK_SUBSYSTEM,
}
