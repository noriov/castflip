use core::mem::size_of;

use castflip::DecastMem;
use crate::{FData1, FVals1, IData1, IVals1, UData1, UVals1,
	    UData2, UVals2, UData3, UVals3, UData4, UVals4};


macro_rules! test {
    ( $data:expr, $ty:ty ) => {{
	{
	    let data = $data;

	    let mut raw_bytes_from_ne = [0_u8; size_of::<$ty>()];
	    let mut swp_bytes_from_se = [0_u8; size_of::<$ty>()];

	    raw_bytes_from_ne.decast::<$ty>(&data.ne_vals).unwrap();
	    swp_bytes_from_se.decast::<$ty>(&data.se_vals).unwrap();

	    assert_eq!(raw_bytes_from_ne, data.raw_bytes);
	    assert_eq!(swp_bytes_from_se, data.swp_bytes);
	}
	{
	    let data = $data;

	    let mut raw_bytes_from_ne = [0_u8; size_of::<$ty>()];
	    let mut swp_bytes_from_se = [0_u8; size_of::<$ty>()];

	    // The type parameter of decast() can be omitted where
	    // the Rust compiler can infer the type of the result.
	    raw_bytes_from_ne.decast(&data.ne_vals).unwrap();
	    swp_bytes_from_se.decast(&data.se_vals).unwrap();

	    assert_eq!(raw_bytes_from_ne, data.raw_bytes);
	    assert_eq!(swp_bytes_from_se, data.swp_bytes);
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
