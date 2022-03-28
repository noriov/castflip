use castflip::{Flip, NE, SE, LE, BE};
use crate::UData2;


#[test]
fn udata2() {
    {
	let udata2 = UData2::gen();

	let mut se_box = Box::new(udata2.ne_vals.array);

	se_box.flip_var_swapped();

	assert_eq!(*se_box, udata2.se_vals.array);
    }
    {
	let udata2 = UData2::gen();

	let mut ne_box = Box::new(udata2.ne_vals.array);
	let mut se_box = Box::new(udata2.ne_vals.array);
	let mut le_box = Box::new(udata2.ne_vals.array);
	let mut be_box = Box::new(udata2.ne_vals.array);

	ne_box.flip_var(NE);
	se_box.flip_var(SE);
	le_box.flip_var(LE);
	be_box.flip_var(BE);

	assert_eq!(*ne_box, udata2.ne_vals.array);
	assert_eq!(*se_box, udata2.se_vals.array);
	assert_eq!(*le_box, udata2.le_vals.array);
	assert_eq!(*be_box, udata2.be_vals.array);
    }
}
