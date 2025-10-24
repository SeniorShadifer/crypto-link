mod conf;

use clap::Parser;

use crate::conf::ServerConfiguration;

#[derive(clap::Parser, Debug)]
#[command(name = "CryptoLink Server")]
#[command(about = "Server for CryptoLink - messenger with assymetric encryption.")]
struct Arguments {
    /// Path to directory, which contains server configuration, database, files and other data.
    #[arg(short, long, default_value = "./server_data")]
    server_data_path: String,

    #[arg(short, long, default_value = "DEBUG")]
    log_level: String,
}

fn main() {
    let arguments = Arguments::parse();

    shared::log
        ::setup_logger(format!("{}/.log", arguments.server_data_path), arguments.log_level.as_str())
        .expect("Cannot setup logger!");
    log::info!("Logger successfully installed!");

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
}
