#![allow(dead_code)]

/// The multiboot header magic value.
pub const HEADER_MAGIC: u32 = 0x1BADB002;

/// Instructs cootloader to provide memory info.
pub const HEADER_MEMORY_INFO: u32 = 0x2;

/// A multiboot header.
#[repr(C)]
pub struct Header {
    pub magic: u32,    // Must be equal to HEADER_MAGIC.
    pub flags: u32,    // Feature flags.
    pub checksum: u32, // The above fields plus this one must equal 0 mod 2^32.

    // These are only valid if AOUT_KLUDGE is set.
    pub header_addr: u32,
    pub load_addr: u32,
    pub load_end_addr: u32,
    pub bss_end_addr: u32,
    pub entry_addr: u32,

    // These are only valid if VIDEO_MODE is set.
    pub mode_type: u32,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

/// A multiboot bootloader magic (must be stored in %eax).
pub const BOOTLOADER_MAGIC: u32 = 0x2BADB002;

/// A symbol table for a.out.
#[repr(C)]
pub struct AoutSymbolTable {
    pub tab_size: u32,
    pub str_size: u32,
    pub addr: u32,
    pub reserved: u32,
}

/// A section header table for ELF.
#[repr(C)]
pub struct ElfSectionHeaderTable {
    pub num: u32,
    pub size: u32,
    pub addr: u32,
    pub shndx: u32,
}

/// A binary content table.
#[repr(C)]
pub union BinaryContentTable {
    aout_sym: AoutSymbolTable,
    elf_sec: ElfSectionHeaderTable,
}

/// A multiboot info.
#[repr(C)]
pub struct Info {
    // Multiboot info version number.
    pub flags: u32,

    // Available memory from BIOS.
    pub mem_lower: u32,
    pub mem_upper: u32,

    // "root" partition.
    pub boot_device: u32,

    // Kernel command line.
    pub cmdline: u32,

    // Boot-Module list.
    pub mods_count: u32,
    pub mods_addr: u32,

    // Binary content.
    pub content: BinaryContentTable,

    // Memory Mapping buffer.
    pub mmap_length: u32,
    pub mmap_addr: u32,

    // Drive Info buffe.
    pub drives_length: u32,
    pub drives_addr: u32,

    // ROM configuration table.
    pub config_table: u32,

    // Boot Loader Name.
    pub boot_loader_name: u32,

    // APM table.
    pub apm_table: u32,

    // Video.
    pub vbe_control_info: u32,
    pub vbe_mode_info: u32,
    pub vbe_mode: u16,
    pub vbe_interface_seg: u16,
    pub vbe_interface_off: u16,
    pub vbe_interface_len: u16,
}
