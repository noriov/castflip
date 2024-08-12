use castflip::{Cast, Flip, NopFlip, EncastMem, SE, LE};

fn test_cast() {
    #[repr(C)]
    #[derive(Cast)]
    struct NamedTest {
	val1:	u16,
	val2:	u16,
    }

    #[repr(C)]
    #[derive(Cast)]
    struct UnnamedTest (
	u16,
	u16,
    );

    #[repr(C)]
    #[derive(Cast)]
    struct UnitTest;

    #[repr(C)]
    #[derive(Cast, NopFlip)]
    union UnionTest {
	val1:	u16,
	val2:	u16,
    }

    let bytes1: [u8; 4] = [0x12, 0x34, 0x56, 0x78];

    let named: NamedTest = bytes1.encast().unwrap();
    let unnamed: UnnamedTest = bytes1.encast().unwrap();
    let _unit: UnitTest = bytes1.encast().unwrap();
    let union: UnionTest = bytes1.encast().unwrap();

    if cfg!(target_endian = "little") {
	assert_eq!(named.val1, 0x3412);
	assert_eq!(named.val2, 0x7856);
	assert_eq!(unnamed.0, 0x3412);
	assert_eq!(unnamed.1, 0x7856);
	unsafe {
	    assert_eq!(union.val1, 0x3412);
	}
    } else if cfg!(target_endian = "big") {
	assert_eq!(named.val1, 0x1234);
	assert_eq!(named.val2, 0x5678);
	assert_eq!(unnamed.0, 0x1234);
	assert_eq!(unnamed.1, 0x5678);
	unsafe {
	    assert_eq!(union.val1, 0x1234);
	}
    }
}

fn test_cast_flip() {
    #[repr(C)]
    #[derive(Cast, Flip)]
    struct NamedTest {
	val1:	u16,
	val2:	u16,
    }

    #[repr(C)]
    #[derive(Cast, Flip)]
    struct UnnamedTest (
	u16,
	u16,
    );

    #[repr(C)]
    #[derive(Cast, Flip)]
    struct UnitTest;

    let bytes1: [u8; 4] = [0x12, 0x34, 0x56, 0x78];

    let named: NamedTest = bytes1.encastf(LE).unwrap();
    let unnamed: UnnamedTest = bytes1.encastf(LE).unwrap();
    let _unit: UnitTest = bytes1.encastf(LE).unwrap();

    assert_eq!(named.val1, 0x3412);
    assert_eq!(named.val2, 0x7856);
    assert_eq!(unnamed.0, 0x3412);
    assert_eq!(unnamed.1, 0x7856);
}

fn test_cast_nopflip() {
    #[repr(C)]
    #[derive(Cast, NopFlip)]
    struct NamedTest {
	val1:	u16,
	val2:	u16,
    }

    #[repr(C)]
    #[derive(Cast, NopFlip)]
    struct UnnamedTest (
	u16,
	u16,
    );

    #[repr(C)]
    #[derive(Cast, NopFlip)]
    struct UnitTest;

    #[repr(C)]
    #[derive(Cast, NopFlip)]
    union UnionTest {
	val1:	u16,
	val2:	u16,
    }

    let bytes1: [u8; 4] = [0x12, 0x34, 0x56, 0x78];

    let named: NamedTest = bytes1.encast().unwrap();
    let unnamed: UnnamedTest = bytes1.encast().unwrap();
    let _unit: UnitTest = bytes1.encast().unwrap();
    let union: UnionTest = bytes1.encast().unwrap();

    if cfg!(target_endian = "little") {
	assert_eq!(named.val1, 0x3412);
	assert_eq!(named.val2, 0x7856);
	assert_eq!(unnamed.0, 0x3412);
	assert_eq!(unnamed.1, 0x7856);
	unsafe {
	    assert_eq!(union.val1, 0x3412);
	}
    } else if cfg!(target_endian = "big") {
	assert_eq!(named.val1, 0x1234);
	assert_eq!(named.val2, 0x5678);
	assert_eq!(unnamed.0, 0x1234);
	assert_eq!(unnamed.1, 0x5678);
	unsafe {
	    assert_eq!(union.val1, 0x1234);
	}
    }
}

fn test_flip() {
    #[derive(Flip)]
    struct NamedTest {
	val1:	u16,
	val2:	u16,
    }

    #[derive(Flip)]
    struct UnnamedTest (
	u16,
	u16,
    );

    let named1 = NamedTest { val1: 0x1234, val2: 0x5678 };
    let named2 = named1.flip_val(SE);
    let mut named3 = named1;
    named3.flip_var(SE);

    let unnamed1 = UnnamedTest ( 0x1234, 0x5678 );
    let unnamed2 = unnamed1.flip_val(SE);
    let mut unnamed3 = unnamed1;
    unnamed3.flip_var(SE);

    assert_eq!(named2.val1, 0x3412);
    assert_eq!(named2.val2, 0x7856);
    assert_eq!(named3.val1, 0x3412);
    assert_eq!(named3.val2, 0x7856);
    assert_eq!(unnamed2.0, 0x3412);
    assert_eq!(unnamed2.1, 0x7856);
    assert_eq!(unnamed3.0, 0x3412);
    assert_eq!(unnamed3.1, 0x7856);
}

fn test_nopflip() {
    #[derive(NopFlip)]
    struct NamedTest {
	val1:	u16,
	val2:	u16,
    }

    #[derive(NopFlip)]
    struct UnnamedTest (
	u16,
	u16,
    );

    #[derive(NopFlip)]
    union UnionTest {
	val1:	u16,
	val2:	u16,
    }

    let named1 = NamedTest { val1: 0x1234, val2: 0x5678 };
    let named2 = named1.flip_val(SE);
    let mut named3 = named1;
    named3.flip_var(SE);

    let unnamed1 = UnnamedTest ( 0x1234, 0x5678 );
    let unnamed2 = unnamed1.flip_val(SE);
    let mut unnamed3 = unnamed1;
    unnamed3.flip_var(SE);

    let union1 = UnionTest { val1: 0x1234 };
    let union2 = union1.flip_val(SE);
    let mut union3 = UnionTest { val2: 0x5678 };
    union3.flip_var(SE);

    assert_eq!(named2.val1, 0x1234);
    assert_eq!(named2.val2, 0x5678);
    assert_eq!(named3.val1, 0x1234);
    assert_eq!(named3.val2, 0x5678);

    assert_eq!(unnamed2.0, 0x1234);
    assert_eq!(unnamed2.1, 0x5678);
    assert_eq!(unnamed3.0, 0x1234);
    assert_eq!(unnamed3.1, 0x5678);

    unsafe {
	assert_eq!(union1.val1, 0x1234);
	assert_eq!(union2.val1, 0x1234);
	assert_eq!(union3.val2, 0x5678);
    }
}

#[test]
fn test() {
    test_cast();
    test_cast_flip();
    test_cast_nopflip();
    test_flip();
    test_nopflip();
}
