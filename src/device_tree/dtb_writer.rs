use super::common::*;
use super::dtb_format::*;
use super::struct_item::*;

pub struct DtbReservedMem<'a> {
    buf: &'a [u8],
    offset: usize,
}

impl<'a> DtbReservedMem<'a> {
    pub fn new(buf: &'a mut [u8]) -> Result<DtbReservedMem<'a>> {
        Err(Error::BadMagic)
    }

    pub fn add_entry(&mut self, entry: &ReservedMemEntry) -> Result<()> {
        Err(Error::BadMagic)
    }
}

pub struct DtbWriter<'a> {
    buf: &'a [u8],
}

impl<'a> DtbWriter<'a> {
    pub fn new(buf: &'a mut [u8]) -> Result<DtbWriter<'a>> {
        Err(Error::BadMagic)
    }

    pub fn with_reserved_mem(
        reserved_mem: DtbReservedMem<'a>,
    ) -> Result<DtbWriter<'a>> {
        Err(Error::BadMagic)
    }

    pub fn add_item<'b>(&mut self, item: &StructItem<'b>) -> Result<()> {
        Err(Error::BadMagic)
    }

    pub fn build_blob(writer: DtbWriter<'a>) -> Result<&'a [u8]> {
        Err(Error::BadMagic)
    }
}
