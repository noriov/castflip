use castflip::EncastMem;
use crate::{IData1, IVals1};


#[test]
fn idata1() {
    let idata1 = IData1::gen();

    let ne_vals_from_ne = IVals1 {
	val1_i8:  idata1.ne_bytes[ 0 ..  1].encast::<i8>().unwrap(),
	val2_i8:  idata1.ne_bytes[ 1 ..  2].encast::<i8>().unwrap(),
	val_i16:  idata1.ne_bytes[ 2 ..  4].encast::<i16>().unwrap(),
	val_i32:  idata1.ne_bytes[ 4 ..  8].encast::<i32>().unwrap(),
	val_i64:  idata1.ne_bytes[ 8 .. 16].encast::<i64>().unwrap(),
	val_i128: idata1.ne_bytes[16 .. 32].encast::<i128>().unwrap(),
    };

    assert_eq!(ne_vals_from_ne, idata1.ne_vals);
}
