use core::mem::size_of;
use core::slice::from_raw_parts;
use core::str::{from_utf8, Utf8Error};

use super::*;

#[derive(Debug, PartialEq)]
pub enum Error {
    BadMagic,
    BadNodeName,
    BadPropertyName,
    BadStrEncoding(Utf8Error),
    BadStructItemType,
    BadStructToken,
    BadTotalSize,
    BadU32List,
    BadVersion,
    BufferTooSmall,
    NoMoreStructItems,
    NoZeroReservedMemEntry,
    OverlappingReservedMem,
    OverlappingStrings,
    OverlappingStruct,
    UnalignedReservedMem,
    UnalignedStruct,
    UnexpectedEndOfBlob,
    UnexpectedEndOfStruct,
    UnsupportedCompVersion,
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum DtbStructItem<'a> {
    BeginNode { name: &'a str },
    Property { name: &'a str, value: &'a [u8] },
    EndNode,
}

impl<'a> DtbStructItem<'a> {
    pub fn is_node(&self) -> bool {
        match self {
            DtbStructItem::BeginNode { name: _ } => true,
            _ => false,
        }
    }

    pub fn is_property(&self) -> bool {
        match self {
            DtbStructItem::Property { name: _, value: _ } => true,
            _ => false,
        }
    }

    pub fn name(&self) -> Result<&'a str> {
        match self {
            DtbStructItem::BeginNode { name } => Ok(name),
            DtbStructItem::Property { name, value: _ } => Ok(name),
            _ => Err(Error::BadStructItemType),
        }
    }

    pub fn value(&self) -> Result<&'a [u8]> {
        match self {
            DtbStructItem::Property { name: _, value } => Ok(value),
            _ => Err(Error::BadStructItemType),
        }
    }

    pub fn value_str(&self) -> Result<&'a str> {
        let value = self.value()?;
        match from_utf8(&value[..value.len() - 1]) {
            Ok(value_str) => Ok(value_str),
            Err(err) => Err(Error::BadStrEncoding(err)),
        }
    }

    pub fn value_str_list<'b>(
        &self,
        buf: &'b mut [&'a str],
    ) -> Result<&'b [&'a str]> {
        let mut i = 0;
        for part in self.value_str()?.split("\0") {
            if i >= buf.len() {
                return Err(Error::BufferTooSmall);
            }
            buf[i] = part;
            i += 1;
        }
        Ok(&buf[..i])
    }

    pub fn value_u32_list<'b>(&self, buf: &'b mut [u32]) -> Result<&'b [u32]> {
        let value = self.value()?;

        if value.len() % 4 != 0 {
            return Err(Error::BadU32List);
        }

        let len = value.len() / 4;
        if buf.len() < len {
            return Err(Error::BufferTooSmall);
        }

        for i in 0..len {
            buf[i] = u32::from_be(unsafe {
                *(value.as_ptr().offset(4 * i as isize) as *const u32)
            });
        }

        Ok(&buf[..len])
    }
}

pub struct DtbStructIterator<'a> {
    struct_block: &'a [u8],
    strings_block: &'a [u8],
    offset: usize,
}

const DTB_TOKEN_SIZE: usize = 4;

impl<'a> DtbStructIterator<'a> {
    fn set_offset(&mut self, offset: usize) {
        self.offset =
            ((offset + DTB_TOKEN_SIZE - 1) / DTB_TOKEN_SIZE) * DTB_TOKEN_SIZE;
    }

    fn read_node(&mut self) -> Result<DtbStructItem<'a>> {
        let offset = self.offset + DTB_TOKEN_SIZE;
        for (i, chr) in (&self.struct_block[offset..]).iter().enumerate() {
            if *chr != 0 {
                continue;
            }
            return match from_utf8(&self.struct_block[offset..offset + i]) {
                Ok(name) => {
                    self.set_offset(offset + i + 1);
                    Ok(DtbStructItem::BeginNode { name: name })
                }
                Err(err) => Err(Error::BadStrEncoding(err)),
            };
        }
        Err(Error::BadNodeName)
    }

    fn assert_enough_struct(&self, offset: usize, size: usize) -> Result<()> {
        if self.struct_block.len() < offset + size {
            Err(Error::UnexpectedEndOfStruct)
        } else {
            Ok(())
        }
    }

    fn read_property(&mut self) -> Result<DtbStructItem<'a>> {
        let mut offset = self.offset + DTB_TOKEN_SIZE;
        let desc_size = size_of::<DtbPropertyDesc>();
        self.assert_enough_struct(offset, desc_size)?;

        let desc_be = unsafe {
            &*((&self.struct_block[offset..]).as_ptr()
                as *const DtbPropertyDesc) as &DtbPropertyDesc
        };
        offset += desc_size;

        let value_size = u32::from_be(desc_be.value_size) as usize;
        self.assert_enough_struct(offset, value_size)?;
        let value = &self.struct_block[offset..offset + value_size];
        offset += value_size;

        let name_offset = u32::from_be(desc_be.name_offset) as usize;
        for (i, chr) in (&self.strings_block[name_offset..]).iter().enumerate()
        {
            if *chr != 0 {
                continue;
            }
            return match from_utf8(
                &self.strings_block[name_offset..name_offset + i],
            ) {
                Ok(name) => {
                    self.set_offset(offset);
                    Ok(DtbStructItem::Property {
                        name: name,
                        value: value,
                    })
                }
                Err(err) => Err(Error::BadStrEncoding(err)),
            };
        }

        Err(Error::BadPropertyName)
    }

    pub fn next(&mut self) -> Result<DtbStructItem<'a>> {
        loop {
            self.assert_enough_struct(self.offset, DTB_TOKEN_SIZE)?;

            let token = u32::from_be(unsafe {
                *((&self.struct_block[self.offset..]).as_ptr() as *const u32)
            });

            if token == DTB_NOP {
                self.offset += DTB_TOKEN_SIZE;
                continue;
            }

            return match token {
                DTB_BEGIN_NODE => self.read_node(),
                DTB_PROPERTY => self.read_property(),
                DTB_END_NODE => {
                    self.offset += DTB_TOKEN_SIZE;
                    Ok(DtbStructItem::EndNode)
                }
                DTB_END => Err(Error::NoMoreStructItems),
                _ => Err(Error::BadStructToken),
            };
        }
    }
}

impl<'a> Iterator for DtbStructIterator<'a> {
    type Item = DtbStructItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next() {
            Ok(item) => Some(item),
            Err(_) => None,
        }
    }
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
        unsafe { &*(blob.as_ptr() as *const DtbHeader) as &DtbHeader };

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

    let index = reserved
        .iter()
        .position(|ref e| e.address == 0 && e.size == 0);
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

impl<'a> DtbReader<'a> {
    pub fn new(blob: &'a [u8]) -> Result<Self> {
        let header = get_header(blob)?;
        Ok(DtbReader::<'a> {
            reserved_mem: get_reserved_mem(blob, &header)?,
            struct_block: get_struct_block(blob, &header)?,
            strings_block: get_strings_block(blob, &header)?,
        })
    }

    pub fn struct_iter(&self) -> DtbStructIterator<'a> {
        DtbStructIterator::<'a> {
            struct_block: self.struct_block,
            strings_block: self.strings_block,
            offset: 0,
        }
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

    fn assert_node<'a>(iter: &mut DtbStructIterator<'a>, name: &str) {
        let item = iter.next().unwrap();
        assert!(item.is_node());
        assert_eq!(item.name().unwrap(), name);
    }

    fn assert_str_property<'a>(
        iter: &mut DtbStructIterator<'a>,
        name: &str,
        value: &str,
    ) {
        let item = iter.next().unwrap();
        assert!(item.is_property());
        assert_eq!(item.name().unwrap(), name);
        assert_eq!(item.value_str().unwrap(), value);
    }

    fn assert_str_list_property<'a>(
        iter: &mut DtbStructIterator<'a>,
        name: &str,
        value: &[&str],
    ) {
        let item = iter.next().unwrap();
        assert!(item.is_property());
        assert_eq!(item.name().unwrap(), name);
        let mut buf = [""; 8];
        assert_eq!(item.value_str_list(&mut buf).unwrap(), value);
    }

    fn assert_u32_list_property<'a>(
        iter: &mut DtbStructIterator<'a>,
        name: &str,
        value: &[u32],
    ) {
        let item = iter.next().unwrap();
        assert!(item.is_property());
        assert_eq!(item.name().unwrap(), name);
        let mut buf = [0u32; 8];
        assert_eq!(item.value_u32_list(&mut buf).unwrap(), value);
    }

    #[test]
    fn test_struct_enum() {
        let mut buf = Vec::new();
        let mut iter = new_reader(&mut buf, "sample").unwrap().struct_iter();
        assert_node(&mut iter, "");
        assert_node(&mut iter, "node1");
        assert_str_property(&mut iter, "a-string-property", "A string");
        assert_str_list_property(
            &mut iter,
            "a-string-list-property",
            &["first string", "second string"],
        );
        assert_eq!(
            iter.next().unwrap(),
            DtbStructItem::Property {
                name: "a-byte-data-property",
                value: &[0x01, 0x23, 0x34, 0x56],
            }
        );
        assert_node(&mut iter, "child-node1");
        assert_eq!(
            iter.next().unwrap(),
            DtbStructItem::Property {
                name: "first-child-property",
                value: &[],
            }
        );
        assert_u32_list_property(&mut iter, "second-child-property", &[1]);
        assert_str_property(&mut iter, "a-string-property", "Hello, world");
        assert_eq!(iter.next().unwrap(), DtbStructItem::EndNode);
        assert_node(&mut iter, "child-node2");
        assert_eq!(iter.next().unwrap(), DtbStructItem::EndNode);
        assert_eq!(iter.next().unwrap(), DtbStructItem::EndNode);
        assert_node(&mut iter, "node2");
        assert_eq!(
            iter.next().unwrap(),
            DtbStructItem::Property {
                name: "an-empty-property",
                value: &[],
            }
        );
        assert_u32_list_property(&mut iter, "a-cell-property", &[1, 2, 3, 4]);
        assert_node(&mut iter, "child-node1");
        assert_eq!(iter.next().unwrap(), DtbStructItem::EndNode);
        assert_eq!(iter.next().unwrap(), DtbStructItem::EndNode);
        assert_eq!(iter.next().unwrap(), DtbStructItem::EndNode);
        assert_eq!(iter.next().unwrap_err(), Error::NoMoreStructItems);
        assert_eq!(iter.next().unwrap_err(), Error::NoMoreStructItems);
    }
}
