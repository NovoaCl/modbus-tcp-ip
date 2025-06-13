# Modbus TCP/IP Project in Rust

This project is a Rust-based application that reads Modbus TCP/IP registers and stores the results in a PostgreSQL database.

Features

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


