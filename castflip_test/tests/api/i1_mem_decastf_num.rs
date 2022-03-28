use core::mem::size_of;

use castflip::{DecastMem, NE, SE, LE, BE};

use crate::{IData1, IVals1};


#[test]
fn idata1() {
    let idata1 = IData1::gen();

    let mut ne_bytes = [0_u8; size_of::<IVals1>()];
    let mut se_bytes = [0_u8; size_of::<IVals1>()];
    let mut le_bytes = [0_u8; size_of::<IVals1>()];
    let mut be_bytes = [0_u8; size_of::<IVals1>()];

    let mut ne_off = 0;
    ne_off += ne_bytes[ne_off..].decastf::<i8>(&idata1.ne_vals.val1_i8, NE)
	.unwrap();
    ne_off += ne_bytes[ne_off..].decastf::<i8>(&idata1.ne_vals.val2_i8, NE)
	.unwrap();
    ne_off += ne_bytes[ne_off..].decastf::<i16>(&idata1.ne_vals.val_i16, NE)
	.unwrap();
    ne_off += ne_bytes[ne_off..].decastf::<i32>(&idata1.ne_vals.val_i32, NE)
	.unwrap();
    ne_off += ne_bytes[ne_off..].decastf::<i64>(&idata1.ne_vals.val_i64, NE)
	.unwrap();
    ne_off += ne_bytes[ne_off..].decastf::<i128>(&idata1.ne_vals.val_i128, NE)
	.unwrap();

    let mut se_off = 0;
    se_off += se_bytes[se_off..].decastf::<i8>(&idata1.ne_vals.val1_i8, SE)
	.unwrap();
    se_off += se_bytes[se_off..].decastf::<i8>(&idata1.ne_vals.val2_i8, SE)
	.unwrap();
    se_off += se_bytes[se_off..].decastf::<i16>(&idata1.ne_vals.val_i16, SE)
	.unwrap();
    se_off += se_bytes[se_off..].decastf::<i32>(&idata1.ne_vals.val_i32, SE)
	.unwrap();
    se_off += se_bytes[se_off..].decastf::<i64>(&idata1.ne_vals.val_i64, SE)
	.unwrap();
    se_off += se_bytes[se_off..].decastf::<i128>(&idata1.ne_vals.val_i128, SE)
	.unwrap();

    let mut le_off = 0;
    le_off += le_bytes[le_off..].decastf::<i8>(&idata1.ne_vals.val1_i8, LE)
	.unwrap();
    le_off += le_bytes[le_off..].decastf::<i8>(&idata1.ne_vals.val2_i8, LE)
	.unwrap();
    le_off += le_bytes[le_off..].decastf::<i16>(&idata1.ne_vals.val_i16, LE)
	.unwrap();
    le_off += le_bytes[le_off..].decastf::<i32>(&idata1.ne_vals.val_i32, LE)
	.unwrap();
    le_off += le_bytes[le_off..].decastf::<i64>(&idata1.ne_vals.val_i64, LE)
	.unwrap();
    le_off += le_bytes[le_off..].decastf::<i128>(&idata1.ne_vals.val_i128, LE)
	.unwrap();

    let mut be_off = 0;
    be_off += be_bytes[be_off..].decastf::<i8>(&idata1.ne_vals.val1_i8, BE)
	.unwrap();
    be_off += be_bytes[be_off..].decastf::<i8>(&idata1.ne_vals.val2_i8, BE)
	.unwrap();
    be_off += be_bytes[be_off..].decastf::<i16>(&idata1.ne_vals.val_i16, BE)
	.unwrap();
    be_off += be_bytes[be_off..].decastf::<i32>(&idata1.ne_vals.val_i32, BE)
	.unwrap();
    be_off += be_bytes[be_off..].decastf::<i64>(&idata1.ne_vals.val_i64, BE)
	.unwrap();
    be_off += be_bytes[be_off..].decastf::<i128>(&idata1.ne_vals.val_i128, BE)
	.unwrap();

    assert_eq!(ne_off, size_of::<IVals1>());
    assert_eq!(se_off, size_of::<IVals1>());
    assert_eq!(le_off, size_of::<IVals1>());
    assert_eq!(be_off, size_of::<IVals1>());

    assert_eq!(ne_bytes, idata1.ne_bytes);
    assert_eq!(se_bytes, idata1.se_bytes);
    assert_eq!(le_bytes, idata1.le_bytes);
    assert_eq!(be_bytes, idata1.be_bytes);
}
