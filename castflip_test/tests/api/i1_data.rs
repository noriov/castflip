use core::mem::size_of;
use castflip::{Cast, Flip};


#[derive(Clone, Copy, Debug)]
pub struct IData1 {
    pub raw_bytes:	[u8; size_of::<IVals1>()],
    pub swp_bytes:	[u8; size_of::<IVals1>()],
    pub ne_vals:	IVals1,
    pub se_vals:	IVals1,
    pub le_vals:	IVals1,
    pub be_vals:	IVals1,
}

#[repr(C)]
#[derive(Cast, Clone, Copy, Debug, Eq, Flip, PartialEq)]
pub struct IVals1 {
    pub val1_i8:	i8,
    pub val2_i8:	i8,
    pub val_i16:	i16,
    pub val_i32:	i32,
    pub val_i64:	i64,
    pub val_i128:	i128,
}


impl IData1 {
    pub fn gen() -> IData1 {
	let mut raw_bytes = [0_u8; size_of::<IVals1>()];
	let mut swp_bytes = [0_u8; size_of::<IVals1>()];

	// Generate raw bytes data.
	for i in 0 .. raw_bytes.len() {
	    raw_bytes[i] = ((255 - i * 3) & 0xFF) as u8;
	}

	// Construct swapped bytes data from raw bytes data.

	// val1_i8
	swp_bytes[0x00] = raw_bytes[0x00];
	// val2_i8
	swp_bytes[0x01] = raw_bytes[0x01];

	// val_i16
	swp_bytes[0x02] = raw_bytes[0x03];
	swp_bytes[0x03] = raw_bytes[0x02];

	// val_i32
	swp_bytes[0x04] = raw_bytes[0x07];
	swp_bytes[0x05] = raw_bytes[0x06];
	swp_bytes[0x06] = raw_bytes[0x05];
	swp_bytes[0x07] = raw_bytes[0x04];

	// val_i64
	swp_bytes[0x08] = raw_bytes[0x0F];
	swp_bytes[0x09] = raw_bytes[0x0E];
	swp_bytes[0x0A] = raw_bytes[0x0D];
	swp_bytes[0x0B] = raw_bytes[0x0C];
	swp_bytes[0x0C] = raw_bytes[0x0B];
	swp_bytes[0x0D] = raw_bytes[0x0A];
	swp_bytes[0x0E] = raw_bytes[0x09];
	swp_bytes[0x0F] = raw_bytes[0x08];

	// val_i128
	swp_bytes[0x10] = raw_bytes[0x1F];
	swp_bytes[0x11] = raw_bytes[0x1E];
	swp_bytes[0x12] = raw_bytes[0x1D];
	swp_bytes[0x13] = raw_bytes[0x1C];
	swp_bytes[0x14] = raw_bytes[0x1B];
	swp_bytes[0x15] = raw_bytes[0x1A];
	swp_bytes[0x16] = raw_bytes[0x19];
	swp_bytes[0x17] = raw_bytes[0x18];
	swp_bytes[0x18] = raw_bytes[0x17];
	swp_bytes[0x19] = raw_bytes[0x16];
	swp_bytes[0x1A] = raw_bytes[0x15];
	swp_bytes[0x1B] = raw_bytes[0x14];
	swp_bytes[0x1C] = raw_bytes[0x13];
	swp_bytes[0x1D] = raw_bytes[0x12];
	swp_bytes[0x1E] = raw_bytes[0x11];
	swp_bytes[0x1F] = raw_bytes[0x10];

	// Construct little endian values.
	let le_vals = IVals1{
	    val1_i8:	   raw_bytes[0x00] as i8,
	    val2_i8:	   raw_bytes[0x01] as i8,
	    val_i16:	(((raw_bytes[0x02] as  u16)) |
			 ((raw_bytes[0x03] as  u16) <<   8)) as i16,
	    val_i32:	(((raw_bytes[0x04] as  u32)) |
			 ((raw_bytes[0x05] as  u32) <<   8) |
			 ((raw_bytes[0x06] as  u32) <<  16) |
			 ((raw_bytes[0x07] as  u32) <<  24)) as i32,
	    val_i64:	(((raw_bytes[0x08] as  u64)) |
			 ((raw_bytes[0x09] as  u64) <<   8) |
			 ((raw_bytes[0x0A] as  u64) <<  16) |
			 ((raw_bytes[0x0B] as  u64) <<  24) |
			 ((raw_bytes[0x0C] as  u64) <<  32) |
			 ((raw_bytes[0x0D] as  u64) <<  40) |
			 ((raw_bytes[0x0E] as  u64) <<  48) |
			 ((raw_bytes[0x0F] as  u64) <<  56)) as i64,
	    val_i128:	(((raw_bytes[0x10] as u128)) |
			 ((raw_bytes[0x11] as u128) <<   8) |
			 ((raw_bytes[0x12] as u128) <<  16) |
			 ((raw_bytes[0x13] as u128) <<  24) |
			 ((raw_bytes[0x14] as u128) <<  32) |
			 ((raw_bytes[0x15] as u128) <<  40) |
			 ((raw_bytes[0x16] as u128) <<  48) |
			 ((raw_bytes[0x17] as u128) <<  56) |
			 ((raw_bytes[0x18] as u128) <<  64) |
			 ((raw_bytes[0x19] as u128) <<  72) |
			 ((raw_bytes[0x1A] as u128) <<  80) |
			 ((raw_bytes[0x1B] as u128) <<  88) |
			 ((raw_bytes[0x1C] as u128) <<  96) |
			 ((raw_bytes[0x1D] as u128) << 104) |
			 ((raw_bytes[0x1E] as u128) << 112) |
			 ((raw_bytes[0x1F] as u128) << 120)) as i128,
	};

	// Calculate big endian values.
	let be_vals = IVals1{
	    val1_i8:	le_vals.val1_i8,
	    val2_i8:	le_vals.val2_i8,
	    val_i16:	le_vals.val_i16.swap_bytes(),
	    val_i32:	le_vals.val_i32.swap_bytes(),
	    val_i64:	le_vals.val_i64.swap_bytes(),
	    val_i128:	le_vals.val_i128.swap_bytes(),
	};

	// Calculate native endian values.
	let ne_vals = IVals1{
	    val1_i8:	le_vals.val1_i8,
	    val2_i8:	le_vals.val2_i8,
	    val_i16:	i16::from_le(le_vals.val_i16),
	    val_i32:	i32::from_le(le_vals.val_i32),
	    val_i64:	i64::from_le(le_vals.val_i64),
	    val_i128:	i128::from_le(le_vals.val_i128),
	};

	// Calculate swapped endian values.
	let se_vals = IVals1{
	    val1_i8:	ne_vals.val1_i8,
	    val2_i8:	ne_vals.val2_i8,
	    val_i16:	ne_vals.val_i16.swap_bytes(),
	    val_i32:	ne_vals.val_i32.swap_bytes(),
	    val_i64:	ne_vals.val_i64.swap_bytes(),
	    val_i128:	ne_vals.val_i128.swap_bytes(),
	};

	// Construct IData1.
	return IData1{ raw_bytes, swp_bytes,
		       ne_vals, se_vals, le_vals, be_vals, };
    }
}
