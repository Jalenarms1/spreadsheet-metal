#[derive(Debug)]
enum CellValue {
    Int,
    Float,
    Text,
    Bool,
    None
}

#[derive(Debug)]
pub struct MetalFrame {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<CellValue>>,
    sst: Vec<String>
}

impl MetalFrame {
    pub fn new() -> Self {
        return Self { columns: Vec::new(), rows: Vec::new(), sst: Vec::new() };
    }
}
