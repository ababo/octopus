use core::mem::size_of;
use core::slice::from_raw_parts;

use super::*;

#[derive(Debug, PartialEq)]
pub enum Error {
    BadMagic,
    UnexpectedEndOfFile,
    BadTotalSize,
    UnalignedReservedMemBlock,
    OverlappingReservedMemBlock,
    NoEmptyReservedMemEntry,
    UnalignedStructBlock,
    OverlappingStructBlock,
    OverlappingStringsBlock,
}

pub type Result<T> = core::result::Result<T, Error>;

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

fn get_header<'a>(blob: &'a [u8]) -> Result<&'a DtbHeader> {
    if blob.len() < 4 {
        return Err(Error::BadMagic);
    }

    let header =
        unsafe { &*(&blob[0] as *const u8 as *const DtbHeader) as &DtbHeader };

    if u32::from_be(header.magic) != DTB_MAGIC {
        return Err(Error::BadMagic);
    }

    if blob.len() < size_of::<DtbHeader>() {
        return Err(Error::UnexpectedEndOfFile);
    }

    if header.totalsize as usize != blob.len() {
        return Err(Error::BadTotalSize);
    }

    Ok(header)
}

fn get_reserved_mem<'a>(
    blob: &'a [u8],
    header: &'a DtbHeader,
) -> Result<&'a [DtbReservedMemEntry]> {
    let reserved_max_size =
        (header.off_dt_struct - header.off_mem_rsvmap) as usize;

    if reserved_max_size % 8 != 0 {
        return Err(Error::UnalignedReservedMemBlock);
    }

    if header.off_mem_rsvmap > header.off_dt_struct {
        return Err(Error::OverlappingReservedMemBlock);
    }

    let reserved = &blob[header.off_mem_rsvmap as usize] as *const u8
        as *const DtbReservedMemEntry;
    let reserved = unsafe {
        from_raw_parts(
            reserved,
            reserved_max_size / size_of::<DtbReservedMemEntry>(),
        )
    };

    let index = reserved.iter().position(|ref e| e.addr == 0 && e.size == 0);
    if index.is_none() {
        return Err(Error::NoEmptyReservedMemEntry);
    }

    Ok(&reserved[..index.unwrap()])
}

fn get_dt_struct<'a>(
    blob: &'a [u8],
    header: &'a DtbHeader,
) -> Result<&'a [u32]> {
    if header.size_dt_struct % 4 != 0 {
        return Err(Error::UnalignedStructBlock);
    }

    if header.off_dt_struct + header.size_dt_struct > header.off_dt_strings {
        return Err(Error::OverlappingStructBlock);
    }

    let offset = header.off_dt_struct as usize;
    let dt_struct = &blob[offset..offset + header.size_dt_struct as usize];
    Ok(unsafe {
        from_raw_parts(
            &dt_struct[0] as *const u8 as *const u32,
            dt_struct.len() / 4,
        )
    })
}

fn get_dt_strings<'a>(
    blob: &'a [u8],
    header: &'a DtbHeader,
) -> Result<&'a [u8]> {
    if header.off_dt_strings + header.size_dt_strings > header.totalsize {
        return Err(Error::OverlappingStringsBlock);
    }

    let offset = header.off_dt_strings as usize;
    Ok(&blob[offset..offset + header.size_dt_strings as usize])
}

fn get_root_node<'a>(
    blob: &'a [u8],
    header: &'a DtbHeader,
) -> Result<DtbNode<'a>> {
    Err(Error::BadMagic)
}

impl<'a> DtbReader<'a> {
    pub fn new(blob: &'a [u8]) -> Result<Self> {
        let header = get_header(blob)?;
        Ok(DtbReader::<'a> {
            reserved_mem: get_reserved_mem(blob, &header)?,
            dt_struct: get_dt_struct(blob, header)?,
            dt_strings: get_dt_strings(blob, header)?,
            root_node: get_root_node(blob, header)?,
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

    #[test]
    fn test_errors() {
        let cases: &[(&'static str, Error)] = &[("bad_magic", Error::BadMagic)];

        let mut buf = Vec::new();
        let path = Path::new(file!()).parent().unwrap().join("tests");
        for (name, err) in cases {
            let filename = path.join(String::from(*name) + ".dtb");
            let mut file = File::open(filename).unwrap();
            file.read_to_end(&mut buf).unwrap();
            assert_eq!(DtbReader::new(buf.as_slice()).unwrap_err(), *err);
        }
    }
}
