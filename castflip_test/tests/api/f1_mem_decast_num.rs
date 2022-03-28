use core::mem::size_of;

use castflip::DecastMem;
use crate::{FData1, FVals1};


#[test]
fn fdata1() {
    let fdata1 = FData1::gen();

    let mut ne_bytes = [0_u8; size_of::<FVals1>()];

    let mut off = 0;
    off += ne_bytes[off..].decast::<f32>(&fdata1.ne_vals.val1_f32).unwrap();
    off += ne_bytes[off..].decast::<f32>(&fdata1.ne_vals.val2_f32).unwrap();
    off += ne_bytes[off..].decast::<f64>(&fdata1.ne_vals.val1_f64).unwrap();
    off += ne_bytes[off..].decast::<f64>(&fdata1.ne_vals.val2_f64).unwrap();

    assert_eq!(off, size_of::<FVals1>());
    assert_eq!(ne_bytes, fdata1.ne_bytes);
}
