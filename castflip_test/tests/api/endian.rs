use castflip::Endian;


#[test]
fn test() {
    if cfg!(target_endian = "little") {
        // Test1: relative()
        assert_eq!(Endian::Native.relative(), Endian::Native);
        assert_eq!(Endian::Swapped.relative(), Endian::Swapped);
        assert_eq!(Endian::Little.relative(), Endian::Native);
        assert_eq!(Endian::Big.relative(), Endian::Swapped);

        // Test2: absolute()
        assert_eq!(Endian::Native.absolute(), Endian::Little);
        assert_eq!(Endian::Swapped.absolute(), Endian::Big);
        assert_eq!(Endian::Little.absolute(), Endian::Little);
        assert_eq!(Endian::Big.absolute(), Endian::Big);

        // Test3: need_swap()
        assert_eq!(Endian::Native.need_swap(), false);
        assert_eq!(Endian::Swapped.need_swap(), true);
        assert_eq!(Endian::Little.need_swap(), false);
        assert_eq!(Endian::Big.need_swap(), true);
    } else if cfg!(target_endian = "big") {
        // Test1: relative()
        assert_eq!(Endian::Native.relative(), Endian::Native);
        assert_eq!(Endian::Swapped.relative(), Endian::Swapped);
        assert_eq!(Endian::Little.relative(), Endian::Swapped);
        assert_eq!(Endian::Big.relative(), Endian::Native);

        // Test2: absolute()
        assert_eq!(Endian::Native.absolute(), Endian::Big);
        assert_eq!(Endian::Swapped.absolute(), Endian::Little);
        assert_eq!(Endian::Little.absolute(), Endian::Little);
        assert_eq!(Endian::Big.absolute(), Endian::Big);

        // Test3: need_swap()
        assert_eq!(Endian::Native.need_swap(), false);
        assert_eq!(Endian::Swapped.need_swap(), true);
        assert_eq!(Endian::Little.need_swap(), true);
        assert_eq!(Endian::Big.need_swap(), false);
    } else {
        panic!();
    }

    // Test4: name()
    assert_eq!(Endian::Native.name(), "Native");
    assert_eq!(Endian::Swapped.name(), "Swapped");
    assert_eq!(Endian::Little.name(), "Little");
    assert_eq!(Endian::Big.name(), "Big");
}
