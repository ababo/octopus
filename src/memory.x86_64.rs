static GDT_TYPE_DATA: u64 = 0x2 << 40;
static GDT_TYPE_CODE: u64 = 0xA << 40;
static GDT_NONSYS: u64 = 1 << 44;
static GDT_PRESENT: u64 = 1 << 47;
static GDT_BITS64: u64 = 1 << 53;
static GDT_BITS32: u64 = 1 << 54;

static GDT: [u64; 3] = [
    0,
    GDT_TYPE_CODE | GDT_NONSYS | GDT_PRESENT | GDT_BITS64,
    GDT_TYPE_DATA | GDT_NONSYS | GDT_PRESENT | GDT_BITS32,
];

#[repr(packed)]
struct GdtDesc {
    #[allow(dead_code)]
    limit: u16,
    #[allow(dead_code)]
    offset: *const u64, // 64-bit pointer instead of u32 to keep it simple.
}

unsafe impl Sync for GdtDesc {}

// To be passed to lgdt instruction.
#[export_name = "_gdt_desc"]
#[linkage = "external"]
static GDT_DESC: GdtDesc = GdtDesc {
    limit: (GDT.len() * 8 - 1) as u16,
    offset: &GDT as *const u64,
};

const PAGE_TABLE_NUM_ENTRIES: usize = 512;

#[repr(align(4096))]
struct PageTable {
    #[allow(dead_code)]
    entries: [u64; PAGE_TABLE_NUM_ENTRIES],
}

#[export_name = "_pml4"]
#[linkage = "external"]
static mut PML4: PageTable = PageTable {
    entries: [0; PAGE_TABLE_NUM_ENTRIES],
};

#[export_name = "_pdp"]
#[linkage = "external"]
static mut PDP: PageTable = PageTable {
    entries: [0; PAGE_TABLE_NUM_ENTRIES],
};

#[export_name = "_pd"]
#[linkage = "external"]
static mut PD: PageTable = PageTable {
    entries: [0; PAGE_TABLE_NUM_ENTRIES],
};

const BSP_STACK_SIZE: usize = 64 * 1024;

#[export_name = "_bsp_stack_size"]
#[linkage = "external"]
#[repr(align(16))]
struct BspStack {
    #[allow(dead_code)]
    body: [u8; BSP_STACK_SIZE],
}

#[export_name = "_bsp_stack"]
#[linkage = "external"]
static mut BSP_STACK: BspStack = BspStack {
    body: [0; BSP_STACK_SIZE],
};

#[export_name = "_bsp_stack_size"]
#[linkage = "external"]
static BSP_STACK_SIZE2: usize = BSP_STACK_SIZE;
