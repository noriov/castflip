use core::mem::size_of;

use castflip::{Cast, Flip};


#[derive(Debug)]
pub struct UData2 {
    pub raw_bytes:	[u8; size_of::<UVals2>()],
    pub swp_bytes:	[u8; size_of::<UVals2>()],
    pub ne_vals:	UVals2,
    pub se_vals:	UVals2,
    pub le_vals:	UVals2,
    pub be_vals:	UVals2,
}

#[repr(C)]
#[derive(Cast, Clone, Copy, Debug, Eq, Flip, PartialEq)]
pub struct UVals2 {
    pub array:	[u32; NELEM2],
}

pub const NELEM2: usize = 4;


impl UData2 {
    pub fn gen() -> UData2 {
	let mut raw_bytes = [0_u8; size_of::<UVals2>()];
	let mut swp_bytes = [0_u8; size_of::<UVals2>()];

	// Generate raw bytes data.
	for i in 0 .. raw_bytes.len() {
	    raw_bytes[i] = ((255 - i * 3) & 0xFF) as u8;
	}

	// Construct swapped bytes data from raw bytes data.

	// array[0]
	swp_bytes[0x00] = raw_bytes[0x03];
	swp_bytes[0x01] = raw_bytes[0x02];
	swp_bytes[0x02] = raw_bytes[0x01];
	swp_bytes[0x03] = raw_bytes[0x00];

	// array[1]
	swp_bytes[0x04] = raw_bytes[0x07];
	swp_bytes[0x05] = raw_bytes[0x06];
	swp_bytes[0x06] = raw_bytes[0x05];
	swp_bytes[0x07] = raw_bytes[0x04];

	// array[2]
	swp_bytes[0x08] = raw_bytes[0x0B];
	swp_bytes[0x09] = raw_bytes[0x0A];
	swp_bytes[0x0A] = raw_bytes[0x09];
	swp_bytes[0x0B] = raw_bytes[0x08];

	// array[3]
	swp_bytes[0x0C] = raw_bytes[0x0F];
	swp_bytes[0x0D] = raw_bytes[0x0E];
	swp_bytes[0x0E] = raw_bytes[0x0D];
	swp_bytes[0x0F] = raw_bytes[0x0C];

	#[allow(unused_parens)]
	let val1 = (((raw_bytes[0x00] as  u32)) |
		    ((raw_bytes[0x01] as  u32) <<   8) |
		    ((raw_bytes[0x02] as  u32) <<  16) |
		    ((raw_bytes[0x03] as  u32) <<  24));
	#[allow(unused_parens)]
	let val2 = (((raw_bytes[0x04] as  u32)) |
		    ((raw_bytes[0x05] as  u32) <<   8) |
		    ((raw_bytes[0x06] as  u32) <<  16) |
		    ((raw_bytes[0x07] as  u32) <<  24));
	#[allow(unused_parens)]
	let val3 = (((raw_bytes[0x08] as  u32)) |
		    ((raw_bytes[0x09] as  u32) <<   8) |
		    ((raw_bytes[0x0A] as  u32) <<  16) |
		    ((raw_bytes[0x0B] as  u32) <<  24));
	#[allow(unused_parens)]
	let val4 = (((raw_bytes[0x0C] as  u32)) |
		    ((raw_bytes[0x0D] as  u32) <<   8) |
		    ((raw_bytes[0x0E] as  u32) <<  16) |
		    ((raw_bytes[0x0F] as  u32) <<  24));

	// Construct little endian values.
	let le_vals = UVals2{
	    array:	[ val1, val2, val3, val4 ],
	};

	// Calculate big endian values.
	let be_vals = UVals2{
	    array:	[ val1.swap_bytes(),
			  val2.swap_bytes(),
			  val3.swap_bytes(),
			  val4.swap_bytes() ],
	};

	// Calculate native endian values.
	let ne_vals = UVals2{
	    array:	[ u32::from_le(val1),
			  u32::from_le(val2),
			  u32::from_le(val3),
			  u32::from_le(val4) ],
	};

	// Calculate swapped endian values.
	let se_vals = UVals2{
	    array:	[ ne_vals.array[0].swap_bytes(),
			  ne_vals.array[1].swap_bytes(),
			  ne_vals.array[2].swap_bytes(),
			  ne_vals.array[3].swap_bytes(), ],
	};

	return UData2{ raw_bytes, swp_bytes,
		       ne_vals, se_vals, le_vals, be_vals, };
    }
}
