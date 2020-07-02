use rusqlite::{params, Connection};
use rusqlite::NO_PARAMS;

// pub mod xml_tools;
// use xml_tools::*;

pub fn create_db() -> rusqlite::Result<Connection> {
    let conn = Connection::open("main.db")?;

    conn.execute(
        "create table if not exists params (
             id integer primary key,
             varname text not null unique,
             frname text not null,
             kmr integer,
             mr integer,
             signal integer,
             typemr text,
             value text
         )",
        NO_PARAMS,
    )?;
    Ok(conn)
}

/* инициализация таблицы, внесение всех параметров с заглушкой value на None */
pub fn init_data(info: &Vec<crate::xml_tools::Item>, val: &Vec<crate::xml_tools::StateItems>, conn: &Connection) -> rusqlite::Result<()> {
    let none = "None".to_string();
    
    let size_list = info.len();
    for ind in 0..size_list {
        let query_result = conn.execute("INSERT INTO params (varname, frname, kmr, mr, signal, typemr, value) values (?1, ?2, ?3, ?4, ?5, ?6, ?7)", 
            params![&info[ind].varname, &info[ind].frname, &info[ind].kmr, &info[ind].mr, &info[ind].signal, &info[ind].typemr, &none]
        );

    }

    return Ok(());
} 

/* обновление значения у конкретного параметра */
pub fn update_value<T: ToString>(varname: &String, value: T, conn: &Connection)  {
    let query_result = conn.execute("UPDATE params SET value = (?1) WHERE varname = (?2)", params![&value.to_string(), &varname]);
    match query_result {
        Ok(1) => {},
        e => panic!("ERR: {:?}", e)
    }
}