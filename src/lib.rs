use std::{fs::{self, File}, io::Read};
use zip::ZipArchive;
use xml::{self, reader::XmlEvent, EventReader};
mod xml_utils;


#[derive(Debug, Clone)]
enum CellValue {
    Int(i32),
    Float(f64),
    Text(String),
    Bool(bool),
    None
}

#[derive(Debug)]
enum XmlSheetType {
    SharedStringSheet,
    DataSheet(String)
}

impl XmlSheetType {
    pub fn get_file_name(&self) -> String {
        match self {
           Self::SharedStringSheet => String::from("xl/sharedStrings.xml"),
           Self::DataSheet(sheetName) => format!("xl/worksheets/{}.xml", sheetName)
        }
    }
}

#[derive(Debug)]
pub struct MetalFrame {
    pub columns: Vec<CellValue>,
    pub rows: Vec<Vec<CellValue>>,
    sst: Vec<String>
}

impl MetalFrame {

    pub fn read_excel(path: &str, sheet: &str, column_row: i8) -> Self {
        let mut mf = Self { columns: Vec::new(), rows: Vec::new(), sst: Vec::new() };

        let file = File::open(path).expect("file not read");

        let mut zip_archive = ZipArchive::new(file).expect("zip archive not unwrapped");

        let mut sst_file = zip_archive.by_name(&XmlSheetType::SharedStringSheet.get_file_name());

        let mut xml_contents = String::new();

        match sst_file.as_mut() {
            Ok(file) => {
                if file.read_to_string(&mut xml_contents).is_ok(){
                    println!("{:?}", xml_contents);
                }
            }
            Err(err) => {
                
            }
        }

        mf.sst = xml_utils::get_sst(xml_contents.as_bytes());

        drop(sst_file);

        let file_name = XmlSheetType::DataSheet(sheet.to_ascii_lowercase()).get_file_name();

        println!("{file_name}");
        
        let mut zip_file = zip_archive.by_name(&file_name);

        let mut file_contents = String::new();

        match zip_file.as_mut() {
            Ok(file) => {
                println!("{}", file.name());

                if file.read_to_string(&mut file_contents).is_ok() {
                    // println!("{file_contents}");
                } 

            }
            Err(e) => {
                eprintln!("Error: {}", e)
            }
        }
        
        let mut sheet_data = String::new();


        if let (Some(sheet_data_index), Some(end_sheet_data_index)) = (file_contents.find("<sheetData>"), file_contents.find("</sheetData>")) {
            // println!("{sheet_data_index}");
            // println!("{end_sheet_data_index}");
            
            sheet_data = file_contents[sheet_data_index + "<sheetData>".len()..end_sheet_data_index].to_string();
            println!("{}", sheet_data);
        
            let rows = xml_utils::read_rows(sheet_data.as_bytes(), &mf.sst);
            // println!("{:?}", rows);
            mf.rows = rows.clone();
            let col_row = rows.get(0);
            match col_row {
                Some(row) => {
                    mf.columns = row.clone();
                }
                None => {}
            }
        }



        return mf;
    }
    
    
}
