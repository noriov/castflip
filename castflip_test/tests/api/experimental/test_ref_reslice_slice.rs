use castflip::experimental::Reslice;
use crate::{FData1, FVals1, IData1, IVals1, UData1, UVals1,
	    UData2, UVals2, UData3, UVals3, UData4, UVals4};


macro_rules! test {
    ( $data:expr, $ty:ty ) => {{
	unsafe {
	    // Test1: &[u8] -> &[$ty]

	    let data = $data;

	    let cloned_data = data.clone();
	    let ne_slice_u8 = &cloned_data.ne_bytes[..];

	    let ne_resliced = ne_slice_u8.reslice::<$ty>().unwrap();

	    assert_eq!(ne_resliced[0], data.ne_vals);
	}
	unsafe {
	    // Test2: Vec<u8> -> &[$ty]

	    let data = $data;

	    let ne_vec_u8 = data.ne_bytes.to_vec();

	    let ne_resliced = ne_vec_u8.reslice::<$ty>().unwrap();

	    assert_eq!(ne_resliced[0], data.ne_vals);
	}
	unsafe {
	    // Test3: [u8; N] -> &[$ty]

	    let data = $data;

	    let cloned_data = data.clone();

	    let ne_resliced = cloned_data.ne_bytes.reslice::<$ty>().unwrap();

	    assert_eq!(ne_resliced[0], data.ne_vals);
	}
	unsafe {
	    // Test4: &[$ty] -> &[u8]

	    let data = $data;

	    let ne_slice_vals = &[data.ne_vals];

	    let ne_slice_u8 = ne_slice_vals.reslice::<u8>().unwrap();

	    assert_eq!(ne_slice_u8, &data.ne_bytes[..]);
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
