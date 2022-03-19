use castflip::EncastMem;
use crate::{IData1, IVals1};


#[test]
fn idata1() {
    {
	let idata1 = IData1::gen();

	let ne_vals_from_raw = IVals1{
	    val1_i8:  idata1.raw_bytes[ 0 ..  1].encast::<i8>().unwrap(),
	    val2_i8:  idata1.raw_bytes[ 1 ..  2].encast::<i8>().unwrap(),
	    val_i16:  idata1.raw_bytes[ 2 ..  4].encast::<i16>().unwrap(),
	    val_i32:  idata1.raw_bytes[ 4 ..  8].encast::<i32>().unwrap(),
	    val_i64:  idata1.raw_bytes[ 8 .. 16].encast::<i64>().unwrap(),
	    val_i128: idata1.raw_bytes[16 .. 32].encast::<i128>().unwrap(),
	};

	assert_eq!(ne_vals_from_raw, idata1.ne_vals);
    }
    {
	let idata1 = IData1::gen();

	// The type parameter of encast() can be omitted where
	// the Rust compiler can infer the type of the result.
	let ne_vals_from_raw = IVals1{
	    val1_i8:  idata1.raw_bytes[ 0 ..  1].encast().unwrap(),
	    val2_i8:  idata1.raw_bytes[ 1 ..  2].encast().unwrap(),
	    val_i16:  idata1.raw_bytes[ 2 ..  4].encast().unwrap(),
	    val_i32:  idata1.raw_bytes[ 4 ..  8].encast().unwrap(),
	    val_i64:  idata1.raw_bytes[ 8 .. 16].encast().unwrap(),
	    val_i128: idata1.raw_bytes[16 .. 32].encast().unwrap(),
	};

	assert_eq!(ne_vals_from_raw, idata1.ne_vals);
    }
}
