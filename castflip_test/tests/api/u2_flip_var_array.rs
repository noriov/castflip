use castflip::{Flip, NE, SE, LE, BE};
use crate::UData2;


#[test]
fn udata2() {
    {
	let udata2 = UData2::gen();

	let mut se_array_from_ne = udata2.ne_vals.array;
	let mut ne_array_from_se = udata2.se_vals.array;

	se_array_from_ne.flip_var_swapped();
	ne_array_from_se.flip_var_swapped();

	assert_eq!(se_array_from_ne, udata2.se_vals.array);
	assert_eq!(ne_array_from_se, udata2.ne_vals.array);
    }
    {
	let udata2 = UData2::gen();

	let mut ne_array_from_ne = udata2.ne_vals.array;
	let mut se_array_from_ne = udata2.ne_vals.array;
	let mut le_array_from_ne = udata2.ne_vals.array;
	let mut be_array_from_ne = udata2.ne_vals.array;

	ne_array_from_ne.flip_var(NE);
	se_array_from_ne.flip_var(SE);
	le_array_from_ne.flip_var(LE);
	be_array_from_ne.flip_var(BE);

	assert_eq!(ne_array_from_ne, udata2.ne_vals.array);
	assert_eq!(se_array_from_ne, udata2.se_vals.array);
	assert_eq!(le_array_from_ne, udata2.le_vals.array);
	assert_eq!(be_array_from_ne, udata2.be_vals.array);
    }
}
