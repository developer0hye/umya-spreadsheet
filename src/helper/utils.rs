#[macro_export]
macro_rules! from_err {
    ($from:ty, $to:tt, $var:tt) => {
        impl From<$from> for $to {
            #[inline]
            fn from(e: $from) -> $to {
                $to::$var(e)
            }
        }
    };
}

/// Decodes a text span and resolves its entity and character references.
///
/// quick-xml reports references as separate `GeneralRef` events, so a text
/// event alone would read `R&amp;D` as `R` and `D`. Readers take the raw
/// element content with `Reader::read_text_into` and unescape it here.
/// CDATA content stays literal; malformed spans keep their raw text.
pub(crate) fn unescape_xml_text(e: &quick_xml::events::BytesText<'_>) -> String {
    let decoded = match e.decode() {
        Ok(decoded) => decoded,
        Err(_) => String::from_utf8_lossy(e),
    };
    let mut remaining = decoded.as_ref();
    let mut output = String::with_capacity(remaining.len());
    while let Some(start) = remaining.find("<![CDATA[") {
        let prefix = &remaining[..start];
        output.push_str(&quick_xml::escape::unescape(prefix).unwrap_or(prefix.into()));
        let content = &remaining[start + 9..];
        let Some(end) = content.find("]]>") else {
            return decoded.into_owned();
        };
        output.push_str(&content[..end]);
        remaining = &content[end + 3..];
    }
    output.push_str(&quick_xml::escape::unescape(remaining).unwrap_or(remaining.into()));
    output
}

#[cfg(test)]
mod tests {
    #[test]
    fn malformed_text_keeps_the_existing_tolerant_behavior() {
        for text in ["R&broken", "<![CDATA[unfinished", "before &unknown; after"] {
            let raw = quick_xml::events::BytesText::from_escaped(text);
            assert_eq!(super::unescape_xml_text(&raw), text);
        }
    }
}
