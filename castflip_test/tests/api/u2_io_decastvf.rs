use core::mem::size_of;
use std::io::Cursor;

use castflip::{DecastIO, NE, SE, LE, BE};
use crate::{UData2, NELEM2};


#[test]
fn udata2() {
    {
	let udata2 = UData2::gen();

	let mut ne_output = Cursor::new(vec![0_u8; size_of::<u32>() * NELEM2]);
	let mut se_output = Cursor::new(vec![0_u8; size_of::<u32>() * NELEM2]);
	let mut le_output = Cursor::new(vec![0_u8; size_of::<u32>() * NELEM2]);
	let mut be_output = Cursor::new(vec![0_u8; size_of::<u32>() * NELEM2]);

	let ne_vec = udata2.ne_vals.array.to_vec();
	let se_vec = udata2.se_vals.array.to_vec();
	let le_vec = udata2.le_vals.array.to_vec();
	let be_vec = udata2.be_vals.array.to_vec();

	ne_output.decastvf::<u32>(&ne_vec, NE).unwrap();
	se_output.decastvf::<u32>(&se_vec, SE).unwrap();
	le_output.decastvf::<u32>(&le_vec, LE).unwrap();
	be_output.decastvf::<u32>(&be_vec, BE).unwrap();

	assert_eq!(ne_output.into_inner(), udata2.raw_bytes);
	assert_eq!(se_output.into_inner(), udata2.raw_bytes);
	assert_eq!(le_output.into_inner(), udata2.raw_bytes);
	assert_eq!(be_output.into_inner(), udata2.raw_bytes);
    }
    {
	let udata2 = UData2::gen();

	let mut ne_output = Cursor::new(vec![0_u8; size_of::<u32>() * NELEM2]);
	let mut se_output = Cursor::new(vec![0_u8; size_of::<u32>() * NELEM2]);
	let mut le_output = Cursor::new(vec![0_u8; size_of::<u32>() * NELEM2]);
	let mut be_output = Cursor::new(vec![0_u8; size_of::<u32>() * NELEM2]);

	let ne_vec = udata2.ne_vals.array.to_vec();
	let se_vec = udata2.se_vals.array.to_vec();
	let le_vec = udata2.le_vals.array.to_vec();
	let be_vec = udata2.be_vals.array.to_vec();

	// The type parameter of decastvf() can be omitted where
	// the Rust compiler can infer the type of the result.
	ne_output.decastvf(&ne_vec, NE).unwrap();
	se_output.decastvf(&se_vec, SE).unwrap();
	le_output.decastvf(&le_vec, LE).unwrap();
	be_output.decastvf(&be_vec, BE).unwrap();

	assert_eq!(ne_output.into_inner(), udata2.raw_bytes);
	assert_eq!(se_output.into_inner(), udata2.raw_bytes);
	assert_eq!(le_output.into_inner(), udata2.raw_bytes);
	assert_eq!(be_output.into_inner(), udata2.raw_bytes);
    }
}
