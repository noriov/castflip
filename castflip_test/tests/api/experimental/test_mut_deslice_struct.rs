use castflip::experimental::Deslice;
use crate::{FData1, FVals1, IData1, IVals1, UData1, UVals1,
	    UData2, UVals2, UData3, UVals3, UData4, UVals4};


macro_rules! test {
    ( $data:expr, $ty:ty ) => {{
	unsafe {
	    let data = $data;

	    let mut raw_bytes = data.raw_bytes;
	    let mut swp_bytes = data.swp_bytes;

	    let raw_slice_u8 = &mut raw_bytes[..];
	    let swp_slice_u8 = &mut swp_bytes[..];

	    let raw_vals_ptr = raw_slice_u8.deslice_mut::<$ty>().unwrap();
	    let swp_vals_ptr = swp_slice_u8.deslice_mut::<$ty>().unwrap();

	    assert_eq!(*raw_vals_ptr, data.ne_vals);
	    assert_eq!(*swp_vals_ptr, data.se_vals);
	}
	unsafe {
	    let data = $data;

	    let mut raw_bytes = data.raw_bytes;
	    let mut swp_bytes = data.swp_bytes;

	    let raw_slice_u8 = &mut raw_bytes[..];
	    let swp_slice_u8 = &mut swp_bytes[..];

	    // The type parameter of deslice_mut() can be omitted where
	    // the Rust compiler can infer the type of the result.
	    let raw_vals_ptr: &mut $ty = raw_slice_u8.deslice_mut().unwrap();
	    let swp_vals_ptr: &mut $ty = swp_slice_u8.deslice_mut().unwrap();

	    assert_eq!(*raw_vals_ptr, data.ne_vals);
	    assert_eq!(*swp_vals_ptr, data.se_vals);
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
