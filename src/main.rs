
use rusqlite::{Connection, Error, Result};



fn main() {

    let result_create_db = setup_sql_lite_data_base();
    match result_create_db {
        Ok(_) => println!("Database Created Succesfully"),
        Err(..) => println!("Error Creating Database"),
    }
}




fn setup_sql_lite_data_base() -> Result<(),  Error>  {

    println!("Seting up SqlLite DB");
    let conn = Connection::open("sqlite.db").map_err(|_| "Failed to open database").expect("Failed to open database");

    conn.execute("CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY, name TEXT NOT NULL )", [], )?;
    Ok(())
}

async fn check_redis_connection() {
    
}   
