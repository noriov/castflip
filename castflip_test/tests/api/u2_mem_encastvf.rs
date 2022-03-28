use castflip::{EncastMem, NE, SE, LE, BE};
use crate::{UData2, NELEM2};


#[test]
fn udata2() {
    let udata2 = UData2::gen();

    let ne_vec_from_ne = udata2.ne_bytes.encastvf::<u32>(NELEM2, NE).unwrap();
    let ne_vec_from_se = udata2.se_bytes.encastvf::<u32>(NELEM2, SE).unwrap();
    let ne_vec_from_le = udata2.le_bytes.encastvf::<u32>(NELEM2, LE).unwrap();
    let ne_vec_from_be = udata2.be_bytes.encastvf::<u32>(NELEM2, BE).unwrap();

    assert_eq!(ne_vec_from_ne, udata2.ne_vals.array.to_vec());
    assert_eq!(ne_vec_from_se, udata2.ne_vals.array.to_vec());
    assert_eq!(ne_vec_from_le, udata2.ne_vals.array.to_vec());
    assert_eq!(ne_vec_from_be, udata2.ne_vals.array.to_vec());
}
