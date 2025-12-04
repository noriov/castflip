use castflip::experimental::FlipUnsized;
use castflip::{NE, SE, LE, BE};
use crate::UData2;


#[test]
fn udata2() {
    {
        let udata2 = UData2::gen();

        let mut se_array = udata2.ne_vals.array;

        let se_slice = &mut se_array[..];

        se_slice.flip_var_swapped();

        assert_eq!(se_slice, &udata2.se_vals.array[..]);

        assert_eq!(se_array, udata2.se_vals.array);
    }
    {
        let udata2 = UData2::gen();

        let mut ne_array = udata2.ne_vals.array;
        let mut se_array = udata2.ne_vals.array;
        let mut le_array = udata2.ne_vals.array;
        let mut be_array = udata2.ne_vals.array;

        let ne_slice = &mut ne_array[..];
        let se_slice = &mut se_array[..];
        let le_slice = &mut le_array[..];
        let be_slice = &mut be_array[..];

        ne_slice.flip_var(NE);
        se_slice.flip_var(SE);
        le_slice.flip_var(LE);
        be_slice.flip_var(BE);

        assert_eq!(ne_slice, &udata2.ne_vals.array[..]);
        assert_eq!(se_slice, &udata2.se_vals.array[..]);
        assert_eq!(le_slice, &udata2.le_vals.array[..]);
        assert_eq!(be_slice, &udata2.be_vals.array[..]);

        assert_eq!(ne_array, udata2.ne_vals.array);
        assert_eq!(se_array, udata2.se_vals.array);
        assert_eq!(le_array, udata2.le_vals.array);
        assert_eq!(be_array, udata2.be_vals.array);
    }
}
