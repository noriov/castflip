use core::mem::size_of;
use std::io::Cursor;

use castflip::DecastIO;
use crate::{FData1, FVals1, IData1, IVals1, UData1, UVals1,
	    UData2, UVals2, UData3, UVals3, UData4, UVals4};


macro_rules! test {
    ( $data:expr, $ty:ty ) => {{
	{
	    let data = $data;

	    let mut ne_output = Cursor::new(vec![0_u8; size_of::<$ty>()]);
	    let mut se_output = Cursor::new(vec![0_u8; size_of::<$ty>()]);

	    ne_output.decast::<$ty>(&data.ne_vals).unwrap();
	    se_output.decast::<$ty>(&data.se_vals).unwrap();

	    assert_eq!(ne_output.into_inner(), data.raw_bytes);
	    assert_eq!(se_output.into_inner(), data.swp_bytes);
	}
	{
	    let data = $data;

	    let mut ne_output = Cursor::new(vec![0_u8; size_of::<$ty>()]);
	    let mut se_output = Cursor::new(vec![0_u8; size_of::<$ty>()]);

	    // The type parameter of decast() can be omitted where
	    // the Rust compiler can infer the type of the result.
	    ne_output.decast(&data.ne_vals).unwrap();
	    se_output.decast(&data.se_vals).unwrap();

	    assert_eq!(ne_output.into_inner(), data.raw_bytes);
	    assert_eq!(se_output.into_inner(), data.swp_bytes);
	}
    }}
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
