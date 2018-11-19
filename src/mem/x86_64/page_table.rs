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
