// text
use crate::reader::driver::*;
use crate::writer::driver::*;
use md5::Digest;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct ThreadedCommentText {
    value: Box<str>,
}

impl ThreadedCommentText {
    #[inline]
    pub(crate) fn get_value(&self) -> &str {
        &self.value
    }

    #[inline]
    pub(crate) fn set_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.value = value.into().into_boxed_str();
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
        // text
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        write_start_tag(writer, "text", attributes, false);
        write_text_node(writer, &*self.value);
        write_end_tag(writer, "text");
    }
}
