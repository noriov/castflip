use std::io::Cursor;

use castflip::{EncastIO, NE, SE, LE, BE};
use crate::{UData2, NELEM2};


#[test]
fn udata2() {
    let udata2 = UData2::gen();

    let mut ne_input = Cursor::new(udata2.ne_bytes.clone());
    let mut se_input = Cursor::new(udata2.se_bytes.clone());
    let mut le_input = Cursor::new(udata2.le_bytes.clone());
    let mut be_input = Cursor::new(udata2.be_bytes.clone());

    let ne_vec_from_ne = ne_input.encastvf::<u32>(NELEM2, NE).unwrap();
    let ne_vec_from_se = se_input.encastvf::<u32>(NELEM2, SE).unwrap();
    let ne_vec_from_le = le_input.encastvf::<u32>(NELEM2, LE).unwrap();
    let ne_vec_from_be = be_input.encastvf::<u32>(NELEM2, BE).unwrap();

    assert_eq!(ne_vec_from_ne, udata2.ne_vals.array.to_vec());
    assert_eq!(ne_vec_from_se, udata2.ne_vals.array.to_vec());
    assert_eq!(ne_vec_from_le, udata2.ne_vals.array.to_vec());
    assert_eq!(ne_vec_from_be, udata2.ne_vals.array.to_vec());
}
