use core::mem::size_of;

use castflip::experimental::DecastArg;
use castflip::{NE, SE, LE, BE};
use crate::{IData1, IVals1};


#[test]
fn idata1() {
    let idata1 = IData1::gen();

    let mut raw_bytes_from_ne = [0_u8; size_of::<IVals1>()];
    let mut raw_bytes_from_se = [0_u8; size_of::<IVals1>()];
    let mut raw_bytes_from_le = [0_u8; size_of::<IVals1>()];
    let mut raw_bytes_from_be = [0_u8; size_of::<IVals1>()];

    i8::decastf(  &mut raw_bytes_from_ne[ 0 ..  1],
		  &idata1.ne_vals.val1_i8, NE).unwrap();
    i8::decastf(  &mut raw_bytes_from_ne[ 1 ..  2],
		  &idata1.ne_vals.val2_i8, NE).unwrap();
    i16::decastf( &mut raw_bytes_from_ne[ 2 ..  4],
		  &idata1.ne_vals.val_i16, NE).unwrap();
    i32::decastf( &mut raw_bytes_from_ne[ 4 ..  8],
		  &idata1.ne_vals.val_i32, NE).unwrap();
    i64::decastf( &mut raw_bytes_from_ne[ 8 .. 16],
		  &idata1.ne_vals.val_i64, NE).unwrap();
    i128::decastf(&mut raw_bytes_from_ne[16 .. 32],
		  &idata1.ne_vals.val_i128, NE).unwrap();

    i8::decastf(  &mut raw_bytes_from_se[ 0 ..  1],
		  &idata1.se_vals.val1_i8, SE).unwrap();
    i8::decastf(  &mut raw_bytes_from_se[ 1 ..  2],
		  &idata1.se_vals.val2_i8, SE).unwrap();
    i16::decastf( &mut raw_bytes_from_se[ 2 ..  4],
		  &idata1.se_vals.val_i16, SE).unwrap();
    i32::decastf( &mut raw_bytes_from_se[ 4 ..  8],
		  &idata1.se_vals.val_i32, SE).unwrap();
    i64::decastf( &mut raw_bytes_from_se[ 8 .. 16],
		  &idata1.se_vals.val_i64, SE).unwrap();
    i128::decastf(&mut raw_bytes_from_se[16 .. 32],
		  &idata1.se_vals.val_i128, SE).unwrap();

    i8::decastf(  &mut raw_bytes_from_le[ 0 ..  1],
		  &idata1.le_vals.val1_i8, LE).unwrap();
    i8::decastf(  &mut raw_bytes_from_le[ 1 ..  2],
		  &idata1.le_vals.val2_i8, LE).unwrap();
    i16::decastf( &mut raw_bytes_from_le[ 2 ..  4],
		  &idata1.le_vals.val_i16, LE).unwrap();
    i32::decastf( &mut raw_bytes_from_le[ 4 ..  8],
		  &idata1.le_vals.val_i32, LE).unwrap();
    i64::decastf( &mut raw_bytes_from_le[ 8 .. 16],
		  &idata1.le_vals.val_i64, LE).unwrap();
    i128::decastf(&mut raw_bytes_from_le[16 .. 32],
		  &idata1.le_vals.val_i128, LE).unwrap();

    i8::decastf(  &mut raw_bytes_from_be[ 0 ..  1],
		  &idata1.be_vals.val1_i8, BE).unwrap();
    i8::decastf(  &mut raw_bytes_from_be[ 1 ..  2],
		  &idata1.be_vals.val2_i8, BE).unwrap();
    i16::decastf( &mut raw_bytes_from_be[ 2 ..  4],
		  &idata1.be_vals.val_i16, BE).unwrap();
    i32::decastf( &mut raw_bytes_from_be[ 4 ..  8],
		  &idata1.be_vals.val_i32, BE).unwrap();
    i64::decastf( &mut raw_bytes_from_be[ 8 .. 16],
		  &idata1.be_vals.val_i64, BE).unwrap();
    i128::decastf(&mut raw_bytes_from_be[16 .. 32],
		  &idata1.be_vals.val_i128, BE).unwrap();

    assert_eq!(raw_bytes_from_ne, idata1.raw_bytes);
    assert_eq!(raw_bytes_from_se, idata1.raw_bytes);
    assert_eq!(raw_bytes_from_le, idata1.raw_bytes);
    assert_eq!(raw_bytes_from_be, idata1.raw_bytes);
}
