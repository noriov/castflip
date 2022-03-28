use std::io::Cursor;

use castflip::{EncastIO, NE, SE, LE, BE};
use crate::{IData1, IVals1};


#[test]
fn idata1() {
    let idata1 = IData1::gen();

    let mut ne_input = Cursor::new(idata1.ne_bytes.clone());
    let mut se_input = Cursor::new(idata1.se_bytes.clone());
    let mut le_input = Cursor::new(idata1.le_bytes.clone());
    let mut be_input = Cursor::new(idata1.be_bytes.clone());

    let ne_vals_from_ne = IVals1 {
	val1_i8:  ne_input.encastf::<i8>(NE).unwrap(),
	val2_i8:  ne_input.encastf::<i8>(NE).unwrap(),
	val_i16:  ne_input.encastf::<i16>(NE).unwrap(),
	val_i32:  ne_input.encastf::<i32>(NE).unwrap(),
	val_i64:  ne_input.encastf::<i64>(NE).unwrap(),
	val_i128: ne_input.encastf::<i128>(NE).unwrap(),
    };

    let ne_vals_from_se = IVals1 {
	val1_i8:  se_input.encastf::<i8>(SE).unwrap(),
	val2_i8:  se_input.encastf::<i8>(SE).unwrap(),
	val_i16:  se_input.encastf::<i16>(SE).unwrap(),
	val_i32:  se_input.encastf::<i32>(SE).unwrap(),
	val_i64:  se_input.encastf::<i64>(SE).unwrap(),
	val_i128: se_input.encastf::<i128>(SE).unwrap(),
    };

    let ne_vals_from_le = IVals1 {
	val1_i8:  le_input.encastf::<i8>(LE).unwrap(),
	val2_i8:  le_input.encastf::<i8>(LE).unwrap(),
	val_i16:  le_input.encastf::<i16>(LE).unwrap(),
	val_i32:  le_input.encastf::<i32>(LE).unwrap(),
	val_i64:  le_input.encastf::<i64>(LE).unwrap(),
	val_i128: le_input.encastf::<i128>(LE).unwrap(),
    };

    let ne_vals_from_be = IVals1 {
	val1_i8:  be_input.encastf::<i8>(BE).unwrap(),
	val2_i8:  be_input.encastf::<i8>(BE).unwrap(),
	val_i16:  be_input.encastf::<i16>(BE).unwrap(),
	val_i32:  be_input.encastf::<i32>(BE).unwrap(),
	val_i64:  be_input.encastf::<i64>(BE).unwrap(),
	val_i128: be_input.encastf::<i128>(BE).unwrap(),
    };

    assert_eq!(ne_vals_from_ne, idata1.ne_vals);
    assert_eq!(ne_vals_from_se, idata1.ne_vals);
    assert_eq!(ne_vals_from_le, idata1.ne_vals);
    assert_eq!(ne_vals_from_be, idata1.ne_vals);
}
