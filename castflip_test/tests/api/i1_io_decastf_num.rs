use core::mem::size_of;
use std::io::Cursor;

use castflip::{DecastIO, NE, SE, LE, BE};
use crate::{IData1, IVals1};


#[test]
fn idata1() {
    let idata1 = IData1::gen();

    let mut ne_output = Cursor::new(vec![0_u8; size_of::<IVals1>()]);
    let mut se_output = Cursor::new(vec![0_u8; size_of::<IVals1>()]);
    let mut le_output = Cursor::new(vec![0_u8; size_of::<IVals1>()]);
    let mut be_output = Cursor::new(vec![0_u8; size_of::<IVals1>()]);

    let mut ne_size = 0;
    ne_size += ne_output.decastf::<i8>(&idata1.ne_vals.val1_i8, NE).unwrap();
    ne_size += ne_output.decastf::<i8>(&idata1.ne_vals.val2_i8, NE).unwrap();
    ne_size += ne_output.decastf::<i16>(&idata1.ne_vals.val_i16, NE).unwrap();
    ne_size += ne_output.decastf::<i32>(&idata1.ne_vals.val_i32, NE).unwrap();
    ne_size += ne_output.decastf::<i64>(&idata1.ne_vals.val_i64, NE).unwrap();
    ne_size += ne_output.decastf::<i128>(&idata1.ne_vals.val_i128, NE).unwrap();

    let mut se_size = 0;
    se_size += se_output.decastf::<i8>(&idata1.ne_vals.val1_i8, SE).unwrap();
    se_size += se_output.decastf::<i8>(&idata1.ne_vals.val2_i8, SE).unwrap();
    se_size += se_output.decastf::<i16>(&idata1.ne_vals.val_i16, SE).unwrap();
    se_size += se_output.decastf::<i32>(&idata1.ne_vals.val_i32, SE).unwrap();
    se_size += se_output.decastf::<i64>(&idata1.ne_vals.val_i64, SE).unwrap();
    se_size += se_output.decastf::<i128>(&idata1.ne_vals.val_i128, SE).unwrap();

    let mut le_size = 0;
    le_size += le_output.decastf::<i8>(&idata1.ne_vals.val1_i8, LE).unwrap();
    le_size += le_output.decastf::<i8>(&idata1.ne_vals.val2_i8, LE).unwrap();
    le_size += le_output.decastf::<i16>(&idata1.ne_vals.val_i16, LE).unwrap();
    le_size += le_output.decastf::<i32>(&idata1.ne_vals.val_i32, LE).unwrap();
    le_size += le_output.decastf::<i64>(&idata1.ne_vals.val_i64, LE).unwrap();
    le_size += le_output.decastf::<i128>(&idata1.ne_vals.val_i128, LE).unwrap();

    let mut be_size = 0;
    be_size += be_output.decastf::<i8>(&idata1.ne_vals.val1_i8, BE).unwrap();
    be_size += be_output.decastf::<i8>(&idata1.ne_vals.val2_i8, BE).unwrap();
    be_size += be_output.decastf::<i16>(&idata1.ne_vals.val_i16, BE).unwrap();
    be_size += be_output.decastf::<i32>(&idata1.ne_vals.val_i32, BE).unwrap();
    be_size += be_output.decastf::<i64>(&idata1.ne_vals.val_i64, BE).unwrap();
    be_size += be_output.decastf::<i128>(&idata1.ne_vals.val_i128, BE).unwrap();

    assert_eq!(ne_size, size_of::<IVals1>());
    assert_eq!(se_size, size_of::<IVals1>());
    assert_eq!(le_size, size_of::<IVals1>());
    assert_eq!(be_size, size_of::<IVals1>());

    assert_eq!(ne_output.into_inner(), idata1.ne_bytes);
    assert_eq!(se_output.into_inner(), idata1.se_bytes);
    assert_eq!(le_output.into_inner(), idata1.le_bytes);
    assert_eq!(be_output.into_inner(), idata1.be_bytes);
}
