use castflip::{EncastMem, NE, SE, LE, BE};
use crate::{FData1, FVals1};


#[test]
fn fdata1() {
    let fdata1 = FData1::gen();

    let ne_vals_from_ne = FVals1 {
	val1_f32:  fdata1.ne_bytes[ 0 ..  4].encastf::<f32>(NE).unwrap(),
	val2_f32:  fdata1.ne_bytes[ 4 ..  8].encastf::<f32>(NE).unwrap(),
	val1_f64:  fdata1.ne_bytes[ 8 .. 16].encastf::<f64>(NE).unwrap(),
	val2_f64:  fdata1.ne_bytes[16 .. 24].encastf::<f64>(NE).unwrap(),
    };

    let ne_vals_from_se = FVals1 {
	val1_f32:  fdata1.se_bytes[ 0 ..  4].encastf::<f32>(SE).unwrap(),
	val2_f32:  fdata1.se_bytes[ 4 ..  8].encastf::<f32>(SE).unwrap(),
	val1_f64:  fdata1.se_bytes[ 8 .. 16].encastf::<f64>(SE).unwrap(),
	val2_f64:  fdata1.se_bytes[16 .. 24].encastf::<f64>(SE).unwrap(),
    };

    let ne_vals_from_le = FVals1 {
	val1_f32:  fdata1.le_bytes[ 0 ..  4].encastf::<f32>(LE).unwrap(),
	val2_f32:  fdata1.le_bytes[ 4 ..  8].encastf::<f32>(LE).unwrap(),
	val1_f64:  fdata1.le_bytes[ 8 .. 16].encastf::<f64>(LE).unwrap(),
	val2_f64:  fdata1.le_bytes[16 .. 24].encastf::<f64>(LE).unwrap(),
    };

    let ne_vals_from_be = FVals1 {
	val1_f32:  fdata1.be_bytes[ 0 ..  4].encastf::<f32>(BE).unwrap(),
	val2_f32:  fdata1.be_bytes[ 4 ..  8].encastf::<f32>(BE).unwrap(),
	val1_f64:  fdata1.be_bytes[ 8 .. 16].encastf::<f64>(BE).unwrap(),
	val2_f64:  fdata1.be_bytes[16 .. 24].encastf::<f64>(BE).unwrap(),
    };

    assert_eq!(ne_vals_from_ne, fdata1.ne_vals);
    assert_eq!(ne_vals_from_se, fdata1.ne_vals);
    assert_eq!(ne_vals_from_le, fdata1.ne_vals);
    assert_eq!(ne_vals_from_be, fdata1.ne_vals);
}
