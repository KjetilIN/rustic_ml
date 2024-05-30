use std::collections::HashMap;

pub struct Dataframe{
    pub data: HashMap<String, Vec<String>>,
    pub rows: usize,
    pub cols: usize,
}

impl Dataframe {

    pub fn read_csv(path: String) -> Result<Self, ()>{
        unimplemented!()
    }

    pub fn read_table (path: String, delimiter: char) -> Result<Self, ()>{
        unimplemented!()
    }

    pub fn to_csv(path:String) -> Result<(), ()>{
        unimplemented!()
    }

    pub fn print_data(){
        unimplemented!()
    }
    
}