use core::mem::size_of;
use std::io::Cursor;

use castflip::DecastIO;
use crate::{FData1, FVals1};


#[test]
fn fdata1() {
    let fdata1 = FData1::gen();

    let mut ne_output = Cursor::new(vec![0_u8; size_of::<FVals1>()]);

    let mut size = 0;
    size += ne_output.decast::<f32>(&fdata1.ne_vals.val1_f32).unwrap();
    size += ne_output.decast::<f32>(&fdata1.ne_vals.val2_f32).unwrap();
    size += ne_output.decast::<f64>(&fdata1.ne_vals.val1_f64).unwrap();
    size += ne_output.decast::<f64>(&fdata1.ne_vals.val2_f64).unwrap();

    assert_eq!(size, size_of::<FVals1>());
    assert_eq!(ne_output.into_inner(), fdata1.ne_bytes);
}
