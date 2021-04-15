# Purpose

This project is meant as a demonstration and learn object for developing a
small web application with rust. It provides a table to track bloodpressure
measurement entities.

# Setup and Startup

## Install dependencies

### Install Rust

Installation of Rust is best described at their [Install Guide](https://www.rust-lang.org/tools/install).

### Install Diesel

Installation of Diesel is best described at their [Install Guide](https://diesel.rs/guides/getting-started/).

## Setup

# Diesel setup

It is necessary to add create folder `migrations` and the file `diesel.toml`
manually.

## Generate a new table

To generate a new table with name `bloodpressure` use `generate`, this
generates a new folder for the migration and two files `up.sql` and `down.sql`.
```bash
diesel migration generate bloodpressure
```

## Run migrations

To run the migrations, simply call `run`. This updates the file `src/schema.rs`.
(Or whatever was assigned for `file` in `diesel.toml`). All migrations, which
are not run until now will be updated.
```bash
diesel migration run
```

## Build and start

To build and start the web-server, cargo has to be used.
```bash
cargo run --bin run_server
```
To only build everything run `cargo build`

There are also some binaries to be used via bash. Which implement the following
functionality:
* Add a new measurement entity.
```bash
./target/debug/add
```
* Delete a measurement entity.
```bash
./target/debug/delete
```
* Print the measurement entities to shell.
```bash
./target/debug/export
```
