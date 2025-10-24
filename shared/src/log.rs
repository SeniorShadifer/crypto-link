use colored::*;

pub fn setup_logger(
    log_file_path: String,
    log_level: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let colors = fern::colors::ColoredLevelConfig
        ::new()
        .error(fern::colors::Color::Red)
        .warn(fern::colors::Color::Yellow)
        .info(fern::colors::Color::Green)
        .debug(fern::colors::Color::Blue)
        .trace(fern::colors::Color::Black);

    crate::fs::create_path_to_file(&log_file_path)?;

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
        .level(match log_level {
            "ERROR" => log::LevelFilter::Error,
            "WARN" => log::LevelFilter::Warn,
            "INFO" => log::LevelFilter::Info,
            "DEBUG" => log::LevelFilter::Debug,
            "TRACE" => log::LevelFilter::Trace,
            _ => log::LevelFilter::Debug,
        })
        .chain(std::io::stdout())
        .chain(
            std::fs::OpenOptions::new().write(true).append(true).create(true).open(log_file_path)?
        )
        .apply()?;

    Ok(())
}
