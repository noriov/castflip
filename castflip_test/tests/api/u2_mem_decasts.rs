use core::mem::size_of;

use castflip::DecastMem;
use crate::{UData2, NELEM2};


#[test]
fn udata2() {
    let udata2 = UData2::gen();

    let mut ne_bytes = [0_u8; size_of::<u32>() * NELEM2];

    let ne_slice = &udata2.ne_vals.array[..];

    let ne_size = ne_bytes.decasts::<u32>(ne_slice).unwrap();

    assert_eq!(ne_size, size_of::<u32>() * NELEM2);
    assert_eq!(ne_bytes, udata2.ne_bytes);
}
