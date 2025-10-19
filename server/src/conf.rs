use std::path::Path;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ServerConfiguration {
    #[serde(default)]
    server: ServerSection,

    #[serde(default)]
    encryption: EncryptionSection,
}

impl ServerConfiguration {
    pub fn load_from_file(path: &str) -> Result<ServerConfiguration, Box<dyn std::error::Error>> {
        if Path::new(path).exists() {
            return Ok(serde_ini::from_str(std::fs::read_to_string(path)?.as_str())?);
        } else {
            shared::fs::create_path_to_file(path)?;
            std::fs::copy(std::env::current_exe()?.join("..").join("default.conf"), path)?;
            return Ok(ServerConfiguration {
                server: ServerSection::default(),
                encryption: EncryptionSection::default(),
            });
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
    #[serde(default)]
    use_assymetric_incapsulation: bool,
}

impl Default for EncryptionSection {
    fn default() -> Self {
        Self { use_assymetric_incapsulation: true }
    }
}
