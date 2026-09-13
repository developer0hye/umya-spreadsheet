use crate::helper::coordinate::*;
use crate::reader::driver::*;
use crate::structs::UInt32Value;
use crate::traits::AdjustmentValue;
use crate::writer::driver::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct CommentColumnTarget {
    value: UInt32Value,
}

impl CommentColumnTarget {
    #[inline]
    pub fn get_value(&self) -> &u32 {
        self.value.get_value()
    }

    #[inline]
    pub fn set_value(&mut self, value: u32) -> &mut Self {
        self.value.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        let text = reader.read_text_into(e.name(), &mut buf).unwrap();
        self.value
            .set_value_string(crate::helper::utils::unescape_xml_text(&text));
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // x:Column
        write_start_tag(writer, "x:Column", vec![], false);
        write_text_node(writer, self.value.get_value_string());
        write_end_tag(writer, "x:Column");
    }
}
impl AdjustmentValue for CommentColumnTarget {
    #[inline]
    fn adjustment_insert_value(&mut self, root_num: &u32, offset_num: &u32) {
        self.value.set_value(
            adjustment_insert_coordinate(&(self.value.get_value() + &1), root_num, offset_num) - 1,
        );
    }

    #[inline]
    fn adjustment_remove_value(&mut self, root_num: &u32, offset_num: &u32) {
        self.value.set_value(
            adjustment_remove_coordinate(&(self.value.get_value() + &1), root_num, offset_num) - 1,
        );
    }

    #[inline]
    fn is_remove_value(&self, root_num: &u32, offset_num: &u32) -> bool {
        is_remove_coordinate(&(self.value.get_value() + 1), root_num, offset_num)
    }
}
