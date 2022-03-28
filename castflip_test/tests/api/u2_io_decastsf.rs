use core::mem::size_of;
use std::io::Cursor;

use castflip::{DecastIO, NE, SE, LE, BE};
use crate::{UData2, NELEM2};


#[test]
fn udata2() {
    let udata2 = UData2::gen();

    let mut ne_output = Cursor::new(vec![0_u8; size_of::<u32>() * NELEM2]);
    let mut se_output = Cursor::new(vec![0_u8; size_of::<u32>() * NELEM2]);
    let mut le_output = Cursor::new(vec![0_u8; size_of::<u32>() * NELEM2]);
    let mut be_output = Cursor::new(vec![0_u8; size_of::<u32>() * NELEM2]);

    let ne_slice = &udata2.ne_vals.array[..];

    let ne_size = ne_output.decastsf::<u32>(ne_slice, NE).unwrap();
    let se_size = se_output.decastsf::<u32>(ne_slice, SE).unwrap();
    let le_size = le_output.decastsf::<u32>(ne_slice, LE).unwrap();
    let be_size = be_output.decastsf::<u32>(ne_slice, BE).unwrap();

    assert_eq!(ne_size, size_of::<u32>() * NELEM2);
    assert_eq!(se_size, size_of::<u32>() * NELEM2);
    assert_eq!(le_size, size_of::<u32>() * NELEM2);
    assert_eq!(be_size, size_of::<u32>() * NELEM2);

    assert_eq!(ne_output.into_inner(), udata2.ne_bytes);
    assert_eq!(se_output.into_inner(), udata2.se_bytes);
    assert_eq!(le_output.into_inner(), udata2.le_bytes);
    assert_eq!(be_output.into_inner(), udata2.be_bytes);
}
