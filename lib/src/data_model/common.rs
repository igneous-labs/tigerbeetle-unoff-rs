use std::num::{NonZeroU16, NonZeroU32};

use crate::u128_id::U128Id;

pub trait HasId {
    fn id(&self) -> U128Id;
}

pub trait HasUserDataU128 {
    fn user_data_128(&self) -> u128;
}

pub trait HasUserDataU64 {
    fn user_data_64(&self) -> u64;
}

pub trait HasUserDataU32 {
    fn user_data_32(&self) -> u32;
}

pub trait HasLedger {
    fn ledger(&self) -> NonZeroU32;
}

pub trait HasCode {
    fn code(&self) -> NonZeroU16;
}

pub trait HasTimestamp {
    fn timestamp(&self) -> u64;
}

/// Convenience traits to implement for events to create that dont care about user data
pub trait EmptyUserData {}

impl<T: EmptyUserData> HasUserDataU128 for T {
    #[inline]
    fn user_data_128(&self) -> u128 {
        0
    }
}

impl<T: EmptyUserData> HasUserDataU64 for T {
    #[inline]
    fn user_data_64(&self) -> u64 {
        0
    }
}

impl<T: EmptyUserData> HasUserDataU32 for T {
    #[inline]
    fn user_data_32(&self) -> u32 {
        0
    }
}

/// Non-imported events to create must have their timestamp field set to 0
pub trait CreateNativeEventTimestamp {}

impl<T: CreateNativeEventTimestamp> HasTimestamp for T {
    #[inline]
    fn timestamp(&self) -> u64 {
        0
    }
}
