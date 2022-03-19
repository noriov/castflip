use std::io::Cursor;

use castflip::EncastIO;
use crate::{UData2, NELEM2};


#[test]
fn udata2() {
    {
	let udata2 = UData2::gen();

	let mut raw_input = Cursor::new(udata2.raw_bytes.clone());
	let mut swp_input = Cursor::new(udata2.swp_bytes.clone());

	let ne_vec_from_raw = raw_input.encastv::<u32>(NELEM2).unwrap();
	let se_vec_from_swp = swp_input.encastv::<u32>(NELEM2).unwrap();

	assert_eq!(ne_vec_from_raw, udata2.ne_vals.array.to_vec());
	assert_eq!(se_vec_from_swp, udata2.se_vals.array.to_vec());
    }
    {
	let udata2 = UData2::gen();

	let mut raw_input = Cursor::new(udata2.raw_bytes.clone());
	let mut swp_input = Cursor::new(udata2.swp_bytes.clone());

	// The type parameter of encastv() can be omitted where
	// the Rust compiler can infer the type of the result.
	let ne_vec_from_raw: Vec<u32> = raw_input.encastv(NELEM2).unwrap();
	let se_vec_from_swp: Vec<u32> = swp_input.encastv(NELEM2).unwrap();

	assert_eq!(ne_vec_from_raw, udata2.ne_vals.array.to_vec());
	assert_eq!(se_vec_from_swp, udata2.se_vals.array.to_vec());
    }
}
