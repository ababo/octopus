// The magic field should contain this.
const HEADER_MAGIC: u32 = 0x1BADB002;

// Must pass memory information to OS.
const HEADER_MEMORY_INFO: u32 = 0x00000002;

#[repr(C)]
struct Header {
    magic: u32,    // Must be equal to HEADER_MAGIC.
    flags: u32,    // Feature flags.
    checksum: u32, // The above fields plus this one must equal 0 mod 2^32.

    // These are only valid if AOUT_KLUDGE is set.
    header_addr: u32,
    load_addr: u32,
    load_end_addr: u32,
    bss_end_addr: u32,
    entry_addr: u32,

    // These are only valid if VIDEO_MODE is set.
    mode_type: u32,
    width: u32,
    height: u32,
    depth: u32,
}

const HEADER_FLAGS: u32 = HEADER_MEMORY_INFO;

#[linkage = "external"]
#[link_section = ".header"]
static MULTIBOOT_HEADER: Header = Header {
    magic: HEADER_MAGIC,
    flags: HEADER_FLAGS,
    checksum: (-((HEADER_MAGIC + HEADER_FLAGS) as i32) as u32),
    header_addr: 0,
    load_addr: 0,
    load_end_addr: 0, //
    bss_end_addr: 0,
    entry_addr: 0,
    mode_type: 0,
    width: 0,
    height: 0,
    depth: 0,
};
