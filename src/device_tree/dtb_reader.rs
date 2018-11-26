use core::mem::size_of;
use core::slice::from_raw_parts;
use core::str::{from_utf8, Utf8Error};

use super::*;

#[derive(Debug, PartialEq)]
pub enum Error {
    BadMagic,
    UnexpectedEndOfBlob,
    BadVersion,
    UnsupportedCompVersion,
    BadTotalSize,
    UnalignedReservedMem,
    OverlappingReservedMem,
    NoZeroReservedMemEntry,
    UnalignedStruct,
    OverlappingStruct,
    OverlappingStrings,
    BadNodeName,
    BadStrEncoding(Utf8Error),
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub struct DtbNode<'a> {
    pub name: &'a str,
    offset: usize,
}

#[derive(Debug)]
pub struct DtbReader<'a> {
    reserved_mem: &'a [ReservedMemEntry],
    struct_block: &'a [u8],
    strings_block: &'a [u8],
}

fn get_header<'a>(blob: &'a [u8]) -> Result<DtbHeader> {
    if blob.len() < 4 {
        return Err(Error::BadMagic);
    }

    let be_header =
        unsafe { &*(&blob[0] as *const u8 as *const DtbHeader) as &DtbHeader };

    if u32::from_be(be_header.magic) != DTB_MAGIC {
        return Err(Error::BadMagic);
    }

    if blob.len() < size_of::<DtbHeader>() {
        return Err(Error::UnexpectedEndOfBlob);
    }

    let header = DtbHeader {
        magic: DTB_MAGIC,
        total_size: u32::from_be(be_header.total_size),
        struct_offset: u32::from_be(be_header.struct_offset),
        strings_offset: u32::from_be(be_header.strings_offset),
        reserved_mem_offset: u32::from_be(be_header.reserved_mem_offset),
        version: u32::from_be(be_header.version),
        last_comp_version: u32::from_be(be_header.last_comp_version),
        bsp_cpu_id: u32::from_be(be_header.bsp_cpu_id),
        strings_size: u32::from_be(be_header.strings_size),
        struct_size: u32::from_be(be_header.struct_size),
    };

    if header.version < header.last_comp_version {
        return Err(Error::BadVersion);
    }

    if header.last_comp_version != DTB_COMP_VERSION {
        return Err(Error::UnsupportedCompVersion);
    }

    if header.total_size as usize != blob.len() {
        return Err(Error::BadTotalSize);
    }

    Ok(header)
}

fn get_reserved_mem<'a>(
    blob: &'a [u8],
    header: &DtbHeader,
) -> Result<&'a [ReservedMemEntry]> {
    let entry_size = size_of::<ReservedMemEntry>();
    if header.reserved_mem_offset + entry_size as u32 > header.struct_offset {
        return Err(Error::OverlappingReservedMem);
    }

    if header.reserved_mem_offset % 8 != 0 {
        return Err(Error::UnalignedReservedMem);
    }

    let reserved_max_size =
        (header.struct_offset - header.reserved_mem_offset) as usize;
    let reserved = unsafe {
        let ptr = blob.as_ptr().offset(header.reserved_mem_offset as isize)
            as *const ReservedMemEntry;
        from_raw_parts(ptr, reserved_max_size / entry_size)
    };

    let index = reserved.iter().position(|ref e| e.addr == 0 && e.size == 0);
    if index.is_none() {
        return Err(Error::NoZeroReservedMemEntry);
    }

    Ok(&reserved[..index.unwrap()])
}

fn get_struct_block<'a>(
    blob: &'a [u8],
    header: &DtbHeader,
) -> Result<&'a [u8]> {
    if header.struct_offset % 4 != 0 || header.struct_size % 4 != 0 {
        return Err(Error::UnalignedStruct);
    }

    if header.struct_offset + header.struct_size > header.strings_offset {
        return Err(Error::OverlappingStruct);
    }

    let offset = header.struct_offset as usize;
    Ok(&blob[offset..offset + header.struct_size as usize])
}

fn get_strings_block<'a>(
    blob: &'a [u8],
    header: &DtbHeader,
) -> Result<&'a [u8]> {
    if header.strings_offset + header.strings_size > header.total_size {
        return Err(Error::OverlappingStrings);
    }

    let offset = header.strings_offset as usize;
    Ok(&blob[offset..offset + header.strings_size as usize])
}

fn read_node<'a>(struct_block: &'a [u8], offset: usize) -> Result<DtbNode<'a>> {
    for (i, chr) in (&struct_block[offset..]).iter().enumerate() {
        if *chr == 0 {
            return match from_utf8(&struct_block[offset..i]) {
                Ok(name) => Ok(DtbNode::<'a> {
                    name: name,
                    offset: ((i + 4 - 1) / 4) * 4,
                }),
                Err(err) => Err(Error::BadStrEncoding(err)),
            };
        }
    }
    Err(Error::BadNodeName)
}

impl<'a> DtbReader<'a> {
    pub fn new(blob: &'a [u8]) -> Result<Self> {
        let header = get_header(blob)?;
        Ok(DtbReader::<'a> {
            reserved_mem: get_reserved_mem(blob, &header)?,
            struct_block: get_struct_block(blob, &header)?,
            strings_block: get_strings_block(blob, &header)?,
        })
    }

    pub fn find_node(parent: &DtbNode<'a>, path: &str) -> Result<DtbNode<'a>> {
        Err(Error::BadMagic)
    }

    pub fn find_str_prop(parent: &DtbNode<'a>, path: &str) -> Result<&'a str> {
        Err(Error::BadMagic)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    fn new_reader<'a>(
        buf: &'a mut Vec<u8>,
        name: &str,
    ) -> Result<DtbReader<'a>> {
        let path = Path::new(file!()).parent().unwrap().join("test_dtb");
        let filename = path.join(String::from(name) + ".dtb");
        let mut file = File::open(filename).unwrap();
        buf.resize(0, 0);
        file.read_to_end(buf).unwrap();
        DtbReader::new(buf.as_slice())
    }

    macro_rules! test_error {
        ($fn_name:ident, $err:ident) => {
            #[test]
            fn $fn_name() {
                let mut buf = Vec::new();
                let reader = new_reader(&mut buf, &stringify!($fn_name)[5..]);
                assert_eq!(reader.unwrap_err(), Error::$err);
            }
        };
    }

    test_error!(test_bad_magic, BadMagic);
    test_error!(test_unexpected_end_of_blob, UnexpectedEndOfBlob);
    test_error!(test_bad_version, BadVersion);
    test_error!(test_unsupported_comp_version, UnsupportedCompVersion);
    test_error!(test_bad_total_size, BadTotalSize);
    test_error!(test_unaligned_reserved_mem, UnalignedReservedMem);
    test_error!(test_overlapping_reserved_mem, OverlappingReservedMem);
    test_error!(test_no_zero_reserved_mem_entry, NoZeroReservedMemEntry);
    test_error!(test_unaligned_struct, UnalignedStruct);
    test_error!(test_unaligned_struct2, UnalignedStruct);
    test_error!(test_overlapping_struct, OverlappingStruct);
    test_error!(test_overlapping_strings, OverlappingStrings);
}
