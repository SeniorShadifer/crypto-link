use std::path::Path;

use serde::Deserialize;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ServerConfiguration {
    #[serde(default)]
    server: ServerSection,

    #[serde(default)]
    encryption: EncryptionSection,

    #[serde(default)]
    pub logging: LoggingSection,
}

impl ServerConfiguration {
    pub fn load_from_file(path: &str) -> Result<ServerConfiguration, Box<dyn std::error::Error>> {
        if Path::new(path).exists() {
            return Ok(serde_ini::from_str(std::fs::read_to_string(path)?.as_str())?);
        } else {
            log::info!(
                "Configuration file '{}' is not exists. Trying to load default values...",
                path
            );

            shared::fs::create_path_to_file(path)?;

            let default_conf_file_name = "default.server.conf";
            let path_to_default_conf = std::env
                ::current_exe()?
                .join("..")
                .join(default_conf_file_name);

            if !path_to_default_conf.exists() {
                return Err(
                    Box::new(
                        std::io::Error::new(
                            std::io::ErrorKind::NotFound,
                            format!("One of system package files ('<PROGRAM_ROOT>/{}') is not exists. Try reinstall program.", default_conf_file_name)
                        )
                    )
                );
            }

            std::fs::copy(path_to_default_conf, path)?;
            return Ok(serde_ini::from_str(std::fs::read_to_string(path)?.as_str())?);
        }
    }
}

impl Default for ServerConfiguration {
    fn default() -> Self {
        Self {
            server: ServerSection::default(),
            encryption: EncryptionSection::default(),
            logging: LoggingSection::default(),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ServerSection {
    #[serde(default)]
    address: String,
}

impl Default for ServerSection {
    fn default() -> Self {
        Self { address: ":5140".to_string() }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EncryptionSection {
    #[serde(default, deserialize_with = "deserialize_bool")]
    use_assymetric_incapsulation: bool,

    #[serde(default)]
    incapsulation_algorythm: String,
}

impl Default for EncryptionSection {
    fn default() -> Self {
        Self {
            use_assymetric_incapsulation: true,
            incapsulation_algorythm: "Kyber1024".to_string(),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LoggingSection {
    #[serde(default)]
    pub log_level: String,
}

impl Default for LoggingSection {
    fn default() -> Self {
        Self {
            log_level: "INFO".to_string(),
        }
    }
}

fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where D: serde::Deserializer<'de>
{
    return Ok(String::deserialize(deserializer)?.parse().map_err(serde::de::Error::custom)?);
}
