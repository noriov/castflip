use std::io::Cursor;

use castflip::EncastIO;
use crate::{IData1, IVals1};


#[test]
fn idata1() {
    let idata1 = IData1::gen();

    let mut ne_input = Cursor::new(idata1.ne_bytes.clone());

    let ne_vals_from_ne = IVals1 {
        val1_i8:  ne_input.encast::<i8>().unwrap(),
        val2_i8:  ne_input.encast::<i8>().unwrap(),
        val_i16:  ne_input.encast::<i16>().unwrap(),
        val_i32:  ne_input.encast::<i32>().unwrap(),
        val_i64:  ne_input.encast::<i64>().unwrap(),
        val_i128: ne_input.encast::<i128>().unwrap(),
    };

    assert_eq!(ne_vals_from_ne, idata1.ne_vals);
}
