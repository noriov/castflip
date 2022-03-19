use crate::{Endian, Flip};


///
/// Defines methods to flip the endianness of variables in slices and
/// tuples.
///
/// Note: In this crate, the term `encast` means decoding a number of
/// bytes to one or more values, the term `decast` means encoding one
/// or more variables to a number of bytes, and the term `endian-flip`
/// means flipping the endianness of value(s).
///
/// # Example
///
/// In the example below, a vector and a tuple are `endian-flip`ped by
/// using methods defined in `FlipUnsized`.
///
/// ```
/// # fn main() { test(); }
/// # fn test() -> Option<()> {
/// use castflip::experimental::FlipUnsized;
/// use castflip::SE;
///
/// // Flip the endianness of a vector.
/// let mut vec1 = vec![0xFEDC_u16, 0xBA98, 0x7654, 0x3210];
/// vec1.flip_var(SE);
/// assert_eq!(vec1, vec![0xDCFE_u16, 0x98BA, 0x5476, 0x1032]);
///
/// // Flip the endianness of a tuple.
/// let mut tuple2 = (0xFEDC_u16, 0xBA98_u16, 0x7654_u16, 0x3210_u16);
/// tuple2.flip_var(SE);
/// assert_eq!(tuple2, (0xDCFE_u16, 0x98BA_u16, 0x5476_u16, 0x1032_u16));
/// # Some(())
/// # }
/// ```
///
/// In the example above, method `flip_var` flips the endianness of
/// `self` (e.g., `vec1` and `tuple2`).  [`Flip`] must be implemented
/// for all the elements.
///
pub trait FlipUnsized {
    /// Flips the endianness of the variable (`self`).
    fn flip_var_swapped(&mut self);

    /// Flips the endianness of the variable (`self`) if `endian` is
    /// different from the one on the target system.
    fn flip_var(&mut self, endian: Endian) {
	if endian.need_swap() {
	    self.flip_var_swapped();
	}
    }
}


impl<T> FlipUnsized for [T]
where
    T: Flip
{
    fn flip_var_swapped(&mut self) {
	for elem in self {
	    elem.flip_var_swapped();
	}
    }
}


impl FlipUnsized for ()
{
    fn flip_var_swapped(&mut self) {
    }
}

impl<T0> FlipUnsized for (T0,)
where
    T0: Flip,
{
    fn flip_var_swapped(&mut self) {
	self.0.flip_var_swapped();
    }
}

impl<T0, T1> FlipUnsized for (T0, T1)
where
    T0: Flip,
    T1: Flip,
{
    fn flip_var_swapped(&mut self) {
	self.0.flip_var_swapped();
	self.1.flip_var_swapped();
    }
}

impl<T0, T1, T2> FlipUnsized for (T0, T1, T2)
where
    T0: Flip,
    T1: Flip,
    T2: Flip,
{
    fn flip_var_swapped(&mut self) {
	self.0.flip_var_swapped();
	self.1.flip_var_swapped();
	self.2.flip_var_swapped();
    }
}

impl<T0, T1, T2, T3> FlipUnsized for (T0, T1, T2, T3)
where
    T0: Flip,
    T1: Flip,
    T2: Flip,
    T3: Flip,
{
    fn flip_var_swapped(&mut self) {
	self.0.flip_var_swapped();
	self.1.flip_var_swapped();
	self.2.flip_var_swapped();
	self.3.flip_var_swapped();
    }
}

impl<T0, T1, T2, T3, T4> FlipUnsized for (T0, T1, T2, T3, T4)
where
    T0: Flip,
    T1: Flip,
    T2: Flip,
    T3: Flip,
    T4: Flip,
{
    fn flip_var_swapped(&mut self) {
	self.0.flip_var_swapped();
	self.1.flip_var_swapped();
	self.2.flip_var_swapped();
	self.3.flip_var_swapped();
	self.4.flip_var_swapped();
    }
}

impl<T0, T1, T2, T3, T4, T5> FlipUnsized for (T0, T1, T2, T3, T4, T5)
where
    T0: Flip,
    T1: Flip,
    T2: Flip,
    T3: Flip,
    T4: Flip,
    T5: Flip,
{
    fn flip_var_swapped(&mut self) {
	self.0.flip_var_swapped();
	self.1.flip_var_swapped();
	self.2.flip_var_swapped();
	self.3.flip_var_swapped();
	self.4.flip_var_swapped();
	self.5.flip_var_swapped();
    }
}

impl<T0, T1, T2, T3, T4, T5, T6> FlipUnsized for (T0, T1, T2, T3, T4, T5, T6)
where
    T0: Flip,
    T1: Flip,
    T2: Flip,
    T3: Flip,
    T4: Flip,
    T5: Flip,
    T6: Flip,
{
    fn flip_var_swapped(&mut self) {
	self.0.flip_var_swapped();
	self.1.flip_var_swapped();
	self.2.flip_var_swapped();
	self.3.flip_var_swapped();
	self.4.flip_var_swapped();
	self.5.flip_var_swapped();
	self.6.flip_var_swapped();
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7> FlipUnsized
    for (T0, T1, T2, T3, T4, T5, T6, T7)
where
    T0: Flip,
    T1: Flip,
    T2: Flip,
    T3: Flip,
    T4: Flip,
    T5: Flip,
    T6: Flip,
    T7: Flip,
{
    fn flip_var_swapped(&mut self) {
	self.0.flip_var_swapped();
	self.1.flip_var_swapped();
	self.2.flip_var_swapped();
	self.3.flip_var_swapped();
	self.4.flip_var_swapped();
	self.5.flip_var_swapped();
	self.6.flip_var_swapped();
	self.7.flip_var_swapped();
    }
}
