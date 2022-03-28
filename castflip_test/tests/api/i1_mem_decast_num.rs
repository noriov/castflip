use core::mem::size_of;

use castflip::DecastMem;
use crate::{IData1, IVals1};


#[test]
fn idata1() {
    let idata1 = IData1::gen();

    let mut ne_bytes = [0_u8; size_of::<IVals1>()];

    let mut off = 0;
    off += ne_bytes[off..].decast::<i8>(&idata1.ne_vals.val1_i8).unwrap();
    off += ne_bytes[off..].decast::<i8>(&idata1.ne_vals.val2_i8).unwrap();
    off += ne_bytes[off..].decast::<i16>(&idata1.ne_vals.val_i16).unwrap();
    off += ne_bytes[off..].decast::<i32>(&idata1.ne_vals.val_i32).unwrap();
    off += ne_bytes[off..].decast::<i64>(&idata1.ne_vals.val_i64).unwrap();
    off += ne_bytes[off..].decast::<i128>(&idata1.ne_vals.val_i128).unwrap();

    assert_eq!(off, size_of::<IVals1>());
    assert_eq!(ne_bytes, idata1.ne_bytes);
}
