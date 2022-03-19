use castflip::experimental::Enslice;
use crate::{FData1, IData1, UData1, UData2, UData3, UData4};


macro_rules! test {
    ( $data:expr ) => {{
	unsafe {
	    let data = $data;

	    let ne_vals = data.ne_vals;
	    let se_vals = data.se_vals;

	    let ne_bytes_ptr = ne_vals.enslice::<u8>().unwrap();
	    let se_bytes_ptr = se_vals.enslice::<u8>().unwrap();

	    assert_eq!(*ne_bytes_ptr, data.raw_bytes);
	    assert_eq!(*se_bytes_ptr, data.swp_bytes);
	}
	unsafe {
	    let data = $data;

	    let ne_vals = data.ne_vals;
	    let se_vals = data.se_vals;

	    // The type parameter of enslice() can be omitted where
	    // the Rust compiler can infer the type of the result.
	    let ne_bytes_ptr: &[u8] = ne_vals.enslice().unwrap();
	    let se_bytes_ptr: &[u8] = se_vals.enslice().unwrap();

	    assert_eq!(*ne_bytes_ptr, data.raw_bytes);
	    assert_eq!(*se_bytes_ptr, data.swp_bytes);
	}
    }}
}


#[test]
fn fdata1() {
    test!(FData1::gen());
}

#[test]
fn idata1() {
    test!(IData1::gen());
}

#[test]
fn udata1() {
    test!(UData1::gen());
}

#[test]
fn udata2() {
    test!(UData2::gen());
}

#[test]
fn udata3() {
    test!(UData3::gen());
}

#[test]
fn udata4() {
    test!(UData4::gen());
}
