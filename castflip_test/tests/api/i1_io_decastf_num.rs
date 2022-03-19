use core::mem::size_of;
use std::io::Cursor;

use castflip::{DecastIO, NE, SE, LE, BE};
use crate::{IData1, IVals1};


#[test]
fn idata1() {
    {
	let idata1 = IData1::gen();

	let mut ne_output = Cursor::new(vec![0_u8; size_of::<IVals1>()]);
	let mut se_output = Cursor::new(vec![0_u8; size_of::<IVals1>()]);
	let mut le_output = Cursor::new(vec![0_u8; size_of::<IVals1>()]);
	let mut be_output = Cursor::new(vec![0_u8; size_of::<IVals1>()]);

	ne_output.decastf::<i8>(&idata1.ne_vals.val1_i8, NE).unwrap();
	ne_output.decastf::<i8>(&idata1.ne_vals.val2_i8, NE).unwrap();
	ne_output.decastf::<i16>(&idata1.ne_vals.val_i16, NE).unwrap();
	ne_output.decastf::<i32>(&idata1.ne_vals.val_i32, NE).unwrap();
	ne_output.decastf::<i64>(&idata1.ne_vals.val_i64, NE).unwrap();
	ne_output.decastf::<i128>(&idata1.ne_vals.val_i128, NE).unwrap();

	se_output.decastf::<i8>(&idata1.se_vals.val1_i8, SE).unwrap();
	se_output.decastf::<i8>(&idata1.se_vals.val2_i8, SE).unwrap();
	se_output.decastf::<i16>(&idata1.se_vals.val_i16, SE).unwrap();
	se_output.decastf::<i32>(&idata1.se_vals.val_i32, SE).unwrap();
	se_output.decastf::<i64>(&idata1.se_vals.val_i64, SE).unwrap();
	se_output.decastf::<i128>(&idata1.se_vals.val_i128, SE).unwrap();

	le_output.decastf::<i8>(&idata1.le_vals.val1_i8, LE).unwrap();
	le_output.decastf::<i8>(&idata1.le_vals.val2_i8, LE).unwrap();
	le_output.decastf::<i16>(&idata1.le_vals.val_i16, LE).unwrap();
	le_output.decastf::<i32>(&idata1.le_vals.val_i32, LE).unwrap();
	le_output.decastf::<i64>(&idata1.le_vals.val_i64, LE).unwrap();
	le_output.decastf::<i128>(&idata1.le_vals.val_i128, LE).unwrap();

	be_output.decastf::<i8>(&idata1.be_vals.val1_i8, BE).unwrap();
	be_output.decastf::<i8>(&idata1.be_vals.val2_i8, BE).unwrap();
	be_output.decastf::<i16>(&idata1.be_vals.val_i16, BE).unwrap();
	be_output.decastf::<i32>(&idata1.be_vals.val_i32, BE).unwrap();
	be_output.decastf::<i64>(&idata1.be_vals.val_i64, BE).unwrap();
	be_output.decastf::<i128>(&idata1.be_vals.val_i128, BE).unwrap();

	assert_eq!(ne_output.into_inner(), idata1.raw_bytes);
	assert_eq!(se_output.into_inner(), idata1.raw_bytes);
	assert_eq!(le_output.into_inner(), idata1.raw_bytes);
	assert_eq!(be_output.into_inner(), idata1.raw_bytes);
    }
    {
	let idata1 = IData1::gen();

	let mut ne_output = Cursor::new(vec![0_u8; size_of::<IVals1>()]);
	let mut se_output = Cursor::new(vec![0_u8; size_of::<IVals1>()]);
	let mut le_output = Cursor::new(vec![0_u8; size_of::<IVals1>()]);
	let mut be_output = Cursor::new(vec![0_u8; size_of::<IVals1>()]);

	// The type parameter of decastf() can be omitted where
	// the Rust compiler can infer the type of the result.

	ne_output.decastf(&idata1.ne_vals.val1_i8, NE).unwrap();
	ne_output.decastf(&idata1.ne_vals.val2_i8, NE).unwrap();
	ne_output.decastf(&idata1.ne_vals.val_i16, NE).unwrap();
	ne_output.decastf(&idata1.ne_vals.val_i32, NE).unwrap();
	ne_output.decastf(&idata1.ne_vals.val_i64, NE).unwrap();
	ne_output.decastf(&idata1.ne_vals.val_i128, NE).unwrap();

	se_output.decastf(&idata1.se_vals.val1_i8, SE).unwrap();
	se_output.decastf(&idata1.se_vals.val2_i8, SE).unwrap();
	se_output.decastf(&idata1.se_vals.val_i16, SE).unwrap();
	se_output.decastf(&idata1.se_vals.val_i32, SE).unwrap();
	se_output.decastf(&idata1.se_vals.val_i64, SE).unwrap();
	se_output.decastf(&idata1.se_vals.val_i128, SE).unwrap();

	le_output.decastf(&idata1.le_vals.val1_i8, LE).unwrap();
	le_output.decastf(&idata1.le_vals.val2_i8, LE).unwrap();
	le_output.decastf(&idata1.le_vals.val_i16, LE).unwrap();
	le_output.decastf(&idata1.le_vals.val_i32, LE).unwrap();
	le_output.decastf(&idata1.le_vals.val_i64, LE).unwrap();
	le_output.decastf(&idata1.le_vals.val_i128, LE).unwrap();

	be_output.decastf(&idata1.be_vals.val1_i8, BE).unwrap();
	be_output.decastf(&idata1.be_vals.val2_i8, BE).unwrap();
	be_output.decastf(&idata1.be_vals.val_i16, BE).unwrap();
	be_output.decastf(&idata1.be_vals.val_i32, BE).unwrap();
	be_output.decastf(&idata1.be_vals.val_i64, BE).unwrap();
	be_output.decastf(&idata1.be_vals.val_i128, BE).unwrap();

	assert_eq!(ne_output.into_inner(), idata1.raw_bytes);
	assert_eq!(se_output.into_inner(), idata1.raw_bytes);
	assert_eq!(le_output.into_inner(), idata1.raw_bytes);
	assert_eq!(be_output.into_inner(), idata1.raw_bytes);
    }
}
