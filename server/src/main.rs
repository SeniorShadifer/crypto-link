mod conf;
mod args;

use clap::Parser;

use crate::conf::ServerConfiguration;

#[tokio::main]
async fn main() {
    env_logger::init();
    log::info!("Logger successfully installed!");

    let arguments = args::Arguments::parse();

    let configuration = match
        conf::ServerConfiguration::load_from_file(
            format!("{}/.conf", arguments.server_data_path).as_str()
        )
    {
        Ok(result) => {
            log::info!("Configuration successfully loaded!");
            result
        }
        Err(e) => {
            log::error!(
                "Error while loading server configuration: {}. Will be used default values.",
                e
            );
            ServerConfiguration::default()
        }
    };

    log::debug!("Configuration: \n{:#?}", configuration);

    let connection = rusqlite::Connection
        ::open(format!("{}/.db", arguments.server_data_path))
        .unwrap_or_else(|e| {
            log::error!("CRITICAL ERROR: Cannot create SQLite connection: {}", e);
            panic!("{}", e);
        });

    log::info!("SQLite connection initialized successfully!");

    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS users 
    (
        id INTEGER PRIMARY KEY,
        nickname TEXT NOT NULL,

        password_salt BYTEA,
        password_hashing_iterations INTEGER,
        password_checksum_hash BYTEA,
        password_checksum_salt BYTEA,
        password_checksum_iterations INTEGER,

        assymetric_encryption_algorythm TEXT NOT NULL,
        public_key BYTEA NOT NULL,
        public_key_hash BYTEA NOT NULL
    )",
            ()
        )
        .unwrap_or_else(|e| {
            log::error!("Error creating table 'users' in server SQLite database: {}", e);
            panic!("{}", e);
        });
}
