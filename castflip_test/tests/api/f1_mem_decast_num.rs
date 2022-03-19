use core::mem::size_of;

use castflip::DecastMem;
use crate::{FData1, FVals1};


#[test]
fn fdata1() {
    {
	let fdata1 = FData1::gen();

	let mut raw_bytes_from_ne = [0_u8; size_of::<FVals1>()];

	raw_bytes_from_ne[ 0 ..  4]
	    .decast::<f32>(&fdata1.ne_vals.val1_f32).unwrap();
	raw_bytes_from_ne[ 4 ..  8]
	    .decast::<f32>(&fdata1.ne_vals.val2_f32).unwrap();
	raw_bytes_from_ne[ 8 .. 16]
	    .decast::<f64>(&fdata1.ne_vals.val1_f64).unwrap();
	raw_bytes_from_ne[16 .. 24]
	    .decast::<f64>(&fdata1.ne_vals.val2_f64).unwrap();

	assert_eq!(raw_bytes_from_ne, fdata1.raw_bytes);
    }
    {
	let fdata1 = FData1::gen();

	let mut raw_bytes_from_ne = [0_u8; size_of::<FVals1>()];

	// The type parameter of decast() can be omitted where
	// the Rust compiler can infer the type of the result.
	raw_bytes_from_ne[ 0 ..  4].decast(&fdata1.ne_vals.val1_f32).unwrap();
	raw_bytes_from_ne[ 4 ..  8].decast(&fdata1.ne_vals.val2_f32).unwrap();
	raw_bytes_from_ne[ 8 .. 16].decast(&fdata1.ne_vals.val1_f64).unwrap();
	raw_bytes_from_ne[16 .. 24].decast(&fdata1.ne_vals.val2_f64).unwrap();

	assert_eq!(raw_bytes_from_ne, fdata1.raw_bytes);
    }
}
