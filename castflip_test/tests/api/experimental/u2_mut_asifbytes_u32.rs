use castflip::experimental::AsifBytes;
use crate::UData2;


#[test]
fn udata2() {
    unsafe {
        // Test1: UVals2 -> &[u8]

        let mut udata2 = UData2::gen();

        let ne_slice_u8 = udata2.ne_vals.asif_bytes_mut();

        assert_eq!(ne_slice_u8, &udata2.ne_bytes[..]);
    }
    unsafe {
        // Test2: &[u32] -> &[u8]

        let mut udata2 = UData2::gen();

        let ne_slice_u32 = &mut udata2.ne_vals.array[..];

        let ne_slice_u8 = ne_slice_u32.asif_bytes_mut();

        assert_eq!(ne_slice_u8, &udata2.ne_bytes[..]);
    }
    unsafe {
        // Test3: empty &[u32] -> empty &[u8]
        let slice_u32: &mut [u32] = &mut [];
        let slice_u8 = slice_u32.asif_bytes_mut();
        assert_eq!(slice_u8.len(), 0);
    }
}
