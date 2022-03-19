use core::mem::size_of;
use std::io::Cursor;

use castflip::{DecastIO, NE, SE, LE, BE};
use crate::{FData1, FVals1};


#[test]
fn fdata1() {
    {
	let fdata1 = FData1::gen();

	let mut ne_output = Cursor::new(vec![0_u8; size_of::<FVals1>()]);
	let mut se_output = Cursor::new(vec![0_u8; size_of::<FVals1>()]);
	let mut le_output = Cursor::new(vec![0_u8; size_of::<FVals1>()]);
	let mut be_output = Cursor::new(vec![0_u8; size_of::<FVals1>()]);

	ne_output.decastf::<f32>(&fdata1.ne_vals.val1_f32, NE).unwrap();
	ne_output.decastf::<f32>(&fdata1.ne_vals.val2_f32, NE).unwrap();
	ne_output.decastf::<f64>(&fdata1.ne_vals.val1_f64, NE).unwrap();
	ne_output.decastf::<f64>(&fdata1.ne_vals.val2_f64, NE).unwrap();

	se_output.decastf::<f32>(&fdata1.se_vals.val1_f32, SE).unwrap();
	se_output.decastf::<f32>(&fdata1.se_vals.val2_f32, SE).unwrap();
	se_output.decastf::<f64>(&fdata1.se_vals.val1_f64, SE).unwrap();
	se_output.decastf::<f64>(&fdata1.se_vals.val2_f64, SE).unwrap();

	le_output.decastf::<f32>(&fdata1.le_vals.val1_f32, LE).unwrap();
	le_output.decastf::<f32>(&fdata1.le_vals.val2_f32, LE).unwrap();
	le_output.decastf::<f64>(&fdata1.le_vals.val1_f64, LE).unwrap();
	le_output.decastf::<f64>(&fdata1.le_vals.val2_f64, LE).unwrap();

	be_output.decastf::<f32>(&fdata1.be_vals.val1_f32, BE).unwrap();
	be_output.decastf::<f32>(&fdata1.be_vals.val2_f32, BE).unwrap();
	be_output.decastf::<f64>(&fdata1.be_vals.val1_f64, BE).unwrap();
	be_output.decastf::<f64>(&fdata1.be_vals.val2_f64, BE).unwrap();

	assert_eq!(ne_output.into_inner(), fdata1.raw_bytes);
	assert_eq!(se_output.into_inner(), fdata1.raw_bytes);
	assert_eq!(le_output.into_inner(), fdata1.raw_bytes);
	assert_eq!(be_output.into_inner(), fdata1.raw_bytes);
    }
    {
	let fdata1 = FData1::gen();

	let mut ne_output = Cursor::new(vec![0_u8; size_of::<FVals1>()]);
	let mut se_output = Cursor::new(vec![0_u8; size_of::<FVals1>()]);
	let mut le_output = Cursor::new(vec![0_u8; size_of::<FVals1>()]);
	let mut be_output = Cursor::new(vec![0_u8; size_of::<FVals1>()]);

	// The type parameter of decastf() can be omitted where
	// the Rust compiler can infer the type of the result.

	ne_output.decastf(&fdata1.ne_vals.val1_f32, NE).unwrap();
	ne_output.decastf(&fdata1.ne_vals.val2_f32, NE).unwrap();
	ne_output.decastf(&fdata1.ne_vals.val1_f64, NE).unwrap();
	ne_output.decastf(&fdata1.ne_vals.val2_f64, NE).unwrap();

	se_output.decastf(&fdata1.se_vals.val1_f32, SE).unwrap();
	se_output.decastf(&fdata1.se_vals.val2_f32, SE).unwrap();
	se_output.decastf(&fdata1.se_vals.val1_f64, SE).unwrap();
	se_output.decastf(&fdata1.se_vals.val2_f64, SE).unwrap();

	le_output.decastf(&fdata1.le_vals.val1_f32, LE).unwrap();
	le_output.decastf(&fdata1.le_vals.val2_f32, LE).unwrap();
	le_output.decastf(&fdata1.le_vals.val1_f64, LE).unwrap();
	le_output.decastf(&fdata1.le_vals.val2_f64, LE).unwrap();

	be_output.decastf(&fdata1.be_vals.val1_f32, BE).unwrap();
	be_output.decastf(&fdata1.be_vals.val2_f32, BE).unwrap();
	be_output.decastf(&fdata1.be_vals.val1_f64, BE).unwrap();
	be_output.decastf(&fdata1.be_vals.val2_f64, BE).unwrap();

	assert_eq!(ne_output.into_inner(), fdata1.raw_bytes);
	assert_eq!(se_output.into_inner(), fdata1.raw_bytes);
	assert_eq!(le_output.into_inner(), fdata1.raw_bytes);
	assert_eq!(be_output.into_inner(), fdata1.raw_bytes);
    }
}
