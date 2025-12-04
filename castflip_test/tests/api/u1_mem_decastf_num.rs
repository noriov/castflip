use core::mem::size_of;

use castflip::{DecastMem, NE, SE, LE, BE};
use crate::{UData1, UVals1};


#[test]
fn udata1() {
    let udata1 = UData1::gen();

    let mut ne_bytes = [0_u8; size_of::<UVals1>()];
    let mut se_bytes = [0_u8; size_of::<UVals1>()];
    let mut le_bytes = [0_u8; size_of::<UVals1>()];
    let mut be_bytes = [0_u8; size_of::<UVals1>()];

    let mut ne_off = 0;
    ne_off += ne_bytes[ne_off..].decastf::<u8>(&udata1.ne_vals.val1_u8, NE)
        .unwrap();
    ne_off += ne_bytes[ne_off..].decastf::<u8>(&udata1.ne_vals.val2_u8, NE)
        .unwrap();
    ne_off += ne_bytes[ne_off..].decastf::<u16>(&udata1.ne_vals.val_u16, NE)
        .unwrap();
    ne_off += ne_bytes[ne_off..].decastf::<u32>(&udata1.ne_vals.val_u32, NE)
        .unwrap();
    ne_off += ne_bytes[ne_off..].decastf::<u64>(&udata1.ne_vals.val_u64, NE)
        .unwrap();
    ne_off += ne_bytes[ne_off..].decastf::<u128>(&udata1.ne_vals.val_u128, NE)
        .unwrap();

    let mut se_off = 0;
    se_off += se_bytes[se_off..].decastf::<u8>(&udata1.ne_vals.val1_u8, SE)
        .unwrap();
    se_off += se_bytes[se_off..].decastf::<u8>(&udata1.ne_vals.val2_u8, SE)
        .unwrap();
    se_off += se_bytes[se_off..].decastf::<u16>(&udata1.ne_vals.val_u16, SE)
        .unwrap();
    se_off += se_bytes[se_off..].decastf::<u32>(&udata1.ne_vals.val_u32, SE)
        .unwrap();
    se_off += se_bytes[se_off..].decastf::<u64>(&udata1.ne_vals.val_u64, SE)
        .unwrap();
    se_off += se_bytes[se_off..].decastf::<u128>(&udata1.ne_vals.val_u128, SE)
        .unwrap();

    let mut le_off = 0;
    le_off += le_bytes[le_off..].decastf::<u8>(&udata1.ne_vals.val1_u8, LE)
        .unwrap();
    le_off += le_bytes[le_off..].decastf::<u8>(&udata1.ne_vals.val2_u8, LE)
        .unwrap();
    le_off += le_bytes[le_off..].decastf::<u16>(&udata1.ne_vals.val_u16, LE)
        .unwrap();
    le_off += le_bytes[le_off..].decastf::<u32>(&udata1.ne_vals.val_u32, LE)
        .unwrap();
    le_off += le_bytes[le_off..].decastf::<u64>(&udata1.ne_vals.val_u64, LE)
        .unwrap();
    le_off += le_bytes[le_off..].decastf::<u128>(&udata1.ne_vals.val_u128, LE)
        .unwrap();

    let mut be_off = 0;
    be_off += be_bytes[be_off..].decastf::<u8>(&udata1.ne_vals.val1_u8, BE)
        .unwrap();
    be_off += be_bytes[be_off..].decastf::<u8>(&udata1.ne_vals.val2_u8, BE)
        .unwrap();
    be_off += be_bytes[be_off..].decastf::<u16>(&udata1.ne_vals.val_u16, BE)
        .unwrap();
    be_off += be_bytes[be_off..].decastf::<u32>(&udata1.ne_vals.val_u32, BE)
        .unwrap();
    be_off += be_bytes[be_off..].decastf::<u64>(&udata1.ne_vals.val_u64, BE)
        .unwrap();
    be_off += be_bytes[be_off..].decastf::<u128>(&udata1.ne_vals.val_u128, BE)
        .unwrap();

    assert_eq!(ne_off, size_of::<UVals1>());
    assert_eq!(se_off, size_of::<UVals1>());
    assert_eq!(le_off, size_of::<UVals1>());
    assert_eq!(be_off, size_of::<UVals1>());

    assert_eq!(ne_bytes, udata1.ne_bytes);
    assert_eq!(se_bytes, udata1.se_bytes);
    assert_eq!(le_bytes, udata1.le_bytes);
    assert_eq!(be_bytes, udata1.be_bytes);
}
