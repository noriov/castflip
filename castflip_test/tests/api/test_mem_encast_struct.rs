use castflip::EncastMem;
use crate::{FData1, FVals1, IData1, IVals1, UData1, UVals1,
	    UData2, UVals2, UData3, UVals3, UData4, UVals4};


macro_rules! test {
    ( $data:expr, $ty:ty ) => {{
	{
	    let data = $data;

	    let ne_vals_from_raw = data.raw_bytes.encast::<$ty>().unwrap();
	    let se_vals_from_swp = data.swp_bytes.encast::<$ty>().unwrap();

	    assert_eq!(ne_vals_from_raw, data.ne_vals);
	    assert_eq!(se_vals_from_swp, data.se_vals);
	}
	{
	    let data = $data;

	    // The type parameter of encast() can be omitted where
	    // the Rust compiler can infer the type of the result.
	    let ne_vals_from_raw: $ty = data.raw_bytes.encast().unwrap();
	    let se_vals_from_swp: $ty = data.swp_bytes.encast().unwrap();

	    assert_eq!(ne_vals_from_raw, data.ne_vals);
	    assert_eq!(se_vals_from_swp, data.se_vals);
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
