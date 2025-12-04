use std::io::Cursor;

use castflip::{EncastIO, NE, SE, LE, BE};
use crate::{UData1, UVals1};


#[test]
fn udata1() {
    let udata1 = UData1::gen();

    let mut ne_input = Cursor::new(udata1.ne_bytes.clone());
    let mut se_input = Cursor::new(udata1.se_bytes.clone());
    let mut le_input = Cursor::new(udata1.le_bytes.clone());
    let mut be_input = Cursor::new(udata1.be_bytes.clone());

    let ne_vals_from_ne = UVals1 {
        val1_u8:  ne_input.encastf::<u8>(NE).unwrap(),
        val2_u8:  ne_input.encastf::<u8>(NE).unwrap(),
        val_u16:  ne_input.encastf::<u16>(NE).unwrap(),
        val_u32:  ne_input.encastf::<u32>(NE).unwrap(),
        val_u64:  ne_input.encastf::<u64>(NE).unwrap(),
        val_u128: ne_input.encastf::<u128>(NE).unwrap(),
    };

    let ne_vals_from_se = UVals1 {
        val1_u8:  se_input.encastf::<u8>(SE).unwrap(),
        val2_u8:  se_input.encastf::<u8>(SE).unwrap(),
        val_u16:  se_input.encastf::<u16>(SE).unwrap(),
        val_u32:  se_input.encastf::<u32>(SE).unwrap(),
        val_u64:  se_input.encastf::<u64>(SE).unwrap(),
        val_u128: se_input.encastf::<u128>(SE).unwrap(),
    };

    let ne_vals_from_le = UVals1 {
        val1_u8:  le_input.encastf::<u8>(LE).unwrap(),
        val2_u8:  le_input.encastf::<u8>(LE).unwrap(),
        val_u16:  le_input.encastf::<u16>(LE).unwrap(),
        val_u32:  le_input.encastf::<u32>(LE).unwrap(),
        val_u64:  le_input.encastf::<u64>(LE).unwrap(),
        val_u128: le_input.encastf::<u128>(LE).unwrap(),
    };

    let ne_vals_from_be = UVals1 {
        val1_u8:  be_input.encastf::<u8>(BE).unwrap(),
        val2_u8:  be_input.encastf::<u8>(BE).unwrap(),
        val_u16:  be_input.encastf::<u16>(BE).unwrap(),
        val_u32:  be_input.encastf::<u32>(BE).unwrap(),
        val_u64:  be_input.encastf::<u64>(BE).unwrap(),
        val_u128: be_input.encastf::<u128>(BE).unwrap(),
    };

    assert_eq!(ne_vals_from_ne, udata1.ne_vals);
    assert_eq!(ne_vals_from_se, udata1.ne_vals);
    assert_eq!(ne_vals_from_le, udata1.ne_vals);
    assert_eq!(ne_vals_from_be, udata1.ne_vals);
}
