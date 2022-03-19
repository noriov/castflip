use std::io::Cursor;

use castflip::{EncastIO, NE, SE, LE, BE};
use crate::{UData2, NELEM2};


#[test]
fn udata2() {
    {
	let udata2 = UData2::gen();

	let mut raw_input1 = Cursor::new(udata2.raw_bytes.clone());
	let mut raw_input2 = Cursor::new(udata2.raw_bytes.clone());
	let mut raw_input3 = Cursor::new(udata2.raw_bytes.clone());
	let mut raw_input4 = Cursor::new(udata2.raw_bytes.clone());

	let ne_vec = raw_input1.encastvf::<u32>(NELEM2, NE).unwrap();
	let se_vec = raw_input2.encastvf::<u32>(NELEM2, SE).unwrap();
	let le_vec = raw_input3.encastvf::<u32>(NELEM2, LE).unwrap();
	let be_vec = raw_input4.encastvf::<u32>(NELEM2, BE).unwrap();

	assert_eq!(ne_vec, udata2.ne_vals.array.to_vec());
	assert_eq!(se_vec, udata2.se_vals.array.to_vec());
	assert_eq!(le_vec, udata2.le_vals.array.to_vec());
	assert_eq!(be_vec, udata2.be_vals.array.to_vec());
    }
    {
	let udata2 = UData2::gen();

	let mut raw_input1 = Cursor::new(udata2.raw_bytes.clone());
	let mut raw_input2 = Cursor::new(udata2.raw_bytes.clone());
	let mut raw_input3 = Cursor::new(udata2.raw_bytes.clone());
	let mut raw_input4 = Cursor::new(udata2.raw_bytes.clone());

	// The type parameter of encastvf() can be omitted where
	// the Rust compiler can infer the type of the result.
	let ne_vec: Vec<u32> = raw_input1.encastvf(NELEM2, NE).unwrap();
	let se_vec: Vec<u32> = raw_input2.encastvf(NELEM2, SE).unwrap();
	let le_vec: Vec<u32> = raw_input3.encastvf(NELEM2, LE).unwrap();
	let be_vec: Vec<u32> = raw_input4.encastvf(NELEM2, BE).unwrap();

	assert_eq!(ne_vec, udata2.ne_vals.array.to_vec());
	assert_eq!(se_vec, udata2.se_vals.array.to_vec());
	assert_eq!(le_vec, udata2.le_vals.array.to_vec());
	assert_eq!(be_vec, udata2.be_vals.array.to_vec());
    }
}
