use tokio_postgres::{Client, Connection, NoTls};

pub async fn connect(
    connection_str: &str,
) -> Result<(Client, Connection<tokio::net::TcpStream>), tokio_postgres::Error> {
    tokio_postgres::connect(connection_str, NoTls).await
}
