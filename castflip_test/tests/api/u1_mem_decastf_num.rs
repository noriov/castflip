use core::mem::size_of;

use castflip::{DecastMem, NE, SE, LE, BE};
use crate::{UData1, UVals1};


#[test]
fn udata1() {
    {
	let udata1 = UData1::gen();

	let mut raw_bytes_from_ne = [0_u8; size_of::<UVals1>()];
	let mut raw_bytes_from_se = [0_u8; size_of::<UVals1>()];
	let mut raw_bytes_from_le = [0_u8; size_of::<UVals1>()];
	let mut raw_bytes_from_be = [0_u8; size_of::<UVals1>()];

	raw_bytes_from_ne[ 0 ..  1]
	    .decastf::<u8>(&udata1.ne_vals.val1_u8, NE).unwrap();
	raw_bytes_from_ne[ 1 ..  2]
	    .decastf::<u8>(&udata1.ne_vals.val2_u8, NE).unwrap();
	raw_bytes_from_ne[ 2 ..  4]
	    .decastf::<u16>(&udata1.ne_vals.val_u16, NE).unwrap();
	raw_bytes_from_ne[ 4 ..  8]
	    .decastf::<u32>(&udata1.ne_vals.val_u32, NE).unwrap();
	raw_bytes_from_ne[ 8 .. 16]
	    .decastf::<u64>(&udata1.ne_vals.val_u64, NE).unwrap();
	raw_bytes_from_ne[16 .. 32]
	    .decastf::<u128>(&udata1.ne_vals.val_u128, NE).unwrap();

	raw_bytes_from_se[ 0 ..  1]
	    .decastf::<u8>(&udata1.se_vals.val1_u8, SE).unwrap();
	raw_bytes_from_se[ 1 ..  2]
	    .decastf::<u8>(&udata1.se_vals.val2_u8, SE).unwrap();
	raw_bytes_from_se[ 2 ..  4]
	    .decastf::<u16>(&udata1.se_vals.val_u16, SE).unwrap();
	raw_bytes_from_se[ 4 ..  8]
	    .decastf::<u32>(&udata1.se_vals.val_u32, SE).unwrap();
	raw_bytes_from_se[ 8 .. 16]
	    .decastf::<u64>(&udata1.se_vals.val_u64, SE).unwrap();
	raw_bytes_from_se[16 .. 32]
	    .decastf::<u128>(&udata1.se_vals.val_u128, SE).unwrap();

	raw_bytes_from_le[ 0 ..  1]
	    .decastf::<u8>(&udata1.le_vals.val1_u8, LE).unwrap();
	raw_bytes_from_le[ 1 ..  2]
	    .decastf::<u8>(&udata1.le_vals.val2_u8, LE).unwrap();
	raw_bytes_from_le[ 2 ..  4]
	    .decastf::<u16>(&udata1.le_vals.val_u16, LE).unwrap();
	raw_bytes_from_le[ 4 ..  8]
	    .decastf::<u32>(&udata1.le_vals.val_u32, LE).unwrap();
	raw_bytes_from_le[ 8 .. 16]
	    .decastf::<u64>(&udata1.le_vals.val_u64, LE).unwrap();
	raw_bytes_from_le[16 .. 32]
	    .decastf::<u128>(&udata1.le_vals.val_u128, LE).unwrap();

	raw_bytes_from_be[ 0 ..  1]
	    .decastf::<u8>(&udata1.be_vals.val1_u8, BE).unwrap();
	raw_bytes_from_be[ 1 ..  2]
	    .decastf::<u8>(&udata1.be_vals.val2_u8, BE).unwrap();
	raw_bytes_from_be[ 2 ..  4]
	    .decastf::<u16>(&udata1.be_vals.val_u16, BE).unwrap();
	raw_bytes_from_be[ 4 ..  8]
	    .decastf::<u32>(&udata1.be_vals.val_u32, BE).unwrap();
	raw_bytes_from_be[ 8 .. 16]
	    .decastf::<u64>(&udata1.be_vals.val_u64, BE).unwrap();
	raw_bytes_from_be[16 .. 32]
	    .decastf::<u128>(&udata1.be_vals.val_u128, BE).unwrap();

	assert_eq!(raw_bytes_from_ne, udata1.raw_bytes);
	assert_eq!(raw_bytes_from_se, udata1.raw_bytes);
	assert_eq!(raw_bytes_from_le, udata1.raw_bytes);
	assert_eq!(raw_bytes_from_be, udata1.raw_bytes);
    }
    {
	let udata1 = UData1::gen();

	// The type parameter of decastf() can be omitted where
	// the Rust compiler can infer the type of the result.

	let mut raw_bytes_from_ne = [0_u8; size_of::<UVals1>()];
	let mut raw_bytes_from_se = [0_u8; size_of::<UVals1>()];
	let mut raw_bytes_from_le = [0_u8; size_of::<UVals1>()];
	let mut raw_bytes_from_be = [0_u8; size_of::<UVals1>()];

	raw_bytes_from_ne[ 0 ..  1]
	    .decastf(&udata1.ne_vals.val1_u8, NE).unwrap();
	raw_bytes_from_ne[ 1 ..  2]
	    .decastf(&udata1.ne_vals.val2_u8, NE).unwrap();
	raw_bytes_from_ne[ 2 ..  4]
	    .decastf(&udata1.ne_vals.val_u16, NE).unwrap();
	raw_bytes_from_ne[ 4 ..  8]
	    .decastf(&udata1.ne_vals.val_u32, NE).unwrap();
	raw_bytes_from_ne[ 8 .. 16]
	    .decastf(&udata1.ne_vals.val_u64, NE).unwrap();
	raw_bytes_from_ne[16 .. 32]
	    .decastf(&udata1.ne_vals.val_u128, NE).unwrap();

	raw_bytes_from_se[ 0 ..  1]
	    .decastf(&udata1.se_vals.val1_u8, SE).unwrap();
	raw_bytes_from_se[ 1 ..  2]
	    .decastf(&udata1.se_vals.val2_u8, SE).unwrap();
	raw_bytes_from_se[ 2 ..  4]
	    .decastf(&udata1.se_vals.val_u16, SE).unwrap();
	raw_bytes_from_se[ 4 ..  8]
	    .decastf(&udata1.se_vals.val_u32, SE).unwrap();
	raw_bytes_from_se[ 8 .. 16]
	    .decastf(&udata1.se_vals.val_u64, SE).unwrap();
	raw_bytes_from_se[16 .. 32]
	    .decastf(&udata1.se_vals.val_u128, SE).unwrap();

	raw_bytes_from_le[ 0 ..  1]
	    .decastf(&udata1.le_vals.val1_u8, LE).unwrap();
	raw_bytes_from_le[ 1 ..  2]
	    .decastf(&udata1.le_vals.val2_u8, LE).unwrap();
	raw_bytes_from_le[ 2 ..  4]
	    .decastf(&udata1.le_vals.val_u16, LE).unwrap();
	raw_bytes_from_le[ 4 ..  8]
	    .decastf(&udata1.le_vals.val_u32, LE).unwrap();
	raw_bytes_from_le[ 8 .. 16]
	    .decastf(&udata1.le_vals.val_u64, LE).unwrap();
	raw_bytes_from_le[16 .. 32]
	    .decastf(&udata1.le_vals.val_u128, LE).unwrap();

	raw_bytes_from_be[ 0 ..  1]
	    .decastf(&udata1.be_vals.val1_u8, BE).unwrap();
	raw_bytes_from_be[ 1 ..  2]
	    .decastf(&udata1.be_vals.val2_u8, BE).unwrap();
	raw_bytes_from_be[ 2 ..  4]
	    .decastf(&udata1.be_vals.val_u16, BE).unwrap();
	raw_bytes_from_be[ 4 ..  8]
	    .decastf(&udata1.be_vals.val_u32, BE).unwrap();
	raw_bytes_from_be[ 8 .. 16]
	    .decastf(&udata1.be_vals.val_u64, BE).unwrap();
	raw_bytes_from_be[16 .. 32]
	    .decastf(&udata1.be_vals.val_u128, BE).unwrap();

	assert_eq!(raw_bytes_from_ne, udata1.raw_bytes);
	assert_eq!(raw_bytes_from_se, udata1.raw_bytes);
	assert_eq!(raw_bytes_from_le, udata1.raw_bytes);
	assert_eq!(raw_bytes_from_be, udata1.raw_bytes);
    }
}
