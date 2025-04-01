use std::{fs::{self, File}, io::Read};
use zip::ZipArchive;
use xml::{self, reader::XmlEvent, EventReader};
mod xml_utils;


#[derive(Debug, Clone)]
pub enum CellValue {
    Int(i32),
    Float(f64),
    Text(String),
    Bool(bool),
    None
}

impl CellValue {
    pub fn as_str(&mut self) -> Option<&str> {
        if let CellValue::Text(v) = self {
            Some(v)
        } else {
            None
        }
    }
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

#[derive(Debug, Clone)]
pub struct MetalFrame {
    pub columns: Vec<CellValue>,
    pub rows: Vec<Vec<CellValue>>,
    sst: Vec<String>
}

impl MetalFrame {

    pub fn get_rows(&self) -> &Vec<Vec<CellValue>> {
        return &self.rows
    }

    pub fn by_col(&mut self, col: &str) -> Vec<String> {
        let val_vec = Vec::new();

        let col_index = self.columns.iter_mut().position(|c| c.as_str().expect("val not unwrapped") == col).expect("column not found");

        println!("{col_index}");

        return val_vec;
    }

    pub fn read_excel(path: &str, sheet: &str, column_row: Option<usize>) -> Self {
        let column_row = column_row.unwrap_or(0);

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
                eprintln!("{}", err)
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
                } 

            }
            Err(e) => {
                eprintln!("Error: {}", e)
            }
        }
        
        let mut sheet_data = String::new();


        if let (Some(sheet_data_index), Some(end_sheet_data_index)) = (file_contents.find("<sheetData>"), file_contents.find("</sheetData>")) {
            
            sheet_data = file_contents[sheet_data_index + "<sheetData>".len()..end_sheet_data_index].to_string();
            println!("{}", sheet_data);
        
            let rows = xml_utils::read_rows(sheet_data.as_bytes(), &mf.sst);
            mf.rows = rows.clone();
        }

        mf.columns = mf.rows.get(column_row).expect("Header row not found").to_vec();

        return mf;
    }
    
    
}
