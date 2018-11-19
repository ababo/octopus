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
