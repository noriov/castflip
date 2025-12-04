use core::mem::size_of;

use castflip::{Cast, Flip};


#[repr(C)]
#[derive(Clone, Debug)]
pub struct UData3 {
    pub ne_vals:        UVals3,
    pub se_vals:        UVals3,
    pub le_vals:        UVals3,
    pub be_vals:        UVals3,
    pub ne_bytes:       [u8; size_of::<UVals3>()],
    pub se_bytes:       [u8; size_of::<UVals3>()],
    pub le_bytes:       [u8; size_of::<UVals3>()],
    pub be_bytes:       [u8; size_of::<UVals3>()],
}

#[repr(C)]
#[derive(Cast, Clone, Copy, Debug, Eq, Flip, PartialEq)]
pub struct UVals3 (
    pub u16,
    pub u16,
    pub u32,
);

const _: () = assert!(size_of::<UVals3>() == 0x08);


impl UData3 {
    pub fn gen() -> UData3 {
        let mut ne_bytes = [0_u8; size_of::<UVals3>()];
        let mut se_bytes = [0_u8; size_of::<UVals3>()];

        // Generate bytes in native-endian.
        for i in 0 .. ne_bytes.len() {
            ne_bytes[i] = ((255 - i * 3) & 0xFF) as u8;
        }

        // Constuct bytes in swapped-endian.

        // self.0
        se_bytes[0x00] = ne_bytes[0x01];
        se_bytes[0x01] = ne_bytes[0x00];

        // self.1
        se_bytes[0x02] = ne_bytes[0x03];
        se_bytes[0x03] = ne_bytes[0x02];

        // self.2
        se_bytes[0x04] = ne_bytes[0x07];
        se_bytes[0x05] = ne_bytes[0x06];
        se_bytes[0x06] = ne_bytes[0x05];
        se_bytes[0x07] = ne_bytes[0x04];

        // Construct values in little-endian.
        let le_vals = UVals3 (
            // self.0
            ((ne_bytes[0x00] as  u16)) |
            ((ne_bytes[0x01] as  u16) <<   8),
            // self.1
            ((ne_bytes[0x02] as  u16)) |
            ((ne_bytes[0x03] as  u16) <<   8),
            // self.2
            ((ne_bytes[0x04] as  u32)) |
            ((ne_bytes[0x05] as  u32) <<   8) |
            ((ne_bytes[0x06] as  u32) <<  16) |
            ((ne_bytes[0x07] as  u32) <<  24),
        );

        // Calculate values in big-endian.
        let be_vals = UVals3 (
            le_vals.0.swap_bytes(),
            le_vals.1.swap_bytes(),
            le_vals.2.swap_bytes(),
        );

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

        // Construct UData3.
        return UData3 { ne_vals, se_vals, le_vals, be_vals,
                        ne_bytes, se_bytes, le_bytes, be_bytes, };
    }
}
