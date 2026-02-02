use redis::{Client, Commands};
use rusqlite::{Connection, Error, Result};



fn main() {

    let result_create_db = setup_sql_lite_data_base_on_disk();
    match result_create_db {
        Ok(_) => println!("Database Created Succesfully"),
        Err(..) => println!("Error Creating Database"),
    }
    let redis_client = check_redis_connection();
   let mut redis_client = match redis_client {
        Ok(conn) => {
            println!("Redis Connected");
            conn
        } ,
        Err(_) => {
            println!("Could Not connect to Redis");
            panic!()
        },
    };
    bench_one_insert_and_select_sqlite(&result_create_db.unwrap());
    bench_one_insert_and_select_sqlite(&setup_sql_lite_data_base_in_memory().unwrap());
    bench_one_insert_and_select_redis(&mut redis_client);
}




fn setup_sql_lite_data_base_on_disk() -> Result<Connection,  Error>  {
    let conn = Connection::open("sqlite.db").map_err(|_| "Failed to open database").expect("Failed to open database");

    conn.execute("CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY, name TEXT NOT NULL )", [], )?;
    Ok(conn)
}

fn setup_sql_lite_data_base_in_memory() -> Result<Connection,  Error>  {
    let conn = Connection::open_in_memory().map_err(|_| "Failed to open database").expect("Failed to open database");
    conn.execute("CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY, name TEXT NOT NULL )", [], )?;
    Ok(conn)
}

fn check_redis_connection() -> Result<redis::Connection, redis::RedisError> {
    let redis_client = redis::Client::open("redis://127.0.0.1:6379/").unwrap().get_connection()?;
    Ok(redis_client)
}


fn bench_one_insert_and_select_sqlite(sqlite_conn: &Connection) {
    let start = std::time::Instant::now();
    sqlite_conn.execute("INSERT INTO test (name) VALUES (?1)", ["test"]).expect("Insert failed");
    sqlite_conn.execute("SELECT * FROM test WHERE id = ?", [0]).expect("Request Failed");
    let elapsed = start.elapsed();

}

fn bench_one_insert_and_select_redis(redis_conn: &mut redis::Connection) {
    let start = std::time::Instant::now();
    let request: () =   redis_conn.set(0, 0).expect("Insert failed");
    let result: () = redis_conn.get(0).expect("Request Failed");
    let elapsed = start.elapsed();
    println!("R: {:?}", elapsed)
}
