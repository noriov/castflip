use core::mem::size_of;

use castflip::EncastMem;
use crate::{UData2, NELEM2};


#[test]
fn udata2() {
    let udata2 = UData2::gen();

    let mut ne_array_from_raw = [0_u32; NELEM2];

    let ne_slice = &mut ne_array_from_raw[..];

    let ne_size = udata2.ne_bytes.encasts::<u32>(ne_slice).unwrap();

    assert_eq!(ne_size, size_of::<u32>() * NELEM2);
    assert_eq!(ne_array_from_raw, udata2.ne_vals.array);
}
