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

    shared
        ::setup_logger(format!("{}/.log", arguments.server_data_path))
        .expect("Cannot setup logger!");
    log::info!("Logger successfully installed!");
}
