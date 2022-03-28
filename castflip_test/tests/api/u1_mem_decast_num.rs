use core::mem::size_of;

use castflip::DecastMem;
use crate::{UData1, UVals1};


#[test]
fn udata1() {
    let udata1 = UData1::gen();

    let mut ne_bytes = [0_u8; size_of::<UVals1>()];

    let mut off = 0;
    off += ne_bytes[off..].decast::<u8>(&udata1.ne_vals.val1_u8).unwrap();
    off += ne_bytes[off..].decast::<u8>(&udata1.ne_vals.val2_u8).unwrap();
    off += ne_bytes[off..].decast::<u16>(&udata1.ne_vals.val_u16).unwrap();
    off += ne_bytes[off..].decast::<u32>(&udata1.ne_vals.val_u32).unwrap();
    off += ne_bytes[off..].decast::<u64>(&udata1.ne_vals.val_u64).unwrap();
    off += ne_bytes[off..].decast::<u128>(&udata1.ne_vals.val_u128).unwrap();

    assert_eq!(off, size_of::<UVals1>());
    assert_eq!(ne_bytes, udata1.ne_bytes);
}
