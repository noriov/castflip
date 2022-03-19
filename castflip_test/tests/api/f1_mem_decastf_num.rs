use core::mem::size_of;

use castflip::{DecastMem, NE, SE, LE, BE};
use crate::{FData1, FVals1};


#[test]
fn fdata1() {
    {
	let fdata1 = FData1::gen();

	let mut raw_bytes_from_ne = [0_u8; size_of::<FVals1>()];
	let mut raw_bytes_from_se = [0_u8; size_of::<FVals1>()];
	let mut raw_bytes_from_le = [0_u8; size_of::<FVals1>()];
	let mut raw_bytes_from_be = [0_u8; size_of::<FVals1>()];

	raw_bytes_from_ne[ 0 ..  4]
	    .decastf::<f32>(&fdata1.ne_vals.val1_f32,NE).unwrap();
	raw_bytes_from_ne[ 4 ..  8]
	    .decastf::<f32>(&fdata1.ne_vals.val2_f32,NE).unwrap();
	raw_bytes_from_ne[ 8 .. 16]
	    .decastf::<f64>(&fdata1.ne_vals.val1_f64,NE).unwrap();
	raw_bytes_from_ne[16 .. 24]
	    .decastf::<f64>(&fdata1.ne_vals.val2_f64,NE).unwrap();

	raw_bytes_from_se[ 0 ..  4]
	    .decastf::<f32>(&fdata1.se_vals.val1_f32,SE).unwrap();
	raw_bytes_from_se[ 4 ..  8]
	    .decastf::<f32>(&fdata1.se_vals.val2_f32,SE).unwrap();
	raw_bytes_from_se[ 8 .. 16]
	    .decastf::<f64>(&fdata1.se_vals.val1_f64,SE).unwrap();
	raw_bytes_from_se[16 .. 24]
	    .decastf::<f64>(&fdata1.se_vals.val2_f64,SE).unwrap();

	raw_bytes_from_le[ 0 ..  4]
	    .decastf::<f32>(&fdata1.le_vals.val1_f32,LE).unwrap();
	raw_bytes_from_le[ 4 ..  8]
	    .decastf::<f32>(&fdata1.le_vals.val2_f32,LE).unwrap();
	raw_bytes_from_le[ 8 .. 16]
	    .decastf::<f64>(&fdata1.le_vals.val1_f64,LE).unwrap();
	raw_bytes_from_le[16 .. 24]
	    .decastf::<f64>(&fdata1.le_vals.val2_f64,LE).unwrap();

	raw_bytes_from_be[ 0 ..  4]
	    .decastf::<f32>(&fdata1.be_vals.val1_f32,BE).unwrap();
	raw_bytes_from_be[ 4 ..  8]
	    .decastf::<f32>(&fdata1.be_vals.val2_f32,BE).unwrap();
	raw_bytes_from_be[ 8 .. 16]
	    .decastf::<f64>(&fdata1.be_vals.val1_f64,BE).unwrap();
	raw_bytes_from_be[16 .. 24]
	    .decastf::<f64>(&fdata1.be_vals.val2_f64,BE).unwrap();

	assert_eq!(raw_bytes_from_ne, fdata1.raw_bytes);
	assert_eq!(raw_bytes_from_se, fdata1.raw_bytes);
	assert_eq!(raw_bytes_from_le, fdata1.raw_bytes);
	assert_eq!(raw_bytes_from_be, fdata1.raw_bytes);
    }
    {
	let fdata1 = FData1::gen();

	// The type parameter of decastf() can be omitted where
	// the Rust compiler can infer the type of the result.

	let mut raw_bytes_from_ne = [0_u8; size_of::<FVals1>()];
	let mut raw_bytes_from_se = [0_u8; size_of::<FVals1>()];
	let mut raw_bytes_from_le = [0_u8; size_of::<FVals1>()];
	let mut raw_bytes_from_be = [0_u8; size_of::<FVals1>()];

	raw_bytes_from_ne[ 0 ..  4]
	    .decastf(&fdata1.ne_vals.val1_f32, NE).unwrap();
	raw_bytes_from_ne[ 4 ..  8]
	    .decastf(&fdata1.ne_vals.val2_f32, NE).unwrap();
	raw_bytes_from_ne[ 8 .. 16]
	    .decastf(&fdata1.ne_vals.val1_f64, NE).unwrap();
	raw_bytes_from_ne[16 .. 24]
	    .decastf(&fdata1.ne_vals.val2_f64, NE).unwrap();

	raw_bytes_from_se[ 0 ..  4]
	    .decastf(&fdata1.se_vals.val1_f32, SE).unwrap();
	raw_bytes_from_se[ 4 ..  8]
	    .decastf(&fdata1.se_vals.val2_f32, SE).unwrap();
	raw_bytes_from_se[ 8 .. 16]
	    .decastf(&fdata1.se_vals.val1_f64, SE).unwrap();
	raw_bytes_from_se[16 .. 24]
	    .decastf(&fdata1.se_vals.val2_f64, SE).unwrap();

	raw_bytes_from_le[ 0 ..  4]
	    .decastf(&fdata1.le_vals.val1_f32, LE).unwrap();
	raw_bytes_from_le[ 4 ..  8]
	    .decastf(&fdata1.le_vals.val2_f32, LE).unwrap();
	raw_bytes_from_le[ 8 .. 16]
	    .decastf(&fdata1.le_vals.val1_f64, LE).unwrap();
	raw_bytes_from_le[16 .. 24]
	    .decastf(&fdata1.le_vals.val2_f64, LE).unwrap();

	raw_bytes_from_be[ 0 ..  4]
	    .decastf(&fdata1.be_vals.val1_f32, BE).unwrap();
	raw_bytes_from_be[ 4 ..  8]
	    .decastf(&fdata1.be_vals.val2_f32, BE).unwrap();
	raw_bytes_from_be[ 8 .. 16]
	    .decastf(&fdata1.be_vals.val1_f64, BE).unwrap();
	raw_bytes_from_be[16 .. 24]
	    .decastf(&fdata1.be_vals.val2_f64, BE).unwrap();

	assert_eq!(raw_bytes_from_ne, fdata1.raw_bytes);
	assert_eq!(raw_bytes_from_se, fdata1.raw_bytes);
	assert_eq!(raw_bytes_from_le, fdata1.raw_bytes);
	assert_eq!(raw_bytes_from_be, fdata1.raw_bytes);
    }
}
