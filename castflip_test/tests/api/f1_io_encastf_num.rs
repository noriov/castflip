use std::io::Cursor;

use castflip::{EncastIO, NE, SE, LE, BE};
use crate::{FData1, FVals1};


#[test]
fn fdata1() {
    {
	let fdata1 = FData1::gen();

	let mut raw_input1 = Cursor::new(fdata1.raw_bytes.clone());
	let mut raw_input2 = Cursor::new(fdata1.raw_bytes.clone());
	let mut raw_input3 = Cursor::new(fdata1.raw_bytes.clone());
	let mut raw_input4 = Cursor::new(fdata1.raw_bytes.clone());

	let ne_vals_from_raw = FVals1{
	    val1_f32:	raw_input1.encastf::<f32>(NE).unwrap(),
	    val2_f32:	raw_input1.encastf::<f32>(NE).unwrap(),
	    val1_f64:	raw_input1.encastf::<f64>(NE).unwrap(),
	    val2_f64:	raw_input1.encastf::<f64>(NE).unwrap(),
	};

	let se_vals_from_raw = FVals1{
	    val1_f32:	raw_input2.encastf::<f32>(SE).unwrap(),
	    val2_f32:	raw_input2.encastf::<f32>(SE).unwrap(),
	    val1_f64:	raw_input2.encastf::<f64>(SE).unwrap(),
	    val2_f64:	raw_input2.encastf::<f64>(SE).unwrap(),
	};

	let le_vals_from_raw = FVals1{
	    val1_f32:	raw_input3.encastf::<f32>(LE).unwrap(),
	    val2_f32:	raw_input3.encastf::<f32>(LE).unwrap(),
	    val1_f64:	raw_input3.encastf::<f64>(LE).unwrap(),
	    val2_f64:	raw_input3.encastf::<f64>(LE).unwrap(),
	};

	let be_vals_from_raw = FVals1{
	    val1_f32:	raw_input4.encastf::<f32>(BE).unwrap(),
	    val2_f32:	raw_input4.encastf::<f32>(BE).unwrap(),
	    val1_f64:	raw_input4.encastf::<f64>(BE).unwrap(),
	    val2_f64:	raw_input4.encastf::<f64>(BE).unwrap(),
	};

	assert_eq!(ne_vals_from_raw, fdata1.ne_vals);
	assert_eq!(se_vals_from_raw, fdata1.se_vals);
	assert_eq!(le_vals_from_raw, fdata1.le_vals);
	assert_eq!(be_vals_from_raw, fdata1.be_vals);
    }
    {
	let fdata1 = FData1::gen();

	let mut raw_input1 = Cursor::new(fdata1.raw_bytes.clone());
	let mut raw_input2 = Cursor::new(fdata1.raw_bytes.clone());
	let mut raw_input3 = Cursor::new(fdata1.raw_bytes.clone());
	let mut raw_input4 = Cursor::new(fdata1.raw_bytes.clone());

	// The type parameter of encastf() can be omitted where
	// the Rust compiler can infer the type of the result.

	let ne_vals_from_raw = FVals1{
	    val1_f32:	raw_input1.encastf(NE).unwrap(),
	    val2_f32:	raw_input1.encastf(NE).unwrap(),
	    val1_f64:	raw_input1.encastf(NE).unwrap(),
	    val2_f64:	raw_input1.encastf(NE).unwrap(),
	};

	let se_vals_from_raw = FVals1{
	    val1_f32:	raw_input2.encastf(SE).unwrap(),
	    val2_f32:	raw_input2.encastf(SE).unwrap(),
	    val1_f64:	raw_input2.encastf(SE).unwrap(),
	    val2_f64:	raw_input2.encastf(SE).unwrap(),
	};

	let le_vals_from_raw = FVals1{
	    val1_f32:	raw_input3.encastf(LE).unwrap(),
	    val2_f32:	raw_input3.encastf(LE).unwrap(),
	    val1_f64:	raw_input3.encastf(LE).unwrap(),
	    val2_f64:	raw_input3.encastf(LE).unwrap(),
	};

	let be_vals_from_raw = FVals1{
	    val1_f32:	raw_input4.encastf(BE).unwrap(),
	    val2_f32:	raw_input4.encastf(BE).unwrap(),
	    val1_f64:	raw_input4.encastf(BE).unwrap(),
	    val2_f64:	raw_input4.encastf(BE).unwrap(),
	};

	assert_eq!(ne_vals_from_raw, fdata1.ne_vals);
	assert_eq!(se_vals_from_raw, fdata1.se_vals);
	assert_eq!(le_vals_from_raw, fdata1.le_vals);
	assert_eq!(be_vals_from_raw, fdata1.be_vals);
    }
}
