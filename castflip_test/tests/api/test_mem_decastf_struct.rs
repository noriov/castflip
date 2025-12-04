use core::mem::size_of;

use castflip::{DecastMem, NE, SE, LE, BE};
use crate::{FData1, FVals1, IData1, IVals1, UData1, UVals1,
            UData2, UVals2, UData3, UVals3, UData4, UVals4};


macro_rules! test {
    ( $data:expr, $ty:ty ) => {
        {
            let data = $data;

            let mut ne_bytes = [0_u8; size_of::<$ty>()];
            let mut se_bytes = [0_u8; size_of::<$ty>()];
            let mut le_bytes = [0_u8; size_of::<$ty>()];
            let mut be_bytes = [0_u8; size_of::<$ty>()];

            ne_bytes.decastf::<$ty>(&data.ne_vals, NE).unwrap();
            se_bytes.decastf::<$ty>(&data.ne_vals, SE).unwrap();
            le_bytes.decastf::<$ty>(&data.ne_vals, LE).unwrap();
            be_bytes.decastf::<$ty>(&data.ne_vals, BE).unwrap();

            assert_eq!(ne_bytes, data.ne_bytes);
            assert_eq!(se_bytes, data.se_bytes);
            assert_eq!(le_bytes, data.le_bytes);
            assert_eq!(be_bytes, data.be_bytes);
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
