use spreadsheet_metal::MetalFrame;

fn main() {
    let file1 = "/Users/jalenarms/Downloads/Financial Sample.xlsx";
    let file2 = "/Users/jalenarms/Documents/excel-testing/xltest1.xlsx";
    let sheet_name = "sheet1";

    let mut mf = MetalFrame::read_excel( file1, sheet_name, Some(0));


    println!("{:?}", mf.get_rows());
    println!("{:?}", mf.by_col("Product"));
}


