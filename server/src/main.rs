use clap::Parser;
use colored::*;

#[derive(clap::Parser, Debug)]
#[command(name = "CryptoLink Server")]
#[command(about = "Server for CryptoLink - messenger with assymetric encryption.")]
struct Arguments {
    /// Path to directory, which contains server configuration, database, files and other data.
    #[arg(short, long, default_value = "./server_data")]
    server_data_path: String,
}

fn setup_logger(log_file_path: String) -> Result<(), Box<dyn std::error::Error>> {
    let colors = fern::colors::ColoredLevelConfig
        ::new()
        .error(fern::colors::Color::Red)
        .warn(fern::colors::Color::Yellow)
        .info(fern::colors::Color::Green)
        .debug(fern::colors::Color::Blue)
        .trace(fern::colors::Color::Black);

    let path = std::path::Path::new(log_file_path.as_str());
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    fern::Dispatch
        ::new()
        .format(move |out, message, record| {
            out.finish(
                format_args!(
                    "{} {}: {}",
                    colored::ColoredString
                        ::from(
                            format!(
                                "[{}]",
                                chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
                            )
                        )
                        .black(),
                    colors.color(record.level()),
                    message
                )
            )
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(
            std::fs::OpenOptions::new().write(true).append(true).create(true).open(log_file_path)?
        )
        .apply()?;

    Ok(())
}

fn main() {
    let arguments = Arguments::parse();

    setup_logger(format!("{}/.log", arguments.server_data_path)).expect("Cannot setup logger!");
    log::info!("Logger successfully installed!");
}
