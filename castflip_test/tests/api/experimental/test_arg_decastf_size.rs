use core::mem::size_of;

use castflip::experimental::DecastArg;
use castflip::{NE, SE, LE, BE};
use crate::{IData1, UData1};


macro_rules! test {
    ( $data:expr, $field:ident, $ty:ty , $start:expr, $end:expr ) => {
	{
	    let data = $data;

	    let mut raw_bytes_from_ne = [0_u8; size_of::<$ty>()];
	    let mut raw_bytes_from_se = [0_u8; size_of::<$ty>()];
	    let mut raw_bytes_from_le = [0_u8; size_of::<$ty>()];
	    let mut raw_bytes_from_be = [0_u8; size_of::<$ty>()];

	    let ne_size = data.ne_vals.$field as $ty;
	    let se_size = data.se_vals.$field as $ty;
	    let le_size = data.le_vals.$field as $ty;
	    let be_size = data.be_vals.$field as $ty;

	    <$ty>::decastf(&mut raw_bytes_from_ne, &ne_size, NE).unwrap();
	    <$ty>::decastf(&mut raw_bytes_from_se, &se_size, SE).unwrap();
	    <$ty>::decastf(&mut raw_bytes_from_le, &le_size, LE).unwrap();
	    <$ty>::decastf(&mut raw_bytes_from_be, &be_size, BE).unwrap();

	    assert_eq!(raw_bytes_from_ne, data.raw_bytes[$start .. $end]);
	    assert_eq!(raw_bytes_from_se, data.raw_bytes[$start .. $end]);
	    assert_eq!(raw_bytes_from_le, data.raw_bytes[$start .. $end]);
	    assert_eq!(raw_bytes_from_be, data.raw_bytes[$start .. $end]);
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
