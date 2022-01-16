use std::io::*;

/// Display struct
pub struct Display{
    page_index: usize,
    last_index: usize,
    back_index: Vec<usize>,
    pub page_buffer: Vec<Page>,
    
}

type Action = Box<dyn Fn(&Display,Response)->Response>;
pub struct Page{
    rows: Vec<String>,
    action: Action
}
impl Page{
    pub fn new(action:Action)->Self{
        Page{rows:Vec::new(),action}
    }
    pub fn push(&mut self, value: String){
        self.rows.push(value)
    }
}