use core::mem::size_of;

use castflip::{DecastMem, NE, SE, LE, BE};
use crate::{FData1, FVals1};


#[test]
fn fdata1() {
    let fdata1 = FData1::gen();

    let mut ne_bytes = [0_u8; size_of::<FVals1>()];
    let mut se_bytes = [0_u8; size_of::<FVals1>()];
    let mut le_bytes = [0_u8; size_of::<FVals1>()];
    let mut be_bytes = [0_u8; size_of::<FVals1>()];

    let mut ne_off = 0;
    ne_off += ne_bytes[ne_off..].decastf::<f32>(&fdata1.ne_vals.val1_f32, NE)
	.unwrap();
    ne_off += ne_bytes[ne_off..].decastf::<f32>(&fdata1.ne_vals.val2_f32, NE)
	.unwrap();
    ne_off += ne_bytes[ne_off..].decastf::<f64>(&fdata1.ne_vals.val1_f64, NE)
	.unwrap();
    ne_off += ne_bytes[ne_off..].decastf::<f64>(&fdata1.ne_vals.val2_f64, NE)
	.unwrap();

    let mut se_off = 0;
    se_off += se_bytes[se_off..].decastf::<f32>(&fdata1.ne_vals.val1_f32, SE)
	.unwrap();
    se_off += se_bytes[se_off..].decastf::<f32>(&fdata1.ne_vals.val2_f32, SE)
	.unwrap();
    se_off += se_bytes[se_off..].decastf::<f64>(&fdata1.ne_vals.val1_f64, SE)
	.unwrap();
    se_off += se_bytes[se_off..].decastf::<f64>(&fdata1.ne_vals.val2_f64, SE)
	.unwrap();

    let mut le_off = 0;
    le_off += le_bytes[le_off..].decastf::<f32>(&fdata1.ne_vals.val1_f32, LE)
	.unwrap();
    le_off += le_bytes[le_off..].decastf::<f32>(&fdata1.ne_vals.val2_f32, LE)
	.unwrap();
    le_off += le_bytes[le_off..].decastf::<f64>(&fdata1.ne_vals.val1_f64, LE)
	.unwrap();
    le_off += le_bytes[le_off..].decastf::<f64>(&fdata1.ne_vals.val2_f64, LE)
	.unwrap();

    let mut be_off = 0;
    be_off += be_bytes[be_off..].decastf::<f32>(&fdata1.ne_vals.val1_f32, BE)
	.unwrap();
    be_off += be_bytes[be_off..].decastf::<f32>(&fdata1.ne_vals.val2_f32, BE)
	.unwrap();
    be_off += be_bytes[be_off..].decastf::<f64>(&fdata1.ne_vals.val1_f64, BE)
	.unwrap();
    be_off += be_bytes[be_off..].decastf::<f64>(&fdata1.ne_vals.val2_f64, BE)
	.unwrap();

    assert_eq!(ne_off, size_of::<FVals1>());
    assert_eq!(se_off, size_of::<FVals1>());
    assert_eq!(le_off, size_of::<FVals1>());
    assert_eq!(be_off, size_of::<FVals1>());

    assert_eq!(ne_bytes, fdata1.ne_bytes);
    assert_eq!(se_bytes, fdata1.se_bytes);
    assert_eq!(le_bytes, fdata1.le_bytes);
    assert_eq!(be_bytes, fdata1.be_bytes);
}
