use core::mem::size_of;

use castflip::{DecastMem, NE, SE, LE, BE};
use crate::{UData2, NELEM2};


#[test]
fn udata2() {
    let udata2 = UData2::gen();

    let mut ne_bytes = [0_u8; size_of::<u32>() * NELEM2];
    let mut se_bytes = [0_u8; size_of::<u32>() * NELEM2];
    let mut le_bytes = [0_u8; size_of::<u32>() * NELEM2];
    let mut be_bytes = [0_u8; size_of::<u32>() * NELEM2];

    let ne_vec = udata2.ne_vals.array.to_vec();

    let ne_size = ne_bytes.decastvf::<u32>(&ne_vec, NE).unwrap();
    let se_size = se_bytes.decastvf::<u32>(&ne_vec, SE).unwrap();
    let le_size = le_bytes.decastvf::<u32>(&ne_vec, LE).unwrap();
    let be_size = be_bytes.decastvf::<u32>(&ne_vec, BE).unwrap();

    assert_eq!(ne_size, size_of::<u32>() * NELEM2);
    assert_eq!(se_size, size_of::<u32>() * NELEM2);
    assert_eq!(le_size, size_of::<u32>() * NELEM2);
    assert_eq!(be_size, size_of::<u32>() * NELEM2);

    assert_eq!(ne_bytes, udata2.ne_bytes);
    assert_eq!(se_bytes, udata2.se_bytes);
    assert_eq!(le_bytes, udata2.le_bytes);
    assert_eq!(be_bytes, udata2.be_bytes);
}
