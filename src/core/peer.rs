use std::collections::HashMap;


pub type OpinionsMap = HashMap<String, i32>;

pub struct Peer {
    pub name : String,
    pub opinions : OpinionsMap

}

impl Peer{
    pub fn new(name : &str) -> Self {
        Self { 
            name : name.to_owned(),
            opinions : HashMap::new()
        }
    }
    pub fn set_opinion(&mut self, name : &str, val : i32) -> () {

        //todo: figure out how to pass a ref for lookup
        *self.opinions.entry(name.to_owned()).or_insert(val) = val;
    }
    pub fn set_opinions(&mut self, opinions : &OpinionsMap) -> () {
        for (opn_name, opn_value) in opinions.into_iter(){
            self.set_opinion(opn_name, *opn_value);
        }

    }
    pub fn get_opinion(&self, name : &str) -> i32 {

        if !self.opinions.contains_key(name){
            return 0;
        }
        return self.opinions[name];
    }
}