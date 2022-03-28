use core::mem::size_of;
use castflip::{Cast, Flip};


#[derive(Clone, Copy, Debug)]
pub struct UData1 {
    pub ne_vals:	UVals1,
    pub se_vals:	UVals1,
    pub le_vals:	UVals1,
    pub be_vals:	UVals1,
    pub ne_bytes:	[u8; size_of::<UVals1>()],
    pub se_bytes:	[u8; size_of::<UVals1>()],
    pub le_bytes:	[u8; size_of::<UVals1>()],
    pub be_bytes:	[u8; size_of::<UVals1>()],
}

#[repr(C)]
#[derive(Cast, Clone, Copy, Debug, Eq, Flip, PartialEq)]
pub struct UVals1 {
    pub val1_u8:	u8,
    pub val2_u8:	u8,
    pub val_u16:	u16,
    pub val_u32:	u32,
    pub val_u64:	u64,
    pub val_u128:	u128,
}


impl UData1 {
    pub fn gen() -> UData1 {
	let mut ne_bytes = [0_u8; size_of::<UVals1>()];
	let mut se_bytes = [0_u8; size_of::<UVals1>()];

	// Generate bytes in native-endian.
	for i in 0 .. ne_bytes.len() {
	    ne_bytes[i] = ((255 - i * 3) & 0xFF) as u8;
	}

	// Constuct bytes in swapped-endian.

	// val1_u8
	se_bytes[0x00] = ne_bytes[0x00];
	// val2_u8
	se_bytes[0x01] = ne_bytes[0x01];

	// val_u16
	se_bytes[0x02] = ne_bytes[0x03];
	se_bytes[0x03] = ne_bytes[0x02];

	// val_u32
	se_bytes[0x04] = ne_bytes[0x07];
	se_bytes[0x05] = ne_bytes[0x06];
	se_bytes[0x06] = ne_bytes[0x05];
	se_bytes[0x07] = ne_bytes[0x04];

	// val_u64
	se_bytes[0x08] = ne_bytes[0x0F];
	se_bytes[0x09] = ne_bytes[0x0E];
	se_bytes[0x0A] = ne_bytes[0x0D];
	se_bytes[0x0B] = ne_bytes[0x0C];
	se_bytes[0x0C] = ne_bytes[0x0B];
	se_bytes[0x0D] = ne_bytes[0x0A];
	se_bytes[0x0E] = ne_bytes[0x09];
	se_bytes[0x0F] = ne_bytes[0x08];

	// val_u128
	se_bytes[0x10] = ne_bytes[0x1F];
	se_bytes[0x11] = ne_bytes[0x1E];
	se_bytes[0x12] = ne_bytes[0x1D];
	se_bytes[0x13] = ne_bytes[0x1C];
	se_bytes[0x14] = ne_bytes[0x1B];
	se_bytes[0x15] = ne_bytes[0x1A];
	se_bytes[0x16] = ne_bytes[0x19];
	se_bytes[0x17] = ne_bytes[0x18];
	se_bytes[0x18] = ne_bytes[0x17];
	se_bytes[0x19] = ne_bytes[0x16];
	se_bytes[0x1A] = ne_bytes[0x15];
	se_bytes[0x1B] = ne_bytes[0x14];
	se_bytes[0x1C] = ne_bytes[0x13];
	se_bytes[0x1D] = ne_bytes[0x12];
	se_bytes[0x1E] = ne_bytes[0x11];
	se_bytes[0x1F] = ne_bytes[0x10];

	// Construct values in little-endian.
	let le_vals = UVals1 {
	    val1_u8:	   ne_bytes[0x00],
	    val2_u8:	   ne_bytes[0x01],
	    val_u16:	(((ne_bytes[0x02] as  u16)) |
			 ((ne_bytes[0x03] as  u16) <<   8)),
	    val_u32:	(((ne_bytes[0x04] as  u32)) |
			 ((ne_bytes[0x05] as  u32) <<   8) |
			 ((ne_bytes[0x06] as  u32) <<  16) |
			 ((ne_bytes[0x07] as  u32) <<  24)),
	    val_u64:	(((ne_bytes[0x08] as  u64)) |
			 ((ne_bytes[0x09] as  u64) <<   8) |
			 ((ne_bytes[0x0A] as  u64) <<  16) |
			 ((ne_bytes[0x0B] as  u64) <<  24) |
			 ((ne_bytes[0x0C] as  u64) <<  32) |
			 ((ne_bytes[0x0D] as  u64) <<  40) |
			 ((ne_bytes[0x0E] as  u64) <<  48) |
			 ((ne_bytes[0x0F] as  u64) <<  56)),
	    val_u128:	(((ne_bytes[0x10] as u128)) |
			 ((ne_bytes[0x11] as u128) <<   8) |
			 ((ne_bytes[0x12] as u128) <<  16) |
			 ((ne_bytes[0x13] as u128) <<  24) |
			 ((ne_bytes[0x14] as u128) <<  32) |
			 ((ne_bytes[0x15] as u128) <<  40) |
			 ((ne_bytes[0x16] as u128) <<  48) |
			 ((ne_bytes[0x17] as u128) <<  56) |
			 ((ne_bytes[0x18] as u128) <<  64) |
			 ((ne_bytes[0x19] as u128) <<  72) |
			 ((ne_bytes[0x1A] as u128) <<  80) |
			 ((ne_bytes[0x1B] as u128) <<  88) |
			 ((ne_bytes[0x1C] as u128) <<  96) |
			 ((ne_bytes[0x1D] as u128) << 104) |
			 ((ne_bytes[0x1E] as u128) << 112) |
			 ((ne_bytes[0x1F] as u128) << 120)),
	};

	// Calculate values in big-endian.
	let be_vals = UVals1 {
	    val1_u8:	le_vals.val1_u8,
	    val2_u8:	le_vals.val2_u8,
	    val_u16:	le_vals.val_u16.swap_bytes(),
	    val_u32:	le_vals.val_u32.swap_bytes(),
	    val_u64:	le_vals.val_u64.swap_bytes(),
	    val_u128:	le_vals.val_u128.swap_bytes(),
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

	// Construct UData1.
	return UData1 { ne_bytes, se_bytes, le_bytes, be_bytes,
			ne_vals, se_vals, le_vals, be_vals, };
    }
}
