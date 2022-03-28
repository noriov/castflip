use core::mem::size_of;

use castflip::experimental::DecastArg;
use castflip::{NE, SE, LE, BE};
use crate::{FData1, FVals1};


#[test]
fn fdata1() {
    let fdata1 = FData1::gen();

    let mut ne_bytes = [0_u8; size_of::<FVals1>()];
    let mut se_bytes = [0_u8; size_of::<FVals1>()];
    let mut le_bytes = [0_u8; size_of::<FVals1>()];
    let mut be_bytes = [0_u8; size_of::<FVals1>()];

    f32::decastf(&mut ne_bytes[ 0 ..  4],
		 &fdata1.ne_vals.val1_f32, NE).unwrap();
    f32::decastf(&mut ne_bytes[ 4 ..  8],
		 &fdata1.ne_vals.val2_f32, NE).unwrap();
    f64::decastf(&mut ne_bytes[ 8 .. 16],
		 &fdata1.ne_vals.val1_f64, NE).unwrap();
    f64::decastf(&mut ne_bytes[16 .. 24],
		 &fdata1.ne_vals.val2_f64, NE).unwrap();

    f32::decastf(&mut se_bytes[ 0 ..  4],
		 &fdata1.ne_vals.val1_f32, SE).unwrap();
    f32::decastf(&mut se_bytes[ 4 ..  8],
		 &fdata1.ne_vals.val2_f32, SE).unwrap();
    f64::decastf(&mut se_bytes[ 8 .. 16],
		 &fdata1.ne_vals.val1_f64, SE).unwrap();
    f64::decastf(&mut se_bytes[16 .. 24],
		 &fdata1.ne_vals.val2_f64, SE).unwrap();

    f32::decastf(&mut le_bytes[ 0 ..  4],
		 &fdata1.ne_vals.val1_f32, LE).unwrap();
    f32::decastf(&mut le_bytes[ 4 ..  8],
		 &fdata1.ne_vals.val2_f32, LE).unwrap();
    f64::decastf(&mut le_bytes[ 8 .. 16],
		 &fdata1.ne_vals.val1_f64, LE).unwrap();
    f64::decastf(&mut le_bytes[16 .. 24],
		 &fdata1.ne_vals.val2_f64, LE).unwrap();

    f32::decastf(&mut be_bytes[ 0 ..  4],
		 &fdata1.ne_vals.val1_f32, BE).unwrap();
    f32::decastf(&mut be_bytes[ 4 ..  8],
		 &fdata1.ne_vals.val2_f32, BE).unwrap();
    f64::decastf(&mut be_bytes[ 8 .. 16],
		 &fdata1.ne_vals.val1_f64, BE).unwrap();
    f64::decastf(&mut be_bytes[16 .. 24],
		 &fdata1.ne_vals.val2_f64, BE).unwrap();

    assert_eq!(ne_bytes, fdata1.ne_bytes);
    assert_eq!(se_bytes, fdata1.se_bytes);
    assert_eq!(le_bytes, fdata1.le_bytes);
    assert_eq!(be_bytes, fdata1.be_bytes);
}
