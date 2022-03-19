use castflip::{EncastMem, NE, SE, LE, BE};
use crate::{FData1, FVals1};


#[test]
fn fdata1() {
    {
	let fdata1 = FData1::gen();

	let ne_vals_from_raw = FVals1{
	    val1_f32:  fdata1.raw_bytes[ 0 ..  4].encastf::<f32>(NE).unwrap(),
	    val2_f32:  fdata1.raw_bytes[ 4 ..  8].encastf::<f32>(NE).unwrap(),
	    val1_f64:  fdata1.raw_bytes[ 8 .. 16].encastf::<f64>(NE).unwrap(),
	    val2_f64:  fdata1.raw_bytes[16 .. 24].encastf::<f64>(NE).unwrap(),
	};

	let se_vals_from_raw = FVals1{
	    val1_f32:  fdata1.raw_bytes[ 0 ..  4].encastf::<f32>(SE).unwrap(),
	    val2_f32:  fdata1.raw_bytes[ 4 ..  8].encastf::<f32>(SE).unwrap(),
	    val1_f64:  fdata1.raw_bytes[ 8 .. 16].encastf::<f64>(SE).unwrap(),
	    val2_f64:  fdata1.raw_bytes[16 .. 24].encastf::<f64>(SE).unwrap(),
	};

	let le_vals_from_raw = FVals1{
	    val1_f32:  fdata1.raw_bytes[ 0 ..  4].encastf::<f32>(LE).unwrap(),
	    val2_f32:  fdata1.raw_bytes[ 4 ..  8].encastf::<f32>(LE).unwrap(),
	    val1_f64:  fdata1.raw_bytes[ 8 .. 16].encastf::<f64>(LE).unwrap(),
	    val2_f64:  fdata1.raw_bytes[16 .. 24].encastf::<f64>(LE).unwrap(),
	};

	let be_vals_from_raw = FVals1{
	    val1_f32:  fdata1.raw_bytes[ 0 ..  4].encastf::<f32>(BE).unwrap(),
	    val2_f32:  fdata1.raw_bytes[ 4 ..  8].encastf::<f32>(BE).unwrap(),
	    val1_f64:  fdata1.raw_bytes[ 8 .. 16].encastf::<f64>(BE).unwrap(),
	    val2_f64:  fdata1.raw_bytes[16 .. 24].encastf::<f64>(BE).unwrap(),
	};

	assert_eq!(ne_vals_from_raw, fdata1.ne_vals);
	assert_eq!(se_vals_from_raw, fdata1.se_vals);
	assert_eq!(le_vals_from_raw, fdata1.le_vals);
	assert_eq!(be_vals_from_raw, fdata1.be_vals);
    }
    {
	let fdata1 = FData1::gen();

	// The type parameter of encastf() can be omitted where
	// the Rust compiler can infer the type of the result.

	let ne_vals_from_raw = FVals1{
	    val1_f32:  fdata1.raw_bytes[ 0 ..  4].encastf(NE).unwrap(),
	    val2_f32:  fdata1.raw_bytes[ 4 ..  8].encastf(NE).unwrap(),
	    val1_f64:  fdata1.raw_bytes[ 8 .. 16].encastf(NE).unwrap(),
	    val2_f64:  fdata1.raw_bytes[16 .. 24].encastf(NE).unwrap(),
	};

	let se_vals_from_raw = FVals1{
	    val1_f32:  fdata1.raw_bytes[ 0 ..  4].encastf(SE).unwrap(),
	    val2_f32:  fdata1.raw_bytes[ 4 ..  8].encastf(SE).unwrap(),
	    val1_f64:  fdata1.raw_bytes[ 8 .. 16].encastf(SE).unwrap(),
	    val2_f64:  fdata1.raw_bytes[16 .. 24].encastf(SE).unwrap(),
	};

	let le_vals_from_raw = FVals1{
	    val1_f32:  fdata1.raw_bytes[ 0 ..  4].encastf(LE).unwrap(),
	    val2_f32:  fdata1.raw_bytes[ 4 ..  8].encastf(LE).unwrap(),
	    val1_f64:  fdata1.raw_bytes[ 8 .. 16].encastf(LE).unwrap(),
	    val2_f64:  fdata1.raw_bytes[16 .. 24].encastf(LE).unwrap(),
	};

	let be_vals_from_raw = FVals1{
	    val1_f32:  fdata1.raw_bytes[ 0 ..  4].encastf(BE).unwrap(),
	    val2_f32:  fdata1.raw_bytes[ 4 ..  8].encastf(BE).unwrap(),
	    val1_f64:  fdata1.raw_bytes[ 8 .. 16].encastf(BE).unwrap(),
	    val2_f64:  fdata1.raw_bytes[16 .. 24].encastf(BE).unwrap(),
	};

	assert_eq!(ne_vals_from_raw, fdata1.ne_vals);
	assert_eq!(se_vals_from_raw, fdata1.se_vals);
	assert_eq!(le_vals_from_raw, fdata1.le_vals);
	assert_eq!(be_vals_from_raw, fdata1.be_vals);
    }
}
