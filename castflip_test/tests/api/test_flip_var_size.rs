use core::mem::size_of;

use castflip::{Flip, NE, SE, LE, BE};
use crate::{IData1, UData1};


macro_rules! test {
    ( $data:expr, $field:ident, $ty:ty ) => {{
	{
	    let data = $data;

	    let mut se_val_from_ne = data.ne_vals.$field as $ty;
	    let mut ne_val_from_se = data.se_vals.$field as $ty;

	    se_val_from_ne.flip_var_swapped();
	    ne_val_from_se.flip_var_swapped();

	    assert_eq!(se_val_from_ne, data.se_vals.$field as $ty);
	    assert_eq!(ne_val_from_se, data.ne_vals.$field as $ty);
	}
	{
	    let data = $data;

	    let mut ne_val_from_ne = data.ne_vals.$field as $ty;
	    let mut se_val_from_ne = data.ne_vals.$field as $ty;
	    let mut le_val_from_ne = data.ne_vals.$field as $ty;
	    let mut be_val_from_ne = data.ne_vals.$field as $ty;

	    ne_val_from_ne.flip_var(NE);
	    se_val_from_ne.flip_var(SE);
	    le_val_from_ne.flip_var(LE);
	    be_val_from_ne.flip_var(BE);

	    assert_eq!(ne_val_from_ne, data.ne_vals.$field as $ty);
	    assert_eq!(se_val_from_ne, data.se_vals.$field as $ty);
	    assert_eq!(le_val_from_ne, data.le_vals.$field as $ty);
	    assert_eq!(be_val_from_ne, data.be_vals.$field as $ty);
	}
    }}
}


#[test]
fn idata1() {
    let idata1 = IData1::gen();

    match ISIZE_SIZE {
	I32_SIZE  => test!(idata1, val_i32,  isize),
	I64_SIZE  => test!(idata1, val_i64,  isize),
	I128_SIZE => test!(idata1, val_i128, isize),
	_ => panic!(),
    }

    const I32_SIZE: usize = size_of::<i32>();
    const I64_SIZE: usize = size_of::<i64>();
    const I128_SIZE: usize = size_of::<i128>();
    const ISIZE_SIZE: usize = size_of::<isize>();
}

#[test]
fn udata1() {
    let udata1 = UData1::gen();

    match USIZE_SIZE {
	U32_SIZE  => test!(udata1, val_u32,  usize),
	U64_SIZE  => test!(udata1, val_u64,  usize),
	U128_SIZE => test!(udata1, val_u128, usize),
	_ => panic!(),
    }

    const U32_SIZE: usize = size_of::<u32>();
    const U64_SIZE: usize = size_of::<u64>();
    const U128_SIZE: usize = size_of::<u128>();
    const USIZE_SIZE: usize = size_of::<usize>();
}
