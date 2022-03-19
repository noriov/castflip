use std::io::Cursor;

use castflip::EncastIO;
use crate::{UData1, UVals1};


#[test]
fn udata1() {
    {
	let udata1 = UData1::gen();

	let mut raw_input = Cursor::new(udata1.raw_bytes.clone());

	let ne_vals_from_raw = UVals1{
	    val1_u8:	raw_input.encast::<u8>().unwrap(),
	    val2_u8:	raw_input.encast::<u8>().unwrap(),
	    val_u16:	raw_input.encast::<u16>().unwrap(),
	    val_u32:	raw_input.encast::<u32>().unwrap(),
	    val_u64:	raw_input.encast::<u64>().unwrap(),
	    val_u128:	raw_input.encast::<u128>().unwrap(),
	};

	assert_eq!(ne_vals_from_raw, udata1.ne_vals);
    }
    {
	let udata1 = UData1::gen();

	let mut raw_input = Cursor::new(udata1.raw_bytes.clone());

	// The type parameter of encast() can be omitted where
	// the Rust compiler can infer the type of the result.
	let ne_vals_from_raw = UVals1{
	    val1_u8:	raw_input.encast().unwrap(),
	    val2_u8:	raw_input.encast().unwrap(),
	    val_u16:	raw_input.encast().unwrap(),
	    val_u32:	raw_input.encast().unwrap(),
	    val_u64:	raw_input.encast().unwrap(),
	    val_u128:	raw_input.encast().unwrap(),
	};

	assert_eq!(ne_vals_from_raw, udata1.ne_vals);
    }
}
