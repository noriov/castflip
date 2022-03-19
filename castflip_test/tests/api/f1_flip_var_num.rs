use castflip::{Flip, NE, SE, LE, BE};
use crate::FData1;


#[test]
fn fdata1() {
    {
	let fdata1 = FData1::gen();

	let mut se_vals_from_ne = fdata1.ne_vals;
	se_vals_from_ne.val1_f32  .flip_var_swapped();
	se_vals_from_ne.val2_f32  .flip_var_swapped();
	se_vals_from_ne.val1_f64  .flip_var_swapped();
	se_vals_from_ne.val2_f64  .flip_var_swapped();

	let mut ne_vals_from_se = fdata1.se_vals;
	ne_vals_from_se.val1_f32  .flip_var_swapped();
	ne_vals_from_se.val2_f32  .flip_var_swapped();
	ne_vals_from_se.val1_f64  .flip_var_swapped();
	ne_vals_from_se.val2_f64  .flip_var_swapped();

	assert_eq!(se_vals_from_ne, fdata1.se_vals);
	assert_eq!(ne_vals_from_se, fdata1.ne_vals);
    }
    {
	let fdata1 = FData1::gen();

	let mut ne_vals_from_ne = fdata1.ne_vals;
	ne_vals_from_ne.val1_f32  .flip_var(NE);
	ne_vals_from_ne.val2_f32  .flip_var(NE);
	ne_vals_from_ne.val1_f64  .flip_var(NE);
	ne_vals_from_ne.val2_f64  .flip_var(NE);

	let mut se_vals_from_ne = fdata1.ne_vals;
	se_vals_from_ne.val1_f32  .flip_var(SE);
	se_vals_from_ne.val2_f32  .flip_var(SE);
	se_vals_from_ne.val1_f64  .flip_var(SE);
	se_vals_from_ne.val2_f64  .flip_var(SE);

	let mut le_vals_from_ne = fdata1.ne_vals;
	le_vals_from_ne.val1_f32  .flip_var(LE);
	le_vals_from_ne.val2_f32  .flip_var(LE);
	le_vals_from_ne.val1_f64  .flip_var(LE);
	le_vals_from_ne.val2_f64  .flip_var(LE);

	let mut be_vals_from_ne = fdata1.ne_vals;
	be_vals_from_ne.val1_f32  .flip_var(BE);
	be_vals_from_ne.val2_f32  .flip_var(BE);
	be_vals_from_ne.val1_f64  .flip_var(BE);
	be_vals_from_ne.val2_f64  .flip_var(BE);

	assert_eq!(ne_vals_from_ne, fdata1.ne_vals);
	assert_eq!(se_vals_from_ne, fdata1.se_vals);
	assert_eq!(le_vals_from_ne, fdata1.le_vals);
	assert_eq!(be_vals_from_ne, fdata1.be_vals);
    }
}
