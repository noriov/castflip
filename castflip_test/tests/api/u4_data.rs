use core::mem::size_of;

use castflip::{Cast, Flip};
use crate::{UData1, UVals1, UData2, UVals2, UData3, UVals3};


#[derive(Debug)]
pub struct UData4 {
    pub raw_bytes:	[u8; size_of::<UVals4>()],
    pub swp_bytes:	[u8; size_of::<UVals4>()],
    pub ne_vals:	UVals4,
    pub se_vals:	UVals4,
    pub le_vals:	UVals4,
    pub be_vals:	UVals4,
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
	let mut raw_bytes = [0_u8; size_of::<UVals4>()];
	let mut swp_bytes = [0_u8; size_of::<UVals4>()];

	let udata1 = UData1::gen();
	let udata2 = UData2::gen();
	let udata3 = UData3::gen();

	// Construct raw bytes data.
	let mut raw_vec: Vec<u8> = Vec::new();
	raw_vec.extend_from_slice(&udata1.raw_bytes);
	raw_vec.extend_from_slice(&udata2.raw_bytes);
	raw_vec.extend_from_slice(&udata3.raw_bytes);
	raw_bytes.copy_from_slice(&raw_vec);

	// Construct swapped bytes data.
	let mut swp_vec: Vec<u8> = Vec::new();
	swp_vec.extend_from_slice(&udata1.swp_bytes);
	swp_vec.extend_from_slice(&udata2.swp_bytes);
	swp_vec.extend_from_slice(&udata3.swp_bytes);
	swp_bytes.copy_from_slice(&swp_vec);

	// Construct native endian values.
	let ne_vals = UVals4{
	    vals1:	udata1.ne_vals,
	    vals2:	udata2.ne_vals,
	    vals3:	udata3.ne_vals,
	};

	// Construct swapped endian values.
	let se_vals = UVals4{
	    vals1:	udata1.se_vals,
	    vals2:	udata2.se_vals,
	    vals3:	udata3.se_vals,
	};

	// Construct little endian values.
	let le_vals = UVals4{
	    vals1:	udata1.le_vals,
	    vals2:	udata2.le_vals,
	    vals3:	udata3.le_vals,
	};

	// Calculate big endian values.
	let be_vals = UVals4{
	    vals1:	udata1.be_vals,
	    vals2:	udata2.be_vals,
	    vals3:	udata3.be_vals,
	};

	return UData4{ raw_bytes, swp_bytes,
		       ne_vals, se_vals, le_vals, be_vals, };
    }
}
