use castflip::{Flip, NE, SE, LE, BE};
use crate::{IData1, IVals1};


#[test]
fn idata1() {
    {
	let idata1 = IData1::gen();

	let se_vals_from_ne = IVals1 {
	    val1_i8:  idata1.ne_vals.val1_i8.flip_val_swapped(),
	    val2_i8:  idata1.ne_vals.val2_i8.flip_val_swapped(),
	    val_i16:  idata1.ne_vals.val_i16.flip_val_swapped(),
	    val_i32:  idata1.ne_vals.val_i32.flip_val_swapped(),
	    val_i64:  idata1.ne_vals.val_i64.flip_val_swapped(),
	    val_i128: idata1.ne_vals.val_i128.flip_val_swapped(),
	};

	let ne_vals_from_se = IVals1 {
	    val1_i8:  idata1.se_vals.val1_i8.flip_val_swapped(),
	    val2_i8:  idata1.se_vals.val2_i8.flip_val_swapped(),
	    val_i16:  idata1.se_vals.val_i16.flip_val_swapped(),
	    val_i32:  idata1.se_vals.val_i32.flip_val_swapped(),
	    val_i64:  idata1.se_vals.val_i64.flip_val_swapped(),
	    val_i128: idata1.se_vals.val_i128.flip_val_swapped(),
	};

	assert_eq!(se_vals_from_ne, idata1.se_vals);
	assert_eq!(ne_vals_from_se, idata1.ne_vals);
    }
    {
	let idata1 = IData1::gen();

	let ne_vals_from_ne = IVals1 {
	    val1_i8:  idata1.ne_vals.val1_i8.flip_val(NE),
	    val2_i8:  idata1.ne_vals.val2_i8.flip_val(NE),
	    val_i16:  idata1.ne_vals.val_i16.flip_val(NE),
	    val_i32:  idata1.ne_vals.val_i32.flip_val(NE),
	    val_i64:  idata1.ne_vals.val_i64.flip_val(NE),
	    val_i128: idata1.ne_vals.val_i128.flip_val(NE),
	};

	let se_vals_from_ne = IVals1 {
	    val1_i8:  idata1.ne_vals.val1_i8.flip_val(SE),
	    val2_i8:  idata1.ne_vals.val2_i8.flip_val(SE),
	    val_i16:  idata1.ne_vals.val_i16.flip_val(SE),
	    val_i32:  idata1.ne_vals.val_i32.flip_val(SE),
	    val_i64:  idata1.ne_vals.val_i64.flip_val(SE),
	    val_i128: idata1.ne_vals.val_i128.flip_val(SE),
	};

	let le_vals_from_ne = IVals1 {
	    val1_i8:  idata1.ne_vals.val1_i8.flip_val(LE),
	    val2_i8:  idata1.ne_vals.val2_i8.flip_val(LE),
	    val_i16:  idata1.ne_vals.val_i16.flip_val(LE),
	    val_i32:  idata1.ne_vals.val_i32.flip_val(LE),
	    val_i64:  idata1.ne_vals.val_i64.flip_val(LE),
	    val_i128: idata1.ne_vals.val_i128.flip_val(LE),
	};

	let be_vals_from_ne = IVals1 {
	    val1_i8:  idata1.ne_vals.val1_i8.flip_val(BE),
	    val2_i8:  idata1.ne_vals.val2_i8.flip_val(BE),
	    val_i16:  idata1.ne_vals.val_i16.flip_val(BE),
	    val_i32:  idata1.ne_vals.val_i32.flip_val(BE),
	    val_i64:  idata1.ne_vals.val_i64.flip_val(BE),
	    val_i128: idata1.ne_vals.val_i128.flip_val(BE),
	};

	assert_eq!(ne_vals_from_ne, idata1.ne_vals);
	assert_eq!(se_vals_from_ne, idata1.se_vals);
	assert_eq!(le_vals_from_ne, idata1.le_vals);
	assert_eq!(be_vals_from_ne, idata1.be_vals);
    }
}
