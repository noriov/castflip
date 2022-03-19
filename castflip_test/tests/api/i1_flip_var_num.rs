use castflip::{Flip, NE, SE, LE, BE};
use crate::IData1;


#[test]
fn idata1() {
    {
	let idata1 = IData1::gen();

	let mut se_vals_from_ne = idata1.ne_vals;
	se_vals_from_ne.val1_i8  .flip_var_swapped();
	se_vals_from_ne.val2_i8  .flip_var_swapped();
	se_vals_from_ne.val_i16  .flip_var_swapped();
	se_vals_from_ne.val_i32  .flip_var_swapped();
	se_vals_from_ne.val_i64  .flip_var_swapped();
	se_vals_from_ne.val_i128 .flip_var_swapped();

	let mut ne_vals_from_se = idata1.se_vals;
	ne_vals_from_se.val1_i8  .flip_var_swapped();
	ne_vals_from_se.val2_i8  .flip_var_swapped();
	ne_vals_from_se.val_i16  .flip_var_swapped();
	ne_vals_from_se.val_i32  .flip_var_swapped();
	ne_vals_from_se.val_i64  .flip_var_swapped();
	ne_vals_from_se.val_i128 .flip_var_swapped();

	assert_eq!(se_vals_from_ne, idata1.se_vals);
	assert_eq!(ne_vals_from_se, idata1.ne_vals);
    }
    {
	let idata1 = IData1::gen();

	let mut ne_vals_from_ne = idata1.ne_vals;
	ne_vals_from_ne.val1_i8  .flip_var(NE);
	ne_vals_from_ne.val2_i8  .flip_var(NE);
	ne_vals_from_ne.val_i16  .flip_var(NE);
	ne_vals_from_ne.val_i32  .flip_var(NE);
	ne_vals_from_ne.val_i64  .flip_var(NE);
	ne_vals_from_ne.val_i128 .flip_var(NE);

	let mut se_vals_from_ne = idata1.ne_vals;
	se_vals_from_ne.val1_i8  .flip_var(SE);
	se_vals_from_ne.val2_i8  .flip_var(SE);
	se_vals_from_ne.val_i16  .flip_var(SE);
	se_vals_from_ne.val_i32  .flip_var(SE);
	se_vals_from_ne.val_i64  .flip_var(SE);
	se_vals_from_ne.val_i128 .flip_var(SE);

	let mut le_vals_from_ne = idata1.ne_vals;
	le_vals_from_ne.val1_i8  .flip_var(LE);
	le_vals_from_ne.val2_i8  .flip_var(LE);
	le_vals_from_ne.val_i16  .flip_var(LE);
	le_vals_from_ne.val_i32  .flip_var(LE);
	le_vals_from_ne.val_i64  .flip_var(LE);
	le_vals_from_ne.val_i128 .flip_var(LE);

	let mut be_vals_from_ne = idata1.ne_vals;
	be_vals_from_ne.val1_i8  .flip_var(BE);
	be_vals_from_ne.val2_i8  .flip_var(BE);
	be_vals_from_ne.val_i16  .flip_var(BE);
	be_vals_from_ne.val_i32  .flip_var(BE);
	be_vals_from_ne.val_i64  .flip_var(BE);
	be_vals_from_ne.val_i128 .flip_var(BE);

	assert_eq!(ne_vals_from_ne, idata1.ne_vals);
	assert_eq!(se_vals_from_ne, idata1.se_vals);
	assert_eq!(le_vals_from_ne, idata1.le_vals);
	assert_eq!(be_vals_from_ne, idata1.be_vals);
    }
}
