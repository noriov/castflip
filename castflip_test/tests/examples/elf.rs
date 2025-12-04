//
// An example of encoding and decoding ELF Header (64-bit)
//
// Reference:
//      Elf64_Ehdr in <elf.h> on Linux
//
//      https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
//

use castflip::{Cast, DecastMem, EncastMem, Endian, Flip};


// Rust declaration of e_ident in Elf64_Ehdr
#[repr(C)]
#[derive(Cast, Flip)]
struct ElfIdent {
    magic:      [u8; 4],        // Magic 0x7F "ELF"
    class:      u8,             // File class (1 = 32-bit, 2 = 64-bit)
    data:       u8,             // Data encoding (1 = little, 2 = big endian)
    version:    u8,             // File version
    osabi:      u8,             // OS ABI
    abiversion: u8,             // ABI version
    pad:        [u8; 7],        // padding
}

const ELF_MAGIC: [u8; 4] = [ 0x7F, b'E', b'L', b'F' ];

impl ElfIdent {
    fn read(bytes: &[u8]) -> Option<(u32, Endian)> {
        let ident = bytes.encast::<Self>()?;
        let bits = match ident.class {
            1 => 32,    // ELF 32-bit
            2 => 64,    // ELF 64-bit
            _ => { return None; }
        };
        let endian = match ident.data {
            1 => Endian::Little,
            2 => Endian::Big,
            _ => { return None; }
        };
        Some((bits, endian))
    }
}


// Rust declaration of Elf64_Ehdr
#[repr(C)]
#[derive(Cast, Flip)]
struct Elf64Ehdr {
    ident:      ElfIdent,
    file_type:  u16,            // Object file type
    machine:    u16,            // Architecture
    version:    u32,            // Object file version
    entry:      u64,            // Entry point virtual address
    phoff:      u64,            // Program header table file offset
    shoff:      u64,            // Section header table file offset
    flags:      u32,            // Processor-specific flags
    ehsize:     u16,            // ELF header size in bytes
    phentsize:  u16,            // Program header table entry size
    phnum:      u16,            // Program header table entry count
    shentsize:  u16,            // Section header table entry size
    shnum:      u16,            // Section header table entry count
    shstrndx:   u16,            // Section header string table index
}


struct ElfInfo {
    hdr:        Elf64Ehdr,
    endian:     Endian,
}

impl ElfInfo {
    // In the example below, the generic type parameters of encastf() and
    // decastf() are omitted because the Rust compiler infers them.

    fn read(bytes: &[u8]) -> Option<Self> {
        let (bits, endian) = ElfIdent::read(bytes)?;
        let hdr = match bits {
            32 => { return None; } // Not supported in this example.
            64 => bytes.encastf(endian)?,
            _ => { return None; }
        };
        Some(Self { hdr, endian })
    }

    fn write(&self, bytes: &mut [u8]) -> Option<usize> {
        Some(bytes.decastf(&self.hdr, self.endian)?)
    }
}


#[test]
fn test() {
    // An example of ELF header
    const DATA_SIZE: usize = 0x40;
    let bytes_input: [u8; DATA_SIZE] = [
        0x7F, 0x45, 0x4C, 0x46, 0x02, 0x01, 0x01, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x03, 0x00, 0x3E, 0x00, 0x01, 0x00, 0x00, 0x00,
        0xD0, 0x67, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xC0, 0x23, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x38, 0x00,
        0x0D, 0x00, 0x40, 0x00, 0x1E, 0x00, 0x1D, 0x00,
    ];

    // Output buffer (initially filled with zeros).
    let mut bytes_output = [0_u8; DATA_SIZE];

    // Read ELF header.
    let elf = ElfInfo::read(&bytes_input).unwrap();

    // Write ELF header.
    let size = elf.write(&mut bytes_output).unwrap();

    // Check the output.
    assert_eq!(size, DATA_SIZE);
    assert_eq!(bytes_output, bytes_input);

    // Check the input.
    assert_eq!(elf.hdr.ident.magic, ELF_MAGIC);         // 0x7F "ELF"
    assert_eq!(elf.hdr.ident.class, 2);                 // ELF 64-bit
    assert_eq!(elf.hdr.ident.data, 1);                  // Little endian
    assert_eq!(elf.hdr.ident.version, 1);               // (should be 1)
    assert_eq!(elf.hdr.ident.osabi, 0);                 // OS ABI (Linux)
    assert_eq!(elf.hdr.ident.abiversion, 0);            // ABI version
    assert_eq!(elf.hdr.ident.pad, [0_u8; 7]);           // Padding
    assert_eq!(elf.hdr.file_type, 3);                   // Shared object file
    assert_eq!(elf.hdr.machine, 0x3E);                  // AMD x86-64
    assert_eq!(elf.hdr.version, 1);                     // (should be 1)
    assert_eq!(elf.hdr.entry, 0x67D0);                  // Entry point
    assert_eq!(elf.hdr.phoff, 0x40);                    // Program header off
    assert_eq!(elf.hdr.shoff, 0x02_23C0);               // Section header off
    assert_eq!(elf.hdr.flags, 0);                       // Flags
    assert_eq!(elf.hdr.ehsize, 0x40);                   // ELF Header size
    assert_eq!(elf.hdr.phentsize, 0x38);                // Program header size
    assert_eq!(elf.hdr.phnum, 0x0D);                    // Program header num
    assert_eq!(elf.hdr.shentsize, 0x40);                // Program header size
    assert_eq!(elf.hdr.shnum, 0x1E);                    // Program header num
    assert_eq!(elf.hdr.shstrndx, 0x1D);                 // Index of sec names
}
