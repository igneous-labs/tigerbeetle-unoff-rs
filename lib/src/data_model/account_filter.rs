use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64};

use bitflags::bitflags;
use tigerbeetle_unoff_sys::tb_account_filter_t;

use crate::{consts::MAX_TRANSFERS_PER_MSG, u128_id::U128Id};

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct AccountFilterFlags: u32 {
        const NONE = 0;
        const DEBITS = 1 << 0;
        const CREDITS = 1 << 1;
        const REVERSED = 1 << 2;
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccountFilterTimestamp(NonZeroU64);

impl AccountFilterTimestamp {
    pub const MAX_EXCL: u64 = 2u64.pow(63);

    #[inline]
    pub const fn get_nonzero(self) -> NonZeroU64 {
        self.0
    }

    #[inline]
    pub const fn get(self) -> u64 {
        self.0.get()
    }

    #[inline]
    pub const fn new(n: u64) -> Option<Self> {
        if n >= Self::MAX_EXCL {
            None
        } else {
            match NonZeroU64::new(n) {
                Some(n) => Some(Self(n)),
                None => None,
            }
        }
    }

    /// # Safety
    /// - `n` must not be 0 or `>= Self::MAX_EXCL`
    #[inline]
    pub const unsafe fn new_unchecked(n: u64) -> Self {
        Self(NonZeroU64::new_unchecked(n))
    }

    #[inline]
    pub const fn new_nonzero(n: NonZeroU64) -> Option<Self> {
        if n.get() >= Self::MAX_EXCL {
            None
        } else {
            Some(Self(n))
        }
    }

    /// # Safety
    /// - `n` must not be `>= Self::MAX_EXCL`
    #[inline]
    pub const unsafe fn new_unchecked_nonzero(n: NonZeroU64) -> Self {
        Self(n)
    }
}

impl From<AccountFilterTimestamp> for NonZeroU64 {
    #[inline]
    fn from(value: AccountFilterTimestamp) -> Self {
        value.get_nonzero()
    }
}

impl From<AccountFilterTimestamp> for u64 {
    #[inline]
    fn from(value: AccountFilterTimestamp) -> Self {
        value.get()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AccountFilter {
    pub account_id: U128Id,
    pub user_data_128: Option<NonZeroU128>,
    pub user_data_64: Option<NonZeroU64>,
    pub user_data_32: Option<NonZeroU32>,
    pub code: Option<NonZeroU16>,
    pub timestamp_min: Option<AccountFilterTimestamp>,
    pub timestamp_max: Option<AccountFilterTimestamp>,

    /// Must be 0 < limit <= MAX_TRANSFERS_PER_MSG.
    /// Defaults to MAX_TRANSFERS_PER_MSG if None
    pub limit: Option<NonZeroU32>,

    /// Must either have [`AccountFilterFlags::DEBITS`] or [`AccountFilterFlags::CREDITS`] set,
    /// else query will return empty
    pub flags: AccountFilterFlags,
}

impl AccountFilter {
    #[inline]
    pub const fn new_default(account_id: U128Id) -> Self {
        Self {
            account_id,
            user_data_128: None,
            user_data_64: None,
            user_data_32: None,
            code: None,
            timestamp_min: None,
            timestamp_max: None,
            limit: None,
            flags: AccountFilterFlags::DEBITS.union(AccountFilterFlags::CREDITS),
        }
    }

    /// Sets flags to both debits and credits if [`Self::flags`] contains neither
    #[inline]
    pub const fn to_account_filter_t(self) -> tb_account_filter_t {
        tb_account_filter_t {
            account_id: self.account_id.get(),
            user_data_128: match self.user_data_128 {
                Some(n) => n.get(),
                None => 0,
            },
            user_data_64: match self.user_data_64 {
                Some(n) => n.get(),
                None => 0,
            },
            user_data_32: match self.user_data_32 {
                Some(n) => n.get(),
                None => 0,
            },
            code: match self.code {
                Some(n) => n.get(),
                None => 0,
            },
            reserved: [0; 58],
            timestamp_min: match self.timestamp_min {
                Some(n) => n.get(),
                None => 0,
            },
            timestamp_max: match self.timestamp_max {
                Some(n) => n.get(),
                None => 0,
            },
            limit: match self.limit {
                Some(n) => n.get(),
                None => MAX_TRANSFERS_PER_MSG as u32,
            },
            flags: if !self.flags.contains(AccountFilterFlags::DEBITS)
                && !self.flags.contains(AccountFilterFlags::CREDITS)
            {
                self.flags
                    .union(AccountFilterFlags::DEBITS)
                    .union(AccountFilterFlags::CREDITS)
            } else {
                self.flags
            }
            .bits(),
        }
    }
}

impl From<&AccountFilter> for tb_account_filter_t {
    #[inline]
    fn from(value: &AccountFilter) -> Self {
        value.to_account_filter_t()
    }
}

impl From<AccountFilter> for tb_account_filter_t {
    #[inline]
    fn from(value: AccountFilter) -> Self {
        (&value).into()
    }
}
