use castflip::experimental::FlipUnsized;
use castflip::{NE, SE, LE, BE};
use crate::UData2;


#[test]
fn udata2() {
    {
	let udata2 = UData2::gen();

	let mut se_vec_from_ne = udata2.ne_vals.array.to_vec();
	let mut ne_vec_from_se = udata2.se_vals.array.to_vec();

	se_vec_from_ne.flip_var_swapped();
	ne_vec_from_se.flip_var_swapped();

	assert_eq!(se_vec_from_ne, udata2.se_vals.array.to_vec());
	assert_eq!(ne_vec_from_se, udata2.ne_vals.array.to_vec());
    }
    {
	let udata2 = UData2::gen();

	let mut ne_vec_from_ne = udata2.ne_vals.array.to_vec();
	let mut se_vec_from_ne = udata2.ne_vals.array.to_vec();
	let mut le_vec_from_ne = udata2.ne_vals.array.to_vec();
	let mut be_vec_from_ne = udata2.ne_vals.array.to_vec();

	ne_vec_from_ne.flip_var(NE);
	se_vec_from_ne.flip_var(SE);
	le_vec_from_ne.flip_var(LE);
	be_vec_from_ne.flip_var(BE);

	assert_eq!(ne_vec_from_ne, udata2.ne_vals.array.to_vec());
	assert_eq!(se_vec_from_ne, udata2.se_vals.array.to_vec());
	assert_eq!(le_vec_from_ne, udata2.le_vals.array.to_vec());
	assert_eq!(be_vec_from_ne, udata2.be_vals.array.to_vec());
    }
}
