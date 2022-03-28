use core::mem::size_of;

use castflip::experimental::DecastArg;
use crate::{IData1, IVals1};


#[test]
fn idata1() {
    let idata1 = IData1::gen();

    let mut ne_bytes = [0_u8; size_of::<IVals1>()];

    i8::decast(  &mut ne_bytes[ 0 ..  1], &idata1.ne_vals.val1_i8).unwrap();
    i8::decast(  &mut ne_bytes[ 1 ..  2], &idata1.ne_vals.val2_i8).unwrap();
    i16::decast( &mut ne_bytes[ 2 ..  4], &idata1.ne_vals.val_i16).unwrap();
    i32::decast( &mut ne_bytes[ 4 ..  8], &idata1.ne_vals.val_i32).unwrap();
    i64::decast( &mut ne_bytes[ 8 .. 16], &idata1.ne_vals.val_i64).unwrap();
    i128::decast(&mut ne_bytes[16 .. 32], &idata1.ne_vals.val_i128).unwrap();

    assert_eq!(ne_bytes, idata1.ne_bytes);
}
