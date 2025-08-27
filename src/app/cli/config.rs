use std::{ fs, io::{ Result, Write }, path::PathBuf };

use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    #[serde(skip)]
    pub config_path: PathBuf,

    pub storage_path: Option<String>,
}

impl Config {
    pub fn save(&self) -> Result<()> {
        let config_toml = toml::to_string_pretty(&self).unwrap();
        fs::write(&self.config_path, config_toml)
    }

    fn get_config_path(&self) -> PathBuf {
        let config_dir = dirs
            ::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("twin");

        fs::create_dir_all(&config_dir).expect("Failed to create config directory");

        config_dir.join("config.toml")
    }

    pub fn load_config(&mut self) -> Result<Config> {
        self.config_path = self.get_config_path();

        if self.config_path.exists() {
            let file_contents = fs::read_to_string(&self.config_path)?;
            let config = toml::from_str(&file_contents).unwrap_or_default();

            Ok(config)
        } else {
            let config = Config::default();
            let config_toml = toml::to_string_pretty(&config).unwrap();

            let mut file = fs::File::create(&self.config_path)?;
            file.write_all(config_toml.as_bytes())?;

            Ok(config)
        }
    }
}
