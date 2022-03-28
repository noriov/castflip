use core::mem::size_of;
use std::io::Cursor;

use castflip::DecastIO;
use crate::{IData1, IVals1};


#[test]
fn idata1() {
    let idata1 = IData1::gen();

    let mut ne_output = Cursor::new(vec![0_u8; size_of::<IVals1>()]);

    let mut size = 0;
    size += ne_output.decast::<i8>(&idata1.ne_vals.val1_i8).unwrap();
    size += ne_output.decast::<i8>(&idata1.ne_vals.val2_i8).unwrap();
    size += ne_output.decast::<i16>(&idata1.ne_vals.val_i16).unwrap();
    size += ne_output.decast::<i32>(&idata1.ne_vals.val_i32).unwrap();
    size += ne_output.decast::<i64>(&idata1.ne_vals.val_i64).unwrap();
    size += ne_output.decast::<i128>(&idata1.ne_vals.val_i128).unwrap();

    assert_eq!(size, size_of::<IVals1>());
    assert_eq!(ne_output.into_inner(), idata1.ne_bytes);
}
