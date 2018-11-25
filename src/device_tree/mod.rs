const DTB_MAGIC: u32 = 0xD00DFEED;
const DTB_COMP_VERSION: u32 = 16;

const DTB_BEGIN_NODE: u32 = 1;
const DTB_END_NODE: u32 = 2;
const DTB_PROP: u32 = 3;
const DTB_NOP: u32 = 4;
const DTB_END: u32 = 9;

#[repr(C)]
struct DtbHeader {
    pub magic: u32,
    pub total_size: u32,
    pub struct_offset: u32,
    pub strings_offset: u32,
    pub reserved_mem_offset: u32,
    pub version: u32,
    pub last_comp_version: u32,
    pub bsp_cpu_id: u32,
    pub strings_size: u32,
    pub struct_size: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct ReservedMemEntry {
    pub addr: u64,
    pub size: u64,
}

pub mod dtb_reader;
