use std::io::{Cursor, Read, Write};

use quick_xml::{events::Event, Reader, Writer};
use umya_spreadsheet::{reader, writer};
use zip::{write::SimpleFileOptions, ZipArchive, ZipWriter};

fn workbook_with_worksheet_prefix(prefix: &str) -> Vec<u8> {
    let mut book = umya_spreadsheet::new_file();
    let sheet = book.get_sheet_mut(&0).unwrap();
    sheet.get_cell_mut("A1").set_value("Quarterly report");
    sheet.get_cell_mut("B2").set_value_number(240);
    sheet
        .get_cell_mut("A1")
        .get_style_mut()
        .get_font_mut()
        .set_bold(true);
    sheet
        .get_header_footer_mut()
        .get_odd_header_mut()
        .set_value("&CReport heading");
    sheet
        .get_header_footer_mut()
        .get_odd_footer_mut()
        .set_value("&LInternal&R&P / &N");
    let mut original = Vec::new();
    writer::xlsx::write_writer(&book, &mut original).unwrap();
    let mut input = ZipArchive::new(Cursor::new(original)).unwrap();
    let mut output = ZipWriter::new(Cursor::new(Vec::new()));
    for index in 0..input.len() {
        let mut part = input.by_index(index).unwrap();
        let mut bytes = Vec::new();
        part.read_to_end(&mut bytes).unwrap();
        if part.name() == "xl/worksheets/sheet1.xml" && !prefix.is_empty() {
            let mut xml = Reader::from_reader(bytes.as_slice());
            let mut serialized = Writer::new(Vec::new());
            loop {
                let event = xml.read_event().unwrap();
                let is_empty = matches!(event, Event::Empty(_));
                let event = match event {
                    Event::Start(element) | Event::Empty(element) => {
                        let mut renamed = element.into_owned();
                        let local = String::from_utf8(renamed.name().as_ref().to_vec()).unwrap();
                        renamed.set_name(format!("{prefix}:{local}").as_bytes());
                        if local == "worksheet" {
                            renamed.push_attribute((
                                format!("xmlns:{prefix}").as_str(),
                                "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
                            ));
                        }
                        if is_empty {
                            Event::Empty(renamed)
                        } else {
                            Event::Start(renamed)
                        }
                    }
                    Event::End(element) => Event::End(quick_xml::events::BytesEnd::new(format!(
                        "{prefix}:{}",
                        std::str::from_utf8(element.name().as_ref()).unwrap()
                    ))),
                    Event::Eof => break,
                    other => other.into_owned(),
                };
                serialized.write_event(event).unwrap();
            }
            bytes = serialized.into_inner();
        }
        output
            .start_file(part.name(), SimpleFileOptions::default())
            .unwrap();
        output.write_all(&bytes).unwrap();
    }
    output.finish().unwrap().into_inner()
}

fn assert_book_contents(book: &umya_spreadsheet::Spreadsheet) {
    let sheet = book.get_sheet(&0).unwrap();
    assert_eq!(
        sheet.get_cell("A1").unwrap().get_value(),
        "Quarterly report",
        "worksheet text must survive namespace handling"
    );
    assert_eq!(
        sheet.get_cell("B2").unwrap().get_value_number(),
        Some(240.0)
    );
    assert!(*sheet
        .get_cell("A1")
        .unwrap()
        .get_style()
        .get_font()
        .unwrap()
        .get_bold());
    assert_eq!(
        sheet.get_header_footer().get_odd_header().get_value(),
        "&CReport heading"
    );
    assert_eq!(
        sheet.get_header_footer().get_odd_footer().get_value(),
        "&LInternal&R&P / &N"
    );
}

fn assert_worksheet_contents(prefix: &str) {
    let bytes = workbook_with_worksheet_prefix(prefix);
    for with_sheet_read in [false, true] {
        let mut book =
            reader::xlsx::read_reader(Cursor::new(bytes.clone()), with_sheet_read).unwrap();
        if !with_sheet_read {
            let cells = book.get_lazy_read_sheet_cells(&0).unwrap();
            let mut values: Vec<String> = cells
                .get_collection()
                .iter()
                .map(|cell| cell.get_value().to_string())
                .collect();
            values.sort();
            assert_eq!(values, vec!["240", "Quarterly report"]);
        }
        book.read_sheet(0);
        assert_book_contents(&book);
        let mut rewritten = Vec::new();
        writer::xlsx::write_writer(&book, &mut rewritten).unwrap();
        let reopened = reader::xlsx::read_reader(Cursor::new(rewritten), true).unwrap();
        assert_book_contents(&reopened);
    }
}

#[test]
fn default_worksheet_namespace_preserves_contents() {
    assert_worksheet_contents("");
}

#[test]
fn short_worksheet_namespace_prefix_preserves_contents() {
    assert_worksheet_contents("s");
}

#[test]
fn long_worksheet_namespace_prefix_preserves_contents() {
    assert_worksheet_contents("spreadsheet");
}
