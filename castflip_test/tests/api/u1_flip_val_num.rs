use castflip::{Flip, NE, SE, LE, BE};
use crate::{UData1, UVals1};


#[test]
fn udata1() {
    {
	let udata1 = UData1::gen();

	let se_vals_from_ne = UVals1 {
	    val1_u8:  udata1.ne_vals.val1_u8.flip_val_swapped(),
	    val2_u8:  udata1.ne_vals.val2_u8.flip_val_swapped(),
	    val_u16:  udata1.ne_vals.val_u16.flip_val_swapped(),
	    val_u32:  udata1.ne_vals.val_u32.flip_val_swapped(),
	    val_u64:  udata1.ne_vals.val_u64.flip_val_swapped(),
	    val_u128: udata1.ne_vals.val_u128.flip_val_swapped(),
	};

	let ne_vals_from_se = UVals1 {
	    val1_u8:  udata1.se_vals.val1_u8.flip_val_swapped(),
	    val2_u8:  udata1.se_vals.val2_u8.flip_val_swapped(),
	    val_u16:  udata1.se_vals.val_u16.flip_val_swapped(),
	    val_u32:  udata1.se_vals.val_u32.flip_val_swapped(),
	    val_u64:  udata1.se_vals.val_u64.flip_val_swapped(),
	    val_u128: udata1.se_vals.val_u128.flip_val_swapped(),
	};

	assert_eq!(se_vals_from_ne, udata1.se_vals);
	assert_eq!(ne_vals_from_se, udata1.ne_vals);
    }
    {
	let udata1 = UData1::gen();

	let ne_vals_from_ne = UVals1 {
	    val1_u8:  udata1.ne_vals.val1_u8.flip_val(NE),
	    val2_u8:  udata1.ne_vals.val2_u8.flip_val(NE),
	    val_u16:  udata1.ne_vals.val_u16.flip_val(NE),
	    val_u32:  udata1.ne_vals.val_u32.flip_val(NE),
	    val_u64:  udata1.ne_vals.val_u64.flip_val(NE),
	    val_u128: udata1.ne_vals.val_u128.flip_val(NE),
	};

	let se_vals_from_ne = UVals1 {
	    val1_u8:  udata1.ne_vals.val1_u8.flip_val(SE),
	    val2_u8:  udata1.ne_vals.val2_u8.flip_val(SE),
	    val_u16:  udata1.ne_vals.val_u16.flip_val(SE),
	    val_u32:  udata1.ne_vals.val_u32.flip_val(SE),
	    val_u64:  udata1.ne_vals.val_u64.flip_val(SE),
	    val_u128: udata1.ne_vals.val_u128.flip_val(SE),
	};

	let le_vals_from_ne = UVals1 {
	    val1_u8:  udata1.ne_vals.val1_u8.flip_val(LE),
	    val2_u8:  udata1.ne_vals.val2_u8.flip_val(LE),
	    val_u16:  udata1.ne_vals.val_u16.flip_val(LE),
	    val_u32:  udata1.ne_vals.val_u32.flip_val(LE),
	    val_u64:  udata1.ne_vals.val_u64.flip_val(LE),
	    val_u128: udata1.ne_vals.val_u128.flip_val(LE),
	};

	let be_vals_from_ne = UVals1 {
	    val1_u8:  udata1.ne_vals.val1_u8.flip_val(BE),
	    val2_u8:  udata1.ne_vals.val2_u8.flip_val(BE),
	    val_u16:  udata1.ne_vals.val_u16.flip_val(BE),
	    val_u32:  udata1.ne_vals.val_u32.flip_val(BE),
	    val_u64:  udata1.ne_vals.val_u64.flip_val(BE),
	    val_u128: udata1.ne_vals.val_u128.flip_val(BE),
	};

	assert_eq!(ne_vals_from_ne, udata1.ne_vals);
	assert_eq!(se_vals_from_ne, udata1.se_vals);
	assert_eq!(le_vals_from_ne, udata1.le_vals);
	assert_eq!(be_vals_from_ne, udata1.be_vals);
    }
}
