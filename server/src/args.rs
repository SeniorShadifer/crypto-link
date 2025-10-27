#[derive(clap::Parser, Debug)]
#[command(name = "CryptoLink Server")]
#[command(about = "Server for CryptoLink - messenger with assymetric encryption.")]
pub struct Arguments {
    /// Path to directory, which contains server configuration, database, files and other data.
    #[arg(short, long, default_value = "./server_data")]
    pub server_data_path: String,
}
