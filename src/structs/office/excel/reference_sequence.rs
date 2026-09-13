// xm:sqref
use crate::reader::driver::*;
use crate::structs::Coordinate;
use crate::structs::Range;
use crate::writer::driver::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use std::vec;
use thin_vec::ThinVec;

#[derive(Default, Debug, Clone)]
pub struct ReferenceSequence {
    value: ThinVec<Range>,
}
impl ReferenceSequence {
    #[inline]
    pub fn get_value(&self) -> &[Range] {
        &self.value
    }

    #[inline]
    pub fn get_value_mut(&mut self) -> &mut ThinVec<Range> {
        &mut self.value
    }

    #[inline]
    pub fn set_value(&mut self, value: impl Into<ThinVec<Range>>) -> &mut Self {
        self.value = value.into();
        self
    }

    #[inline]
    pub fn add_value(&mut self, value: Range) -> &mut Self {
        self.value.push(value);
        self
    }

    #[inline]
    pub fn remove_value(&mut self) -> &mut Self {
        self.value.clear();
        self
    }

    pub fn set_sqref<S: Into<String>>(&mut self, value: S) -> &mut Self {
        value.into().split(' ').for_each(|range_value| {
            let mut range = Range::default();
            range.set_range(range_value);
            self.value.push(range);
        });
        self
    }

    #[inline]
    pub fn get_sqref(&self) -> String {
        self.value
            .iter()
            .map(|range| range.get_range())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        let text = reader.read_text_into(e.name(), &mut buf).unwrap();
        self.set_sqref(crate::helper::utils::unescape_xml_text(&text));
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        write_start_tag(writer, "xm:sqref", vec![], false);
        write_text_node(writer, &self.get_sqref());
        write_end_tag(writer, "xm:sqref");
    }
}
