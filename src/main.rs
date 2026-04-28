use dotenvy::dotenv;
use postgres::{Client, NoTls};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Silently ignore if .env is missing
    dotenv().ok();
    let pghost = env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string());
    let pgdbname = env::var("POSTGRES_DB").unwrap_or_else(|_| "pgdb".to_string());
    let pguser = env::var("POSTGRES_USER").unwrap_or_else(|_| "pguser".to_string());
    let pgpwd = env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "pgpwd".to_string());
    let connect_params = format!(
        "host={} dbname={} user={} password={}",
        pghost, pgdbname, pguser, pgpwd
    );

    println!("Connecting to database with params: {}", connect_params);

    // 1. Establish connection
    let mut client = Client::connect(&connect_params, NoTls)?;

    // 2. Execute a query
    for row in client.query("SELECT 1 + 1", &[])? {
        let result: i32 = row.get(0);
        println!("Result: {}", result);
    }

    Ok(())
}
