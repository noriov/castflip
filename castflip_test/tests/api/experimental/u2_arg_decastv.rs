use core::mem::size_of;

use castflip::experimental::DecastArg;
use crate::{UData2, NELEM2};


#[test]
fn udata2() {
    let udata2 = UData2::gen();

    let ne_vec = udata2.ne_vals.array.to_vec();
    let se_vec = udata2.se_vals.array.to_vec();

    let mut raw_bytes_from_ne = [0_u8; size_of::<u32>() * NELEM2];
    let mut swp_bytes_from_se = [0_u8; size_of::<u32>() * NELEM2];

    u32::decastv(&mut raw_bytes_from_ne, &ne_vec).unwrap();
    u32::decastv(&mut swp_bytes_from_se, &se_vec).unwrap();

    assert_eq!(raw_bytes_from_ne, udata2.raw_bytes);
    assert_eq!(swp_bytes_from_se, udata2.swp_bytes);
}
