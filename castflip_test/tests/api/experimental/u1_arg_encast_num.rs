use castflip::experimental::EncastArg;
use crate::{UData1, UVals1};


#[test]
fn udata1() {
    let udata1 = UData1::gen();

    let ne_vals_from_ne = UVals1 {
	val1_u8:  u8::encast(  &udata1.ne_bytes[ 0 ..  1]).unwrap(),
	val2_u8:  u8::encast(  &udata1.ne_bytes[ 1 ..  2]).unwrap(),
	val_u16:  u16::encast( &udata1.ne_bytes[ 2 ..  4]).unwrap(),
	val_u32:  u32::encast( &udata1.ne_bytes[ 4 ..  8]).unwrap(),
	val_u64:  u64::encast( &udata1.ne_bytes[ 8 .. 16]).unwrap(),
	val_u128: u128::encast(&udata1.ne_bytes[16 .. 32]).unwrap(),
    };

    assert_eq!(ne_vals_from_ne, udata1.ne_vals);
}
