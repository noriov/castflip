use castflip::experimental::FlipUnsized;
use castflip::{NE, SE, LE, BE};
use crate::UData2;


#[test]
fn udata2() {
    {
	let udata2 = UData2::gen();

	let expected_ne_tuple = ( udata2.ne_vals.array[0],
				  udata2.ne_vals.array[1],
				  udata2.ne_vals.array[2],
				  udata2.ne_vals.array[3] );
	let expected_se_tuple = ( udata2.se_vals.array[0],
				  udata2.se_vals.array[1],
				  udata2.se_vals.array[2],
				  udata2.se_vals.array[3] );

	let mut se_tuple_from_ne = expected_ne_tuple;
	let mut ne_tuple_from_se = expected_se_tuple;

	se_tuple_from_ne.flip_var_swapped();
	ne_tuple_from_se.flip_var_swapped();

	assert_eq!(se_tuple_from_ne, expected_se_tuple);
	assert_eq!(ne_tuple_from_se, expected_ne_tuple);
    }
    {
	let udata2 = UData2::gen();

	let expected_ne_tuple = ( udata2.ne_vals.array[0],
				  udata2.ne_vals.array[1],
				  udata2.ne_vals.array[2],
				  udata2.ne_vals.array[3] );
	let expected_se_tuple = ( udata2.se_vals.array[0],
				  udata2.se_vals.array[1],
				  udata2.se_vals.array[2],
				  udata2.se_vals.array[3] );
	let expected_le_tuple = ( udata2.le_vals.array[0],
				  udata2.le_vals.array[1],
				  udata2.le_vals.array[2],
				  udata2.le_vals.array[3] );
	let expected_be_tuple = ( udata2.be_vals.array[0],
				  udata2.be_vals.array[1],
				  udata2.be_vals.array[2],
				  udata2.be_vals.array[3] );

	let mut ne_tuple_from_ne = expected_ne_tuple;
	let mut se_tuple_from_ne = expected_ne_tuple;
	let mut le_tuple_from_ne = expected_ne_tuple;
	let mut be_tuple_from_ne = expected_ne_tuple;

	ne_tuple_from_ne.flip_var(NE);
	se_tuple_from_ne.flip_var(SE);
	le_tuple_from_ne.flip_var(LE);
	be_tuple_from_ne.flip_var(BE);

	assert_eq!(ne_tuple_from_ne, expected_ne_tuple);
	assert_eq!(se_tuple_from_ne, expected_se_tuple);
	assert_eq!(le_tuple_from_ne, expected_le_tuple);
	assert_eq!(be_tuple_from_ne, expected_be_tuple);
    }
}
