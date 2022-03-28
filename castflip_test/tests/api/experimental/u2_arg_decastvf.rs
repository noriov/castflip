use core::mem::size_of;

use castflip::experimental::DecastArg;
use castflip::{NE, SE, LE, BE};
use crate::{UData2, NELEM2};


#[test]
fn udata2() {
    let udata2 = UData2::gen();

    let ne_vec = udata2.ne_vals.array.to_vec();

    let mut ne_bytes = [0_u8; size_of::<u32>() * NELEM2];
    let mut se_bytes = [0_u8; size_of::<u32>() * NELEM2];
    let mut le_bytes = [0_u8; size_of::<u32>() * NELEM2];
    let mut be_bytes = [0_u8; size_of::<u32>() * NELEM2];

    u32::decastvf(&mut ne_bytes, &ne_vec, NE).unwrap();
    u32::decastvf(&mut se_bytes, &ne_vec, SE).unwrap();
    u32::decastvf(&mut le_bytes, &ne_vec, LE).unwrap();
    u32::decastvf(&mut be_bytes, &ne_vec, BE).unwrap();

    assert_eq!(ne_bytes, udata2.ne_bytes);
    assert_eq!(se_bytes, udata2.se_bytes);
    assert_eq!(le_bytes, udata2.le_bytes);
    assert_eq!(be_bytes, udata2.be_bytes);
}
