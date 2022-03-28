use core::mem::size_of;

use castflip::experimental::DecastArg;
use crate::{FData1, FVals1};


#[test]
fn fdata1() {
    let fdata1 = FData1::gen();

    let mut ne_bytes = [0_u8; size_of::<FVals1>()];

    f32::decast(&mut ne_bytes[ 0 ..  4], &fdata1.ne_vals.val1_f32).unwrap();
    f32::decast(&mut ne_bytes[ 4 ..  8], &fdata1.ne_vals.val2_f32).unwrap();
    f64::decast(&mut ne_bytes[ 8 .. 16], &fdata1.ne_vals.val1_f64).unwrap();
    f64::decast(&mut ne_bytes[16 .. 24], &fdata1.ne_vals.val2_f64).unwrap();

    assert_eq!(ne_bytes, fdata1.ne_bytes);
}
