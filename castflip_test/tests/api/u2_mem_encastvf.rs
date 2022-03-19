use castflip::{EncastMem, NE, SE, LE, BE};
use crate::{UData2, NELEM2};


#[test]
fn udata2() {
    {
	let udata2 = UData2::gen();

	let ne_vec_from_raw =
	    udata2.raw_bytes.encastvf::<u32>(NELEM2, NE).unwrap();
	let se_vec_from_raw =
	    udata2.raw_bytes.encastvf::<u32>(NELEM2, SE).unwrap();
	let le_vec_from_raw =
	    udata2.raw_bytes.encastvf::<u32>(NELEM2, LE).unwrap();
	let be_vec_from_raw =
	    udata2.raw_bytes.encastvf::<u32>(NELEM2, BE).unwrap();

	assert_eq!(ne_vec_from_raw, udata2.ne_vals.array.to_vec());
	assert_eq!(se_vec_from_raw, udata2.se_vals.array.to_vec());
	assert_eq!(le_vec_from_raw, udata2.le_vals.array.to_vec());
	assert_eq!(be_vec_from_raw, udata2.be_vals.array.to_vec());
    }
    {
	let udata2 = UData2::gen();

	// The type parameter of encastf_vec() can be omitted where
	// the Rust compiler can infer the type of the result.
	let ne_vec_from_raw: Vec<u32> =
	    udata2.raw_bytes.encastvf(NELEM2, NE).unwrap();
	let se_vec_from_raw: Vec<u32> =
	    udata2.raw_bytes.encastvf(NELEM2, SE).unwrap();
	let le_vec_from_raw: Vec<u32> =
	    udata2.raw_bytes.encastvf(NELEM2, LE).unwrap();
	let be_vec_from_raw: Vec<u32> =
	    udata2.raw_bytes.encastvf(NELEM2, BE).unwrap();

	assert_eq!(ne_vec_from_raw, udata2.ne_vals.array.to_vec());
	assert_eq!(se_vec_from_raw, udata2.se_vals.array.to_vec());
	assert_eq!(le_vec_from_raw, udata2.le_vals.array.to_vec());
	assert_eq!(be_vec_from_raw, udata2.be_vals.array.to_vec());
    }
}
