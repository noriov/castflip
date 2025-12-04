use castflip::{Flip, NE, SE, LE, BE};
use crate::UData2;


#[test]
fn udata2() {
    {
        let udata2 = UData2::gen();

        let mut se_array = udata2.ne_vals.array;

        se_array.flip_var_swapped();

        assert_eq!(se_array, udata2.se_vals.array);
    }
    {
        let udata2 = UData2::gen();

        let mut ne_array = udata2.ne_vals.array;
        let mut se_array = udata2.ne_vals.array;
        let mut le_array = udata2.ne_vals.array;
        let mut be_array = udata2.ne_vals.array;

        ne_array.flip_var(NE);
        se_array.flip_var(SE);
        le_array.flip_var(LE);
        be_array.flip_var(BE);

        assert_eq!(ne_array, udata2.ne_vals.array);
        assert_eq!(se_array, udata2.se_vals.array);
        assert_eq!(le_array, udata2.le_vals.array);
        assert_eq!(be_array, udata2.be_vals.array);
    }
}
