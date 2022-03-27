//
// An example of encoding and decoding Mach-o Fat Header
//
// Reference:
//	struct fat_header and struct fat_arch in <mach-o/fat.h> at
//	/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include/
//
//	https://en.wikipedia.org/wiki/Mach-O
//

use core::mem::size_of_val;

use castflip::{Cast, DecastMem, EncastMem, Endian, Flip};


// Rust declaration of struct fat_header
#[repr(C)]
#[derive(Cast, Flip)]
struct FatHeader {
    magic:	u32,
    nfat_arch:	u32,
}

const FAT_MAGIC32: u32 = 0xCAFEBABE;	// Native Endian, 32-bit
const FAT_MAGIC64: u32 = 0xCAFEBABF;	// Native Endian, 64-bit
const FAT_CIGAM32: u32 = 0xBEBAFECA;	// Swapped Endian, 32-bit
const FAT_CIGAM64: u32 = 0xBFBAFECA;	// Swapped Endian, 64-bit


// Rust declaration of struct fat_arch
#[repr(C)]
#[derive(Cast, Flip)]
struct FatArch {
    cputype:	i32,
    cpusubtype:	i32,
    offset:	u32,
    size:	u32,
    align:	u32,
}


// Storage of results.
struct FatInfo {
    hdr:	FatHeader,
    arch_vec:	Vec<FatArch>,
    endian:	Endian,
}

impl FatInfo {
    // In the example below, the generic type parameters of EncastMem and
    // DecastMem are omitted because the Rust compiler can infer them.

    fn read(bytes: &[u8]) -> Option<Self> {
	// The generic type parameter can be inferred as u32
	// because the types of the constants are u32.
	let endian = match bytes.encast()? {
	    FAT_MAGIC32 | FAT_MAGIC64 => Endian::Native,
	    FAT_CIGAM32 | FAT_CIGAM64 => Endian::Swapped,
	    _ => { return None; },
	};

	let hdr: FatHeader = bytes.encastf(endian)?;

	let arch_off = size_of_val(&hdr);
	let narch = hdr.nfat_arch as usize;
	let arch_vec = bytes[arch_off ..].encastvf(narch, endian)?;

	Some(Self { hdr, arch_vec, endian })
    }

    fn write(&self, bytes: &mut [u8]) -> Option<usize> {
	let mut off = 0;
	off += bytes.decastf(&self.hdr, self.endian)?;
	off += bytes[off ..].decastvf(&self.arch_vec, self.endian)?;
	Some(off)
    }
}


#[test]
fn test() {
    // Fictional example of Mach-o fat header.
    const DATA_SIZE: usize = 0x30;
    let bytes_input: [u8; DATA_SIZE] = [
	0xCA, 0xFE, 0xBA, 0xBE, 0x00, 0x00, 0x00, 0x02,
	0x01, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x03,
	0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x9A, 0xB0,
	0x00, 0x00, 0x00, 0x0E, 0x01, 0x00, 0x00, 0x0C,
	0x80, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x00,
	0x00, 0x00, 0x9B, 0x70, 0x00, 0x00, 0x00, 0x0E,
    ];

    // Output buffer (initially filled with zeros).
    let mut bytes_output = [0_u8; DATA_SIZE];

    // Read Mach-o fat header.
    let fat = FatInfo::read(&bytes_input).unwrap();

    // Write Mach-o fat header.
    let size = fat.write(&mut bytes_output).unwrap();

    // Check the output.
    assert_eq!(size, DATA_SIZE);
    assert_eq!(bytes_output, bytes_input);

    // Check the input.
    assert_eq!(fat.hdr.magic, FAT_MAGIC32);		// = 0xCAFEBABE
    assert_eq!(fat.hdr.nfat_arch, 2);			// Number of archs
    assert_eq!(fat.arch_vec[0].cputype, 0x1000007);	// X86_64
    assert_eq!(fat.arch_vec[0].cpusubtype, 3);		// X86_ALL
    assert_eq!(fat.arch_vec[0].offset, 0x4000);		// File offset
    assert_eq!(fat.arch_vec[0].size, 0x9AB0);		// Size
    assert_eq!(fat.arch_vec[0].align, 0x0E);		// Alignment
    assert_eq!(fat.arch_vec[1].cputype, 0x100000C);	// ARM64
    assert_eq!(fat.arch_vec[1].cpusubtype,
	       0x80000002_u32 as i32);    		// ARM64E + PtrAuth
    assert_eq!(fat.arch_vec[1].offset, 0x10000);	// File offset
    assert_eq!(fat.arch_vec[1].size, 0x9B70);		// Size
    assert_eq!(fat.arch_vec[1].align, 0x0E);		// Alignment
}
