use castflip::experimental::Reslice;
use crate::{UData2, UVals2};


#[test]
fn udata2() {
    unsafe {
	// Test1: &[u8] -> &[u32]

	let udata2 = UData2::gen();

	let raw_slice_u8 = &udata2.raw_bytes[..];
	let swp_slice_u8 = &udata2.swp_bytes[..];

	let raw_slice_u32 = raw_slice_u8.reslice::<u32>().unwrap();

	// The type parameter of reslice() can be omitted where
	// the Rust compiler can infer the type of the result.
	let swp_slice_u32: &[u32] = swp_slice_u8.reslice().unwrap();

	assert_eq!(raw_slice_u32, &udata2.ne_vals.array[..]);
	assert_eq!(swp_slice_u32, &udata2.se_vals.array[..]);
    }
    unsafe {
	// Test2: &[u32] -> &[u8]

	let udata2 = UData2::gen();

	let ne_slice_u32 = &udata2.ne_vals.array[..];
	let se_slice_u32 = &udata2.se_vals.array[..];

	let raw_slice_u8 = ne_slice_u32.reslice::<u8>().unwrap();
	let swp_slice_u8: &[u8] = se_slice_u32.reslice().unwrap();

	assert_eq!(raw_slice_u8, &udata2.raw_bytes[..]);
	assert_eq!(swp_slice_u8, &udata2.swp_bytes[..]);
    }
    unsafe {
	// Test3: &[UVals2] -> &[u32]

	let udata2 = UData2::gen();

	let ne_slice_uvals2 = &[udata2.ne_vals];
	let se_slice_uvals2 = &[udata2.se_vals];

	let ne_slice_u32 = ne_slice_uvals2.reslice::<u32>().unwrap();
	let se_slice_u32: &[u32] = se_slice_uvals2.reslice().unwrap();

	assert_eq!(ne_slice_u32, &udata2.ne_vals.array[..]);
	assert_eq!(se_slice_u32, &udata2.se_vals.array[..]);
    }
    unsafe {
	// Test4: &[u32] -> &[UVals2]

	let udata2 = UData2::gen();

	let ne_slice_u32 = &udata2.ne_vals.array[..];
	let se_slice_u32 = &udata2.se_vals.array[..];

	let ne_slice_uvals2 = ne_slice_u32.reslice::<UVals2>().unwrap();
	let se_slice_uvals2: &[UVals2] = se_slice_u32.reslice().unwrap();

	assert_eq!(ne_slice_uvals2[0], udata2.ne_vals);
	assert_eq!(se_slice_uvals2[0], udata2.se_vals);
    }
    unsafe {
	// Test5: empty &[u32] -> empty &[u8]
	let slice_u32: &[u32] = &[];
	let slice_u8_1 = slice_u32.reslice::<u8>().unwrap();
	let slice_u8_2: &[u8] = slice_u32.reslice().unwrap();
	assert_eq!(slice_u8_1.len(), 0);
	assert_eq!(slice_u8_2.len(), 0);
    }
}
