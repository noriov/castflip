use core::mem::size_of;

use castflip::{Cast, Flip};


#[derive(Debug)]
pub struct UData3 {
    pub raw_bytes:	[u8; size_of::<UVals3>()],
    pub swp_bytes:	[u8; size_of::<UVals3>()],
    pub ne_vals:	UVals3,
    pub se_vals:	UVals3,
    pub le_vals:	UVals3,
    pub be_vals:	UVals3,
}

#[repr(C)]
#[derive(Cast, Clone, Copy, Debug, Eq, Flip, PartialEq)]
pub struct UVals3 (
    pub u16,
    pub u16,
    pub u32,
);


impl UData3 {
    pub fn gen() -> UData3 {
	let mut raw_bytes = [0_u8; size_of::<UVals3>()];
	let mut swp_bytes = [0_u8; size_of::<UVals3>()];

	// Generate raw bytes data.
	for i in 0 .. raw_bytes.len() {
	    raw_bytes[i] = ((255 - i * 3) & 0xFF) as u8;
	}

	// Construct swapped bytes data from raw bytes data.

	// self.0
	swp_bytes[0x00] = raw_bytes[0x01];
	swp_bytes[0x01] = raw_bytes[0x00];

	// self.1
	swp_bytes[0x02] = raw_bytes[0x03];
	swp_bytes[0x03] = raw_bytes[0x02];

	// self.2
	swp_bytes[0x04] = raw_bytes[0x07];
	swp_bytes[0x05] = raw_bytes[0x06];
	swp_bytes[0x06] = raw_bytes[0x05];
	swp_bytes[0x07] = raw_bytes[0x04];

	// Construct little endian values.
	let le_vals = UVals3(
	    // self.0
	    ((raw_bytes[0x00] as  u16)) |
	    ((raw_bytes[0x01] as  u16) <<   8),
	    // self.1
	    ((raw_bytes[0x02] as  u16)) |
	    ((raw_bytes[0x03] as  u16) <<   8),
	    // self.2
	    ((raw_bytes[0x04] as  u32)) |
	    ((raw_bytes[0x05] as  u32) <<   8) |
	    ((raw_bytes[0x06] as  u32) <<  16) |
	    ((raw_bytes[0x07] as  u32) <<  24),
	);

	// Calculate big endian values.
	let be_vals = UVals3(
	    le_vals.0.swap_bytes(),
	    le_vals.1.swap_bytes(),
	    le_vals.2.swap_bytes(),
	);

	// Calculate native endian values.
	let ne_vals = UVals3(
	    u16::from_le(le_vals.0),
	    u16::from_le(le_vals.1),
	    u32::from_le(le_vals.2),
	);

	// Calculate swapped endian values.
	let se_vals = UVals3(
	    ne_vals.0.swap_bytes(),
	    ne_vals.1.swap_bytes(),
	    ne_vals.2.swap_bytes(),
	);

	return UData3{ raw_bytes, swp_bytes,
		       ne_vals, se_vals, le_vals, be_vals, };
    }
}
