use core::mem::size_of;
use std::io::Cursor;

use castflip::{EncastIO, NE, SE, LE, BE};
use crate::{IData1, UData1};


macro_rules! test {
    ( $data:expr, $field:ident, $ty:ty , $start:expr, $end:expr ) => {{
	{
	    let data = $data;

	    let mut raw_input1 =
		Cursor::new(data.raw_bytes[$start .. $end].to_vec());
	    let mut raw_input2 =
		Cursor::new(data.raw_bytes[$start .. $end].to_vec());
	    let mut raw_input3 =
		Cursor::new(data.raw_bytes[$start .. $end].to_vec());
	    let mut raw_input4 =
		Cursor::new(data.raw_bytes[$start .. $end].to_vec());

	    let ne_size = raw_input1.encastf::<$ty>(NE).unwrap();
	    let se_size = raw_input2.encastf::<$ty>(SE).unwrap();
	    let le_size = raw_input3.encastf::<$ty>(LE).unwrap();
	    let be_size = raw_input4.encastf::<$ty>(BE).unwrap();

	    assert_eq!(ne_size, data.ne_vals.$field as $ty);
	    assert_eq!(se_size, data.se_vals.$field as $ty);
	    assert_eq!(le_size, data.le_vals.$field as $ty);
	    assert_eq!(be_size, data.be_vals.$field as $ty);
	}
	{
	    let data = $data;

	    let mut raw_input1 =
		Cursor::new(data.raw_bytes[$start .. $end].to_vec());
	    let mut raw_input2 =
		Cursor::new(data.raw_bytes[$start .. $end].to_vec());
	    let mut raw_input3 =
		Cursor::new(data.raw_bytes[$start .. $end].to_vec());
	    let mut raw_input4 =
		Cursor::new(data.raw_bytes[$start .. $end].to_vec());

	    // The type parameter of encastf() can be omitted where
	    // the Rust compiler can infer the type of the result.
	    let ne_size: $ty = raw_input1.encastf(NE).unwrap();
	    let se_size: $ty = raw_input2.encastf(SE).unwrap();
	    let le_size: $ty = raw_input3.encastf(LE).unwrap();
	    let be_size: $ty = raw_input4.encastf(BE).unwrap();

	    assert_eq!(ne_size, data.ne_vals.$field as $ty);
	    assert_eq!(se_size, data.se_vals.$field as $ty);
	    assert_eq!(le_size, data.le_vals.$field as $ty);
	    assert_eq!(be_size, data.be_vals.$field as $ty);
	}
    }}
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
