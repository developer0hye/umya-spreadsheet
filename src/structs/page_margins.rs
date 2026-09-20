use crate::structs::DoubleValue;

use crate::reader::driver::*;
use crate::writer::driver::*;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct PageMargins {
    left: DoubleValue,
    right: DoubleValue,
    top: DoubleValue,
    bottom: DoubleValue,
    header: DoubleValue,
    footer: DoubleValue,
}
impl PageMargins {
    #[inline]
    pub fn get_left(&self) -> &f64 {
        self.left.get_value()
    }

    /// Whether the left margin was explicitly set, including zero.
    #[inline]
    #[must_use]
    pub fn has_left(&self) -> bool {
        self.left.has_value()
    }

    #[inline]
    pub fn set_left(&mut self, value: f64) -> &mut Self {
        self.left.set_value(value);
        self
    }

    #[inline]
    pub fn get_right(&self) -> &f64 {
        self.right.get_value()
    }

    /// Whether the right margin was explicitly set, including zero.
    #[inline]
    #[must_use]
    pub fn has_right(&self) -> bool {
        self.right.has_value()
    }

    #[inline]
    pub fn set_right(&mut self, value: f64) -> &mut Self {
        self.right.set_value(value);
        self
    }

    #[inline]
    pub fn get_top(&self) -> &f64 {
        self.top.get_value()
    }

    /// Whether the top margin was explicitly set, including zero.
    #[inline]
    #[must_use]
    pub fn has_top(&self) -> bool {
        self.top.has_value()
    }

    #[inline]
    pub fn set_top(&mut self, value: f64) -> &mut Self {
        self.top.set_value(value);
        self
    }

    #[inline]
    pub fn get_bottom(&self) -> &f64 {
        self.bottom.get_value()
    }

    /// Whether the bottom margin was explicitly set, including zero.
    #[inline]
    #[must_use]
    pub fn has_bottom(&self) -> bool {
        self.bottom.has_value()
    }

    #[inline]
    pub fn set_bottom(&mut self, value: f64) -> &mut Self {
        self.bottom.set_value(value);
        self
    }

    #[inline]
    pub fn get_header(&self) -> &f64 {
        self.header.get_value()
    }

    /// Whether the header margin was explicitly set, including zero.
    #[inline]
    #[must_use]
    pub fn has_header(&self) -> bool {
        self.header.has_value()
    }

    #[inline]
    pub fn set_header(&mut self, value: f64) -> &mut Self {
        self.header.set_value(value);
        self
    }

    #[inline]
    pub fn get_footer(&self) -> &f64 {
        self.footer.get_value()
    }

    /// Whether the footer margin was explicitly set, including zero.
    #[inline]
    #[must_use]
    pub fn has_footer(&self) -> bool {
        self.footer.has_value()
    }

    #[inline]
    pub fn set_footer(&mut self, value: f64) -> &mut Self {
        self.footer.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        self.left
            .set_value_string(get_attribute(e, b"left").unwrap());
        self.right
            .set_value_string(get_attribute(e, b"right").unwrap());
        self.top.set_value_string(get_attribute(e, b"top").unwrap());
        self.bottom
            .set_value_string(get_attribute(e, b"bottom").unwrap());
        self.header
            .set_value_string(get_attribute(e, b"header").unwrap());
        self.footer
            .set_value_string(get_attribute(e, b"footer").unwrap());
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // Numeric zero is a valid explicit margin, not an absence marker.
        let value_or_default = |value: &DoubleValue, default: &str| -> String {
            if value.has_value() {
                value.get_value_string()
            } else {
                default.to_string()
            }
        };
        let left = value_or_default(&self.left, "0.7");
        let right = value_or_default(&self.right, "0.7");
        let top = value_or_default(&self.top, "0.75");
        let bottom = value_or_default(&self.bottom, "0.75");
        let header = value_or_default(&self.header, "0.3");
        let footer = value_or_default(&self.footer, "0.3");
        let attributes = vec![
            ("left", left.as_str()),
            ("right", right.as_str()),
            ("top", top.as_str()),
            ("bottom", bottom.as_str()),
            ("header", header.as_str()),
            ("footer", footer.as_str()),
        ];
        write_start_tag(writer, "pageMargins", attributes, true);
    }
}
