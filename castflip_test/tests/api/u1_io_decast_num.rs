use core::mem::size_of;
use std::io::Cursor;

use castflip::DecastIO;
use crate::{UData1, UVals1};


#[test]
fn udata1() {
    {
	let udata1 = UData1::gen();

	let mut ne_output = Cursor::new(vec![0_u8; size_of::<UVals1>()]);

	ne_output.decast::<u8>(&udata1.ne_vals.val1_u8).unwrap();
	ne_output.decast::<u8>(&udata1.ne_vals.val2_u8).unwrap();
	ne_output.decast::<u16>(&udata1.ne_vals.val_u16).unwrap();
	ne_output.decast::<u32>(&udata1.ne_vals.val_u32).unwrap();
	ne_output.decast::<u64>(&udata1.ne_vals.val_u64).unwrap();
	ne_output.decast::<u128>(&udata1.ne_vals.val_u128).unwrap();

	assert_eq!(ne_output.into_inner(), udata1.raw_bytes);
    }
    {
	let udata1 = UData1::gen();

	let mut ne_output = Cursor::new(vec![0_u8; size_of::<UVals1>()]);

	// The type parameter of decast() can be omitted where
	// the Rust compiler can infer the type of the result.
	ne_output.decast(&udata1.ne_vals.val1_u8).unwrap();
	ne_output.decast(&udata1.ne_vals.val2_u8).unwrap();
	ne_output.decast(&udata1.ne_vals.val_u16).unwrap();
	ne_output.decast(&udata1.ne_vals.val_u32).unwrap();
	ne_output.decast(&udata1.ne_vals.val_u64).unwrap();
	ne_output.decast(&udata1.ne_vals.val_u128).unwrap();

	assert_eq!(ne_output.into_inner(), udata1.raw_bytes);
    }
}
