use castflip::experimental::Reslice;
use crate::{UData2, UVals2};


#[test]
fn udata2() {
    unsafe {
	// Test1: &[u8] -> &[u32]

	let udata2 = UData2::gen();

	let mut ne_bytes = udata2.ne_bytes;

	let ne_slice_u8 = &mut ne_bytes[..];

	let ne_slice_u32 = ne_slice_u8.reslice_mut::<u32>().unwrap();

	assert_eq!(ne_slice_u32, &udata2.ne_vals.array[..]);
    }
    unsafe {
	// Test2: &[u32] -> &[u8]

	let udata2 = UData2::gen();

	let mut ne_array = udata2.ne_vals.array;

	let ne_slice_u32 = &mut ne_array[..];

	let ne_slice_u8 = ne_slice_u32.reslice_mut::<u8>().unwrap();

	assert_eq!(ne_slice_u8, &udata2.ne_bytes[..]);
    }
    unsafe {
	// Test3: &[UVals2] -> &[u32]

	let udata2 = UData2::gen();

	let ne_slice_uvals2 = &mut [udata2.ne_vals];

	let ne_slice_u32 = ne_slice_uvals2.reslice_mut::<u32>().unwrap();

	assert_eq!(ne_slice_u32, &udata2.ne_vals.array[..]);
    }
    unsafe {
	// Test4: &[u32] -> &[UVals2]

	let udata2 = UData2::gen();

	let mut ne_array = udata2.ne_vals.array;

	let ne_slice_u32 = &mut ne_array[..];

	let ne_slice_uvals2 = ne_slice_u32.reslice_mut::<UVals2>().unwrap();

	assert_eq!(ne_slice_uvals2[0], udata2.ne_vals);
    }
    unsafe {
	// Test5: empty &mut [u32] -> empty &mut [u8]
	let slice_u32: &mut [u32] = &mut [];

	let slice_u8 = slice_u32.reslice_mut::<u8>().unwrap();

	assert_eq!(slice_u8.len(), 0);
    }
}
