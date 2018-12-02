use core::str::Utf8Error;

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
    BadValueStr,
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

#[derive(Debug)]
#[repr(C)]
pub struct ReservedMemEntry {
    pub address: u64,
    pub size: u64,
}
