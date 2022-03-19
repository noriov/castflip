use castflip::experimental::EncastArg;
use crate::{FData1, FVals1};


#[test]
fn fdata1() {
    let fdata1 = FData1::gen();

    let ne_vals_from_raw = FVals1{
	val1_f32:  f32::encast(&fdata1.raw_bytes[ 0 ..  4]).unwrap(),
	val2_f32:  f32::encast(&fdata1.raw_bytes[ 4 ..  8]).unwrap(),
	val1_f64:  f64::encast(&fdata1.raw_bytes[ 8 .. 16]).unwrap(),
	val2_f64:  f64::encast(&fdata1.raw_bytes[16 .. 24]).unwrap(),
    };

    assert_eq!(ne_vals_from_raw, fdata1.ne_vals);
}
