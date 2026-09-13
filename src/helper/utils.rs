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
/// Malformed input keeps the raw text instead of panicking.
pub(crate) fn unescape_xml_text(e: &quick_xml::events::BytesText<'_>) -> String {
    let decoded = match e.decode() {
        Ok(decoded) => decoded,
        Err(_) => String::from_utf8_lossy(e),
    };
    match quick_xml::escape::unescape(&decoded) {
        Ok(unescaped) => unescaped.into_owned(),
        Err(_) => decoded.into_owned(),
    }
}
