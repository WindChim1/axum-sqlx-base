use std::path::Path;

use umya_spreadsheet::reader::xlsx;

#[test]
fn package_test() {
    let path = Path::new("./test.xlsx");
    let mut book = xlsx::read(path).expect("connot Read file");
    let sheet = book.get_sheet_by_name_mut("Sheet1").unwrap();
    let cells = sheet.get_cell_collection();

    for ele in cells {
        println!("cell:{:?}", ele.get_value())
    }

    // println!("value:{}", a);
    assert_eq!(1, 0)
}
