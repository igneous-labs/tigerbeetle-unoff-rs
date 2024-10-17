use std::num::NonZeroU128;

/// Tigerbeetle has many u128 ID types that are enforced to
/// not == 0 and not == u128::MAX
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct U128Id(NonZeroU128);

impl U128Id {
    #[inline]
    pub const fn get_nonzero(self) -> NonZeroU128 {
        self.0
    }

    #[inline]
    pub const fn get(self) -> u128 {
        self.0.get()
    }

    #[inline]
    pub const fn new(n: u128) -> Option<Self> {
        if n == u128::MAX {
            None
        } else {
            match NonZeroU128::new(n) {
                Some(n) => Some(Self(n)),
                None => None,
            }
        }
    }

    /// # Safety
    /// - `n` must not be 0 or `u128::MAX`
    #[inline]
    pub const unsafe fn new_unchecked(n: u128) -> Self {
        Self(NonZeroU128::new_unchecked(n))
    }

    #[inline]
    pub const fn new_nonzero(n: NonZeroU128) -> Option<Self> {
        if n.get() == u128::MAX {
            None
        } else {
            Some(Self(n))
        }
    }

    /// # Safety
    /// - `n` must not be `u128::MAX`
    #[inline]
    pub const unsafe fn new_unchecked_nonzero(n: NonZeroU128) -> Self {
        Self(n)
    }
}

impl From<U128Id> for NonZeroU128 {
    #[inline]
    fn from(value: U128Id) -> Self {
        value.get_nonzero()
    }
}

impl From<U128Id> for u128 {
    #[inline]
    fn from(value: U128Id) -> Self {
        value.get()
    }
}
