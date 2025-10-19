mod conf;

use clap::Parser;

#[derive(clap::Parser, Debug)]
#[command(name = "CryptoLink Server")]
#[command(about = "Server for CryptoLink - messenger with assymetric encryption.")]
struct Arguments {
    /// Path to directory, which contains server configuration, database, files and other data.
    #[arg(short, long, default_value = "./server_data")]
    server_data_path: String,
}

fn main() {
    let arguments = Arguments::parse();

    shared::log
        ::setup_logger(format!("{}/.log", arguments.server_data_path))
        .expect("Cannot setup logger!");
    log::info!("Logger successfully installed!");

    let config_path = format!("{}/.conf", arguments.server_data_path);
    let configuration = conf::ServerConfiguration::load_from_file(config_path.as_str());
    log::info!("Configuration successfully loaded!");
    log::debug!("Configuration: {:#?}", configuration);
}
