use core::mem::size_of;

use castflip::experimental::EncastArg;
use castflip::{NE, SE, LE, BE};
use crate::{UData2, NELEM2};


#[test]
fn udata2() {
    let udata2 = UData2::gen();

    let mut ne_array_from_ne = [0_u32; NELEM2];
    let mut ne_array_from_se = [0_u32; NELEM2];
    let mut ne_array_from_le = [0_u32; NELEM2];
    let mut ne_array_from_be = [0_u32; NELEM2];

    let ne_slice_from_ne = &mut ne_array_from_ne[..];
    let ne_slice_from_se = &mut ne_array_from_se[..];
    let ne_slice_from_le = &mut ne_array_from_le[..];
    let ne_slice_from_be = &mut ne_array_from_be[..];

    let ne_size = u32::encastsf(&udata2.ne_bytes, ne_slice_from_ne, NE)
        .unwrap();
    let se_size = u32::encastsf(&udata2.se_bytes, ne_slice_from_se, SE)
        .unwrap();
    let le_size = u32::encastsf(&udata2.le_bytes, ne_slice_from_le, LE)
        .unwrap();
    let be_size = u32::encastsf(&udata2.be_bytes, ne_slice_from_be, BE)
        .unwrap();

    assert_eq!(ne_size, size_of::<u32>() * NELEM2);
    assert_eq!(se_size, size_of::<u32>() * NELEM2);
    assert_eq!(le_size, size_of::<u32>() * NELEM2);
    assert_eq!(be_size, size_of::<u32>() * NELEM2);

    assert_eq!(ne_array_from_ne, udata2.ne_vals.array);
    assert_eq!(ne_array_from_se, udata2.ne_vals.array);
    assert_eq!(ne_array_from_le, udata2.ne_vals.array);
    assert_eq!(ne_array_from_be, udata2.ne_vals.array);
}
