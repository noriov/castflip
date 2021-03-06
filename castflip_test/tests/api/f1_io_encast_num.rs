use std::io::Cursor;

use castflip::EncastIO;
use crate::{FData1, FVals1};


#[test]
fn fdata1() {
    let fdata1 = FData1::gen();

    let mut ne_input = Cursor::new(fdata1.ne_bytes.clone());

    let ne_vals_from_ne = FVals1 {
	val1_f32:  ne_input.encast::<f32>().unwrap(),
	val2_f32:  ne_input.encast::<f32>().unwrap(),
	val1_f64:  ne_input.encast::<f64>().unwrap(),
	val2_f64:  ne_input.encast::<f64>().unwrap(),
    };

    assert_eq!(ne_vals_from_ne, fdata1.ne_vals);
}
