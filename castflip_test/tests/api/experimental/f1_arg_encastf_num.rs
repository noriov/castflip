use castflip::experimental::EncastArg;
use castflip::{NE, SE, LE, BE};
use crate::{FData1, FVals1};


#[test]
fn fdata1() {
    let fdata1 = FData1::gen();

    let ne_vals_from_raw = FVals1{
	val1_f32:  f32::encastf(&fdata1.raw_bytes[ 0 ..  4], NE).unwrap(),
	val2_f32:  f32::encastf(&fdata1.raw_bytes[ 4 ..  8], NE).unwrap(),
	val1_f64:  f64::encastf(&fdata1.raw_bytes[ 8 .. 16], NE).unwrap(),
	val2_f64:  f64::encastf(&fdata1.raw_bytes[16 .. 24], NE).unwrap(),
    };

    let se_vals_from_raw = FVals1{
	val1_f32:  f32::encastf(&fdata1.raw_bytes[ 0 ..  4], SE).unwrap(),
	val2_f32:  f32::encastf(&fdata1.raw_bytes[ 4 ..  8], SE).unwrap(),
	val1_f64:  f64::encastf(&fdata1.raw_bytes[ 8 .. 16], SE).unwrap(),
	val2_f64:  f64::encastf(&fdata1.raw_bytes[16 .. 24], SE).unwrap(),
    };

    let le_vals_from_raw = FVals1{
	val1_f32:  f32::encastf(&fdata1.raw_bytes[ 0 ..  4], LE).unwrap(),
	val2_f32:  f32::encastf(&fdata1.raw_bytes[ 4 ..  8], LE).unwrap(),
	val1_f64:  f64::encastf(&fdata1.raw_bytes[ 8 .. 16], LE).unwrap(),
	val2_f64:  f64::encastf(&fdata1.raw_bytes[16 .. 24], LE).unwrap(),
    };

    let be_vals_from_raw = FVals1{
	val1_f32:  f32::encastf(&fdata1.raw_bytes[ 0 ..  4], BE).unwrap(),
	val2_f32:  f32::encastf(&fdata1.raw_bytes[ 4 ..  8], BE).unwrap(),
	val1_f64:  f64::encastf(&fdata1.raw_bytes[ 8 .. 16], BE).unwrap(),
	val2_f64:  f64::encastf(&fdata1.raw_bytes[16 .. 24], BE).unwrap(),
    };

    assert_eq!(ne_vals_from_raw, fdata1.ne_vals);
    assert_eq!(se_vals_from_raw, fdata1.se_vals);
    assert_eq!(le_vals_from_raw, fdata1.le_vals);
    assert_eq!(be_vals_from_raw, fdata1.be_vals);
}
