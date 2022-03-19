use core::mem::size_of;

use castflip::DecastMem;
use crate::{UData1, UVals1};


#[test]
fn udata1() {
    {
	let udata1 = UData1::gen();

	let mut raw_bytes_from_ne = [0_u8; size_of::<UVals1>()];

	raw_bytes_from_ne[ 0 ..  1]
	    .decast::<u8>(&udata1.ne_vals.val1_u8).unwrap();
	raw_bytes_from_ne[ 1 ..  2]
	    .decast::<u8>(&udata1.ne_vals.val2_u8).unwrap();
	raw_bytes_from_ne[ 2 ..  4]
	    .decast::<u16>(&udata1.ne_vals.val_u16).unwrap();
	raw_bytes_from_ne[ 4 ..  8]
	    .decast::<u32>(&udata1.ne_vals.val_u32).unwrap();
	raw_bytes_from_ne[ 8 .. 16]
	    .decast::<u64>(&udata1.ne_vals.val_u64).unwrap();
	raw_bytes_from_ne[16 .. 32]
	    .decast::<u128>(&udata1.ne_vals.val_u128).unwrap();

	assert_eq!(raw_bytes_from_ne, udata1.raw_bytes);
    }
    {
	let udata1 = UData1::gen();

	let mut raw_bytes_from_ne = [0_u8; size_of::<UVals1>()];

	// The type parameter of decast() can be omitted where
	// the Rust compiler can infer the type of the result.
	raw_bytes_from_ne[ 0 ..  1].decast(&udata1.ne_vals.val1_u8).unwrap();
	raw_bytes_from_ne[ 1 ..  2].decast(&udata1.ne_vals.val2_u8).unwrap();
	raw_bytes_from_ne[ 2 ..  4].decast(&udata1.ne_vals.val_u16).unwrap();
	raw_bytes_from_ne[ 4 ..  8].decast(&udata1.ne_vals.val_u32).unwrap();
	raw_bytes_from_ne[ 8 .. 16].decast(&udata1.ne_vals.val_u64).unwrap();
	raw_bytes_from_ne[16 .. 32].decast(&udata1.ne_vals.val_u128).unwrap();

	assert_eq!(raw_bytes_from_ne, udata1.raw_bytes);
    }
}
