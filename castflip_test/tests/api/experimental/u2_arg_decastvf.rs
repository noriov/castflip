use core::mem::size_of;

use castflip::experimental::DecastArg;
use castflip::{NE, SE, LE, BE};
use crate::{UData2, NELEM2};


#[test]
fn udata2() {
    let udata2 = UData2::gen();

    let ne_vec = udata2.ne_vals.array.to_vec();
    let se_vec = udata2.se_vals.array.to_vec();
    let le_vec = udata2.le_vals.array.to_vec();
    let be_vec = udata2.be_vals.array.to_vec();

    let mut raw_bytes_from_ne = [0_u8; size_of::<u32>() * NELEM2];
    let mut raw_bytes_from_se = [0_u8; size_of::<u32>() * NELEM2];
    let mut raw_bytes_from_le = [0_u8; size_of::<u32>() * NELEM2];
    let mut raw_bytes_from_be = [0_u8; size_of::<u32>() * NELEM2];

    u32::decastvf(&mut raw_bytes_from_ne, &ne_vec, NE).unwrap();
    u32::decastvf(&mut raw_bytes_from_se, &se_vec, SE).unwrap();
    u32::decastvf(&mut raw_bytes_from_le, &le_vec, LE).unwrap();
    u32::decastvf(&mut raw_bytes_from_be, &be_vec, BE).unwrap();

    assert_eq!(raw_bytes_from_ne, udata2.raw_bytes);
    assert_eq!(raw_bytes_from_se, udata2.raw_bytes);
    assert_eq!(raw_bytes_from_le, udata2.raw_bytes);
    assert_eq!(raw_bytes_from_be, udata2.raw_bytes);
}
