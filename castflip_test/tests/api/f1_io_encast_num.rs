use std::io::Cursor;

use castflip::EncastIO;
use crate::{FData1, FVals1};


#[test]
fn fdata1() {
    {
	let fdata1 = FData1::gen();

	let mut raw_input = Cursor::new(fdata1.raw_bytes.clone());

	let ne_vals_from_raw = FVals1{
	    val1_f32:	raw_input.encast::<f32>().unwrap(),
	    val2_f32:	raw_input.encast::<f32>().unwrap(),
	    val1_f64:	raw_input.encast::<f64>().unwrap(),
	    val2_f64:	raw_input.encast::<f64>().unwrap(),
	};

	assert_eq!(ne_vals_from_raw, fdata1.ne_vals);
    }
    {
	let fdata1 = FData1::gen();

	let mut raw_input = Cursor::new(fdata1.raw_bytes.clone());

	// The type parameter of encast() can be omitted where
	// the Rust compiler can infer the type of the result.
	let ne_vals_from_raw = FVals1{
	    val1_f32:	raw_input.encast().unwrap(),
	    val2_f32:	raw_input.encast().unwrap(),
	    val1_f64:	raw_input.encast().unwrap(),
	    val2_f64:	raw_input.encast().unwrap(),
	};

	assert_eq!(ne_vals_from_raw, fdata1.ne_vals);
    }
}
