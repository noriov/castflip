use castflip::{Flip, NE, SE, LE, BE};
use crate::{FData1, FVals1};


#[test]
fn fdata1() {
    {
	let fdata1 = FData1::gen();

	let se_vals = FVals1 {
	    val1_f32:  fdata1.ne_vals.val1_f32  .flip_val_swapped(),
	    val2_f32:  fdata1.ne_vals.val2_f32  .flip_val_swapped(),
	    val1_f64:  fdata1.ne_vals.val1_f64  .flip_val_swapped(),
	    val2_f64:  fdata1.ne_vals.val2_f64  .flip_val_swapped(),
	};

	assert_eq!(se_vals, fdata1.se_vals);
    }
    {
	let fdata1 = FData1::gen();

	let ne_vals = FVals1 {
	    val1_f32:  fdata1.ne_vals.val1_f32  .flip_val(NE),
	    val2_f32:  fdata1.ne_vals.val2_f32  .flip_val(NE),
	    val1_f64:  fdata1.ne_vals.val1_f64  .flip_val(NE),
	    val2_f64:  fdata1.ne_vals.val2_f64  .flip_val(NE),
	};

	let se_vals = FVals1 {
	    val1_f32:  fdata1.ne_vals.val1_f32  .flip_val(SE),
	    val2_f32:  fdata1.ne_vals.val2_f32  .flip_val(SE),
	    val1_f64:  fdata1.ne_vals.val1_f64  .flip_val(SE),
	    val2_f64:  fdata1.ne_vals.val2_f64  .flip_val(SE),
	};

	let le_vals = FVals1 {
	    val1_f32:  fdata1.ne_vals.val1_f32  .flip_val(LE),
	    val2_f32:  fdata1.ne_vals.val2_f32  .flip_val(LE),
	    val1_f64:  fdata1.ne_vals.val1_f64  .flip_val(LE),
	    val2_f64:  fdata1.ne_vals.val2_f64  .flip_val(LE),
	};

	let be_vals = FVals1 {
	    val1_f32:  fdata1.ne_vals.val1_f32  .flip_val(BE),
	    val2_f32:  fdata1.ne_vals.val2_f32  .flip_val(BE),
	    val1_f64:  fdata1.ne_vals.val1_f64  .flip_val(BE),
	    val2_f64:  fdata1.ne_vals.val2_f64  .flip_val(BE),
	};

	assert_eq!(ne_vals, fdata1.ne_vals);
	assert_eq!(se_vals, fdata1.se_vals);
	assert_eq!(le_vals, fdata1.le_vals);
	assert_eq!(be_vals, fdata1.be_vals);
    }
}
