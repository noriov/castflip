use core::mem::size_of;

use castflip::DecastMem;
use crate::{IData1, IVals1};


#[test]
fn idata1() {
    {
	let idata1 = IData1::gen();

	let mut raw_bytes_from_ne = [0_u8; size_of::<IVals1>()];

	raw_bytes_from_ne[ 0 ..  1]
	    .decast::<i8>(&idata1.ne_vals.val1_i8).unwrap();
	raw_bytes_from_ne[ 1 ..  2]
	    .decast::<i8>(&idata1.ne_vals.val2_i8).unwrap();
	raw_bytes_from_ne[ 2 ..  4]
	    .decast::<i16>(&idata1.ne_vals.val_i16).unwrap();
	raw_bytes_from_ne[ 4 ..  8]
	    .decast::<i32>(&idata1.ne_vals.val_i32).unwrap();
	raw_bytes_from_ne[ 8 .. 16]
	    .decast::<i64>(&idata1.ne_vals.val_i64).unwrap();
	raw_bytes_from_ne[16 .. 32]
	    .decast::<i128>(&idata1.ne_vals.val_i128).unwrap();

	assert_eq!(raw_bytes_from_ne, idata1.raw_bytes);
    }
    {
	let idata1 = IData1::gen();

	let mut raw_bytes_from_ne = [0_u8; size_of::<IVals1>()];

	// The type parameter of decast() can be omitted where
	// the Rust compiler can infer the type of the result.
	raw_bytes_from_ne[ 0 ..  1].decast(&idata1.ne_vals.val1_i8).unwrap();
	raw_bytes_from_ne[ 1 ..  2].decast(&idata1.ne_vals.val2_i8).unwrap();
	raw_bytes_from_ne[ 2 ..  4].decast(&idata1.ne_vals.val_i16).unwrap();
	raw_bytes_from_ne[ 4 ..  8].decast(&idata1.ne_vals.val_i32).unwrap();
	raw_bytes_from_ne[ 8 .. 16].decast(&idata1.ne_vals.val_i64).unwrap();
	raw_bytes_from_ne[16 .. 32].decast(&idata1.ne_vals.val_i128).unwrap();

	assert_eq!(raw_bytes_from_ne, idata1.raw_bytes);
    }
}
