use core::mem::size_of;

use castflip::{EncastMem, NE, SE, LE, BE};
use crate::{IData1, UData1};


macro_rules! test {
    ( $data:expr, $field:ident, $ty:ty , $start:expr, $end:expr ) => {
	{
	    let data = $data;

	    let ne_size_from_ne =
		data.ne_bytes[$start .. $end].encastf::<$ty>(NE).unwrap();
	    let ne_size_from_se =
		data.se_bytes[$start .. $end].encastf::<$ty>(SE).unwrap();
	    let ne_size_from_le =
		data.le_bytes[$start .. $end].encastf::<$ty>(LE).unwrap();
	    let ne_size_from_be =
		data.be_bytes[$start .. $end].encastf::<$ty>(BE).unwrap();

	    assert_eq!(ne_size_from_ne, data.ne_vals.$field as $ty);
	    assert_eq!(ne_size_from_se, data.ne_vals.$field as $ty);
	    assert_eq!(ne_size_from_le, data.ne_vals.$field as $ty);
	    assert_eq!(ne_size_from_be, data.ne_vals.$field as $ty);
	}
    }
}


#[test]
fn idata1() {
    let idata1 = IData1::gen();

    match ISIZE_SIZE {
	I32_SIZE  => test!(idata1, val_i32,  isize,  4,  8),
	I64_SIZE  => test!(idata1, val_i64,  isize,  8, 16),
	I128_SIZE => test!(idata1, val_i128, isize, 16, 32),
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
	U32_SIZE  => test!(udata1, val_u32,  usize,  4,  8),
	U64_SIZE  => test!(udata1, val_u64,  usize,  8, 16),
	U128_SIZE => test!(udata1, val_u128, usize, 16, 32),
	_ => panic!(),
    }

    const U32_SIZE: usize = size_of::<u32>();
    const U64_SIZE: usize = size_of::<u64>();
    const U128_SIZE: usize = size_of::<u128>();
    const USIZE_SIZE: usize = size_of::<usize>();
}
