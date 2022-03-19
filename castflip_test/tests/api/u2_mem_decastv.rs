use core::mem::size_of;

use castflip::DecastMem;
use crate::{UData2, NELEM2};


#[test]
fn udata2() {
    {
	let udata2 = UData2::gen();

	let ne_vec = udata2.ne_vals.array.to_vec();
	let se_vec = udata2.se_vals.array.to_vec();

	let mut raw_bytes_from_ne = [0_u8; size_of::<u32>() * NELEM2];
	let mut swp_bytes_from_se = [0_u8; size_of::<u32>() * NELEM2];

	raw_bytes_from_ne.decastv::<u32>(&ne_vec).unwrap();
	swp_bytes_from_se.decastv::<u32>(&se_vec).unwrap();

	assert_eq!(raw_bytes_from_ne, udata2.raw_bytes);
	assert_eq!(swp_bytes_from_se, udata2.swp_bytes);
    }
    {
	let udata2 = UData2::gen();

	let ne_vec = udata2.ne_vals.array.to_vec();
	let se_vec = udata2.se_vals.array.to_vec();

	let mut raw_bytes_from_ne = [0_u8; size_of::<u32>() * NELEM2];
	let mut swp_bytes_from_se = [0_u8; size_of::<u32>() * NELEM2];

	// The type parameter of decastv() can be omitted where
	// the Rust compiler can infer the type of the result.
	raw_bytes_from_ne.decastv(&ne_vec).unwrap();
	swp_bytes_from_se.decastv(&se_vec).unwrap();

	assert_eq!(raw_bytes_from_ne, udata2.raw_bytes);
	assert_eq!(swp_bytes_from_se, udata2.swp_bytes);
    }
}
