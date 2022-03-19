use std::io::Cursor;

use castflip::EncastIO;
use crate::{IData1, IVals1};


#[test]
fn idata1() {
    {
	let idata1 = IData1::gen();

	let mut raw_input = Cursor::new(idata1.raw_bytes.clone());

	let ne_vals_from_raw = IVals1{
	    val1_i8:	raw_input.encast::<i8>().unwrap(),
	    val2_i8:	raw_input.encast::<i8>().unwrap(),
	    val_i16:	raw_input.encast::<i16>().unwrap(),
	    val_i32:	raw_input.encast::<i32>().unwrap(),
	    val_i64:	raw_input.encast::<i64>().unwrap(),
	    val_i128:	raw_input.encast::<i128>().unwrap(),
	};

	assert_eq!(ne_vals_from_raw, idata1.ne_vals);
    }
    {
	let idata1 = IData1::gen();

	let mut raw_input = Cursor::new(idata1.raw_bytes.clone());

	// The type parameter of encast() can be omitted where
	// the Rust compiler can infer the type of the result.
	let ne_vals_from_raw = IVals1{
	    val1_i8:	raw_input.encast().unwrap(),
	    val2_i8:	raw_input.encast().unwrap(),
	    val_i16:	raw_input.encast().unwrap(),
	    val_i32:	raw_input.encast().unwrap(),
	    val_i64:	raw_input.encast().unwrap(),
	    val_i128:	raw_input.encast().unwrap(),
	};

	assert_eq!(ne_vals_from_raw, idata1.ne_vals);
    }
}
