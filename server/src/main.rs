mod conf;
mod args;

use clap::Parser;

use crate::conf::ServerConfiguration;

#[tokio::main]
async fn main() {
    let arguments = args::Arguments::parse();

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
