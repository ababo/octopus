const DTB_MAGIC: u32 = 0xD00DFEED;

const DTB_BEGIN_NODE: u32 = 1;
const DTB_END_NODE: u32 = 2;
const DTB_PROP: u32 = 3;
const DTB_NOP: u32 = 4;
const DTB_END: u32 = 9;

struct DtbHeader {
    pub magic: u32,
    pub totalsize: u32,
    pub off_dt_struct: u32,
    pub off_dt_strings: u32,
    pub off_mem_rsvmap: u32,
    pub version: u32,
    pub last_comp_version: u32,
    pub boot_cpuid_phys: u32,
    pub size_dt_strings: u32,
    pub size_dt_struct: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct DtbReservedMemEntry {
    pub addr: u64,
    pub size: u64,
}

pub mod dtb_reader;
