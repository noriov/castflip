use castflip::{Flip, NE, SE, LE, BE};
use crate::{FData1, IData1, UData1, UData2, UData3, UData4};


macro_rules! test {
    ( $data:expr ) => {{
	{
	    let data = $data;

	    let mut se_vals_from_ne = data.ne_vals;
	    let mut ne_vals_from_se = data.se_vals;

	    se_vals_from_ne.flip_var_swapped();
	    ne_vals_from_se.flip_var_swapped();

	    assert_eq!(se_vals_from_ne, data.se_vals);
	    assert_eq!(ne_vals_from_se, data.ne_vals);
	}
	{
	    let data = $data;

	    let mut ne_vals_from_ne = data.ne_vals;
	    let mut se_vals_from_ne = data.ne_vals;
	    let mut le_vals_from_ne = data.ne_vals;
	    let mut be_vals_from_ne = data.ne_vals;

	    ne_vals_from_ne.flip_var(NE);
	    se_vals_from_ne.flip_var(SE);
	    le_vals_from_ne.flip_var(LE);
	    be_vals_from_ne.flip_var(BE);

	    assert_eq!(ne_vals_from_ne, data.ne_vals);
	    assert_eq!(se_vals_from_ne, data.se_vals);
	    assert_eq!(le_vals_from_ne, data.le_vals);
	    assert_eq!(be_vals_from_ne, data.be_vals);
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
