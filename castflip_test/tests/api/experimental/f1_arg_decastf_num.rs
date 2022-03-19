use core::mem::size_of;

use castflip::experimental::DecastArg;
use castflip::{NE, SE, LE, BE};
use crate::{FData1, FVals1};


#[test]
fn fdata1() {
    let fdata1 = FData1::gen();

    let mut raw_bytes_from_ne = [0_u8; size_of::<FVals1>()];
    let mut raw_bytes_from_se = [0_u8; size_of::<FVals1>()];
    let mut raw_bytes_from_le = [0_u8; size_of::<FVals1>()];
    let mut raw_bytes_from_be = [0_u8; size_of::<FVals1>()];

    f32::decastf(&mut raw_bytes_from_ne[ 0 ..  4],
		 &fdata1.ne_vals.val1_f32, NE).unwrap();
    f32::decastf(&mut raw_bytes_from_ne[ 4 ..  8],
		 &fdata1.ne_vals.val2_f32, NE).unwrap();
    f64::decastf(&mut raw_bytes_from_ne[ 8 .. 16],
		 &fdata1.ne_vals.val1_f64, NE).unwrap();
    f64::decastf(&mut raw_bytes_from_ne[16 .. 24],
		 &fdata1.ne_vals.val2_f64, NE).unwrap();

    f32::decastf(&mut raw_bytes_from_se[ 0 ..  4],
		 &fdata1.se_vals.val1_f32, SE).unwrap();
    f32::decastf(&mut raw_bytes_from_se[ 4 ..  8],
		 &fdata1.se_vals.val2_f32, SE).unwrap();
    f64::decastf(&mut raw_bytes_from_se[ 8 .. 16],
		 &fdata1.se_vals.val1_f64, SE).unwrap();
    f64::decastf(&mut raw_bytes_from_se[16 .. 24],
		 &fdata1.se_vals.val2_f64, SE).unwrap();

    f32::decastf(&mut raw_bytes_from_le[ 0 ..  4],
		 &fdata1.le_vals.val1_f32, LE).unwrap();
    f32::decastf(&mut raw_bytes_from_le[ 4 ..  8],
		 &fdata1.le_vals.val2_f32, LE).unwrap();
    f64::decastf(&mut raw_bytes_from_le[ 8 .. 16],
		 &fdata1.le_vals.val1_f64, LE).unwrap();
    f64::decastf(&mut raw_bytes_from_le[16 .. 24],
		 &fdata1.le_vals.val2_f64, LE).unwrap();

    f32::decastf(&mut raw_bytes_from_be[ 0 ..  4],
		 &fdata1.be_vals.val1_f32, BE).unwrap();
    f32::decastf(&mut raw_bytes_from_be[ 4 ..  8],
		 &fdata1.be_vals.val2_f32, BE).unwrap();
    f64::decastf(&mut raw_bytes_from_be[ 8 .. 16],
		 &fdata1.be_vals.val1_f64, BE).unwrap();
    f64::decastf(&mut raw_bytes_from_be[16 .. 24],
		 &fdata1.be_vals.val2_f64, BE).unwrap();

    assert_eq!(raw_bytes_from_ne, fdata1.raw_bytes);
    assert_eq!(raw_bytes_from_se, fdata1.raw_bytes);
    assert_eq!(raw_bytes_from_le, fdata1.raw_bytes);
    assert_eq!(raw_bytes_from_be, fdata1.raw_bytes);
}
