# Modbus TCP/IP Project in Rust

This project is a Rust-based application that reads Modbus TCP/IP registers and stores the results in a PostgreSQL database.

## 🔧 Features

- Reads the first 10 registers of the four Modbus types:
  - Coils
  - Discrete Inputs
  - Holding Registers
  - Input Registers
- Register addresses are read from a configurable list
- Results are printed to the terminal and stored in PostgreSQL
- Configuration handled with environment variables
- Clean and modular structure using `tokio`, `modbus`, `dotenvy`, and `tokio-postgres`

---

## 🚀 Setup Instructions

### 1. Clone the repository

```bash
git clone git@github.com:NovoaCl/modbus-tcp-ip.git
cd modbus-tcp-ip

Create .env file:

    MODBUS_SERVER_IP=127.0.0.1
    MODBUS_SERVER_PORT=502
    DATABASE_URL=postgresql://user:password@localhost:5432/mydb

Create the database table (via DBeaver or psql)

    CREATE TABLE IF NOT EXISTS modbus_readings (
        id SERIAL PRIMARY KEY,
        address INTEGER NOT NULL,
        value INTEGER NOT NULL,
        register_type VARCHAR(50) NOT NULL,
        timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );

Run the program

    cargo run
