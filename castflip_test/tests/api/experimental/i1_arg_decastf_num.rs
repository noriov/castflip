use core::mem::size_of;

use castflip::experimental::DecastArg;
use castflip::{NE, SE, LE, BE};
use crate::{IData1, IVals1};


#[test]
fn idata1() {
    let idata1 = IData1::gen();

    let mut ne_bytes = [0_u8; size_of::<IVals1>()];
    let mut se_bytes = [0_u8; size_of::<IVals1>()];
    let mut le_bytes = [0_u8; size_of::<IVals1>()];
    let mut be_bytes = [0_u8; size_of::<IVals1>()];

    i8::decastf(  &mut ne_bytes[ 0 ..  1],
		  &idata1.ne_vals.val1_i8, NE).unwrap();
    i8::decastf(  &mut ne_bytes[ 1 ..  2],
		  &idata1.ne_vals.val2_i8, NE).unwrap();
    i16::decastf( &mut ne_bytes[ 2 ..  4],
		  &idata1.ne_vals.val_i16, NE).unwrap();
    i32::decastf( &mut ne_bytes[ 4 ..  8],
		  &idata1.ne_vals.val_i32, NE).unwrap();
    i64::decastf( &mut ne_bytes[ 8 .. 16],
		  &idata1.ne_vals.val_i64, NE).unwrap();
    i128::decastf(&mut ne_bytes[16 .. 32],
		  &idata1.ne_vals.val_i128, NE).unwrap();

    i8::decastf(  &mut se_bytes[ 0 ..  1],
		  &idata1.ne_vals.val1_i8, SE).unwrap();
    i8::decastf(  &mut se_bytes[ 1 ..  2],
		  &idata1.ne_vals.val2_i8, SE).unwrap();
    i16::decastf( &mut se_bytes[ 2 ..  4],
		  &idata1.ne_vals.val_i16, SE).unwrap();
    i32::decastf( &mut se_bytes[ 4 ..  8],
		  &idata1.ne_vals.val_i32, SE).unwrap();
    i64::decastf( &mut se_bytes[ 8 .. 16],
		  &idata1.ne_vals.val_i64, SE).unwrap();
    i128::decastf(&mut se_bytes[16 .. 32],
		  &idata1.ne_vals.val_i128, SE).unwrap();

    i8::decastf(  &mut le_bytes[ 0 ..  1],
		  &idata1.ne_vals.val1_i8, LE).unwrap();
    i8::decastf(  &mut le_bytes[ 1 ..  2],
		  &idata1.ne_vals.val2_i8, LE).unwrap();
    i16::decastf( &mut le_bytes[ 2 ..  4],
		  &idata1.ne_vals.val_i16, LE).unwrap();
    i32::decastf( &mut le_bytes[ 4 ..  8],
		  &idata1.ne_vals.val_i32, LE).unwrap();
    i64::decastf( &mut le_bytes[ 8 .. 16],
		  &idata1.ne_vals.val_i64, LE).unwrap();
    i128::decastf(&mut le_bytes[16 .. 32],
		  &idata1.ne_vals.val_i128, LE).unwrap();

    i8::decastf(  &mut be_bytes[ 0 ..  1],
		  &idata1.ne_vals.val1_i8, BE).unwrap();
    i8::decastf(  &mut be_bytes[ 1 ..  2],
		  &idata1.ne_vals.val2_i8, BE).unwrap();
    i16::decastf( &mut be_bytes[ 2 ..  4],
		  &idata1.ne_vals.val_i16, BE).unwrap();
    i32::decastf( &mut be_bytes[ 4 ..  8],
		  &idata1.ne_vals.val_i32, BE).unwrap();
    i64::decastf( &mut be_bytes[ 8 .. 16],
		  &idata1.ne_vals.val_i64, BE).unwrap();
    i128::decastf(&mut be_bytes[16 .. 32],
		  &idata1.ne_vals.val_i128, BE).unwrap();

    assert_eq!(ne_bytes, idata1.ne_bytes);
    assert_eq!(se_bytes, idata1.se_bytes);
    assert_eq!(le_bytes, idata1.le_bytes);
    assert_eq!(be_bytes, idata1.be_bytes);
}
