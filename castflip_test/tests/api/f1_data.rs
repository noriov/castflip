use core::mem::size_of;

use castflip::{Cast, Flip};


#[derive(Clone, Copy, Debug)]
pub struct FData1 {
    pub ne_bytes:	[u8; size_of::<FVals1>()],
    pub se_bytes:	[u8; size_of::<FVals1>()],
    pub le_bytes:	[u8; size_of::<FVals1>()],
    pub be_bytes:	[u8; size_of::<FVals1>()],
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
	// Construct FVals1 from native-endian values.
	let ne_vals = FVals1 {
	    val1_f32:	::core::f32::consts::E,
	    val2_f32:	::core::f32::consts::PI,
	    val1_f64:	::core::f64::consts::E,
	    val2_f64:	::core::f64::consts::PI,
	};

	// Construct FVals1 from swapped-endian values.
	let se_vals = FVals1 {
	    val1_f32: f32::from_bits(ne_vals.val1_f32.to_bits().swap_bytes()),
	    val2_f32: f32::from_bits(ne_vals.val2_f32.to_bits().swap_bytes()),
	    val1_f64: f64::from_bits(ne_vals.val1_f64.to_bits().swap_bytes()),
	    val2_f64: f64::from_bits(ne_vals.val2_f64.to_bits().swap_bytes()),
	};

	// Construct a vector in native-endian.
	let mut ne_vec = Vec::<u8>::new();
	ne_vec.extend_from_slice(&ne_vals.val1_f32.to_ne_bytes());
	ne_vec.extend_from_slice(&ne_vals.val2_f32.to_ne_bytes());
	ne_vec.extend_from_slice(&ne_vals.val1_f64.to_ne_bytes());
	ne_vec.extend_from_slice(&ne_vals.val2_f64.to_ne_bytes());

	// Construct a vector in swapped-endian.
	let mut se_vec = Vec::<u8>::new();
	se_vec.extend_from_slice(&se_vals.val1_f32.to_ne_bytes());
	se_vec.extend_from_slice(&se_vals.val2_f32.to_ne_bytes());
	se_vec.extend_from_slice(&se_vals.val1_f64.to_ne_bytes());
	se_vec.extend_from_slice(&se_vals.val2_f64.to_ne_bytes());

	// Convert vectors into arrays.
	let ne_bytes: [u8; size_of::<FVals1>()] = ne_vec.try_into().unwrap();
	let se_bytes: [u8; size_of::<FVals1>()] = se_vec.try_into().unwrap();

	// Prepare LE and BE data.
	let (le_bytes, be_bytes, le_vals, be_vals);
	if cfg!(target_endian = "little") {
	    le_bytes = ne_bytes;
	    be_bytes = se_bytes;
	    le_vals = ne_vals;
	    be_vals = se_vals;
	} else if cfg!(target_endian = "big") {
	    le_bytes = se_bytes;
	    be_bytes = ne_bytes;
	    be_vals = ne_vals;
	    le_vals = se_vals;
	} else {
	    panic!();
	}

	// Construct FData1.
	return FData1 { ne_bytes, se_bytes, le_bytes, be_bytes,
			ne_vals, se_vals, le_vals, be_vals, };
    }
}
