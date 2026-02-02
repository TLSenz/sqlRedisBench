mod tui;

use std::time::Duration;
use redis::Commands;
use rusqlite::{Connection, Error};
use tui::{run_tui, BenchResult};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_tui(run_benchmarks)
}

fn run_benchmarks() -> Vec<BenchResult> {
    let mut results = Vec::new();

    // SQLite on Disk
    let result_create_db = setup_sql_lite_data_base_on_disk();
    let sqlite_disk_duration = match result_create_db {
        Ok(ref conn) => {
            results.push(BenchResult {
                name: "SQLite on Disk Setup".to_string(),
                status: "Success".to_string(),
                duration: None,
                description: None
            });
            Some(bench_one_insert_and_select_sqlite(conn))
        }
        Err(_) => {
            results.push(BenchResult {
                name: "SQLite on Disk Setup".to_string(),
                status: "Error".to_string(),
                duration: None,
                description: None
            });
            None
        }
    };
    if let Some(d) = sqlite_disk_duration {
        results.push(BenchResult {
            name: "SQLite on Disk Bench".to_string(),
            status: "Success".to_string(),
            duration: Some(d),
            description: Some("SQl Lite Database run on Disk. One Insert and one Select".to_string())
        });
    }

    // SQLite in Memory
    let result_create_mem_db = setup_sql_lite_data_base_in_memory();
    let sqlite_mem_duration = match result_create_mem_db {
        Ok(ref conn) => {
            results.push(BenchResult {
                name: "SQLite in Memory Setup".to_string(),
                status: "Success".to_string(),
                duration: None,
                description: None
            });
            Some(bench_one_insert_and_select_sqlite(conn))
        }
        Err(_) => {
            results.push(BenchResult {
                name: "SQLite in Memory Setup".to_string(),
                status: "Error".to_string(),
                duration: None,
                description: None
            });
            None
        }
    };
    if let Some(d) = sqlite_mem_duration {
        results.push(BenchResult {
            name: "SQLite in Memory Bench".to_string(),
            status: "Success".to_string(),
            duration: Some(d),
            description: Some("SQl Lite Database run in Memory. One Insert and one Select".to_string())
        });
    }

    // Redis
    let redis_client_res = check_redis_connection();
    let redis_duration = match redis_client_res {
        Ok(mut conn) => {
            results.push(BenchResult {
                name: "Redis Connection".to_string(),
                status: "Success".to_string(),
                duration: None,
                description: None
            });
            Some(bench_one_insert_and_select_redis(&mut conn))
        }
        Err(_) => {
            results.push(BenchResult {
                name: "Redis Connection".to_string(),
                status: "Error".to_string(),
                duration: None,
                description: None
            });
            None
        }
    };
    if let Some(d) = redis_duration {
        results.push(BenchResult {
            name: "Redis Bench".to_string(),
            status: "Success".to_string(),
            duration: Some(d),
            description: Some("Redis Database. One Insert and one Select. Runs in a Docker Container on Localhost".to_string())
        });
    }
    results
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


fn bench_one_insert_and_select_sqlite(sqlite_conn: &Connection) -> Duration {
    let start = std::time::Instant::now();
    sqlite_conn.execute("INSERT INTO test (name) VALUES (?1)", ["test"]).expect("Insert failed");
    sqlite_conn.execute("SELECT * FROM test WHERE id = ?", [0]).expect("Request Failed");
    let _elapsed = start.elapsed();
    _elapsed
}

fn bench_one_insert_and_select_redis(redis_conn: &mut redis::Connection) -> Duration{
    let start = std::time::Instant::now();
    let _request: () =   redis_conn.set(0, 0).expect("Insert failed");
    let _result: () = redis_conn.get(0).expect("Request Failed");
    let elapsed = start.elapsed();
    elapsed
}
