use core::mem::size_of;

use castflip::experimental::DecastArg;
use crate::{UData2, NELEM2};


#[test]
fn udata2() {
    let udata2 = UData2::gen();

    let mut ne_bytes = [0_u8; size_of::<u32>() * NELEM2];

    let ne_slice = &udata2.ne_vals.array[..];

    let ne_size = u32::decastv(&mut ne_bytes, ne_slice).unwrap();

    assert_eq!(ne_size, size_of::<u32>() * NELEM2);
    assert_eq!(ne_bytes, udata2.ne_bytes);
}
