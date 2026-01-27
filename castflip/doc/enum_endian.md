Defines two types of endiannesses: Relative Endiannesses (Native and
Swapped) and Absolute Endiannesses (Little and Big).

# Description

Enum [`Endian`] defines two categories of endiannesses.

1. Relative Endiannesses
   - Native-Endianness
     (denoted as [`Endian::Native`], or [`NE`] for short)
   - Swapped-Endianness
     (denoted as [`Endian::Swapped`], or [`SE`] for short)

2. Absolute Endiannesses
   - Little-Endianness
     (denoted as [`Endian::Little`], or [`LE`] for short)
   - Big-Endianness
     (denoted as [`Endian::Big`], or [`BE`] for short)

The following example shows the relationship between these two
categories.  Note that method [`Endian::absolute`] returns the
absolute endianness of `self` and method [`Endian::relative`] returns
the relative endianness of `self`.

```rust
# use castflip::Endian;
if cfg!(target_endian = "little") {
    assert_eq!(Endian::Native.absolute(), Endian::Little);
    assert_eq!(Endian::Swapped.absolute(), Endian::Big);
    assert_eq!(Endian::Little.relative(), Endian::Native);
    assert_eq!(Endian::Big.relative(), Endian::Swapped);
} else if cfg!(target_endian = "big") {
    assert_eq!(Endian::Native.absolute(), Endian::Big);
    assert_eq!(Endian::Swapped.absolute(), Endian::Little);
    assert_eq!(Endian::Little.relative(), Endian::Swapped);
    assert_eq!(Endian::Big.relative(), Endian::Native);
# } else {
#   panic!();
}
```

# Relative Endianness vs. Absolute Endianness

The main purpose of enum [`Endian`] is to specify the endianness of a
byte representation of a type.

In most cases, the endianness of a byte representation is specified
using an absolute endianness.  For example, most protocols of the
[Internet protocol suite] adopt big-endianness, while some of them
adopt little-endianness.  In such cases, it is simple to specify the
endianness of each byte representation using an absolute endianness.

In some cases, the endianness of a byte representation needs to be
determined by checking a multi-byte field in a header.  For example,
the [Mach-O] header and the [PCAP] file format have a multi-byte field
to indicate both the endianness and the file type of their files.  In
such cases, it is often simple to specify the endianness of a byte
representation using a relative endianness to parse them.

For another example, the [ELF] format and the [X11] protocol declare
the endianness of their data fields using a single-byte field.  In
such cases, it is simple to specify the endianness of a byte
representation using an absolute endianness to parse them.

[Internet protocol suite]: https://en.wikipedia.org/wiki/Internet_protocol_suite
[Mach-O]: https://en.wikipedia.org/wiki/Mach-O
[PCAP]: https://github.com/IETF-OPSAWG-WG/draft-ietf-opsawg-pcap
[ELF]: https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
[X11]: https://www.x.org/releases/X11R7.7/doc/xproto/x11protocol.html
