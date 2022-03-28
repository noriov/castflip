use castflip::{EncastMem, NE, SE, LE, BE};
use crate::{IData1, IVals1};


#[test]
fn idata1() {
    let idata1 = IData1::gen();

    let ne_vals_from_ne = IVals1 {
	val1_i8:  idata1.ne_bytes[ 0 ..  1].encastf::<i8>(NE).unwrap(),
	val2_i8:  idata1.ne_bytes[ 1 ..  2].encastf::<i8>(NE).unwrap(),
	val_i16:  idata1.ne_bytes[ 2 ..  4].encastf::<i16>(NE).unwrap(),
	val_i32:  idata1.ne_bytes[ 4 ..  8].encastf::<i32>(NE).unwrap(),
	val_i64:  idata1.ne_bytes[ 8 .. 16].encastf::<i64>(NE).unwrap(),
	val_i128: idata1.ne_bytes[16 .. 32].encastf::<i128>(NE).unwrap(),
    };

    let ne_vals_from_se = IVals1 {
	val1_i8:  idata1.se_bytes[ 0 ..  1].encastf::<i8>(SE).unwrap(),
	val2_i8:  idata1.se_bytes[ 1 ..  2].encastf::<i8>(SE).unwrap(),
	val_i16:  idata1.se_bytes[ 2 ..  4].encastf::<i16>(SE).unwrap(),
	val_i32:  idata1.se_bytes[ 4 ..  8].encastf::<i32>(SE).unwrap(),
	val_i64:  idata1.se_bytes[ 8 .. 16].encastf::<i64>(SE).unwrap(),
	val_i128: idata1.se_bytes[16 .. 32].encastf::<i128>(SE).unwrap(),
    };

    let ne_vals_from_le = IVals1 {
	val1_i8:  idata1.le_bytes[ 0 ..  1].encastf::<i8>(LE).unwrap(),
	val2_i8:  idata1.le_bytes[ 1 ..  2].encastf::<i8>(LE).unwrap(),
	val_i16:  idata1.le_bytes[ 2 ..  4].encastf::<i16>(LE).unwrap(),
	val_i32:  idata1.le_bytes[ 4 ..  8].encastf::<i32>(LE).unwrap(),
	val_i64:  idata1.le_bytes[ 8 .. 16].encastf::<i64>(LE).unwrap(),
	val_i128: idata1.le_bytes[16 .. 32].encastf::<i128>(LE).unwrap(),
    };

    let ne_vals_from_be = IVals1 {
	val1_i8:  idata1.be_bytes[ 0 ..  1].encastf::<i8>(BE).unwrap(),
	val2_i8:  idata1.be_bytes[ 1 ..  2].encastf::<i8>(BE).unwrap(),
	val_i16:  idata1.be_bytes[ 2 ..  4].encastf::<i16>(BE).unwrap(),
	val_i32:  idata1.be_bytes[ 4 ..  8].encastf::<i32>(BE).unwrap(),
	val_i64:  idata1.be_bytes[ 8 .. 16].encastf::<i64>(BE).unwrap(),
	val_i128: idata1.be_bytes[16 .. 32].encastf::<i128>(BE).unwrap(),
    };

    assert_eq!(ne_vals_from_ne, idata1.ne_vals);
    assert_eq!(ne_vals_from_se, idata1.ne_vals);
    assert_eq!(ne_vals_from_le, idata1.ne_vals);
    assert_eq!(ne_vals_from_be, idata1.ne_vals);
}
