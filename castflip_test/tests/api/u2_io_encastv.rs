use std::io::Cursor;

use castflip::EncastIO;
use crate::{UData2, NELEM2};


#[test]
fn udata2() {
    let udata2 = UData2::gen();

    let mut ne_input = Cursor::new(udata2.ne_bytes.clone());

    let ne_vec_from_ne = ne_input.encastv::<u32>(NELEM2).unwrap();

    assert_eq!(ne_vec_from_ne, udata2.ne_vals.array.to_vec());
}
