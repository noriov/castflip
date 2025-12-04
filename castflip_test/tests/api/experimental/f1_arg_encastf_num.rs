use castflip::experimental::EncastArg;
use castflip::{NE, SE, LE, BE};
use crate::{FData1, FVals1};


#[test]
fn fdata1() {
    let fdata1 = FData1::gen();

    let ne_vals_from_ne = FVals1 {
        val1_f32:  f32::encastf(&fdata1.ne_bytes[ 0 ..  4], NE).unwrap(),
        val2_f32:  f32::encastf(&fdata1.ne_bytes[ 4 ..  8], NE).unwrap(),
        val1_f64:  f64::encastf(&fdata1.ne_bytes[ 8 .. 16], NE).unwrap(),
        val2_f64:  f64::encastf(&fdata1.ne_bytes[16 .. 24], NE).unwrap(),
    };

    let ne_vals_from_se = FVals1 {
        val1_f32:  f32::encastf(&fdata1.se_bytes[ 0 ..  4], SE).unwrap(),
        val2_f32:  f32::encastf(&fdata1.se_bytes[ 4 ..  8], SE).unwrap(),
        val1_f64:  f64::encastf(&fdata1.se_bytes[ 8 .. 16], SE).unwrap(),
        val2_f64:  f64::encastf(&fdata1.se_bytes[16 .. 24], SE).unwrap(),
    };

    let ne_vals_from_le = FVals1 {
        val1_f32:  f32::encastf(&fdata1.le_bytes[ 0 ..  4], LE).unwrap(),
        val2_f32:  f32::encastf(&fdata1.le_bytes[ 4 ..  8], LE).unwrap(),
        val1_f64:  f64::encastf(&fdata1.le_bytes[ 8 .. 16], LE).unwrap(),
        val2_f64:  f64::encastf(&fdata1.le_bytes[16 .. 24], LE).unwrap(),
    };

    let ne_vals_from_be = FVals1 {
        val1_f32:  f32::encastf(&fdata1.be_bytes[ 0 ..  4], BE).unwrap(),
        val2_f32:  f32::encastf(&fdata1.be_bytes[ 4 ..  8], BE).unwrap(),
        val1_f64:  f64::encastf(&fdata1.be_bytes[ 8 .. 16], BE).unwrap(),
        val2_f64:  f64::encastf(&fdata1.be_bytes[16 .. 24], BE).unwrap(),
    };

    assert_eq!(ne_vals_from_ne, fdata1.ne_vals);
    assert_eq!(ne_vals_from_se, fdata1.ne_vals);
    assert_eq!(ne_vals_from_le, fdata1.ne_vals);
    assert_eq!(ne_vals_from_be, fdata1.ne_vals);
}
