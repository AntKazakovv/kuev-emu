use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;

pub fn create_db() -> Result<()> {
    let conn = Connection::open("main.db")?;

    conn.execute(
        "create table if not exists params (
             id integer primary key,
             varname text not null unique,
             frname text not null,
             kmr integer,
             mr integer,
             signal integer,
             typemr text
         )",
        NO_PARAMS,
    )?;
    Ok(())
}