use core::mem::size_of;
use std::io::Cursor;

use castflip::EncastIO;
use crate::{UData2, NELEM2};


#[test]
fn udata2() {
    let udata2 = UData2::gen();

    let mut ne_input = Cursor::new(udata2.ne_bytes.clone());

    let mut ne_array_from_ne = [0_u32; NELEM2];

    let ne_slice = &mut ne_array_from_ne[..];

    let ne_size = ne_input.encasts::<u32>(ne_slice).unwrap();

    assert_eq!(ne_size, size_of::<u32>() * NELEM2);
    assert_eq!(ne_array_from_ne, udata2.ne_vals.array);
}
