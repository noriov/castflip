use castflip::EncastMem;
use crate::{UData1, UVals1};


#[test]
fn udata1() {
    let udata1 = UData1::gen();

    let ne_vals_from_ne = UVals1 {
	val1_u8:  udata1.ne_bytes[ 0 ..  1].encast::<u8>().unwrap(),
	val2_u8:  udata1.ne_bytes[ 1 ..  2].encast::<u8>().unwrap(),
	val_u16:  udata1.ne_bytes[ 2 ..  4].encast::<u16>().unwrap(),
	val_u32:  udata1.ne_bytes[ 4 ..  8].encast::<u32>().unwrap(),
	val_u64:  udata1.ne_bytes[ 8 .. 16].encast::<u64>().unwrap(),
	val_u128: udata1.ne_bytes[16 .. 32].encast::<u128>().unwrap(),
    };

    assert_eq!(ne_vals_from_ne, udata1.ne_vals);
}
