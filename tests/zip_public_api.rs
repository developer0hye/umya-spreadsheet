use std::io::Cursor;

use umya_spreadsheet::{reader, writer, XlsxError};

#[test]
fn public_reader_accepts_a_valid_workbook_and_returns_zip_errors_for_invalid_input() {
    let mut workbook = umya_spreadsheet::new_file();
    workbook
        .get_sheet_mut(&0)
        .unwrap()
        .get_cell_mut("A1")
        .set_value("valid workbook");

    let mut bytes = Vec::new();
    writer::xlsx::write_writer(&workbook, &mut bytes).unwrap();
    let parsed = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    assert_eq!(
        parsed.get_sheet(&0).unwrap().get_value("A1"),
        "valid workbook"
    );

    let error = reader::xlsx::read_reader(Cursor::new(b"not a zip archive"), true).unwrap_err();
    assert!(matches!(error, XlsxError::Zip(_)));
}
