use core::mem::size_of;
use std::io::Cursor;

use castflip::DecastIO;
use crate::{UData2, NELEM2};


#[test]
fn udata2() {
    let udata2 = UData2::gen();

    let mut ne_output = Cursor::new(vec![0_u8; size_of::<u32>() * NELEM2]);

    let ne_vec = udata2.ne_vals.array.to_vec();

    let ne_size = ne_output.decastv::<u32>(&ne_vec).unwrap();

    assert_eq!(ne_size, size_of::<u32>() * NELEM2);
    assert_eq!(ne_output.into_inner(), udata2.ne_bytes);
}
