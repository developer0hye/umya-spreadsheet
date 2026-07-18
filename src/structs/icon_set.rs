use super::Color;
use super::ConditionalFormatValueObject;
use super::StringValue;
use crate::reader::driver::*;
use crate::writer::driver::*;
use quick_xml::events::BytesStart;
use quick_xml::events::Event;
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use thin_vec::ThinVec;

#[derive(Clone, Default, Debug)]
pub struct IconSet {
    icon_set_type: StringValue,
    cfvo_collection: ThinVec<ConditionalFormatValueObject>,
    color_collection: ThinVec<Color>,
}

impl IconSet {
    #[inline]
    pub fn get_icon_set_type(&self) -> &str {
        self.icon_set_type.get_value_str()
    }

    #[inline]
    pub fn set_icon_set_type<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.icon_set_type.set_value(value);
        self
    }

    #[inline]
    pub fn get_cfvo_collection(&self) -> &[ConditionalFormatValueObject] {
        &self.cfvo_collection
    }

    #[inline]
    pub fn set_cfvo_collection(
        &mut self,
        value: impl Into<ThinVec<ConditionalFormatValueObject>>,
    ) -> &mut Self {
        self.cfvo_collection = value.into();
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
        set_string_from_xml!(self, e, icon_set_type, "iconSet");

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
                    if e.name().into_inner() == b"iconSet" {
                        return
                    }
                },
                Event::Eof => return
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // iconSet
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let icon_set_type = self.icon_set_type.get_value_str();
        if self.icon_set_type.has_value() {
            attributes.push(("iconSet", icon_set_type));
        }
        write_start_tag(writer, "iconSet", attributes, false);

        // cfvo
        for v in &self.cfvo_collection {
            v.write_to(writer);
        }

        // color
        for v in &self.color_collection {
            v.write_to_color(writer);
        }

        write_end_tag(writer, "iconSet");
    }
}
