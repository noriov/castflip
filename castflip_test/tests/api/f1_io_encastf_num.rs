use std::io::Cursor;

use castflip::{EncastIO, NE, SE, LE, BE};
use crate::{FData1, FVals1};


#[test]
fn fdata1() {
    let fdata1 = FData1::gen();

    let mut ne_input = Cursor::new(fdata1.ne_bytes.clone());
    let mut se_input = Cursor::new(fdata1.se_bytes.clone());
    let mut le_input = Cursor::new(fdata1.le_bytes.clone());
    let mut be_input = Cursor::new(fdata1.be_bytes.clone());

    let ne_vals_from_ne = FVals1 {
	val1_f32:  ne_input.encastf::<f32>(NE).unwrap(),
	val2_f32:  ne_input.encastf::<f32>(NE).unwrap(),
	val1_f64:  ne_input.encastf::<f64>(NE).unwrap(),
	val2_f64:  ne_input.encastf::<f64>(NE).unwrap(),
    };

    let ne_vals_from_se = FVals1 {
	val1_f32:  se_input.encastf::<f32>(SE).unwrap(),
	val2_f32:  se_input.encastf::<f32>(SE).unwrap(),
	val1_f64:  se_input.encastf::<f64>(SE).unwrap(),
	val2_f64:  se_input.encastf::<f64>(SE).unwrap(),
    };

    let ne_vals_from_le = FVals1 {
	val1_f32:  le_input.encastf::<f32>(LE).unwrap(),
	val2_f32:  le_input.encastf::<f32>(LE).unwrap(),
	val1_f64:  le_input.encastf::<f64>(LE).unwrap(),
	val2_f64:  le_input.encastf::<f64>(LE).unwrap(),
    };

    let ne_vals_from_be = FVals1 {
	val1_f32:  be_input.encastf::<f32>(BE).unwrap(),
	val2_f32:  be_input.encastf::<f32>(BE).unwrap(),
	val1_f64:  be_input.encastf::<f64>(BE).unwrap(),
	val2_f64:  be_input.encastf::<f64>(BE).unwrap(),
    };

    assert_eq!(ne_vals_from_ne, fdata1.ne_vals);
    assert_eq!(ne_vals_from_se, fdata1.ne_vals);
    assert_eq!(ne_vals_from_le, fdata1.ne_vals);
    assert_eq!(ne_vals_from_be, fdata1.ne_vals);
}
