use std::borrow::Cow;

pub(crate) fn format_as_percentage<'input>(value: &f64, format: &'input str) -> Cow<'input, str> {
    let mut value = value.to_string();
    let mut format = Cow::Borrowed(format);
    format = Cow::Owned(format.replace('%', ""));
    let blocks: Vec<&str> = format.split('.').collect();
    let len = match blocks.get(1) {
        Some(v) => v.len(),
        None => 0,
    };
    value = format!(
        "{:0width$.len$}%",
        100f64 * &value.parse::<f64>().unwrap_or(0.0),
        width = 1,
        len = len
    );
    Cow::Owned(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_as_percentage_keeps_decimal_precision() {
        // Rounding to an integer before applying the decimal format code
        // turned 17.309...% into "17.0%".
        assert_eq!(format_as_percentage(&0.1730909090909091, "0.0%"), "17.3%");
    }

    #[test]
    fn format_as_percentage_integer_format() {
        assert_eq!(format_as_percentage(&0.1730909090909091, "0%"), "17%");
    }

    #[test]
    fn format_as_percentage_two_decimals() {
        assert_eq!(format_as_percentage(&0.5, "0.00%"), "50.00%");
    }
}
