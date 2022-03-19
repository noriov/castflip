use core::mem::size_of;
use std::io::Cursor;

use castflip::DecastIO;
use crate::{UData2, NELEM2};


#[test]
fn udata2() {
    {
	let udata2 = UData2::gen();

	let mut ne_output = Cursor::new(vec![0_u8; size_of::<u32>() * NELEM2]);
	let mut se_output = Cursor::new(vec![0_u8; size_of::<u32>() * NELEM2]);

	let ne_vec = udata2.ne_vals.array.to_vec();
	let se_vec = udata2.se_vals.array.to_vec();

	ne_output.decastv::<u32>(&ne_vec).unwrap();
	se_output.decastv::<u32>(&se_vec).unwrap();

	assert_eq!(ne_output.into_inner(), udata2.raw_bytes);
	assert_eq!(se_output.into_inner(), udata2.swp_bytes);
    }
    {
	let udata2 = UData2::gen();

	let mut ne_output = Cursor::new(vec![0_u8; size_of::<u32>() * NELEM2]);
	let mut se_output = Cursor::new(vec![0_u8; size_of::<u32>() * NELEM2]);

	let ne_vec = udata2.ne_vals.array.to_vec();
	let se_vec = udata2.se_vals.array.to_vec();

	// The type parameter of decastv() can be omitted where
	// the Rust compiler can infer the type of the result.
	ne_output.decastv(&ne_vec).unwrap();
	se_output.decastv(&se_vec).unwrap();

	assert_eq!(ne_output.into_inner(), udata2.raw_bytes);
	assert_eq!(se_output.into_inner(), udata2.swp_bytes);
    }
}
