use core::mem::size_of;

use castflip::experimental::DecastArg;
use castflip::{NE, SE, LE, BE};

use crate::{UData1, UVals1};


#[test]
fn udata1() {
    let udata1 = UData1::gen();

    let mut ne_bytes = [0_u8; size_of::<UVals1>()];
    let mut se_bytes = [0_u8; size_of::<UVals1>()];
    let mut le_bytes = [0_u8; size_of::<UVals1>()];
    let mut be_bytes = [0_u8; size_of::<UVals1>()];

    u8::decastf(  &mut ne_bytes[ 0 ..  1],
                  &udata1.ne_vals.val1_u8, NE).unwrap();
    u8::decastf(  &mut ne_bytes[ 1 ..  2],
                  &udata1.ne_vals.val2_u8, NE).unwrap();
    u16::decastf( &mut ne_bytes[ 2 ..  4],
                  &udata1.ne_vals.val_u16, NE).unwrap();
    u32::decastf( &mut ne_bytes[ 4 ..  8],
                  &udata1.ne_vals.val_u32, NE).unwrap();
    u64::decastf( &mut ne_bytes[ 8 .. 16],
                  &udata1.ne_vals.val_u64, NE).unwrap();
    u128::decastf(&mut ne_bytes[16 .. 32],
                  &udata1.ne_vals.val_u128, NE).unwrap();

    u8::decastf(  &mut se_bytes[ 0 ..  1],
                  &udata1.ne_vals.val1_u8, SE).unwrap();
    u8::decastf(  &mut se_bytes[ 1 ..  2],
                  &udata1.ne_vals.val2_u8, SE).unwrap();
    u16::decastf( &mut se_bytes[ 2 ..  4],
                  &udata1.ne_vals.val_u16, SE).unwrap();
    u32::decastf( &mut se_bytes[ 4 ..  8],
                  &udata1.ne_vals.val_u32, SE).unwrap();
    u64::decastf( &mut se_bytes[ 8 .. 16],
                  &udata1.ne_vals.val_u64, SE).unwrap();
    u128::decastf(&mut se_bytes[16 .. 32],
                  &udata1.ne_vals.val_u128, SE).unwrap();

    u8::decastf(  &mut le_bytes[ 0 ..  1],
                  &udata1.ne_vals.val1_u8, LE).unwrap();
    u8::decastf(  &mut le_bytes[ 1 ..  2],
                  &udata1.ne_vals.val2_u8, LE).unwrap();
    u16::decastf( &mut le_bytes[ 2 ..  4],
                  &udata1.ne_vals.val_u16, LE).unwrap();
    u32::decastf( &mut le_bytes[ 4 ..  8],
                  &udata1.ne_vals.val_u32, LE).unwrap();
    u64::decastf( &mut le_bytes[ 8 .. 16],
                  &udata1.ne_vals.val_u64, LE).unwrap();
    u128::decastf(&mut le_bytes[16 .. 32],
                  &udata1.le_vals.val_u128, LE).unwrap();

    u8::decastf(  &mut be_bytes[ 0 ..  1],
                  &udata1.ne_vals.val1_u8, BE).unwrap();
    u8::decastf(  &mut be_bytes[ 1 ..  2],
                  &udata1.ne_vals.val2_u8, BE).unwrap();
    u16::decastf( &mut be_bytes[ 2 ..  4],
                  &udata1.ne_vals.val_u16, BE).unwrap();
    u32::decastf( &mut be_bytes[ 4 ..  8],
                  &udata1.ne_vals.val_u32, BE).unwrap();
    u64::decastf( &mut be_bytes[ 8 .. 16],
                  &udata1.ne_vals.val_u64, BE).unwrap();
    u128::decastf(&mut be_bytes[16 .. 32],
                  &udata1.ne_vals.val_u128, BE).unwrap();

    assert_eq!(ne_bytes, udata1.ne_bytes);
    assert_eq!(se_bytes, udata1.se_bytes);
    assert_eq!(le_bytes, udata1.le_bytes);
    assert_eq!(be_bytes, udata1.be_bytes);
}
