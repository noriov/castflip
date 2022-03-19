use castflip::experimental::Reslice;
use crate::{UData2, UVals2};


#[test]
fn udata2() {
    unsafe {
	// Test1: &[u8] -> &[u32]

	let udata2 = UData2::gen();

	let mut raw_bytes = udata2.raw_bytes;
	let mut swp_bytes = udata2.swp_bytes;

	let raw_slice_u8 = &mut raw_bytes[..];
	let swp_slice_u8 = &mut swp_bytes[..];

	let raw_slice_u32 = raw_slice_u8.reslice_mut::<u32>().unwrap();

	// The type parameter of reslice() can be omitted where
	// the Rust compiler can infer the type of the result.
	let swp_slice_u32: &mut [u32] = swp_slice_u8.reslice_mut().unwrap();

	assert_eq!(raw_slice_u32, &udata2.ne_vals.array[..]);
	assert_eq!(swp_slice_u32, &udata2.se_vals.array[..]);
    }
    unsafe {
	// Test2: &[u32] -> &[u8]

	let udata2 = UData2::gen();

	let mut ne_array = udata2.ne_vals.array;
	let mut se_array = udata2.se_vals.array;

	let ne_slice_u32 = &mut ne_array[..];
	let se_slice_u32 = &mut se_array[..];

	let raw_slice_u8 = ne_slice_u32.reslice_mut::<u8>().unwrap();
	let swp_slice_u8: &mut [u8] = se_slice_u32.reslice_mut().unwrap();

	assert_eq!(raw_slice_u8, &udata2.raw_bytes[..]);
	assert_eq!(swp_slice_u8, &udata2.swp_bytes[..]);
    }
    unsafe {
	// Test3: &[UVals2] -> &[u32]

	let udata2 = UData2::gen();

	let ne_slice_uvals2 = &mut [udata2.ne_vals];
	let se_slice_uvals2 = &mut [udata2.se_vals];

	let ne_slice_u32 = ne_slice_uvals2.reslice_mut::<u32>().unwrap();
	let se_slice_u32: &mut [u32] = se_slice_uvals2.reslice_mut().unwrap();

	assert_eq!(ne_slice_u32, &udata2.ne_vals.array[..]);
	assert_eq!(se_slice_u32, &udata2.se_vals.array[..]);
    }
    unsafe {
	// Test4: &[u32] -> &[UVals2]

	let udata2 = UData2::gen();

	let mut ne_array = udata2.ne_vals.array;
	let mut se_array = udata2.se_vals.array;

	let ne_slice_u32 = &mut ne_array[..];
	let se_slice_u32 = &mut se_array[..];

	let ne_slice_uvals2 = ne_slice_u32.reslice_mut::<UVals2>().unwrap();
	let se_slice_uvals2: &mut [UVals2] =
	    se_slice_u32.reslice_mut().unwrap();

	assert_eq!(ne_slice_uvals2[0], udata2.ne_vals);
	assert_eq!(se_slice_uvals2[0], udata2.se_vals);
    }
    unsafe {
	// Test5: empty &mut [u32] -> empty &mut [u8]
	let slice_u32_1: &mut [u32] = &mut [];
	let slice_u32_2: &mut [u32] = &mut [];

	let slice_u8_1 = slice_u32_1.reslice_mut::<u8>().unwrap();
	let slice_u8_2: &mut [u8] = slice_u32_2.reslice_mut().unwrap();

	assert_eq!(slice_u8_1.len(), 0);
	assert_eq!(slice_u8_2.len(), 0);
    }
}
