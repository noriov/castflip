use core::mem::size_of;
use std::io::Cursor;

use castflip::{DecastIO, NE, SE, LE, BE};
use crate::{FData1, FVals1};


#[test]
fn fdata1() {
    let fdata1 = FData1::gen();

    let mut ne_output = Cursor::new(vec![0_u8; size_of::<FVals1>()]);
    let mut se_output = Cursor::new(vec![0_u8; size_of::<FVals1>()]);
    let mut le_output = Cursor::new(vec![0_u8; size_of::<FVals1>()]);
    let mut be_output = Cursor::new(vec![0_u8; size_of::<FVals1>()]);

    let mut ne_size = 0;
    ne_size += ne_output.decastf::<f32>(&fdata1.ne_vals.val1_f32, NE).unwrap();
    ne_size += ne_output.decastf::<f32>(&fdata1.ne_vals.val2_f32, NE).unwrap();
    ne_size += ne_output.decastf::<f64>(&fdata1.ne_vals.val1_f64, NE).unwrap();
    ne_size += ne_output.decastf::<f64>(&fdata1.ne_vals.val2_f64, NE).unwrap();

    let mut se_size = 0;
    se_size += se_output.decastf::<f32>(&fdata1.ne_vals.val1_f32, SE).unwrap();
    se_size += se_output.decastf::<f32>(&fdata1.ne_vals.val2_f32, SE).unwrap();
    se_size += se_output.decastf::<f64>(&fdata1.ne_vals.val1_f64, SE).unwrap();
    se_size += se_output.decastf::<f64>(&fdata1.ne_vals.val2_f64, SE).unwrap();

    let mut le_size = 0;
    le_size += le_output.decastf::<f32>(&fdata1.ne_vals.val1_f32, LE).unwrap();
    le_size += le_output.decastf::<f32>(&fdata1.ne_vals.val2_f32, LE).unwrap();
    le_size += le_output.decastf::<f64>(&fdata1.ne_vals.val1_f64, LE).unwrap();
    le_size += le_output.decastf::<f64>(&fdata1.ne_vals.val2_f64, LE).unwrap();

    let mut be_size = 0;
    be_size += be_output.decastf::<f32>(&fdata1.ne_vals.val1_f32, BE).unwrap();
    be_size += be_output.decastf::<f32>(&fdata1.ne_vals.val2_f32, BE).unwrap();
    be_size += be_output.decastf::<f64>(&fdata1.ne_vals.val1_f64, BE).unwrap();
    be_size += be_output.decastf::<f64>(&fdata1.ne_vals.val2_f64, BE).unwrap();

    assert_eq!(ne_size, size_of::<FVals1>());
    assert_eq!(se_size, size_of::<FVals1>());
    assert_eq!(le_size, size_of::<FVals1>());
    assert_eq!(be_size, size_of::<FVals1>());

    assert_eq!(ne_output.into_inner(), fdata1.ne_bytes);
    assert_eq!(se_output.into_inner(), fdata1.se_bytes);
    assert_eq!(le_output.into_inner(), fdata1.le_bytes);
    assert_eq!(be_output.into_inner(), fdata1.be_bytes);
}
