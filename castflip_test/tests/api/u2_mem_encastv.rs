use castflip::EncastMem;
use crate::{UData2, NELEM2};


#[test]
fn udata2() {
    {
	let udata2 = UData2::gen();

	let ne_vec_from_raw = udata2.raw_bytes.encastv::<u32>(NELEM2).unwrap();
	let se_vec_from_swp = udata2.swp_bytes.encastv::<u32>(NELEM2).unwrap();

	assert_eq!(ne_vec_from_raw, udata2.ne_vals.array.to_vec());
	assert_eq!(se_vec_from_swp, udata2.se_vals.array.to_vec());
    }
    {
	let udata2 = UData2::gen();

	// The type parameter of encast_vec() can be omitted where
	// the Rust compiler can infer the type of the result.
	let ne_vec_from_raw: Vec<u32> =
	    udata2.raw_bytes.encastv(NELEM2).unwrap();
	let se_vec_from_swp: Vec<u32> =
	    udata2.swp_bytes.encastv(NELEM2).unwrap();

	assert_eq!(ne_vec_from_raw, udata2.ne_vals.array.to_vec());
	assert_eq!(se_vec_from_swp, udata2.se_vals.array.to_vec());
    }
}
