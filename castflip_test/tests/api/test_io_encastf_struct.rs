use std::io::Cursor;

use castflip::{EncastIO, NE, SE, LE, BE};
use crate::{FData1, FVals1, IData1, IVals1, UData1, UVals1,
	    UData2, UVals2, UData3, UVals3, UData4, UVals4};


macro_rules! test {
    ( $data:expr, $ty:ty ) => {
	{
	    let data = $data;

	    let mut ne_input = Cursor::new(data.ne_bytes.clone());
	    let mut se_input = Cursor::new(data.se_bytes.clone());
	    let mut le_input = Cursor::new(data.le_bytes.clone());
	    let mut be_input = Cursor::new(data.be_bytes.clone());

	    let ne_vals_from_ne = ne_input.encastf::<$ty>(NE).unwrap();
	    let ne_vals_from_se = se_input.encastf::<$ty>(SE).unwrap();
	    let ne_vals_from_le = le_input.encastf::<$ty>(LE).unwrap();
	    let ne_vals_from_be = be_input.encastf::<$ty>(BE).unwrap();

	    assert_eq!(ne_vals_from_ne, data.ne_vals);
	    assert_eq!(ne_vals_from_se, data.ne_vals);
	    assert_eq!(ne_vals_from_le, data.ne_vals);
	    assert_eq!(ne_vals_from_be, data.ne_vals);
	}
    }
}


#[test]
fn fdata1() {
    test!(FData1::gen(), FVals1);
}

#[test]
fn idata1() {
    test!(IData1::gen(), IVals1);
}

#[test]
fn udata1() {
    test!(UData1::gen(), UVals1);
}

#[test]
fn udata2() {
    test!(UData2::gen(), UVals2);
}

#[test]
fn udata3() {
    test!(UData3::gen(), UVals3);
}

#[test]
fn udata4() {
    test!(UData4::gen(), UVals4);
}
