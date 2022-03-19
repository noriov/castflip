use castflip::experimental::EncastArg;
use castflip::{NE, SE, LE, BE};
use crate::{UData2, NELEM2};


#[test]
fn udata2() {
    let udata2 = UData2::gen();

    let ne_vec_from_raw =
	u32::encastvf(&udata2.raw_bytes, NELEM2, NE).unwrap();
    let se_vec_from_raw =
	u32::encastvf(&udata2.raw_bytes, NELEM2, SE).unwrap();
    let le_vec_from_raw =
	u32::encastvf(&udata2.raw_bytes, NELEM2, LE).unwrap();
    let be_vec_from_raw =
	u32::encastvf(&udata2.raw_bytes, NELEM2, BE).unwrap();

    assert_eq!(ne_vec_from_raw, udata2.ne_vals.array.to_vec());
    assert_eq!(se_vec_from_raw, udata2.se_vals.array.to_vec());
    assert_eq!(le_vec_from_raw, udata2.le_vals.array.to_vec());
    assert_eq!(be_vec_from_raw, udata2.be_vals.array.to_vec());
}
