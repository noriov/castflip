use std::io::Cursor;

use castflip::EncastIO;
use crate::{UData1, UVals1};


#[test]
fn udata1() {
    let udata1 = UData1::gen();

    let mut ne_input = Cursor::new(udata1.ne_bytes.clone());

    let ne_vals_from_ne = UVals1 {
        val1_u8:  ne_input.encast::<u8>().unwrap(),
        val2_u8:  ne_input.encast::<u8>().unwrap(),
        val_u16:  ne_input.encast::<u16>().unwrap(),
        val_u32:  ne_input.encast::<u32>().unwrap(),
        val_u64:  ne_input.encast::<u64>().unwrap(),
        val_u128: ne_input.encast::<u128>().unwrap(),
    };

    assert_eq!(ne_vals_from_ne, udata1.ne_vals);
}
