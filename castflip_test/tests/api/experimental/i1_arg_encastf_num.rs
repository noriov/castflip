use castflip::experimental::EncastArg;
use castflip::{NE, SE, LE, BE};
use crate::{IData1, IVals1};


#[test]
fn idata1() {
    let idata1 = IData1::gen();

    let ne_vals_from_ne = IVals1 {
        val1_i8:  i8::encastf(  &idata1.ne_bytes[ 0 ..  1], NE).unwrap(),
        val2_i8:  i8::encastf(  &idata1.ne_bytes[ 1 ..  2], NE).unwrap(),
        val_i16:  i16::encastf( &idata1.ne_bytes[ 2 ..  4], NE).unwrap(),
        val_i32:  i32::encastf( &idata1.ne_bytes[ 4 ..  8], NE).unwrap(),
        val_i64:  i64::encastf( &idata1.ne_bytes[ 8 .. 16], NE).unwrap(),
        val_i128: i128::encastf(&idata1.ne_bytes[16 .. 32], NE).unwrap(),
    };

    let ne_vals_from_se = IVals1 {
        val1_i8:  i8::encastf(  &idata1.se_bytes[ 0 ..  1], SE).unwrap(),
        val2_i8:  i8::encastf(  &idata1.se_bytes[ 1 ..  2], SE).unwrap(),
        val_i16:  i16::encastf( &idata1.se_bytes[ 2 ..  4], SE).unwrap(),
        val_i32:  i32::encastf( &idata1.se_bytes[ 4 ..  8], SE).unwrap(),
        val_i64:  i64::encastf( &idata1.se_bytes[ 8 .. 16], SE).unwrap(),
        val_i128: i128::encastf(&idata1.se_bytes[16 .. 32], SE).unwrap(),
    };

    let ne_vals_from_le = IVals1 {
        val1_i8:  i8::encastf(  &idata1.le_bytes[ 0 ..  1], LE).unwrap(),
        val2_i8:  i8::encastf(  &idata1.le_bytes[ 1 ..  2], LE).unwrap(),
        val_i16:  i16::encastf( &idata1.le_bytes[ 2 ..  4], LE).unwrap(),
        val_i32:  i32::encastf( &idata1.le_bytes[ 4 ..  8], LE).unwrap(),
        val_i64:  i64::encastf( &idata1.le_bytes[ 8 .. 16], LE).unwrap(),
        val_i128: i128::encastf(&idata1.le_bytes[16 .. 32], LE).unwrap(),
    };

    let ne_vals_from_be = IVals1 {
        val1_i8:  i8::encastf(  &idata1.be_bytes[ 0 ..  1], BE).unwrap(),
        val2_i8:  i8::encastf(  &idata1.be_bytes[ 1 ..  2], BE).unwrap(),
        val_i16:  i16::encastf( &idata1.be_bytes[ 2 ..  4], BE).unwrap(),
        val_i32:  i32::encastf( &idata1.be_bytes[ 4 ..  8], BE).unwrap(),
        val_i64:  i64::encastf( &idata1.be_bytes[ 8 .. 16], BE).unwrap(),
        val_i128: i128::encastf(&idata1.be_bytes[16 .. 32], BE).unwrap(),
    };

    assert_eq!(ne_vals_from_ne, idata1.ne_vals);
    assert_eq!(ne_vals_from_se, idata1.ne_vals);
    assert_eq!(ne_vals_from_le, idata1.ne_vals);
    assert_eq!(ne_vals_from_be, idata1.ne_vals);
}
