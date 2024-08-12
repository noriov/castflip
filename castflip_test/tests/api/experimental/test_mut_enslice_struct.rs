use castflip::experimental::Enslice;
use crate::{FData1, IData1, UData1, UData2, UData3, UData4};


macro_rules! test {
    ( $data:expr ) => {{
	unsafe {
	    // Test: &mut $ty -> &mut [u8]

	    let data = $data;

	    let mut ne_vals = data.ne_vals;

	    let ne_ensliced = ne_vals.enslice_mut::<u8>().unwrap();

	    assert_eq!(*ne_ensliced, data.ne_bytes);
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
