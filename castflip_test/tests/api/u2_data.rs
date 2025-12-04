use core::mem::size_of;

use castflip::{Cast, Flip};


#[repr(C)]
#[derive(Clone, Debug)]
pub struct UData2 {
    pub ne_vals:        UVals2,
    pub se_vals:        UVals2,
    pub le_vals:        UVals2,
    pub be_vals:        UVals2,
    pub ne_bytes:       [u8; size_of::<UVals2>()],
    pub se_bytes:       [u8; size_of::<UVals2>()],
    pub le_bytes:       [u8; size_of::<UVals2>()],
    pub be_bytes:       [u8; size_of::<UVals2>()],
}

#[repr(C)]
#[derive(Cast, Clone, Copy, Debug, Eq, Flip, PartialEq)]
pub struct UVals2 {
    pub array:  [u32; NELEM2],
}

pub const NELEM2: usize = 4;

const _: () = assert!(size_of::<UVals2>() == 0x10);


impl UData2 {
    pub fn gen() -> UData2 {
        let mut ne_bytes = [0_u8; size_of::<UVals2>()];
        let mut se_bytes = [0_u8; size_of::<UVals2>()];

        // Generate bytes in native-endian.
        for i in 0 .. ne_bytes.len() {
            ne_bytes[i] = ((255 - i * 3) & 0xFF) as u8;
        }

        // Constuct bytes in swapped-endian.

        // array[0]
        se_bytes[0x00] = ne_bytes[0x03];
        se_bytes[0x01] = ne_bytes[0x02];
        se_bytes[0x02] = ne_bytes[0x01];
        se_bytes[0x03] = ne_bytes[0x00];

        // array[1]
        se_bytes[0x04] = ne_bytes[0x07];
        se_bytes[0x05] = ne_bytes[0x06];
        se_bytes[0x06] = ne_bytes[0x05];
        se_bytes[0x07] = ne_bytes[0x04];

        // array[2]
        se_bytes[0x08] = ne_bytes[0x0B];
        se_bytes[0x09] = ne_bytes[0x0A];
        se_bytes[0x0A] = ne_bytes[0x09];
        se_bytes[0x0B] = ne_bytes[0x08];

        // array[3]
        se_bytes[0x0C] = ne_bytes[0x0F];
        se_bytes[0x0D] = ne_bytes[0x0E];
        se_bytes[0x0E] = ne_bytes[0x0D];
        se_bytes[0x0F] = ne_bytes[0x0C];

        // Construct values in little-endian.
        let le_vals = UVals2 {
            array:      [ (((ne_bytes[0x00] as u32)) |
                           ((ne_bytes[0x01] as u32) <<   8) |
                           ((ne_bytes[0x02] as u32) <<  16) |
                           ((ne_bytes[0x03] as u32) <<  24)),

                          (((ne_bytes[0x04] as u32)) |
                           ((ne_bytes[0x05] as u32) <<   8) |
                           ((ne_bytes[0x06] as u32) <<  16) |
                           ((ne_bytes[0x07] as u32) <<  24)),

                          (((ne_bytes[0x08] as u32)) |
                           ((ne_bytes[0x09] as u32) <<   8) |
                           ((ne_bytes[0x0A] as u32) <<  16) |
                           ((ne_bytes[0x0B] as u32) <<  24)),

                          (((ne_bytes[0x0C] as u32)) |
                           ((ne_bytes[0x0D] as u32) <<   8) |
                           ((ne_bytes[0x0E] as u32) <<  16) |
                           ((ne_bytes[0x0F] as u32) <<  24)) ],
        };

        // Calculate values in big-endian.
        let be_vals = UVals2 {
            array:      [ le_vals.array[0].swap_bytes(),
                          le_vals.array[1].swap_bytes(),
                          le_vals.array[2].swap_bytes(),
                          le_vals.array[3].swap_bytes() ],
        };

        // Prepare LE and BE data.
        let (le_bytes, be_bytes, ne_vals, se_vals);
        if cfg!(target_endian = "little") {
            le_bytes = ne_bytes;
            be_bytes = se_bytes;
            ne_vals = le_vals;
            se_vals = be_vals;
        } else if cfg!(target_endian = "big") {
            le_bytes = se_bytes;
            be_bytes = ne_bytes;
            ne_vals = be_vals;
            se_vals = le_vals;
        } else {
            panic!();
        }

        // Construct UData2.
        return UData2 { ne_vals, se_vals, le_vals, be_vals,
                        ne_bytes, se_bytes, le_bytes, be_bytes, };
    }
}
