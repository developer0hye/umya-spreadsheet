use super::BooleanValue;
use super::Color;
use super::ConditionalFormatValueObject;
use super::UInt32Value;
use crate::reader::driver::*;
use crate::writer::driver::*;
use quick_xml::events::BytesStart;
use quick_xml::events::Event;
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use thin_vec::ThinVec;

#[derive(Clone, Default, Debug)]
pub struct DataBar {
    min_length: UInt32Value,
    max_length: UInt32Value,
    show_value: BooleanValue,
    cfvo_collection: ThinVec<ConditionalFormatValueObject>,
    color_collection: ThinVec<Color>,
}

impl DataBar {
    /// minLength attribute; the OOXML default is 10 (percent of cell width).
    #[inline]
    pub fn get_min_length(&self) -> u32 {
        if self.min_length.has_value() {
            *self.min_length.get_value()
        } else {
            10
        }
    }

    #[inline]
    pub fn set_min_length(&mut self, value: u32) -> &mut Self {
        self.min_length.set_value(value);
        self
    }

    /// maxLength attribute; the OOXML default is 90 (percent of cell width).
    #[inline]
    pub fn get_max_length(&self) -> u32 {
        if self.max_length.has_value() {
            *self.max_length.get_value()
        } else {
            90
        }
    }

    #[inline]
    pub fn set_max_length(&mut self, value: u32) -> &mut Self {
        self.max_length.set_value(value);
        self
    }

    /// showValue attribute; the OOXML default is true.
    #[inline]
    pub fn get_show_value(&self) -> bool {
        if self.show_value.has_value() {
            *self.show_value.get_value()
        } else {
            true
        }
    }

    #[inline]
    pub fn set_show_value(&mut self, value: bool) -> &mut Self {
        self.show_value.set_value(value);
        self
    }

    #[inline]
    pub fn get_cfvo_collection(&self) -> &[ConditionalFormatValueObject] {
        &self.cfvo_collection
    }

    #[inline]
    pub fn set_cfvo_collection(
        &mut self,
        value: ThinVec<ConditionalFormatValueObject>,
    ) -> &mut Self {
        self.cfvo_collection = value;
        self
    }

    #[inline]
    pub fn add_cfvo_collection(&mut self, value: ConditionalFormatValueObject) -> &mut Self {
        self.cfvo_collection.push(value);
        self
    }

    #[inline]
    pub fn get_color_collection(&self) -> &[Color] {
        &self.color_collection
    }

    #[inline]
    pub fn set_color_collection(&mut self, value: impl Into<ThinVec<Color>>) -> &mut Self {
        self.color_collection = value.into();
        self
    }

    #[inline]
    pub fn add_color_collection(&mut self, value: Color) -> &mut Self {
        self.color_collection.push(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, min_length, "minLength");
        set_string_from_xml!(self, e, max_length, "maxLength");
        set_string_from_xml!(self, e, show_value, "showValue");

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                match e.name().into_inner() {
                    b"cfvo" => {
                        let mut obj = ConditionalFormatValueObject::default();
                        obj.set_attributes(reader, e, true);
                        self.cfvo_collection.push(obj);
                    }
                    b"color" => {
                        let mut obj = Color::default();
                        obj.set_attributes(reader, e, true);
                        self.color_collection.push(obj);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"dataBar" {
                    return
                }
            },
            Event::Eof => return
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // dataBar
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let min_length = self.min_length.get_value_string();
        if self.min_length.has_value() {
            attributes.push(("minLength", &min_length));
        }
        let max_length = self.max_length.get_value_string();
        if self.max_length.has_value() {
            attributes.push(("maxLength", &max_length));
        }
        let show_value = self.show_value.get_value_string();
        if self.show_value.has_value() {
            attributes.push(("showValue", show_value));
        }
        write_start_tag(writer, "dataBar", attributes, false);

        // cfvo
        for v in &self.cfvo_collection {
            v.write_to(writer);
        }

        // color
        for v in &self.color_collection {
            v.write_to_color(writer);
        }

        write_end_tag(writer, "dataBar");
    }
}
