use castflip::EncastMem;
use crate::{FData1, FVals1};


#[test]
fn fdata1() {
    {
	let fdata1 = FData1::gen();

	let ne_vals_from_raw = FVals1{
	    val1_f32:  fdata1.raw_bytes[ 0 ..  4].encast::<f32>().unwrap(),
	    val2_f32:  fdata1.raw_bytes[ 4 ..  8].encast::<f32>().unwrap(),
	    val1_f64:  fdata1.raw_bytes[ 8 .. 16].encast::<f64>().unwrap(),
	    val2_f64:  fdata1.raw_bytes[16 .. 24].encast::<f64>().unwrap(),
	};

	assert_eq!(ne_vals_from_raw, fdata1.ne_vals);
    }
    {
	let fdata1 = FData1::gen();

	// The type parameter of encast() can be omitted where
	// the Rust compiler can infer the type of the result.
	let ne_vals_from_raw = FVals1{
	    val1_f32:  fdata1.raw_bytes[ 0 ..  4].encast().unwrap(),
	    val2_f32:  fdata1.raw_bytes[ 4 ..  8].encast().unwrap(),
	    val1_f64:  fdata1.raw_bytes[ 8 .. 16].encast().unwrap(),
	    val2_f64:  fdata1.raw_bytes[16 .. 24].encast().unwrap(),
	};

	assert_eq!(ne_vals_from_raw, fdata1.ne_vals);
    }
}
