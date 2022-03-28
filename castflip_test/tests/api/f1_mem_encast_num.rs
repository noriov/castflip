use castflip::EncastMem;
use crate::{FData1, FVals1};


#[test]
fn fdata1() {
    let fdata1 = FData1::gen();

    let ne_vals_from_ne = FVals1 {
	val1_f32:  fdata1.ne_bytes[ 0 ..  4].encast::<f32>().unwrap(),
	val2_f32:  fdata1.ne_bytes[ 4 ..  8].encast::<f32>().unwrap(),
	val1_f64:  fdata1.ne_bytes[ 8 .. 16].encast::<f64>().unwrap(),
	val2_f64:  fdata1.ne_bytes[16 .. 24].encast::<f64>().unwrap(),
    };

    assert_eq!(ne_vals_from_ne, fdata1.ne_vals);
}
