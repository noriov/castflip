use core::mem::size_of;
use std::io::Cursor;

use castflip::{DecastIO, NE, SE, LE, BE};
use crate::{FData1, FVals1, IData1, IVals1, UData1, UVals1,
            UData2, UVals2, UData3, UVals3, UData4, UVals4};


macro_rules! test {
    ( $data:expr, $ty:ty ) => {
        {
            let data = $data;

            let mut ne_output = Cursor::new(vec![0_u8; size_of::<$ty>()]);
            let mut se_output = Cursor::new(vec![0_u8; size_of::<$ty>()]);
            let mut le_output = Cursor::new(vec![0_u8; size_of::<$ty>()]);
            let mut be_output = Cursor::new(vec![0_u8; size_of::<$ty>()]);

            ne_output.decastf::<$ty>(&data.ne_vals, NE).unwrap();
            se_output.decastf::<$ty>(&data.ne_vals, SE).unwrap();
            le_output.decastf::<$ty>(&data.ne_vals, LE).unwrap();
            be_output.decastf::<$ty>(&data.ne_vals, BE).unwrap();

            assert_eq!(ne_output.into_inner(), data.ne_bytes);
            assert_eq!(se_output.into_inner(), data.se_bytes);
            assert_eq!(le_output.into_inner(), data.le_bytes);
            assert_eq!(be_output.into_inner(), data.be_bytes);
        }
    }
}


#[test]
fn fdata1() {
    test!(FData1::gen(), FVals1);
}

#[test]
fn idata1() {
    test!(IData1::gen(), IVals1);
}

#[test]
fn udata1() {
    test!(UData1::gen(), UVals1);
}

#[test]
fn udata2() {
    test!(UData2::gen(), UVals2);
}

#[test]
fn udata3() {
    test!(UData3::gen(), UVals3);
}

#[test]
fn udata4() {
    test!(UData4::gen(), UVals4);
}
