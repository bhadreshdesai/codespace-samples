use postgres::{Client, NoTls};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Establish connection
    let mut client = Client::connect(
        "host=localhost user=pguser password=pgpwd dbname=pgdb",
        NoTls
    )?;

    // 2. Execute a query
    for row in client.query("SELECT 1 + 1", &[])? {
        let result: i32 = row.get(0);
        println!("Result: {}", result);
    }

    Ok(())
}