#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;



pub mod xml_tools;
pub mod db_tools;
use db_tools::*;
use xml_tools::*;


fn initDevice(maplinked: &Maplinked) -> Vec<StateItems> {
    
    let size_vec_items = maplinked.items.len();
    let mut list_state_items = Vec::new();

    // бежим по длинне списка маплинкед, заполняем затычками струтуру
    // где будем хранить значения параметров
    for ind in 0..size_vec_items {
        // новый элемент структуры
        let mut stateItem = StateItems { 
            varname: &maplinked.items[ind].varname, // берем имя параметра из maplinked
            value: StateVal::Unit // кладем временно нулевое значение
         };
        list_state_items.push( stateItem )
    }
    return list_state_items
}


fn main() {
    // let filename = "./xml_source/maplinked.xml";
    // let maplinked: Maplinked = xmlToStruct(filename.to_string());
    // let mut list_state_items = initDevice(&maplinked);

    // println!("{:#?}", list_state_items[0].varname);
    create_db();

}
