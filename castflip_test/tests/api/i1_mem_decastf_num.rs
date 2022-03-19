use core::mem::size_of;

use castflip::{DecastMem, NE, SE, LE, BE};

use crate::{IData1, IVals1};


#[test]
fn idata1() {
    {
	let idata1 = IData1::gen();

	let mut raw_bytes_from_ne = [0_u8; size_of::<IVals1>()];
	let mut raw_bytes_from_se = [0_u8; size_of::<IVals1>()];
	let mut raw_bytes_from_le = [0_u8; size_of::<IVals1>()];
	let mut raw_bytes_from_be = [0_u8; size_of::<IVals1>()];

	raw_bytes_from_ne[ 0 ..  1]
	    .decastf::<i8>(&idata1.ne_vals.val1_i8, NE).unwrap();
	raw_bytes_from_ne[ 1 ..  2]
	    .decastf::<i8>(&idata1.ne_vals.val2_i8, NE).unwrap();
	raw_bytes_from_ne[ 2 ..  4]
	    .decastf::<i16>(&idata1.ne_vals.val_i16, NE).unwrap();
	raw_bytes_from_ne[ 4 ..  8]
	    .decastf::<i32>(&idata1.ne_vals.val_i32, NE).unwrap();
	raw_bytes_from_ne[ 8 .. 16]
	    .decastf::<i64>(&idata1.ne_vals.val_i64, NE).unwrap();
	raw_bytes_from_ne[16 .. 32]
	    .decastf::<i128>(&idata1.ne_vals.val_i128, NE).unwrap();

	raw_bytes_from_se[ 0 ..  1]
	    .decastf::<i8>(&idata1.se_vals.val1_i8, SE).unwrap();
	raw_bytes_from_se[ 1 ..  2]
	    .decastf::<i8>(&idata1.se_vals.val2_i8, SE).unwrap();
	raw_bytes_from_se[ 2 ..  4]
	    .decastf::<i16>(&idata1.se_vals.val_i16, SE).unwrap();
	raw_bytes_from_se[ 4 ..  8]
	    .decastf::<i32>(&idata1.se_vals.val_i32, SE).unwrap();
	raw_bytes_from_se[ 8 .. 16]
	    .decastf::<i64>(&idata1.se_vals.val_i64, SE).unwrap();
	raw_bytes_from_se[16 .. 32]
	    .decastf::<i128>(&idata1.se_vals.val_i128, SE).unwrap();

	raw_bytes_from_le[ 0 ..  1]
	    .decastf::<i8>(&idata1.le_vals.val1_i8, LE).unwrap();
	raw_bytes_from_le[ 1 ..  2]
	    .decastf::<i8>(&idata1.le_vals.val2_i8, LE).unwrap();
	raw_bytes_from_le[ 2 ..  4]
	    .decastf::<i16>(&idata1.le_vals.val_i16, LE).unwrap();
	raw_bytes_from_le[ 4 ..  8]
	    .decastf::<i32>(&idata1.le_vals.val_i32, LE).unwrap();
	raw_bytes_from_le[ 8 .. 16]
	    .decastf::<i64>(&idata1.le_vals.val_i64, LE).unwrap();
	raw_bytes_from_le[16 .. 32]
	    .decastf::<i128>(&idata1.le_vals.val_i128, LE).unwrap();

	raw_bytes_from_be[ 0 ..  1]
	    .decastf::<i8>(&idata1.be_vals.val1_i8, BE).unwrap();
	raw_bytes_from_be[ 1 ..  2]
	    .decastf::<i8>(&idata1.be_vals.val2_i8, BE).unwrap();
	raw_bytes_from_be[ 2 ..  4]
	    .decastf::<i16>(&idata1.be_vals.val_i16, BE).unwrap();
	raw_bytes_from_be[ 4 ..  8]
	    .decastf::<i32>(&idata1.be_vals.val_i32, BE).unwrap();
	raw_bytes_from_be[ 8 .. 16]
	    .decastf::<i64>(&idata1.be_vals.val_i64, BE).unwrap();
	raw_bytes_from_be[16 .. 32]
	    .decastf::<i128>(&idata1.be_vals.val_i128, BE).unwrap();

	assert_eq!(raw_bytes_from_ne, idata1.raw_bytes);
	assert_eq!(raw_bytes_from_se, idata1.raw_bytes);
	assert_eq!(raw_bytes_from_le, idata1.raw_bytes);
	assert_eq!(raw_bytes_from_be, idata1.raw_bytes);
    }
    {
	let idata1 = IData1::gen();

	// The type parameter of decastf() can be omitted where
	// the Rust compiler can infer the type of the result.

	let mut raw_bytes_from_ne = [0_u8; size_of::<IVals1>()];
	let mut raw_bytes_from_se = [0_u8; size_of::<IVals1>()];
	let mut raw_bytes_from_le = [0_u8; size_of::<IVals1>()];
	let mut raw_bytes_from_be = [0_u8; size_of::<IVals1>()];

	raw_bytes_from_ne[ 0 ..  1]
	    .decastf(&idata1.ne_vals.val1_i8, NE).unwrap();
	raw_bytes_from_ne[ 1 ..  2]
	    .decastf(&idata1.ne_vals.val2_i8, NE).unwrap();
	raw_bytes_from_ne[ 2 ..  4]
	    .decastf(&idata1.ne_vals.val_i16, NE).unwrap();
	raw_bytes_from_ne[ 4 ..  8]
	    .decastf(&idata1.ne_vals.val_i32, NE).unwrap();
	raw_bytes_from_ne[ 8 .. 16]
	    .decastf(&idata1.ne_vals.val_i64, NE).unwrap();
	raw_bytes_from_ne[16 .. 32]
	    .decastf(&idata1.ne_vals.val_i128, NE).unwrap();

	raw_bytes_from_se[ 0 ..  1]
	    .decastf(&idata1.se_vals.val1_i8, SE).unwrap();
	raw_bytes_from_se[ 1 ..  2]
	    .decastf(&idata1.se_vals.val2_i8, SE).unwrap();
	raw_bytes_from_se[ 2 ..  4]
	    .decastf(&idata1.se_vals.val_i16, SE).unwrap();
	raw_bytes_from_se[ 4 ..  8]
	    .decastf(&idata1.se_vals.val_i32, SE).unwrap();
	raw_bytes_from_se[ 8 .. 16]
	    .decastf(&idata1.se_vals.val_i64, SE).unwrap();
	raw_bytes_from_se[16 .. 32]
	    .decastf(&idata1.se_vals.val_i128, SE).unwrap();

	raw_bytes_from_le[ 0 ..  1]
	    .decastf(&idata1.le_vals.val1_i8, LE).unwrap();
	raw_bytes_from_le[ 1 ..  2]
	    .decastf(&idata1.le_vals.val2_i8, LE).unwrap();
	raw_bytes_from_le[ 2 ..  4]
	    .decastf(&idata1.le_vals.val_i16, LE).unwrap();
	raw_bytes_from_le[ 4 ..  8]
	    .decastf(&idata1.le_vals.val_i32, LE).unwrap();
	raw_bytes_from_le[ 8 .. 16]
	    .decastf(&idata1.le_vals.val_i64, LE).unwrap();
	raw_bytes_from_le[16 .. 32]
	    .decastf(&idata1.le_vals.val_i128, LE).unwrap();

	raw_bytes_from_be[ 0 ..  1]
	    .decastf(&idata1.be_vals.val1_i8, BE).unwrap();
	raw_bytes_from_be[ 1 ..  2]
	    .decastf(&idata1.be_vals.val2_i8, BE).unwrap();
	raw_bytes_from_be[ 2 ..  4]
	    .decastf(&idata1.be_vals.val_i16, BE).unwrap();
	raw_bytes_from_be[ 4 ..  8]
	    .decastf(&idata1.be_vals.val_i32, BE).unwrap();
	raw_bytes_from_be[ 8 .. 16]
	    .decastf(&idata1.be_vals.val_i64, BE).unwrap();
	raw_bytes_from_be[16 .. 32]
	    .decastf(&idata1.be_vals.val_i128, BE).unwrap();

	assert_eq!(raw_bytes_from_ne, idata1.raw_bytes);
	assert_eq!(raw_bytes_from_se, idata1.raw_bytes);
	assert_eq!(raw_bytes_from_le, idata1.raw_bytes);
	assert_eq!(raw_bytes_from_be, idata1.raw_bytes);
    }
}
