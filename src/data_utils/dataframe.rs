use std::collections::HashMap;

pub struct Dataframe{
    pub data: HashMap<String, Vec<String>>,
    pub rows: usize,
    pub cols: usize,
}

impl Dataframe {

    pub fn from_csv(path: String) -> Result<Self, ()>{
        unimplemented!()
    }

    pub fn from_file(path: String, delimiter: char) -> Result<Self, ()>{
        unimplemented!()
    }

    pub fn to_csv(&self,path:String) -> Result<(), ()>{
        unimplemented!()
    }

    pub fn head(&self,){
        unimplemented!()
    }

    pub fn tail(&self){
        unimplemented!()
    }

    pub fn memory_usage(&self){
        unimplemented!()
    }

    pub fn is_empty(&self) -> bool{
        unimplemented!()
    }

    pub fn drop_column(&self){

    }

    pub fn at(&self){
        unimplemented!()
    }

    pub fn at_index(&self){
        unimplemented!()
    }
    
}