// xm:f
use crate::reader::driver::*;
use crate::structs::Address;
use crate::writer::driver::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use std::vec;

#[derive(Default, Debug, Clone)]
pub struct Formula {
    value: Address,
}
impl Formula {
    #[inline]
    pub fn get_value(&self) -> &Address {
        &self.value
    }

    #[inline]
    pub fn get_value_mut(&mut self) -> &mut Address {
        &mut self.value
    }

    #[inline]
    pub fn set_value(&mut self, value: Address) -> &mut Self {
        self.value = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        let text = reader.read_text_into(e.name(), &mut buf).unwrap();
        let mut obj = Address::default();
        obj.set_address(crate::helper::utils::unescape_xml_text(&text));
        self.value = obj;
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        write_start_tag(writer, "xm:f", vec![], false);
        write_text_node(writer, &self.value.get_address());
        write_end_tag(writer, "xm:f");
    }
}
