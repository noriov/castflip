///
/// Defines two types of endianness: relative endian (Native or
/// Swapped) and absolute endian (Little or Big).
///
/// Note: In this crate, the term `encast` means decoding a number of
/// bytes to one or more values, the term `decast` means encoding one
/// or more variables to a number of bytes, and the term `endian-flip`
/// means flipping the endianness of value(s).
///
/// # Description
///
/// It defines the following two types of endianness.
///
/// - Relative endian ([`Native`] or  [`Swapped`])
/// - Absolute endian ([`Little`] or  [`Big`])
///
/// Relative endinan is useful when encoding or decoding binaries
/// whose endiannesses are specified by some magic numbers indicating
/// whether native-endian or swapped-endian (e.g., [Mach-o], [PcapFile]).
/// Absolute endian is useful when encoding or decoding binaries whose
/// endiannesses are explicitly specified in some fields or in their
/// specification as little-endian or big-endian (e.g., [ELF], [TCP/IP]).
///
/// For convenience, their short aliases are exported as
/// [`NE`], [`SE`], [`LE`] and [`BE`], respectively.
///
/// The purpose of enum `Endian` is to specify the endianness of the
/// bytes to be `encast`ed or `decast`ed in the arguments of methods.
///
/// # Example
///
/// Enum `Endian` has four methods.  Here are examples of two of them.
///
/// ```
/// use castflip::Endian;
///
/// let abs_endian = Endian::Native.absolute();
/// let abs_name = abs_endian.name();
///
/// if cfg!(target_endian = "little") {
///     assert_eq!(abs_endian, Endian::Little);
///     assert_eq!(abs_name, "Little");
/// } else if cfg!(target_endian = "big") {
///     assert_eq!(abs_endian, Endian::Big);
///     assert_eq!(abs_name, "Big");
/// # } else {
/// #   panic!();
/// }
/// ```
///
/// [`Native`]: #variant.Native
/// [`Swapped`]: #variant.Swapped
/// [`Little`]: #variant.Little
/// [`Big`]: #variant.Big
///
/// [Mach-o]: https://en.wikipedia.org/wiki/Mach-O
/// [PcapFile]: https://wiki.wireshark.org/Development/LibpcapFileFormat
/// [ELF]: https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
/// [TCP/IP]: https://en.wikipedia.org/wiki/Internet_protocol_suite
///
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Endian {
    /// Native-Endian.
    /// - The byte-order is the same as the one on the target system.
    /// - Values can be processed without byte-swapping on the target system.
    /// - Its antonym is `Swapped`.
    Native,

    /// Swapped-Endian.
    /// - The byte-order is different from the one on the target system.
    /// - Values needs to be byte-swapped before and after processing them
    ///   on the target system.
    /// - Its antonym is `Native`.
    Swapped,

    /// Little-Endian.
    /// - Values are stored as the least significant byte comes first.
    /// - The need for byte-swapping depends on the endianness of
    ///   the target system.
    /// - Its antonym is `Big`.
    Little,

    /// Big-Endian.
    /// - Values are stored as the most significant byte comes first.
    /// - The need for byte-swapping depends on the endianness of
    ///   the target system.
    /// - Its antonym is `Little`.
    Big,
}


impl Endian {
    /// Returns the corresponding relative endianness.
    ///
    /// ```
    /// use castflip::Endian;
    ///
    /// // Relative endian
    /// assert_eq!(Endian::Native.relative(), Endian::Native);
    /// assert_eq!(Endian::Swapped.relative(), Endian::Swapped);
    ///
    /// // Absolute endian
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
    pub fn relative(&self) -> Endian {
	#[cfg(target_endian = "little")]
	match self {
	    Self::Native  => Self::Native,
	    Self::Swapped => Self::Swapped,
	    Self::Little  => Self::Native,
	    Self::Big     => Self::Swapped,
	}

	#[cfg(target_endian = "big")]
	match self {
	    Self::Native  => Self::Native,
	    Self::Swapped => Self::Swapped,
	    Self::Little  => Self::Swapped,
	    Self::Big     => Self::Native,
	}
    }

    /// Returns the corresponding absolute endianness.
    ///
    /// ```
    /// use castflip::Endian;
    ///
    /// // Relative endian
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
    /// // Absolute endian
    /// assert_eq!(Endian::Little.absolute(), Endian::Little);
    /// assert_eq!(Endian::Big.absolute(), Endian::Big);
    /// ```
    ///
    pub fn absolute(&self) -> Endian {
	#[cfg(target_endian = "little")]
	match self {
	    Self::Native  => Self::Little,
	    Self::Swapped => Self::Big,
	    Self::Little  => Self::Little,
	    Self::Big     => Self::Big,
	}

	#[cfg(target_endian = "big")]
	match self {
	    Self::Native  => Self::Big,
	    Self::Swapped => Self::Little,
	    Self::Little  => Self::Little,
	    Self::Big     => Self::Big,
	}
    }

    /// Returns `true` if the endianness (self) is different from the
    /// one on the target system.
    ///
    /// ```
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
    pub fn need_swap(self) -> bool {
	#[cfg(target_endian = "little")]
	return self == Self::Swapped || self == Self::Big;

	#[cfg(target_endian = "big")]
	return self == Self::Swapped || self == Self::Little;
    }

    /// Returns the name of the endianness (self).
    ///
    /// ```
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
	    Self::Native	=> "Native",
	    Self::Swapped	=> "Swapped",
	    Self::Little	=> "Little",
	    Self::Big		=> "Big",
	}
    }
}


/// An alias of [`Endian::Native`], which means Native-Endian.
pub const NE: Endian = Endian::Native;

/// An alias of [`Endian::Swapped`], which means Swapped-Endian.
pub const SE: Endian = Endian::Swapped;

/// An alias of [`Endian::Little`], which means Little-Endian.
pub const LE: Endian = Endian::Little;

/// An alias of [`Endian::Big`], which means Big-Endian.
pub const BE: Endian = Endian::Big;
