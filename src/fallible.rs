use core::convert::TryFrom;
use core::fmt;
use super::BitFlags;
use super::BitFlag;

// Coherence doesn't let us use a generic type here. Work around by implementing
// for each integer type manually.
macro_rules! impl_try_from {
    ($($ty:ty),*) => {
        $(
            impl<T> TryFrom<$ty> for BitFlags<T>
            where
                T: BitFlag<Type=$ty>,
            {
                type Error = FromBitsError<T>;

                fn try_from(bits: T::Type) -> Result<Self, Self::Error> {
                    Self::from_bits(bits)
                }
            }
        )*
    };
}

impl_try_from! {
    u8, u16, u32, u64, usize
}

/// The error struct used by [`BitFlags::from_bits`]
/// and the [`TryFrom`] implementation`
/// for invalid values.
///
/// ```
/// # use std::convert::TryInto;
/// # use enumflags2::{bitflags, BitFlags};
/// #[bitflags]
/// #[derive(Clone, Copy, Debug)]
/// #[repr(u8)]
/// enum MyFlags {
///     A = 0b0001,
///     B = 0b0010,
///     C = 0b0100,
///     D = 0b1000,
/// }
///
/// let result: Result<BitFlags<MyFlags>, _> = 0b10101u8.try_into();
/// assert!(result.is_err());
/// let error = result.unwrap_err();
/// assert_eq!(error.truncate(), MyFlags::C | MyFlags::A);
/// assert_eq!(error.invalid_bits(), 0b10000);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct FromBitsError<T: BitFlag> {
    pub(crate) flags: BitFlags<T>,
    pub(crate) invalid: T::Type,
}

impl<T: BitFlag> FromBitsError<T> {
    /// Return the truncated result of the conversion.
    pub fn truncate(self) -> BitFlags<T> {
        self.flags
    }

    /// Return the bits that didn't correspond to any flags.
    pub fn invalid_bits(self) -> T::Type {
        self.invalid
    }
}

impl<T: BitFlag + fmt::Debug> fmt::Display for FromBitsError<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "Invalid bits for {:?}: {:#b}", self.flags, self.invalid)
    }
}

#[cfg(feature = "std")]
impl<T: BitFlag + fmt::Debug> std::error::Error for FromBitsError<T> {
    fn description(&self) -> &str {
        "invalid bitflags representation"
    }
}
