use castflip::{EncastMem, NE, SE, LE, BE};
use crate::{UData1, UVals1};


#[test]
fn udata1() {
    let udata1 = UData1::gen();

    let ne_vals_from_ne = UVals1 {
        val1_u8:  udata1.ne_bytes[ 0 ..  1].encastf::<u8>(NE).unwrap(),
        val2_u8:  udata1.ne_bytes[ 1 ..  2].encastf::<u8>(NE).unwrap(),
        val_u16:  udata1.ne_bytes[ 2 ..  4].encastf::<u16>(NE).unwrap(),
        val_u32:  udata1.ne_bytes[ 4 ..  8].encastf::<u32>(NE).unwrap(),
        val_u64:  udata1.ne_bytes[ 8 .. 16].encastf::<u64>(NE).unwrap(),
        val_u128: udata1.ne_bytes[16 .. 32].encastf::<u128>(NE).unwrap(),
    };

    let ne_vals_from_se = UVals1 {
        val1_u8:  udata1.se_bytes[ 0 ..  1].encastf::<u8>(SE).unwrap(),
        val2_u8:  udata1.se_bytes[ 1 ..  2].encastf::<u8>(SE).unwrap(),
        val_u16:  udata1.se_bytes[ 2 ..  4].encastf::<u16>(SE).unwrap(),
        val_u32:  udata1.se_bytes[ 4 ..  8].encastf::<u32>(SE).unwrap(),
        val_u64:  udata1.se_bytes[ 8 .. 16].encastf::<u64>(SE).unwrap(),
        val_u128: udata1.se_bytes[16 .. 32].encastf::<u128>(SE).unwrap(),
    };

    let ne_vals_from_le = UVals1 {
        val1_u8:  udata1.le_bytes[ 0 ..  1].encastf::<u8>(LE).unwrap(),
        val2_u8:  udata1.le_bytes[ 1 ..  2].encastf::<u8>(LE).unwrap(),
        val_u16:  udata1.le_bytes[ 2 ..  4].encastf::<u16>(LE).unwrap(),
        val_u32:  udata1.le_bytes[ 4 ..  8].encastf::<u32>(LE).unwrap(),
        val_u64:  udata1.le_bytes[ 8 .. 16].encastf::<u64>(LE).unwrap(),
        val_u128: udata1.le_bytes[16 .. 32].encastf::<u128>(LE).unwrap(),
    };

    let ne_vals_from_be = UVals1 {
        val1_u8:  udata1.be_bytes[ 0 ..  1].encastf::<u8>(BE).unwrap(),
        val2_u8:  udata1.be_bytes[ 1 ..  2].encastf::<u8>(BE).unwrap(),
        val_u16:  udata1.be_bytes[ 2 ..  4].encastf::<u16>(BE).unwrap(),
        val_u32:  udata1.be_bytes[ 4 ..  8].encastf::<u32>(BE).unwrap(),
        val_u64:  udata1.be_bytes[ 8 .. 16].encastf::<u64>(BE).unwrap(),
        val_u128: udata1.be_bytes[16 .. 32].encastf::<u128>(BE).unwrap(),
    };

    assert_eq!(ne_vals_from_ne, udata1.ne_vals);
    assert_eq!(ne_vals_from_se, udata1.ne_vals);
    assert_eq!(ne_vals_from_le, udata1.ne_vals);
    assert_eq!(ne_vals_from_be, udata1.ne_vals);
}
