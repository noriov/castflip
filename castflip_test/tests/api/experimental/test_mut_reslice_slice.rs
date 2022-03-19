use castflip::experimental::Reslice;
use crate::{FData1, FVals1, IData1, IVals1, UData1, UVals1,
	    UData2, UVals2, UData3, UVals3, UData4, UVals4};


macro_rules! test {
    ( $data:expr, $ty:ty ) => {{
	unsafe {
	    // Test1: &[u8] -> &[$ty]

	    let data = $data;

	    let mut raw_bytes = data.raw_bytes;
	    let mut swp_bytes = data.swp_bytes;

	    let raw_slice_u8 = &mut raw_bytes[..];
	    let swp_slice_u8 = &mut swp_bytes[..];

	    let raw_slice_vals = raw_slice_u8.reslice_mut::<$ty>().unwrap();

	    // The type parameter of reslice() can be omitted where
	    // the Rust compiler can infer the type of the result.
	    let swp_slice_vals: &mut [$ty] =
		swp_slice_u8.reslice_mut().unwrap();

	    assert_eq!(raw_slice_vals[0], data.ne_vals);
	    assert_eq!(swp_slice_vals[0], data.se_vals);
	}
	unsafe {
	    // Test2: Vec<u8> -> &[$ty]

	    let data = $data;

	    let mut raw_vec_u8 = data.raw_bytes.to_vec();
	    let mut swp_vec_u8 = data.swp_bytes.to_vec();

	    let raw_slice_vals = raw_vec_u8.reslice_mut::<$ty>().unwrap();
	    let swp_slice_vals: &mut [$ty] = swp_vec_u8.reslice_mut().unwrap();

	    assert_eq!(raw_slice_vals[0], data.ne_vals);
	    assert_eq!(swp_slice_vals[0], data.se_vals);
	}
	unsafe {
	    // Test3: [u8;N] -> &[$ty]

	    let data = $data;

	    let mut raw_array_u8 = data.raw_bytes;
	    let mut swp_array_u8 = data.swp_bytes;

	    let raw_slice_vals = raw_array_u8.reslice_mut::<$ty>().unwrap();
	    let swp_slice_vals: &mut [$ty] =
		swp_array_u8.reslice_mut().unwrap();

	    assert_eq!(raw_slice_vals[0], data.ne_vals);
	    assert_eq!(swp_slice_vals[0], data.se_vals);
	}
	unsafe {
	    // Test4: &[$ty] -> &[u8]

	    let data = $data;

	    let ne_slice_vals = &mut [data.ne_vals];
	    let se_slice_vals = &mut [data.se_vals];

	    let raw_slice_u8 = ne_slice_vals.reslice_mut::<u8>().unwrap();
	    let swp_slice_u8: &mut [u8] = se_slice_vals.reslice_mut().unwrap();

	    assert_eq!(raw_slice_u8, &data.raw_bytes[..]);
	    assert_eq!(swp_slice_u8, &data.swp_bytes[..]);
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
