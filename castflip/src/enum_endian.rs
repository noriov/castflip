//
// This file defines enum `Endian`.
//

use crate::include_doc;


//
// Enum `Endian`
//
#[doc = include_doc!("enum_endian.md")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Endian {
    ///
    /// Native endianness, which is the endianness of the target system.
    /// - Category: Relative endianness
    /// - Antonym: Swapped endianness ([`Endian::Swapped`])
    ///
    Native,

    ///
    /// Swapped endianness, which is the byte-swapped endianness of
    /// the target system.
    /// - Category: Relative endianness
    /// - Antonym: Native endianness ([`Endian::Native`])
    ///
    Swapped,

    ///
    /// Little endianness, which stores the least significant byte at
    /// the smallest memory address.
    /// - Category: Absolute endianness
    /// - Antonym: Big endianness ([`Endian::Big`])
    ///
    Little,

    ///
    /// Big endianness, which stores the most significant byte at the
    /// smallest memory address.
    /// - Category: Absolute endianness
    /// - Antonym: Little endianness ([`Endian::Little`])
    ///
    Big,
}


impl Endian {
    ///
    /// Returns the relative endianness of `self`.
    ///
    /// ```rust
    /// use castflip::Endian;
    ///
    /// // Relative endianness
    /// assert_eq!(Endian::Native.relative(), Endian::Native);
    /// assert_eq!(Endian::Swapped.relative(), Endian::Swapped);
    ///
    /// // Absolute endianness
    /// if cfg!(target_endian = "little") {
    ///     assert_eq!(Endian::Little.relative(), Endian::Native);
    ///     assert_eq!(Endian::Big.relative(), Endian::Swapped);
    /// } else if cfg!(target_endian = "big") {
    ///     assert_eq!(Endian::Little.relative(), Endian::Swapped);
    ///     assert_eq!(Endian::Big.relative(), Endian::Native);
    /// # } else {
    /// #   panic!();
    /// }
    /// ```
    ///
    pub fn relative(&self) -> Self {
        if cfg!(target_endian = "little") {
            match self {
                Self::Native  => Self::Native,
                Self::Swapped => Self::Swapped,
                Self::Little  => Self::Native,
                Self::Big     => Self::Swapped,
            }
        } else if cfg!(target_endian = "big") {
            match self {
                Self::Native  => Self::Native,
                Self::Swapped => Self::Swapped,
                Self::Little  => Self::Swapped,
                Self::Big     => Self::Native,
            }
        } else {
            panic!("The endian of the target system is not supported.");
        }
    }

    ///
    /// Returns the absolute endianness of `self`.
    ///
    /// ```rust
    /// use castflip::Endian;
    ///
    /// // Relative endianness
    /// if cfg!(target_endian = "little") {
    ///     assert_eq!(Endian::Native.absolute(), Endian::Little);
    ///     assert_eq!(Endian::Swapped.absolute(), Endian::Big);
    /// } else if cfg!(target_endian = "big") {
    ///     assert_eq!(Endian::Native.absolute(), Endian::Big);
    ///     assert_eq!(Endian::Swapped.absolute(), Endian::Little);
    /// # } else {
    /// #   panic!();
    /// }
    ///
    /// // Absolute endianness
    /// assert_eq!(Endian::Little.absolute(), Endian::Little);
    /// assert_eq!(Endian::Big.absolute(), Endian::Big);
    /// ```
    ///
    pub fn absolute(&self) -> Self {
        if cfg!(target_endian = "little") {
            match self {
                Self::Native  => Self::Little,
                Self::Swapped => Self::Big,
                Self::Little  => Self::Little,
                Self::Big     => Self::Big,
            }
        } else if cfg!(target_endian = "big") {
            match self {
                Self::Native  => Self::Big,
                Self::Swapped => Self::Little,
                Self::Little  => Self::Little,
                Self::Big     => Self::Big,
            }
        } else {
            panic!("The endian of the target system is not supported.");
        }
    }

    ///
    /// Returns `true` if `self` is not equivalent to the endianness
    /// of the target system.
    ///
    /// ```rust
    /// use castflip::Endian;
    ///
    /// // Relative endian
    /// assert_eq!(Endian::Native.need_swap(), false);
    /// assert_eq!(Endian::Swapped.need_swap(), true);
    ///
    /// // Absolute endian
    /// if cfg!(target_endian = "little") {
    ///     assert_eq!(Endian::Little.need_swap(), false);
    ///     assert_eq!(Endian::Big.need_swap(), true);
    /// } else if cfg!(target_endian = "big") {
    ///     assert_eq!(Endian::Little.need_swap(), true);
    ///     assert_eq!(Endian::Big.need_swap(), false);
    /// # } else {
    /// #   panic!();
    /// }
    /// ```
    ///
    #[inline]
    pub fn need_swap(self) -> bool {
        if self == Self::Swapped {
            true
        } else {
            if cfg!(target_endian = "little") {
                self == Self::Big
            } else if cfg!(target_endian = "big") {
                self == Self::Little
            } else {
                panic!("The endian of the target system is not supported.");
            }
        }
    }

    ///
    /// Returns the capitalized name of `self`.
    ///
    /// ```rust
    /// use castflip::Endian;
    ///
    /// assert_eq!(Endian::Native.name(), "Native");
    /// assert_eq!(Endian::Swapped.name(), "Swapped");
    /// assert_eq!(Endian::Little.name(), "Little");
    /// assert_eq!(Endian::Big.name(), "Big");
    /// ```
    ///
    pub fn name(self) -> &'static str {
        match self {
            Self::Native  => "Native",
            Self::Swapped => "Swapped",
            Self::Little  => "Little",
            Self::Big     => "Big",
        }
    }
}


/// Is an alias of [`Endian::Native`]. `NE` stands for the Native Endianness.
pub const NE: Endian = Endian::Native;

/// Is an alias of [`Endian::Swapped`]. `SE` stands for the Swapped Endianness.
pub const SE: Endian = Endian::Swapped;

/// Is an alias of [`Endian::Little`]. `LE` stands for the Little Endianness.
pub const LE: Endian = Endian::Little;

/// Is an alias of [`Endian::Big`]. `BE` stands for the Big Endianness.
pub const BE: Endian = Endian::Big;
