use castflip::experimental::EncastArg;
use crate::{IData1, IVals1};


#[test]
fn idata1() {
    let idata1 = IData1::gen();

    let ne_vals_from_raw = IVals1{
	val1_i8:  i8::encast(  &idata1.raw_bytes[ 0 ..  1]).unwrap(),
	val2_i8:  i8::encast(  &idata1.raw_bytes[ 1 ..  2]).unwrap(),
	val_i16:  i16::encast( &idata1.raw_bytes[ 2 ..  4]).unwrap(),
	val_i32:  i32::encast( &idata1.raw_bytes[ 4 ..  8]).unwrap(),
	val_i64:  i64::encast( &idata1.raw_bytes[ 8 .. 16]).unwrap(),
	val_i128: i128::encast(&idata1.raw_bytes[16 .. 32]).unwrap(),
    };

    assert_eq!(ne_vals_from_raw, idata1.ne_vals);
}
