use core::mem::size_of;

use castflip::{Cast, Flip};
use crate::{UData1, UVals1, UData2, UVals2, UData3, UVals3};


#[derive(Debug)]
pub struct UData4 {
    pub ne_vals:	UVals4,
    pub se_vals:	UVals4,
    pub le_vals:	UVals4,
    pub be_vals:	UVals4,
    pub ne_bytes:	[u8; size_of::<UVals4>()],
    pub se_bytes:	[u8; size_of::<UVals4>()],
    pub le_bytes:	[u8; size_of::<UVals4>()],
    pub be_bytes:	[u8; size_of::<UVals4>()],
}

#[repr(C)]
#[derive(Cast, Clone, Copy, Debug, Eq, Flip, PartialEq)]
pub struct UVals4 {
    pub vals1:		UVals1,
    pub vals2:		UVals2,
    pub vals3:		UVals3,
}


impl UData4 {
    pub fn gen() -> UData4 {
	let udata1 = UData1::gen();
	let udata2 = UData2::gen();
	let udata3 = UData3::gen();

	// Construct a vector in native-endian.
	let mut ne_vec: Vec<u8> = Vec::new();
	ne_vec.extend_from_slice(&udata1.ne_bytes);
	ne_vec.extend_from_slice(&udata2.ne_bytes);
	ne_vec.extend_from_slice(&udata3.ne_bytes);

	// Construct a vector in swappede-endian.
	let mut se_vec: Vec<u8> = Vec::new();
	se_vec.extend_from_slice(&udata1.se_bytes);
	se_vec.extend_from_slice(&udata2.se_bytes);
	se_vec.extend_from_slice(&udata3.se_bytes);

	// Convert vectors into arrays.
	let ne_bytes: [u8; size_of::<UVals4>()] = ne_vec.try_into().unwrap();
	let se_bytes: [u8; size_of::<UVals4>()] = se_vec.try_into().unwrap();

	// Construct native endian values.
	let ne_vals = UVals4 {
	    vals1:	udata1.ne_vals,
	    vals2:	udata2.ne_vals,
	    vals3:	udata3.ne_vals,
	};

	// Construct swapped endian values.
	let se_vals = UVals4 {
	    vals1:	udata1.se_vals,
	    vals2:	udata2.se_vals,
	    vals3:	udata3.se_vals,
	};

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

	// Construct UData1.
	return UData4 { ne_bytes, se_bytes, le_bytes, be_bytes,
			ne_vals, se_vals, le_vals, be_vals, };
    }
}
