use castflip::experimental::EncastArg;
use crate::{UData2, NELEM2};


#[test]
fn udata2() {
    let udata2 = UData2::gen();

    let ne_vec_from_ne = u32::encastv(&udata2.ne_bytes, NELEM2).unwrap();

    assert_eq!(ne_vec_from_ne, udata2.ne_vals.array.to_vec());
}
