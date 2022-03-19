use castflip::{EncastMem, NE, SE, LE, BE};
use crate::{UData1, UVals1};


#[test]
fn udata1() {
    {
	let udata1 = UData1::gen();

	let ne_vals_from_raw = UVals1{
	    val1_u8:  udata1.raw_bytes[ 0 ..  1].encastf::<u8>(NE).unwrap(),
	    val2_u8:  udata1.raw_bytes[ 1 ..  2].encastf::<u8>(NE).unwrap(),
	    val_u16:  udata1.raw_bytes[ 2 ..  4].encastf::<u16>(NE).unwrap(),
	    val_u32:  udata1.raw_bytes[ 4 ..  8].encastf::<u32>(NE).unwrap(),
	    val_u64:  udata1.raw_bytes[ 8 .. 16].encastf::<u64>(NE).unwrap(),
	    val_u128: udata1.raw_bytes[16 .. 32].encastf::<u128>(NE).unwrap(),
	};

	let se_vals_from_raw = UVals1{
	    val1_u8:  udata1.raw_bytes[ 0 ..  1].encastf::<u8>(SE).unwrap(),
	    val2_u8:  udata1.raw_bytes[ 1 ..  2].encastf::<u8>(SE).unwrap(),
	    val_u16:  udata1.raw_bytes[ 2 ..  4].encastf::<u16>(SE).unwrap(),
	    val_u32:  udata1.raw_bytes[ 4 ..  8].encastf::<u32>(SE).unwrap(),
	    val_u64:  udata1.raw_bytes[ 8 .. 16].encastf::<u64>(SE).unwrap(),
	    val_u128: udata1.raw_bytes[16 .. 32].encastf::<u128>(SE).unwrap(),
	};

	let le_vals_from_raw = UVals1{
	    val1_u8:  udata1.raw_bytes[ 0 ..  1].encastf::<u8>(LE).unwrap(),
	    val2_u8:  udata1.raw_bytes[ 1 ..  2].encastf::<u8>(LE).unwrap(),
	    val_u16:  udata1.raw_bytes[ 2 ..  4].encastf::<u16>(LE).unwrap(),
	    val_u32:  udata1.raw_bytes[ 4 ..  8].encastf::<u32>(LE).unwrap(),
	    val_u64:  udata1.raw_bytes[ 8 .. 16].encastf::<u64>(LE).unwrap(),
	    val_u128: udata1.raw_bytes[16 .. 32].encastf::<u128>(LE).unwrap(),
	};

	let be_vals_from_raw = UVals1{
	    val1_u8:  udata1.raw_bytes[ 0 ..  1].encastf::<u8>(BE).unwrap(),
	    val2_u8:  udata1.raw_bytes[ 1 ..  2].encastf::<u8>(BE).unwrap(),
	    val_u16:  udata1.raw_bytes[ 2 ..  4].encastf::<u16>(BE).unwrap(),
	    val_u32:  udata1.raw_bytes[ 4 ..  8].encastf::<u32>(BE).unwrap(),
	    val_u64:  udata1.raw_bytes[ 8 .. 16].encastf::<u64>(BE).unwrap(),
	    val_u128: udata1.raw_bytes[16 .. 32].encastf::<u128>(BE).unwrap(),
	};

	assert_eq!(ne_vals_from_raw, udata1.ne_vals);
	assert_eq!(se_vals_from_raw, udata1.se_vals);
	assert_eq!(le_vals_from_raw, udata1.le_vals);
	assert_eq!(be_vals_from_raw, udata1.be_vals);
    }
    {
	let udata1 = UData1::gen();

	// The type parameter of encastf() can be omitted where
	// the Rust compiler can infer the type of the result.

	let ne_vals_from_raw = UVals1{
	    val1_u8:  udata1.raw_bytes[ 0 ..  1].encastf(NE).unwrap(),
	    val2_u8:  udata1.raw_bytes[ 1 ..  2].encastf(NE).unwrap(),
	    val_u16:  udata1.raw_bytes[ 2 ..  4].encastf(NE).unwrap(),
	    val_u32:  udata1.raw_bytes[ 4 ..  8].encastf(NE).unwrap(),
	    val_u64:  udata1.raw_bytes[ 8 .. 16].encastf(NE).unwrap(),
	    val_u128: udata1.raw_bytes[16 .. 32].encastf(NE).unwrap(),
	};

	let se_vals_from_raw = UVals1{
	    val1_u8:  udata1.raw_bytes[ 0 ..  1].encastf(SE).unwrap(),
	    val2_u8:  udata1.raw_bytes[ 1 ..  2].encastf(SE).unwrap(),
	    val_u16:  udata1.raw_bytes[ 2 ..  4].encastf(SE).unwrap(),
	    val_u32:  udata1.raw_bytes[ 4 ..  8].encastf(SE).unwrap(),
	    val_u64:  udata1.raw_bytes[ 8 .. 16].encastf(SE).unwrap(),
	    val_u128: udata1.raw_bytes[16 .. 32].encastf(SE).unwrap(),
	};

	let le_vals_from_raw = UVals1{
	    val1_u8:  udata1.raw_bytes[ 0 ..  1].encastf(LE).unwrap(),
	    val2_u8:  udata1.raw_bytes[ 1 ..  2].encastf(LE).unwrap(),
	    val_u16:  udata1.raw_bytes[ 2 ..  4].encastf(LE).unwrap(),
	    val_u32:  udata1.raw_bytes[ 4 ..  8].encastf(LE).unwrap(),
	    val_u64:  udata1.raw_bytes[ 8 .. 16].encastf(LE).unwrap(),
	    val_u128: udata1.raw_bytes[16 .. 32].encastf(LE).unwrap(),
	};

	let be_vals_from_raw = UVals1{
	    val1_u8:  udata1.raw_bytes[ 0 ..  1].encastf(BE).unwrap(),
	    val2_u8:  udata1.raw_bytes[ 1 ..  2].encastf(BE).unwrap(),
	    val_u16:  udata1.raw_bytes[ 2 ..  4].encastf(BE).unwrap(),
	    val_u32:  udata1.raw_bytes[ 4 ..  8].encastf(BE).unwrap(),
	    val_u64:  udata1.raw_bytes[ 8 .. 16].encastf(BE).unwrap(),
	    val_u128: udata1.raw_bytes[16 .. 32].encastf(BE).unwrap(),
	};

	assert_eq!(ne_vals_from_raw, udata1.ne_vals);
	assert_eq!(se_vals_from_raw, udata1.se_vals);
	assert_eq!(le_vals_from_raw, udata1.le_vals);
	assert_eq!(be_vals_from_raw, udata1.be_vals);
    }
}
