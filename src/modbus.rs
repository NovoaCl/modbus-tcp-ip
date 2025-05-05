use chrono::Utc;
use tokio_modbus::prelude::*;
use tokio_postgres::Client;

pub async fn read_registers(
    register_type: &str,
    addresses: &[u16],
    context: &mut Context,
    db_client: &Client,
) -> Result<(), Box<dyn std::error::Error>> {
    for &address in addresses {
        let timestamp = Utc::now();

        let values = match register_type {
            "coils" => {
                let result = context.read_coils(address, 10).await?;
                result.iter().map(|b| if *b { 1 } else { 0 }).collect::<Vec<i32>>()
            }
            "discrete_inputs" => {
                let result = context.read_discrete_inputs(address, 10).await?;
                result.iter().map(|b| if *b { 1 } else { 0 }).collect::<Vec<i32>>()
            }
            "input_registers" => {
                let result = context.read_input_registers(address, 10).await?;
                result.iter().map(|&v| v as i32).collect()
            }
            "holding_registers" => {
                let result = context.read_holding_registers(address, 10).await?;
                result.iter().map(|&v| v as i32).collect()
            }
            _ => continue,
        };

        println!("{} @ {} => {:?}", register_type, address, values);

        db_client.execute(
            "INSERT INTO modbus_data (timestamp, register_type, address, values) VALUES ($1, $2, $3, $4)",
            &[&timestamp, &register_type, &(address as i32), &values],
        ).await?;
    }

    Ok(())
}
