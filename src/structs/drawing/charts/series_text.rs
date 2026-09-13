// c:tx
use crate::reader::driver::*;
use crate::structs::StringValue;
use crate::writer::driver::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct SeriesText {
    value: StringValue,
}

impl SeriesText {
    pub fn get_value(&self) -> &str {
        self.value.get_value_str()
    }

    pub fn set_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.value.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        let text = reader.read_text_into(e.name(), &mut buf).unwrap();
        self.set_value(crate::helper::utils::unescape_xml_text(&text));
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:tx
        write_start_tag(writer, "c:tx", vec![], false);

        // c:v
        write_start_tag(writer, "c:v", vec![], false);
        write_text_node(writer, self.value.get_value_str());
        write_end_tag(writer, "c:v");

        write_end_tag(writer, "c:tx");
    }
}
