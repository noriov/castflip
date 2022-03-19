use castflip::experimental::EncastArg;
use crate::{UData2, NELEM2};


#[test]
fn udata2() {
    let udata2 = UData2::gen();

    let ne_vec_from_raw = u32::encastv(&udata2.raw_bytes, NELEM2).unwrap();
    let se_vec_from_swp = u32::encastv(&udata2.swp_bytes, NELEM2).unwrap();

    assert_eq!(ne_vec_from_raw, udata2.ne_vals.array.to_vec());
    assert_eq!(se_vec_from_swp, udata2.se_vals.array.to_vec());
}
