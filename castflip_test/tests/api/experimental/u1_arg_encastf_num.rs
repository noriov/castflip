use castflip::experimental::EncastArg;
use castflip::{NE, SE, LE, BE};
use crate::{UData1, UVals1};


#[test]
fn udata1() {
    let udata1 = UData1::gen();

    let ne_vals_from_raw = UVals1{
	val1_u8:  u8::encastf(  &udata1.raw_bytes[ 0 ..  1], NE).unwrap(),
	val2_u8:  u8::encastf(  &udata1.raw_bytes[ 1 ..  2], NE).unwrap(),
	val_u16:  u16::encastf( &udata1.raw_bytes[ 2 ..  4], NE).unwrap(),
	val_u32:  u32::encastf( &udata1.raw_bytes[ 4 ..  8], NE).unwrap(),
	val_u64:  u64::encastf( &udata1.raw_bytes[ 8 .. 16], NE).unwrap(),
	val_u128: u128::encastf(&udata1.raw_bytes[16 .. 32], NE).unwrap(),
    };

    let se_vals_from_raw = UVals1{
	val1_u8:  u8::encastf(  &udata1.raw_bytes[ 0 ..  1], SE).unwrap(),
	val2_u8:  u8::encastf(  &udata1.raw_bytes[ 1 ..  2], SE).unwrap(),
	val_u16:  u16::encastf( &udata1.raw_bytes[ 2 ..  4], SE).unwrap(),
	val_u32:  u32::encastf( &udata1.raw_bytes[ 4 ..  8], SE).unwrap(),
	val_u64:  u64::encastf( &udata1.raw_bytes[ 8 .. 16], SE).unwrap(),
	val_u128: u128::encastf(&udata1.raw_bytes[16 .. 32], SE).unwrap(),
    };

    let le_vals_from_raw = UVals1{
	val1_u8:  u8::encastf(  &udata1.raw_bytes[ 0 ..  1], LE).unwrap(),
	val2_u8:  u8::encastf(  &udata1.raw_bytes[ 1 ..  2], LE).unwrap(),
	val_u16:  u16::encastf( &udata1.raw_bytes[ 2 ..  4], LE).unwrap(),
	val_u32:  u32::encastf( &udata1.raw_bytes[ 4 ..  8], LE).unwrap(),
	val_u64:  u64::encastf( &udata1.raw_bytes[ 8 .. 16], LE).unwrap(),
	val_u128: u128::encastf(&udata1.raw_bytes[16 .. 32], LE).unwrap(),
    };

    let be_vals_from_raw = UVals1{
	val1_u8:  u8::encastf(  &udata1.raw_bytes[ 0 ..  1], BE).unwrap(),
	val2_u8:  u8::encastf(  &udata1.raw_bytes[ 1 ..  2], BE).unwrap(),
	val_u16:  u16::encastf( &udata1.raw_bytes[ 2 ..  4], BE).unwrap(),
	val_u32:  u32::encastf( &udata1.raw_bytes[ 4 ..  8], BE).unwrap(),
	val_u64:  u64::encastf( &udata1.raw_bytes[ 8 .. 16], BE).unwrap(),
	val_u128: u128::encastf(&udata1.raw_bytes[16 .. 32], BE).unwrap(),
    };

    assert_eq!(ne_vals_from_raw, udata1.ne_vals);
    assert_eq!(se_vals_from_raw, udata1.se_vals);
    assert_eq!(le_vals_from_raw, udata1.le_vals);
    assert_eq!(be_vals_from_raw, udata1.be_vals);
}
