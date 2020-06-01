

use std::fs;

pub fn xmlToStruct(filename: String) -> Maplinked {

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    return serde_xml_rs::from_reader(contents.as_bytes()).unwrap();
}


//описываем структуры в которые десереализуется xml 
#[derive(Deserialize, Debug)]
pub struct Maplinked {
    #[serde(rename="item")]
    pub items: Vec<Item>
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