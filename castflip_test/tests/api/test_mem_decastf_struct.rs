use core::mem::size_of;

use castflip::{DecastMem, NE, SE, LE, BE};
use crate::{FData1, FVals1, IData1, IVals1, UData1, UVals1,
	    UData2, UVals2, UData3, UVals3, UData4, UVals4};


macro_rules! test {
    ( $data:expr, $ty:ty ) => {{
	{
	    let data = $data;

	    let mut raw_bytes_from_ne = [0_u8; size_of::<$ty>()];
	    let mut raw_bytes_from_se = [0_u8; size_of::<$ty>()];
	    let mut raw_bytes_from_le = [0_u8; size_of::<$ty>()];
	    let mut raw_bytes_from_be = [0_u8; size_of::<$ty>()];

	    raw_bytes_from_ne.decastf::<$ty>(&data.ne_vals, NE).unwrap();
	    raw_bytes_from_se.decastf::<$ty>(&data.se_vals, SE).unwrap();
	    raw_bytes_from_le.decastf::<$ty>(&data.le_vals, LE).unwrap();
	    raw_bytes_from_be.decastf::<$ty>(&data.be_vals, BE).unwrap();

	    assert_eq!(raw_bytes_from_ne, data.raw_bytes);
	    assert_eq!(raw_bytes_from_se, data.raw_bytes);
	    assert_eq!(raw_bytes_from_le, data.raw_bytes);
	    assert_eq!(raw_bytes_from_be, data.raw_bytes);
	}
	{
	    let data = $data;

	    let mut raw_bytes_from_ne = [0_u8; size_of::<$ty>()];
	    let mut raw_bytes_from_se = [0_u8; size_of::<$ty>()];
	    let mut raw_bytes_from_le = [0_u8; size_of::<$ty>()];
	    let mut raw_bytes_from_be = [0_u8; size_of::<$ty>()];

	    // The type parameter of decastf() can be omitted where
	    // the Rust compiler can infer the type of the result.
	    raw_bytes_from_ne.decastf(&data.ne_vals, NE).unwrap();
	    raw_bytes_from_se.decastf(&data.se_vals, SE).unwrap();
	    raw_bytes_from_le.decastf(&data.le_vals, LE).unwrap();
	    raw_bytes_from_be.decastf(&data.be_vals, BE).unwrap();

	    assert_eq!(raw_bytes_from_ne, data.raw_bytes);
	    assert_eq!(raw_bytes_from_se, data.raw_bytes);
	    assert_eq!(raw_bytes_from_le, data.raw_bytes);
	    assert_eq!(raw_bytes_from_be, data.raw_bytes);
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
