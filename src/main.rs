mod modbus;
mod database;

use dotenv::dotenv;
use std::env;
use tokio_modbus::prelude::*;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let modbus_address = format!(
        "{}:{}",
        env::var("MODBUS_HOST")?,
        env::var("MODBUS_PORT")?
    );

    let db_connection_string = format!(
        "host={} user={} password={} dbname={}",
        env::var("DB_HOST")?,
        env::var("DB_USER")?,
        env::var("DB_PASSWORD")?,
        env::var("DB_NAME")?
    );

    let (db_client, db_connection) = database::connect(&db_connection_string).await?;
    tokio::spawn(async move {
        if let Err(e) = db_connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    let socket_addr = modbus_address.parse()?;
    let stream = TcpStream::connect(socket_addr).await?;
    let mut context = tokio_modbus::client::tcp::connect(stream).await?;

    let coil_addresses = vec![0];
    let discrete_input_addresses = vec![0];
    let input_register_addresses = vec![0];
    let holding_register_addresses = vec![0];

    modbus::read_registers("coils", &coil_addresses, &mut context, &db_client).await?;
    modbus::read_registers("discrete_inputs", &discrete_input_addresses, &mut context, &db_client).await?;
    modbus::read_registers("input_registers", &input_register_addresses, &mut context, &db_client).await?;
    modbus::read_registers("holding_registers", &holding_register_addresses, &mut context, &db_client).await?;

    Ok(())
}
