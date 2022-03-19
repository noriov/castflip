use core::mem::size_of;

use castflip::experimental::DecastArg;
use crate::{UData1, UVals1};


#[test]
fn udata1() {
    let udata1 = UData1::gen();

    let mut raw_bytes_from_ne = [0_u8; size_of::<UVals1>()];

    u8::decast(  &mut raw_bytes_from_ne[ 0 ..  1],
		  &udata1.ne_vals.val1_u8).unwrap();
    u8::decast(  &mut raw_bytes_from_ne[ 1 ..  2],
		  &udata1.ne_vals.val2_u8).unwrap();
    u16::decast( &mut raw_bytes_from_ne[ 2 ..  4],
		  &udata1.ne_vals.val_u16).unwrap();
    u32::decast( &mut raw_bytes_from_ne[ 4 ..  8],
		  &udata1.ne_vals.val_u32).unwrap();
    u64::decast( &mut raw_bytes_from_ne[ 8 .. 16],
		  &udata1.ne_vals.val_u64).unwrap();
    u128::decast(&mut raw_bytes_from_ne[16 .. 32],
		 &udata1.ne_vals.val_u128).unwrap();

    assert_eq!(raw_bytes_from_ne, udata1.raw_bytes);
}
