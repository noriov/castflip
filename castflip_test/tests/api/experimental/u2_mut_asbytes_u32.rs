use castflip::experimental::AsBytes;
use crate::UData2;


#[test]
fn udata2() {
    unsafe {
	// Test1: UVals2 -> &[u8]

	let mut udata2 = UData2::gen();

	let ne_slice_u8 = udata2.ne_vals.as_bytes_mut();
	let se_slice_u8 = udata2.se_vals.as_bytes_mut();

	assert_eq!(ne_slice_u8, &udata2.raw_bytes[..]);
	assert_eq!(se_slice_u8, &udata2.swp_bytes[..]);
    }
    unsafe {
	// Test2: &[u32] -> &[u8]

	let mut udata2 = UData2::gen();

	let ne_slice_u32 = &mut udata2.ne_vals.array[..];
	let se_slice_u32 = &mut udata2.se_vals.array[..];

	let raw_slice_u8 = ne_slice_u32.as_bytes_mut();
	let swp_slice_u8 = se_slice_u32.as_bytes_mut();

	assert_eq!(raw_slice_u8, &udata2.raw_bytes[..]);
	assert_eq!(swp_slice_u8, &udata2.swp_bytes[..]);
    }
    unsafe {
	// Test3: empty &[u32] -> empty &[u8]
	let slice_u32: &mut [u32] = &mut [];
	let slice_u8 = slice_u32.as_bytes_mut();
	assert_eq!(slice_u8.len(), 0);
    }
}
