use castflip::experimental::EncastArg;
use castflip::{NE, SE, LE, BE};
use crate::{IData1, IVals1};


#[test]
fn idata1() {
    let idata1 = IData1::gen();

    let ne_vals_from_raw = IVals1{
	val1_i8:  i8::encastf(  &idata1.raw_bytes[ 0 ..  1], NE).unwrap(),
	val2_i8:  i8::encastf(  &idata1.raw_bytes[ 1 ..  2], NE).unwrap(),
	val_i16:  i16::encastf( &idata1.raw_bytes[ 2 ..  4], NE).unwrap(),
	val_i32:  i32::encastf( &idata1.raw_bytes[ 4 ..  8], NE).unwrap(),
	val_i64:  i64::encastf( &idata1.raw_bytes[ 8 .. 16], NE).unwrap(),
	val_i128: i128::encastf(&idata1.raw_bytes[16 .. 32], NE).unwrap(),
    };

    let se_vals_from_raw = IVals1{
	val1_i8:  i8::encastf(  &idata1.raw_bytes[ 0 ..  1], SE).unwrap(),
	val2_i8:  i8::encastf(  &idata1.raw_bytes[ 1 ..  2], SE).unwrap(),
	val_i16:  i16::encastf( &idata1.raw_bytes[ 2 ..  4], SE).unwrap(),
	val_i32:  i32::encastf( &idata1.raw_bytes[ 4 ..  8], SE).unwrap(),
	val_i64:  i64::encastf( &idata1.raw_bytes[ 8 .. 16], SE).unwrap(),
	val_i128: i128::encastf(&idata1.raw_bytes[16 .. 32], SE).unwrap(),
    };

    let le_vals_from_raw = IVals1{
	val1_i8:  i8::encastf(  &idata1.raw_bytes[ 0 ..  1], LE).unwrap(),
	val2_i8:  i8::encastf(  &idata1.raw_bytes[ 1 ..  2], LE).unwrap(),
	val_i16:  i16::encastf( &idata1.raw_bytes[ 2 ..  4], LE).unwrap(),
	val_i32:  i32::encastf( &idata1.raw_bytes[ 4 ..  8], LE).unwrap(),
	val_i64:  i64::encastf( &idata1.raw_bytes[ 8 .. 16], LE).unwrap(),
	val_i128: i128::encastf(&idata1.raw_bytes[16 .. 32], LE).unwrap(),
    };

    let be_vals_from_raw = IVals1{
	val1_i8:  i8::encastf(  &idata1.raw_bytes[ 0 ..  1], BE).unwrap(),
	val2_i8:  i8::encastf(  &idata1.raw_bytes[ 1 ..  2], BE).unwrap(),
	val_i16:  i16::encastf( &idata1.raw_bytes[ 2 ..  4], BE).unwrap(),
	val_i32:  i32::encastf( &idata1.raw_bytes[ 4 ..  8], BE).unwrap(),
	val_i64:  i64::encastf( &idata1.raw_bytes[ 8 .. 16], BE).unwrap(),
	val_i128: i128::encastf(&idata1.raw_bytes[16 .. 32], BE).unwrap(),
    };

    assert_eq!(ne_vals_from_raw, idata1.ne_vals);
    assert_eq!(se_vals_from_raw, idata1.se_vals);
    assert_eq!(le_vals_from_raw, idata1.le_vals);
    assert_eq!(be_vals_from_raw, idata1.be_vals);
}
