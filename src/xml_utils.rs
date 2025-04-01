use std::{fmt::Error, num::{ParseFloatError, ParseIntError}, str::ParseBoolError};

use xml::{self, reader::XmlEvent, Encoding, EventReader, ParserConfig};
use regex::Regex;
use crate::CellValue;

pub fn get_sheet_list(xml_data: &[u8]) -> Vec<String> {
    let mut str_vec = Vec::new();

    let parser = xml::EventReader::new(xml_data);

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, ..}) => {
                if name.local_name == "sheet" {
                    let name = attributes.get(0).expect("attr not found");
                    for attr in attributes {
                        if attr.name.local_name == "name" {
                            str_vec.push(attr.value.to_ascii_lowercase());
                        }
                    }
                }
            }
            _ => {}
        }
    }


    return str_vec
}

pub fn get_sst(xml_data: &[u8]) -> Vec<String>{
    let mut sst = Vec::new();

    let parser = EventReader::new(xml_data);
    for e in parser {
        match e {
            Ok(XmlEvent::Characters(val)) => {
                sst.push(val);
            }
            _ => {}
        }
    }

    return sst;
}

pub fn read_rows(xml_data: &[u8], sst: &Vec<String>) -> Vec<Vec<CellValue>> {
    println!("reading rows");
    let xml_string = String::from_utf8_lossy(xml_data).to_string();

    let re = Regex::new(r#"\s+\w+:\w+="[^"]*""#).unwrap();
    let cleaned_xml = re.replace_all(&xml_string, "").to_string();

    let parser = EventReader::from_str(&cleaned_xml);

    let mut grid_vec: Vec<Vec<CellValue>> = Vec::new();

    let mut row_vec: Vec<CellValue> = Vec::new();

    let mut val_type: CellValue = CellValue::None;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                if name.local_name == "c" {
                    if attributes.iter().any(|a| a.name.local_name == "t" && a.value == "s") {
                        val_type = CellValue::Text(String::new())
                        
                    }  else {
                        val_type = CellValue::None;

                    }
                } 
            }

            Ok(XmlEvent::Characters(val)) => {
                // println!("{val}");
                if val.is_empty() {
                    row_vec.push(CellValue::None);
                } else {
                    let new_val = match val_type {
                        CellValue::Text(ref s) => {
                            let parsed_v: usize = val.parse().expect(&format!("{}", val).to_string());
                            let sstv = sst.get(parsed_v).expect("index not found");
                            CellValue::Text(sstv.trim().to_string())
                        },
                        _ => {
                            let parsed_int: Result<i32, ParseIntError> = val.parse();
                            match parsed_int {
                                Ok(v) => {
                                    CellValue::Int(v)
                                }
                                Err(e) => {
                                    let parsed_float: Result<f64, ParseFloatError> = val.parse();
                                    match parsed_float {
                                        Ok(f) => {
                                            CellValue::Float(f)
                                        }
                                        Err(e) => {
                                            let parsed_bool: Result<bool, ParseBoolError> = val.parse();
                                            match parsed_bool {
                                                Ok(b) => {
                                                    CellValue::Bool(b)
                                                }
                                                Err(e) => {
                                                    CellValue::None

                                                }
                                            }

                                        }
                                    }
                                }
                            }
                        }
                        
                    };
                    row_vec.push(new_val);

                }
            }
            Ok(XmlEvent::EndElement { name, .. }) => {
                if name.local_name == "row" {
                    grid_vec.push(row_vec);
                    row_vec = Vec::new();
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
            _ => {}
        }
    }
    
    return grid_vec;
}
