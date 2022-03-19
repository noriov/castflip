use core::mem::size_of;

use castflip::{Cast, Flip};


#[derive(Clone, Copy, Debug)]
pub struct FData1 {
    pub raw_bytes:	[u8; size_of::<FVals1>()],
    pub swp_bytes:	[u8; size_of::<FVals1>()],
    pub ne_vals:	FVals1,
    pub se_vals:	FVals1,
    pub le_vals:	FVals1,
    pub be_vals:	FVals1,
}

#[repr(C)]
#[derive(Cast, Clone, Copy, Debug, Flip, PartialEq)]
pub struct FVals1 {
    pub val1_f32:	f32,
    pub val2_f32:	f32,
    pub val1_f64:	f64,
    pub val2_f64:	f64,
}


impl FData1 {
    pub fn gen() -> FData1 {
	// Construct FVals1 from native endian values.
	let ne_vals = FVals1{
	    val1_f32:	::core::f32::consts::E,
	    val2_f32:	::core::f32::consts::PI,
	    val1_f64:	::core::f64::consts::E,
	    val2_f64:	::core::f64::consts::PI,
	};

	// Convert native endian values to raw bytes data.
	let ne_val1_f32_bytes = ne_vals.val1_f32.to_ne_bytes();
	let ne_val2_f32_bytes = ne_vals.val2_f32.to_ne_bytes();
	let ne_val1_f64_bytes = ne_vals.val1_f64.to_ne_bytes();
	let ne_val2_f64_bytes = ne_vals.val2_f64.to_ne_bytes();

	// Convert raw bytes data to swapped bytes data.
	let mut se_val1_f32_bytes = ne_val1_f32_bytes;
	let mut se_val2_f32_bytes = ne_val2_f32_bytes;
	let mut se_val1_f64_bytes = ne_val1_f64_bytes;
	let mut se_val2_f64_bytes = ne_val2_f64_bytes;
	se_val1_f32_bytes.reverse();
	se_val2_f32_bytes.reverse();
	se_val1_f64_bytes.reverse();
	se_val2_f64_bytes.reverse();

	// Construct FVals1 of swapped endian values.
	let se_vals = FVals1{
	    val1_f32:	f32::from_ne_bytes(se_val1_f32_bytes),
	    val2_f32:	f32::from_ne_bytes(se_val2_f32_bytes),
	    val1_f64:	f64::from_ne_bytes(se_val1_f64_bytes),
	    val2_f64:	f64::from_ne_bytes(se_val2_f64_bytes),
	};

	// Prepare two FVals1's of little endian values and big endian values.
	let (le_vals, be_vals);
	if cfg!(target_endian = "little") {
	    le_vals = ne_vals;
	    be_vals = se_vals;
	} else if cfg!(target_endian = "big") {
	    be_vals = ne_vals;
	    le_vals = se_vals;
	} else {
	    panic!();
	}

	// Construct Vec<u8> from raw bytes data.
	let mut ne_bytes2 = Vec::<u8>::new();
	ne_bytes2.extend_from_slice(&ne_val1_f32_bytes);
	ne_bytes2.extend_from_slice(&ne_val2_f32_bytes);
	ne_bytes2.extend_from_slice(&ne_val1_f64_bytes);
	ne_bytes2.extend_from_slice(&ne_val2_f64_bytes);

	// Construct Vec<u8> from swapped bytes data.
	let mut se_bytes2 = Vec::<u8>::new();
	se_bytes2.extend_from_slice(&se_val1_f32_bytes);
	se_bytes2.extend_from_slice(&se_val2_f32_bytes);
	se_bytes2.extend_from_slice(&se_val1_f64_bytes);
	se_bytes2.extend_from_slice(&se_val2_f64_bytes);

	// Construct two arrays of raw bytes data and swapped bytes data.
	let mut raw_bytes = [0_u8; size_of::<FVals1>()];
	let mut swp_bytes = [0_u8; size_of::<FVals1>()];
	raw_bytes.copy_from_slice(&ne_bytes2);
	swp_bytes.copy_from_slice(&se_bytes2);

	// Construct FData1.
	return FData1{ raw_bytes, swp_bytes,
		       ne_vals, se_vals, le_vals, be_vals, };
    }
}
