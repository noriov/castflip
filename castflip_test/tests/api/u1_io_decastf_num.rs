use core::mem::size_of;
use std::io::Cursor;

use castflip::{DecastIO, NE, SE, LE, BE};
use crate::{UData1, UVals1};


#[test]
fn udata1() {
    let udata1 = UData1::gen();

    let mut ne_output = Cursor::new(vec![0_u8; size_of::<UVals1>()]);
    let mut se_output = Cursor::new(vec![0_u8; size_of::<UVals1>()]);
    let mut le_output = Cursor::new(vec![0_u8; size_of::<UVals1>()]);
    let mut be_output = Cursor::new(vec![0_u8; size_of::<UVals1>()]);

    let mut ne_size = 0;
    ne_size += ne_output.decastf::<u8>(&udata1.ne_vals.val1_u8, NE).unwrap();
    ne_size += ne_output.decastf::<u8>(&udata1.ne_vals.val2_u8, NE).unwrap();
    ne_size += ne_output.decastf::<u16>(&udata1.ne_vals.val_u16, NE).unwrap();
    ne_size += ne_output.decastf::<u32>(&udata1.ne_vals.val_u32, NE).unwrap();
    ne_size += ne_output.decastf::<u64>(&udata1.ne_vals.val_u64, NE).unwrap();
    ne_size += ne_output.decastf::<u128>(&udata1.ne_vals.val_u128, NE).unwrap();

    let mut se_size = 0;
    se_size += se_output.decastf::<u8>(&udata1.ne_vals.val1_u8, SE).unwrap();
    se_size += se_output.decastf::<u8>(&udata1.ne_vals.val2_u8, SE).unwrap();
    se_size += se_output.decastf::<u16>(&udata1.ne_vals.val_u16, SE).unwrap();
    se_size += se_output.decastf::<u32>(&udata1.ne_vals.val_u32, SE).unwrap();
    se_size += se_output.decastf::<u64>(&udata1.ne_vals.val_u64, SE).unwrap();
    se_size += se_output.decastf::<u128>(&udata1.ne_vals.val_u128, SE).unwrap();

    let mut le_size = 0;
    le_size += le_output.decastf::<u8>(&udata1.ne_vals.val1_u8, LE).unwrap();
    le_size += le_output.decastf::<u8>(&udata1.ne_vals.val2_u8, LE).unwrap();
    le_size += le_output.decastf::<u16>(&udata1.ne_vals.val_u16, LE).unwrap();
    le_size += le_output.decastf::<u32>(&udata1.ne_vals.val_u32, LE).unwrap();
    le_size += le_output.decastf::<u64>(&udata1.ne_vals.val_u64, LE).unwrap();
    le_size += le_output.decastf::<u128>(&udata1.ne_vals.val_u128, LE).unwrap();

    let mut be_size = 0;
    be_size += be_output.decastf::<u8>(&udata1.ne_vals.val1_u8, BE).unwrap();
    be_size += be_output.decastf::<u8>(&udata1.ne_vals.val2_u8, BE).unwrap();
    be_size += be_output.decastf::<u16>(&udata1.ne_vals.val_u16, BE).unwrap();
    be_size += be_output.decastf::<u32>(&udata1.ne_vals.val_u32, BE).unwrap();
    be_size += be_output.decastf::<u64>(&udata1.ne_vals.val_u64, BE).unwrap();
    be_size += be_output.decastf::<u128>(&udata1.ne_vals.val_u128, BE).unwrap();

    assert_eq!(ne_size, size_of::<UVals1>());
    assert_eq!(se_size, size_of::<UVals1>());
    assert_eq!(le_size, size_of::<UVals1>());
    assert_eq!(be_size, size_of::<UVals1>());

    assert_eq!(ne_output.into_inner(), udata1.ne_bytes);
    assert_eq!(se_output.into_inner(), udata1.se_bytes);
    assert_eq!(le_output.into_inner(), udata1.le_bytes);
    assert_eq!(be_output.into_inner(), udata1.be_bytes);
}
