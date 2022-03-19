use std::io::Cursor;

use castflip::{EncastIO, NE, SE, LE, BE};
use crate::{IData1, IVals1};


#[test]
fn idata1() {
    {
	let idata1 = IData1::gen();

	let mut raw_input1 = Cursor::new(idata1.raw_bytes.clone());
	let mut raw_input2 = Cursor::new(idata1.raw_bytes.clone());
	let mut raw_input3 = Cursor::new(idata1.raw_bytes.clone());
	let mut raw_input4 = Cursor::new(idata1.raw_bytes.clone());

	let ne_vals_from_raw = IVals1{
	    val1_i8:	raw_input1.encastf::<i8>(NE).unwrap(),
	    val2_i8:	raw_input1.encastf::<i8>(NE).unwrap(),
	    val_i16:	raw_input1.encastf::<i16>(NE).unwrap(),
	    val_i32:	raw_input1.encastf::<i32>(NE).unwrap(),
	    val_i64:	raw_input1.encastf::<i64>(NE).unwrap(),
	    val_i128:	raw_input1.encastf::<i128>(NE).unwrap(),
	};

	let se_vals_from_raw = IVals1{
	    val1_i8:	raw_input2.encastf::<i8>(SE).unwrap(),
	    val2_i8:	raw_input2.encastf::<i8>(SE).unwrap(),
	    val_i16:	raw_input2.encastf::<i16>(SE).unwrap(),
	    val_i32:	raw_input2.encastf::<i32>(SE).unwrap(),
	    val_i64:	raw_input2.encastf::<i64>(SE).unwrap(),
	    val_i128:	raw_input2.encastf::<i128>(SE).unwrap(),
	};

	let le_vals_from_raw = IVals1{
	    val1_i8:	raw_input3.encastf::<i8>(LE).unwrap(),
	    val2_i8:	raw_input3.encastf::<i8>(LE).unwrap(),
	    val_i16:	raw_input3.encastf::<i16>(LE).unwrap(),
	    val_i32:	raw_input3.encastf::<i32>(LE).unwrap(),
	    val_i64:	raw_input3.encastf::<i64>(LE).unwrap(),
	    val_i128:	raw_input3.encastf::<i128>(LE).unwrap(),
	};

	let be_vals_from_raw = IVals1{
	    val1_i8:	raw_input4.encastf::<i8>(BE).unwrap(),
	    val2_i8:	raw_input4.encastf::<i8>(BE).unwrap(),
	    val_i16:	raw_input4.encastf::<i16>(BE).unwrap(),
	    val_i32:	raw_input4.encastf::<i32>(BE).unwrap(),
	    val_i64:	raw_input4.encastf::<i64>(BE).unwrap(),
	    val_i128:	raw_input4.encastf::<i128>(BE).unwrap(),
	};

	assert_eq!(ne_vals_from_raw, idata1.ne_vals);
	assert_eq!(se_vals_from_raw, idata1.se_vals);
	assert_eq!(le_vals_from_raw, idata1.le_vals);
	assert_eq!(be_vals_from_raw, idata1.be_vals);
    }
    {
	let idata1 = IData1::gen();

	let mut raw_input1 = Cursor::new(idata1.raw_bytes.clone());
	let mut raw_input2 = Cursor::new(idata1.raw_bytes.clone());
	let mut raw_input3 = Cursor::new(idata1.raw_bytes.clone());
	let mut raw_input4 = Cursor::new(idata1.raw_bytes.clone());

	// The type parameter of encastf() can be omitted where
	// the Rust compiler can infer the type of the result.

	let ne_vals_from_raw = IVals1{
	    val1_i8:	raw_input1.encastf(NE).unwrap(),
	    val2_i8:	raw_input1.encastf(NE).unwrap(),
	    val_i16:	raw_input1.encastf(NE).unwrap(),
	    val_i32:	raw_input1.encastf(NE).unwrap(),
	    val_i64:	raw_input1.encastf(NE).unwrap(),
	    val_i128:	raw_input1.encastf(NE).unwrap(),
	};

	let se_vals_from_raw = IVals1{
	    val1_i8:	raw_input2.encastf(SE).unwrap(),
	    val2_i8:	raw_input2.encastf(SE).unwrap(),
	    val_i16:	raw_input2.encastf(SE).unwrap(),
	    val_i32:	raw_input2.encastf(SE).unwrap(),
	    val_i64:	raw_input2.encastf(SE).unwrap(),
	    val_i128:	raw_input2.encastf(SE).unwrap(),
	};

	let le_vals_from_raw = IVals1{
	    val1_i8:	raw_input3.encastf(LE).unwrap(),
	    val2_i8:	raw_input3.encastf(LE).unwrap(),
	    val_i16:	raw_input3.encastf(LE).unwrap(),
	    val_i32:	raw_input3.encastf(LE).unwrap(),
	    val_i64:	raw_input3.encastf(LE).unwrap(),
	    val_i128:	raw_input3.encastf(LE).unwrap(),
	};

	let be_vals_from_raw = IVals1{
	    val1_i8:	raw_input4.encastf(BE).unwrap(),
	    val2_i8:	raw_input4.encastf(BE).unwrap(),
	    val_i16:	raw_input4.encastf(BE).unwrap(),
	    val_i32:	raw_input4.encastf(BE).unwrap(),
	    val_i64:	raw_input4.encastf(BE).unwrap(),
	    val_i128:	raw_input4.encastf(BE).unwrap(),
	};

	assert_eq!(ne_vals_from_raw, idata1.ne_vals);
	assert_eq!(se_vals_from_raw, idata1.se_vals);
	assert_eq!(le_vals_from_raw, idata1.le_vals);
	assert_eq!(be_vals_from_raw, idata1.be_vals);
    }
}
