use castflip::{Flip, NE, SE, LE, BE};
use crate::{FData1, IData1, UData1, UData2, UData3, UData4};


macro_rules! test {
    ( $data:expr ) => {{
	{
	    let data = $data;

	    let ne_vals = data.ne_vals;
	    let se_vals = data.se_vals;

	    let se_vals_from_ne = ne_vals.flip_val_swapped();
	    let ne_vals_from_se = se_vals.flip_val_swapped();

	    assert_eq!(se_vals_from_ne, data.se_vals);
	    assert_eq!(ne_vals_from_se, data.ne_vals);
	}
	{
	    let data = $data;

	    let ne_vals = data.ne_vals;

	    let ne_vals_from_ne = ne_vals.flip_val(NE);
	    let se_vals_from_ne = ne_vals.flip_val(SE);
	    let le_vals_from_ne = ne_vals.flip_val(LE);
	    let be_vals_from_ne = ne_vals.flip_val(BE);

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
