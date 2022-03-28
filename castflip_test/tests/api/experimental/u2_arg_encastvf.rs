use castflip::experimental::EncastArg;
use castflip::{NE, SE, LE, BE};
use crate::{UData2, NELEM2};


#[test]
fn udata2() {
    let udata2 = UData2::gen();

    let ne_vec_from_ne = u32::encastvf(&udata2.ne_bytes, NELEM2, NE).unwrap();
    let ne_vec_from_se = u32::encastvf(&udata2.se_bytes, NELEM2, SE).unwrap();
    let ne_vec_from_le = u32::encastvf(&udata2.le_bytes, NELEM2, LE).unwrap();
    let ne_vec_from_be = u32::encastvf(&udata2.be_bytes, NELEM2, BE).unwrap();

    assert_eq!(ne_vec_from_ne, udata2.ne_vals.array.to_vec());
    assert_eq!(ne_vec_from_se, udata2.ne_vals.array.to_vec());
    assert_eq!(ne_vec_from_le, udata2.ne_vals.array.to_vec());
    assert_eq!(ne_vec_from_be, udata2.ne_vals.array.to_vec());
}
