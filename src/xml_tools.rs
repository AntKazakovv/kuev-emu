// pub mod db_tools;

use std::fs;
use  std::option::Option;
use serde_xml_rs::*;
use serde::Deserialize;


pub fn xmlToStruct(filename: String) -> Maplinked {

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    return serde_xml_rs::from_reader(contents.as_bytes()).unwrap();
}

pub fn xmlPackToStruct<T: serde::de::DeserializeOwned>(xml: String) -> Result<T, serde_xml_rs::Error> {
    match serde_xml_rs::from_reader(xml.as_bytes()) {
        Ok(res) => {
            let result: T = res;
            return Ok(result);
        },
        Err(e) => return Err(e)
    }
    
}

//описываем структуры в которые десереализуется xml 

// --maplinked.xml--
#[derive(Deserialize, Debug)]
pub struct Maplinked {
    #[serde(rename="item")]
    pub items: Vec<Item>
}

impl Maplinked {
    pub fn new(filename: String) -> Maplinked {

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
    
        let instance: Maplinked = serde_xml_rs::from_reader(contents.as_bytes()).unwrap();
        Maplinked {
            items: instance.items
        }
    }

    pub fn initDevice(&self) -> Vec<StateItems> {
    
        let size_vec_items = self.items.len();
        let mut list_state_items = Vec::new();
    
        // бежим по длинне списка маплинкед, заполняем затычками струтуру
        // где будем хранить значения параметров
        for ind in 0..size_vec_items {
            // новый элемент структуры
            let mut stateItem = StateItems { 
                varname: &self.items[ind].varname, // берем имя параметра из maplinked
                value: StateVal::Unit // кладем временно нулевое значение
             };
            list_state_items.push( stateItem )
        }
        return list_state_items
    }
}

#[derive(Deserialize, Debug)]
pub struct Item {
    pub varname: String,
    pub frname: String,
    pub kmr: i32,
    pub mr: i32,
    pub signal: i32,
    pub typemr: String
}

// ---------------

// --<get>--

#[derive(Deserialize, Debug, PartialEq)]
pub struct Get{
    #[serde(rename = "$value")]
    items: Vec<MyGet>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum MyGet {
    URL(String),
    UUID(String)
}
// pub struct Get {
//     pub url: String,
//     pub uuid: String
// }

// trait XmlPacks {
//     fn foo();
// }

// impl XmlPacks for Get {
//     fn foo(){}
// }
// ---------------

// структуры для хранения значений параметров


pub enum StateVal {
    Valaue_str(String),
    Value_int(i64),
    Value_float(u32),
    Unit
}

pub struct StateItems<'a> {
    pub varname: &'a String,
    pub value: StateVal
}

    // let dd = StateItems { varname: "Test".to_string(), value: StateVal::value_int(10) };
    // listStateItems.push( State::varname_str("Test".to_string()) );
    // println!("{:?}", listStateItems[1]);
    // for x in listStateItems {
    //     match x {
    //         State::varname_int(i) => println!("{}", i),
    //         State::varname_float(f) => println!("{}", f),
    //         State::varname_str(s) => println!("{}", s)
    //     };
    // }