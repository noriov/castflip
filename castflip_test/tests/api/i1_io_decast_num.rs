use core::mem::size_of;
use std::io::Cursor;

use castflip::DecastIO;
use crate::{IData1, IVals1};


#[test]
fn idata1() {
    {
	let idata1 = IData1::gen();

	let mut ne_output = Cursor::new(vec![0_u8; size_of::<IVals1>()]);

	ne_output.decast::<i8>(&idata1.ne_vals.val1_i8).unwrap();
	ne_output.decast::<i8>(&idata1.ne_vals.val2_i8).unwrap();
	ne_output.decast::<i16>(&idata1.ne_vals.val_i16).unwrap();
	ne_output.decast::<i32>(&idata1.ne_vals.val_i32).unwrap();
	ne_output.decast::<i64>(&idata1.ne_vals.val_i64).unwrap();
	ne_output.decast::<i128>(&idata1.ne_vals.val_i128).unwrap();

	assert_eq!(ne_output.into_inner(), idata1.raw_bytes);
    }
    {
	let idata1 = IData1::gen();

	let mut ne_output = Cursor::new(vec![0_u8; size_of::<IVals1>()]);

	// The type parameter of decast() can be omitted where
	// the Rust compiler can infer the type of the result.
	ne_output.decast(&idata1.ne_vals.val1_i8).unwrap();
	ne_output.decast(&idata1.ne_vals.val2_i8).unwrap();
	ne_output.decast(&idata1.ne_vals.val_i16).unwrap();
	ne_output.decast(&idata1.ne_vals.val_i32).unwrap();
	ne_output.decast(&idata1.ne_vals.val_i64).unwrap();
	ne_output.decast(&idata1.ne_vals.val_i128).unwrap();

	assert_eq!(ne_output.into_inner(), idata1.raw_bytes);
    }
}
