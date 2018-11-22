#[derive(Debug, PartialEq)]
pub enum Error {
    UnknownFormat,
    MalformedData,
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
#[repr(C)]
pub struct DtbReservedMemEntry {
    pub addr: u64,
    pub size: u64,
}

#[derive(Debug)]
pub struct DtbNode<'a> {
    pub name: &'a str,
    pub addr: usize,
    offset: usize,
}

#[derive(Debug)]
pub struct DtbReader<'a> {
    pub reserved_mem: &'a [DtbReservedMemEntry],
    pub root_node: DtbNode<'a>,
    dt_struct: &'a [u32],
    dt_strings: &'a [u8],
}

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

impl<'a> DtbReader<'a> {
    pub fn new(blob: &'a [u8]) -> Result<Self> {
        if blob.len() < core::mem::size_of::<DtbHeader>() {
            return Err(Error::UnknownFormat);
        }

        let header = unsafe {
            &*(&blob[0] as *const u8 as *const DtbHeader) as &DtbHeader
        };

        if u32::from_be(header.magic) != DTB_MAGIC {
            return Err(Error::UnknownFormat);
        }

        if header.totalsize as usize != blob.len() {
            return Err(Error::MalformedData);
        }

        let reserved_max_size =
            (header.off_dt_struct - header.off_mem_rsvmap) as usize;
        let reserved_unaligned = reserved_max_size % 8 != 0;
        let reserved_overlaps = header.off_mem_rsvmap > header.off_dt_struct;
        let struct_overlaps = header.off_dt_struct + header.size_dt_struct
            > header.off_dt_strings;
        let strings_overlaps =
            header.off_dt_strings + header.size_dt_strings > header.totalsize;
        if reserved_unaligned
            || reserved_overlaps
            || struct_overlaps
            || strings_overlaps
        {
            return Err(Error::MalformedData);
        }

        let reserved = &blob[header.off_mem_rsvmap as usize] as *const u8
            as *const DtbReservedMemEntry;
        let reserved = unsafe {
            core::slice::from_raw_parts(
                reserved,
                reserved_max_size / core::mem::size_of::<DtbReservedMemEntry>(),
            )
        };

        let index =
            reserved.iter().position(|ref e| e.addr == 0 && e.size == 0);
        if index.is_none() {
            return Err(Error::MalformedData);
        }
        let reserved = &reserved[..index.unwrap()];

        Err(Error::UnknownFormat)
    }

    pub fn find_node(parent: &DtbNode<'a>, path: &str) -> Result<DtbNode<'a>> {
        Err(Error::UnknownFormat)
    }

    pub fn find_str_prop(parent: &DtbNode<'a>, path: &str) -> Result<&'a str> {
        Err(Error::UnknownFormat)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY_HEADER: &[u8] = &[
        0xD0, 0x0D, 0xFE, 0xED, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    #[test]
    fn test_unknown_format() {
        let reader = DtbReader::new(&EMPTY_HEADER[..4]);
        assert_eq!(reader.unwrap_err(), Error::UnknownFormat);

        let reader = DtbReader::new(&EMPTY_HEADER[..39]);
        assert_eq!(reader.unwrap_err(), Error::UnknownFormat);
    }

    #[test]
    fn test_malformed_data() {
        let reader = DtbReader::new(&EMPTY_HEADER[..40]);
        assert_eq!(reader.unwrap_err(), Error::MalformedData);
    }
}
